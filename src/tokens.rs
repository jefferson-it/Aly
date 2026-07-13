mod tokens {
    use core::fmt;

    use crate::validators::{is_any_value, reference::is_reference};

    #[derive(PartialEq, Clone)]
    pub enum Tokens {
        // Variable
        Let,
        Const,
        Identifier,
        Reference,
        Value,
        This,
        Pointer,
        // Brackets/Braces
        LeftParenthesis,
        RightParenthesis,
        LeftBrace,
        RightBrace,
        LeftBracket,
        RightBracket,
        // Structures
        If,
        Elif,
        Else,
        Try,
        Catch,
        Finally,
        Throw,
        Fun,
        Return,
        Async,
        Await,
        Loop,
        Do,
        Match,
        Struct,
        Model,
        // Math Operator
        Addition,
        Subtraction,
        Division,
        Multiplication,
        Modulus,
        Percent,
        // Relational Operator
        Equal,
        NotEqual,
        LessThan,
        LessThanOrEqual,
        GreaterThan,
        GreaterThanOrEqual,
        // Conditional Operator
        Not,
        And,
        Or,
        Xor,
        // Another
        Comma,        // ,
        Semicolon,    // ;
        Colon,        // :
        Dot,          // .
        CommentLine,  // #
        CommentMulti, // ## ... ##
        SimpleQuote,
        DupleQuote,
        None,
    }

    impl fmt::Display for Tokens {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.literal())
        }
    }

    impl fmt::Debug for Tokens {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "literal: {}, Id: {}", self.literal(), self.id())
        }
    }

    impl Tokens {
        pub fn literal(&self) -> String {
            let result = match self {
                Tokens::Let => "let",
                Tokens::Const => "const",
                Tokens::Identifier => "=",
                Tokens::This => "@",
                Tokens::Pointer => "&",
                Tokens::LeftParenthesis => "(",
                Tokens::RightParenthesis => ")",
                Tokens::LeftBrace => "{",
                Tokens::RightBrace => "}",
                Tokens::LeftBracket => "[",
                Tokens::RightBracket => "]",
                Tokens::If => "if",
                Tokens::Elif => "elif",
                Tokens::Else => "else",
                Tokens::Try => "try",
                Tokens::Catch => "catch",
                Tokens::Finally => "finally",
                Tokens::Throw => "throw",
                Tokens::Fun => "fun",
                Tokens::Return => "return",
                Tokens::Async => "async",
                Tokens::Await => "await",
                Tokens::Loop => "loop",
                Tokens::Do => "do",
                Tokens::Match => "match",
                Tokens::Struct => "struct",
                Tokens::Model => "model",
                Tokens::Addition => "+",
                Tokens::Subtraction => "-",
                Tokens::Division => "/",
                Tokens::Multiplication => "*",
                Tokens::Modulus => "|",
                Tokens::Percent => "%",
                Tokens::Equal => "eq",
                Tokens::NotEqual => "neq",
                Tokens::LessThan => "lt",
                Tokens::LessThanOrEqual => "lte",
                Tokens::GreaterThan => "gt",
                Tokens::GreaterThanOrEqual => "gte",
                Tokens::Not => "not",
                Tokens::And => "and",
                Tokens::Or => "or",
                Tokens::Xor => "xor",
                Tokens::Comma => ",",
                Tokens::Semicolon => ";",
                Tokens::Colon => ":",
                Tokens::Dot => ".",
                Tokens::CommentLine => "#",
                Tokens::CommentMulti => "##",
                Tokens::Reference => "ref",
                Tokens::Value => "val",
                Tokens::None => "None",
                Tokens::SimpleQuote => "'",
                Tokens::DupleQuote => "\"",
            };

            String::from(result)
        }

        pub fn id(&self) -> String {
            let result = match self {
                Tokens::Let => "def_let",
                Tokens::Const => "def_const",
                Tokens::Identifier => "identifier",
                Tokens::This => "this",
                Tokens::Pointer => "pointer",
                Tokens::LeftParenthesis => "left_parenthesis",
                Tokens::RightParenthesis => "right_parenthesis",
                Tokens::LeftBrace => "left_brace",
                Tokens::RightBrace => "right_brace",
                Tokens::LeftBracket => "left_bracket",
                Tokens::RightBracket => "right_bracket",
                Tokens::If => "block_if",
                Tokens::Elif => "block_elif",
                Tokens::Else => "block_else",
                Tokens::Try => "block_try",
                Tokens::Catch => "block_catch",
                Tokens::Finally => "block_finally",
                Tokens::Throw => "throw",
                Tokens::Fun => "def_fun",
                Tokens::Return => "fun_return",
                Tokens::Async => "fun_async",
                Tokens::Await => "fun_await",
                Tokens::Loop => "block_loop",
                Tokens::Do => "block_do",
                Tokens::Match => "block_match",
                Tokens::Struct => "def_struct",
                Tokens::Model => "def_model",
                Tokens::Addition => "math_addition",
                Tokens::Subtraction => "math_subtraction",
                Tokens::Division => "math_division",
                Tokens::Multiplication => "math_multiplication",
                Tokens::Modulus => "math_modulus",
                Tokens::Percent => "math_percent",
                Tokens::Equal => "relational_equal",
                Tokens::NotEqual => "relational_not_equal",
                Tokens::LessThan => "relational_less",
                Tokens::LessThanOrEqual => "relational_less_equal",
                Tokens::GreaterThan => "relational_greater_than",
                Tokens::GreaterThanOrEqual => "relational_greater_equal",
                Tokens::Not => "conditional_not",
                Tokens::And => "conditional_and",
                Tokens::Or => "conditional_or",
                Tokens::Xor => "conditional_xor",
                Tokens::Comma => "comma",
                Tokens::Semicolon => "semicolon",
                Tokens::Colon => "colon",
                Tokens::Dot => "dot",
                Tokens::CommentLine => "comment",
                Tokens::CommentMulti => "comment_multi",
                Tokens::Reference => "reference",
                Tokens::Value => "value",
                Tokens::None => "none",
                Tokens::SimpleQuote => "simple_quote",
                Tokens::DupleQuote => "duple_quote",
            };

            String::from(result)
        }
    }

    pub fn get_token(expression: String) -> Tokens {
        match expression.as_str() {
            "let" => Tokens::Let,
            "const" => Tokens::Const,
            "=" => Tokens::Identifier,
            "@" => Tokens::This,
            "&" => Tokens::Pointer,
            "(" => Tokens::LeftParenthesis,
            ")" => Tokens::RightParenthesis,
            "{" => Tokens::LeftBrace,
            "}" => Tokens::RightBrace,
            "[" => Tokens::LeftBracket,
            "]" => Tokens::RightBracket,
            "if" => Tokens::If,
            "elif" => Tokens::Elif,
            "else" => Tokens::Else,
            "try" => Tokens::Try,
            "catch" => Tokens::Catch,
            "finally" => Tokens::Finally,
            "throw" => Tokens::Throw,
            "fun" => Tokens::Fun,
            "return" => Tokens::Return,
            "async" => Tokens::Async,
            "await" => Tokens::Await,
            "loop" => Tokens::Loop,
            "do" => Tokens::Do,
            "match" => Tokens::Match,
            "struct" => Tokens::Struct,
            "model" => Tokens::Model,
            "+" => Tokens::Addition,
            "-" => Tokens::Subtraction,
            "/" => Tokens::Division,
            "*" => Tokens::Multiplication,
            "|" => Tokens::Modulus,
            "%" => Tokens::Percent,
            "eq" | "EQ" => Tokens::Equal,
            "neq" | "NEQ" => Tokens::NotEqual,
            "lt" | "LT" => Tokens::LessThan,
            "lte" | "LTE" => Tokens::LessThanOrEqual,
            "gt" | "GT" => Tokens::GreaterThan,
            "gte" | "GTE" => Tokens::GreaterThanOrEqual,
            "not" | "NOT" => Tokens::Not,
            "and" | "AND" => Tokens::And,
            "or" | "OR" => Tokens::Or,
            "xor" | "XOR" => Tokens::Xor,
            "," => Tokens::Comma,
            ";" => Tokens::Semicolon,
            ":" => Tokens::Colon,
            "." => Tokens::Dot,
            "#" => Tokens::CommentLine,
            "##" => Tokens::CommentMulti,
            "None" => Tokens::None,
            "\"" => Tokens::DupleQuote,
            "'" => Tokens::SimpleQuote,
            _ => {
                if is_any_value(&expression) {
                    Tokens::Value
                } else if is_reference(&expression) {
                    Tokens::Reference
                } else {
                    Tokens::None
                }
            }
        }
    }

    pub fn get_literal(tk: Tokens) -> String {
        tk.literal()
    }

    pub fn get_id(tk: Tokens) -> String {
        tk.id()
    }
}

pub use tokens::*;
