pub mod linter {
    pub struct LintDiagnostic {
        pub line: usize,
        pub message: String,
    }

    pub fn lint_code(source: &str) -> Vec<LintDiagnostic> {
        let mut diagnostics = Vec::new();

        for (i, line) in source.lines().enumerate() {
            let line_num = i + 1;
            let trimmed = line.trim();

            if trimmed.contains("let ") && !trimmed.contains('=') && !trimmed.ends_with(';') {
                diagnostics.push(LintDiagnostic {
                    line: line_num,
                    message: "Variável declarada sem valor inicial ou ponto e vírgula.".to_string(),
                });
            }

            if line.len() > 120 {
                diagnostics.push(LintDiagnostic {
                    line: line_num,
                    message: "Linha excede 120 caracteres.".to_string(),
                });
            }
        }

        diagnostics
    }
}
pub use linter::*;
