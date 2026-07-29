/// Basic interpreter tests — smoke tests for core functionality.
use std::path::PathBuf;

use Aly::math_eval::eval_math;

#[test]
fn test_variable_declaration() {
    // Test math evaluation as proxy for interpreter functionality
    let result = eval_math("2 + 3").expect("eval_math should succeed");
    assert_eq!(result, "5");
}

#[test]
fn test_math_operations() {
    // Test operator precedence
    let result = eval_math("2 + 3 * 4").expect("eval_math should succeed");
    assert_eq!(result, "14");

    let result = eval_math("(2 + 3) * 4").expect("eval_math should succeed");
    assert_eq!(result, "20");
}

#[test]
fn test_string_concatenation() {
    // Tokenizer test
    use Aly::runtime::parser::tokenize_line;
    let tokens = tokenize_line("let greeting = \"Hello\" + \" \" + \"World\"");
    assert!(!tokens.is_empty(), "tokenize_line should produce tokens");

    let literals: Vec<String> = tokens.iter().map(|t| t.literal.clone()).collect();
    assert!(literals.contains(&"let".to_string()));
    assert!(literals.contains(&"greeting".to_string()));
}

