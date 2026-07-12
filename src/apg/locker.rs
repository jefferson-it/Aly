use std::fs;
use std::path::Path;
use std::collections::HashMap;
use toml::Value;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone, Default)]
pub struct Lockfile {
    pub packages: HashMap<String, String>, // name -> version
}

pub fn parse_lockfile(path: &str) -> Result<Lockfile, Box<dyn std::error::Error>> {
    let content = fs::read_to_string(path)?;
    let value: Value = toml::from_str(&content)?;
    
    let packages = value.get("packages")
        .and_then(|v| v.as_table())
        .ok_or("Missing [packages] section in lockfile")?
        .iter()
        .map(|(k, v)| (k.to_string(), v.to_string().trim_matches('"').to_string()))
        .collect();
    
    Ok(Lockfile { packages })
}

pub fn write_lockfile(path: &str, lockfile: &Lockfile) -> Result<(), Box<dyn std::error::Error>> {
    let mut table = toml::Value::Table(toml::map::Map::new());
    let table = table.as_table_mut().unwrap();
    
    let mut packages_table = toml::Value::Table(toml::map::Map::new());
    packages_table.as_table_mut().unwrap().extend(
        lockfile.packages.iter()
            .map(|(k, v)| (k.clone(), toml::Value::String(v.clone())))
    );
    
    table.insert("packages".to_string(), packages_table);
    
    let content = toml::to_string(&table)?;
    fs::write(path, content)?;
    Ok(())
}

pub fn convert_to_lockfile(dependencies: &[(String, String)]) -> Lockfile {
    let mut packages = HashMap::new();
    for (name, version) in dependencies {
        packages.insert(name.clone(), version.clone());
    }
    Lockfile { packages }
}

pub fn generate_lockfile(project_dir: &str) -> Result<Lockfile, Box<dyn std::error::Error>> {
    let manifest_path = Path::new(project_dir).join("aly.toml");
    if !manifest_path.exists() {
        return Err("aly.toml not found".into());
    }
    
    let manifest_content = fs::read_to_string(manifest_path)?;
    let value: Value = toml::from_str(&manifest_content)?;
    
    let mut dependencies = Vec::new();
    
    if let Some(deps_table) = value.get("dependencies") {
        if let Some(deps) = deps_table.as_table() {
            for (name, spec) in deps {
                let version = match spec.as_str() {
                    Some(v) => v.to_string(),
                    None => "0.0.0".to_string(),
                };
                dependencies.push((name.clone(), version));
            }
        }
    }
    
    Ok(convert_to_lockfile(&dependencies))
}
