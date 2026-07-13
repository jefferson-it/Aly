mod codec_mod {
    use crate::native::types::{Validator, ValueData};
    use crate::validators::str::{put_quoted_str, remove_quoted_str};
    use crate::native::std::{split_args, arg as std_arg};

    // base64.encode(string) -> string
    pub fn base64_encode(x: String) -> Box<dyn Validator> {
        use base64::Engine;
        let input = std_arg(&split_args(&x, 1), 0);
        let encoded = base64::engine::general_purpose::STANDARD.encode(input.as_bytes());
        Box::new(put_quoted_str(encoded))
    }

    // base64.decode(string) -> string
    pub fn base64_decode(x: String) -> Box<dyn Validator> {
        use base64::Engine;
        let input = std_arg(&split_args(&x, 1), 0);
        match base64::engine::general_purpose::STANDARD.decode(input.as_bytes()) {
            Ok(bytes) => {
                let decoded = String::from_utf8_lossy(&bytes).to_string();
                Box::new(put_quoted_str(decoded))
            }
            Err(e) => {
                eprintln!("RuntimeError [base64.decode]: {}", e);
                Box::new(put_quoted_str(String::new()))
            }
        }
    }

    // hex.encode(string) -> string
    pub fn hex_encode(x: String) -> Box<dyn Validator> {
        let input = std_arg(&split_args(&x, 1), 0);
        let encoded = hex::encode(input.as_bytes());
        Box::new(put_quoted_str(encoded))
    }

    // hex.decode(string) -> string
    pub fn hex_decode(x: String) -> Box<dyn Validator> {
        let input = std_arg(&split_args(&x, 1), 0);
        match hex::decode(input.as_bytes()) {
            Ok(bytes) => {
                let decoded = String::from_utf8_lossy(&bytes).to_string();
                Box::new(put_quoted_str(decoded))
            }
            Err(e) => {
                eprintln!("RuntimeError [hex.decode]: {}", e);
                Box::new(put_quoted_str(String::new()))
            }
        }
    }
}

pub use codec_mod::*;
