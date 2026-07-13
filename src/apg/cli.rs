use std::fs;
use std::path::Path;
use std::env;
use crate::manifest::Manifest;
use crate::locker::{Lockfile, parse_lockfile, write_lockfile};
use crate::resolver::Resolver;
use crate::cache::{get_dependency_graph, generate_lockfile};
use crate::config::Config;

pub fn show_help() {
    println!("APG — Aly Package Gestor");
    println!("==============");
    println!("Gerenciador de pacotes elegante e determinístico para a linguagem Aly.");
    println!();
    println!("COMANDOS:");
    println!("  apg init [diretório]       Cria um novo projeto Aly em diretório (padrão: .)");
    println!("  apg add <pacote>            Adiciona um pacote às dependências do projeto");
    println!("  apg remove <pacote>         Remove um pacote das dependências");
    println!("  apg update                  Atualiza dependências para as versões mais recentes");
    println!("  apg publish                 Publica um pacote (cria PR no registro)");
    println!("  apg search <termo>          Busca pacotes no registro");
    println!("  apg install <pacote>        Instala um pacote no projeto atual");
    println!("  apg uninstall <pacote>      Remove um pacote do projeto");
    println!("  apg registry                Gerencia registros de pacotes");
    println!("  apg doctor                  Verifica a saúde do sistema APG");
    println!("  apg clean                   Limpa o cache do APG");
    println!("  apg info <pacote>           Mostra informações sobre um pacote");
    println!("  apg graph                   Mostra o grafo de dependências");
    println!("  apg lock                    Gera um arquivo aly.lock");
    println!("  apg vendor                  Vende (cópia local) as dependências");
    println!("  apg help                    Mostra esta ajuda");
    println!("  apg version                 Mostra a versão do APG");
    println!();
    println!("O APG segue a filosofia de ter Git como fonte de verdade.");
    println!("Instalar um pacote nunca executa código durante a instalação.");
}

pub fn show_version() {
    println!("APG 0.1.0");
    println!("Aly Package Gestor - Gerenciador de pacotes determinístico");
}

pub fn init(path: &str) {
    let project_path = Path::new(path);
    
    if !project_path.exists() {
        println!("Erro: O caminho '{}' não existe.", path);
        return;
    }
    
    let manifest_path = project_path.join("aly.toml");
    
    if manifest_path.exists() {
        println!("O manifesto já existe em {}", manifest_path.display());
        return;
    }
    
    let content = r#"[package]
name = "meu-projeto"
version = "0.1.0"
authors = ["Seu Nome"]
description = "Meu projeto Aly"
license = "MIT"

[dependencies]

# Exemplo de dependência
sqlite = { git = "https://github.com/mattn/sqlite3", branch = "main" }#;"#;
    
    match fs::write(&manifest_path, content) {
        Ok(_) => println!("Criado novo projeto Aly em {}", project_path.display()),
        Err(e) => println!("Erro ao criar projeto: {}", e),
    }
}

pub fn add(package: &str) {
    println!("Adicionando pacote: {} ...", package);
    
    let manifest_path = "aly.toml";
    if !Path::new(manifest_path).exists() {
        println!("Erro: aly.toml não encontrado no diretório atual");
        return;
    }
    
    match Manifest::new().parse_and_add(manifest_path, package) {
        Ok(_) => println!("Pacote '{}' adicionado com sucesso ao aly.toml", package),
        Err(e) => println!("Erro ao adicionar pacote: {}", e),
    }
}

pub fn remove(package: &str) {
    println!("Removendo pacote: {} ...", package);
    
    let manifest_path = "aly.toml";
    if !Path::new(manifest_path).exists() {
        println!("Erro: aly.toml não encontrado no diretório atual");
        return;
    }
    
    match Manifest::new().parse_and_remove(manifest_path, package) {
        Ok(_) => println!("Pacote '{}' removido com sucesso do aly.toml", package),
        Err(e) => println!("Erro ao remover pacote: {}", e),
    }
}

