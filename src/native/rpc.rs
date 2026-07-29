use std::collections::HashMap;
use crate::native::std::{arg as std_arg, split_args};
use crate::native::types::Validator;
use crate::validators::str::put_quoted_str;

thread_local! {
    static RPC_REGISTRY: std::cell::RefCell<HashMap<String, String>> = std::cell::RefCell::new(HashMap::new());
}

fn ok_str(s: String) -> Box<dyn Validator> {
    Box::new(put_quoted_str(s))
}

pub fn rpc_register(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let handler = std_arg(&args, 1);
    RPC_REGISTRY.with(|r| {
        r.borrow_mut().insert(name.to_string(), handler.to_string());
    });
    ok_str(format!("RPC '{}' registered", name))
}

pub fn rpc_call(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let payload = std_arg(&args, 1);
    RPC_REGISTRY.with(|r| {
        let guard = r.borrow();
        if guard.contains_key(&name) {
            ok_str(format!("RPC '{}' called with payload: {}", name, payload))
        } else {
            ok_str(format!("RPC '{}' not found", name))
        }
    })
}

pub fn rpc_serve(x: String) -> Box<dyn Validator> {
    let port: u16 = std_arg(&split_args(&x, 1), 0).parse().unwrap_or(9090);
    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    rt.spawn(async move {
        let addr = format!("0.0.0.0:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind RPC listener");
        while let Ok((_stream, _)) = listener.accept().await {
            let _ = _stream;
        }
    });
    ok_str(format!("RPC server listening on 0.0.0.0:{}", port))
}

pub fn rpc_broadcast(x: String) -> Box<dyn Validator> {
    let payload = std_arg(&split_args(&x, 1), 0);
    RPC_REGISTRY.with(|r| {
        let count = r.borrow().len();
        ok_str(format!("RPC broadcast: '{}' => {} endpoints", payload, count))
    })
}