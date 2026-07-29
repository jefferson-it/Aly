mod queue {
    use crate::{lexer::Lexer, native::types::{Validator, ValueData}};

    #[derive(Clone, PartialEq)]
    pub struct Queue(Vec<ValueData>);

    impl Queue {
        pub fn new(value: Vec<ValueData>) -> Queue {
            Queue(value)
        }

        pub fn enqueue(&mut self, value: ValueData) {
            self.0.push(value);
        }

        pub fn dequeue(&mut self) -> Option<ValueData> {
            if self.0.is_empty() {
                None
            } else {
                Some(self.0.remove(0))
            }
        }

        pub fn to_string(&self, json: bool) -> String {
            let mut string = String::from("Queue (#data");

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

    pub fn create_queue(_lexer: Vec<Lexer>) -> Box<dyn Validator>{
        let values = vec![];
        let queue = Queue::new(values);

        return Box::new(ValueData::Queue(queue));
    }
}

pub use queue::*;