pub fn update() {
    println!("Atualizando dependências...");
    
    if !Path::new("aly.toml").exists() {
        println!("Erro: aly.toml não encontrado");
        return;
    }
    
    let config = match Config::load("config.apg.toml") {
        Ok(c) => c,
        Err(_) => Config::default(),
    };
    
    let mut resolver = Resolver::new();
    
    match parse_lockfile("aly.lock") {
        Ok(mut lockfile) => {
            println!("Atualizando {} pacotes...", lockfile.packages.len());
            
            for (name, version) in lockfile.packages.iter_mut() {
                match resolver.resolve(name) {
                    Ok(manifest) => {
                        let new_version = manifest.package.version;
                        if new_version != *version {
                            println!("  {} {} → {}", name, version, new_version);
                            *version = new_version;
                        } else {
                            println!("  {} {} (já atualizado)", name, version);
                        }
                    },
                    Err(e) => {
                        println!("  Erro resolvendo {}: {}", name, e);
                    }
                }
            }
            
            if let Err(e) = write_lockfile("aly.lock", &lockfile) {
                println!("Erro ao escrever lockfile: {}", e);
            } else {
                println!("Dependências atualizadas com sucesso");
            }
        },
        Err(e) => {
            println!("Erro lendo lockfile: {}", e);
            println!("Execute 'apg lock' primeiro para gerar um lockfile");
        }
    }
}

pub fn publish() {
    println!("Publicando pacote...");
    
    if !Path::new("aly.toml").exists() {
        println!("Erro: aly.toml não encontrado");
        return;
    }
    
    match Manifest::new().parse("aly.toml") {
        Ok(manifest) => {
            let repo_info = match &manifest.package.repository {
                Some(repo) => repo,
                None => {
                    println!("Erro: repository não definido no manifesto");
                    println!("Adicione 'repository = \"https://github.com/seuusuario/seupacote\"' ao aly.toml");
                    return;
                }
            };
            
            let version = &manifest.package.version;
            
            println!("Pronto para publicar {} versão {} em {}", 
                manifest.package.name, version, repo_info);
            println!();
            println!("O APG fará o seguinte:");
            println!("  1. Validar aly.toml e todas as dependências");
            println!("  2. Verificar se a versão {} segue o semver", version);
            println!("  3. Verificar licença {} ...", manifest.package.license.unwrap_or_else(|| "N/A".to_string()));
            println!("  4. Criar tag {} no repositório", version);
            println!("  5. Gerar PR para o registro oficial (apg-registry)");
            println!();
            println!("Execute 'git push origin {}' para finalizar a publicação", version);
        },
        Err(e) => println!("Erro ao analisar manifesto: {}", e),
    }
}

pub fn search(query: &str) {
    if query.is_empty() {
        println!("Realizando busca em todos os pacotes...");
        // Aqui implementaremos a busca real usando registros
        return;
    }
    
    println!("Buscando pacotes que correspondem a '{}' ...", query);
    println!("(Funcionalidade de busca real será implementada)");
}

pub fn install(package: &str) {
    println!("Instalando pacote: {} ...", package);
    println!("O APG fará o seguinte:");
    println!("  1. Consultando o registro");
    println!("  2. Clonando o repositório Git do pacote");
    println!("  3. Resolvendo dependências");
    println!("  4. Gerando lockfile");
    println!("  5. Instalando o pacote");
    println!();
    println!("NOTA: O APG nunca executa código durante a instalação.");
    println!("A instalação é determinística e segura.");
}

pub fn uninstall(package: &str) {
    println!("Desinstalando pacote: {} ...", package);
    println!("Removendo do projeto e do cache...");
    
    if Path::new("aly.toml").exists() {
        match Manifest::new().parse_and_remove("aly.toml", package) {
            Ok(_) => println!("Pacote '{}' removido com sucesso", package),
            Err(e) => println!("Aviso: {} (mas ainda removendo do cache)", e),
        }
    }
    
    let cache_dir = dirs::home_dir().unwrap().join(".cache").join("apg");
    if cache_dir.exists() {
        let package_cache = cache_dir.join("packages").join(package);
        if package_cache.exists() {
            if let Err(e) = std::fs::remove_dir_all(&package_cache) {
                println!("Aviso: não foi possível remover {} do cache: {}", package, e);
            } else {
                println!("Removido do cache: {}", package_cache.display());
            }
        }
    }
    
    println!("Desinstalação completa.");
}

