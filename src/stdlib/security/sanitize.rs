use crate::vm::value::Value;
use crate::error::{AlyError, AlyErrorKind, AlyResult};

fn html_escape(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    for c in s.chars() {
        match c {
            '&' => out.push_str("&amp;"),
            '<' => out.push_str("&lt;"),
            '>' => out.push_str("&gt;"),
            '"' => out.push_str("&quot;"),
            '\'' => out.push_str("&#x27;"),
            '/' => out.push_str("&#x2F;"),
            _ => out.push(c),
        }
    }
    out
}

fn urlencode(s: &str) -> String {
    let mut out = String::new();
    for byte in s.bytes() {
        match byte {
            b'A'..=b'Z' | b'a'..=b'z' | b'0'..=b'9' | b'-' | b'_' | b'.' | b'~' => out.push(byte as char),
            b' ' => out.push_str("%20"),
            _ => out.push_str(&format!("%{:02X}", byte)),
        }
    }
    out
}

pub fn sanitize_sql(input: &str) -> String {
    input.replace('\'', "''")
        .replace('\\', "\\\\")
        .replace('\0', "")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t")
}

pub fn sanitize_html(input: &str) -> String {
    html_escape(input)
}

pub fn sanitize_url(input: &str) -> String {
    urlencode(input)
}

pub fn sanitize_email(email: &str) -> AlyResult<String> {
    let trimmed = email.trim();
    let email_re = regex::Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$").unwrap();
    if !email_re.is_match(trimmed) {
        return Err(AlyError::runtime(format!("Invalid email format: {}", trimmed)));
    }
    let parts: Vec<&str> = trimmed.splitn(2, '@').collect();
    let local = parts[0].to_lowercase();
    let domain = parts[1].to_lowercase();
    Ok(format!("{}@{}", local, domain))
}

pub fn sanitize_filename(name: &str) -> String {
    let sanitized: String = name.chars()
        .map(|c| match c {
            '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|' | '\0' => '_',
            _ => c,
        })
        .collect();
    sanitized.trim_matches(&['.', ' ', '_'][..]).to_string()
}

pub fn contains_sqli(input: &str) -> bool {
    let lower = input.to_lowercase();
    let sqli_patterns = [
        "' OR '1'='1", "1' OR '1'='1", "1 OR 1=1", "' OR 1=1",
        "'; DROP TABLE", "'; DELETE", "'; UPDATE", "'; INSERT",
        "admin'--", "UNION SELECT", "-- ", "/*", "*/", "xp_cmdshell",
        "exec xp_", "EXEC(", "1; DROP", "1'; DROP",
    ];
    sqli_patterns.iter().any(|p| lower.contains(p))
}

pub fn contains_xss(input: &str) -> bool {
    let lower = input.to_lowercase();
    let xss_patterns = [
        "<script", "<iframe", "<object", "<embed", "<svg/onload",
        "onerror=", "onclick=", "onload=", "onmouseover=",
        "javascript:", "alert(", "document.cookie", "fromcharcode",
        "eval(", "<img src=x", "<body onload",
    ];
    xss_patterns.iter().any(|p| lower.contains(p))
}

pub fn register(runtime: &mut crate::aly::Runtime) -> AlyResult<()> {
    runtime.register_function("sanitize_sql", Value::NativeFn(Box::new(|args| {
        let input = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        Ok(Value::Str(sanitize_sql(input)))
    })))?);

    runtime.register_function("sanitize_html", Value::NativeFn(Box::new(|args| {
        let input = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        Ok(Value::Str(sanitize_html(input)))
    })))?);

    runtime.register_function("sanitize_url", Value::NativeFn(Box::new(|args| {
        let input = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        Ok(Value::Str(sanitize_url(input)))
    })))?);

    runtime.register_function("sanitize_email", Value::NativeFn(Box::new(|args| {
        let input = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        sanitize_email(input).map(Value::Str)
    })))?);

    runtime.register_function("sanitize_filename", Value::NativeFn(Box::new(|args| {
        let input = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        Ok(Value::Str(sanitize_filename(input)))
    })))?);

    runtime.register_function("check_sqli", Value::NativeFn(Box::new(|args| {
        let input = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        Ok(Value::Bool(contains_sqli(input)))
    })))?);

    runtime.register_function("check_xss", Value::NativeFn(Box::new(|args| {
        let input = args.get(0).and_then(|v| v.as_str()).unwrap_or("");
        Ok(Value::Bool(contains_xss(input)))
    })))?);

    Ok(())
}