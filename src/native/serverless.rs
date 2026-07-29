use std::collections::HashMap;
use crate::native::std::{arg as std_arg, split_args};
use crate::native::types::Validator;
use crate::validators::str::put_quoted_str;

thread_local! {
    static FUNCTIONS: std::cell::RefCell<HashMap<String, String>> = std::cell::RefCell::new(HashMap::new());
    static INVOKE_COUNT: std::cell::RefCell<HashMap<String, u64>> = std::cell::RefCell::new(HashMap::new());
}

fn ok_str(s: String) -> Box<dyn Validator> {
    Box::new(put_quoted_str(s))
}

pub fn sl_define(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let handler = std_arg(&args, 1);
    FUNCTIONS.with(|f| {
        f.borrow_mut().insert(name.to_string(), handler.to_string());
    });
    ok_str(format!("Serverless function '{}' defined", name))
}

pub fn sl_invoke(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let event = std_arg(&args, 1);
    FUNCTIONS.with(|f| {
        let handler = f.borrow().get(&name).cloned();
        drop(f);
        INVOKE_COUNT.with(|c| {
            let mut counts = c.borrow_mut();
            let entry = counts.entry(name.to_string()).or_insert(0);
            *entry += 1;
            let current_count = *entry;
            drop(counts);
            match handler {
                Some(h) => ok_str(format!("Function '{}' invoked (call #{}): handler='{}', event={}", name, current_count, h, event)),
                None => ok_str(format!("Function '{}' not found", name)),
            }
        })
    })
}

pub fn sl_list(_x: String) -> Box<dyn Validator> {
    FUNCTIONS.with(|f| {
        let guard = f.borrow();
        if guard.is_empty() {
            ok_str("No serverless functions defined".to_string())
        } else {
            let lines: Vec<String> = guard.iter().map(|(k, v)| format!("  {} => {}", k, v)).collect();
            ok_str(format!("Serverless functions:\n{}", lines.join("\n")))
        }
    })
}

pub fn sl_invoke_count(x: String) -> Box<dyn Validator> {
    let name = std_arg(&split_args(&x, 1), 0);
    INVOKE_COUNT.with(|c| {
        let guard = c.borrow();
        let count = guard.get(&name).cloned().unwrap_or(0);
        ok_str(format!("Function '{}' invoked {} times", name, count))
    })
}

pub fn sl_serve(x: String) -> Box<dyn Validator> {
    let port: u16 = std_arg(&split_args(&x, 1), 0).parse().unwrap_or(3000);
    let rt = tokio::runtime::Runtime::new().expect("Failed to create tokio runtime");
    rt.spawn(async move {
        let addr = format!("0.0.0.0:{}", port);
        let listener = tokio::net::TcpListener::bind(&addr).await.expect("Failed to bind serverless listener");
        while let Ok((_stream, _)) = listener.accept().await {
            let _ = _stream;
        }
    });
    ok_str(format!("Serverless runtime listening on http://0.0.0.0:{}", port))
}