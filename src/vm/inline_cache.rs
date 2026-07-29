use std::collections::HashMap;
use super::value::Value;

/// InlineCache structures for property access optimization.
///
/// This module implements caching strategies to eliminate
/// repeated property lookups in interpreter code paths.
pub struct InlineCache {
    /// Property access cache (object hash → cached property values)
    property_cache: HashMap<u64, Vec<Value>>,
    /// Global variable cache (global_name → cached Value)
    global_cache: HashMap<String, Value>,
    /// Upvalue cache (closure_idx → cached upvalue references)
    upvalue_cache: HashMap<usize, Value>,
}

impl InlineCache {
    /// Create a new empty cache
    pub fn new() -> Self {
        Self {
            property_cache: HashMap::new(),
            global_cache: HashMap::new(),
            upvalue_cache: HashMap::new(),
        }
    }
    
    /// Cache a property access result for faster future lookups
    pub fn cache_property_access(&mut self, obj_hash: u64, prop_name: &str, _value: Value) {
        self.property_cache.entry(obj_hash)
            .or_insert_with(Vec::new)
            .push(Value::Str(prop_name.to_string()));
    }
    
    /// Retrieve a cached property value
    pub fn get_cached_property(&self, obj_hash: u64, prop_name: &str) -> Option<Value> {
        self.property_cache.get(&obj_hash)
            .and_then(|cache| {
                cache.iter()
                    .find(|v| match v {
                        Value::Str(s) => s == prop_name,
                        _ => false,
                    })
            }).cloned()
    }
    
    /// Cache a global variable lookup for faster future access
    pub fn cache_global_lookup(&mut self, global_name: &str, value: Value) {
        self.global_cache.insert(global_name.to_string(), value);
    }
    
    /// Get a previously cached global value
    pub fn get_cached_global(&self, global_name: &str) -> Option<Value> {
        self.global_cache.get(global_name).cloned()
    }
    
    /// Cache an upvalue access for closure optimization
    pub fn cache_upvalue(&mut self, closure_idx: usize, upvalue: Value) {
        self.upvalue_cache.insert(closure_idx, upvalue);
    }
    
    /// Retrieve a cached upvalue
    pub fn get_cached_upvalue(&self, closure_idx: usize) -> Option<Value> {
        self.upvalue_cache.get(&closure_idx).cloned()
    }
}

impl Default for InlineCache {
    fn default() -> Self {
        Self::new()
    }
}