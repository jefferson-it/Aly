mod json_mod {
    use std::collections::HashMap;

    use crate::native::create_object::Object;
    use crate::native::types::{Validator, ValueData};
    use crate::native::vector::Vector;
    use crate::validators::str::remove_quoted_str;

    pub fn json_parse(x: String) -> Box<dyn Validator> {
        let input = remove_quoted_str(x.trim().to_string());
        let v: serde_json::Value = match serde_json::from_str(&input) {
            Ok(v) => v,
            Err(e) => panic!("json.parse error: {}", e),
        };
        Box::new(json_to_aly(v))
    }

    pub fn json_stringify(x: String) -> Box<dyn Validator> {
        let val = if x.trim().is_empty() {
            serde_json::Value::Null
        } else {
            match serde_json::from_str(&x) {
                Ok(v) => v,
                Err(_) => {
                    let wrapped = format!("\"{}\"", x);
                    match serde_json::from_str(&wrapped) {
                        Ok(v) => v,
                        Err(e) => panic!("json.stringify error: {}", e),
                    }
                }
            }
        };
        let result = serde_json::to_string_pretty(&val).unwrap_or_default();
        Box::new(crate::validators::str::put_quoted_str(result))
    }

    fn json_to_aly(v: serde_json::Value) -> ValueData {
        match v {
            serde_json::Value::Null => ValueData::String("None".to_owned()),
            serde_json::Value::Bool(b) => ValueData::Bool(b),
            serde_json::Value::Number(n) => {
                if let Some(i) = n.as_i64() {
                    ValueData::Int(i)
                } else if let Some(f) = n.as_f64() {
                    ValueData::Float(f)
                } else {
                    ValueData::String("None".to_owned())
                }
            }
            serde_json::Value::String(s) => ValueData::String(s),
            serde_json::Value::Array(arr) => {
                let items: Vec<ValueData> = arr.into_iter().map(json_to_aly).collect();
                ValueData::Vec(Vector::new(items))
            }
            serde_json::Value::Object(obj) => {
                let mut result = Object::new(vec![], HashMap::new());
                for (k, v) in obj {
                    result.set_item(k, json_to_aly(v));
                }
                ValueData::Object(result)
            }
        }
    }
}

pub use json_mod::*;
