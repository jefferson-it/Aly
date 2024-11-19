mod number {
    use regex::Regex;

    use crate::tokens::Tokens;

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