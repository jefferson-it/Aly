use std::process::Command;
use std::path::Path;
use crate::runtime::jot::{JotBackend, JotValue as RuntimeJotValue, JotError};

pub struct ExternalBackend {
    pub binary_path: String,
}

impl ExternalBackend {
    pub fn new<P: AsRef<Path>>(binary_path: P) -> Result<Self, String> {
        let path = binary_path.as_ref().to_string_lossy().to_string();
        if path != "jot" && !Path::new(&path).exists() {
            return Err(format!("JOT binary not found: {}", path));
        }
        Ok(Self { binary_path: path })
    }
}

impl JotBackend for ExternalBackend {
    fn parse_string(&self, source: &str) -> Result<RuntimeJotValue, JotError> {
        let tmp_dir = std::env::temp_dir();
        let tmp_file = tmp_dir.join(format!("jot_{}.jot", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::write(&tmp_file, source).map_err(|e| JotError::Io(e.to_string()))?;
        
        let result = self.parse_file(tmp_file.to_str().unwrap());
        
        let _ = std::fs::remove_file(&tmp_file);
        result
    }

    fn parse_file(&self, path: &str) -> Result<RuntimeJotValue, JotError> {
        let output = Command::new(&self.binary_path)
            .args(["conv", path, "json"])
            .output()
            .map_err(|e| JotError::Io(format!("Failed to execute jot: {}", e)))?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(JotError::Parse(stderr.to_string()));
        }

        let json_str = String::from_utf8_lossy(&output.stdout);
        json_to_jot_value(&json_str)
    }

    fn to_format(&self, value: &RuntimeJotValue, format: &str) -> Result<String, JotError> {
        let jot_str = jot_value_to_jot_string(value);
        let tmp_dir = std::env::temp_dir();
        let tmp_file = tmp_dir.join(format!("jot_{}.jot", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::write(&tmp_file, jot_str).map_err(|e| JotError::Io(e.to_string()))?;

        let output = Command::new(&self.binary_path)
            .args(["conv", tmp_file.to_str().unwrap(), format])
            .output()
            .map_err(|e| JotError::Io(format!("Failed to execute jot: {}", e)))?;

        let _ = std::fs::remove_file(&tmp_file);

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(JotError::Convert(stderr.to_string()));
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    fn pick(&self, value: &RuntimeJotValue, path: &str) -> Result<String, JotError> {
        let jot_str = jot_value_to_jot_string(value);
        let tmp_dir = std::env::temp_dir();
        let tmp_file = tmp_dir.join(format!("jot_{}.jot", std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::write(&tmp_file, jot_str).map_err(|e| JotError::Io(e.to_string()))?;

        let output = Command::new(&self.binary_path)
            .args(["pick", tmp_file.to_str().unwrap(), path])
            .output()
            .map_err(|e| JotError::Io(format!("Failed to execute jot: {}", e)))?;

        let _ = std::fs::remove_file(&tmp_file);

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            return Err(JotError::Pick(stderr.to_string()));
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    fn format_jot(&self, value: &RuntimeJotValue) -> String {
        jot_value_to_jot_string(value)
    }
}

fn jot_value_to_jot_string(value: &RuntimeJotValue) -> String {
    fn inner(val: &RuntimeJotValue, indent: usize) -> String {
        let pad = "    ".repeat(indent);
        match val {
            RuntimeJotValue::Null => format!("{}null", pad),
            RuntimeJotValue::Boolean(b) => format!("{}{}", pad, b),
            RuntimeJotValue::Integer(i) => format!("{}{}", pad, i),
            RuntimeJotValue::Float(f) => {
                let s = f.to_string();
                if s.contains('.') || s.contains('e') { format!("{}{}", pad, s) } else { format!("{}{}.0", pad, f) }
            }
            RuntimeJotValue::String(s) => format!("{}{}", pad, s),
            RuntimeJotValue::Array(arr) => {
                if arr.is_empty() { return format!("{}[]", pad); }
                let mut out = format!("{}[\n", pad);
                let item_pad = "    ".repeat(indent + 1);
                for v in arr {
                    out.push_str(&format!("{}{}\n", item_pad, inner(v, 0).trim_start()));
                }
                out.push_str(&format!("{}]", pad));
                out
            }
            RuntimeJotValue::Object(map) => {
                if map.is_empty() { return format!("{} {{}}", pad); }
                let mut out = String::new();
                for (k, v) in map {
                    match v {
                        RuntimeJotValue::Object(_) | RuntimeJotValue::Array(_) => {
                            out.push_str(&format!("{}:\n{}", k, inner(v, indent + 1)));
                        }
                        _ => {
                            out.push_str(&format!("{} {} {}\n", pad, k, inner(v, 0).trim()));
                        }
                    }
                }
                out.trim_end().to_string()
            }
        }
    }
    inner(value, 0)
}

fn json_to_jot_value(json_str: &str) -> Result<RuntimeJotValue, JotError> {
    use serde_json::Value;

    let json: Value = serde_json::from_str(json_str).map_err(|e| JotError::Parse(e.to_string()))?;
    json_value_to_jot(&json)
}

fn json_value_to_jot(json: &serde_json::Value) -> Result<RuntimeJotValue, JotError> {
    use serde_json::Value;

    Ok(match json {
        Value::Null => RuntimeJotValue::Null,
        Value::Bool(b) => RuntimeJotValue::Boolean(*b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                RuntimeJotValue::Integer(i)
            } else if let Some(f) = n.as_f64() {
                RuntimeJotValue::Float(f)
            } else {
                return Err(JotError::Parse("Invalid number".to_string()));
            }
        }
        Value::String(s) => RuntimeJotValue::String(s.clone()),
        Value::Array(arr) => RuntimeJotValue::Array(arr.iter().map(json_value_to_jot).collect::<Result<Vec<_>, _>>()?),
        Value::Object(map) => {
            let mut obj = std::collections::HashMap::new();
            for (k, v) in map {
                obj.insert(k.clone(), json_value_to_jot(v)?);
            }
            RuntimeJotValue::Object(obj)
        }
    })
}