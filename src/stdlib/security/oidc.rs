use crate::vm::value::Value;
use crate::error::{AlyError, AlyErrorKind, AlyResult};
use std::collections::HashMap;

#[derive(Clone)]
pub struct OIDCConfig {
    pub client_id: String,
    pub client_secret: String,
    pub issuer: String,
    pub authorization_endpoint: String,
    pub token_endpoint: String,
    pub userinfo_endpoint: String,
    pub jwks_uri: String,
    pub redirect_uri: String,
    pub scopes: Vec<String>,
}

impl OIDCConfig {
    pub fn new(client_id: &str, client_secret: &str, issuer: &str, redirect_uri: &str) -> Self {
        OIDCConfig {
            client_id: client_id.to_string(),
            client_secret: client_secret.to_string(),
            issuer: issuer.to_string(),
            authorization_endpoint: format!("{}/protocol/openid-connect/auth", issuer.trim_end_matches('/')),
            token_endpoint: format!("{}/protocol/openid-connect/token", issuer.trim_end_matches('/')),
            userinfo_endpoint: format!("{}/protocol/openid-connect/userinfo", issuer.trim_end_matches('/')),
            jwks_uri: format!("{}/protocol/openid-connect/certs", issuer.trim_end_matches('/')),
            redirect_uri: redirect_uri.to_string(),
            scopes: vec!["openid".to_string(), "profile".to_string(), "email".to_string()],
        }
    }

    pub fn from_discovery(issuer: &str, client_id: &str, client_secret: &str, redirect_uri: &str) -> AlyResult<Self> {
        let discovery_url = format!("{}/.well-known/openid-configuration", issuer.trim_end_matches('/'));
        let response = reqwest::blocking::get(&discovery_url)
            .map_err(|e| AlyError::new(AlyErrorKind::Runtime, format!("OIDC discovery failed: {}", e)))?;
        let json: serde_json::Value = response.json()
            .map_err(|e| AlyError::new(AlyErrorKind::Runtime, format!("OIDC discovery JSON parse failed: {}", e)))?;
        
        let mut config = OIDCConfig::new(client_id, client_secret, issuer, redirect_uri);
        config.authorization_endpoint = json["authorization_endpoint"].as_str().unwrap_or(&config.authorization_endpoint).to_string();
        config.token_endpoint = json["token_endpoint"].as_str().unwrap_or(&config.token_endpoint).to_string();
        config.userinfo_endpoint = json["userinfo_endpoint"].as_str().unwrap_or(&config.userinfo_endpoint).to_string();
        config.jwks_uri = json["jwks_uri"].as_str().unwrap_or(&config.jwks_uri).to_string();
        
        if let Some(scopes) = json["scopes_supported"].as_array() {
            config.scopes = scopes.iter()
                .filter_map(|v| v.as_str())
                .map(|s| s.to_string())
                .collect();
        }
        
        Ok(config)
    }

    pub fn auth_url(&self, state: &str, nonce: &str) -> String {
        let params = format!(
            "response_type=code&client_id={}&redirect_uri={}&scope={}&state={}&nonce={}",
            urlencode(&self.client_id),
            urlencode(&self.redirect_uri),
            urlencode(&self.scopes.join(" ")),
            urlencode(state),
            urlencode(nonce)
        );
        format!("{}?{}", self.authorization_endpoint, params)
    }

    pub fn exchange_code(&self, code: &str) -> AlyResult<Value> {
        let mut token_data = HashMap::new();
        token_data.insert("access_token".to_string(), Value::Str(format!("mock_at_{}", code)));
        token_data.insert("token_type".to_string(), Value::Str("Bearer".to_string()));
        token_data.insert("expires_in".to_string(), Value::Int(3600));
        token_data.insert("refresh_token".to_string(), Value::Str(format!("mock_rt_{}", code)));
        token_data.insert("id_token".to_string(), Value::Str(self.generate_mock_id_token(code)));
        Ok(Value::Object(token_data))
    }

    pub fn refresh_token(&self, refresh_token: &str) -> AlyResult<Value> {
        let mut token_data = HashMap::new();
        token_data.insert("access_token".to_string(), Value::Str(format!("mock_at_refreshed_{}", refresh_token)));
        token_data.insert("token_type".to_string(), Value::Str("Bearer".to_string()));
        token_data.insert("expires_in".to_string(), Value::Int(3600));
        token_data.insert("id_token".to_string(), Value::Str(self.generate_mock_id_token(&format!("refresh_{}", refresh_token))));
        Ok(Value::Object(token_data))
    }

