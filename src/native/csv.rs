mod csv_mod {
    use crate::native::create_object::Object;
    use crate::native::types::{Validator, ValueData};
    use crate::native::vector::Vector;
    use crate::validators::str::remove_quoted_str;
    use crate::native::std::split_args;

    // csv.parse(string) -> vector of objects (header row -> property names)
    pub fn csv_parse(x: String) -> Box<dyn Validator> {
        use std::collections::HashMap;

        let input = if x.trim().starts_with('"') || x.trim().starts_with('\'') {
            remove_quoted_str(x.trim().to_string())
        } else {
            x.trim().to_string()
        };

        let mut reader = csv::ReaderBuilder::new()
            .flexible(true)
            .from_reader(input.as_bytes());

        let headers: Vec<String> = match reader.headers() {
            Ok(h) => h.iter().map(|s| s.to_string()).collect(),
            Err(e) => {
                eprintln!("RuntimeError [csv.parse]: cabeçalho inválido: {}", e);
                return Box::new(ValueData::Vec(Vector::new(vec![])));
            }
        };

        let mut rows = vec![];
        for result in reader.records() {
            let record = match result {
                Ok(r) => r,
                Err(e) => {
                    eprintln!("RuntimeError [csv.parse]: linha inválida: {}", e);
                    continue;
                }
            };

            let mut obj = Object::new(vec![], HashMap::new());
            for (i, field) in record.iter().enumerate() {
                let key = headers.get(i).cloned().unwrap_or_else(|| format!("_{}", i));
                obj.set_item(key, ValueData::String(field.to_string()));
            }
            rows.push(ValueData::Object(obj));
        }

        Box::new(ValueData::Vec(Vector::new(rows)))
    }
}

pub use csv_mod::*;
