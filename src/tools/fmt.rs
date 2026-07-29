pub mod fmt {
    pub fn format_code(source: &str) -> String {
        let mut formatted = String::new();
        let mut indent_level: usize = 0;

        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.is_empty() {
                formatted.push('\n');
                continue;
            }

            if trimmed.starts_with('}') || trimmed.starts_with(']') || trimmed.starts_with(')') {
                indent_level = indent_level.saturating_sub(1);
            }

            let indent = "    ".repeat(indent_level);
            formatted.push_str(&format!("{}{}\n", indent, trimmed));

            if trimmed.ends_with('{') || trimmed.ends_with('[') || trimmed.ends_with('(') {
                indent_level += 1;
            }
        }

        formatted
    }
}
pub use fmt::*;
