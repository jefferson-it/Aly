use std::fs;
use std::path::Path;
use std::collections::HashMap;
use crate::apg::manifest::Manifest;
use crate::apg::downloader::GitRepo;

#[derive(Debug, Clone)]
pub struct ResolutionResult {
    pub path: String,
    pub version: String,
    pub repo_url: String,
    pub registry_name: String,
}

pub struct Resolver {
    cache: HashMap<String, ResolutionResult>,
    registries: Vec<crate::apg::registry::Registry>,
}

impl Resolver {
    pub fn new() -> Self {
        let registries = crate::apg::registry::load_registries_config();
        Resolver {
            cache: HashMap::new(),
            registries,
        }
    }
    
    pub fn resolve(&mut self, package_name: &str) -> Result<ResolutionResult, Box<dyn std::error::Error>> {
        if let Some(cached) = self.cache.get(package_name) {
            return Ok(cached.clone());
        }
        
        let mut last_error: Option<String> = None;
        for registry in &self.registries {
            if crate::apg::registry::check_package_exists(registry, package_name) {
                let temp_dir = crate::apg::registry::fetch_package_from_registry(registry, package_name)
                    .ok_or_else(|| format!("Não foi possível baixar pacote {} do registro {}", package_name, registry.name))?;
                
                let manifest_path = format!("{}/packages/{}.toml", temp_dir, package_name);
                let manifest_content = std::fs::read_to_string(&manifest_path)?;
                let manifest: crate::apg::manifest::Manifest = toml::from_str(&manifest_content)?;
                
                let _ = std::fs::remove_dir_all(&temp_dir);
                
                let result = ResolutionResult {
                    path: temp_dir,
                    version: manifest.package.version,
                    repo_url: manifest.package.repository.unwrap_or_default(),
                    registry_name: registry.name.clone(),
                };
                
                self.cache.insert(package_name.to_string(), result.clone());
                return Ok(result);
            }
        }
        
        Err(format!("Pacote '{}' não encontrado em nenhum registro", package_name).into())
    }
}
