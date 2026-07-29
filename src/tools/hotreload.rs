use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::Duration;

/// Hot reload watcher: monitors file changes and triggers reloads.
pub struct HotReloader {
    watched_files: HashMap<PathBuf, std::time::SystemTime>,
    running: bool,
}

impl HotReloader {
    pub fn new() -> Self {
        HotReloader {
            watched_files: HashMap::new(),
            running: false,
        }
    }

    /// Add a file to watch for changes.
    pub fn watch(&mut self, path: &Path) {
        if let Ok(metadata) = path.metadata() {
            if let Ok(modified) = metadata.modified() {
                self.watched_files.insert(path.to_path_buf(), modified);
            }
        }
    }

    /// Check if any watched files have changed (polling-based).
    /// Returns a list of changed file paths.
    pub fn check_changes(&mut self) -> Vec<PathBuf> {
        let mut changed = Vec::new();
        let mut to_remove = Vec::new();

        for (path, last_modified) in &self.watched_files {
            if !path.exists() {
                to_remove.push(path.clone());
                continue;
            }
            if let Ok(metadata) = path.metadata() {
                if let Ok(modified) = metadata.modified() {
                    if modified > *last_modified {
                        changed.push(path.clone());
                    }
                }
            }
        }

        for path in to_remove {
            self.watched_files.remove(&path);
        }

        // Update timestamps for changed files
        for path in &changed {
            if let Ok(metadata) = path.metadata() {
                if let Ok(modified) = metadata.modified() {
                    self.watched_files.insert(path.clone(), modified);
                }
            }
        }

        changed
    }

    /// Start a background watcher thread that calls `on_change` when files change.
    pub fn start_watcher<F>(&mut self, interval_ms: u64, mut on_change: F)
    where
        F: FnMut(Vec<PathBuf>) + Send + 'static,
    {
        self.running = true;
        let files = self.watched_files.keys().cloned().collect::<Vec<_>>();

        std::thread::spawn(move || {
            let mut last_modified: HashMap<PathBuf, std::time::SystemTime> = HashMap::new();
            for path in &files {
                if let Ok(metadata) = path.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        last_modified.insert(path.clone(), modified);
                    }
                }
            }

            loop {
                std::thread::sleep(Duration::from_millis(interval_ms));
                let mut changed = Vec::new();
                for path in &files {
                    if !path.exists() {
                        continue;
                    }
                    if let Ok(metadata) = path.metadata() {
                        if let Ok(modified) = metadata.modified() {
                            if let Some(prev) = last_modified.get(path) {
                                if modified > *prev {
                                    changed.push(path.clone());
                                    last_modified.insert(path.clone(), modified);
                                }
                            }
                        }
                    }
                }
                if !changed.is_empty() {
                    on_change(changed);
                }
            }
        });
    }

    /// Recompile and reload a module from source.
    pub fn reload_module(source: &str) -> Result<crate::vm::chunk::Chunk, String> {
        let program = crate::compiler::parser::parse_program(source);
        let chunk = crate::vm::compiler::compile_program(&program)
            .map_err(|e| format!("HotReload compile error: {}", e))?;
        Ok(chunk)
    }

    /// Load a module into the VM, replacing existing definitions.
    pub fn load_module(vm: &mut crate::vm::vm::VM, chunk: crate::vm::chunk::Chunk) {
        vm.load_chunk(chunk);
    }
}
