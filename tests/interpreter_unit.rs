#[cfg(test)]
mod interpreter_unit_tests {
    // Unit tests for core interpreter components
    use Aly::math_eval::eval_math;

    #[test]
    fn parse_number_literal() {
        let result = eval_math("42").expect("should parse number");
        assert_eq!(result, "42");
    }

    #[test]
    fn parse_binary_operation() {
        let result = eval_math("1 + 2 * 3").expect("should parse binary op");
        assert_eq!(result, "7");
    }

    #[test]
    fn evaluate_literal_number() {
        let result = eval_math("5").expect("should evaluate number");
        assert_eq!(result, "5");
    }

    #[test]
    fn evaluate_addition() {
        let result = eval_math("2 + 3").expect("should add");
        assert_eq!(result, "5");
    }

    #[test]
    fn variable_declaration_syntax() {
        let tokens = aly::runtime::parser::tokenize_line("let x = 10");
        assert!(!tokens.is_empty());
        let literals: Vec<String> = tokens.iter().map(|t| t.literal.clone()).collect();
        assert!(literals.contains(&"let".to_string()));
        assert!(literals.contains(&"x".to_string()));
    }

    #[test]
    fn ternary_operator_exists() {
        let tk = aly::tokens::get_token("?".to_string());
        assert_eq!(tk.literal(), "?");
    }

    #[test]
    fn compound_assign_tokens_exist() {
        assert_eq!(aly::tokens::get_token("+=".to_string()).literal(), "+=");
        assert_eq!(aly::tokens::get_token("-=".to_string()).literal(), "-=");
    }

    #[test]
    fn error_system_works() {
        use Aly::error::{AlyError, AlyErrorKind};
        let err = AlyError::runtime("test error".to_string());
        assert_eq!(err.kind, AlyErrorKind::Runtime);
        assert!(err.message.contains("test error"));
    }
}
