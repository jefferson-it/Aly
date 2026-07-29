use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::sync::Mutex;

use sha2::{Sha256, Digest};

pub struct IncrementalCompiler {
    cache_dir: PathBuf,
    state: Mutex<IncrementalState>,
}

struct IncrementalState {
    file_hashes: HashMap<PathBuf, String>,
    dependencies: HashMap<PathBuf, Vec<PathBuf>>,
    reverse_deps: HashMap<PathBuf, Vec<PathBuf>>,
    output_timestamps: HashMap<PathBuf, u64>,
}

impl IncrementalCompiler {
    pub fn new(cache_dir: PathBuf) -> Self {
        let state = IncrementalState {
            file_hashes: HashMap::new(),
            dependencies: HashMap::new(),
            reverse_deps: HashMap::new(),
            output_timestamps: HashMap::new(),
        };
        IncrementalCompiler { cache_dir, state: Mutex::new(state) }
    }

    pub fn hash_file(path: &Path) -> Result<String, String> {
        let contents = std::fs::read(path)
            .map_err(|e| format!("Failed to read '{}': {}", path.display(), e))?;
        let mut hasher = Sha256::new();
        hasher.update(&contents);
        Ok(format!("{:x}", hasher.finalize()))
    }

    pub fn is_modified(&self, path: &Path) -> Result<bool, String> {
        let state = self.state.lock().unwrap();
        let current_hash = Self::hash_file(path)?;
        match state.file_hashes.get(path) {
            Some(old_hash) => Ok(old_hash != &current_hash),
            None => Ok(true),
        }
    }

    pub fn record_build(&self, path: &Path) -> Result<(), String> {
        let hash = Self::hash_file(path)?;
        let mut state = self.state.lock().unwrap();
        state.file_hashes.insert(path.to_path_buf(), hash);
        Ok(())
    }

    pub fn set_dependencies(&self, source: &Path, deps: &[PathBuf]) {
        let mut state = self.state.lock().unwrap();
        state.dependencies.insert(source.to_path_buf(), deps.to_vec());
        for dep in deps {
            state.reverse_deps.entry(dep.clone())
                .or_default()
                .push(source.to_path_buf());
        }
    }

    pub fn get_affected(&self, changed: &[PathBuf]) -> HashSet<PathBuf> {
        let state = self.state.lock().unwrap();
        let mut affected = HashSet::new();
        let mut queue: Vec<PathBuf> = changed.to_vec();
        while let Some(path) = queue.pop() {
            if affected.insert(path.clone()) {
                if let Some(rdeps) = state.reverse_deps.get(&path) {
                    queue.extend(rdeps.iter().cloned());
                }
            }
        }
        affected
    }

    pub fn needs_rebuild(&self, source: &Path) -> Result<bool, String> {
        if !source.exists() {
            return Err(format!("Source file '{}' does not exist", source.display()));
        }
        if self.is_modified(source)? {
            return Ok(true);
        }
        let state = self.state.lock().unwrap();
        if let Some(deps) = state.dependencies.get(source) {
            for dep in deps {
                if !dep.exists() {
                    return Ok(true);
                }
                let dep_hash = Self::hash_file(dep)?;
                if state.file_hashes.get(dep) != Some(&dep_hash) {
                    return Ok(true);
                }
            }
        }
        Ok(false)
    }

    pub fn get_output_stale(&self, source: &Path, output: &Path) -> Result<bool, String> {
        let src_modified = std::fs::metadata(source)
            .and_then(|m| m.modified())
            .map_err(|e| format!("Failed to get metadata: {}", e))?;
        let out_modified = std::fs::metadata(output)
            .and_then(|m| m.modified())
            .map_err(|e| format!("Failed to get metadata: {}", e))?;
        Ok(src_modified > out_modified)
    }

    pub fn save_state(&self) -> Result<(), String> {
        let state = self.state.lock().unwrap();
        let state_path = self.cache_dir.join("incremental_state.json");
        let serializable: HashMap<String, serde_json::Value> = HashMap::new();
        let json = serde_json::to_string_pretty(&serializable)
            .map_err(|e| format!("Failed to serialize state: {}", e))?;
        std::fs::write(&state_path, &json)
            .map_err(|e| format!("Failed to write state: {}", e))?;
        Ok(())
    }

    pub fn load_state(&self) -> Result<(), String> {
        let state_path = self.cache_dir.join("incremental_state.json");
        if !state_path.exists() {
            return Ok(());
        }
        let json = std::fs::read_to_string(&state_path)
            .map_err(|e| format!("Failed to read state: {}", e))?;
        Ok(())
    }

    pub fn clear(&self) {
        let mut state = self.state.lock().unwrap();
        state.file_hashes.clear();
        state.dependencies.clear();
        state.reverse_deps.clear();
        state.output_timestamps.clear();
    }

    pub fn compile_changed<F>(&self, source: &Path, compiler: F) -> Result<bool, String>
    where F: Fn(&Path) -> Result<(), String>
    {
        if !self.needs_rebuild(source)? {
            return Ok(false);
        }
        compiler(source)?;
        self.record_build(source)?;
        Ok(true)
    }
}
