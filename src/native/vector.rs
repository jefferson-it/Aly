mod vector {
    use crate::{lexer::Lexer, native::{process_value, types::{Validator, ValueData}}, runtime::interpreter::exec, tokens::Tokens, validators::structures::{is_close, is_opened}};


    #[derive(Clone)]
    pub struct Vector(Vec<ValueData>);

    impl PartialEq for Vector {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }

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

        pub fn set_index(&mut self, index: usize, value: ValueData) {
            if index < self.0.len() {
                self.0[index] = value;
            }
        }

        pub fn to_string(&self, json: bool) -> String {
            let mut string = String::from("Vector (#data");

            if json {
                string = string.replace("#data", &self.to_json(0));
            }

            string.push_str(")");

            string
        }

        pub fn map<F>(&self, f: F) -> Vector 
        where F: Fn(&ValueData) -> ValueData {
            Vector(self.0.iter().map(f).collect())
        }

        pub fn filter<F>(&self, f: F) -> Vector 
        where F: Fn(&ValueData) -> bool {
            Vector(self.0.iter().filter(|x| f(x)).cloned().collect())
        }

        pub fn reduce<F, T>(&self, initial: T, f: F) -> T
        where F: Fn(T, &ValueData) -> T {
            self.0.iter().fold(initial, f)
        }

        pub fn each<F>(&self, f: F) 
        where F: Fn(&ValueData) {
            self.0.iter().for_each(f);
        }

        pub fn find<F>(&self, f: F) -> Option<&ValueData>
        where F: Fn(&ValueData) -> bool {
            self.0.iter().find(|x| f(x))
        }

        pub fn len(&self) -> usize {
            self.0.len()
        }

        pub fn get_elements(&self) -> &Vec<ValueData> {
            &self.0
        }

        pub fn sort(&mut self) {
            // Note: Simplistic sort, assuming ValueData supports ordering.
            // For complex types, this may need custom comparison.
            self.0.sort_by(|a, b| {
                a.to_string(false).partial_cmp(&b.to_string(false)).unwrap()
            });
        }

        pub fn slice(&self, start: usize, end: usize) -> Vector {
            Vector(self.0[start..end].to_vec())
        }

        pub fn flat(&self) -> Vector {
            let mut flattened = Vec::new();
            for item in &self.0 {
                match item {
                    ValueData::Vec(v) => flattened.extend(v.0.clone()),
                    _ => flattened.push(item.clone()),
                }
            }
            Vector(flattened)
        }

        pub fn concat(&mut self, other: Vector) {
            self.0.extend(other.0);
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