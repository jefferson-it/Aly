pub mod doc {
    pub fn generate_docs(source: &str) -> String {
        let mut docs = String::from("# Documentação do Módulo\n\n");

        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("#") {
                docs.push_str(&format!("{}\n", trimmed));
            } else if trimmed.starts_with("fun ") || trimmed.starts_with("função ") {
                docs.push_str(&format!("\n### Função `{}`\n", trimmed));
            } else if trimmed.starts_with("Schema ") || trimmed.starts_with("schema ") {
                docs.push_str(&format!("\n## Schema `{}`\n", trimmed));
            }
        }

        docs
    }
}
pub use doc::*;
