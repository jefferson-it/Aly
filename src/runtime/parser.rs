mod parser {
    use crate::{aly::Aly, lexer::Lexer, native::types::Validator, runtime::interpreter::exec, tokens::{get_token, Tokens}, validators::{is_char, is_conditional_exp, is_num, numeric::is_math_operator, str::split_str, structures::{close_str, has_open_str, is_close, is_opened, open_str}}};

    // To interpreter code
    pub fn get_lexer(run: &mut Aly, lines: Vec<&str>) {
        let mut lexers = vec![];
        let mut to_end = 0;
        let mut is_str = Tokens::None;
        let mut ind = 1;
        let mut comment_multi = false;

        for line in lines {
            let mut exp = String::new();
            let mut previous = "";

            for letter in line.split("") {
                exp.push_str(
                    &letter_per_letter(letter, previous, &mut to_end, &mut is_str)
                );

                previous = letter;
            }
            
            exp = exp.replace("  ", " ");

            ind += 1;

            let expressions = split_str(&exp.trim());

            for expression in expressions {
                if expression.trim() == ";" {
                    continue;
                } else if expression.trim() == Tokens::CommentLine.literal() {
                    break;
                } else if expression.trim() == Tokens::CommentMulti.literal() {
                    comment_multi = if comment_multi { false } else { true };
                    
                    continue;
                } 

                if comment_multi {
                    continue;
                }
                
                lexers.push(
                    Lexer::new(
                        get_token(expression.clone()), 
                        expression, 
                        ind
                    )
                );
            }

            if to_end == 0 {
                let mut val: Box<dyn Validator> = Box::new(String::new());
                
                exec(run, &mut lexers, &mut val);
            } else if to_end < 0 {
                panic!("Error on line {ind}: Closing a brace, but not open!")   
            }
        }


        if to_end > 0 {
            panic!("Error on line {ind}: Opening a brace, but not closing!")   
        }
    }

    fn letter_per_letter(letter: &str, previous: &str, to_end: &mut i32, is_str: &mut Tokens) -> String {
        let res = match letter.trim() {
            "" | " " => letter.to_string(),
            _ => {
                let tk = get_token(letter.to_owned());
                
                if has_open_str(is_str.clone()) {
                    if close_str(tk.clone(), is_str.clone()) {
                        *is_str = Tokens::None;

                        return format!("{} ", letter);
                    }
                    
                    return letter.to_string();
                } 

                if letter.trim() == Tokens::Semicolon.literal() {
                    return format!(" {} ", letter);
                }

                if tk.id() == Tokens::CommentLine.id() && previous.trim() != Tokens::CommentLine.literal() {
                    return format!(" {}", letter);
                }

                if tk.id() == Tokens::Comma.id() {
                    return format!(" {} ", letter);
                }
                
                
                else if is_conditional_exp(tk.clone()) {
                    return format!(" {}", letter);
                } else if (
                        previous.trim() == Tokens::GreaterThan.literal() ||
                        previous.trim() == Tokens::LessThan.literal() ||
                        previous.trim() == Tokens::Identifier.literal()
                    ) && tk.id() == Tokens::Identifier.id() {
                    return format!("{} ", letter);
                }

                if is_opened(tk.clone()) {
                    *to_end += 1;
                    *is_str = tk.clone();
                    return format!(" {}", letter);
                } else if is_close(tk.clone()) {
                    *to_end -= 1;
                    *is_str = Tokens::None;
                    return format!(" {}", letter);
                } else if is_math_operator(tk.clone()) || tk.id() == Tokens::Dot.id() {
                    return format!(" {} ", letter);
                }

                if is_char(letter) {
                    if is_char(previous) || 
                        is_num(previous) || 
                        previous == " " ||
                        previous == Tokens::Pointer.literal() {
                        letter.to_string()
                    } else {
                        format!(" {}", letter)
                    }
                } else if is_num(letter) {
                    if is_num(previous) || is_char(previous) || previous == " " {
                        letter.to_string()
                    } else {
                        format!(" {}", letter)
                    }
                } else if open_str(Tokens::None, is_str.clone()){
                    letter.to_string()
                } else {

                    if tk.id() == "identifier" {
                        return format!(" {}", letter);
                    } else if tk.literal() == Tokens::Pointer.literal() {
                        return format!(" {}", letter);
                    }

                    if open_str(tk.clone(), is_str.clone()) {
                        *is_str = tk.clone();
                        format!(" {}", letter)
                    } else {
                        *is_str = Tokens::None;
                        letter.to_string()
                    }
                }
            }
        };
    
        res
    }

}

pub use parser::*;