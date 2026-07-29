// Aly curl module: HTTP client inspired by fetch API (JavaScript).
//
// API:
//   curl(url)                          - Simple GET request
//   curl.GET(url)                      - GET request
//   curl.POST(url, body)               - POST request with JSON body
//   curl.PUT(url, body)                - PUT request with JSON body
//   curl.DELETE(url)                   - DELETE request
//   curl.PATCH(url, body)              - PATCH request with JSON body
//   curl(url, options)                 - Flexible request with options object
//   curl[url](...args)                 - Dynamic method access via bracket notation

use std::collections::HashMap;
use std::time::Duration;

use crate::native::types::{Validator, ValueData};
use crate::validators::str::{put_quoted_str, remove_quoted_str};
use crate::native::std::{split_args, arg};

fn ok_str(s: String) -> Box<dyn Validator> {
    Box::new(put_quoted_str(s))
}

fn ok_int(i: i32) -> Box<dyn Validator> {
    Box::new(ValueData::Int(i))
}

#[allow(dead_code)]
fn ok_bool(b: bool) -> Box<dyn Validator> {
    Box::new(ValueData::Bool(b))
}

#[allow(dead_code)]
/// Build query string from object
fn build_query_string(params: &HashMap<String, String>) -> String {
    if params.is_empty() {
        return String::new();
    }
    
    let pairs: Vec<String> = params
        .iter()
        .map(|(k, v)| {
            let encoded_key = url_encode(k);
            let encoded_val = url_encode(v);
            format!("{}={}", encoded_key, encoded_val)
        })
        .collect();
    
    pairs.join("&")
}

#[allow(dead_code)]
/// Simple URL encoding
fn url_encode(s: &str) -> String {
    let mut result = String::new();
    for c in s.chars() {
        match c {
            'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => {
                result.push(c);
            }
            ' ' => result.push_str("%20"),
            _ => {
                result.push_str(&format!("%{:02X}", c as u8));
            }
        }
    }
    result
}

/// Parse query string from URL

/// Build URL with query parameters

/// Make HTTP request with the given method
fn make_request(
    method: &str,
    url: &str,
    body: Option<&str>,
    headers: Option<&HashMap<String, String>>,
    timeout: u64,
) -> Result<String, String> {
    use reqwest::blocking::Client;
    
    let client = Client::builder()
        .timeout(Duration::from_secs(timeout))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    let mut request = match method {
        "GET" => client.get(url),
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "DELETE" => client.delete(url),
        "PATCH" => client.patch(url),
        "HEAD" => client.head(url),
        _ => return Err(format!("Unsupported HTTP method: {}", method)),
    };
    
    // Add headers
    if let Some(h) = headers {
        for (k, v) in h {
            request = request.header(k, v);
        }
    }
    
    // Add body if present
    if let Some(b) = body {
        request = request.header("Content-Type", "application/json").body(b.to_owned());
    }
    
    let response = request
        .send()
        .map_err(|e| format!("HTTP request failed: {}", e))?;
    
    let status = response.status();
    let body_text = response
        .text()
        .map_err(|e| format!("Failed to read response body: {}", e))?;
    
    if !status.is_success() {
        return Err(format!("HTTP {} {}: {}", method, url, status));
    }
    
    Ok(body_text)
}

/// Get HTTP status code
fn get_status_code(url: &str, method: &str) -> Result<i32, String> {
    use reqwest::blocking::Client;
    
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))?;
    
    let response = match method {
        "GET" => client.get(url).send(),
        "POST" => client.post(url).send(),
        "PUT" => client.put(url).send(),
        "DELETE" => client.delete(url).send(),
        "PATCH" => client.patch(url).send(),
        _ => return Err(format!("Unsupported HTTP method: {}", method)),
    };
    
    match response {
        Ok(resp) => Ok(resp.status().as_u16() as i32),
        Err(e) => Err(format!("HTTP request failed: {}", e)),
    }
}

pub fn curl_main(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 10);
    
    if args_list.is_empty() {
        eprintln!("RuntimeError [curl]: URL is required");
        return ok_str(String::new());
    }
    
    let url = arg(&args_list, 0);
    
    // Simple GET request
    match make_request("GET", &url, None, None, 30) {
        Ok(body) => ok_str(body),
        Err(e) => {
            eprintln!("RuntimeError [curl]: {}", e);
            ok_str(String::new())
        }
    }
}

pub fn curl_get(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 2);
    let url = arg(&args_list, 0);
    
    match make_request("GET", &url, None, None, 30) {
        Ok(b) => ok_str(b),
        Err(e) => {
            eprintln!("RuntimeError [curl.GET]: {}", e);
            ok_str(String::new())
        }
    }
}

