use crate::error::{AlyError, AlyErrorKind, AlyResult};
use crate::vm::value::Value;
use rcgen::{Certificate, CertificateParams, DistinguishedName, KeyPair, SanType};
use rustls::{
    client::danger::{HandshakeSignatureValid, ServerCertVerifier, ServerCertVerified},
    pki_types::{CertificateDer, PrivateKeyDer, ServerName, UnixTime},
    server::{ClientHello, ResolvesServerCert},
    sign::CertifiedKey,
    RootCertStore, ClientConfig, ServerConfig,
};
use std::collections::HashMap;
use std::sync::{Arc, OnceLock};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use tokio_rustls::{TlsAcceptor, TlsConnector};
use pem::{encode, Pem};
use x509_parser::prelude::*;

static TLS_CONFIG_CACHE: OnceLock<TlsConfigCache> = OnceLock::new();

#[derive(Debug, Clone)]
pub struct TlsConfigCache {
    server_configs: Arc<tokio::sync::RwLock<HashMap<String, Arc<ServerConfig>>>>,
    client_configs: Arc<tokio::sync::RwLock<HashMap<String, Arc<ClientConfig>>>>,
}

impl TlsConfigCache {
    pub fn new() -> Self {
        Self {
            server_configs: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
            client_configs: Arc::new(tokio::sync::RwLock::new(HashMap::new())),
        }
    }

    pub async fn get_or_create_server_config(
        &self,
        cert_pem: &str,
        key_pem: &str,
    ) -> AlyResult<Arc<ServerConfig>> {
        let cache_key = format!("{}:{}", cert_pem, key_pem);
        
        if let Some(config) = self.server_configs.read().await.get(&cache_key) {
            return Ok(config.clone());
        }

        let config = create_server_config(cert_pem, key_pem)?;
        let config_arc = Arc::new(config);
        self.server_configs.write().await.insert(cache_key, config_arc.clone());
        Ok(config_arc)
    }

    pub async fn get_or_create_client_config(
        &self,
        ca_pem: Option<&str>,
        verify_hostname: bool,
    ) -> AlyResult<Arc<ClientConfig>> {
        let cache_key = format!("ca:{}:verify:{}", ca_pem.unwrap_or("none"), verify_hostname);
        
        if let Some(config) = self.client_configs.read().await.get(&cache_key) {
            return Ok(config.clone());
        }

        let config = create_client_config(ca_pem, verify_hostname)?;
        let config_arc = Arc::new(config);
        self.client_configs.write().await.insert(cache_key, config_arc.clone());
        Ok(config_arc)
    }
}

fn create_server_config(cert_pem: &str, key_pem: &str) -> AlyResult<ServerConfig> {
    let cert = parse_cert_pem(cert_pem)?;
    let key = parse_key_pem(key_pem)?;

    let certified_key = CertifiedKey::new(vec![cert], key);
    
    let resolver = Arc::new(SingleCertResolver(certified_key));
    
    let mut config = ServerConfig::builder()
        .with_no_client_auth()
        .with_cert_resolver(resolver);

    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
    
    Ok(config)
}

fn create_client_config(ca_pem: Option<&str>, verify_hostname: bool) -> AlyResult<ClientConfig> {
    let mut root_store = RootCertStore::empty();
    
    if let Some(ca) = ca_pem {
        for cert in parse_pem_certs(ca)? {
            root_store.add(cert)?;
        }
    } else {
        root_store.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    }

    let mut config = ClientConfig::builder()
        .with_root_certificates(root_store)
        .with_no_client_auth();

    if !verify_hostname {
        config = config.with_dangerous_custom_verifier(Arc::new(NoHostnameVerification));
    }

    config.alpn_protocols = vec![b"h2".to_vec(), b"http/1.1".to_vec()];
    config.enable_early_data = true;
    
    Ok(config)
}

fn parse_cert_pem(pem: &str) -> AlyResult<CertificateDer<'static>> {
    let certs = parse_pem_certs(pem)?;
    certs.first().cloned().ok_or_else(|| AlyError::new(AlyErrorKind::Import, "No certificate found in PEM"))
}

