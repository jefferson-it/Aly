use std::fs;
use std::path::Path;
use std::collections::{HashMap, VecDeque};
use crate::apg::manifest::{Manifest, DependencySpec};
use crate::apg::resolver::Resolver;

pub fn get_dependency_graph(project_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let manifest_path = Path::new(project_dir).join("aly.toml");
    if !manifest_path.exists() {
        return Err("aly.toml not found".into());
    }
    
    let manifest = Manifest::new();
    let manifest = manifest.parse(&manifest_path.to_string_lossy())?;
    println!("Projeto: {}", manifest.package.name);
    println!("Versão: {}", manifest.package.version);
    
    if let Some(deps) = manifest.dependencies {
        println!("\nDependências:");
        println!("  {} -> {}", deps.len(), deps.len());
        
        let mut queue = VecDeque::new();
        for dep in &deps {
            queue.push_back((dep.name.clone(), 0));
        }
        
        let mut visited = HashMap::new();
        while let Some((dep_name, depth)) = queue.pop_front() {
            if visited.contains_key(&dep_name) {
                continue;
            }
            visited.insert(dep_name.clone(), depth);
            
            let indent = "  ".repeat(depth);
            print!("{}{}", indent, dep_name);
            
            if depth < 2 {
                queue.push_back((dep_name.clone(), depth + 1));
            }
            println!(" ({}).", depth);
        }
    }
    
    Ok(())
}

pub fn clean_cache() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir()
        .ok_or("Could not find home directory")?;
    
    let cache_dir = home_dir.join(".cache").join("apg");
    
    if cache_dir.exists() {
        match fs::read_dir(&cache_dir) {
            Ok(entries) => {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_dir() {
                        match fs::remove_dir_all(&path) {
                            Ok(_) => {},
                            Err(e) => {
                                println!("Warning: Failed to remove {}: {}", path.display(), e);
                            }
                        }
                    } else {
                        match fs::remove_file(&path) {
                            Ok(_) => {},
                            Err(e) => {
                                println!("Warning: Failed to remove {}: {}", path.display(), e);
                            }
                        }
                    }
                }
            },
            Err(e) => {
                return Err(format!("Failed to read cache directory: {}", e).into());
            }
        }
        
        println!("Cache cleaned: {}", cache_dir.display());
    } else {
        println!("No cache directory found: {}", cache_dir.display());
    }
    
    Ok(())
}

pub fn get_cache_info() -> Result<(), Box<dyn std::error::Error>> {
    let home_dir = dirs::home_dir()
        .ok_or("Could not find home directory")?;
    
    let cache_dir = home_dir.join(".cache").join("apg");
    
    if !cache_dir.exists() {
        println!("Cache directory does not exist: {}", cache_dir.display());
        return Ok(());
    }
    
    println!("Cache directory: {}", cache_dir.display());
    
    let packages_dir = cache_dir.join("packages");
    if packages_dir.exists() && packages_dir.is_dir() {
        let entries = fs::read_dir(packages_dir)?;
        let mut package_count = 0;
        let mut total_size = 0;
        
        for entry in entries.flatten() {
            if let Ok(metadata) = entry.metadata() {
                package_count += 1;
                total_size += metadata.len();
            }
        }
        
        println!("Packages cached: {}", package_count);
        
        if package_count > 0 {
            println!("Total size: {} bytes", total_size);
            if total_size > 1024 {
                println!("            {:.2} KB", total_size as f64 / 1024.0);
            }
        }
    } else {
        println!("No packages directory found in cache");
    }
    
    Ok(())
}
