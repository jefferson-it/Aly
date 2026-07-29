mod enumeration {
    use crate::{lexer::Lexer, native::types::{Validator, ValueData}};
    use std::collections::HashMap;

    #[derive(Clone, PartialEq)]
    pub struct Enumeration {
        pub name: String,
        pub variants: HashMap<String, ValueData>,
    }

    impl Enumeration {
        pub fn new(name: String, variants: HashMap<String, ValueData>) -> Enumeration {
            Enumeration { name, variants }
        }

        pub fn to_string(&self, _json: bool) -> String {
            format!("Enum ({})", self.name)
        }
    }

    pub fn create_enum(_lexer: Vec<Lexer>) -> Box<dyn Validator>{
        // Simplest implementation for now
        let enum_val = Enumeration::new("UnnamedEnum".to_string(), HashMap::new());

        return Box::new(ValueData::Enum(enum_val));
    }
}

pub use enumeration::*;
