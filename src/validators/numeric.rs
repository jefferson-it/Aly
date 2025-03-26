mod number {
    use regex::Regex;

    use crate::{lexer::Lexer, native::types::Validator, tokens::Tokens};

    pub fn def_value_float(lexers: Vec<Lexer>) -> Box<dyn Validator>  {
        let mut value = String::new();
        let mut l: i32 = 0;

        for lex in lexers {
            let chars = lex.literal.chars();
            
            if l != lex.line { l = lex.line };

            for char in chars {
                if char.is_numeric() {
                    value.push(char);
                } else if char == '.' {
                    value.push(char);
                } else {
                    panic!("{}", format!("Erro on line {}: the {} is invalid to float value", lex.line, char))
                }
            }           
        }

        if !is_float(&value.clone()) {
            panic!("{}", format!("Erro on line {}: the {} is invalid float", l, value))
        }

        return Box::new(value);
    }

    pub fn is_any_number(item: &str) -> bool {
        is_int(item) || is_float(item)
    }

    pub fn is_int(item: &str) -> bool {
        let re = Regex::new(r"^-?[0-9]+$").unwrap();

        re.is_match(item)
    }

    pub fn is_float(item: &str) -> bool {
        let re = Regex::new(r"^\d+(\.\d+)?$").unwrap();

        re.is_match(item)
    }

    // Math
    pub fn is_math_operator(item: Tokens) -> bool {

        match item {
            Tokens::Addition |
            Tokens::Subtraction |
            Tokens::Percent |
            Tokens::Multiplication |
            Tokens::Division |
            Tokens::Modulus |
            Tokens::LeftParenthesis |
            Tokens::RightParenthesis => true,
            _ => false
        }
    }
}

pub use number::*;