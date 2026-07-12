use std::fs;
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone)]
pub struct Registry {
    pub name: String,
    pub url: String,
}

impl Registry {
    pub fn new(name: &str, url: &str) -> Self {
        Registry {
            name: name.to_string(),
            url: url.to_string(),
        }
    }
}

pub fn get_default_registry() -> Registry {
    Registry::new("official", "https://github.com/aly/apg-registry")
}

pub fn fetch_package_from_registry(registry: &Registry, package_name: &str) -> Option<String> {
    let temp_dir = format!("/tmp/apg-registry-check-{}.git", package_name);
    let manifest_path = format!("{}/packages/{}.toml", temp_dir, package_name);
    
    let clone_result = Command::new("git")
        .args(["clone", "--depth", "1", "--single-branch", "--quiet", 
               &format!("{}/{}.git", registry.url, package_name), &temp_dir])
        .output();
    
    if let Ok(output) = clone_result {
        if output.status.success() {
            if Path::new(&manifest_path).exists() {
                return Some(temp_dir);
            }
            let _ = std::fs::remove_dir_all(&temp_dir);
        } else {
            let _ = std::fs::remove_dir_all(&temp_dir);
        }
    }
    
    None
}

pub fn load_registries_config() -> Vec<Registry> {
    let config_path = dirs::home_dir().unwrap().join(".config").join("apg").join("registries.toml");
    
    if config_path.exists() {
        match std::fs::read_to_string(&config_path) {
            Ok(content) => {
                match toml::from_str::<Vec<Registry>>(&content) {
                    Ok(registries) => return registries,
                    Err(_) => {
                        eprintln!("Aviso: Erro ao analisar configurações de registro em {}", config_path.display());
                    }
                }
            },
            Err(_) => {
                eprintln!("Aviso: Não foi possível ler configurações de registro em {}", config_path.display());
            }
        }
    }
    
    vec![get_default_registry()]
}

pub fn check_package_exists(registry: &Registry, package_name: &str) -> bool {
    fetch_package_from_registry(registry, package_name).is_some()
}
