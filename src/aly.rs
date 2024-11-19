
mod aly {
    use std::env;

    use crate::{lexer::Lexer, native::{fs::read_file, fun_input, fun_print, process_value, tomb, types::{Type, Validator, ValueData}, vars::*}, runtime::parser::get_lexer, tokens::Tokens, validators::{str::remove_quoted_str, structures::{is_close, is_opened}}, Act};
    
    pub struct Aly {
        // args: Vec<String>,
        action: Act,
        datas: Vec<Var>
    }

    impl Aly {
        pub fn new(action: Act) -> Aly {
            let cwd = match env::current_dir() {
                Err(why) => panic!("Erro ao iniciar o programa, {why}"),
                Ok(item) => item,
            };

            Aly {
                // args: vec![],
                action,
                datas: vec![
                    Var::new(String::from("_dir_call"), format!("\"{}\"",  cwd.display().to_string()), false)
                ]
            }
        }

        // Runtime
        pub fn run(&mut self, file: String){
            self.datas.push(Var::new(String::from("print"), fun_print as fn(&mut Aly, String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("input"), fun_input as fn(&mut Aly, String) -> Box<dyn Validator>, false));
            self.datas.push(Var::new(String::from("tomb"), tomb as fn(&mut Aly, String) -> Box<dyn Validator>, false));

            match self.action {
                Act::Run => self.run_code(file),
                Act::Cli => {},
                Act::Comp => {},
            }
        }
        // Internal Functions 
        fn run_code(&mut self, path: String) {
            let file_to_run = read_file(path);
            let codes: Vec<&str> = file_to_run.trim().split("\n").collect();

            get_lexer(self, codes);
        }

        // Variable manager
        pub fn get_vars(&self) -> &Vec<Var> {
            &self.datas
        }

        pub fn create_variable(&mut self, lexers: Vec<Lexer>) {
            if lexers[0].token.id() != "def_let" {
                let name = &lexers[0];
                let identifier = &lexers[1];
                let value = process_value(self, (&lexers[2..]).to_vec());
                let var = self.get_var(name.clone());

                match identifier.token {
                    Tokens::Identifier => (),
                    _ => panic!("")
                };

                                
                let result = match var {
                    Ok(v) => v.change_value(value),
                    Err(err) => panic!("{err}"),
                };

                if result.is_err() {
                    result.unwrap_or_else(|err| panic!("Error on line {}: {}", name.line, err));
                }

                return;
            }

            let name = &lexers[1];

            if lexers.len() == 2 {
                let value = ValueData::String("None".to_owned());    

                let var = Var::new(name.literal.to_string(), value.to_string(false), true);

                self.datas.push(var);

                return;
            } 
            
            let identifier = &lexers[2];
            let value = process_value(self, (&lexers[3..]).to_vec());

            match identifier.token {
                Tokens::Identifier => (),
                _ => panic!("")
            };

            let var = Var::new(name.literal.to_string(), value, true);

            self.datas.push(var)
        }
    
        pub fn create_constant(&mut self, lexers: Vec<Lexer>) {
            let name = &lexers[1];

            if lexers.len() == 2 {

                return;
            } 
            
            let identifier = &lexers[2];
            let value = process_value(self, (&lexers[3..]).to_vec());

            match identifier.token {
                Tokens::Identifier => (),
                _ => panic!("")
            };

            let constant = Var::new(name.literal.to_string(), value, false);

            self.datas.push(constant)
        } 

        pub fn get_var(&mut self, name: Lexer) -> Result<&mut Var, String> {
            let var = self.datas.iter_mut().find(|var| var.compare_var(name.literal.clone()));

            match var {
                Some(v) => Ok(v),
                None => Err(format!(
                    "Error on line {}: Variable {} not found", 
                    name.line,
                    name.literal,
                )),
            }
        }

        pub fn get_var_per_name(&mut self, name: String) -> Result<&mut Var, String> {
            let var = self.datas.iter_mut().find(|var| var.compare_var(name.clone()));

            match var {
                Some(v) => Ok(v),
                None => Err(format!(
                    "Variable {} not found",
                    name
                )),
            }
        }

        pub fn get_var_prop(&mut self, lexers: Vec<Lexer>) -> Box<dyn Validator> {
            let var_target = match self.get_var(lexers[0].clone()) {
                Ok(var) => var.get_prop(lexers[1..].to_vec()),
                Err(err) => panic!("{err}"),
            };

            var_target
        }

        // Function
        pub fn function_run(&mut self, lexers: Vec<Lexer>) -> Box<dyn Validator> {
            let name = &lexers[0];
            let mut params: Vec<Lexer> = vec![];
            let mut fun_body: Vec<Lexer> = vec![];
            let mut another_fun = 0;

            for lex in &lexers[2..lexers.len() - 1] {
                if lex.literal == "," {
                    continue;
                } else if is_opened(lex.token.clone()) {
                    another_fun += 1;

                    if another_fun == 1 {
                        let ind = params.len() - 1;
                        fun_body.push(lexers[2 + ind].clone());
                    }

                    fun_body.push(lex.clone());
                    
                    params.pop();
                    
                }else if is_close(lex.token.clone()) {
                    another_fun -= 1;
                    fun_body.push(lex.clone());

                    if another_fun == 0 {
                        let res = process_value(self, fun_body.clone());
                        let lexer_res = Lexer::new(Tokens::Value, res.to_string(true), lex.line);

                        params.push(lexer_res);
                        fun_body.clear(); 
                    }
                } else {
                    if another_fun > 0 {
                        fun_body.push(lex.clone());
                    } else {
                        params.push(lex.clone())
                    }
                }
            }

            return match self.get_var(name.clone()) {
                Ok(ok) => {
                    match ok.get_type() {
                        Type::NativeFunction | 
                        Type::Function => {
                            match ok.get_value() {
                                ValueData::NativeFunction(fun) => {
                                    let param = remove_quoted_str(process_value(self, params).to_string(false));
                                    fun(self, param)
                                },
                                _ => {
                                    Box::new("None".to_owned())
                                }
                            }
                        },
                        _ => panic!("Error on line {}: {} is not a function!", name.line, name.literal)
                    }
                },
                Err(err) => panic!("{}", err),
            };
        }
    }
}

pub use aly::*;