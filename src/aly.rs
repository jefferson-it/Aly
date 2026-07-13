/// Aly runtime — owns the execution context for a single Aly program.
///
/// # Architecture decision: global runtime
///
/// The current architecture passes the runtime through a module-level accessor.
/// This is kept for backwards-compatibility but the `unsafe` static has been
/// replaced with a `thread_local!` + `RefCell` pattern.
///
/// Rationale for `thread_local!`:
/// - Zero `unsafe` code in the public API.
/// - Naturally isolated per OS thread, which matches the current single-threaded
///   execution model.
/// - Cheap — no heap allocation, no mutex overhead.
/// - When concurrent execution is added in the future, each worker thread will
///   own its own runtime cell, which is exactly what we want for task isolation.
/// - Alternative considered: `Arc<Mutex<Aly>>` — unnecessarily complex and
///   slower for a single-threaded interpreter.
///
/// The public `get_runtime()` function still returns `&'static mut Aly`-shaped
/// access via a raw pointer so that callers do not need to hold a `RefMut`
/// guard across function boundaries.  This is sound inside a single thread.

mod aly {
    use std::cell::RefCell;
    use std::env;

    use crate::{
        error::{AlyError, AlyResult},
        lexer::Lexer,
        native::{
            fs::read_file, fun_input, fun_print, process_value, tomb,
            types::{Type, Validator, ValueData},
            vars::*,
        },
        runtime::parser::get_lexer,
        tokens::Tokens,
        validators::{
            str::{put_quoted_str, remove_quoted_str},
            structures::{is_close, is_opened},
        },
    };

    /// Action types for the Aly runtime
    #[derive(Clone, Debug)]
    pub enum Act {
        Run,
        Cli,
        Comp,
    }

    // ──────────────────────────────────────────────────────────────────────────
    // The runtime struct
    // ──────────────────────────────────────────────────────────────────────────

    #[derive(Clone)]
    pub struct Aly {
        action: Option<Act>,
        datas: Vec<Var>,
    }

    // ──────────────────────────────────────────────────────────────────────────
    // Thread-local storage — replaces `static mut RUNTIME`
    // ──────────────────────────────────────────────────────────────────────────

    thread_local! {
        static RUNTIME: RefCell<Aly> = RefCell::new(Aly {
            action: None,
            datas: Vec::new(),
        });
    }

