mod types {
    use core::fmt;

    use crate::{aly::Aly, lexer::Lexer, native::{create_object::Object, vector::Vector}, validators::{conversor_to_bool, conversor_to_float, conversor_to_int, is_bool, is_num, numeric::{is_float, is_int}, str::{is_any_str, put_quoted_str, remove_quoted_str}}};

    pub enum Type {
        Int,
        Float,
        String,
        Bool,
        Vec,
        Obj,
        None,
        Struct(String),
        Model(String),
        Function,   
        NativeFunction,
    }

    impl fmt::Display for Type {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            let res = match self {
                Type::Int => "int",
                Type::Float => "float",
                Type::String => "string",
                Type::Bool => "boolean",
                Type::Vec => "vector",
                Type::None => "None",
                Type::Obj => "obj",
                Type::Struct(_) => "Struct",
                Type::Model(_) => "Model",
                Type::Function => "Function",
                Type::NativeFunction => "NativeFunction",
            };

            write!(f, "{}", res)
        }
    }

    pub enum ValueData {
        Int(i32),
        Float(f32),
        String(String),
        Bool(bool),
        Vec(Vector),
        Object(Object),
        Function(Vec<Lexer>),   
        NativeFunction(fn(&mut Aly, String) -> Box<dyn Validator>),
    }

    impl ValueData {
        pub fn to_string(&self, qt: bool) -> String {
            match self {
                ValueData::Int(int) => int.to_string(),
                ValueData::Float(f) => f.to_string(),
                ValueData::String(s) => {
                    if qt {
                        put_quoted_str(s.to_string())
                    } else {
                        s.to_string()
                    }
                },
                ValueData::Bool(bool)  => bool.to_string(),
                ValueData::Vec(vec) => vec.to_json(0),
                ValueData::Function(_) => "Function".to_owned(),
                ValueData::NativeFunction(_) => "NativeFunction".to_owned(),
                ValueData::Object(obj) => obj.to_string(false),
            }
        }

        fn extract(&self) -> Box<dyn Validator> {
            match self {
                ValueData::Int(i) => Box::new(i.to_string()),
                ValueData::Float(f) => Box::new(f.to_string()),
                ValueData::String(s) => Box::new(s.to_string()),
                ValueData::Bool(b) => Box::new(b.to_string()),
                ValueData::Object(o) => Box::new(o.clone()),
                ValueData::Function(_) |
                ValueData::NativeFunction(_) => Box::new("Function".to_string()),
                ValueData::Vec(_) => Box::new("Vec".to_string()),
            }
        } 

        pub fn get_prop(&self, is_mut: bool, props: Vec<Lexer>) -> Box<dyn Validator> {
            let mut props_vec = vec![];
            
            let data = self.extract();
            let line = props[0].line;

            for prop in props {
                if prop.literal != "." {
                    props_vec.push(prop.literal.clone())
                }
            }

            return Box::new(put_quoted_str(if props_vec.len() == 1 {
                self.prop(&props_vec[0], is_mut, line, data).to_string(false)
            } else {
                let mut tmp = self.prop(props_vec[0].as_str(), is_mut, line, data);

                for prop in &props_vec[1..] {
                    tmp = match tmp {
                        ValueData::String(ref str) => {
                            if is_any_str(&str) {
                                tmp
                            } else {
                                ValueData::String(put_quoted_str(tmp.to_string(false)))
                            }
                        },
                        _ => tmp
                    };

                    let boxed_tmp: Box<dyn Validator> = Box::new(tmp);
                    
                    tmp = self.prop(prop.as_str(), is_mut, line, boxed_tmp);
                }
                
                tmp.to_string(false)                            
            }));
        }

        pub fn prop(&self, prop: &str, is_mut: bool, line: i32, data: Box<dyn Validator>) -> ValueData {
            let (type_data, value) = data.valid();

        
            match prop {
                "type" => ValueData::String(type_data.to_string()),
                "len" => ValueData::Int(value.to_string(false).len().try_into().unwrap()),
                "is_mut" => ValueData::Bool(is_mut),
                "to_int" => {
                    let mut val = value.to_string(false).trim().to_string();
                    if val.contains(".") {
                        val = val.split(".").collect::<Vec<&str>>()[0].trim().to_owned();
                    }
                    ValueData::Int(conversor_to_int(val))
                },
                "to_str" => {
                    ValueData::String(match self {
                        ValueData::Object(obj) => obj.to_json(0),
                        _ => put_quoted_str(self.to_string(false))
                    })
                },
                "to_float" => {
                    let mut val = value.to_string(false).trim().to_owned().replace(" ", "");
                    if !val.contains(".") {
                        val = format!("{}.00", val);
                    }
                    ValueData::Float(conversor_to_float(val))
                },
                item => {
                    if is_num(item) || item == "_" {
                        match type_data {
                            Type::String => {
                                let x = conversor_to_int(item.to_owned());
                                let val = value.to_string(false);
                                if x <= -1 {
                                    return ValueData::String(val.chars().last().unwrap().to_string());
                                }
                                return ValueData::String(val.chars().nth(x.try_into().unwrap()).unwrap().to_string());
                            },
                            _ => {
                                match self {
                                    ValueData::Vec(vec) => {
                                        let x = conversor_to_int(item.to_owned());

                                        vec.get_index(x.try_into().unwrap())
                                    },
                                    _ => panic!(
                                        "Error on line {}: The type {} is not indexable", 
                                        line,
                                        type_data
                                    )
                                }
                            }
                        }
                    } else if type_data.to_string() == "obj" {
                        match value {
                            ValueData::Object(obj) => obj.get_item(prop.to_owned()),
                            _ => todo!()
                        }
                    } else {
                        ValueData::String("None".to_owned())
                    }
                }
            }
        }        

    }

    impl Clone for ValueData {
        fn clone(&self) -> Self {
            match self {
                ValueData::Int(d) => ValueData::Int(d.clone()),
                ValueData::Float(d) => ValueData::Float(d.clone()),
                ValueData::String(d) => ValueData::String(d.clone()),
                ValueData::Bool(d) => ValueData::Bool(d.clone()),
                ValueData::Vec(d) => ValueData::Vec(d.clone()),
                ValueData::Function(fun) => ValueData::Function(fun.clone()),
                ValueData::NativeFunction(fun) => ValueData::NativeFunction(fun.clone()),
                ValueData::Object(obj) => ValueData::Object(obj.clone()),
            }
        }
    }

    // Validator value
    
    pub trait Validator {
        fn valid(&self) -> (Type, ValueData);
    }

    impl Validator for () {
        fn valid(&self) -> (Type, ValueData) {            
            (Type::None, ValueData::String("None".to_owned())) 
        }
    }

    impl Validator for Object {
        fn valid(&self) -> (Type, ValueData) {            
            (Type::Obj, ValueData::Object(self.clone()))
        }
    }

    impl Validator for ValueData {
        fn valid(&self) -> (Type, ValueData) {            
            match self {
                ValueData::Int(int) => (Type::Int, ValueData::Int(*int)),
                ValueData::Float(float) => (Type::Float, ValueData::Float(*float)),
                ValueData::String(str) => {
                    str.valid()
                },
                ValueData::Bool(bool) => (Type::Bool, ValueData::Bool(*bool)),
                ValueData::Vec(vec) => (Type::Vec, ValueData::Vec(vec.clone())),
                ValueData::Object(obj) => (Type::Obj, ValueData::Object(obj.clone())),
                ValueData::Function(fun) => (Type::Function, ValueData::Function(fun.clone())),
                ValueData::NativeFunction(fun) => (Type::NativeFunction, ValueData::NativeFunction(*fun)),
            }
        }
    }

    
    impl Validator for fn(&mut Aly, String) -> Box<dyn Validator> {
        fn valid(&self) -> (Type, ValueData) {
            (Type::NativeFunction, ValueData::NativeFunction(*self)) 
        }
    }

    impl Validator for bool {
        fn valid(&self) -> (Type, ValueData) {
            (Type::Bool, ValueData::Bool(*self))
        }
    }

    impl Validator for String {
        fn valid(&self) -> (Type, ValueData) {
            if is_any_str(self) {
                (Type::String, ValueData::String(remove_quoted_str(self.clone()))) 
            } else if is_bool(&self) {
                (Type::Bool, ValueData::Bool(conversor_to_bool(self.clone())))
            } else if is_int(self) {
                (Type::Int, ValueData::Int(conversor_to_int(self.clone())))
            } else if is_float(self) {
                (Type::Int, ValueData::Float(conversor_to_float(self.clone())))
            } else {
                (Type::None, ValueData::String("None".to_owned())) 
            }
        }
    }
    

    pub fn is_valid_data<T: Validator>(data: T) -> (Type, ValueData) {
        data.valid()
    }
}

pub use types::*;