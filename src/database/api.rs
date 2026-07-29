use crate::vm::value::Value;

use super::driver::DatabaseConfig;
use super::registry::{self, register_driver, with_driver};

fn extract_str<'a>(args: &'a [Value], index: usize) -> Option<&'a str> {
    args.get(index).and_then(|v| {
        if let Value::Str(s) = v { Some(s.as_str()) } else { None }
    })
}

fn db_open(args: &[Value]) -> Value {
    let uri = match extract_str(args, 0) {
        Some(s) => s,
        None => return Value::Str("error: db.open() requires a URI argument".to_string()),
    };

    let alias = extract_str(args, 1).unwrap_or("default");

    let config = match DatabaseConfig::from_uri(uri) {
        Ok(c) => c,
        Err(e) => return Value::Str(format!("error: {}", e)),
    };

    let scheme = config.scheme.clone();

    match open_driver(&scheme) {
        Ok(mut driver) => {
            match driver.connect(&config) {
                Ok(()) => {
                    register_driver(alias, driver);
                    Value::Str(format!("connected:{}", alias))
                }
                Err(e) => Value::Str(format!("error: connection failed: {}", e)),
            }
        }
        Err(e) => Value::Str(format!("error: {}", e)),
    }
}

fn db_query(args: &[Value]) -> Value {
    let sql = match extract_str(args, 0) {
        Some(s) => s,
        None => return Value::Str("error: db.query() requires an SQL string".to_string()),
    };
    let alias = extract_str(args, 1).unwrap_or("default");

    match with_driver(alias, |driver| driver.query(sql)) {
        Ok(val) => val,
        Err(e) => Value::Str(format!("error: {}", e)),
    }
}

fn db_execute(args: &[Value]) -> Value {
    let sql = match extract_str(args, 0) {
        Some(s) => s,
        None => return Value::Str("error: db.execute() requires an SQL string".to_string()),
    };
    let alias = extract_str(args, 1).unwrap_or("default");

    match with_driver(alias, |driver| driver.execute(sql)) {
        Ok(val) => val,
        Err(e) => Value::Str(format!("error: {}", e)),
    }
}

fn db_close(args: &[Value]) -> Value {
    let alias = extract_str(args, 0).unwrap_or("default");

    let result = with_driver(alias, |driver| driver.close());
    registry::remove_connection(alias);
    match result {
        Ok(()) => Value::Nil,
        Err(e) => Value::Str(format!("error: {}", e)),
    }
}

fn db_drivers(_args: &[Value]) -> Value {
    let drivers = registry::list_drivers();
    let items: Vec<Value> = drivers.into_iter().map(Value::Str).collect();
    Value::Vec(std::rc::Rc::new(std::cell::RefCell::new(items)))
}

fn db_use(args: &[Value]) -> Value {
    let alias = match extract_str(args, 0) {
        Some(s) => s,
        None => return Value::Str("error: db.use() requires a connection name".to_string()),
    };
    if registry::has_driver(alias) {
        Value::Str(format!("using:{}", alias))
    } else {
        Value::Str(format!("error: no connection named '{}'", alias))
    }
}

pub fn vm_native_functions() -> Vec<(String, Value)> {
    vec![
        ("db.open".to_string(), Value::Native(db_open, "db.open")),
        ("db.query".to_string(), Value::Native(db_query, "db.query")),
        ("db.execute".to_string(), Value::Native(db_execute, "db.execute")),
        ("db.close".to_string(), Value::Native(db_close, "db.close")),
        ("db.drivers".to_string(), Value::Native(db_drivers, "db.drivers")),
        ("db.use".to_string(), Value::Native(db_use, "db.use")),
    ]
}

fn open_driver(scheme: &str) -> Result<Box<dyn super::driver::DatabaseDriver>, String> {
    let builtin: Option<Result<Box<dyn super::driver::DatabaseDriver>, String>> = match scheme {
        "sqlite" => Some(Ok(Box::new(crate::stdlib::database::sqlite::SQLiteDriver::new()))),
        "postgres" | "postgresql" => Some(Ok(Box::new(crate::stdlib::database::postgres::PostgresDriver::new()))),
        "mysql" | "mariadb" => Some(Ok(Box::new(crate::stdlib::database::mysql::MySqlDriver::new()))),
        "redis" => Some(Ok(Box::new(crate::stdlib::database::redis::RedisDriver::new()))),
        _ => None,
    };

    if let Some(result) = builtin {
        return result;
    }

    match super::plugin::create_from_factory(scheme, "") {
        Some(Ok(driver)) => Ok(driver),
        Some(Err(e)) => Err(format!("factory error for '{}': {}", scheme, e)),
        None => {
            let available = super::plugin::list_factories();
            let mut msg = format!("unsupported database scheme '{}'. Built-in: sqlite, postgres, mysql, redis", scheme);
            if !available.is_empty() {
                msg.push_str(&format!(". Plugins: {}", available.join(", ")));
            }
            Err(msg)
        }
    }
}
