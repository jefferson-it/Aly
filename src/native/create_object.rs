mod create_object {
    use std::{collections::HashMap, vec};

    use linked_hash_map::LinkedHashMap;

    use crate::{aly::Aly, lexer::Lexer, native::{process_value, types::{Validator, ValueData}}, tokens::Tokens, validators::structures::{is_closed_brace, is_opened_brace}};

    #[derive(Clone)]
    pub struct Object {
        literal: LinkedHashMap<String, ValueData>
    }

    impl Object {
        pub fn new(props: Vec<String>, draft: HashMap<String, ValueData>) -> Object {
            let mut final_data = LinkedHashMap::new();
            for prop in &props {
                if let Some(item) = draft.get(prop) {
                    let (_, value_final) = item.valid();

                    final_data.insert(prop.to_string(), value_final);
                }
            }

            Object {
                literal: final_data
            }
        }
        // getter
        pub fn get_item(&self, prop: String) -> ValueData {
            if let Some(res) = self.literal.get(&prop) {
                return res.clone()
            } else {
                ValueData::String("None".to_owned())
            }
        }

        // Printer
        pub fn to_string(&self, json: bool) -> String {
            let mut string = String::from("Object (#data");

            if json {
                string = string.replace("#data", &self.to_json(0));
            } else {
                string = string.replace("#data", &self.to_object(0));
            }

            string.push_str(")");

            string
        }

        pub fn to_object(&self, child: i32) -> String {
            let mut obj = String::from("{\n");
            let space_prop = "   ".repeat(child.max(1) as usize);
            
            for (name, data) in &self.literal {
                let info = match data {
                    ValueData::Object(obj) => {
                        let res = obj.to_object(child + 1);

                        format!("{space_prop}{}: {}\n", name, res)
                    },
                    _ => format!("{}{}: {}\n", space_prop.repeat((child + 1).try_into().unwrap()), name, data.to_string(true)),
                };

                obj.push_str(&info);
            }

            if child > 0 {
                obj.push_str(&format!("{}", space_prop));
                obj.push_str("}");
            } else {
                obj.push_str("}");
            } 

            return obj.to_owned();
        }
    

        pub fn to_json(&self, child: i32) -> String {
            let mut json = String::from("{\n");
            let space_prop = "   ".repeat(child.max(1) as usize);
            
            for (name, data) in &self.literal {
                let info = match data {
                    ValueData::Object(obj) => {
                        let res = obj.to_json(child + 1);

                        format!("{space_prop}\"{}\": {}\n", name, res)
                    },
                    _ => format!("{}\"{}\": {}\n", space_prop.repeat((child + 1).try_into().unwrap()), name, data.to_string(true)),
                };

                json.push_str(&info);
            }

            if child > 0 {
                json.push_str(&format!("{}", space_prop));
                json.push_str("}");
            } else {
                json.push_str("}");
            } 

            return json.to_owned();
        }
    }

    pub fn create_object(run: &mut Aly, lexers: Vec<Lexer>) -> Box<dyn Validator> {
        let mut another_obj = 0;
        let mut to_process  = vec![];
        let mut props: Vec<Lexer> = vec![];
        let mut datas: HashMap<String, Vec<Lexer>> = HashMap::new();
        let mut final_result: HashMap<String, ValueData> = HashMap::new();
       
        for lex in lexers[1..lexers.len() - 1].to_vec() {
            if lex.token.id() == Tokens::Comma.id() {
                to_process.push(lex)
            } else if lex.token.id() == Tokens::Colon.id() {
                let ind = to_process.len() - 1;

                to_process[ind].literal.push_str(&Tokens::Colon.literal());
           
            } else {
                to_process.push(lex)
            }
        }

        for mut item in to_process {

            if is_opened_brace(item.token.clone()) {
                another_obj += 1;
            } else if is_closed_brace(item.token.clone()) {
                another_obj -= 1;
                
                if another_obj == 0 {
                }
            }


            if another_obj > 0 {
                let prop = props[props.len() - 1].literal.clone();
                push_data(item, prop, &mut datas);
                continue;
            }

            if item.literal.contains(&Tokens::Colon.literal()) {
                if let Some(ind) = item.literal.find(":") {
                    let mut res = item.literal.clone();
                    
                    res.replace_range(ind..ind + 1, "");
    
                    item.literal = res;
    
                }

                props.push(item)
            } else {
                let prop = props[props.len() - 1].literal.clone();
                
                if item.literal.contains(&Tokens::Comma.literal()) {
                    if let Some(ind) = item.literal.find(",") {
                        let mut res = item.literal.clone();
                        
                        res.replace_range(ind..ind + 1, "");
        
                        item.literal = res.clone();
                        
                    }

                    push_data(item, prop.clone(), &mut datas);
                } else {
                    push_data(item, prop, &mut datas);
                }
            }
        }

        fn push_data(item: Lexer, prop: String, datas: &mut HashMap<String, Vec<Lexer>>) {
            if item.token.id() == Tokens::Comma.id() {
                return;
            }

            if let Some(item_on_data) = datas.get_mut(&prop) {
                item_on_data.push(item.clone())
            } else {
                datas.insert(prop,  vec![item.clone()]);
            }
        }

        let props_str: Vec<String> = props.iter().map(|f| f.literal.clone()).collect();

        for prop in &props_str {

            if let Some(item) = datas.get(prop) {

                if item.iter().find(|t| t.token.id() == "this").is_some() {
                    let mut new_exp = vec![];
                    let mut is_this = false;
    
                    for exp in item {
                        if exp.token.id() == Tokens::This.id() {
                            is_this = true;
                        } else {
                            if is_this {
                                if let Some(res) = final_result.get(&exp.literal) {
                                    let lexer = Lexer::new(
                                        Tokens::Value, 
                                        res.to_string(false), 
                                        exp.line
                                    );
    
                                    new_exp.push(lexer.clone());

                                    continue;
                                } 
    
                                is_this = false;
                            }
    
                            new_exp.push(exp.clone());
                        }
                    }

                    let value = process_value(run, new_exp);
    
                    final_result.insert(prop.to_string(), value);
                    
                    continue;
                }
    
                let value = process_value(run, item.clone());
    
                final_result.insert(prop.to_string(), value);
            } else {
                final_result.insert(prop.to_string(), ValueData::String(String::from("None")));
            }
      
        }

        let object = Object::new(props_str.clone(), final_result);

        Box::new(ValueData::Object(object))
    }

}

pub use create_object::*;