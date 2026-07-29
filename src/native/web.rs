// Aly Web Module: SSR, SSG, SPA, PWA support

use axum::{
    Router,
    routing::{get, post},
    response::{Html, IntoResponse, Response},
    extract::{Path as AxumPath, State, Query},
    http::StatusCode,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::fs;
use std::path::{Path, PathBuf};
use tokio::sync::RwLock;
use tower_http::services::ServeDir;
use tower_http::cors::CorsLayer;

use crate::native::types::{Validator, ValueData};
use crate::native::std::{split_args, arg};
use crate::validators::str::put_quoted_str;
use crate::aly::get_runtime;
use crate::native::dom_backend::DomBackend;

fn ok_str(s: String) -> Box<dyn Validator> {
    Box::new(put_quoted_str(s))
}

fn ok_int(i: i32) -> Box<dyn Validator> {
    Box::new(ValueData::Int(i as i32))
}

fn ok_bool(b: bool) -> Box<dyn Validator> {
    Box::new(ValueData::Bool(b))
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRoute {
    pub path: String,
    pub template: String,
    pub data: HashMap<String, serde_json::Value>,
    pub layout: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SSRContext {
    pub path: String,
    pub query: HashMap<String, String>,
    pub headers: HashMap<String, String>,
    pub cookies: HashMap<String, String>,
    pub session: Option<serde_json::Value>,
    pub user: Option<serde_json::Value>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageProps {
    pub html: String,
    pub head: String,
    pub scripts: Vec<String>,
    pub styles: Vec<String>,
    pub meta: HashMap<String, String>,
    pub preload: Vec<String>,
}

pub struct WebServer {
    routes: Arc<RwLock<HashMap<String, PageRoute>>>,
    layouts: Arc<RwLock<HashMap<String, String>>>,
    static_dir: Option<PathBuf>,
    port: u16,
    ssr_enabled: bool,
    ssg_output_dir: Option<PathBuf>,
    spa_mode: bool,
    pwa_config: Option<PWAConfig>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PWAConfig {
    pub name: String,
    pub short_name: String,
    pub description: String,
    pub start_url: String,
    pub display: String,
    pub background_color: String,
    pub theme_color: String,
    pub icons: Vec<PWAIcon>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PWAIcon {
    pub src: String,
    pub sizes: String,
    pub r#type: String,
}

impl WebServer {
    pub fn new(port: u16) -> Self {
        Self {
            routes: Arc::new(RwLock::new(HashMap::new())),
            layouts: Arc::new(RwLock::new(HashMap::new())),
            static_dir: None,
            port,
            ssr_enabled: true,
            ssg_output_dir: None,
            spa_mode: false,
            pwa_config: None,
        }
    }

    pub fn with_static_dir(mut self, dir: impl AsRef<Path>) -> Self {
        self.static_dir = Some(dir.as_ref().to_path_buf());
        self
    }

    pub fn with_ssr(mut self, enabled: bool) -> Self {
        self.ssr_enabled = enabled;
        self
    }

    pub fn with_ssg(mut self, output_dir: impl AsRef<Path>) -> Self {
        self.ssg_output_dir = Some(output_dir.as_ref().to_path_buf());
        self
    }

    pub fn with_spa(mut self, enabled: bool) -> Self {
        self.spa_mode = enabled;
        self
    }

    pub fn with_pwa(mut self, config: PWAConfig) -> Self {
        self.pwa_config = Some(config);
        self
    }

    pub fn add_route(&self, path: &str, template: &str, data: HashMap<String, serde_json::Value>) {
        let route = PageRoute {
            path: path.to_string(),
            template: template.to_string(),
            data,
            layout: None,
        };
        self.routes.blocking_write().insert(path.to_string(), route);
    }

    pub fn add_route_with_layout(&self, path: &str, template: &str, layout: &str, data: HashMap<String, serde_json::Value>) {
        let route = PageRoute {
            path: path.to_string(),
            template: template.to_string(),
            data,
            layout: Some(layout.to_string()),
        };
        self.routes.blocking_write().insert(path.to_string(), route);
    }

    pub fn add_layout(&self, name: &str, layout: &str) {
        self.layouts.blocking_write().insert(name.to_string(), layout.to_string());
    }

    pub async fn render_ssr(&self, path: &str, context: SSRContext) -> PageProps {
        let routes = self.routes.read().await;
        let layouts = self.layouts.read().await;

        let route = routes.get(path).or_else(|| {
            routes.iter().find(|(k, _)| self.match_route(k, path)).map(|(_, v)| v)
        });

        let mut html = String::new();
        let mut head = String::new();
        let mut scripts = Vec::new();
        let mut styles = Vec::new();
        let mut meta = HashMap::new();
        let mut preload = Vec::new();

        if let Some(route) = route {
            let template = &route.template;
            let data = &route.data;

            let mut context_json = serde_json::to_value(&context).unwrap_or(serde_json::Value::Null);
            if let serde_json::Value::Object(ref mut obj) = context_json {
                for (k, v) in data {
                    obj.insert(k.clone(), v.clone());
                }
            }

            html = self.render_template(template, &context_json).await;

            if let Some(layout_name) = &route.layout {
                if let Some(layout) = layouts.get(layout_name) {
                    html = layout.replace("{{content}}", &html);
                }
            }
        } else if self.spa_mode {
            html = self.render_spa_shell().await;
        } else {
            html = self.render_404().await;
        }

        if let Some(pwa) = &self.pwa_config {
            head.push_str(&self.generate_pwa_meta(pwa));
            meta.insert("theme-color".to_string(), pwa.theme_color.clone());
        }

        PageProps {
            html,
            head,
            scripts,
            styles,
            meta,
            preload,
        }
    }

    async fn render_template(&self, template: &str, context: &serde_json::Value) -> String {
        let mut result = template.to_string();

        if let serde_json::Value::Object(obj) = context {
            for (key, value) in obj {
                let placeholder = format!("{{{{{}}}}}", key);
                let replacement = match value {
                    serde_json::Value::String(s) => s.clone(),
                    serde_json::Value::Number(n) => n.to_string(),
                    serde_json::Value::Bool(b) => b.to_string(),
                    serde_json::Value::Array(arr) => serde_json::to_string(arr).unwrap_or_default(),
                    serde_json::Value::Object(obj) => serde_json::to_string(obj).unwrap_or_default(),
                    serde_json::Value::Null => String::new(),
                };
                result = result.replace(&placeholder, &replacement);
            }
        }

        result = self.render_conditionals(&result, context);
        result = self.render_loops(&result, context);

        result
    }

    fn render_conditionals(&self, template: &str, context: &serde_json::Value) -> String {
        let mut result = template.to_string();
        let re = regex::Regex::new(r"\{\{#if\s+(\w+)\}\}(.*?)\{\{/if\}\}").unwrap();
        
        for cap in re.captures_iter(template) {
            let condition = &cap[1];
            let content = &cap[2];
            
            let should_render = match context.get(condition) {
                Some(serde_json::Value::Bool(b)) => *b,
                Some(serde_json::Value::String(s)) => !s.is_empty(),
                Some(serde_json::Value::Number(n)) => n.as_i64().unwrap_or(0) != 0,
                Some(serde_json::Value::Array(arr)) => !arr.is_empty(),
                Some(serde_json::Value::Object(obj)) => !obj.is_empty(),
                _ => false,
            };

            if should_render {
                result = result.replace(&cap[0], content);
            } else {
                result = result.replace(&cap[0], "");
            }
        }
        result
    }

    fn render_loops(&self, template: &str, context: &serde_json::Value) -> String {
        let mut result = template.to_string();
        let re = regex::Regex::new(r"\{\{#each\s+(\w+)\}\}(.*?)\{\{/each\}\}").unwrap();
        
        for cap in re.captures_iter(template) {
            let array_name = &cap[1];
            let item_template = &cap[2];
            
            if let Some(serde_json::Value::Array(items)) = context.get(array_name) {
                let mut rendered_items = String::new();
                for (index, item) in items.iter().enumerate() {
                    let mut item_rendered = item_template.to_string();
                    
                    if let serde_json::Value::Object(obj) = item {
                        for (key, value) in obj {
                            let placeholder = format!("{{{{{}}}}}", key);
                            let replacement = match value {
                                serde_json::Value::String(s) => s.clone(),
                                serde_json::Value::Number(n) => n.to_string(),
                                serde_json::Value::Bool(b) => b.to_string(),
                                _ => value.to_string(),
                            };
                            item_rendered = item_rendered.replace(&placeholder, &replacement);
                        }
                    }
                    
                    item_rendered = item_rendered.replace("{{@index}}", &index.to_string());
                    rendered_items.push_str(&item_rendered);
                }
                result = result.replace(&cap[0], &rendered_items);
            }
        }
        result
    }

    fn match_route(&self, pattern: &str, path: &str) -> bool {
        if pattern == path {
            return true;
        }
        
        let pattern_parts: Vec<&str> = pattern.split('/').collect();
        let path_parts: Vec<&str> = path.split('/').collect();
        
        if pattern_parts.len() != path_parts.len() {
            return false;
        }
        
        for (p, part) in pattern_parts.iter().zip(path_parts.iter()) {
            if p.starts_with(':') {
                continue;
            }
            if p != part {
                return false;
            }
        }
        true
    }

    async fn render_spa_shell(&self) -> String {
        let mut html = String::from("<!DOCTYPE html>\n<html lang=\"en\">\n<head>\n");
        html.push_str("  <meta charset=\"UTF-8\">\n");
        html.push_str("  <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">\n");
        html.push_str("  <title>Aly SPA</title>\n");
        
        if let Some(pwa) = &self.pwa_config {
            html.push_str(&self.generate_pwa_meta(pwa));
        }
        
        html.push_str("</head>\n<body>\n");
        html.push_str("  <div id=\"app\"></div>\n");
        html.push_str("  <script src=\"/hydrate.js\"></script>\n");
        html.push_str("</body>\n</html>");
        
        html
    }

    async fn render_404(&self) -> String {
        String::from("<!DOCTYPE html>\n<html>\n<head><title>404 - Not Found</title></head>\n<body><h1>404 - Page Not Found</h1></body>\n</html>")
    }

    fn generate_pwa_meta(&self, pwa: &PWAConfig) -> String {
        let mut meta = String::new();
        meta.push_str(&format!("  <link rel=\"manifest\" href=\"/manifest.json\">\n"));
        meta.push_str(&format!("  <meta name=\"theme-color\" content=\"{}\">\n", pwa.theme_color));
        meta.push_str(&format!("  <meta name=\"apple-mobile-web-app-capable\" content=\"yes\">\n"));
        meta.push_str(&format!("  <meta name=\"apple-mobile-web-app-status-bar-style\" content=\"default\">\n"));
        meta.push_str(&format!("  <meta name=\"apple-mobile-web-app-title\" content=\"{}\">\n", pwa.name));
        for icon in &pwa.icons {
            meta.push_str(&format!("  <link rel=\"icon\" href=\"{}\" sizes=\"{}\" type=\"{}\">\n", icon.src, icon.sizes, icon.r#type));
        }
        meta
    }

    pub async fn generate_ssg(&self) -> Result<Vec<String>, String> {
        let output_dir = self.ssg_output_dir.as_ref().ok_or("SSG output directory not configured")?;
        
        fs::create_dir_all(output_dir).map_err(|e| format!("Failed to create output dir: {}", e))?;
        
        let routes = self.routes.read().await;
        let mut generated = Vec::new();

        for (path, route) in routes.iter() {
            let context = SSRContext {
                path: path.clone(),
                query: HashMap::new(),
                headers: HashMap::new(),
                cookies: HashMap::new(),
                session: None,
                user: None,
            };

            let props = self.render_ssr(path, context).await;
            
            let file_path = if path == "/" {
                output_dir.join("index.html")
            } else {
                let clean_path = path.trim_start_matches('/');
                let mut p = output_dir.join(clean_path);
                if p.extension().is_none() {
                    p.push("index.html");
                }
                p
            };

            if let Some(parent) = file_path.parent() {
                fs::create_dir_all(parent).map_err(|e| format!("Failed to create dir: {}", e))?;
            }

            let mut full_html = String::from("<!DOCTYPE html>\n<html>\n<head>\n");
            full_html.push_str(&props.head);
            full_html.push_str("</head>\n<body>\n");
            full_html.push_str(&props.html);
            full_html.push_str("\n</body>\n</html>");

            fs::write(&file_path, full_html).map_err(|e| format!("Failed to write file: {}", e))?;
            generated.push(file_path.display().to_string());
        }

        if let Some(pwa) = &self.pwa_config {
            let manifest = self.generate_manifest(pwa);
            let manifest_path = output_dir.join("manifest.json");
            fs::write(&manifest_path, manifest).map_err(|e| format!("Failed to write manifest: {}", e))?;
            generated.push(manifest_path.display().to_string());

            let sw = self.generate_service_worker();
            let sw_path = output_dir.join("sw.js");
            fs::write(&sw_path, sw).map_err(|e| format!("Failed to write service worker: {}", e))?;
            generated.push(sw_path.display().to_string());
        }

        if let Some(static_dir) = &self.static_dir {
            if static_dir.exists() {
                let copy_dir = output_dir.join("static");
                fs::create_dir_all(&copy_dir).ok();
                Self::copy_dir(static_dir, &copy_dir).ok();
            }
        }

        Ok(generated)
    }

    fn copy_dir(src: &Path, dst: &Path) -> std::io::Result<()> {
        fs::create_dir_all(dst)?;
        for entry in fs::read_dir(src)? {
            let entry = entry?;
            let src_path = entry.path();
            let dst_path = dst.join(entry.file_name());
            if src_path.is_dir() {
                Self::copy_dir(&src_path, &dst_path)?;
            } else {
                fs::copy(&src_path, &dst_path)?;
            }
        }
        Ok(())
    }

    fn generate_manifest(&self, pwa: &PWAConfig) -> String {
        serde_json::to_string_pretty(pwa).unwrap_or_default()
    }

    fn generate_service_worker(&self) -> String {
        r#"
const CACHE_NAME = 'aly-pwa-v1';
const urlsToCache = [
  '/',
  '/index.html',
  '/manifest.json',
  '/hydrate.js'
];

self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME)
      .then((cache) => cache.addAll(urlsToCache))
  );
});

self.addEventListener('fetch', (event) => {
  event.respondWith(
    caches.match(event.request)
      .then((response) => {
        if (response) {
          return response;
        }
        return fetch(event.request).then((response) => {
          if (!response || response.status !== 200 || response.type !== 'basic') {
            return response;
          }
          const responseToCache = response.clone();
          caches.open(CACHE_NAME)
            .then((cache) => {
              cache.put(event.request, responseToCache);
            });
          return response;
        });
      })
  );
});

self.addEventListener('activate', (event) => {
  event.waitUntil(
    caches.keys().then((cacheNames) => {
      return Promise.all(
        cacheNames.map((cacheName) => {
          if (cacheName !== CACHE_NAME) {
            return caches.delete(cacheName);
          }
        })
      );
    })
  );
});
"#.to_string()
    }

    pub async fn start(self) -> Result<(), String> {
        let routes = self.routes.clone();
        let layouts = self.layouts.clone();
        let static_dir = self.static_dir.clone();
        let ssr_enabled = self.ssr_enabled;
        let spa_mode = self.spa_mode;
        let pwa_config = self.pwa_config.clone();
        let port = self.port;

        let app_state = Arc::new(WebServerState {
            routes,
            layouts,
            ssr_enabled,
            spa_mode,
            pwa_config,
        });

        let mut router = Router::new()
            .route("/manifest.json", get(serve_manifest))
            .route("/sw.js", get(serve_service_worker))
            .route("/hydrate.js", get(serve_hydration_script))
            .layer(CorsLayer::permissive())
            .with_state(app_state.clone());

        if let Some(static_dir) = static_dir {
            router = router.nest_service("/static", ServeDir::new(static_dir));
        }

        router = router.fallback(spa_fallback);

        let addr = format!("127.0.0.1:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr).await
            .map_err(|e| format!("Failed to bind: {}", e))?;

        println!("Aly Web Server running on http://{}", addr);
        
        axum::serve(listener, router).await
            .map_err(|e| format!("Server error: {}", e))
    }
}

#[derive(Clone)]
struct WebServerState {
    routes: Arc<RwLock<HashMap<String, PageRoute>>>,
    layouts: Arc<RwLock<HashMap<String, String>>>,
    ssr_enabled: bool,
    spa_mode: bool,
    pwa_config: Option<PWAConfig>,
}

async fn serve_manifest(State(state): State<Arc<WebServerState>>) -> impl IntoResponse {
    if let Some(pwa) = &state.pwa_config {
        let manifest = serde_json::to_string_pretty(pwa).unwrap_or_default();
        Response::builder()
            .header("Content-Type", "application/manifest+json")
            .body(manifest)
            .unwrap()
    } else {
        (StatusCode::NOT_FOUND, "PWA not configured").into_response()
    }
}

async fn serve_service_worker() -> impl IntoResponse {
    let sw = r#"
const CACHE_NAME = 'aly-pwa-v1';
const urlsToCache = ['/', '/index.html', '/manifest.json', '/hydrate.js'];

self.addEventListener('install', (event) => {
  event.waitUntil(
    caches.open(CACHE_NAME).then((cache) => cache.addAll(urlsToCache))
  );
});

self.addEventListener('fetch', (event) => {
  event.respondWith(
    caches.match(event.request).then((response) => {
      return response || fetch(event.request).then((response) => {
        if (response && response.status === 200 && response.type === 'basic') {
          const responseToCache = response.clone();
          caches.open(CACHE_NAME).then((cache) => cache.put(event.request, responseToCache));
        }
        return response;
      });
    })
  );
});
"#;
    Response::builder()
        .header("Content-Type", "application/javascript")
        .header("Service-Worker-Allowed", "/")
        .body(sw)
        .unwrap()
}

async fn serve_hydration_script() -> impl IntoResponse {
    let script = include_str!("../../web_assets/hydrate.js");
    Response::builder()
        .header("Content-Type", "application/javascript")
        .body(script.to_string())
        .unwrap()
}

async fn spa_fallback(
    State(state): State<Arc<WebServerState>>,
    AxumPath(path): AxumPath<String>,
    Query(query): Query<HashMap<String, String>>,
) -> impl IntoResponse {
    let path = format!("/{}", path);
    
    if state.ssr_enabled {
        let context = SSRContext {
            path: path.clone(),
            query,
            headers: HashMap::new(),
            cookies: HashMap::new(),
            session: None,
            user: None,
        };
        
        let server = WebServer::new(0);
        server.routes = state.routes.clone();
        server.layouts = state.layouts.clone();
        server.ssr_enabled = state.ssr_enabled;
        server.spa_mode = state.spa_mode;
        server.pwa_config = state.pwa_config.clone();
        
        let props = server.render_ssr(&path, context).await;
        
        let mut html = String::from("<!DOCTYPE html>\n<html>\n<head>\n");
        html.push_str(&props.head);
        html.push_str("</head>\n<body>\n");
        html.push_str(&props.html);
        html.push_str("\n<script src=\"/hydrate.js\"></script>\n");
        html.push_str("</body>\n</html>");
        
        Html(html).into_response()
    } else {
        let props = WebServer::new(0).render_spa_shell().await;
        Html(props).into_response()
    }
}

pub fn web_server_serve(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 4);
    let port_str = arg(&args_list, 0);
    let static_dir = arg(&args_list, 1);
    let ssr_str = arg(&args_list, 2);
    let spa_str = arg(&args_list, 3);

    let port: u16 = match port_str.parse() {
        Ok(p) => p,
        Err(_) => return ok_str("None".to_string()),
    };

    let ssr_enabled = ssr_str == "true";
    let spa_mode = spa_str == "true";

    let mut server = WebServer::new(port)
        .with_ssr(ssr_enabled)
        .with_spa(spa_mode);

    if !static_dir.is_empty() && static_dir != "none" {
        server = server.with_static_dir(static_dir);
    }

    std::thread::spawn(move || {
        let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
        rt.block_on(async {
            if let Err(e) = server.start().await {
                eprintln!("Web server error: {}", e);
            }
        });
    });

    std::thread::sleep(std::time::Duration::from_millis(100));
    ok_str("None".to_string())
}

pub fn web_add_route(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 3);
    let path = arg(&args_list, 0);
    let template = arg(&args_list, 1);
    let data_json = arg(&args_list, 2);

    let data: HashMap<String, serde_json::Value> = if data_json.is_empty() {
        HashMap::new()
    } else {
        serde_json::from_str(&data_json).unwrap_or_default()
    };

    let runtime = get_runtime();
    if let Some(server) = runtime.get_web_server() {
        server.add_route(&path, &template, data);
        ok_str("None".to_string())
    } else {
        ok_str("None".to_string())
    }
}

pub fn web_add_layout(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 2);
    let name = arg(&args_list, 0);
    let layout = arg(&args_list, 1);

    let runtime = get_runtime();
    if let Some(server) = runtime.get_web_server() {
        server.add_layout(&name, &layout);
        ok_str("None".to_string())
    } else {
        ok_str("None".to_string())
    }
}

pub fn web_ssg_generate(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 1);
    let output_dir = arg(&args_list, 0);

    let runtime = get_runtime();
    if let Some(server) = runtime.get_web_server() {
        let server_clone = server.clone();
        let output_dir = output_dir.to_string();
        
        std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
            rt.block_on(async {
                match server_clone.generate_ssg().await {
                    Ok(files) => {
                        println!("SSG generated {} files:", files.len());
                        for f in files {
                            println!("  {}", f);
                        }
                    }
                    Err(e) => eprintln!("SSG error: {}", e),
                }
            });
        });

        ok_str("None".to_string())
    } else {
        ok_str("None".to_string())
    }
}

