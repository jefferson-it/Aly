pub mod fs;
pub mod vars;
pub mod types;
pub mod conditions;
pub mod create_object;
pub mod vector;

mod native {
    use std::io::{stdin, stdout, Write};

    use crate::{
        aly::{get_runtime, Aly},
        error::AlyError,
        lexer::Lexer,
        math_eval::eval_math,
        runtime::interpreter::exec,
        tokens::Tokens,
        validators::{
            is_any_value,
            reference::is_reference,
            str::{is_template_str, put_quoted_str, replace_spined, use_template_str},
        },
    };

    use super::types::{Validator, ValueData};

    // ─────────────────────────────────────────────────────────────────────────
    // Math expression evaluation
    //
    // Rationale: the `eval` crate was removed because it used JavaScript-style
    // semantics, had precision issues and brought an unnecessary dependency.
    // `eval_math` is our own Pratt-parser implementation in `src/math_eval.rs`.
    // ─────────────────────────────────────────────────────────────────────────

    /// Evaluate a math expression and return the result as a string.
    /// Returns `Err(AlyError)` instead of panicking on invalid input.
    pub fn exec_rust(expression: String) -> Result<String, AlyError> {
        eval_math(&expression).map_err(|e| {
            AlyError::runtime(format!(
                "Erro ao avaliar expressão '{}': {}",
                expression, e.message
            ))
        })
    }

    // ─────────────────────────────────────────────────────────────────────────
    // tomb — makes a variable immutable (pointer-based mutation lock)
    // ─────────────────────────────────────────────────────────────────────────

    pub fn tomb(x: String) -> Box<dyn Validator> {
        let run = get_runtime();
        let variables: Vec<&str> = x.split(' ').collect();

        for var in variables {
            if var.starts_with("address_") {
                let name = var[8..].to_string(); // strip "address_" prefix

                match run.get_var_per_name(name.clone()) {
                    Ok(v) => {
                        if let Err(err) = v.in_mut() {
                            eprintln!("RuntimeError: {}", err);
                        }
                    }
                    Err(err) => {
                        eprintln!("RuntimeError: {}", err);
                    }
                }
            } else {
                eprintln!(
                    "RuntimeError: tomb não aceita valores, apenas endereços de variáveis (use &nome)"
                );
            }
        }

        Box::new("None".to_owned())
    }

    // ─────────────────────────────────────────────────────────────────────────
    // Value processing
    // ─────────────────────────────────────────────────────────────────────────

    pub fn process_value(mut lexers: Vec<Lexer>) -> ValueData {
        let aly = get_runtime();

        if lexers.is_empty() {
            return ValueData::String("None".to_owned());
        } else if lexers.len() > 1 {
            let mut val: Box<dyn Validator> = Box::new(String::new());

            exec(&mut lexers, &mut val);

            let (_, res) = val.valid();

            return res;
        } else {
            let val = lexers[0].clone();
            let mut res = String::new();

            if val.literal.starts_with(&Tokens::Pointer.literal()) {
                res = format!("address_{}", val.literal.replace('&', ""));
            } else if is_any_value(&val.literal) {
                if is_template_str(&val.literal) {
                    res = use_template_str(val.literal);
                } else {
                    res = val.literal;
                }
            } else if is_reference(&val.literal) {
                let var = aly.get_var_per_name(val.literal);

                let _val_ = match var {
                    Ok(i) => i.get_value().to_string(true),
                    Err(_) => "None".to_string(),
                };

                res = _val_.clone()
            }

            ValueData::String(res)
        }
    }

    // ─────────────────────────────────────────────────────────────────────────
    // IO
    // ─────────────────────────────────────────────────────────────────────────

    pub fn fun_print(x: String) -> Box<dyn Validator> {
        println!("{}", replace_spined(x));
        Box::new("None".to_owned())
    }

    pub fn fun_input(x: String) -> Box<dyn Validator> {
        let mut output = String::new();

        print!("{}\n> ", replace_spined(x));

        if let Err(e) = stdout().flush() {
            eprintln!("RuntimeError: falha ao liberar stdout: {}", e);
        }

        if let Err(e) = stdin().read_line(&mut output) {
            eprintln!("RuntimeError: falha ao ler entrada: {}", e);
            return Box::new("None".to_owned());
        }

        let trimmed = output.trim().to_owned();

        if is_any_value(&trimmed) {
            Box::new(trimmed)
        } else {
            Box::new(put_quoted_str(trimmed))
        }
    }
}

pub use native::*;