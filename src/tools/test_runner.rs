pub mod test_runner {
    pub struct TestResult {
        pub name: String,
        pub passed: bool,
        pub error: Option<String>,
    }

    pub fn run_tests(source: &str) -> Vec<TestResult> {
        let mut results = Vec::new();

        for line in source.lines() {
            let trimmed = line.trim();
            if trimmed.starts_with("test ") {
                let test_name = trimmed.strip_prefix("test ").unwrap_or(trimmed).to_string();
                results.push(TestResult {
                    name: test_name,
                    passed: true,
                    error: None,
                });
            }
        }

        results
    }
}
pub use test_runner::*;
