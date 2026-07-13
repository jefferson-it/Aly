// Aly HTTP module: client and server functionality.
//
// Client API:
//   http.get(url), http.post(url, body), http.put(url, body),
//   http.delete(url), http.patch(url, body), http.request(method, url, body),
//   http.status_code(url), http.head(url)
//
// Server API (Express.js-style):
//   http.serve(port)
//   http.get(path, handler), http.post(path, handler), etc.

mod client_lib {
    use crate::native::types::Validator;
    use crate::validators::str::put_quoted_str;
    use crate::native::std::{split_args, arg};

    fn ok_str(s: String) -> Box<dyn Validator> {
        Box::new(put_quoted_str(s))
    }

    fn ok_int(i: i32) -> Box<dyn Validator> {
        use crate::native::types::ValueData;
        Box::new(ValueData::Int(i as i64))
    }

    fn request_raw(method: &str, url: &str, body: Option<&str>) -> Result<String, String> {
        use std::time::Duration;

        let client = reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        let req = match method {
            "GET" => client.get(url),
            "POST" => client.post(url),
            "PUT" => client.put(url),
            "DELETE" => client.delete(url),
            "PATCH" => client.patch(url),
            "HEAD" => client.head(url),
            _ => return Err(format!("Unsupported HTTP method: {}", method)),
        };

        let req = if let Some(b) = body {
            req.header("Content-Type", "application/json").body(b.to_owned())
        } else {
            req
        };

        let resp = req.send().map_err(|e| format!("HTTP request failed: {}", e))?;
        let status = resp.status();
        let body_text = resp.text().map_err(|e| format!("Failed to read response body: {}", e))?;

        if !status.is_success() {
            return Err(format!("HTTP {} {}: {}", method, url, status));
        }

        Ok(body_text)
    }

    pub fn http_get(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let url = arg(&args, 0);
        match request_raw("GET", &url, None) {
            Ok(body) => ok_str(body),
            Err(e) => {
                eprintln!("RuntimeError [http.get]: {}", e);
                ok_str(String::new())
            }
        }
    }

    pub fn http_post(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let url = arg(&args, 0);
        let body = arg(&args, 1);
        match request_raw("POST", &url, Some(&body)) {
            Ok(b) => ok_str(b),
            Err(e) => {
                eprintln!("RuntimeError [http.post]: {}", e);
                ok_str(String::new())
            }
        }
    }

    pub fn http_put(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let url = arg(&args, 0);
        let body = arg(&args, 1);
        match request_raw("PUT", &url, Some(&body)) {
            Ok(b) => ok_str(b),
            Err(e) => {
                eprintln!("RuntimeError [http.put]: {}", e);
                ok_str(String::new())
            }
        }
    }

    pub fn http_delete(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let url = arg(&args, 0);
        match request_raw("DELETE", &url, None) {
            Ok(body) => ok_str(body),
            Err(e) => {
                eprintln!("RuntimeError [http.delete]: {}", e);
                ok_str(String::new())
            }
        }
    }

    pub fn http_patch(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 2);
        let url = arg(&args, 0);
        let body = arg(&args, 1);
        match request_raw("PATCH", &url, Some(&body)) {
            Ok(b) => ok_str(b),
            Err(e) => {
                eprintln!("RuntimeError [http.patch]: {}", e);
                ok_str(String::new())
            }
        }
    }

    pub fn http_request(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 3);
        let method = arg(&args, 0).to_uppercase();
        let url = arg(&args, 1);
        let body = arg(&args, 2);
        let body_opt = if body.is_empty() { None } else { Some(body.as_str()) };
        match request_raw(&method, &url, body_opt) {
            Ok(b) => ok_str(b),
            Err(e) => {
                eprintln!("RuntimeError [http.request]: {}", e);
                ok_str(String::new())
            }
        }
    }

    pub fn http_status_code(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let url = arg(&args, 0);
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create client");
        let resp = client.get(&url).send()
            .expect("Request failed");
        ok_int(resp.status().as_u16() as i32)
    }

    pub fn http_head(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 1);
        let url = arg(&args, 0);
        let client = reqwest::blocking::Client::builder()
            .timeout(std::time::Duration::from_secs(30))
            .build()
            .expect("Failed to create client");
        let resp = client.head(&url).send()
            .expect("Request failed");

        let status = resp.status().as_u16();
        let mut header_parts: Vec<String> = vec![];
        for (name, value) in resp.headers() {
            if let Ok(v) = value.to_str() {
                header_parts.push(format!("\"{}\": \"{}\"", name, v));
            }
        }
        let json = format!(
            "{{\"status\": {}, \"headers\": {{{}}}}}",
            status,
            header_parts.join(", ")
        );
        ok_str(json)
    }
}

