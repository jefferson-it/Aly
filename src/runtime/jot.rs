use std::collections::HashMap;
use std::sync::OnceLock;
use crate::native::types::Validator;
use crate::error::AlyError;

impl From<JotError> for AlyError {
    fn from(err: JotError) -> Self {
        AlyError::runtime(err.to_string())
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum JotValue {
    Null,
    Boolean(bool),
    Integer(i64),
    Float(f64),
    String(String),
    Array(Vec<JotValue>),
    Object(HashMap<String, JotValue>),
}

#[derive(Debug, thiserror::Error)]
pub enum JotError {
    #[error("Parse error: {0}")]
    Parse(String),
    #[error("IO error: {0}")]
    Io(String),
    #[error("Pick error: {0}")]
    Pick(String),
    #[error("Convert error: {0}")]
    Convert(String),
    #[error("Runtime not available: {0}")]
    RuntimeUnavailable(String),
}

impl JotValue {
    pub fn pick(&self, keys: &[String]) -> Option<&JotValue> {
        let mut current = self;
        for key in keys {
            match current {
                JotValue::Object(map) => {
                    current = map.get(key)?;
                }
                _ => return None,
            }
        }
        Some(current)
    }

    pub fn to_string(&self) -> String {
        format_jot(self, 0)
    }
}

fn format_jot(val: &JotValue, indent: usize) -> String {
    let pad = "    ".repeat(indent);
    match val {
        JotValue::Null => format!("{}null", pad),
        JotValue::Boolean(b) => format!("{}{}", pad, b),
        JotValue::Integer(i) => format!("{}{}", pad, i),
        JotValue::Float(f) => {
            let s = f.to_string();
            if s.contains('.') || s.contains('e') { format!("{}{}", pad, s) } else { format!("{}{}.0", pad, f) }
        }
        JotValue::String(s) => format!("{}{}", pad, s),
        JotValue::Array(arr) => {
            if arr.is_empty() { return format!("{}[]", pad); }
            let mut out = format!("{}[\n", pad);
            let item_pad = "    ".repeat(indent + 1);
            for v in arr {
                out.push_str(&format!("{}{}\n", item_pad, format_jot(v, 0).trim_start()));
            }
            out.push_str(&format!("{}]", pad));
            out
        }
        JotValue::Object(map) => {
            if map.is_empty() { return format!("{} {{}}", pad); }
            let mut out = String::new();
            for (k, v) in map {
                match v {
                    JotValue::Object(_) | JotValue::Array(_) => {
                        out.push_str(&format!("{}:\n{}", k, format_jot(v, indent + 1)));
                    }
                    _ => {
                        out.push_str(&format!("{} {} {}\n", pad, k, format_jot(v, 0).trim()));
                    }
                }
            }
            out.trim_end().to_string()
        }
    }
}

pub trait JotInstanceValidator: Validator + Send + Sync {
    fn jot_to_string(&self, format: Option<&str>) -> String;
    fn jot_get(&self, path: &str) -> Option<crate::native::types::ValueData>;
    fn jot_has(&self, path: &str) -> bool;
    fn jot_pick(&self, path: &str) -> Result<String, JotError>;
    fn jot_keys(&self) -> Vec<String>;
    fn jot_to_object(&self) -> crate::native::types::ValueData;
    fn jot_to_format(&self, format: &str) -> Result<String, JotError>;
}

pub trait JotBackend: Send + Sync {
    fn parse_string(&self, source: &str) -> Result<JotValue, JotError>;
    fn parse_file(&self, path: &str) -> Result<JotValue, JotError>;
    fn to_format(&self, value: &JotValue, format: &str) -> Result<String, JotError>;
    fn pick(&self, value: &JotValue, path: &str) -> Result<String, JotError>;
    fn format_jot(&self, value: &JotValue) -> String;
}

pub struct JotInstance {
    value: JotValue,
    source_info: SourceInfo,
}

#[derive(Debug, Clone)]
pub struct SourceInfo {
    pub source: String,
    pub is_file: bool,
}

static RUNTIME: OnceLock<JotRuntime> = OnceLock::new();

// Re-export backend types from submodules
#[cfg(feature = "jot-embed")]
pub use crate::runtime::jot_embed::EmbedBackend;
#[cfg(not(feature = "jot-embed"))]
pub use crate::runtime::jot_embed::EmbedBackend;

pub use crate::runtime::jot_external::ExternalBackend;

pub enum JotRuntime {
    Embed(EmbedBackend),
    External(ExternalBackend),
}

impl JotRuntime {
    pub fn current() -> &'static JotRuntime {
        RUNTIME.get_or_init(|| {
            #[cfg(feature = "jot-embed")]
            {
                JotRuntime::Embed(EmbedBackend::new())
            }
            #[cfg(not(feature = "jot-embed"))]
            {
                JotRuntime::External(ExternalBackend::new("jot").unwrap_or_else(|_| ExternalBackend { binary_path: "jot".to_string() }))
            }
        })
    }

    pub fn set(runtime: JotRuntime) {
        let _ = RUNTIME.set(runtime);
    }

    pub fn name(&self) -> &str {
        match self {
            JotRuntime::Embed(_) => "embed",
            JotRuntime::External(b) => b.binary_path.as_str(),
        }
    }

    pub fn parse_string(&self, source: &str) -> Result<JotValue, JotError> {
        match self {
            JotRuntime::Embed(b) => b.parse_string(source),
            JotRuntime::External(b) => b.parse_string(source),
        }
    }

    pub fn parse_file(&self, path: &str) -> Result<JotValue, JotError> {
        match self {
            JotRuntime::Embed(b) => b.parse_file(path),
            JotRuntime::External(b) => b.parse_file(path),
        }
    }

    pub fn to_format(&self, value: &JotValue, format: &str) -> Result<String, JotError> {
        match self {
            JotRuntime::Embed(b) => b.to_format(value, format),
            JotRuntime::External(b) => b.to_format(value, format),
        }
    }

    pub fn pick(&self, value: &JotValue, path: &str) -> Result<String, JotError> {
        match self {
            JotRuntime::Embed(b) => b.pick(value, path),
            JotRuntime::External(b) => b.pick(value, path),
        }
    }
}

#[cfg(feature = "jot-embed")]
fn jot_value_from_core(core: jot_core::JotValue) -> Result<JotValue, JotError> {
    use jot_core::JotValue as Core;
    Ok(match core {
        Core::Null => JotValue::Null,
        Core::Boolean(b) => JotValue::Boolean(b),
        Core::Integer(i) => JotValue::Integer(i),
        Core::Float(f) => JotValue::Float(f),
        Core::String(s) => JotValue::String(s),
        Core::Array(arr) => JotValue::Array(arr.into_iter().map(jot_value_from_core).collect::<Result<_, _>>()?),
        Core::Object(map) => JotValue::Object(map.into_iter().map(|(k, v)| Ok((k, jot_value_from_core(v)?))).collect::<Result<_, _>>()?),
    })
}

#[cfg(feature = "jot-embed")]
fn jot_value_to_core(val: &JotValue) -> Result<jot_core::JotValue, JotError> {
    use jot_core::JotValue as Core;
    Ok(match val {
        JotValue::Null => Core::Null,
        JotValue::Boolean(b) => Core::Boolean(*b),
        JotValue::Integer(i) => Core::Integer(*i),
        JotValue::Float(f) => Core::Float(*f),
        JotValue::String(s) => Core::String(s.clone()),
        JotValue::Array(arr) => Core::Array(arr.iter().map(jot_value_to_core).collect::<Result<_, _>>()?),
        JotValue::Object(map) => Core::Object(map.iter().map(|(k, v)| Ok((k.clone(), jot_value_to_core(v)?))).collect::<Result<_, _>>()?),
    })
}

fn json_to_jot_value(json_str: &str) -> Result<JotValue, JotError> {
    use serde_json::Value;
    let json: Value = serde_json::from_str(json_str).map_err(|e| JotError::Parse(e.to_string()))?;
    json_value_to_jot(&json)
}

fn json_value_to_jot(json: &serde_json::Value) -> Result<JotValue, JotError> {
    use serde_json::Value;
    Ok(match json {
        Value::Null => JotValue::Null,
        Value::Bool(b) => JotValue::Boolean(*b),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                JotValue::Integer(i)
            } else if let Some(f) = n.as_f64() {
                JotValue::Float(f)
            } else {
                return Err(JotError::Parse("Invalid number".into()));
            }
        }
        Value::String(s) => JotValue::String(s.clone()),
        Value::Array(arr) => JotValue::Array(arr.iter().map(json_value_to_jot).collect::<Result<_, _>>()?),
        Value::Object(map) => {
            let mut obj = HashMap::new();
            for (k, v) in map {
                obj.insert(k.clone(), json_value_to_jot(v)?);
            }
            JotValue::Object(obj)
        }
    })
}

