/// Thread‑local storage implementation for global variables and state.
///
/// This module provides a thread‑safe global registry that can be accessed
/// quickly from any part of the VM without requiring global mutex locks.
/// Used for caching frequently accessed globals and reducing contention.
///
/// ## Design
/// - `ThreadLocalStore` maintains per‑thread hash maps
/// - Global look‑ups delegate to the current thread’s store
/// - Supports fast get/set with zero‑allocation path for common cases
pub struct ThreadLocalStore {
    /// Per‑thread storage (one map per OS thread)
    thread_maps: std::collections::ThreadLocal<std::collections::HashMap<String, Value>>,
}

impl ThreadLocalStore {
    /// Create a new empty thread‑local store
    pub fn new() -> Self {
        Self {
            thread_maps: std::collections::ThreadLocal::new(),
        }
    }
    
    /// Get a mutable reference to the current thread’s map
    fn current_map() -> std::cell::RefMut<'static, std::collections::HashMap<String, Value>> {
        self.thread_maps.with(|map| map.get_or_default().clone())
    }
    
    /// Fast get operation for existing globals
    pub fn get(&self, key: &str) -> Option<Value> {
        let map = Self::current_map();
        map.get(key).cloned()
    }
    
    /// Fast set operation for new globals
    pub fn set(&self, key: String, value: Value) {
        let map = Self::current_map();
        map.insert(key, value);
    }
    
    /// Remove a global value
    pub fn remove(&self, key: &str) -> Option<Value> {
        let map = Self::current_map();
        map.remove(key)
    }
    
    /// Iterate over all globals in current thread
    pub fn iter(&self) -> std::collections::hash_map::Iter<'static, String, Value> {
        let map = Self::current_map();
        map.iter()
    }
    
    /// Clear all globals in current thread
    pub fn clear(&self) {
        let map = Self::current_map();
        map.clear();
    }
}