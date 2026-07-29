mod tls_mod {
    use crate::native::std::{arg as std_arg, split_args};
    use crate::native::types::Validator;
    use crate::validators::str::put_quoted_str;

    /// Fetch a URL with TLS support via reqwest/rustls.
    /// Usage: tls.fetch(method, url, body?)
    pub fn tls_fetch(args: String) -> Box<dyn Validator> {
        use std::time::Duration;

        let args_list = split_args(&args, 3);
        let method = std_arg(&args_list, 0).to_uppercase();
        let url = std_arg(&args_list, 1);
        let body = if args_list.len() > 2 && std_arg(&args_list, 2) != "None" {
            std_arg(&args_list, 2)
        } else {
            String::new()
        };

        let client = match reqwest::blocking::Client::builder()
            .timeout(Duration::from_secs(60))
            .danger_accept_invalid_certs(false)
            .user_agent("Aly/1.0")
            .build()
        {
            Ok(c) => c,
            Err(e) => {
                eprintln!("RuntimeError [tls.fetch]: {}", e);
                return Box::new(put_quoted_str(String::new()));
            }
        };

        let req = match method.as_str() {
            "GET" => client.get(&url),
            "POST" => client.post(&url).body(body),
            "PUT" => client.put(&url).body(body),
            "DELETE" => client.delete(&url),
            "PATCH" => client.patch(&url).body(body),
            "HEAD" => client.head(&url),
            _ => {
                eprintln!("RuntimeError [tls.fetch]: método não suportado: {}", method);
                return Box::new(put_quoted_str(String::new()));
            }
        };

        match req.send() {
            Ok(resp) => {
                let status = resp.status().as_u16();
                let body_text = resp.text().unwrap_or_default();
                let result = format!(
                    "{{\"status\":{}, \"body\":\"{}\"}}",
                    status,
                    body_text.replace('\\', "\\\\").replace('"', "\\\"")
                );
                Box::new(put_quoted_str(result))
            }
            Err(e) => {
                eprintln!("RuntimeError [tls.fetch]: {}", e);
                Box::new(put_quoted_str(String::new()))
            }
        }
    }
}

pub use tls_mod::*;