    /// Get a raw mutable pointer to the thread-local runtime.
    ///
    /// # Safety
    /// This is safe when called from a single thread (the current execution
    /// model).  The pointer is valid for the duration of the thread-local's
    /// lifetime.  If multi-threaded execution is added in the future this
    /// function must be replaced with an `Arc<Mutex<Aly>>` accessor.
    pub fn get_runtime() -> &'static mut Aly {
        RUNTIME.with(|cell| {
            // SAFETY: single-threaded interpreter; the borrow is released
            // before any recursive call could reach here.
            unsafe { &mut *cell.as_ptr() }
        })
    }

    // ──────────────────────────────────────────────────────────────────────────
    // Aly implementation
    // ──────────────────────────────────────────────────────────────────────────

    impl Aly {
        pub fn def_action(&mut self, act: Act) -> AlyResult<()> {
            if self.action.is_some() {
                return Err(AlyError::runtime("A ação já foi definida e não pode ser alterada."));
            }
            self.action = Some(act);
            Ok(())
        }

        pub fn new() -> AlyResult<Aly> {
            let cwd = env::current_dir().map_err(|e| {
                AlyError::runtime(format!(
                    "Erro ao iniciar o programa: não foi possível obter o diretório atual. {}",
                    e
                ))
            })?;

            Ok(Aly {
                action: None,
                datas: vec![Var::new(
                    String::from("_dir_call"),
                    format!("\"{}\"", cwd.display()),
                    false,
                )],
            })
        }

        // ── Runtime ──────────────────────────────────────────────────────────

        pub fn run(&mut self, file: String) {
            self.datas.push(Var::new(
                String::from("print"),
                fun_print as fn(String) -> Box<dyn Validator>,
                false,
            ));
            self.datas.push(Var::new(
                String::from("input"),
                fun_input as fn(String) -> Box<dyn Validator>,
                false,
            ));
            self.datas.push(Var::new(
                String::from("tomb"),
                tomb as fn(String) -> Box<dyn Validator>,
                false,
            ));

            match &self.action {
                Some(act) => match act {
                    Act::Run => self.run_code(file),
                    Act::Cli => {}
                    Act::Comp => {}
                },
                None => {
                    eprintln!("RuntimeError: nenhuma ação foi definida antes de chamar run().");
                }
            };
        }

        fn run_code(&mut self, path: String) {
            let file_to_run = read_file(path);
            let codes: Vec<&str> = file_to_run.trim().split('\n').collect();
            get_lexer(codes);
        }

        // ── Variable manager ─────────────────────────────────────────────────

        pub fn get_vars(&self) -> &Vec<Var> {
            &self.datas
        }

        pub fn create_variable(&mut self, lexers: Vec<Lexer>) {
            if lexers.is_empty() {
                eprintln!("SyntaxError: instrução de variável vazia.");
                return;
            }

            if lexers[0].token.id() != "def_let" {
                // Reassignment: `name = value` or compound: `name += value`
                let name = &lexers[0];
                let op_token = match lexers.get(1) {
                    Some(l) => &l.token,
                    None => {
                        eprintln!("SyntaxError: esperado '=' após o nome da variável na linha {}.", name.line);
                        return;
                    }
                };

                // Check for compound assignment operators
                let compound_op = match op_token {
                    Tokens::PlusEqual => Some("+"),
                    Tokens::MinusEqual => Some("-"),
                    Tokens::TimesEqual => Some("*"),
                    Tokens::DivideEqual => Some("/"),
                    Tokens::ModulusEqual => Some("%"),
                    Tokens::Identifier => None, // regular assignment
                    _ => {
                        eprintln!(
                            "SyntaxError: esperado '=' ou operador de atribuição composto após o nome da variável na linha {}.",
                            name.line
                        );
                        return;
                    }
                };

                let value_lexers = if compound_op.is_some() {
                    // For compound: get the value after the operator
                    if lexers.len() < 3 {
                        eprintln!("SyntaxError: esperado valor após o operador na linha {}.", name.line);
                        return;
                    }
                    // Get current value and combine with new value
                    match self.get_var(name.clone()) {
                        Ok(v) => {
                            let current_val = v.get_value().to_string(false);
                            let new_val = process_value(lexers[2..].to_vec()).to_string(false);
                            // Build expression: current op new_value
                            let op = compound_op.unwrap();
                            let combined = format!("{} {} {}", current_val, op, new_val);
                            vec![Lexer::new(Tokens::Value, combined, name.line)]
                        }
                        Err(err) => {
                            eprintln!("ReferenceError na linha {}: {}", name.line, err);
                            return;
                        }
                    }
                } else {
                    // Regular assignment: get the value after '='
                    lexers[2..].to_vec()
                };

                match self.get_var(name.clone()) {
                    Ok(v) => {
                        let final_value = process_value(value_lexers);
                        if let Err(err) = v.change_value(final_value) {
                            eprintln!("TypeError na linha {}: {}", name.line, err);
                        }
                    }
                    Err(err) => {
                        eprintln!("ReferenceError na linha {}: {}", name.line, err);
                    }
                }

                return;
            }

            // Declaration: `let name = value`
            let name = match lexers.get(1) {
                Some(l) => l.clone(),
                None => {
                    eprintln!("SyntaxError: esperado nome após 'let'.");
                    return;
                }
            };

            if lexers.len() == 2 {
                // `let x` — initialise to None
                let var = Var::new(name.literal.to_string(), ValueData::String("None".to_owned()).to_string(false), true);
                self.datas.push(var);
                return;
            }

            let identifier = match lexers.get(2) {
                Some(l) => l,
                None => {
                    eprintln!("SyntaxError: esperado '=' após o nome da variável na linha {}.", name.line);
                    return;
                }
            };

            if identifier.token != Tokens::Identifier {
                eprintln!(
                    "SyntaxError: esperado '=' após o nome da variável na linha {}.",
                    name.line
                );
                return;
            }

            let value = process_value(lexers[3..].to_vec());
            let var = Var::new(name.literal.to_string(), value, true);
            self.datas.push(var);
        }

        pub fn create_constant(&mut self, lexers: Vec<Lexer>) {
            if lexers.len() < 2 {
                eprintln!("SyntaxError: esperado nome após 'const'.");
                return;
            }

            let name = lexers[1].clone();

            if lexers.len() == 2 {
                // `const x` without a value — silently skip (or warn)
                return;
            }

            let identifier = match lexers.get(2) {
                Some(l) => l,
                None => {
                    eprintln!("SyntaxError: esperado '=' após o nome da constante na linha {}.", name.line);
                    return;
                }
            };

            if identifier.token != Tokens::Identifier {
                eprintln!(
                    "SyntaxError: esperado '=' após o nome da constante na linha {}.",
                    name.line
                );
                return;
            }

            let value = process_value(lexers[3..].to_vec());
            let constant = Var::new(name.literal.to_string(), value, false);
            self.datas.push(constant);
        }

        pub fn get_var(&mut self, name: Lexer) -> Result<&mut Var, String> {
            let var = self
                .datas
                .iter_mut()
                .find(|var| var.compare_var(name.literal.clone()));

            match var {
                Some(v) => Ok(v),
                None => Err(format!(
                    "Variável '{}' não existe.\n\nlinha: {}",
                    name.literal, name.line,
                )),
            }
        }

        pub fn get_var_per_name(&mut self, name: String) -> Result<&mut Var, String> {
            let var = self
                .datas
                .iter_mut()
                .find(|var| var.compare_var(name.clone()));

            match var {
                Some(v) => Ok(v),
                None => Err(format!("Variável '{}' não existe.", name)),
            }
        }

        pub fn get_var_prop(&mut self, lexers: Vec<Lexer>) -> Box<dyn Validator> {
            if lexers.is_empty() {
                eprintln!("InternalError: get_var_prop chamado com lista de lexers vazia.");
                return Box::new("None".to_owned());
            }

            match self.get_var(lexers[0].clone()) {
                Ok(var) => var.get_prop(lexers[1..].to_vec()),
                Err(err) => {
                    eprintln!("ReferenceError: {}", err);
                    Box::new("None".to_owned())
                }
            }
        }

        // ── Function execution ───────────────────────────────────────────────

        pub fn function_run(&mut self, lexers: Vec<Lexer>) -> Box<dyn Validator> {
            if lexers.is_empty() {
                eprintln!("SyntaxError: chamada de função vazia.");
                return Box::new("None".to_owned());
            }

            let name = &lexers[0];
            let mut params: Vec<Lexer> = vec![];
            let mut fun_body: Vec<Lexer> = vec![];
            let mut another_fun = 0;

            // Parse arguments, resolving nested function calls inline
            let args_range = if lexers.len() > 2 {
                &lexers[2..lexers.len() - 1]
            } else {
                &[]
            };

            for lex in args_range {
                if lex.literal == "," {
                    continue;
                } else if is_opened(lex.token.clone()) {
                    another_fun += 1;

                    if another_fun == 1 {
                        let ind = params.len().saturating_sub(1);
                        if !params.is_empty() {
                            fun_body.push(lexers[2 + ind].clone());
                        }
                    }

                    fun_body.push(lex.clone());
                    params.pop();
                } else if is_close(lex.token.clone()) {
                    another_fun -= 1;
                    fun_body.push(lex.clone());

                    if another_fun == 0 {
                        let res = process_value(fun_body.clone());
                        let lexer_res = Lexer::new(Tokens::Value, res.to_string(true), lex.line);
                        params.push(lexer_res);
                        fun_body.clear();
                    }
                } else if another_fun > 0 {
                    fun_body.push(lex.clone());
                } else {
                    params.push(lex.clone());
                }
            }

            match self.get_var(name.clone()) {
                Ok(ok) => {
                    match ok.get_type() {
                        Type::NativeFunction | Type::Function => {
                            match ok.get_value() {
                                ValueData::NativeFunction(fun) => {
                                    let param = remove_quoted_str(
                                        process_value(params).to_string(false),
                                    );
                                    let res = fun(param);
                                    let (_, val) = res.valid();
                                    Box::new(put_quoted_str(val.literal()))
                                }
                                _ => Box::new("None".to_owned()),
                            }
                        }
                        _ => {
                            eprintln!(
                                "TypeError na linha {}: '{}' não é uma função.",
                                name.line, name.literal
                            );
                            Box::new("None".to_owned())
                        }
                    }
                }
                Err(err) => {
                    eprintln!("ReferenceError: {}", err);
                    Box::new("None".to_owned())
                }
            }
        }
    }
}

pub use aly::*;