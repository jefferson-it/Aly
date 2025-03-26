mod vector {
    use crate::{aly::get_runtime, lexer::Lexer, native::{process_value, types::{Validator, ValueData}}, runtime::interpreter::exec, tokens::Tokens, validators::structures::{is_close, is_opened}};


    #[derive(Clone)]
    pub struct Vector(Vec<ValueData>);

    impl Vector {
        pub fn new(value: Vec<ValueData>) -> Vector {
            let mut final_data = vec![];

            for item in &value {
                let (_, value_final) = item.valid();

                final_data.push(value_final);
            }

            Vector(final_data)
        }

        pub fn get_index(&self, index: usize) -> ValueData {
            if let Some(res) = self.0.get(index) {
                return res.clone();
            } 
    
            ValueData::String(String::from("None"))
        }

        pub fn to_string(&self, json: bool) -> String {
            let mut string = String::from("Vector (#data");

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
                let info = match data {
                    ValueData::Object(obj) => {
                        let res = obj.to_json(child + 1);

                        format!("{space_prop}{}\n", res)
                    },
                    ValueData::Vec(vec) => {
                        let res = vec.to_json(child + 1);

                        format!("{space_prop}{}\n", res)
                    },
                    _ => format!("{}{}\n", space_prop.repeat((child + 1).try_into().unwrap()), data.to_string(true)),
                };

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

    // Pub create

    pub fn create_array(lexer: Vec<Lexer>) -> Box<dyn Validator>{
        let mut another = 0;
        let mut new_vec = vec![];
        let mut values = vec![];
        // let run = get_runtime();

        for item in lexer[1..lexer.len() - 1].to_vec() {
            if item.token.id() == Tokens::Comma.id() { continue; }

            if is_opened(item.token.clone()) {
                another += 1;
                new_vec.push(item.clone());
                continue;
            } else if is_close(item.token.clone()){
                another -= 1;
                new_vec.push(item.clone());


                if another == 0 {
                    let mut res: Box<dyn Validator> = Box::new(String::new());

                    exec(&mut new_vec, &mut res);
    
                    new_vec.clear();
    
                    values.push(res.valid().1);
                }

                continue;
            }

            if another > 0 {
                new_vec.push(item.clone());
                continue;
            }

            let val = process_value(vec![item.clone()]);

            values.push(val.clone());
        } 

        let arr = Vector::new(values);

        return Box::new(ValueData::Vec(arr));
    }
}

pub use vector::*;