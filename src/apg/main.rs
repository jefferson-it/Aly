mod cli;
mod config;
mod manifest;
mod registry;
mod resolver;
mod locker;
mod downloader;
mod cache;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Uso: apg <comando> [opções]");
        return;
    }

    match args[1].as_str() {
        "init" => cli::init(),
        "add" => {
            if args.len() > 2 { cli::add(&args[2]) } else { println!("Erro: Nome do pacote necessário") }
        },
        "remove" => {
            if args.len() > 2 { cli::remove(&args[2]) } else { println!("Erro: Nome do pacote necessário") }
        },
        "update" => cli::update(),
        "publish" => cli::publish(),
        "search" => {
            if args.len() > 2 { cli::search(&args[2]) } else { println!("Erro: Termo de busca necessário") }
        },
        "install" => {
            if args.len() > 2 { cli::install(&args[2]) } else { println!("Erro: Nome do pacote necessário") }
        },
        "uninstall" => {
            if args.len() > 2 { cli::uninstall(&args[2]) } else { println!("Erro: Nome do pacote necessário") }
        },
        "registry" => cli::registry(),
        "doctor" => cli::doctor(),
        "clean" => cli::clean(),
        "info" => {
            if args.len() > 2 { cli::info(&args[2]) } else { println!("Erro: Nome do pacote necessário") }
        },
        "graph" => cli::graph(),
        "lock" => cli::lock(),
        "vendor" => cli::vendor(),
        "help" | "--help" => cli::show_help(),
        _ => {
            println!("Comando desconhecido: {}", args[1]);
            cli::show_help();
        }
    }
}
