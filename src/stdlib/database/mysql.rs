use crate::vm::value::Value;
use crate::database::driver::{DatabaseDriver, DatabaseConfig};

pub struct MySqlDriver {
    config: Option<DatabaseConfig>,
    connected: bool,
}

impl MySqlDriver {
    pub fn new() -> Self {
        MySqlDriver {
            config: None,
            connected: false,
        }
    }
}

impl DatabaseDriver for MySqlDriver {
    fn name(&self) -> &'static str {
        "mysql"
    }

    fn connect(&mut self, config: &DatabaseConfig) -> Result<(), String> {
        self.config = Some(config.clone());
        self.connected = true;
        Ok(())
    }

    fn query(&mut self, sql: &str) -> Result<Value, String> {
        if !self.connected {
            return Err("Not connected. Call db.open() first.".to_string());
        }
        let upper = sql.to_uppercase().trim().to_string();
        if upper.starts_with("SELECT") || upper.starts_with("SHOW") {
            let preview = if sql.len() > 40 { &sql[..40] } else { sql };
            Ok(Value::Str(format!("mysql_query_ok: {}", preview)))
        } else {
            Ok(Value::Int(0))
        }
    }

    fn execute(&mut self, _sql: &str) -> Result<Value, String> {
        if !self.connected {
            return Err("Not connected. Call db.open() first.".to_string());
        }
        Ok(Value::Int(1))
    }

    fn close(&mut self) -> Result<(), String> {
        self.connected = false;
        self.config = None;
        Ok(())
    }
}


unsafe impl Send for MySqlDriver {}
unsafe impl Sync for MySqlDriver {}
