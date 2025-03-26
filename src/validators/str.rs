mod str {
    use regex::Regex;

    use crate::{aly::get_runtime, lexer::Lexer, tokens::get_token};

    pub fn split_str(code: &str) -> Vec<String> {
        let re = Regex::new(r#"("[^"\\]*(?:\\.[^"\\]*)*"|'[^'\\]*(?:\\.[^'\\]*)*'|\S+)"#).unwrap();
        let mut vector: Vec<String> = vec![];
    
        let res: Vec<&str> = re.captures_iter(code)
            .map(|cap| cap.get(0).unwrap().as_str())
            .collect();

        for item in &res {
            vector.push(item.to_string())
        }

        return vector;
    }

    pub fn is_any_str(item: &str) -> bool {
        is_simple_str(item) || is_template_str(item)
    }

    pub fn is_simple_str(item: &str) -> bool{
        let re = Regex::new(r#"('[^']*')"#).unwrap();
        
        re.is_match(item)
    }
    
    pub fn is_template_str(item: &str) -> bool{
        let re = Regex::new(r#"("[^"]*")"#).unwrap();
        
        re.is_match(item)
    }

    pub fn replace_spined(text: String) -> String{
        return text
            .replace("\\n", "\n")
            .replace("\\'", "\'")
            .replace("\\\"", "\"")
            .replace("\\r", "\r")
            .replace("\\t", "\t");
    }


    pub fn put_quoted_str(data: String) -> String {
        return format!("\"{data}\"");
    }

    pub fn remove_quoted_str(mut data: String) -> String {
        if data.starts_with("'") {
            if let Some(ind) = data.find("\'") {
                data.replace_range(ind..ind + 1, "");
            }
            if let Some(ind) = data.rfind("\'") {
                data.replace_range(ind..ind + 1, "");
            }
        } else if data.starts_with("\"") {
            if let Some(ind) = data.find("\"") {
                data.replace_range(ind..ind + 1, "");
            }
            if let Some(ind) = data.rfind("\"") {
                data.replace_range(ind..ind + 1, "");
            }
        }

        return data;
    }

    pub fn use_template_str(_str_: String) -> String {
        let aly = get_runtime();

        let mut output = String::new();
        let variables_capture = Regex::new(r#"\$([a-zA-Z]\w*(?:\.[a-zA-Z]\w*)*)"#).unwrap();
        let mut final_str = _str_.clone();
        let mut capt_vars: Vec<_> = variables_capture
            .captures_iter(&_str_)
            .map(|cap| cap.get(1).unwrap().as_str())
            .collect();

        capt_vars.sort_by_key(|cap| cap.len());
        capt_vars.reverse();

        for cap in capt_vars {
            let variable = &aly.get_vars();
            let var_expect = cap.replace("$", "");
            let props: Vec<&str> = var_expect.split(".").collect();
            let variable_found = variable.iter().find(|&v| v.compare_var(props[0].to_owned()));

            match variable_found {
                Some(variable) => {
                    if props.len() > 1 {
                        let mut lex = vec![];

                        for i in props[1..].to_vec() {
                            lex.push(
                                Lexer::new(
                                    get_token(i.to_owned()), 
                                    i.to_owned(), 
                                    0
                                )
                            );
                        }

                        let val = &variable.get_value().get_prop(false, lex).valid().1.to_string(false);
                        
                        final_str = final_str.replace(&format!("${}", cap), &remove_quoted_str(val.to_string())).clone();
                    } else {
                        final_str = final_str.replace(&format!("${}", cap), &variable.get_value().to_string(false)).clone();
                    }
                },
                None => {
                    panic!("Error -> The variable {var_expect} is not defined");
                }
            }
        }
        
        output.push_str(&final_str);

        return output;
    }
    
}

pub use str::*;