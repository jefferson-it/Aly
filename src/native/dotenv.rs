mod dotenv_lib {
    use std::fs;

    use crate::native::types::Validator;
    use crate::validators::str::remove_quoted_str;

    // dotenv(".env.development") or dotenv() -- reads .env by default
    pub fn fun_dotenv(x: String) -> Box<dyn Validator> {
        let path = if x.trim().is_empty() {
            ".env".to_string()
        } else {
            remove_quoted_str(x.trim().to_string())
        };

        let content = match fs::read_to_string(&path) {
            Ok(c) => c,
            Err(_) => return Box::new("None".to_owned()),
        };

        for line in content.lines() {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            let line = line.strip_prefix("export ").unwrap_or(line).trim();

            if let Some(eq_pos) = line.find('=') {
                let key = line[..eq_pos].trim();
                let mut value = line[eq_pos + 1..].trim().to_string();

                // Strip surrounding quotes if present
                if (value.starts_with('"') && value.ends_with('"'))
                    || (value.starts_with('\'') && value.ends_with('\''))
                {
                    value = value[1..value.len() - 1].to_string();
                }

                if !key.is_empty() {
                    std::env::set_var(key, &value);
                }
            }
        }

        Box::new("None".to_owned())
    }
}

pub use dotenv_lib::*;
