mod stack {
    use crate::{lexer::Lexer, native::types::{Validator, ValueData}};

    #[derive(Clone, PartialEq)]
    pub struct Stack(Vec<ValueData>);

    impl Stack {
        pub fn new(value: Vec<ValueData>) -> Stack {
            Stack(value)
        }

        pub fn push(&mut self, value: ValueData) {
            self.0.push(value);
        }

        pub fn pop(&mut self) -> Option<ValueData> {
            self.0.pop()
        }

        pub fn to_string(&self, json: bool) -> String {
            let mut string = String::from("Stack (#data");

            if json {
                string = string.replace("#data", &self.to_json(0));
            }

            string.push_str(")");

            string
        }

        pub fn len(&self) -> usize {
            self.0.len()
        }

        pub fn to_json(&self, child: i32) -> String {
            let mut json = String::from("[\n");
            let space_prop = "   ".repeat(child.max(1) as usize);
            
            for data in &self.0 {
                let info = format!("{}{}\n", space_prop.repeat((child + 1).try_into().unwrap()), data.to_string(true));
                json.push_str(&info);
            }

            if child > 0 {
                json.push_str(&format!("{}", space_prop));
                json.push_str("]");
            } else {
                json.push_str("]");
            } 

            return json.to_owned();
        }
    }

    pub fn create_stack(_lexer: Vec<Lexer>) -> Box<dyn Validator>{
        // A stack can be created from a vector, for example.
        // For now, let's just initialize it with empty or existing elements.
        let values = vec![];
        let stack = Stack::new(values);

        return Box::new(ValueData::Stack(stack));
    }
}

pub use stack::*;
