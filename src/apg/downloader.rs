use std::fs;
use std::path::Path;
use std::process::Command;
use crate::apg::manifest::DependencySpec;

#[derive(Debug, Clone)]
pub struct GitRepo {
    pub url: String,
    pub branch: Option<String>,
    pub tag: Option<String>,
    pub commit: Option<String>,
    pub path: Option<String>,
}

pub fn clone_repository(repo: &GitRepo, target_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut clone_cmd = Command::new("git");
    clone_cmd.arg("clone");
    
    if let Some(branch) = &repo.branch {
        clone_cmd.args(["--branch", branch]);
    }
    
    if let Some(tag) = &repo.tag {
        clone_cmd.args(["--tag", tag]);
    }
    
    if let Some(commit) = &repo.commit {
        clone_cmd.args(["--depth", "1"]);
        clone_cmd.arg(&format!("{},{}", repo.url, commit));
    } else {
        clone_cmd.arg(&repo.url);
    }
    
    clone_cmd.arg(target_dir);
    
    let output = clone_cmd.output()?;
    if !output.status.success() {
        return Err(format!("Failed to clone repository: {}", String::from_utf8_lossy(&output.stderr)).into());
    }
    
    Ok(())
}

pub fn parse_dependency_spec(spec: &DependencySpec) -> GitRepo {
    match spec {
        DependencySpec::Simple(version) => {
            GitRepo {
                url: format!("https://github.com/example/{}.git", version),
                branch: None,
                tag: None,
                commit: None,
                path: None,
            }
        },
        DependencySpec::Detailed { git, branch, tag, commit, path } => {
            GitRepo {
                url: git.clone().unwrap_or_default(),
                branch: branch.clone(),
                tag: tag.clone(),
                commit: commit.clone(),
                path: path.clone(),
            }
        }
    }
}

pub fn download_package(package_name: &str, spec: &DependencySpec) -> Result<String, Box<dyn std::error::Error>> {
    let cache_dir = dirs::home_dir()
        .ok_or("Could not find home directory")?
        .join(".cache")
        .join("apg")
        .join("packages")
        .join(package_name);
    
    if cache_dir.exists() && cache_dir.is_dir() {
        return Ok(cache_dir.to_string_lossy().to_string());
    }
    
    let repo = parse_dependency_spec(spec);
    
    clone_repository(&repo, &cache_dir.to_string_lossy())?;
    
    Ok(cache_dir.to_string_lossy().to_string())
}
