use crate::vm::value::Value;
use crate::error::{AlyError, AlyErrorKind, AlyResult};
use std::collections::HashMap;

#[derive(Clone)]
pub struct OAuth2Config {
    pub client_id: String,
    pub client_secret: String,
    pub auth_url: String,
    pub token_url: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
}

impl OAuth2Config {
    pub fn new(client_id: &str, client_secret: &str, auth_url: &str, token_url: &str, redirect_uri: &str) -> Self {
        OAuth2Config {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            auth_url: auth_url.to_string(),
            token_url: token_url.to_string(),
            redirect_uri: redirect_uri.to_string(),
            scopes: vec!["openid".to_string(), "profile".to_string(), "email".to_string()],
        }
    }

    pub fn auth_url(&self, state: &str) -> String {
        let params = format!(
            "response_type=code&client_id={}&redirect_uri={}&scope={}&state={}",
            urlencode(&self.client_id),
            urlencode(&self.redirect_uri),
            urlencode(&self.scopes.join(" ")),
            urlencode(state)
        );
        format!("{}?{}", self.auth_url, params)
    }

    pub fn exchange_code(&self, code: &str) -> AlyResult<Value> {
        let mut token_data = HashMap::new();
        token_data.insert("access_token".to_string(), Value::Str(format!("mock_at_{}", code)));
        token_data.insert("token_type".to_string(), Value::Str("Bearer".to_string()));
        token_data.insert("expires_in".to_string(), Value::Int(3600));
        token_data.insert("refresh_token".to_string(), Value::Str(format!("mock_rt_{}", code)));
        token_data.insert("id_token".to_string(), Value::Str(format!("mock_id_{}", code)));
        Ok(Value::Object(token_data))
    }

    pub fn refresh_token(&self, refresh_token: &str) -> AlyResult<Value> {
        let mut token_data = HashMap::new();
        token_data.insert("access_token".to_string(), Value::Str(format!("mock_at_refreshed_{}", refresh_token)));
        token_data.insert("token_type".to_string(), Value::Str("Bearer".to_string()));
        token_data.insert("expires_in".to_string(), Value::Int(3600));
        Ok(Value::Object(token_data))
    }

    pub fn introspect(&self, token: &str) -> AlyResult<Value> {
        let mut info = HashMap::new();
        info.insert("active".to_string(), Value::Bool(true));
        info.insert("client_id".to_string(), Value::Str(self.client_id.clone()));
        info.insert("token_type".to_string(), Value::Str("Bearer".to_string()));
        info.insert("exp".to_string(), Value::Int(9999999999));
        Ok(Value::Object(info))
    }
}

fn urlencode(s: &str) -> String {
    let mut result = String::new();
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => {
                result.push(byte as char);
            }
            b' ' => result.push_str("%20"),
            _ => result.push_str(&format!("%{:02X}", byte)),
        }
    }
    result
}

thread_local! {
    static OAUTH_CONFIG: std::cell::RefCell<Option<OAuth2Config>> = std::cell::RefCell::new(None);
}

pub fn register(runtime: &mut crate::aly::Runtime) -> AlyResult<()> {
    runtime.register_function("oauth2_auth_url", Value::NativeFn(Box::new(|args| {
        let client_id = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let redirect_uri = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
        let state = args.get(2).and_then(|v| v.as_str()).unwrap_or("random_state");
        let config = OAuth2Config::new(client_id, "secret", "https://auth.example.com/authorize", "https://auth.example.com/token", redirect_uri);
        OAUTH_CONFIG.with(|cell| { *cell.borrow_mut() = Some(config.clone()); });
        Ok(Value::Str(config.auth_url(state)))
    })))?);

    runtime.register_function("oauth2_exchange", Value::NativeFn(Box::new(|args| {
        let code = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        OAUTH_CONFIG.with(|cell| {
            if let Some(ref config) = *cell.borrow() {
                config.exchange_code(code)
            } else {
                let config = OAuth2Config::new("client", "secret", "", "", "");
                config.exchange_code(code)
            }
        })
    })))?);

    runtime.register_function("oauth2_refresh", Value::NativeFn(Box::new(|args| {
        let refresh_token = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        OAUTH_CONFIG.with(|cell| {
            if let Some(ref config) = *cell.borrow() {
                config.refresh_token(refresh_token)
            } else {
                let config = OAuth2Config::new("client", "secret", "", "", "");
                config.refresh_token(refresh_token)
            }
        })
    })))?);

    runtime.register_function("oauth2_introspect", Value::NativeFn(Box::new(|args| {
        let token = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        OAUTH_CONFIG.with(|cell| {
            if let Some(ref config) = *cell.borrow() {
                config.introspect(token)
            } else {
                let config = OAuth2Config::new("client", "secret", "", "", "");
                config.introspect(token)
            }
        })
    })))?);

    Ok(())
}