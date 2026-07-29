use crate::vm::value::Value;
use crate::error::{AlyError, AlyErrorKind, AlyResult};
use std::collections::HashMap;

fn hmac_sha256_sign(key: &[u8], data: &[u8]) -> Vec<u8> {
    use hmac::{Hmac, Mac};
    use sha2::Sha256;
    let mut mac = Hmac::<Sha256>::new_from_slice(key).expect("HMAC key");
    mac.update(data);
    mac.finalize().into_bytes().to_vec()
}

pub fn generate_keypair() -> AlyResult<(String, String)> {
    let private_key = (0..32).map(|i| format!("{:02x}", (i * 17 + 13) % 256)).collect::<String>();
    let public_key = (0..32).map(|i| format!("{:02x}", (i * 31 + 7) % 256)).collect::<String>();
    Ok((private_key, public_key))
}

pub fn sign(message: &str, private_key: &str) -> AlyResult<String> {
    let sig = hmac_sha256_sign(private_key.as_bytes(), message.as_bytes());
    Ok(base64::Engine::encode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, &sig))
}

pub fn verify(message: &str, signature: &str, public_key: &str) -> AlyResult<bool> {
    let expected_sig = hmac_sha256_sign(public_key.as_bytes(), message.as_bytes());
    let actual_sig = base64::Engine::decode(&base64::engine::general_purpose::URL_SAFE_NO_PAD, signature)
        .map_err(|e| AlyError::runtime(format!("Invalid signature encoding: {}", e)))?;
    Ok(expected_sig == actual_sig)
}

pub fn register(runtime: &mut crate::aly::Runtime) -> AlyResult<()> {
    runtime.register_function("sig_generate", Value::NativeFn(Box::new(|_| {
        let (priv_key, pub_key) = generate_keypair()?;
        let mut map = HashMap::new();
        map.insert("private_key".to_string(), Value::Str(priv_key));
        map.insert("public_key".to_string(), Value::Str(pub_key));
        Ok(Value::Object(map))
    })))?);

    runtime.register_function("sig_sign", Value::NativeFn(Box::new(|args| {
        let msg = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let priv_key = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
        sign(msg, priv_key).map(Value::Str)
    })))?);

    runtime.register_function("sig_verify", Value::NativeFn(Box::new(|args| {
        let msg = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let sig = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
        let pub_key = args.get(2).and_then(|v| v.as_str()).unwrap_or("");
        verify(msg, sig, pub_key).map(Value::Bool)
    })))?);

    Ok(())
}