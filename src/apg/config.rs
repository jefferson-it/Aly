use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct RegistryConfig {
    pub name: String,
    pub url: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct WorkspaceConfig {
    pub members: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub registries: Vec<RegistryConfig>,
    pub cache_dir: String,
    pub workspace: Option<WorkspaceConfig>,
    pub security: SecurityConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SecurityConfig {
    pub check_typosquatting: bool,
    pub allow_git_urls: bool,
    pub allow_local_paths: bool,
}

impl Config {
    pub fn load(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let mut config: Config = toml::from_str(&content)?;
        
        if config.cache_dir.is_empty() {
            config.cache_dir = "~/.cache/apg".to_string();
        }
        
        if config.security.check_typosquatting.is_none() {
            config.security.check_typosquatting = Some(true);
        }
        if config.security.allow_git_urls.is_none() {
            config.security.allow_git_urls = Some(true);
        }
        if config.security.allow_local_paths.is_none() {
            config.security.allow_local_paths = Some(true);
        }
        
        Ok(config)
    }
    
    pub fn save(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = toml::to_string(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    pub fn get_default() -> Self {
        Config {
            registries: vec![
                RegistryConfig { 
                    name: "official".to_string(), 
                    url: "https://github.com/aly/apg-registry".to_string() 
                }
            ],
            cache_dir: "~/.cache/apg".to_string(),
            workspace: None,
            security: SecurityConfig {
                check_typosquatting: true,
                allow_git_urls: true,
                allow_local_paths: true,
            },
        }
    }
}
