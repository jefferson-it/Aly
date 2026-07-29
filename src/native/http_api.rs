mod api {
    use axum::{
        Router,
        routing::{get, post},
        response::{IntoResponse, Json},
        extract::Json as ExtractJson,
    };
    use tower_http::cors::CorsLayer;

    pub fn http_api_serve(args: String) -> Box<dyn crate::native::types::Validator> {
        use crate::native::std::{split_args, arg};
        use crate::validators::str::put_quoted_str;
        
        let args_list = split_args(&args, 1);
        let port_str = arg(&args_list, 0);
        
        let port: u16 = match port_str.parse() {
            Ok(p) => p,
            Err(e) => {
                eprintln!("RuntimeError [http_api_serve]: porta inválida '{}': {}", port_str, e);
                return Box::new(put_quoted_str("None".to_string()));
            }
        };

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
            rt.block_on(async move {
                serve_api_server(port).await;
            });
        });

        std::thread::sleep(std::time::Duration::from_millis(100));
        
        Box::new(put_quoted_str("None".to_string()))
    }

    async fn serve_api_server(port: u16) {
        let router = Router::new()
            .route("/api/vars", get(list_vars))
            .route("/api/doc", get(doc))
            .route("/api/run", post(run_script));

        let addr = format!("127.0.0.1:{}", port);
        let listener = match tokio::net::TcpListener::bind(&addr).await {
            Ok(l) => l,
            Err(e) => {
                eprintln!("RuntimeError [http_api_serve]: falha ao binding na porta {}: {}", port, e);
                return;
            }
        };

        let router_with_cors = router.layer(CorsLayer::permissive());

        println!("Aly HTTP API server listening on http://{}", addr);
        
        if let Err(e) = axum::serve(listener, router_with_cors).await {
            eprintln!("RuntimeError [http_api_serve]: erro no servidor: {}", e);
        }
    }

    async fn list_vars() -> impl IntoResponse {
        use crate::aly::get_runtime;
        let run = get_runtime();
        let vars = run.get_vars();
        let json_value: serde_json::Value = vars
            .iter()
            .filter_map(|var| {
                let mut obj = serde_json::Map::new();
                obj.insert("name".to_string(), serde_json::Value::String(var.get_name()));
                obj.insert("type".to_string(), serde_json::Value::String(var.get_type().to_string()));
                match var.get_value() {
                    crate::native::types::ValueData::String(v) => {
                        obj.insert("value".to_string(), serde_json::Value::String(v));
                    }
                    crate::native::types::ValueData::Int(v) => {
                        obj.insert("value".to_string(), serde_json::Value::Number(v.into()));
                    }
                    crate::native::types::ValueData::Float(v) => {
                        obj.insert("value".to_string(), serde_json::Value::Number(serde_json::Number::from_f64(v.into()).unwrap()));
                    }
                    crate::native::types::ValueData::Bool(v) => {
                        obj.insert("value".to_string(), serde_json::Value::Bool(v));
                    }
                    _ => {
                        let v = var.get_value().to_string(true);
                        obj.insert("value".to_string(), serde_json::Value::String(v));
                    }
                }
                Some(serde_json::Value::Object(obj))
            })
            .collect();

        Json(json_value)
    }

    async fn doc() -> impl IntoResponse {
        let mut doc: serde_json::Value = serde_json::Value::Object(serde_json::Map::new());

        use crate::aly::get_runtime;
        let run = get_runtime();
        let vars = run.get_vars();

        for var in vars {
            match var.get_value() {
                crate::native::types::ValueData::Object(obj) => {
                    let mut map: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();

                    for prop in obj.keys() {
                        map.insert(prop.clone(), serde_json::Value::String("prop".to_string()));
                    }

                    let key = var.get_name();
                    let namespace_items: serde_json::Value = serde_json::Value::Object(map);
                    let mut namespace_obj: serde_json::Map<String, serde_json::Value> = serde_json::Map::new();
                    namespace_obj.insert(key, namespace_items);

                    let mut doc_map: serde_json::Map<String, serde_json::Value> = doc.as_object().unwrap().clone();
                    doc_map.extend(namespace_obj);
                    doc = serde_json::Value::Object(doc_map);
                }
                _ => {}
            }
        }

        Json(doc)
    }

    async fn run_script(ExtractJson(payload): ExtractJson<serde_json::Value>) -> impl IntoResponse {
        use crate::runtime::parser::get_lexer;
        let code = match payload.get("code") {
            Some(code) => code.as_str().unwrap().to_string(),
            None => "return None".to_string(),
        };

        let lines: Vec<&str> = code.trim().lines().collect();
        get_lexer(lines);

        let mut result = serde_json::Value::Null;
        use crate::aly::get_runtime;
        let run = get_runtime();
        let vars = run.get_vars();

        if !vars.is_empty() {
            let var = vars.last().unwrap();
            let value = var.get_value();
            result = match value {
                crate::native::types::ValueData::String(v) => serde_json::Value::String(v),
                crate::native::types::ValueData::Int(v) => serde_json::Value::Number(v.into()),
                crate::native::types::ValueData::Float(v) => serde_json::Value::Number(serde_json::Number::from_f64(v.into()).unwrap()),
                crate::native::types::ValueData::Bool(v) => serde_json::Value::Bool(v),
                _ => serde_json::Value::String(value.to_string(true)),
            };
        }

        Json(serde_json::json!({ "result": result }))
    }
}

pub use api::http_api_serve;
