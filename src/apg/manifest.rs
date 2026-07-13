use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PackageInfo {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub license: Option<String>,
    pub authors: Option<Vec<String>>,
    pub repository: Option<String>,
    pub homepage: Option<String>,
    pub keywords: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Dependency {
    pub name: String,
    #[serde(flatten)]
    pub spec: DependencySpec,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(untagged)]
pub enum DependencySpec {
    Simple(String),
    Detailed {
        git: Option<String>,
        branch: Option<String>,
        tag: Option<String>,
        commit: Option<String>,
        path: Option<String>,
    },
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Manifest {
    pub package: PackageInfo,
    pub dependencies: Option<Vec<Dependency>>,
}

impl Manifest {
    pub fn new() -> Self {
        Self {
            package: PackageInfo {
                name: String::new(),
                version: String::new(),
                description: None,
                license: None,
                authors: None,
                repository: None,
                homepage: None,
                keywords: None,
            },
            dependencies: None,
        }
    }
    
    pub fn parse(&self, path: &str) -> Result<Manifest, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let value: toml::Value = toml::from_str(&content)?;

        let package = value.get("package").ok_or("Missing [package] section")?.clone();
        let mut package_obj: PackageInfo = package.try_into()?;

        let dependencies = value.get("dependencies")
            .and_then(|v| if v.as_table().is_some() { Some(v.as_table().unwrap().clone()) } else { None });

        let mut deps: Option<Vec<Dependency>> = None;
        if let Some(dep_table) = dependencies {
            let mut deps_vec = Vec::new();
            for (name, spec) in dep_table.iter() {
                let spec_value = spec.clone();
                deps_vec.push(Dependency {
                    name: name.to_string(),
                    spec: spec_value.try_into()
                        .unwrap_or(DependencySpec::Simple(spec.to_string())),
                });
            }
            deps = Some(deps_vec);
        }

        Ok(Manifest {
            package: package_obj,
            dependencies: deps,
        })
    }
    
    pub fn parse_and_add(&mut self, path: &str, package_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let value: toml::Value = toml::from_str(&content)?;

        let package = value.get("package").ok_or("Missing [package] section")?.clone();
        let mut package_obj: PackageInfo = package.try_into()?;

        let dependencies = value.get("dependencies")
            .and_then(|v| if v.as_table().is_some() { Some(v.as_table().unwrap().clone()) } else { None });

        let mut deps: Option<Vec<Dependency>> = None;
        if let Some(dep_table) = dependencies {
            let mut deps_vec = Vec::new();
            for (name, spec) in dep_table.iter() {
                let spec_value = spec.clone();
                deps_vec.push(Dependency {
                    name: name.to_string(),
                    spec: spec_value.try_into()
                        .unwrap_or(DependencySpec::Simple(spec.to_string())),
                });
            }
            deps = Some(deps_vec);
        }

        self.package = package_obj;
        self.dependencies = deps;
        
        let dep = Dependency {
            name: package_name.to_string(),
            spec: DependencySpec::Simple("1.0.0".to_string()),
        };
        
        if let Some(deps) = &mut self.dependencies {
            deps.push(dep);
        } else {
            self.dependencies = Some(vec![dep]);
        }
        
        self.write(path)
    }
    
    pub fn parse_and_remove(&mut self, path: &str, package_name: &str) -> Result<(), Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let value: toml::Value = toml::from_str(&content)?;

        let package = value.get("package").ok_or("Missing [package] section")?.clone();
        let mut package_obj: PackageInfo = package.try_into()?;

        let dependencies = value.get("dependencies")
            .and_then(|v| if v.as_table().is_some() { Some(v.as_table().unwrap().clone()) } else { None });

        let mut deps: Option<Vec<Dependency>> = None;
        if let Some(dep_table) = dependencies {
            let mut deps_vec = Vec::new();
            for (name, spec) in dep_table.iter() {
                let spec_value = spec.clone();
                deps_vec.push(Dependency {
                    name: name.to_string(),
                    spec: spec_value.try_into()
                        .unwrap_or(DependencySpec::Simple(spec.to_string())),
                });
            }
            deps = Some(deps_vec);
        }

        self.package = package_obj;
        self.dependencies = deps;
        
        if let Some(deps) = &mut self.dependencies {
            deps.retain(|dep| dep.name != package_name);
        }
        
        self.write(path)
    }
    
    pub fn write(&self, path: &str) -> Result<(), Box<dyn std::error::Error>> {
        let mut table = toml::Value::Table(toml::map::Map::new());
        
        let mut package_table_val = toml::Value::Table(toml::map::Map::new());
        {
            let package_table = package_table_val.as_table_mut().unwrap();
            package_table.insert("name".to_string(), toml::Value::String(self.package.name.clone()));
            package_table.insert("version".to_string(), toml::Value::String(self.package.version.clone()));
            if let Some(desc) = &self.package.description {
                package_table.insert("description".to_string(), toml::Value::String(desc.clone()));
            }
            if let Some(license) = &self.package.license {
                package_table.insert("license".to_string(), toml::Value::String(license.clone()));
            }
            if let Some(authors) = &self.package.authors {
                let authors_array = authors.iter().map(|a| toml::Value::String(a.clone())).collect();
                package_table.insert("authors".to_string(), toml::Value::Array(authors_array));
            }
            if let Some(repo) = &self.package.repository {
                package_table.insert("repository".to_string(), toml::Value::String(repo.clone()));
            }
            if let Some(homepage) = &self.package.homepage {
                package_table.insert("homepage".to_string(), toml::Value::String(homepage.clone()));
            }
            if let Some(keywords) = &self.package.keywords {
                let keywords_array = keywords.iter().map(|k| toml::Value::String(k.clone())).collect();
                package_table.insert("keywords".to_string(), toml::Value::Array(keywords_array));
            }
        }
        
        table.as_table_mut().unwrap().insert("package".to_string(), package_table_val);
        
        if let Some(deps) = &self.dependencies {
            let mut deps_table_val = toml::Value::Table(toml::map::Map::new());
            {
                let deps_table = deps_table_val.as_table_mut().unwrap();
                
                for dep in deps {
                    match &dep.spec {
                        DependencySpec::Simple(version) => {
                            deps_table.insert(dep.name.clone(), toml::Value::String(version.clone()));
                        },
                        DependencySpec::Detailed { git, branch, tag, commit, path } => {
                            let mut dep_table_val = toml::Value::Table(toml::map::Map::new());
                            {
                                let dep_table = dep_table_val.as_table_mut().unwrap();
                                if let Some(git_url) = git {
                                    dep_table.insert("git".to_string(), toml::Value::String(git_url.clone()));
                                }
                                if let Some(branch_name) = branch {
                                    dep_table.insert("branch".to_string(), toml::Value::String(branch_name.clone()));
                                }
                                if let Some(tag_name) = tag {
                                    dep_table.insert("tag".to_string(), toml::Value::String(tag_name.clone()));
                                }
                                if let Some(commit_hash) = commit {
                                    dep_table.insert("commit".to_string(), toml::Value::String(commit_hash.clone()));
                                }
                                if let Some(path_str) = path {
                                    dep_table.insert("path".to_string(), toml::Value::String(path_str.clone()));
                                }
                            }
                            deps_table.insert(dep.name.clone(), dep_table_val);
                        }
                    }
                }
            }
            
            table.as_table_mut().unwrap().insert("dependencies".to_string(), deps_table_val);
        }
        
        let content = toml::to_string(&table)?;
        std::fs::write(path, content)?;
        Ok(())
    }
}