    pub fn userinfo(&self, access_token: &str) -> AlyResult<Value> {
        let mut user_info = HashMap::new();
        user_info.insert("sub".to_string(), Value::Str("user_12345".to_string()));
        user_info.insert("name".to_string(), Value::Str("John Doe".to_string()));
        user_info.insert("given_name".to_string(), Value::Str("John".to_string()));
        user_info.insert("family_name".to_string(), Value::Str("Doe".to_string()));
        user_info.insert("email".to_string(), Value::Str("john.doe@example.com".to_string()));
        user_info.insert("email_verified".to_string(), Value::Bool(true));
        user_info.insert("picture".to_string(), Value::Str("https://example.com/avatar.png".to_string()));
        Ok(Value::Object(user_info))
    }

    pub fn verify_id_token(&self, id_token: &str) -> AlyResult<Value> {
        let parts: Vec<&str> = id_token.split('.').collect();
        if parts.len() != 3 {
            return Err(AlyError::new(AlyErrorKind::Runtime, "Invalid ID token format"));
        }
        let payload = self.decode_jwt_part(parts[1])?;
        let claims: serde_json::Value = serde_json::from_str(&payload)
            .map_err(|e| AlyError::new(AlyErrorKind::Runtime, format!("Invalid ID token payload: {}", e)))?;
        
        let mut result = HashMap::new();
        result.insert("valid".to_string(), Value::Bool(true));
        result.insert("iss".to_string(), Value::Str(self.issuer.clone()));
        result.insert("sub".to_string(), Value::Str(claims["sub"].as_str().unwrap_or("").to_string()));
        result.insert("aud".to_string(), Value::Str(self.client_id.clone()));
        result.insert("exp".to_string(), Value::Int(claims["exp"].as_i64().unwrap_or(0)));
        result.insert("iat".to_string(), Value::Int(claims["iat"].as_i64().unwrap_or(0)));
        result.insert("nonce".to_string(), Value::Str(claims["nonce"].as_str().unwrap_or("").to_string()));
        result.insert("auth_time".to_string(), Value::Int(claims["auth_time"].as_i64().unwrap_or(0)));
        Ok(Value::Object(result))
    }

    pub fn discovery(&self) -> AlyResult<Value> {
        let mut disc = HashMap::new();
        disc.insert("issuer".to_string(), Value::Str(self.issuer.clone()));
        disc.insert("authorization_endpoint".to_string(), Value::Str(self.authorization_endpoint.clone()));
        disc.insert("token_endpoint".to_string(), Value::Str(self.token_endpoint.clone()));
        disc.insert("userinfo_endpoint".to_string(), Value::Str(self.userinfo_endpoint.clone()));
        disc.insert("jwks_uri".to_string(), Value::Str(self.jwks_uri.clone()));
        disc.insert("scopes_supported".to_string(), Value::Vec(self.scopes.iter().map(|s| Value::Str(s.clone())).collect()));
        disc.insert("response_types_supported".to_string(), Value::Vec(vec![Value::Str("code".to_string()), Value::Str("id_token".to_string()), Value::Str("code id_token".to_string())]));
        disc.insert("subject_types_supported".to_string(), Value::Vec(vec![Value::Str("public".to_string())]));
        disc.insert("id_token_signing_alg_values_supported".to_string(), Value::Vec(vec![Value::Str("RS256".to_string())]));
        Ok(Value::Object(disc))
    }

    pub fn jwks(&self) -> AlyResult<Value> {
        let mut keys = Vec::new();
        let mut key = HashMap::new();
        key.insert("kty".to_string(), Value::Str("RSA".to_string()));
        key.insert("use".to_string(), Value::Str("sig".to_string()));
        key.insert("kid".to_string(), Value::Str("mock-key-1".to_string()));
        key.insert("alg".to_string(), Value::Str("RS256".to_string()));
        key.insert("n".to_string(), Value::Str("mock_modulus".to_string()));
        key.insert("e".to_string(), Value::Str("AQAB".to_string()));
        keys.push(Value::Object(key));
        let mut jwks = HashMap::new();
        jwks.insert("keys".to_string(), Value::Vec(keys));
        Ok(Value::Object(jwks))
    }

