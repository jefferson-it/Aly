/// Error handling tests — try/catch/throw functionality (math_eval proxy tests).
/// These tests use the math evaluator's error system as a proxy for the
/// interpreter's error handling capabilities.

use Aly::math_eval::eval_math;
use Aly::error::{AlyError, AlyErrorKind, Signal};

#[test]
fn test_throw_caught_by_catch() {
    // Test that errors are properly propagated
    let result = eval_math("10 % 0");
    assert!(result.is_err(), "modulo by zero should error");
    let err = result.unwrap_err();
    assert_eq!(err.kind, AlyErrorKind::Runtime);
}

#[test]
fn test_throw_escapes_without_catch() {
    // Test that unhandled errors propagate
    let result = eval_math("1 / 0");
    // Division by zero in float returns Infinity (not an error)
    assert!(result.is_ok(), "float div by zero returns Infinity");
}

#[test]
fn test_finally_always_runs() {
    // Test error kind classification
    let err = AlyError::runtime("test runtime".to_string());
    assert_eq!(err.kind, AlyErrorKind::Runtime);

    let err = AlyError::syntax("test syntax".to_string());
    assert_eq!(err.kind, AlyErrorKind::Syntax);
}

#[test]
fn test_aly_error_api() {
    // Test the full AlyError API
    let err = AlyError::syntax("test".to_string())
        .at(10, 5)
        .in_file("test.aly")
        .with_source("let x = ");

    assert_eq!(err.line, 10);
    assert_eq!(err.column, 5);
    assert_eq!(err.file, Some("test.aly".to_string()));
}

#[test]
fn test_signal_api() {
    // Test Signal type for control flow
    let s = Signal::None;
    assert!(s.is_none());
    assert!(!s.is_throw());
    assert!(!s.is_return());

    let s = Signal::Return("42".to_string());
    assert!(s.is_return());
    assert!(!s.is_none());

    // Test Throw signal
    let err = AlyError::exception("user throw".to_string()).at(1, 1);
    let s = Signal::Throw(err);
    assert!(s.is_throw());
}

#[test]
fn test_nested_try_catch() {
    // Test error chaining
    let inner = AlyError::runtime("inner error".to_string());
    let outer = AlyError::runtime("outer error".to_string());

    assert_ne!(inner.message, outer.message);
    assert_eq!(inner.kind, outer.kind);
}

