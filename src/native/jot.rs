use crate::native::types::ValueData;
use crate::error::AlyError;
use crate::runtime::jot::{JotRuntime, JotInstanceValidator};

pub fn jot_runtime_get() -> Result<ValueData, AlyError> {
    let rt = JotRuntime::current();
    Ok(ValueData::String(rt.name().to_string()))
}

pub fn jot_runtime_set(args: Vec<ValueData>) -> Result<ValueData, AlyError> {
    let arg = args.first().ok_or_else(|| AlyError::runtime("JOT.setRuntime: expected argument"))?;
    let path = arg.as_string().ok_or_else(|| AlyError::runtime("JOT.setRuntime: expected string"))?;
    
    if path == "embed" {
        #[cfg(feature = "jot-embed")]
        {
            JotRuntime::set(crate::runtime::jot::JotRuntime::Embed(crate::runtime::jot_embed::EmbedBackend::new()));
            Ok(ValueData::String("None".to_string()))
        }
        #[cfg(not(feature = "jot-embed"))]
        {
            Err(AlyError::runtime("JOT embed feature not compiled. Recompile with --features jot-embed"))
        }
    } else {
        let backend = crate::runtime::jot_external::ExternalBackend::new(path)
            .map_err(|e| AlyError::runtime(format!("Invalid JOT binary: {}", e)))?;
        JotRuntime::set(crate::runtime::jot::JotRuntime::External(backend));
        Ok(ValueData::String("None".to_string()))
    }
}

pub fn jot_constructor(args: Vec<ValueData>) -> Result<ValueData, AlyError> {
    if args.is_empty() {
        return Err(AlyError::runtime("JOT: expected source string or filename"));
    }
    
    let source = args[0].as_string().ok_or_else(|| AlyError::runtime("JOT: source must be string"))?;
    let is_file = args.get(1).and_then(|v| v.as_bool()).unwrap_or(false);
    
    let rt = JotRuntime::current();
    let value = if is_file {
        rt.parse_file(&source).map_err(|e| AlyError::runtime(e.to_string()))?
    } else {
        rt.parse_string(&source).map_err(|e| AlyError::runtime(e.to_string()))?
    };
    
    let instance = crate::runtime::jot::JotInstance::new(value, source, is_file);
    Ok(ValueData::JotInstance(Box::new(instance)))
}

pub fn jot_parser(args: Vec<ValueData>) -> Result<ValueData, AlyError> {
    if args.is_empty() {
        return Err(AlyError::runtime("JOT.parser: expected JOT instance"));
    }
    
    let instance = args[0].as_jot_instance()
        .ok_or_else(|| AlyError::runtime("JOT.parser: first argument must be JOT instance"))?;
    
    let format = args.get(1).and_then(|v| v.as_string()).unwrap_or("native".to_string());
    
    match format.as_str() {
        "native" => Ok(instance.jot_to_object()),
        _ => Ok(ValueData::String(instance.jot_to_format(&format)?))
    }
}

pub fn jot_stringify(args: Vec<ValueData>) -> Result<ValueData, AlyError> {
    if args.is_empty() {
        return Err(AlyError::runtime("JOT.stringify: expected JOT instance"));
    }
    
    let instance = args[0].as_jot_instance()
        .ok_or_else(|| AlyError::runtime("JOT.stringify: first argument must be JOT instance"))?;
    
    let format = args.get(1).and_then(|v| v.as_string()).unwrap_or("jot".to_string());
    Ok(ValueData::String(instance.jot_to_string(Some(&format))))
}

pub fn jot_pick(args: Vec<ValueData>) -> Result<ValueData, AlyError> {
    if args.len() < 2 {
        return Err(AlyError::runtime("JOT.pick: expected instance and path"));
    }
    
    let instance = args[0].as_jot_instance()
        .ok_or_else(|| AlyError::runtime("JOT.pick: first argument must be JOT instance"))?;
    let path = args[1].as_string().ok_or_else(|| AlyError::runtime("JOT.pick: path must be string"))?;
    
    Ok(ValueData::String(instance.jot_pick(&path)?))
}

pub fn jot_get(args: Vec<ValueData>) -> Result<ValueData, AlyError> {
    if args.len() < 2 {
        return Err(AlyError::runtime("JOT.get: expected instance and path"));
    }
    
    let instance = args[0].as_jot_instance()
        .ok_or_else(|| AlyError::runtime("JOT.get: first argument must be JOT instance"))?;
    let path = args[1].as_string().ok_or_else(|| AlyError::runtime("JOT.get: path must be string"))?;
    
    Ok(instance.jot_get(&path).unwrap_or(ValueData::String("None".to_string())))
}

pub fn jot_has(args: Vec<ValueData>) -> Result<ValueData, AlyError> {
    if args.len() < 2 {
        return Err(AlyError::runtime("JOT.has: expected instance and path"));
    }
    
    let instance = args[0].as_jot_instance()
        .ok_or_else(|| AlyError::runtime("JOT.has: first argument must be JOT instance"))?;
    let path = args[1].as_string().ok_or_else(|| AlyError::runtime("JOT.has: path must be string"))?;
    
    Ok(ValueData::Bool(instance.jot_has(&path)))
}

pub fn jot_keys(args: Vec<ValueData>) -> Result<ValueData, AlyError> {
    if args.is_empty() {
        return Err(AlyError::runtime("JOT.keys: expected JOT instance"));
    }
    
    let instance = args[0].as_jot_instance()
        .ok_or_else(|| AlyError::runtime("JOT.keys: first argument must be JOT instance"))?;
    
    let keys = instance.jot_keys();
    let vec = crate::native::vector::Vector::new(keys.into_iter().map(ValueData::String).collect());
    Ok(ValueData::Vec(vec))
}

pub fn jot_to_object(args: Vec<ValueData>) -> Result<ValueData, AlyError> {
    if args.is_empty() {
        return Err(AlyError::runtime("JOT.toObject: expected JOT instance"));
    }
    
    let instance = args[0].as_jot_instance()
        .ok_or_else(|| AlyError::runtime("JOT.toObject: first argument must be JOT instance"))?;
    
    Ok(instance.jot_to_object())
}

trait ValueDataExt {
    fn as_jot_instance(&self) -> Option<&dyn JotInstanceValidator>;
    fn as_string(&self) -> Option<&str>;
    fn as_bool(&self) -> Option<bool>;
}

impl ValueDataExt for ValueData {
    fn as_jot_instance(&self) -> Option<&dyn JotInstanceValidator> {
        match self {
            ValueData::JotInstance(boxed) => Some(boxed.as_ref()),
            _ => None,
        }
    }
    
    fn as_string(&self) -> Option<&str> {
        match self {
            ValueData::String(s) => Some(s),
            _ => None,
        }
    }
    
    fn as_bool(&self) -> Option<bool> {
        match self {
            ValueData::Bool(b) => Some(*b),
            _ => None,
        }
    }
}