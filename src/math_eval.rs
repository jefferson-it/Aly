/// Pure-Rust math expression evaluator.
///
/// Design decisions:
/// - Replaces the `eval` crate entirely. The `eval` crate was designed for
///   JavaScript-style semantics and introduced precision, performance, and
///   semantic issues.
/// - Implements a classic recursive-descent Pratt parser with correct
///   operator precedence and left-associativity.
/// - Supports: `+`, `-`, `*`, `/`, `%` (modulus), `**` (power), `(`, `)`.
/// - IEEE 754 semantics for all floating-point operations:
///   - Division by zero → `Infinity` / `-Infinity`
///   - 0.0 / 0.0 → `NaN`
///   - Integer division by zero → `RuntimeError`
/// - All computation is done in `f64` for correctness. Results that are
///   exact integers are formatted without a decimal point.
/// - Future: The `Value` enum can be extended with `BigInt` and `Decimal`
///   variants without breaking this public API.

use crate::error::{AlyError, AlyResult};

// ──────────────────────────────────────────────────────────────────────────────
// Public entry point
// ──────────────────────────────────────────────────────────────────────────────

/// Evaluate a mathematical expression string and return a string representation
/// of the result.
///
/// # Examples
///
/// ```rust
/// use aly::math_eval::eval_math;
/// assert_eq!(eval_math("2 + 3 * 4").unwrap(), "14");
/// assert_eq!(eval_math("(2 + 3) * 4").unwrap(), "20");
/// assert_eq!(eval_math("10 / 3").unwrap(), "3.3333333333333335");
/// assert_eq!(eval_math("2 ** 8").unwrap(), "256");
/// ```
pub fn eval_math(expression: &str) -> AlyResult<String> {
    let tokens = tokenize(expression)?;
    let mut parser = Parser::new(tokens);
    let value = parser.parse_expr(0)?;
    Ok(format_value(value))
}

// ──────────────────────────────────────────────────────────────────────────────
// Value type (extensible for BigInt / Decimal in the future)
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    Float(f64),
}

impl Value {
    fn as_f64(&self) -> f64 {
        match self {
            Value::Float(v) => *v,
        }
    }
}