fn parse_key_pem(pem: &str) -> AlyResult<PrivateKeyDer<'static>> {
    let pkcs8 = pem::parse(pem)
        .map_err(|e| AlyError::new(AlyErrorKind::Import, format!("Failed to parse PEM: {}", e)))?;
    
    if pkcs8.tag() != "PRIVATE KEY" && pkcs8.tag() != "RSA PRIVATE KEY" {
        return Err(AlyError::new(AlyErrorKind::Import, "Invalid private key PEM"));
    }
    
    Ok(PrivateKeyDer::Pkcs8(pkcs8.contents().into()))
}

fn parse_pem_certs(pem: &str) -> AlyResult<Vec<CertificateDer<'static>>> {
    let mut certs = Vec::new();
    for block in pem::parse_many(pem)? {
        if block.tag() == "CERTIFICATE" {
            certs.push(CertificateDer::from(block.contents()));
        }
    }
    if certs.is_empty() {
        return Err(AlyError::new(AlyErrorKind::Import, "No certificates found in PEM"));
    }
    Ok(certs)
}

struct SingleCertResolver(CertifiedKey);

impl ResolvesServerCert for SingleCertResolver {
    fn resolve(&self, _client_hello: ClientHello<'_>) -> Option<Arc<CertifiedKey>> {
        Some(Arc::new(self.0.clone()))
    }
}

struct NoHostnameVerification;

impl ServerCertVerifier for NoHostnameVerification {
    fn verify_server_cert(
        &self,
        _end_entity: &CertificateDer<'_>,
        _intermediates: &[CertificateDer<'_>],
        _server_name: &ServerName<'_>,
        _ocsp_response: &[u8],
        _now: UnixTime,
    ) -> Result<ServerCertVerified, rustls::Error> {
        Ok(ServerCertVerified::assertion())
    }

    fn verify_tls12_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn verify_tls13_signature(
        &self,
        _message: &[u8],
        _cert: &CertificateDer<'_>,
        _dss: &rustls::DigitallySignedStruct,
    ) -> Result<HandshakeSignatureValid, rustls::Error> {
        Ok(HandshakeSignatureValid::assertion())
    }

    fn supported_verify_schemes(&self) -> Vec<rustls::SignatureScheme> {
        vec![
            rustls::SignatureScheme::RSA_PKCS1_SHA256,
            rustls::SignatureScheme::RSA_PKCS1_SHA384,
            rustls::SignatureScheme::RSA_PKCS1_SHA512,
            rustls::SignatureScheme::ECDSA_NISTP256_SHA256,
            rustls::SignatureScheme::ECDSA_NISTP384_SHA384,
            rustls::SignatureScheme::ED25519,
        ]
    }
}

pub fn get_tls_cache() -> &'static TlsConfigCache {
    TLS_CONFIG_CACHE.get_or_init(TlsConfigCache::new)
}

pub fn create_self_signed_cert(domain: &str) -> AlyResult<(String, String)> {
    let mut params = CertificateParams::new(vec![domain.to_string()])?;
    params.distinguished_name = DistinguishedName::new();
    params.distinguished_name.push(DnType::CommonName, domain);
    params.distinguished_name.push(DnType::OrganizationName, "Aly Self-Signed");
    params.subject_alt_names = vec![SanType::DnsName(domain.to_string())];
    
    let key_pair = KeyPair::generate()?;
    let cert = params.self_signed(&key_pair)?;
    
    let cert_pem = cert.pem();
    let key_pem = key_pair.serialize_pem();
    
    Ok((cert_pem, key_pem))
}

pub async fn serve_https(
    port: u16,
    cert_pem: &str,
    key_pem: &str,
    router: axum::Router,
) -> AlyResult<()> {
    let cache = get_tls_cache();
    let config = cache.get_or_create_server_config(cert_pem, key_pem).await?;
    let acceptor = TlsAcceptor::from(config);
    
    let addr = format!("0.0.0.0:{}", port);
    let listener = tokio::net::TcpListener::bind(&addr).await
        .map_err(|e| AlyError::runtime(format!("Failed to bind to {}: {}", addr, e)))?;
    
    println!("Aly HTTPS server listening on https://{}", addr);
    
    axum::serve(
        tokio_stream::wrappers::TcpListenerStream::new(listener)
            .map(move |stream| {
                let acceptor = acceptor.clone();
                async move {
                    match stream {
                        Ok(tcp) => acceptor.accept(tcp).await.map_err(Into::into),
                        Err(e) => Err(e.into()),
                    }
                }
            }),
        router.into_make_service(),
    ).await
    .map_err(|e| AlyError::runtime(format!("HTTPS server error: {}", e)))?;
    
    Ok(())
}

