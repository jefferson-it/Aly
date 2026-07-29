use std::collections::HashMap;
use crate::vm::value::Value;

pub trait DatabaseDriver: Send {
    fn name(&self) -> &'static str;

    fn connect(&mut self, config: &DatabaseConfig) -> Result<(), String>;

    fn query(&mut self, sql: &str) -> Result<Value, String>;

    fn execute(&mut self, sql: &str) -> Result<Value, String>;

    fn close(&mut self) -> Result<(), String>;
}

#[derive(Clone, Debug)]
pub struct DatabaseConfig {
    pub scheme: String,
    pub host: String,
    pub port: u16,
    pub database: String,
    pub user: String,
    pub password: String,
    pub options: HashMap<String, String>,
    pub raw_uri: String,
}

impl DatabaseConfig {
    pub fn from_uri(uri: &str) -> Result<Self, String> {
        let raw_uri = uri.to_string();

        if !uri.contains("://") {
            if uri.starts_with(":memory:") || uri.contains('/') || uri.ends_with(".db") {
                return Ok(DatabaseConfig {
                    scheme: "sqlite".to_string(),
                    host: String::new(),
                    port: 0,
                    database: uri.to_string(),
                    user: String::new(),
                    password: String::new(),
                    options: HashMap::new(),
                    raw_uri,
                });
            }
            return Err(format!("Invalid database URI: {}", uri));
        }

        let (scheme, rest) = uri.split_once("://").unwrap();
        let scheme = scheme.to_lowercase();

        let (userinfo, host_and_db) = if let Some(at_pos) = rest.rfind('@') {
            let userinfo_part = &rest[..at_pos];
            let rest_part = &rest[at_pos + 1..];
            (Some(userinfo_part), rest_part)
        } else {
            (None, rest)
        };

        let (host_part, db_part) = if let Some(slash_pos) = host_and_db.find('/') {
            (&host_and_db[..slash_pos], Some(&host_and_db[slash_pos + 1..]))
        } else {
            (host_and_db, None)
        };

        let (host, port) = if let Some(colon_pos) = host_part.rfind(':') {
            let h = &host_part[..colon_pos];
            let p: u16 = host_part[colon_pos + 1..].parse().unwrap_or(match scheme.as_str() {
                "postgres" | "postgresql" => 5432,
                "mysql" | "mariadb" => 3306,
                "redis" => 6379,
                "mongodb" => 27017,
                _ => 0,
            });
            (h.to_string(), p)
        } else {
            (host_part.to_string(), match scheme.as_str() {
                "postgres" | "postgresql" => 5432,
                "mysql" | "mariadb" => 3306,
                "redis" => 6379,
                "mongodb" => 27017,
                _ => 0,
            })
        };

        let (user, password) = if let Some(ui) = userinfo {
            if let Some(colon_pos) = ui.find(':') {
                (ui[..colon_pos].to_string(), ui[colon_pos + 1..].to_string())
            } else {
                (ui.to_string(), String::new())
            }
        } else {
            (String::new(), String::new())
        };

        let (database, options_str) = if let Some(db) = db_part {
            if let Some(query_pos) = db.find('?') {
                (db[..query_pos].to_string(), Some(&db[query_pos + 1..]))
            } else {
                (db.to_string(), None)
            }
        } else {
            (String::new(), None)
        };

        let mut options = HashMap::new();
        if let Some(qs) = options_str {
            for pair in qs.split('&') {
                if let Some(eq_pos) = pair.find('=') {
                    options.insert(pair[..eq_pos].to_string(), pair[eq_pos + 1..].to_string());
                }
            }
        }

        Ok(DatabaseConfig {
            scheme,
            host,
            port,
            database,
            user,
            password,
            options,
            raw_uri,
        })
    }
}
