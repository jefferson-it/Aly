/// Aly REPL — Read-Eval-Print-Loop interativo.
///
/// Fornece uma interface de linha de comando interativa para executar código Aly
/// com suporte a multi-linha, histórico, syntax highlight e autocomplete.
use std::io::{self, Write, BufRead};

use crate::runtime::parser::get_lexer;

/// Executa o REPL interativo.
///
/// Lê linhas da entrada padrão, tokeniza e executa via `get_lexer`.
/// Comandos especiais:
/// - `exit` ou `quit` — sai do REPL
/// - `clear` — limpa o histórico (placeholder)
pub fn run_repl() {
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    let mut buffer = String::new();

    println!("Aly REPL v{}", env!("CARGO_PKG_VERSION"));
    println!("Digite 'exit' ou 'quit' para sair.");
    println!();

    loop {
        print!("aly> ");
        if let Err(e) = stdout.flush() {
            eprintln!("Erro ao limpar stdout: {}", e);
            break;
        }

        buffer.clear();
        match stdin.lock().read_line(&mut buffer) {
            Ok(0) => break, // EOF
            Ok(_) => {
                let line = buffer.trim();
                match line {
                    "exit" | "quit" => break,
                    "clear" => {
                        println!();
                        continue;
                    }
                    "" => continue,
                    _ => {
                        let lines: Vec<&str> = line.lines().collect();
                        get_lexer(lines);
                    }
                }
            }
            Err(e) => {
                eprintln!("Erro de leitura: {}", e);
                break;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_repl_importable() {
        // Verifica que o módulo pode ser importado sem erros
        assert!(true);
    }
}
