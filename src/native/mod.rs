pub mod fs;
pub mod vars;
pub mod types;
pub mod conditions;
pub mod create_object;
pub mod vector;

mod native {
    use std::io::{stdin, stdout, Write};

    use eval::eval;

    use crate::{aly::Aly, lexer::Lexer, runtime::interpreter::exec, tokens::Tokens, validators::{is_any_value, reference::is_reference, str::{is_template_str, put_quoted_str, replace_spined, use_template_str}}};

    use super::types::{Validator, ValueData};
    
    // Fun
    pub fn exec_rust(expression: String) -> Result<String, String> {

        match eval(&expression.clone()) {
            Err(e) => Err(format!("Error on exec the expression {expression}\n{e}")),
            Ok(it) => Ok(it.to_string())
        }
    } 

    pub fn tomb(run: &mut Aly, x: String) -> Box<dyn Validator> {
        let variables: Vec<&str> = x.split(" ").collect();

        for var in variables {
            if var.starts_with("address_") {
                let mut name = var.to_string();
                
                name.replace_range(..8, "");

                let result = match run.get_var_per_name(name.clone()) {
                    Ok(var) => var.in_mut(),
                    Err(err) => panic!("{err}"),
                };

                if result.is_err() {
                    result.unwrap_or_else(|err| panic!("Error on tomb: {}", err));
                }   
            } else {
                panic!("Error!! Tomb don't accept values, only variables address!")
            }
        }

        Box::new("None".to_owned())
    }

    // Catch
    pub fn catch_error(_e: String){

    }

    // Process
    pub fn process_value(aly: &mut Aly, mut lexers: Vec<Lexer>) -> ValueData {
        if lexers.len() == 0 {
            return ValueData::String("None".to_owned());   
        } else if lexers.len() > 1 {
            let mut val: Box<dyn Validator> = Box::new(String::new());
            
            exec(aly, &mut lexers, &mut val);

            let (_, res) = val.valid();

            return res;
        } else {
            let val = lexers[0].clone();
            let mut res = String::new();

            if val.literal.starts_with(&Tokens::Pointer.literal()) {
                res = format!("address_{}", val.literal.replace("&", ""));
            } else if is_any_value(&val.literal) {
                if is_template_str(&val.literal) {
                    res = use_template_str(aly, val.literal);
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

    // IO
    pub fn fun_print(_: &mut Aly, x: String) -> Box<dyn Validator> {
        println!("{}", replace_spined(x));

        return Box::new("None".to_owned());
    }
    
    pub fn fun_input(_: &mut Aly, x: String) -> Box<dyn Validator> {
        let mut output = String::new();
        
        print!("{}\n> ", replace_spined(x));
        
        stdout().flush().expect("Failed to flush stdout");
        stdin().read_line(&mut output).expect("Erro ao ler o dado!");

        if is_any_value(&output.trim()) {
            return Box::new(output.trim().to_owned());
        } else {
            return Box::new(put_quoted_str(output.trim().to_owned()));
        }

    }

}

pub use native::*;