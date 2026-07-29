use std::collections::HashMap;
use crate::native::std::{arg as std_arg, split_args};
use crate::native::types::Validator;
use crate::validators::str::put_quoted_str;

thread_local! {
    static SERVICES_REGISTRY: std::cell::RefCell<HashMap<String, String>> = std::cell::RefCell::new(HashMap::new());
    static SERVICE_INSTANCES: std::cell::RefCell<HashMap<String, Vec<String>>> = std::cell::RefCell::new(HashMap::new());
}

fn ok_str(s: String) -> Box<dyn Validator> {
    Box::new(put_quoted_str(s))
}

pub fn ms_register(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 2);
    let name = std_arg(&args, 0);
    let address = std_arg(&args, 1);
    SERVICES_REGISTRY.with(|r| {
        r.borrow_mut().insert(name.to_string(), address.to_string());
    });
    ok_str(format!("Service '{}' registered at '{}'", name, address))
}

pub fn ms_discover(x: String) -> Box<dyn Validator> {
    let name = std_arg(&split_args(&x, 1), 0);
    SERVICES_REGISTRY.with(|r| {
        let guard = r.borrow();
        if let Some(addr) = guard.get(&name) {
            ok_str(format!("Service '{}' => '{}'", name, addr))
        } else {
            ok_str(format!("Service '{}' not found", name))
        }
    })
}

pub fn ms_call(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let service = std_arg(&args, 0);
    let endpoint = std_arg(&args, 1);
    let payload = std_arg(&args, 2);
    SERVICES_REGISTRY.with(|r| {
        let guard = r.borrow();
        if let Some(addr) = guard.get(&service) {
            ok_str(format!("Calling '{}' at '{}' + '{}' with payload: {}", service, addr, endpoint, payload))
        } else {
            ok_str(format!("Service '{}' not found", service))
        }
    })
}

pub fn ms_register_instance(x: String) -> Box<dyn Validator> {
    let args = split_args(&x, 3);
    let name = std_arg(&args, 0);
    let instance_id = std_arg(&args, 1);
    let address = std_arg(&args, 2);
    SERVICE_INSTANCES.with(|r| {
        r.borrow_mut().entry(name.to_string()).or_default().push(format!("{}:{}", instance_id, address));
    });
    ok_str(format!("Instance '{}' registered for service '{}'", instance_id, name))
}

pub fn ms_list(_x: String) -> Box<dyn Validator> {
    SERVICES_REGISTRY.with(|r| {
        let guard = r.borrow();
        if guard.is_empty() {
            ok_str("No services registered".to_string())
        } else {
            let lines: Vec<String> = guard.iter().map(|(k, v)| format!("  {} => {}", k, v)).collect();
            ok_str(format!("Registered services:\n{}", lines.join("\n")))
        }
    })
}