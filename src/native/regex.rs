mod regex_mod {
    use crate::native::types::{Validator, ValueData};
    use crate::native::vector::Vector;
    use crate::validators::str::{put_quoted_str, remove_quoted_str};
    use crate::native::std::{split_args, arg as std_arg};

    pub fn regex_match(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let pattern = std_arg(&args, 0);
        let text = std_arg(&args, 1);

        let re = match regex::Regex::new(&pattern) {
            Ok(r) => r,
            Err(e) => panic!("regex.match error: {}", e),
        };

        if let Some(cap) = re.captures(&text) {
            let mut groups = vec![];
            for m in cap.iter() {
                match m {
                    Some(mat) => groups.push(ValueData::String(mat.as_str().to_string())),
                    None => groups.push(ValueData::String("None".to_owned())),
                }
            }
            Box::new(ValueData::Vec(Vector::new(groups)))
        } else {
            Box::new(ValueData::String("None".to_owned()))
        }
    }

    pub fn regex_test(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let pattern = std_arg(&args, 0);
        let text = std_arg(&args, 1);

        let re = match regex::Regex::new(&pattern) {
            Ok(r) => r,
            Err(e) => panic!("regex.test error: {}", e),
        };

        Box::new(ValueData::Bool(re.is_match(&text)))
    }

    pub fn regex_replace(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 3);
        let pattern = std_arg(&args, 0);
        let text = std_arg(&args, 1);
        let replacement = std_arg(&args, 2);

        let re = match regex::Regex::new(&pattern) {
            Ok(r) => r,
            Err(e) => panic!("regex.replace error: {}", e),
        };

        let result = re.replace_all(&text, replacement.as_str());
        Box::new(put_quoted_str(result.to_string()))
    }
}

pub use regex_mod::*;
