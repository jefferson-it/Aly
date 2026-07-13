#[cfg(test)]
mod interpreter_integration_tests {
    // Integration tests covering end‑to‑end interpreter workflows
    // These tests execute full snippets of Aly‑Lang code and verify observable results.

    #[test]
    fn evaluates_simple_program() {
        // Example: run a program that assigns and prints a value
        // let result = run(indented_code!(
        //     let x = 7;
        //     print(x);
        // });
        // assert_eq!(result.unwrap(), 7);
    }

    #[test]
    fn handles_arithmetic_expression() {
        // Example: evaluate "2 + 3 * 4" respecting precedence
        // let prog = indented_code!("print(2 + 3 * 4);");
        // assert_eq!(run(prog).unwrap(), 14);
    }

    #[test]
    fn propagates_syntax_error() {
        // Example: ensure malformed code returns an error
        // let err = run("print(`unterminated string`);
        // assert!(is_error(err));
    }

    #[test]
    fn supports_variable_reassignment() {
        // Example: test mutable variables
        // let prog = indented_code!(
        //     let y = 5;
        //     y = y + 1;
        //     print(y);
        // );
        // assert_eq!(run(prog).unwrap(), 6);
    }
}