pub fn curl_post(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 2);
    let url = arg(&args_list, 0);
    let body = args_list.get(1).map(|s| remove_quoted_str(s.clone()));
    
    match make_request("POST", &url, body.as_deref(), None, 30) {
        Ok(b) => ok_str(b),
        Err(e) => {
            eprintln!("RuntimeError [curl.POST]: {}", e);
            ok_str(String::new())
        }
    }
}

pub fn curl_put(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 2);
    let url = arg(&args_list, 0);
    let body = args_list.get(1).map(|s| remove_quoted_str(s.clone()));
    
    match make_request("PUT", &url, body.as_deref(), None, 30) {
        Ok(b) => ok_str(b),
        Err(e) => {
            eprintln!("RuntimeError [curl.PUT]: {}", e);
            ok_str(String::new())
        }
    }
}

pub fn curl_delete(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 1);
    let url = arg(&args_list, 0);
    
    match make_request("DELETE", &url, None, None, 30) {
        Ok(b) => ok_str(b),
        Err(e) => {
            eprintln!("RuntimeError [curl.DELETE]: {}", e);
            ok_str(String::new())
        }
    }
}

pub fn curl_patch(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 2);
    let url = arg(&args_list, 0);
    let body = args_list.get(1).map(|s| remove_quoted_str(s.clone()));
    
    match make_request("PATCH", &url, body.as_deref(), None, 30) {
        Ok(b) => ok_str(b),
        Err(e) => {
            eprintln!("RuntimeError [curl.PATCH]: {}", e);
            ok_str(String::new())
        }
    }
}

pub fn curl_head(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 1);
    let url = arg(&args_list, 0);
    
    use reqwest::blocking::Client;
    let client = Client::builder()
        .timeout(Duration::from_secs(30))
        .build()
        .map_err(|e| format!("Failed to create HTTP client: {}", e))
        .unwrap();
    
    match client.head(&url).send() {
        Ok(resp) => {
            let status = resp.status().as_u16();
            let headers: HashMap<String, String> = resp
                .headers()
                .iter()
                .filter_map(|(k, v)| {
                    Some((k.to_string(), v.to_str().unwrap_or("").to_string()))
                })
                .collect();
            
            let result = serde_json::json!({
                "status": status,
                "headers": headers
            });
            ok_str(serde_json::to_string_pretty(&result).unwrap_or("{}".to_string()))
        }
        Err(e) => {
            eprintln!("RuntimeError [curl.head]: {}", e);
            ok_str(String::new())
        }
    }
}

pub fn curl_status_code(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 1);
    let url = arg(&args_list, 0);
    
    match get_status_code(&url, "GET") {
        Ok(code) => ok_int(code as i32),
        Err(e) => {
            eprintln!("RuntimeError [curl.status_code]: {}", e);
            ok_int(0)
        }
    }
}

pub fn curl_json(args: String) -> Box<dyn Validator> {
    let args_list = split_args(&args, 2);
    let url = arg(&args_list, 0);
    let body = args_list.get(1).map(|s| remove_quoted_str(s.clone()));
    
    match make_request("POST", &url, body.as_deref(), None, 30) {
        Ok(b) => {
            // Try to parse as JSON and return pretty formatted
            match serde_json::from_str::<serde_json::Value>(&b) {
                Ok(json) => ok_str(serde_json::to_string_pretty(&json).unwrap_or(b)),
                Err(_) => ok_str(b),
            }
        }
        Err(e) => {
            eprintln!("RuntimeError [curl.json]: {}", e);
            ok_str(String::new())
        }
    }
}

use crate::native::create_object::Object;
use linked_hash_map::LinkedHashMap;

pub fn create_curl_object() -> Box<dyn Validator> {
    let mut map = LinkedHashMap::new();
    
    // Main function - curl(url)
    map.insert("call".to_string(), ValueData::NativeFunction(curl_main as fn(String) -> Box<dyn Validator>));
    
    // HTTP methods
    map.insert("GET".to_string(), ValueData::NativeFunction(curl_get as fn(String) -> Box<dyn Validator>));
    map.insert("POST".to_string(), ValueData::NativeFunction(curl_post as fn(String) -> Box<dyn Validator>));
    map.insert("PUT".to_string(), ValueData::NativeFunction(curl_put as fn(String) -> Box<dyn Validator>));
    map.insert("DELETE".to_string(), ValueData::NativeFunction(curl_delete as fn(String) -> Box<dyn Validator>));
    map.insert("PATCH".to_string(), ValueData::NativeFunction(curl_patch as fn(String) -> Box<dyn Validator>));
    map.insert("HEAD".to_string(), ValueData::NativeFunction(curl_head as fn(String) -> Box<dyn Validator>));
    
    // Utility methods
    map.insert("status_code".to_string(), ValueData::NativeFunction(curl_status_code as fn(String) -> Box<dyn Validator>));
    map.insert("json".to_string(), ValueData::NativeFunction(curl_json as fn(String) -> Box<dyn Validator>));
    
    let object = Object::from_map(map);
    Box::new(ValueData::Object(object))
}
