use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::Mutex;
use std::time::{SystemTime, UNIX_EPOCH};

use sha2::{Sha256, Digest};

pub struct CompilationCache {
    cache_dir: PathBuf,
    entries: Mutex<HashMap<String, CacheEntry>>,
    max_size_mb: u64,
}

struct CacheEntry {
    hash: String,
    output_path: PathBuf,
    created_at: u64,
    size: u64,
    artifact_type: ArtifactType,
}

enum ArtifactType {
    Object,
    Assembly,
    Binary,
}

impl CompilationCache {
    pub fn new(cache_dir: PathBuf) -> Self {
        std::fs::create_dir_all(&cache_dir).ok();
        CompilationCache {
            cache_dir,
            entries: Mutex::new(HashMap::new()),
            max_size_mb: 1024,
        }
    }

    pub fn content_hash(data: &[u8]) -> String {
        let mut hasher = Sha256::new();
        hasher.update(data);
        format!("{:x}", hasher.finalize())
    }

    pub fn file_hash(path: &Path) -> Result<String, String> {
        let data = std::fs::read(path)
            .map_err(|e| format!("Failed to read '{}': {}", path.display(), e))?;
        Ok(Self::content_hash(&data))
    }

    pub fn get_cached_output(&self, source_hash: &str, artifact_type: &str) -> Option<PathBuf> {
        let key = format!("{}_{}", source_hash, artifact_type);
        let entries = self.entries.lock().unwrap();
        entries.get(&key).map(|e| e.output_path.clone())
    }

    pub fn store(&self, source_hash: &str, data: &[u8], artifact_type: &str) -> Result<PathBuf, String> {
        let data_hash = Self::content_hash(data);
        let cache_key = format!("{}_{}", source_hash, artifact_type);
        let cached_name = format!("{}_{}", data_hash, artifact_type);
        let cached_path = self.cache_dir.join(&cached_name);

        std::fs::write(&cached_path, data)
            .map_err(|e| format!("Failed to write cache '{}': {}", cached_path.display(), e))?;

        let metadata = std::fs::metadata(&cached_path)
            .map_err(|e| format!("Failed to read metadata: {}", e))?;

        let entry = CacheEntry {
            hash: data_hash,
            output_path: cached_path.clone(),
            created_at: SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_secs(),
            size: metadata.len(),
            artifact_type: match artifact_type {
                "o" => ArtifactType::Object,
                "s" => ArtifactType::Assembly,
                "bin" => ArtifactType::Binary,
                _ => ArtifactType::Object,
            },
        };

        let mut entries = self.entries.lock().unwrap();
        entries.insert(cache_key, entry);

        Ok(cached_path)
    }

    pub fn store_file(&self, source_hash: &str, file_path: &Path, artifact_type: &str) -> Result<PathBuf, String> {
        let data = std::fs::read(file_path)
            .map_err(|e| format!("Failed to read '{}': {}", file_path.display(), e))?;
        self.store(source_hash, &data, artifact_type)
    }

    pub fn retrieve(&self, source_hash: &str, artifact_type: &str, output_path: &Path) -> bool {
        if let Some(cached) = self.get_cached_output(source_hash, artifact_type) {
            if cached.exists() {
                match std::fs::copy(&cached, output_path) {
                    Ok(_) => return true,
                    Err(_) => {}
                }
            }
        }
        false
    }

    pub fn invalidate(&self, source_hash: &str) {
        let mut entries = self.entries.lock().unwrap();
        entries.retain(|k, _| !k.starts_with(source_hash));
    }

    pub fn clear_all(&self) {
        let mut entries = self.entries.lock().unwrap();
        for entry in entries.values() {
            std::fs::remove_file(&entry.output_path).ok();
        }
        entries.clear();
    }

    pub fn get_cache_size(&self) -> Result<u64, String> {
        let entries = self.entries.lock().unwrap();
        Ok(entries.values().map(|e| e.size).sum())
    }

    pub fn prune(&self) -> Result<u64, String> {
        let mut entries = self.entries.lock().unwrap();
        let mut total_size: u64 = entries.values().map(|e| e.size).sum();
        let max_bytes = self.max_size_mb * 1024 * 1024;
        if total_size <= max_bytes {
            return Ok(0);
        }
        let mut sorted: Vec<String> = {
            let mut v: Vec<&CacheEntry> = entries.values().collect();
            v.sort_by_key(|e| e.created_at);
            v.iter().map(|e| e.hash.clone()).collect()
        };
        let mut removed = 0u64;
        while total_size > max_bytes && !sorted.is_empty() {
            let hash = sorted.remove(0);
            if let Some(entry) = entries.values().find(|e| e.hash == hash) {
                total_size -= entry.size;
                removed += entry.size;
                std::fs::remove_file(&entry.output_path).ok();
            }
            entries.retain(|_, e| e.hash != hash);
        }
        Ok(removed)
    }

    pub fn warm_up(&self, source: &Path) -> Result<String, String> {
        let hash = Self::file_hash(source)?;
        Ok(hash)
    }

    pub fn save_index(&self) -> Result<(), String> {
        let index_path = self.cache_dir.join("cache_index.json");
        let entries = self.entries.lock().unwrap();
        let index: HashMap<String, serde_json::Value> = entries.iter().map(|(k, v)| {
            (k.clone(), serde_json::json!({
                "hash": v.hash,
                "output": v.output_path.to_string_lossy(),
                "created": v.created_at,
                "size": v.size,
            }))
        }).collect();
        let json = serde_json::to_string_pretty(&index)
            .map_err(|e| format!("Failed to serialize cache index: {}", e))?;
        std::fs::write(&index_path, &json)
            .map_err(|e| format!("Failed to write cache index: {}", e))?;
        Ok(())
    }

    pub fn load_index(&self) -> Result<(), String> {
        let index_path = self.cache_dir.join("cache_index.json");
        if !index_path.exists() {
            return Ok(());
        }
        let json = std::fs::read_to_string(&index_path)
            .map_err(|e| format!("Failed to read cache index: {}", e))?;
        let index: HashMap<String, serde_json::Value> = serde_json::from_str(&json)
            .map_err(|e| format!("Failed to parse cache index: {}", e))?;
        let mut entries = self.entries.lock().unwrap();
        for (key, value) in &index {
            if let (Some(hash), Some(output), Some(created), Some(size)) = (
                value.get("hash").and_then(|v| v.as_str()),
                value.get("output").and_then(|v| v.as_str()),
                value.get("created").and_then(|v| v.as_u64()),
                value.get("size").and_then(|v| v.as_u64()),
            ) {
                entries.insert(key.clone(), CacheEntry {
                    hash: hash.to_string(),
                    output_path: PathBuf::from(output),
                    created_at: created,
                    size,
                    artifact_type: ArtifactType::Object,
                });
            }
        }
        Ok(())
    }
}
