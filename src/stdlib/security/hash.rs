use crate::vm::value::Value;
use crate::error::{AlyError, AlyErrorKind, AlyResult};

pub fn argon2_hash(password: &str) -> AlyResult<String> {
    argon2::password_hash::SaltString::generate(&mut rand::rngs::OsRng);
    let salt = argon2::password_hash::SaltString::generate(&mut rand::rngs::OsRng);
    let config = argon2::Argon2::default();
    let mut hash_bytes = vec![0u8; 32];
    config.hash_password_into(password.as_bytes(), salt.as_ref(), &mut hash_bytes)
        .map_err(|e| AlyError::runtime(format!("Argon2 hash error: {}", e)))?;
    let b64_hash = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &hash_bytes);
    Ok(format!("$argon2id$v=19$m=19456,t=2,p=1${}${}", salt, b64_hash))
}

pub fn argon2_verify(password: &str, hash: &str) -> AlyResult<bool> {
    if !hash.starts_with("$argon2id$") {
        return Err(AlyError::runtime("Invalid Argon2 hash format".to_string()));
    }
    let parts: Vec<&str> = hash.split('$').collect();
    if parts.len() < 5 {
        return Err(AlyError::runtime("Invalid Argon2 hash format".to_string()));
    }
    let salt_b64 = parts[parts.len() - 2];
    let hash_b64 = parts[parts.len() - 1];
    let salt = argon2::password_hash::SaltString::from_b64(salt_b64)
        .map_err(|e| AlyError::runtime(format!("Invalid salt: {}", e)))?;
    let mut expected_hash = vec![0u8; 32];
    let config = argon2::Argon2::default();
    config.hash_password_into(password.as_bytes(), salt.as_ref(), &mut expected_hash)
        .map_err(|e| AlyError::runtime(format!("Argon2 verify error: {}", e)))?;
    let expected_b64 = base64::Engine::encode(&base64::engine::general_purpose::STANDARD, &expected_hash);
    Ok(expected_b64 == hash_b64)
}

pub fn bcrypt_hash(password: &str, cost: u32) -> AlyResult<String> {
    let cost = cost.max(4).min(14);
    bcrypt::hash(password, cost)
        .map_err(|e| AlyError::runtime(format!("Bcrypt hash error: {}", e)))
}

pub fn bcrypt_verify(password: &str, hash: &str) -> AlyResult<bool> {
    bcrypt::verify(password, hash)
        .map_err(|e| AlyError::runtime(format!("Bcrypt verify error: {}", e)))
}

pub fn register(runtime: &mut crate::aly::Runtime) -> AlyResult<()> {
    runtime.register_function("hash_argon2", Value::NativeFn(Box::new(|args| {
        let password = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        argon2_hash(password).map(Value::Str)
    })))?);

    runtime.register_function("hash_argon2_verify", Value::NativeFn(Box::new(|args| {
        let password = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let hash = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
        argon2_verify(password, hash).map(Value::Bool)
    })))?);

    runtime.register_function("hash_bcrypt", Value::NativeFn(Box::new(|args| {
        let password = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let cost = args.get(1).and_then(|v| v.as_int()).unwrap_or(10) as u32;
        bcrypt_hash(password, cost).map(Value::Str)
    })))?);

    runtime.register_function("hash_bcrypt_verify", Value::NativeFn(Box::new(|args| {
        let password = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let hash = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
        bcrypt_verify(password, hash).map(Value::Bool)
    })))?);

    Ok(())
}