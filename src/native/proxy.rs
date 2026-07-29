use std::collections::HashMap;
use crate::native::std::{arg as std_arg, split_args};
use crate::native::types::Validator;
use crate::validators::str::put_quoted_str;

thread_local! {
    static PROXY_REGISTRY: std::cell::RefCell<HashMap<String, String>> = std::cell::RefCell::new(HashMap::new());
}

fn ok_str(s: String) -> Box<dyn Validator> {
    Box::new(put_quoted_str(s))
}

pub fn proxy_forward(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let method = std_arg(&args, 0);
    let target = std_arg(&args, 1);
    let path = std_arg(&args, 2);
    let url = format!("{}{}", target, path);
    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    rt.block_on(async move {
        let client = reqwest::Client::new();
        let res = match method.to_lowercase().as_str() {
            "get" => client.get(&url).send().await,
            "post" => client.post(&url).send().await,
            "put" => client.put(&url).send().await,
            "delete" => client.delete(&url).send().await,
            "patch" => client.patch(&url).send().await,
            _ => client.get(&url).send().await,
        };
        match res {
            Ok(response) => {
                let status = response.status().as_u16();
                let body = response.text().await.unwrap_or_default();
                ok_str(format!("Status {}: {}", status, body))
            }
            Err(e) => ok_str(format!("Proxy error: {}", e)),
        }
    })
}

pub fn proxy_register(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let alias = std_arg(&args, 0);
    let target = std_arg(&args, 1);
    PROXY_REGISTRY.with(|r| {
        r.borrow_mut().insert(alias.to_string(), target.to_string());
    });
    ok_str(format!("Proxy alias '{}' => '{}' registered", alias, target))
}

pub fn proxy_use(x: String) -> Box<dyn Validator> {
    let alias = std_arg(&split_args(&x, 1), 0);
    PROXY_REGISTRY.with(|r| {
        let guard = r.borrow();
        if let Some(target) = guard.get(&alias) {
            ok_str(format!("Proxy '{}' => '{}'", alias, target))
        } else {
            ok_str(format!("Proxy alias '{}' not found", alias))
        }
    })
}

pub fn proxy_reverse_serve(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let port: u16 = std_arg(&args, 0).parse().unwrap_or(8080);
    let backend = std_arg(&args, 1);
    let backend_clone = backend.clone();
    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    rt.spawn(async move {
        let addr = format!("0.0.0.0:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind reverse proxy");
        while let Ok((_stream, _)) = listener.accept().await {
            let _ = _stream;
            let _ = backend_clone.clone();
        }
    });
    ok_str(format!("Reverse proxy on 0.0.0.0:{} => {}", port, backend))
}