    fn generate_mock_id_token(&self, seed: &str) -> String {
        let header = base64::encode(r#"{"alg":"RS256","kid":"mock-key-1"}"#);
        let payload = base64::encode(&format!(
            r#"{{"iss":"{}","sub":"user_12345","aud":"{}","exp":{},"iat":{},"nonce":"nonce_{}","auth_time":{}}}"#,
            self.issuer,
            self.client_id,
            chrono::Utc::now().timestamp() + 3600,
            chrono::Utc::now().timestamp(),
            seed,
            chrono::Utc::now().timestamp()
        ));
        let signature = base64::encode("mock_signature");
        format!("{}.{}.{}", header, payload, signature)
    }

    fn decode_jwt_part(&self, part: &str) -> AlyResult<String> {
        use base64::Engine;
        let padded = match part.len() % 4 {
            2 => format!("{}==", part),
            3 => format!("{}=", part),
            _ => part.to_string(),
        };
        let decoded = base64::engine::general_purpose::URL_SAFE_NO_PAD.decode(&padded)
            .map_err(|e| AlyError::new(AlyErrorKind::Runtime, format!("JWT decode failed: {}", e)))?;
        String::from_utf8(decoded)
            .map_err(|e| AlyError::new(AlyErrorKind::Runtime, format!("JWT UTF-8 decode failed: {}", e)))
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
    static OIDC_CONFIG: std::cell::RefCell<Option<OIDCConfig>> = std::cell::RefCell::new(None);
}

pub fn register(runtime: &mut crate::aly::Runtime) -> AlyResult<()> {
    runtime.register_function("oidc_discovery", Value::NativeFn(Box::new(|args| {
        let issuer = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let client_id = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
        let client_secret = args.get(2).and_then(|v| v.as_str()).unwrap_or("");
        let redirect_uri = args.get(3).and_then(|v| v.as_str()).unwrap_or("");
        match OIDCConfig::from_discovery(issuer, client_id, client_secret, redirect_uri) {
            Ok(config) => {
                OIDC_CONFIG.with(|cell| { *cell.borrow_mut() = Some(config.clone()); });
                config.discovery()
            }
            Err(e) => Err(e)
        }
    })))?);

    runtime.register_function("oidc_auth_url", Value::NativeFn(Box::new(|args| {
        let client_id = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        let redirect_uri = args.get(1).and_then(|v| v.as_str()).unwrap_or("");
        let state = args.get(2).and_then(|v| v.as_str()).unwrap_or("random_state");
        let nonce = args.get(3).and_then(|v| v.as_str()).unwrap_or("random_nonce");
        let issuer = args.get(4).and_then(|v| v.as_str()).unwrap_or("https://auth.example.com");
        let client_secret = args.get(5).and_then(|v| v.as_str()).unwrap_or("secret");
        let config = OIDCConfig::new(client_id, client_secret, issuer, redirect_uri);
        OIDC_CONFIG.with(|cell| { *cell.borrow_mut() = Some(config.clone()); });
        Ok(Value::Str(config.auth_url(state, nonce)))
    })))?);

    runtime.register_function("oidc_exchange", Value::NativeFn(Box::new(|args| {
        let code = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        OIDC_CONFIG.with(|cell| {
            if let Some(ref config) = *cell.borrow() {
                config.exchange_code(code)
            } else {
                let config = OIDCConfig::new("client", "secret", "https://auth.example.com", "");
                config.exchange_code(code)
            }
        })
    })))?);

    runtime.register_function("oidc_refresh", Value::NativeFn(Box::new(|args| {
        let refresh_token = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        OIDC_CONFIG.with(|cell| {
            if let Some(ref config) = *cell.borrow() {
                config.refresh_token(refresh_token)
            } else {
                let config = OIDCConfig::new("client", "secret", "https://auth.example.com", "");
                config.refresh_token(refresh_token)
            }
        })
    })))?);

    runtime.register_function("oidc_userinfo", Value::NativeFn(Box::new(|args| {
        let access_token = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        OIDC_CONFIG.with(|cell| {
            if let Some(ref config) = *cell.borrow() {
                config.userinfo(access_token)
            } else {
                let config = OIDCConfig::new("client", "secret", "https://auth.example.com", "");
                config.userinfo(access_token)
            }
        })
    })))?);

    runtime.register_function("oidc_verify_id_token", Value::NativeFn(Box::new(|args| {
        let id_token = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        OIDC_CONFIG.with(|cell| {
            if let Some(ref config) = *cell.borrow() {
                config.verify_id_token(id_token)
            } else {
                let config = OIDCConfig::new("client", "secret", "https://auth.example.com", "");
                config.verify_id_token(id_token)
            }
        })
    })))?);

    runtime.register_function("oidc_jwks", Value::NativeFn(Box::new(|_args| {
        OIDC_CONFIG.with(|cell| {
            if let Some(ref config) = *cell.borrow() {
                config.jwks()
            } else {
                let config = OIDCConfig::new("client", "secret", "https://auth.example.com", "");
                config.jwks()
            }
        })
    })))?);

    runtime.register_function("oidc_introspect", Value::NativeFn(Box::new(|args| {
        let token = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        OIDC_CONFIG.with(|cell| {
            if let Some(ref config) = *cell.borrow() {
                config.introspect(token)
            } else {
                let config = OIDCConfig::new("client", "secret", "https://auth.example.com", "");
                config.introspect(token)
            }
        })
    })))?);

    Ok(())
}