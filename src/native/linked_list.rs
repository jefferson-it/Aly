mod linked_list {
    use crate::{lexer::Lexer, native::types::{Validator, ValueData}};

    #[derive(Clone, PartialEq)]
    pub struct Node {
        pub value: ValueData,
        pub next: Option<Box<Node>>,
    }

    #[derive(Clone, PartialEq)]
    pub struct LinkedList {
        pub head: Option<Box<Node>>,
        pub size: usize,
    }

    impl LinkedList {
        pub fn new() -> LinkedList {
            LinkedList { head: None, size: 0 }
        }

        pub fn push(&mut self, value: ValueData) {
            let new_node = Box::new(Node {
                value,
                next: self.head.take(),
            });
            self.head = Some(new_node);
            self.size += 1;
        }

        pub fn to_string(&self, json: bool) -> String {
            let mut string = String::from("LinkedList (#data");

            if json {
                string = string.replace("#data", &self.to_json(0));
            }

            string.push_str(")");

            string
        }

        pub fn len(&self) -> usize {
            self.size
        }

        pub fn to_json(&self, child: i32) -> String {
            let mut json = String::from("[\n");
            let space_prop = "   ".repeat(child.max(1) as usize);
            
            let mut current = &self.head;
            while let Some(node) = current {
                let info = format!("{}{}\n", space_prop.repeat((child + 1).try_into().unwrap()), node.value.to_string(true));
                json.push_str(&info);
                current = &node.next;
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

    pub fn create_linked_list(_lexer: Vec<Lexer>) -> Box<dyn Validator>{
        let list = LinkedList::new();

        return Box::new(ValueData::LinkedList(list));
    }
}

pub use linked_list::*;
