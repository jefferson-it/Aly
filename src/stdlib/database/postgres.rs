use std::collections::HashMap;
use crate::vm::value::Value;
use crate::database::driver::{DatabaseDriver, DatabaseConfig};

pub struct PostgresDriver {
    config: Option<DatabaseConfig>,
    connected: bool,
    cache: HashMap<String, Vec<HashMap<String, Value>>>,
}

impl PostgresDriver {
    pub fn new() -> Self {
        PostgresDriver {
            config: None,
            connected: false,
            cache: HashMap::new(),
        }
    }
}

impl DatabaseDriver for PostgresDriver {
    fn name(&self) -> &'static str {
        "postgres"
    }

    fn connect(&mut self, config: &DatabaseConfig) -> Result<(), String> {
        self.config = Some(config.clone());
        self.connected = true;
        self.cache.clear();
        Ok(())
    }

    fn query(&mut self, sql: &str) -> Result<Value, String> {
        if !self.connected {
            return Err("Not connected. Call db.open() first.".to_string());
        }
        let key = sql.to_string();
        if let Some(rows) = self.cache.get(&key) {
            let vals: Vec<Value> = rows.iter().map(|r| Value::Map(r.iter().map(|(k, v)| (Value::Str(k.clone()), v.clone())).collect())).collect();
            return Ok(Value::Vec(std::rc::Rc::new(std::cell::RefCell::new(vals))));
        }
        let mock_rows = vec![
            map_from_pairs(vec![("result", Value::Str("mock_query_ok".to_string()))])
        ];
        self.cache.insert(key, mock_rows.clone());
        let vals: Vec<Value> = mock_rows.into_iter().map(|r| Value::Map(r.into_iter().map(|(k, v)| (Value::Str(k), v)).collect())).collect();
        Ok(Value::Vec(std::rc::Rc::new(std::cell::RefCell::new(vals))))
    }

    fn execute(&mut self, sql: &str) -> Result<Value, String> {
        if !self.connected {
            return Err("Not connected. Call db.open() first.".to_string());
        }
        let upper = sql.to_uppercase().trim().to_string();
        if upper.starts_with("INSERT") || upper.starts_with("UPDATE") || upper.starts_with("DELETE") {
            Ok(Value::Int(1))
        } else {
            self.query(sql)
        }
    }

    fn close(&mut self) -> Result<(), String> {
        self.connected = false;
        self.cache.clear();
        self.config = None;
        Ok(())
    }
}

fn map_from_pairs(pairs: Vec<(&str, Value)>) -> HashMap<String, Value> {
    let mut m = HashMap::new();
    for (k, v) in pairs {
        m.insert(k.to_string(), v);
    }
    m
}


unsafe impl Send for PostgresDriver {}
unsafe impl Sync for PostgresDriver {}
