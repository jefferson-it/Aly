use std::collections::HashMap;
use crate::vm::value::Value;
use crate::database::driver::{DatabaseDriver, DatabaseConfig};

#[derive(Clone)]
struct ColumnDef {
    name: String,
    col_type: ColumnType,
}

#[derive(Clone)]
enum ColumnType {
    Integer,
    Text,
    Real,
    Blob,
}

#[derive(Clone)]
struct TableSchema {
    name: String,
    columns: Vec<ColumnDef>,
}

pub struct SQLiteDriver {
    tables: HashMap<String, TableSchema>,
    rows: HashMap<String, Vec<HashMap<String, Value>>>,
    connected: bool,
}

impl SQLiteDriver {
    pub fn new() -> Self {
        SQLiteDriver {
            tables: HashMap::new(),
            rows: HashMap::new(),
            connected: false,
        }
    }
}

impl DatabaseDriver for SQLiteDriver {
    fn name(&self) -> &'static str {
        "sqlite"
    }

    fn connect(&mut self, config: &DatabaseConfig) -> Result<(), String> {
        self.tables.clear();
        self.rows.clear();
        self.connected = true;
        Ok(())
    }

    fn query(&mut self, sql: &str) -> Result<Value, String> {
        if !self.connected {
            return Err("Not connected. Call db.open() first.".to_string());
        }
        self.execute_internal(sql)
    }

    fn execute(&mut self, sql: &str) -> Result<Value, String> {
        if !self.connected {
            return Err("Not connected. Call db.open() first.".to_string());
        }
        self.execute_internal(sql)
    }

    fn close(&mut self) -> Result<(), String> {
        self.tables.clear();
        self.rows.clear();
        self.connected = false;
        Ok(())
    }
}

impl SQLiteDriver {
    fn execute_internal(&mut self, sql: &str) -> Result<Value, String> {
        let sql = sql.trim();
        let upper = sql.to_uppercase();
        if upper.starts_with("CREATE TABLE") {
            self.execute_create(sql)
        } else if upper.starts_with("INSERT INTO") {
            self.execute_insert(sql)
        } else if upper.starts_with("SELECT") {
            self.execute_select(sql)
        } else if upper.starts_with("DROP TABLE") {
            self.execute_drop(sql)
        } else {
            Err(format!("Unsupported SQL: {}", sql))
        }
    }

    fn execute_create(&mut self, sql: &str) -> Result<Value, String> {
        let inner = sql.trim_start_matches("CREATE TABLE")
            .trim_start_matches("create table")
            .trim();
        let paren = inner.find('(').ok_or_else(|| "Missing (".to_string())?;
        let name = inner[..paren].trim().to_string();
        let cols_str = &inner[paren + 1..inner.rfind(')').ok_or_else(|| "Missing )".to_string())?];
        let mut columns = Vec::new();
        for col_def in cols_str.split(',') {
            let parts: Vec<&str> = col_def.trim().split_whitespace().collect();
            if parts.len() < 2 { continue; }
            let col_name = parts[0].trim().to_string();
            let col_type = match parts[1].to_uppercase().as_str() {
                "INT" | "INTEGER" | "BIGINT" => ColumnType::Integer,
                "REAL" | "FLOAT" | "DOUBLE" => ColumnType::Real,
                "BLOB" => ColumnType::Blob,
                _ => ColumnType::Text,
            };
            columns.push(ColumnDef { name: col_name, col_type });
        }
        let schema = TableSchema { name: name.clone(), columns };
        self.tables.insert(name.clone(), schema);
        self.rows.insert(name, Vec::new());
        Ok(Value::Int(0))
    }

    fn execute_insert(&mut self, sql: &str) -> Result<Value, String> {
        let lower = sql.to_lowercase();
        let into_pos = lower.find("into ").ok_or_else(|| "Missing INTO".to_string())?;
        let rest = &sql[into_pos + 5..].trim();
        let paren = rest.find('(').ok_or_else(|| "Missing (".to_string())?;
        let table_name = rest[..paren].trim();
        let values_paren = rest.rfind('(').ok_or_else(|| "Missing values (".to_string())?;
        let cols_str = &rest[paren + 1..rest.find(')').ok_or_else(|| "Missing )".to_string())?];
        let vals_str = &rest[values_paren + 1..rest.rfind(')').ok_or_else(|| "Missing closing )".to_string())?];
        let col_names: Vec<&str> = cols_str.split(',').map(|s| s.trim().trim_matches('"').trim_matches('\'')).collect();
        let raw_vals: Vec<&str> = vals_str.split(',').map(|s| s.trim()).collect();
        let schema = self.tables.get(table_name).ok_or_else(|| format!("Table not found: {}", table_name))?;
        let mut row = HashMap::new();
        for (i, col_name) in col_names.iter().enumerate() {
            let raw_val = if i < raw_vals.len() { raw_vals[i] } else { "NULL" };
            let val = parse_sql_value(raw_val);
            row.insert(col_name.to_string(), val);
        }
        self.rows.get_mut(table_name).unwrap().push(row);
        Ok(Value::Int(1))
    }

    fn execute_select(&mut self, sql: &str) -> Result<Value, String> {
        let lower = sql.to_lowercase();
        let from_pos = lower.find("from ").ok_or_else(|| "Missing FROM".to_string())?;
        let select_part = (&sql[6..from_pos]).trim();
        let rest = &sql[from_pos + 5..].trim();
        let table_name = rest.split_whitespace().next().ok_or_else(|| "Missing table name".to_string())?;
        let rows = self.rows.get(table_name).ok_or_else(|| format!("Table not found: {}", table_name))?;
        let all_cols = select_part == "*";
        let selected_cols: Vec<&str> = if all_cols {
            if let Some(schema) = self.tables.get(table_name) {
                schema.columns.iter().map(|c| c.name.as_str()).collect()
            } else {
                Vec::new()
            }
        } else {
            select_part.split(',').map(|s| s.trim()).collect()
        };
        let mut result = Vec::new();
        for row in rows {
            let mut map = Vec::new();
            for col in &selected_cols {
                if let Some(val) = row.get(*col) {
                    map.push((Value::Str(col.to_string()), val.clone()));
                }
            }
            result.push(Value::Map(map));
        }
        Ok(Value::Vec(std::rc::Rc::new(std::cell::RefCell::new(result))))
    }

    fn execute_drop(&mut self, sql: &str) -> Result<Value, String> {
        let name = sql.trim_start_matches("DROP TABLE")
            .trim_start_matches("drop table")
            .trim();
        self.tables.remove(name);
        self.rows.remove(name);
        Ok(Value::Int(0))
    }
}

fn parse_sql_value(s: &str) -> Value {
    let s = s.trim();
    if s.eq_ignore_ascii_case("NULL") {
        return Value::Nil;
    }
    if s.eq_ignore_ascii_case("TRUE") { return Value::Bool(true); }
    if s.eq_ignore_ascii_case("FALSE") { return Value::Bool(false); }
    if let Ok(i) = s.parse::<i64>() { return Value::Int(i); }
    if let Ok(f) = s.parse::<f64>() { return Value::Float(f); }
    let stripped = s.trim_matches('\'').trim_matches('"');
    Value::Str(stripped.to_string())
}


unsafe impl Send for SQLiteDriver {}
unsafe impl Sync for SQLiteDriver {}
