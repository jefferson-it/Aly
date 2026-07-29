mod number {
    use regex::Regex;

    use crate::{lexer::Lexer, native::types::Validator, tokens::Tokens};

    /// Remove underscores from a numeric string (e.g. "1_000_000" -> "1000000")
    pub fn strip_underscores(s: &str) -> String {
        s.replace('_', "")
    }

    pub fn def_value_float(lexers: Vec<Lexer>) -> Box<dyn Validator>  {
        let mut value = String::new();
        let mut l: i32 = 0;

        for lex in lexers {
            let chars = lex.literal.chars();
            
            if l != lex.line { l = lex.line };

            for char in chars {
                if char.is_numeric() || char == '_' {
                    value.push(char);
                } else if char == '.' {
                    value.push(char);
                } else {
                    eprintln!("TypeError [numeric]: caractere '{}' invalido para valor float na linha {}.", char, lex.line);
                    return Box::new(String::from("0.0"));
                }
            }           
        }

        let clean = strip_underscores(&value);

        if !is_float(&clean) {
            eprintln!("TypeError [numeric]: valor '{}' invalido para float na linha {}.", value, l);
            return Box::new(String::from("0.0"));
        }

        return Box::new(clean);
    }

    pub fn is_any_number(item: &str) -> bool {
        let clean = strip_underscores(item);
        is_int(&clean) || is_float(&clean)
    }

    pub fn is_int(item: &str) -> bool {
        let clean = strip_underscores(item);
        let re = Regex::new(r"^-?[0-9]+$").unwrap();
        re.is_match(&clean)
    }

    pub fn is_float(item: &str) -> bool {
        let clean = strip_underscores(item);
        let re = Regex::new(r"^\d+(\.\d+)?$").unwrap();
        re.is_match(&clean)
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