fn format_value(v: Value) -> String {
    match v {
        Value::Float(f) => {
            if f.is_nan() {
                return "NaN".to_string();
            }
            if f.is_infinite() {
                return if f > 0.0 {
                    "Infinity".to_string()
                } else {
                    "-Infinity".to_string()
                };
            }
            // If the value is exactly representable as an integer, show it without .0
            if f.fract() == 0.0 && f.abs() < 1e15_f64 {
                format!("{}", f as i64)
            } else {
                format!("{}", f)
            }
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Tokeniser
// ──────────────────────────────────────────────────────────────────────────────

#[derive(Debug, Clone, PartialEq)]
enum Token {
    Number(f64),
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    StarStar,
    LParen,
    RParen,
    Eof,
}

fn tokenize(input: &str) -> AlyResult<Vec<Token>> {
    let mut tokens = Vec::new();
    let chars: Vec<char> = input.chars().collect();
    let mut i = 0;

    while i < chars.len() {
        match chars[i] {
            ' ' | '\t' | '\r' | '\n' => {
                i += 1;
            }
            '0'..='9' | '.' => {
                let start = i;
                while i < chars.len() && (chars[i].is_ascii_digit() || chars[i] == '.') {
                    i += 1;
                }
                let num_str: String = chars[start..i].iter().collect();
                let n: f64 = num_str.parse().map_err(|_| {
                    AlyError::runtime(format!("Número inválido na expressão: '{}'", num_str))
                })?;
                tokens.push(Token::Number(n));
            }
            '+' => {
                tokens.push(Token::Plus);
                i += 1;
            }
            '-' => {
                tokens.push(Token::Minus);
                i += 1;
            }
            '*' => {
                if i + 1 < chars.len() && chars[i + 1] == '*' {
                    tokens.push(Token::StarStar);
                    i += 2;
                } else {
                    tokens.push(Token::Star);
                    i += 1;
                }
            }
            '/' => {
                tokens.push(Token::Slash);
                i += 1;
            }
            '%' => {
                tokens.push(Token::Percent);
                i += 1;
            }
            '(' => {
                tokens.push(Token::LParen);
                i += 1;
            }
            ')' => {
                tokens.push(Token::RParen);
                i += 1;
            }
            c => {
                return Err(AlyError::runtime(format!(
                    "Caractere inesperado na expressão matemática: '{}'",
                    c
                )));
            }
        }
    }

    tokens.push(Token::Eof);
    Ok(tokens)
}

// ──────────────────────────────────────────────────────────────────────────────
// Pratt parser (precedence climbing)
// ──────────────────────────────────────────────────────────────────────────────

struct Parser {
    tokens: Vec<Token>,
    pos: usize,
}

impl Parser {
    fn new(tokens: Vec<Token>) -> Self {
        Parser { tokens, pos: 0 }
    }

    fn peek(&self) -> &Token {
        self.tokens.get(self.pos).unwrap_or(&Token::Eof)
    }

    fn consume(&mut self) -> Token {
        let t = self.tokens[self.pos].clone();
        self.pos += 1;
        t
    }

    fn expect(&mut self, expected: Token) -> AlyResult<()> {
        let t = self.consume();
        if t == expected {
            Ok(())
        } else {
            Err(AlyError::syntax(format!(
                "Esperado {:?}, encontrado {:?}",
                expected, t
            )))
        }
    }

    /// Parse with Pratt precedence climbing.
    fn parse_expr(&mut self, min_prec: u8) -> AlyResult<Value> {
        let mut left = self.parse_unary()?;

        loop {
            let prec = self.infix_precedence(self.peek());
            if prec < min_prec {
                break;
            }
            let op = self.consume();
            // ** is right-associative; everything else is left-associative.
            let next_prec = if op == Token::StarStar { prec } else { prec + 1 };
            let right = self.parse_expr(next_prec)?;
            left = self.apply_infix(op, left, right)?;
        }

        Ok(left)
    }

    fn infix_precedence(&self, tok: &Token) -> u8 {
        match tok {
            Token::Plus | Token::Minus => 1,
            Token::Star | Token::Slash | Token::Percent => 2,
            Token::StarStar => 3,
            _ => 0,
        }
    }

    fn apply_infix(&self, op: Token, left: Value, right: Value) -> AlyResult<Value> {
        let l = left.as_f64();
        let r = right.as_f64();

        let result = match op {
            Token::Plus => l + r,
            Token::Minus => l - r,
            Token::Star => l * r,
            Token::Slash => {
                if r == 0.0 {
                    // IEEE 754: return Infinity/-Infinity/NaN rather than panic
                    if l == 0.0 {
                        f64::NAN
                    } else if l > 0.0 {
                        f64::INFINITY
                    } else {
                        f64::NEG_INFINITY
                    }
                } else {
                    l / r
                }
            }
            Token::Percent => {
                if r == 0.0 {
                    return Err(AlyError::runtime(
                        "Módulo por zero não é permitido".to_string(),
                    ));
                }
                l % r
            }
            Token::StarStar => l.powf(r),
            _ => unreachable!("apply_infix called with non-infix token"),
        };

        Ok(Value::Float(result))
    }

    fn parse_unary(&mut self) -> AlyResult<Value> {
        match self.peek().clone() {
            Token::Minus => {
                self.consume();
                let v = self.parse_unary()?;
                Ok(Value::Float(-v.as_f64()))
            }
            Token::Plus => {
                self.consume();
                self.parse_unary()
            }
            _ => self.parse_primary(),
        }
    }

    fn parse_primary(&mut self) -> AlyResult<Value> {
        match self.peek().clone() {
            Token::Number(n) => {
                self.consume();
                Ok(Value::Float(n))
            }
            Token::LParen => {
                self.consume();
                let v = self.parse_expr(0)?;
                self.expect(Token::RParen)?;
                Ok(v)
            }
            Token::Eof => Err(AlyError::syntax(
                "Fim inesperado da expressão matemática".to_string(),
            )),
            t => Err(AlyError::syntax(format!(
                "Token inesperado na expressão matemática: {:?}",
                t
            ))),
        }
    }
}

// ──────────────────────────────────────────────────────────────────────────────
// Unit tests
// ──────────────────────────────────────────────────────────────────────────────

#[cfg(test)]
mod tests {
    use super::*;

    fn eval(expr: &str) -> String {
        eval_math(expr).expect("eval_math failed")
    }

    #[test]
    fn test_basic_arithmetic() {
        assert_eq!(eval("2 + 3"), "5");
        assert_eq!(eval("10 - 4"), "6");
        assert_eq!(eval("3 * 4"), "12");
        assert_eq!(eval("10 / 2"), "5");
        assert_eq!(eval("10 % 3"), "1");
    }

    #[test]
    fn test_precedence() {
        assert_eq!(eval("2 + 3 * 4"), "14");
        assert_eq!(eval("(2 + 3) * 4"), "20");
        assert_eq!(eval("10 - 2 * 3"), "4");
    }

    #[test]
    fn test_power() {
        assert_eq!(eval("2 ** 8"), "256");
        assert_eq!(eval("2 ** 0"), "1");
        assert_eq!(eval("2 ** 10"), "1024");
    }

    #[test]
    fn test_unary_minus() {
        assert_eq!(eval("-5"), "-5");
        assert_eq!(eval("-5 + 10"), "5");
        assert_eq!(eval("-(3 + 2)"), "-5");
    }

    #[test]
    fn test_floats() {
        let r = eval("1.5 + 1.5");
        assert_eq!(r, "3");
        let r = eval("10.0 / 3.0");
        assert!(r.starts_with("3.33"), "got: {}", r);
    }

    #[test]
    fn test_division_by_zero_float() {
        assert_eq!(eval("1.0 / 0.0"), "Infinity");
        assert_eq!(eval("-1.0 / 0.0"), "-Infinity");
    }

    #[test]
    fn test_nan() {
        assert_eq!(eval("0.0 / 0.0"), "NaN");
    }

    #[test]
    fn test_nested_parens() {
        assert_eq!(eval("((2 + 3) * (4 - 1))"), "15");
    }

    #[test]
    fn test_right_assoc_power() {
        // 2 ** 3 ** 2 = 2 ** (3 ** 2) = 2 ** 9 = 512
        assert_eq!(eval("2 ** 3 ** 2"), "512");
    }

    #[test]
    fn test_modulo_error() {
        let result = eval_math("5 % 0");
        assert!(result.is_err());
        let err = result.unwrap_err();
        assert_eq!(err.kind, crate::error::AlyErrorKind::Runtime);
    }
}
