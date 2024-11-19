pub mod str;
pub mod numeric;
pub mod path;
pub mod structures;
pub mod reference;

mod validators {
    use regex::Regex;

    use crate::tokens::Tokens;

    use super::{numeric::is_any_number, str::is_any_str};

    // Verify Value
    pub fn is_any_value(item: &str) -> bool {
        is_any_number(item) ||
        is_any_str(item) || 
        is_bool(item)
    }   

    pub fn is_bool(item: &str) -> bool {
        item == "true" || item == "false"
    } 

    pub fn is_char(item: &str) -> bool {
        let re = Regex::new("(?i)[a-z|_]").unwrap();

        re.is_match(item)
    }

    pub fn is_num(item: &str) -> bool {
        let re = Regex::new("[0-9]").unwrap();

        re.is_match(item)
    }

    pub fn is_conditional_exp(tk: Tokens) -> bool{
        match tk {
            Tokens::GreaterThan |
            Tokens::GreaterThanOrEqual |
            Tokens::LessThan |
            Tokens::LessThanOrEqual |
            Tokens::Equal | 
            Tokens::NotEqual |
            Tokens::And |
            Tokens::Or |
            Tokens::Xor | 
            Tokens::Not => true,
            _ => false
        }
    }

    // Conversor

    pub fn conversor_to_int(item: String) -> i32 {
        match item.parse::<i32>() {
            Ok(int) => int,
            Err(_) => {
                -1
            },
        }
    }
    pub fn conversor_to_float(item: String) -> f32 {
        match item.parse::<f32>() {
            Ok(float) => float,
            Err(_) => {
                -1.0
            },
        }
    }
    pub fn conversor_to_bool(item: String) -> bool {
        if item == "true" {
            true
        } else if item == "false" {
            false
        } else {
            // After made the truth value
            // 0 == false
            // x > 0 == true 
            // "" | None == false 
            // !str.empty == true 
            true
        }
    }
}

pub use validators::*;