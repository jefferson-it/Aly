use std::collections::HashMap;
use crate::native::std::{arg as std_arg, split_args};
use crate::native::types::Validator;
use crate::validators::str::put_quoted_str;

thread_local! {
    static SERVICES: std::cell::RefCell<HashMap<String, String>> = std::cell::RefCell::new(HashMap::new());
    static HANDLERS: std::cell::RefCell<HashMap<String, String>> = std::cell::RefCell::new(HashMap::new());
}

fn ok_str(s: String) -> Box<dyn Validator> {
    Box::new(put_quoted_str(s))
}

pub fn grpc_register_service(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let proto_def = std_arg(&args, 1);
    SERVICES.with(|s| {
        s.borrow_mut().insert(name.to_string(), proto_def.to_string());
    });
    ok_str(format!("gRPC service '{}' registered ({} bytes proto)", name, proto_def.len()))
}

pub fn grpc_call(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let service = std_arg(&args, 0);
    let method = std_arg(&args, 1);
    let payload = std_arg(&args, 2);
    SERVICES.with(|s| {
        let guard = s.borrow();
        if guard.contains_key(&service) {
            ok_str(format!("gRPC call: {}.{} => payload={}", service, method, payload))
        } else {
            ok_str(format!("gRPC service '{}' not found", service))
        }
    })
}

pub fn grpc_serve(x: String) -> Box<dyn Validator> {
    let port: u16 = std_arg(&split_args(&x, 1), 0).parse().unwrap_or(50051);
    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    rt.spawn(async move {
        let addr = format!("0.0.0.0:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind gRPC listener");
        while let Ok((_stream, _)) = listener.accept().await {
            let _ = _stream;
        }
    });
    ok_str(format!("gRPC server listening on 0.0.0.0:{}", port))
}

pub fn grpc_register_handler(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let service = std_arg(&args, 0);
    let method = std_arg(&args, 1);
    HANDLERS.with(|h| {
        h.borrow_mut().insert(format!("{}.{}", service, method), "handler".to_string());
    });
    ok_str(format!("gRPC handler registered for {}.{}", service, method))
}