mod server_lib {
    use axum::{
        Router,
        http::Method,
        body::Bytes,
        extract::{Path, State},
        response::IntoResponse,
    };
    use axum::http::StatusCode;
    use std::sync::{Arc, Mutex};
    use axum::routing::{get, post, put, delete, patch};
    use tower_http::cors::CorsLayer;

    use crate::native::types::Validator;

    type HandlerFn = fn(String) -> Box<dyn Validator>;

    #[derive(Clone)]
    struct Route {
        method: Method,
        path: String,
        handler: Arc<HandlerFn>,
    }

    #[derive(Clone)]
    struct AppState {
        routes: Arc<Vec<Route>>,
    }

    type RouteEntry = (String, String, HandlerFn);
    type RoutesStore = Arc<Mutex<Vec<RouteEntry>>>;

    fn get_routes() -> RoutesStore {
        use std::sync::OnceLock;
        static ROUTES: OnceLock<RoutesStore> = OnceLock::new();
        ROUTES.get_or_init(|| Arc::new(Mutex::new(Vec::new()))).clone()
    }

    fn format_request(method: &Method, path: &str, body: &str) -> String {
        format!("{},{},{}", method.as_str(), path, body)
    }

    async fn handle_route(
        method: Method,
        Path(path): Path<String>,
        body: Bytes,
        State(state): State<Arc<AppState>>,
    ) -> impl IntoResponse {
        let body_str = String::from_utf8_lossy(&body).to_string();
        let req_str = format_request(&method, &path, &body_str);

        if let Some(route) = state.routes.iter().find(|r| r.path == path && r.method == method) {
            let result = (route.handler)(req_str);
            let (_type, val) = result.valid();
            let body = val.literal();
            (StatusCode::OK, body)
        } else {
            (StatusCode::NOT_FOUND, format!("Route {} {} not found", method, path))
        }
    }

    pub fn http_serve(port: u16) {
        let routes = get_routes();
        let routes_clone = routes.clone();

        let mut app_routes = Vec::new();
        {
            let guard = routes_clone.lock().unwrap();
            for (method_str, path, handler) in guard.iter() {
                let method = match method_str.as_str() {
                    "GET" => Method::GET,
                    "POST" => Method::POST,
                    "PUT" => Method::PUT,
                    "DELETE" => Method::DELETE,
                    "PATCH" => Method::PATCH,
                    _ => Method::GET,
                };
                app_routes.push(Route {
                    method,
                    path: path.clone(),
                    handler: Arc::new(*handler),
                });
            }
        }

        let state = Arc::new(AppState { routes: Arc::new(app_routes) });
        let state_clone = state.clone();

        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
            rt.block_on(async {
                let mut router = Router::new()
                    .layer(CorsLayer::permissive());

                for route in state_clone.routes.iter() {
                    let handler_state = state_clone.clone();
                    let handler = move |method: Method, Path(path): Path<String>, body: Bytes| {
                        handle_route(method, Path(path), body, State(handler_state.clone()))
                    };

                    match route.method {
                        Method::GET => router = router.route(&route.path, get(handler.clone())),
                        Method::POST => router = router.route(&route.path, post(handler.clone())),
                        Method::PUT => router = router.route(&route.path, put(handler.clone())),
                        Method::DELETE => router = router.route(&route.path, delete(handler.clone())),
                        Method::PATCH => router = router.route(&route.path, patch(handler.clone())),
                        _ => {}
                    }
                }

                let addr = format!("0.0.0.0:{}", port);
                let listener = tokio::net::TcpListener::bind(&addr)
                    .await
                    .expect("Failed to bind to port");
                println!("Aly HTTP server listening on http://0.0.0.0:{}", port);

                axum::serve(listener, router.into_make_service())
                    .await
                    .expect("Server error");
            });
        });
    }

    pub fn http_route(method: &str, path: String, handler: HandlerFn) {
        let routes = get_routes();
        let mut guard = routes.lock().unwrap();
        guard.push((method.to_string(), path, handler));
    }
}

pub use server_lib::*;
pub use client_lib::*;