pub fn web_pwa_config(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 7);
    let name = arg(&args_list, 0);
    let short_name = arg(&args_list, 1);
    let description = arg(&args_list, 2);
    let start_url = arg(&args_list, 3);
    let display = arg(&args_list, 4);
    let bg_color = arg(&args_list, 5);
    let theme_color = arg(&args_list, 6);

    let config = PWAConfig {
        name,
        short_name,
        description,
        start_url,
        display,
        background_color: bg_color,
        theme_color,
        icons: vec![
            PWAIcon { src: "/icons/icon-192.png".to_string(), sizes: "192x192".to_string(), r#type: "image/png".to_string() },
            PWAIcon { src: "/icons/icon-512.png".to_string(), sizes: "512x512".to_string(), r#type: "image/png".to_string() },
        ],
    };

    let runtime = get_runtime();
    if let Some(server) = runtime.get_web_server() {
        server.pwa_config = Some(config);
        ok_str("None".to_string())
    } else {
        ok_str("None".to_string())
    }
}

pub fn web_render_ssr(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 2);
    let path = arg(&args_list, 0);
    let context_json = arg(&args_list, 1);

    let context: SSRContext = if context_json.is_empty() {
        SSRContext {
            path: path.clone(),
            query: HashMap::new(),
            headers: HashMap::new(),
            cookies: HashMap::new(),
            session: None,
            user: None,
        }
    } else {
        serde_json::from_str(&context_json).unwrap_or_else(|_| SSRContext {
            path: path.clone(),
            query: HashMap::new(),
            headers: HashMap::new(),
            cookies: HashMap::new(),
            session: None,
            user: None,
        })
    };

    let runtime = get_runtime();
    if let Some(server) = runtime.get_web_server() {
        let server_clone = server.clone();
        let path_clone = path.clone();
        
        let result = std::thread::spawn(move || {
            let rt = tokio::runtime::Runtime::new().expect("Failed to create runtime");
            rt.block_on(async {
                let props = server_clone.render_ssr(&path_clone, context).await;
                serde_json::to_string(&props).unwrap_or_default()
            })
        }).join().unwrap_or_default();

        ok_str(result)
    } else {
        ok_str("{}".to_string())
    }
}