pub fn create_https_client(
    ca_pem: Option<&str>,
    verify_hostname: bool,
) -> AlyResult<reqwest::Client> {
    let mut builder = reqwest::Client::builder()
        .timeout(Duration::from_secs(30))
        .use_rustls_tls();
    
    if let Some(ca) = ca_pem {
        let cert = reqwest::Certificate::from_pem(ca.as_bytes())
            .map_err(|e| AlyError::new(AlyErrorKind::Import, format!("Invalid CA cert: {}", e)))?;
        builder = builder.add_root_certificate(cert);
    }
    
    if !verify_hostname {
        builder = builder.danger_accept_invalid_hostnames(true)
            .danger_accept_invalid_certs(true);
    }
    
    builder.build()
        .map_err(|e| AlyError::runtime(format!("Failed to create HTTPS client: {}", e)))
}

pub fn cert_info(cert_pem: &str) -> AlyResult<Value> {
    let certs = parse_pem_certs(cert_pem)?;
    let cert = certs.first().ok_or_else(|| AlyError::new(AlyErrorKind::Import, "No certificate"))?;
    
    let parsed = X509Certificate::from_der(cert.as_ref())
        .map_err(|e| AlyError::new(AlyErrorKind::Import, format!("Failed to parse cert: {}", e)))?;
    
    let mut info = HashMap::new();
    info.insert("subject".to_string(), Value::Str(parsed.subject().to_string()));
    info.insert("issuer".to_string(), Value::Str(parsed.issuer().to_string()));
    info.insert("serial".to_string(), Value::Str(parsed.serial_number().to_string()));
    info.insert("not_before".to_string(), Value::Str(parsed.validity().not_before.to_string()));
    info.insert("not_after".to_string(), Value::Str(parsed.validity().not_after.to_string()));
    info.insert("is_ca".to_string(), Value::Bool(parsed.subject_key_identifier().is_some()));
    
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
    let not_after = parsed.validity().not_after.timestamp() as u64;
    info.insert("expired".to_string(), Value::Bool(now > not_after));
    info.insert("days_until_expiry".to_string(), Value::Int(((not_after.saturating_sub(now)) / 86400) as i64));
    
    let sans: Vec<String> = parsed.subject_alternative_name()
        .iter()
        .flat_map(|ext| ext.value().iter())
        .filter_map(|san| match san {
            x509_parser::extensions::GeneralName::DNSName(name) => Some(name.to_string()),
            _ => None,
        })
        .collect();
    info.insert("sans".to_string(), Value::Array(sans.into_iter().map(Value::Str).collect()));
    
    Ok(Value::Object(info))
}

pub fn register(runtime: &mut crate::aly::Runtime) -> AlyResult<()> {
    runtime.register_function("tls_create_self_signed", Value::NativeFn(Box::new(|args| {
        let domain = args.get(0).and_then(|v| v.as_str()).unwrap_or("localhost");
        let (cert, key) = create_self_signed_cert(domain)?;
        let mut map = HashMap::new();
        map.insert("cert_pem".to_string(), Value::Str(cert));
        map.insert("key_pem".to_string(), Value::Str(key));
        Ok(Value::Object(map))
    })))?);

    runtime.register_function("tls_cert_info", Value::NativeFn(Box::new(|args| {
        let cert_pem = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        cert_info(cert_pem)
    })))?);

    runtime.register_function("tls_create_client", Value::NativeFn(Box::new(|args| {
        let ca_pem = args.get(0).and_then(|v| v.as_str());
        let verify = args.get(1).and_then(|v| v.as_bool()).unwrap_or(true);
        let client = create_https_client(ca_pem, verify)?;
        let mut map = HashMap::new();
        map.insert("client".to_string(), Value::Object(map)); // placeholder
        Ok(Value::Object(map))
    })))?);

    Ok(())
}