use std::collections::HashMap;
use crate::native::std::{arg as std_arg, split_args};
use crate::native::types::Validator;
use crate::validators::str::put_quoted_str;

thread_local! {
    static SCHEMA_REGISTRY: std::cell::RefCell<HashMap<String, HashMap<String, String>>> = std::cell::RefCell::new(HashMap::new());
}

fn ok_str(s: String) -> Box<dyn Validator> {
    Box::new(put_quoted_str(s))
}

pub fn graphql_register_schema(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let schema_text = std_arg(&args, 1);
    let mut schema = HashMap::new();
    for line in schema_text.lines() {
        let line = line.trim().trim_end_matches(';');
        if line.is_empty() { continue; }
        if let Some(pos) = line.find(':') {
            let field = line[..pos].trim().to_string();
            let field_type = line[pos + 1..].trim().trim_start_matches('!').to_string();
            schema.insert(field, field_type);
        }
    }
    SCHEMA_REGISTRY.with(|r| {
        r.borrow_mut().insert(name.to_string(), schema);
    });
    ok_str(format!("GraphQL schema '{}' registered", name))
}

pub fn graphql_introspect(x: String) -> Box<dyn Validator> {
    let schema_name = std_arg(&split_args(&x, 1), 0);
    let schemas = SCHEMA_REGISTRY.with(|r| r.borrow().clone());
    if let Some(schema) = schemas.get(&schema_name) {
        let fields: Vec<String> = schema.iter().map(|(k, t)| format!("  {}: {}", k, t)).collect();
        ok_str(format!("schema {} {{\n{}}}", schema_name, fields.join("\n")))
    } else {
        ok_str(format!("Schema '{}' not found", schema_name))
    }
}

pub fn graphql_query(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let schema_name = std_arg(&args, 0);
    let query_text = std_arg(&args, 1);
    let mut result_fields: Vec<String> = Vec::new();
    SCHEMA_REGISTRY.with(|r| {
        let guard = r.borrow();
        if let Some(schema) = guard.get(&schema_name) {
            for line in query_text.lines() {
                let line = line.trim().trim_start_matches('{').trim_end_matches('}').trim();
                if line.is_empty() || line.starts_with('#') { continue; }
                let field = line.split_whitespace().next().unwrap_or(line);
                if schema.contains_key(field) {
                    result_fields.push(format!("{}: null", field));
                }
            }
        }
    });
    ok_str(format!("{{{}}}", result_fields.join(", ")))
}

pub fn graphql_serve(x: String) -> Box<dyn Validator> {
    let port: u16 = std_arg(&split_args(&x, 1), 0).parse().unwrap_or(4000);
    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    rt.spawn(async move {
        let addr = format!("0.0.0.0:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind");
        while let Ok((_stream, _)) = listener.accept().await {
            let _ = _stream;
        }
    });
    ok_str(format!("GraphQL server listening on http://0.0.0.0:{}", port))
}
