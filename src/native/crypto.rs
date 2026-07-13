mod crypto_mod {
    use crate::native::types::{Validator, ValueData};
    use crate::validators::str::{put_quoted_str, remove_quoted_str};
    use crate::native::std::{split_args, arg as std_arg};

    fn hex_digest(bytes: &[u8]) -> String {
        bytes.iter().map(|b| format!("{:02x}", b)).collect()
    }

    // crypto.md5(string) -> string (hex digest)
    pub fn crypto_md5(x: String) -> Box<dyn Validator> {
        use md5::Digest;
        let input = std_arg(&split_args(&x, 1), 0);
        let hash = md5::Md5::digest(input.as_bytes());
        Box::new(put_quoted_str(hex_digest(&hash)))
    }

    // crypto.sha256(string) -> string (hex digest)
    pub fn crypto_sha256(x: String) -> Box<dyn Validator> {
        use sha2::Digest;
        let input = std_arg(&split_args(&x, 1), 0);
        let mut hasher = sha2::Sha256::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        Box::new(put_quoted_str(hex_digest(&result)))
    }

    // crypto.sha512(string) -> string (hex digest)
    pub fn crypto_sha512(x: String) -> Box<dyn Validator> {
        use sha2::Digest;
        let input = std_arg(&split_args(&x, 1), 0);
        let mut hasher = sha2::Sha512::new();
        hasher.update(input.as_bytes());
        let result = hasher.finalize();
        Box::new(put_quoted_str(hex_digest(&result)))
    }

    // crypto.hmac(algorithm, key, data) -> string (hex digest)
    // algorithm: "sha256" or "sha512"
    pub fn crypto_hmac(x: String) -> Box<dyn Validator> {
        use hmac::Mac;
        let args = split_args(&x, 3);
        let algorithm = std_arg(&args, 0).to_lowercase();
        let key = std_arg(&args, 1);
        let data = std_arg(&args, 2);

        let result: Option<Vec<u8>> = match algorithm.as_str() {
            "sha256" => {
                match hmac::Hmac::<sha2::Sha256>::new_from_slice(key.as_bytes()) {
                    Ok(mut mac) => {
                        mac.update(data.as_bytes());
                        Some(mac.finalize().into_bytes().to_vec())
                    }
                    Err(e) => {
                        eprintln!("RuntimeError [crypto.hmac]: chave HMAC inválida: {}", e);
                        None
                    }
                }
            }
            "sha512" => {
                match hmac::Hmac::<sha2::Sha512>::new_from_slice(key.as_bytes()) {
                    Ok(mut mac) => {
                        mac.update(data.as_bytes());
                        Some(mac.finalize().into_bytes().to_vec())
                    }
                    Err(e) => {
                        eprintln!("RuntimeError [crypto.hmac]: chave HMAC inválida: {}", e);
                        None
                    }
                }
            }
            _ => {
                eprintln!("RuntimeError [crypto.hmac]: algoritmo não suportado '{}'. Use 'sha256' ou 'sha512'.", algorithm);
                None
            }
        };

        match result {
            Some(res) => Box::new(put_quoted_str(hex_digest(&res))),
            None => Box::new(put_quoted_str(String::new())),
        }
    }
}

pub use crypto_mod::*;