impl JotInstance {
    pub fn new(value: JotValue, source: String, is_file: bool) -> Self {
        Self {
            value,
            source_info: SourceInfo { source, is_file },
        }
    }

    pub fn get(&self, path: &str) -> Option<crate::native::types::ValueData> {
        let keys: Vec<String> = path.split('.').map(|s| s.to_string()).collect();
        self.value.pick(&keys).map(jot_value_to_aly)
    }

    pub fn has(&self, path: &str) -> bool {
        let keys: Vec<String> = path.split('.').map(|s| s.to_string()).collect();
        self.value.pick(&keys).is_some()
    }

    pub fn pick(&self, path: &str) -> Result<String, JotError> {
        let rt = JotRuntime::current();
        rt.pick(&self.value, path)
    }

    pub fn keys(&self) -> Vec<String> {
        match &self.value {
            JotValue::Object(map) => map.keys().cloned().collect(),
            _ => vec![],
        }
    }

    pub fn to_object(&self) -> crate::native::types::ValueData {
        jot_value_to_aly(&self.value)
    }

    pub fn to_format(&self, format: &str) -> Result<String, JotError> {
        let rt = JotRuntime::current();
        rt.to_format(&self.value, format)
    }

    pub fn to_string(&self, format: Option<&str>) -> String {
        let rt = JotRuntime::current();
        match format {
            Some("json") => rt.to_format(&self.value, "json").unwrap_or_default(),
            Some("yaml") => rt.to_format(&self.value, "yaml").unwrap_or_default(),
            Some("toml") => rt.to_format(&self.value, "toml").unwrap_or_default(),
            Some("py-dict") | Some("pydict") => rt.to_format(&self.value, "py-dict").unwrap_or_default(),
            Some("js-object") | Some("jsobject") => rt.to_format(&self.value, "js-object").unwrap_or_default(),
            Some("jot") => self.value.to_string(),
            _ => self.value.to_string(),
        }
    }
}

