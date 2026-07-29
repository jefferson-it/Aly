use crate::vm::value::Value;
use crate::error::{AlyError, AlyErrorKind, AlyResult};
use std::collections::HashMap;

fn base64url_encode(data: &[u8]) -> String {
    let encoded = base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, data);
    encoded
}

fn base64url_decode(s: &str) -> AlyResult<Vec<u8>> {
    base64::Engine::decode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, s)
        .map_err(|e| AlyError::runtime(format!("Base64 decode error: {}", e)))
}

fn hmac_sha256_sign(secret: &str, data: &str) -> Vec<u8> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    let mut mac = Hmac::<Sha256>::new_from_slice(secret.as_bytes())
        .expect("HMAC key");
    mac.update(data.as_bytes());
    mac.finalize().into_bytes().to_vec()
}

pub fn sign(payload: &Value, secret: &str) -> AlyResult<String> {
    let header = serde_json::json!({"alg": "HS256", "typ": "JWT"});
    let payload_json = serde_json::to_value(payload_to_json(payload))
        .map_err(|e| AlyError::runtime(format!("JSON error: {}", e)))?;
    let header_b64 = base64url_encode(&serde_json::to_vec(&header).unwrap());
    let payload_b64 = base64url_encode(&serde_json::to_vec(&payload_json).unwrap());
    let signing_input = format!("{}.{}", header_b64, payload_b64);
    let signature = hmac_sha256_sign(secret, &signing_input);
    let sig_b64 = base64url_encode(&signature);
    Ok(format!("{}.{}", signing_input, sig_b64))
}

pub fn verify(token: &str, secret: &str) -> AlyResult<Value> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(AlyError::runtime("Invalid JWT format: expected 3 parts".to_string()));
    }
    let signing_input = format!("{}.{}", parts[0], parts[1]);
    let expected_sig = hmac_sha256_sign(secret, &signing_input);
    let actual_sig = base64url_decode(parts[2])?;
    if expected_sig != actual_sig {
        return Err(AlyError::runtime("JWT signature verification failed".to_string()));
    }
    decode(token)
}

pub fn decode(token: &str) -> AlyResult<Value> {
    let parts: Vec<&str> = token.split('.').collect();
    if parts.len() != 3 {
        return Err(AlyError::runtime("Invalid JWT format".to_string()));
    }
    let payload_bytes = base64url_decode(parts[1])?;
    let payload_str = String::from_utf8(payload_bytes)
        .map_err(|e| AlyError::runtime(format!("Invalid UTF-8 in JWT payload: {}", e)))?;
    let parsed: serde_json::Value = serde_json::from_str(&payload_str)
        .map_err(|e| AlyError::runtime(format!("Invalid JSON in JWT payload: {}", e)))?;
    Ok(json_to_aly_value(&parsed))
}

fn payload_to_json(v: &Value) -> serde_json::Value {
    match v {
        Value::Nil => serde_json::Value::Null,
        Value::Bool(b) => serde_json::Value::Bool(*b),
        Value::Int(i) => serde_json::Value::Number((*i).into()),
        Value::Float(f) => serde_json::Value::Number(serde_json::Number::from_f64(*f).unwrap_or(0.into())),
        Value::Str(s) => serde_json::Value::String(s.clone()),
        Value::Object(map) => {
            let mut m = serde_json::Map::new();
            for (k, v) in map { m.insert(k.clone(), payload_to_json(v)); }
            serde_json::Value::Object(m)
        }
        _ => serde_json::Value::Null,
    }
}

fn json_to_aly_value(v: &serde_json::Value) -> Value {
    match v {
        serde_json::Value::Null => Value::Nil,
        serde_json::Value::Bool(b) => Value::Bool(*b),
        serde_json::Value::Number(n) => {
            n.as_i64().map(Value::Int)
                .or_else(|| n.as_f64().map(Value::Float))
                .unwrap_or(Value::Int(0))
        }
        serde_json::Value::String(s) => Value::Str(s.clone()),
        serde_json::Value::Array(arr) => {
            let items: Vec<Value> = arr.iter().map(json_to_aly_value).collect();
            Value::Array(crate::native::vector::AlyVector::from_elements(items))
        }
        serde_json::Value::Object(obj) => {
            let mut map = HashMap::new();
            for (k, v) in obj { map.insert(k.clone(), json_to_aly_value(v)); }
            Value::Object(map)
        }
    }
}

pub fn register(runtime: &mut crate::aly::Runtime) -> AlyResult<()> {
    runtime.register_function("jwt_sign", Value::NativeFn(Box::new(|args| {
        let payload = args.get(0).cloned().unwrap_or(Value::Object(HashMap::new()));
        let secret = args.get(1).and_then(|v| v.as_str()).unwrap_or("secret");
        sign(&payload, secret).map(Value::Str)
    })))?);

    runtime.register_function("jwt_verify", Value::NativeFn(Box::new(|args| {
        let token = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let secret = args.get(1).and_then(|v| v.as_str()).unwrap_or("secret");
        verify(token, secret)
    })))?);

    runtime.register_function("jwt_decode", Value::NativeFn(Box::new(|args| {
        let token = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        decode(token)
    })))?);

    Ok(())
}