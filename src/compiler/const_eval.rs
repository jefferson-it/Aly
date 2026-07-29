use crate::compiler::ast::*;
use crate::vm::value::Value;
use std::cell::RefCell;
use std::rc::Rc;

pub struct ConstEvaluator;

impl ConstEvaluator {
    pub fn new() -> Self {
        ConstEvaluator
    }

    /// Evaluate a constant expression at compile time.
    /// Returns Some(Value) if the expression is a compile-time constant.
    pub fn eval_expr(expr: &Expr) -> Option<Value> {
        match expr {
            Expr::Int(n) => Some(Value::Int(*n)),
            Expr::Float(f) => Some(Value::Float(*f)),
            Expr::Char(c) => Some(Value::Char(*c)),
            Expr::Str(s) => Some(Value::Str(s.clone())),
            Expr::Bool(b) => Some(Value::Bool(*b)),
            Expr::None => Some(Value::Nil),
            Expr::Void => Some(Value::Void),
            Expr::Var(_name) => {
                // Cannot resolve variables at compile time without scope
                None
            }
            Expr::UnaryOp { op, expr } => {
                let val = Self::eval_expr(expr)?;
                match op.as_str() {
                    "-" => match val {
                        Value::Int(n) => Some(Value::Int(-n)),
                        Value::Float(f) => Some(Value::Float(-f)),
                        _ => None,
                    },
                    "!" | "not" | "NOT" => Some(Value::Bool(!matches!(val, Value::Bool(false) | Value::Nil))),
                    _ => None,
                }
            }
            Expr::BinOp { left, op, right } => {
                let l = Self::eval_expr(left)?;
                let r = Self::eval_expr(right)?;
                match op.as_str() {
                    "+" => match (l, r) {
                        (Value::Int(a), Value::Int(b)) => Some(Value::Int(a + b)),
                        (Value::Float(a), Value::Float(b)) => Some(Value::Float(a + b)),
                        (Value::Str(a), Value::Str(b)) => Some(Value::Str(format!("{}{}", a, b))),
                        _ => None,
                    },
                    "-" => match (l, r) {
                        (Value::Int(a), Value::Int(b)) => Some(Value::Int(a - b)),
                        (Value::Float(a), Value::Float(b)) => Some(Value::Float(a - b)),
                        _ => None,
                    },
                    "*" => match (l, r) {
                        (Value::Int(a), Value::Int(b)) => Some(Value::Int(a * b)),
                        (Value::Float(a), Value::Float(b)) => Some(Value::Float(a * b)),
                        _ => None,
                    },
                    "/" => match (l, r) {
                        (Value::Int(a), Value::Int(b)) if b != 0 => Some(Value::Int(a / b)),
                        (Value::Float(a), Value::Float(b)) if b != 0.0 => Some(Value::Float(a / b)),
                        _ => None,
                    },
                    "%" | "|" => match (l, r) {
                        (Value::Int(a), Value::Int(b)) if b != 0 => Some(Value::Int(a % b)),
                        _ => None,
                    },
                    "eq" | "EQ" | "==" => Some(Value::Bool(l == r)),
                    "neq" | "NEQ" | "!=" => Some(Value::Bool(l != r)),
                    "lt" | "LT" => match (&l, &r) {
                        (Value::Int(a), Value::Int(b)) => Some(Value::Bool(a < b)),
                        (Value::Float(a), Value::Float(b)) => Some(Value::Bool(a < b)),
                        _ => None,
                    },
                    "gt" | "GT" => match (&l, &r) {
                        (Value::Int(a), Value::Int(b)) => Some(Value::Bool(a > b)),
                        (Value::Float(a), Value::Float(b)) => Some(Value::Bool(a > b)),
                        _ => None,
                    },
                    "and" | "AND" => {
                        let lb = matches!(l, Value::Bool(true));
                        if !lb { Some(Value::Bool(false)) }
                        else { Some(r) }
                    }
                    "or" | "OR" => {
                        let lb = matches!(l, Value::Bool(true));
                        if lb { Some(l) }
                        else { Some(r) }
                    }
                    "." => match (l, r) {
                        (Value::Str(a), Value::Str(b)) => Some(Value::Str(format!("{}{}", a, b))),
                        _ => None,
                    },
                    _ => None,
                }
            }
            Expr::Array(items) => {
                let mut vals = Vec::new();
                for item in items {
                    vals.push(Self::eval_expr(item)?);
                }
                Some(Value::Vec(Rc::new(RefCell::new(vals))))
            }
            Expr::Tuple(items) => {
                let mut vals = Vec::new();
                for item in items {
                    vals.push(Self::eval_expr(item)?);
                }
                Some(Value::Tuple(vals))
            }
            _ => None,
        }
    }

    /// Evaluate all const declarations in a program and return a value map.
    pub fn eval_consts(stmts: &[Stmt]) -> std::collections::HashMap<String, Value> {
        let mut consts = std::collections::HashMap::new();
        for stmt in stmts {
            if let Stmt::Const { name, init, .. } = stmt {
                if let Some(val) = Self::eval_expr(init) {
                    consts.insert(name.clone(), val);
                }
            }
        }
        consts
    }
}
