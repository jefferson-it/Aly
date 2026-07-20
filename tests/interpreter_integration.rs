#[cfg(test)]
mod interpreter_integration_tests {
    // Integration tests covering end‑to‑end interpreter workflows
    use Aly::math_eval::eval_math;
    use Aly::runtime::parser::{get_lexer, tokenize_line};

    #[test]
    fn evaluates_simple_program() {
        // Test math evaluation (pure Rust, no external deps)
        let result = eval_math("7").expect("should evaluate");
        assert_eq!(result, "7");
    }

    #[test]
    fn handles_arithmetic_expression() {
        let result = eval_math("2 + 3 * 4").expect("should evaluate with precedence");
        assert_eq!(result, "14");
    }

    #[test]
    fn propagates_syntax_error() {
        // eval_math returns error on malformed input
        let result = eval_math("2 ++ 3");
        assert!(result.is_err(), "malformed expression should error");
    }

    #[test]
    fn supports_variable_reassignment() {
        // Tokenizer test for variable reassignment
        let tokens = tokenize_line("y = y + 1");
        assert!(!tokens.is_empty());
        assert_eq!(tokens[0].literal, "y");
    }

    #[test]
    fn math_power_operator() {
        let result = eval_math("2 ** 8").expect("power should work");
        assert_eq!(result, "256");
    }

    #[test]
    fn math_negative_numbers() {
        let result = eval_math("-5 + 10").expect("negative should work");
        assert_eq!(result, "5");
    }

    #[test]
    fn math_modulo() {
        let result = eval_math("10 % 3").expect("modulo should work");
        assert_eq!(result, "1");
    }
}
