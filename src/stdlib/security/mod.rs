pub mod jwt;
pub mod oauth2;
pub mod oidc;
pub mod acme;
pub mod signature;
pub mod hash;
pub mod sanitize;
pub mod tls;

use crate::error::AlyResult;

pub fn register(runtime: &mut crate::aly::Runtime) -> AlyResult<()> {
    jwt::register(runtime)?;
    oauth2::register(runtime)?;
    oidc::register(runtime)?;
    acme::register(runtime)?;
    signature::register(runtime)?;
    hash::register(runtime)?;
    sanitize::register(runtime)?;
    tls::register(runtime)?;
    Ok(())
}