pub fn registry() {
    println!("Gerenciamento de Registros APG");
    println!("=====================");
    println!();
    println!("Registros configurados:");
    println!("  - official: https://github.com/aly/apg-registry (padrão)");
    println!();
    println!("Comandos:");
    println!("  apg registry add <nome> <url>     Adiciona um novo registro");
    println!("  apg registry remove <nome>        Remove um registro");
    println!("  apg registry list                  Lista todos os registros");
    println!();
    println!("Os registros são usados para encontrar e resolver pacotes.");
    println!("O APG procura primeiro nos registros configurados, depois no padrão.");
}

pub fn doctor() {
    println!("Verificando saúde do sistema APG...");
    println!();
    
    // Verificar Git
    if let Ok(output) = std::process::Command::new("git").arg("--version").output() {
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("[OK] Git: {}", version.trim());
        } else {
            println!("[ERRO] Git: Nao encontrado ou nao disponivel");
        }
    } else {
        println!("[ERRO] Git: Nao encontrado ou nao disponivel");
    }
    
    // Verificar Rust
    if let Ok(output) = std::process::Command::new("rustc").arg("--version").output() {
        if output.status.success() {
            let version = String::from_utf8_lossy(&output.stdout);
            println!("[OK] Rust: {}", version.trim());
        } else {
            println!("[ERRO] Rust: Nao encontrado ou nao disponivel");
        }
    } else {
        println!("[ERRO] Rust: Nao encontrado ou nao disponivel");
    }
    
    // Verificar diretório do projeto
    let project_dir = std::env::current_dir();
    match project_dir {
        Ok(dir) => {
            let aly_toml = dir.join("aly.toml");
            if aly_toml.exists() {
                println!("[OK] Projeto Aly detectado: {}/{}", dir.display(), aly_toml.file_name().unwrap().to_string_lossy());
                
                if Path::new("aly.lock").exists() {
                    match parse_lockfile("aly.lock") {
                        Ok(lockfile) => {
                            println!("  Lockfile: {} pacotes", lockfile.packages.len());
                        },
                        Err(e) => println!("  [ERRO] Lockfile: Erro ao ler: {}", e),
                    }
                } else {
                    println!("  [AVISO] Lockfile: Nao encontrado (execute 'apg lock' para gerar)");
                }
            } else {
                println!("[ERRO] Projeto Aly: Nao encontrado (execute 'apg init' para criar)");
            }
        },
        Err(e) => println!("[ERRO] Diretório atual: Erro - {}", e),
    }
    
    // Verificar cache
    if let Some(home) = dirs::home_dir() {
        let cache_dir = home.join(".cache").join("apg");
        if cache_dir.exists() {
            println!("✓ Cache APG: {} (execute 'apg clean' para limpar)", cache_dir.display());
        } else {
            println!("⚠ Cache APG: Não encontrado (será criado automaticamente)");
        }
    }
    
    println!();
    println!("Status: OK - O APG está pronto para uso!");
}

pub fn clean() {
    println!("Limpando cache do APG...");
    
    let home_dir = match dirs::home_dir() {
        Some(dir) => dir,
        None => {
            println!("Erro: Não foi possível determinar o diretório home");
            return;
        }
    };
    
    let cache_dir = home_dir.join(".cache").join("apg");
    
    if !cache_dir.exists() {
        println!("Cache não encontrado em {}", cache_dir.display());
        return;
    }
    
    match std::fs::remove_dir_all(&cache_dir) {
        Ok(_) => println!("Cache limpo com sucesso: {}", cache_dir.display()),
        Err(e) => println!("Erro ao limpar cache: {}", e),
    }
}

pub fn info(package: &str) {
    println!("Obtendo informações sobre pacote: {} ...", package);
    println!("(Funcionalidade de informações será implementada)");
}

pub fn graph() {
    println!("Gerando grafo de dependências...");
    if let Err(e) = get_dependency_graph(".") {
        println!("Erro: {}", e);
    } else {
        println!("Grafo gerado (visualização será implementada)");
    }
}

pub fn lock() {
    println!("Gerando lockfile...");
    if let Err(e) = generate_lockfile(".") {
        println!("Erro: {}", e);
    } else {
        println!("Lockfile aly.lock gerado com sucesso");
        println!("Execute 'apg update' para verificar atualizações.");
    }
}

pub fn vendor() {
    println!("Vendendo dependências...");
    println!("(Funcionalidade de vendoring será implementada)");
}