impl crate::runtime::jot::JotInstanceValidator for JotInstance {
    fn jot_to_string(&self, format: Option<&str>) -> String {
        self.to_string(format)
    }
    
    fn jot_get(&self, path: &str) -> Option<crate::native::types::ValueData> {
        self.get(path)
    }
    
    fn jot_has(&self, path: &str) -> bool {
        self.has(path)
    }
    
    fn jot_pick(&self, path: &str) -> Result<String, JotError> {
        self.pick(path)
    }
    
    fn jot_keys(&self) -> Vec<String> {
        self.keys()
    }

    fn jot_to_object(&self) -> crate::native::types::ValueData {
        self.to_object()
    }

    fn jot_to_format(&self, format: &str) -> Result<String, JotError> {
        self.to_format(format)
    }
}

impl crate::native::types::Validator for JotInstance {
    fn valid(&self) -> (crate::native::types::Type, crate::native::types::ValueData) {
        (crate::native::types::Type::Obj, crate::native::types::ValueData::String("JotInstance".to_string()))
    }
}

fn jot_value_to_aly(val: &JotValue) -> crate::native::types::ValueData {
    use crate::native::types::ValueData;
    match val {
        JotValue::Null => ValueData::String("None".to_string()),
        JotValue::Boolean(b) => ValueData::Bool(*b),
        JotValue::Integer(i) => ValueData::Int(*i as i32),
        JotValue::Float(f) => ValueData::Float(*f as f32),
        JotValue::String(s) => ValueData::String(s.clone()),
        JotValue::Array(arr) => {
            let values: Vec<ValueData> = arr.iter().map(jot_value_to_aly).collect();
            let vec = crate::native::vector::Vector::new(values);
            ValueData::Vec(vec)
        }
        JotValue::Object(map) => {
            let mut draft = HashMap::new();
            for (k, v) in map {
                draft.insert(k.clone(), jot_value_to_aly(v));
            }
            let obj = crate::native::create_object::Object::new(vec![], draft);
            ValueData::Object(obj)
        }
    }
}