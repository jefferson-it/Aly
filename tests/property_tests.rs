#[cfg(test)]
mod property_tests {
    use proptest::prelude::*;
    use Aly::compiler::parser::{parse_program, get_precedence, op_to_c};

    proptest! {
        #[test]
        fn test_get_precedence_known_ops(
            op in prop::sample::select(&[
                "=", "+=", "-=", "*=", "/=",
                "or", "OR",
                "and", "AND",
                "xor", "XOR",
                "eq", "EQ", "neq", "NEQ",
                "lt", "LT", "gt", "GT", "lte", "LTE", "gte", "GTE",
                "bor", "bxor", "^",
                "band", "&",
                "shl", "shr", "<<", ">>",
                "+", "-",
                "*", "/", "|", "%",
            ])
        ) {
            let prec = get_precedence(op);
            prop_assert!(prec > 0, "Operator '{}' should have precedence > 0, got {}", op, prec);
        }

        #[test]
        fn test_get_precedence_unknown_ops_returns_zero(
            op in "[a-zA-Z]+"
        ) {
            if ["or", "and", "xor", "eq", "neq", "lt", "gt", "lte", "gte", "bor", "bxor", "band", "shl", "shr", "bor", "band", "bxor"].contains(&op.as_str()) {
                return Ok(());
            }
            let prec = get_precedence(&op);
            prop_assert_eq!(prec, 0);
        }

        #[test]
        fn test_op_to_c_known_ops(
            op in prop::sample::select(&[
                ("eq", "=="), ("EQ", "=="), ("neq", "!="), ("NEQ", "!="),
                ("lt", "<"), ("LT", "<"), ("lte", "<="), ("LTE", "<="),
                ("gt", ">"), ("GT", ">"), ("gte", ">="), ("GTE", ">="),
                ("and", "&&"), ("AND", "&&"), ("or", "||"), ("OR", "||"),
                ("xor", "^"), ("XOR", "^"),
                ("not", "!"), ("NOT", "!"),
                ("|", "%"), ("band", "&"), ("&", "&"),
                ("bor", "|"), ("bxor", "^"), ("^", "^"),
                ("bnot", "~"), ("~", "~"),
                ("shl", "<<"), ("<<", "<<"), ("shr", ">>"), (">>", ">>"),
            ])
        ) {
            let (input, expected) = op;
            let result = op_to_c(input);
            prop_assert_eq!(result, expected, "op_to_c('{}') = '{}', expected '{}'", input, result, expected);
        }

        #[test]
        fn test_op_to_c_passthrough_unknown(op in "[a-z]+") {
            if ["eq", "neq", "lt", "lte", "gt", "gte", "and", "or", "xor", "not", "band", "bor", "bxor", "bnot", "shl", "shr"].contains(&op.as_str()) {
                return Ok(());
            }
            let result = op_to_c(&op);
            prop_assert_eq!(result, op.clone());
        }
    }

    proptest! {
        #[test]
        fn test_parse_program_number_literals(n in -1000000i64..1000000) {
            let source = n.to_string();
            let result = parse_program(&source);
            prop_assert!(!result.stmts.is_empty(), "Should parse number literal {}", n);
        }

        #[test]
        fn test_parse_program_let_statements(name in "[a-zA-Z_][a-zA-Z0-9_]*") {
            let source = format!("let {} = 1", name);
            let result = parse_program(&source);
            prop_assert!(!result.stmts.is_empty(), "Should parse let statement with name {}", name);
        }

        #[test]
        fn test_parse_program_float_literals(f in -1000.0f64..1000.0) {
            let source = format!("{}", f);
            if source.is_empty() || source == "-" || source == "+" {
                return Ok(());
            }
            let result = parse_program(&source);
            prop_assert!(!result.stmts.is_empty(), "Should parse float literal {}", source);
        }

        #[test]
        fn test_parse_program_string_literals(s in ".*") {
            let source = format!("\"{}\"", s.replace('"', "\\\""));
            let result = parse_program(&source);
            prop_assert!(!result.stmts.is_empty(), "Should parse string literal");
        }
    }
}