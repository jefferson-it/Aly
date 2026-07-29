use super::value::Value;

/// An object pool for recycling heap-allocated containers (Strings and Vectors)
/// to reduce allocator churn during execution.
pub struct ValuePool {
    string_pool: Vec<String>,
    vec_pool: Vec<Vec<Value>>,
}

impl ValuePool {
    pub fn new() -> Self {
        ValuePool {
            string_pool: Vec::with_capacity(128),
            vec_pool: Vec::with_capacity(128),
        }
    }

    /// Retrieve an empty String from the pool, or allocate a new one if empty.
    pub fn get_string(&mut self) -> String {
        self.string_pool.pop().unwrap_or_else(String::new)
    }

    /// Return a String to the pool for future reuse.
    pub fn recycle_string(&mut self, mut s: String) {
        if self.string_pool.len() < 128 {
            s.clear();
            self.string_pool.push(s);
        }
    }

    /// Retrieve an empty Vec from the pool, or allocate a new one if empty.
    pub fn get_vec(&mut self) -> Vec<Value> {
        self.vec_pool.pop().unwrap_or_else(Vec::new)
    }

    /// Return a Vec to the pool for future reuse.
    pub fn recycle_vec(&mut self, mut v: Vec<Value>) {
        if self.vec_pool.len() < 128 {
            v.clear();
            self.vec_pool.push(v);
        }
    }
}

impl Default for ValuePool {
    fn default() -> Self {
        Self::new()
    }
}