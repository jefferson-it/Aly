use std::fs;
use std::path::Path;
use clap::{Parser, Args as ClapArgs};

#[derive(Parser)]
#[command(author, version, about = "Format Aly code files")]
struct CliArgs {
    /// Path to the file to format
    #[arg(value_name = "FILE")]
    file: String,
}

fn main() {
    let args = CliArgs::parse();

    let path = Path::new(&args.file);
    if !path.exists() {
        eprintln!("File not found: {}", args.file);
        std::process::exit(1);
    }

    let content = fs::read_to_string(path).expect("Failed to read file");
    let formatted = format_code(&content);
    fs::write(path, formatted).expect("Failed to write file");
    println!("Formatted {}", args.file);
}

fn format_code(content: &str) -> String {
    let mut lines = content.lines().collect::<Vec<_>>();
    let mut indent_level = 0;
    let mut formatted = String::new();

    for line in lines.iter_mut() {
        // Trim leading/trailing whitespace
        let line = line.trim();

        // Skip empty lines
        if line.is_empty() {
            formatted.push_str("\n");
            continue;
        }

        // Determine indentation based on current indent level
        let spaces = "    ".repeat(indent_level);
        formatted.push_str(&spaces);

        // Handle block start/end
        if line.ends_with(":") || line.ends_with("{") {
            // Increase indent for next line
            indent_level += 1;
        } else if line.starts_with("}") || line.starts_with("]") || line.starts_with(")") {
            // Decrease indent for next line
            indent_level = indent_level.saturating_sub(1);
        }

        formatted.push_str(&line);
        formatted.push_str("\n");
    }

    formatted
}