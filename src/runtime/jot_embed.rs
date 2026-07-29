#[cfg(feature = "jot-embed")]
use jot_parser::{parse_content, parse_file, build_tree};
#[cfg(feature = "jot-embed")]
use jot_serializer::convert;
#[cfg(feature = "jot-embed")]
use jot_core::JotValue as CoreJotValue;

use crate::runtime::jot::{JotBackend, JotValue as RuntimeJotValue, JotError};

#[cfg(feature = "jot-embed")]
pub struct EmbedBackend;

#[cfg(feature = "jot-embed")]
impl EmbedBackend {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(feature = "jot-embed")]
impl JotBackend for EmbedBackend {
    fn parse_string(&self, source: &str) -> Result<RuntimeJotValue, JotError> {
        let tokens = parse_content(source);
        let core_val = build_tree(&tokens);
        core_to_runtime(core_val)
    }

    fn parse_file(&self, path: &str) -> Result<RuntimeJotValue, JotError> {
        let tokens = parse_file(path).map_err(|e| JotError::Parse(e.to_string()))?;
        let core_val = build_tree(&tokens);
        core_to_runtime(core_val)
    }

    fn to_format(&self, value: &RuntimeJotValue, format: &str) -> Result<String, JotError> {
        let core_val = runtime_to_core(value)?;
        convert(&core_val, format).map_err(|e| JotError::Convert(e))
    }

    fn pick(&self, value: &RuntimeJotValue, path: &str) -> Result<String, JotError> {
        let keys: Vec<String> = path.split('.').map(|s| s.to_string()).collect();
        value.pick(&keys)
            .map(|v| v.to_string())
            .ok_or_else(|| JotError::Pick(format!("key '{}' not found", path)))
    }

    fn format_jot(&self, value: &RuntimeJotValue) -> String {
        value.to_string()
    }
}

#[cfg(feature = "jot-embed")]
fn core_to_runtime(core: CoreJotValue) -> Result<RuntimeJotValue, JotError> {
    use jot_core::JotValue as Core;
    Ok(match core {
        Core::Null => RuntimeJotValue::Null,
        Core::Boolean(b) => RuntimeJotValue::Boolean(b),
        Core::Integer(i) => RuntimeJotValue::Integer(i),
        Core::Float(f) => RuntimeJotValue::Float(f),
        Core::String(s) => RuntimeJotValue::String(s),
        Core::Array(arr) => RuntimeJotValue::Array(arr.into_iter().map(core_to_runtime).collect::<Result<_, _>>()?),
        Core::Object(map) => RuntimeJotValue::Object(map.into_iter().map(|(k, v)| Ok((k, core_to_runtime(v)?))).collect::<Result<_, _>>()?),
    })
}

#[cfg(feature = "jot-embed")]
fn runtime_to_core(val: &RuntimeJotValue) -> Result<CoreJotValue, JotError> {
    use jot_core::JotValue as Core;
    Ok(match val {
        RuntimeJotValue::Null => Core::Null,
        RuntimeJotValue::Boolean(b) => Core::Boolean(*b),
        RuntimeJotValue::Integer(i) => Core::Integer(*i),
        RuntimeJotValue::Float(f) => Core::Float(*f),
        RuntimeJotValue::String(s) => Core::String(s.clone()),
        RuntimeJotValue::Array(arr) => Core::Array(arr.iter().map(runtime_to_core).collect::<Result<_, _>>()?),
        RuntimeJotValue::Object(map) => Core::Object(map.iter().map(|(k, v)| Ok((k.clone(), runtime_to_core(v)?))).collect::<Result<_, _>>()?),
    })
}

#[cfg(not(feature = "jot-embed"))]
pub struct EmbedBackend;

#[cfg(not(feature = "jot-embed"))]
impl EmbedBackend {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(not(feature = "jot-embed"))]
impl JotBackend for EmbedBackend {
    fn parse_string(&self, _source: &str) -> Result<RuntimeJotValue, JotError> {
        Err(JotError::Parse("JOT embed feature not enabled. Compile with --features jot-embed".to_string()))
    }

    fn parse_file(&self, _path: &str) -> Result<RuntimeJotValue, JotError> {
        Err(JotError::Parse("JOT embed feature not enabled. Compile with --features jot-embed".to_string()))
    }

    fn to_format(&self, _value: &RuntimeJotValue, _format: &str) -> Result<String, JotError> {
        Err(JotError::Convert("JOT embed feature not enabled".to_string()))
    }

    fn pick(&self, _value: &RuntimeJotValue, _path: &str) -> Result<String, JotError> {
        Err(JotError::Pick("JOT embed feature not enabled".to_string()))
    }

    fn format_jot(&self, _value: &RuntimeJotValue) -> String {
        "JOT embed feature not enabled".to_string()
    }
}