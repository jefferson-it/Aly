mod interpreter {
    use regex::Regex;

    use crate::{aly::Aly, lexer::Lexer, native::{conditions::exec_cond, create_object::create_object, exec_rust, process_value, types::Validator, vars::{is_const_declaration, is_var_declaration}, vector::create_array}, tokens::Tokens, validators::{is_conditional_exp, numeric::is_math_operator, structures::{is_close, is_opened}}};

    fn line_is_dec(previous: Vec<Lexer>) -> bool {
        previous.iter().rev().find(|&item| item.token.id() == "identifier").is_some()
    }

    pub fn exec(run: &mut Aly, lexers: &mut Vec<Lexer>, val: &mut Box<dyn Validator>){
        let mut to_made = "none";
        let mut ind = 0;
        let mut previous: Vec<Lexer> = vec![];

        for lex in &mut *lexers {
            if is_opened(lex.token.clone()) 
                || is_close(lex.token.clone()) 
            {
                to_made = match lex.token.clone() {
                    Tokens::RightParenthesis | 
                    Tokens::LeftParenthesis => {
                        if to_made.contains("_dec") {
                            if line_is_dec(previous.clone()) {
                                to_made
                            } else {
                                "use_fun"
                            }
                        } else {
                            "use_fun"
                        }                        
                    },
                    Tokens::LeftBracket |
                    Tokens::RightBracket => {
                        if line_is_dec(previous.clone()) {
                            to_made
                        } else {
                            "create_array"
                        }
                    },
                    Tokens::LeftBrace |
                    Tokens::RightBrace => {
                        if line_is_dec(previous.clone()) {
                            to_made
                        } else {
                            match to_made {
                                "use_fun" => "dec_fun",
                                _ => "create_object"
                            }
                        }
                    }
                    _ => ""
                }
            }
            else if is_math_operator(lex.token.clone()) {
                to_made = if to_made.contains("_dec") {
                    if line_is_dec(previous.clone()) {
                        to_made
                    } else {
                        "math"
                    }
                } else {
                    "math"
                }
            } else if is_conditional_exp(lex.token.clone()) {
                to_made = if to_made.contains("_dec") {                    
                    if line_is_dec(previous.clone()) {
                        to_made
                    } else {
                        "conditional"
                    }
                } else {
                    "conditional"
                }
            }
            else if is_var_declaration(lex.token.clone()) {

                if (ind > 0 && 
                    to_made == "var_dec" && 
                    is_var_declaration(previous[ind - 1].clone().token)) || 
                    ind == 0 {
                    to_made = "var_dec";
                }
            } else if is_const_declaration(lex.token.clone()) {
                if (ind > 0 && 
                    to_made == "const_dec" && 
                    is_const_declaration(previous[ind - 1].clone().token)) || 
                    ind == 0 {
                    to_made = "const_dec";
                }
            } else if lex.token.id() == Tokens::Dot.id() {
                if to_made == "use_fun" {
                    previous.push(lex.clone());
                    ind += 1;
                    
                    continue;
                }

                to_made = if to_made.contains("_dec") {
                    if previous.iter().rev().skip(1).find(|&item| item.token.id() == "identifier").is_some() {
                        to_made
                    } else {
                        "use_prop"
                    }
                } else {
                    "use_prop"
                }                        
            }

            previous.push(lex.clone());
            ind += 1;
        }

        match to_made {
            "var_dec" => run.create_variable(previous),
            "const_dec" => run.create_constant(previous),
            "use_fun" => *val = run.function_run(previous),
            "math" => {
                let mut exp = String::new();
                let re_percent = Regex::new(r"(\d+\s?%)").unwrap();
                let re_percent_exp = Regex::new(r"(\d+\s?%\s?\d+)").unwrap();
                
                for prev in previous {
                    match prev.token {
                        Tokens::Value |
                        Tokens::Reference => {
                           let data = process_value(run, [prev].to_vec());

                           exp.push_str(&data.to_string(false))
                        }
                        _ => exp.push_str(&prev.literal)
                    }

                    exp.push_str(" ")
                }

                let caps: Vec<_> = re_percent_exp.captures_iter(&exp).map(|cap| cap[0].to_string()).collect();

                for item in caps {
                    let target: Vec<_> = item.split("%").collect();
                    let item1 = target[0].trim();
                    let item2 = target[1].trim();
                    let final_exp = format!("({item1} * ({item2} / 100))");
                    
                    
                    exp = exp.replace(&item, &final_exp);
                } 

                let caps_percent: Vec<_> = re_percent.captures_iter(&exp).map(|cap| cap[0].to_string()).collect();

                for item in caps_percent {
                    let target: Vec<_> = item.split("%").collect();
                    let item1 = target[0].trim();
                    let final_exp = format!("({item1} / 100)");

                    exp = exp.replace(&item, &final_exp);
                }

                exp = exp.replace("|", "%");

                match exec_rust(exp.clone()) {
                    Err(_) => {},
                    Ok(res) => {
                        *val = Box::new(res); 
                    },
                }    
            }
            "conditional" => {
                let res = exec_cond(run, previous);

                *val = res;
            }
            "use_prop" => *val = run.get_var_prop(previous),
            "create_object" => *val = create_object(run, previous),
            "create_array" => *val = create_array(run, previous),
            _ => {
                
            },
        };


        lexers.clear();
    }
}

pub use interpreter::*;