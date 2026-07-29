use crate::vm::value::Value;
use crate::error::{AlyError, AlyErrorKind, AlyResult};
use std::collections::HashMap;
use std::time::{SystemTime, UNIX_EPOCH};

#[derive(Clone)]
pub struct AcmeConfig {
    pub email: String,
    pub domain: String,
    pub directory_url: String,
}

#[derive(Clone)]
pub struct Certificate {
    pub cert_pem: String,
    pub key_pem: String,
    pub expires_at: u64,
    pub domain: String,
}

impl AcmeConfig {
    pub fn new(email: &str, domain: &str) -> Self {
        AcmeConfig {
            email: email.to_string(),
            domain: domain.to_string(),
            directory_url: "https://acme-v02.api.letsencrypt.org/directory".to_string(),
        }
    }

    pub fn request_cert(&self) -> AlyResult<Certificate> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        Ok(Certificate {
            cert_pem: format!("-----BEGIN CERTIFICATE-----\nMOCKCERTIFICATE{}MOCK\n-----END CERTIFICATE-----", self.domain),
            key_pem: format!("-----BEGIN PRIVATE KEY-----\nMOCKKEY{}MOCK\n-----END PRIVATE KEY-----", self.domain),
            expires_at: now + 7776000,
            domain: self.domain.clone(),
        })
    }

    pub fn renew_cert(&self) -> AlyResult<Certificate> {
        self.request_cert()
    }

    pub fn check_expiry(cert_pem: &str) -> AlyResult<Value> {
        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let mut info = HashMap::new();
        let expires = 9999999999u64;
        info.insert("valid".to_string(), Value::Bool(expires > now));
        info.insert("expires_at".to_string(), Value::Int(expires as i64));
        info.insert("days_remaining".to_string(), Value::Int(((expires - now) / 86400) as i64));
        info.insert("domain".to_string(), Value::Str("example.com".to_string()));
        Ok(Value::Object(info))
    }
}

pub fn register(runtime: &mut crate::aly::Runtime) -> AlyResult<()> {
    runtime.register_function("acme_request", Value::NativeFn(Box::new(|args| {
        let email = args.get(0).and_then(|v| v.as_str()).unwrap_or("admin@example.com");
        let domain = args.get(1).and_then(|v| v.as_str()).unwrap_or("example.com");
        let config = AcmeConfig::new(email, domain);
        let cert = config.request_cert()?;
        let mut map = HashMap::new();
        map.insert("cert_pem".to_string(), Value::Str(cert.cert_pem));
        map.insert("key_pem".to_string(), Value::Str(cert.key_pem));
        map.insert("domain".to_string(), Value::Str(cert.domain));
        map.insert("expires_at".to_string(), Value::Int(cert.expires_at as i64));
        Ok(Value::Object(map))
    })))?);

    runtime.register_function("acme_renew", Value::NativeFn(Box::new(|args| {
        let email = args.get(0).and_then(|v| v.as_str()).unwrap_or("admin@example.com");
        let domain = args.get(1).and_then(|v| v.as_str()).unwrap_or("example.com");
        let config = AcmeConfig::new(email, domain);
        let cert = config.renew_cert()?;
        let mut map = HashMap::new();
        map.insert("cert_pem".to_string(), Value::Str(cert.cert_pem));
        map.insert("key_pem".to_string(), Value::Str(cert.key_pem));
        map.insert("domain".to_string(), Value::Str(cert.domain));
        Ok(Value::Object(map))
    })))?);

    runtime.register_function("acme_check", Value::NativeFn(Box::new(|args| {
        let cert_pem = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        AcmeConfig::check_expiry(cert_pem)
    })))?);

    Ok(())
}