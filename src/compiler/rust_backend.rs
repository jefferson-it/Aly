// src/compiler/rust_backend.rs
use crate::compiler::ast::{Expr, Stmt, Program, TemplatePart};

pub fn compile_to_rust(source: &str) -> Result<String, String> {
    let program = crate::compiler::parser::parse_program(source);
    let mut rust_code = String::new();
    
    rust_code.push_str("// Generated from Aly source\n");
    rust_code.push_str("fn main() {\n");
    
    let transpiler = RustTranspiler;
    for stmt in &program.stmts {
        let stmt_code = transpiler.transpile_stmt(stmt, 1)?;
        rust_code.push_str(&stmt_code);
    }
    
    rust_code.push_str("}\n");
    
    Ok(rust_code)
}

struct RustTranspiler;

impl RustTranspiler {
    fn transpile_stmt(&self, stmt: &Stmt, indent: usize) -> Result<String, String> {
        let ind = "    ".repeat(indent);
        match stmt {
            Stmt::Let { name, init, .. } => {
                if let Some(val) = init {
                    let expr_code = self.transpile_expr(val)?;
                    Ok(format!("{}let mut {} = {};\n", ind, name, expr_code))
                } else {
                    Ok(format!("{}let mut {}: Option<i32> = None;\n", ind, name))
                }
            }
            Stmt::Const { name, init, .. } => {
                let expr_code = self.transpile_expr(init)?;
                Ok(format!("{}const {}: i64 = {};\n", ind, name, expr_code))
            }
            Stmt::Assign { target, value } => {
                let target_code = self.transpile_expr(target)?;
                let value_code = self.transpile_expr(value)?;
                Ok(format!("{}{} = {};\n", ind, target_code, value_code))
            }
            Stmt::Expr(expr) => {
                let expr_code = self.transpile_expr(expr)?;
                Ok(format!("{}{};\n", ind, expr_code))
            }
            Stmt::Return(opt_expr) => {
                if let Some(expr) = opt_expr {
                    let expr_code = self.transpile_expr(expr)?;
                    Ok(format!("{}return {};\n", ind, expr_code))
                } else {
                    Ok(format!("{}return;\n", ind))
                }
            }
            Stmt::If { condition, then_body, elifs, else_body } => {
                let cond_code = self.transpile_expr(condition)?;
                let mut code = format!("{}if {} {{\n", ind, cond_code);
                for s in then_body {
                    code.push_str(&self.transpile_stmt(s, indent + 1)?);
                }
                code.push_str(&format!("{}}}", ind));
                
                for (elif_cond, elif_body) in elifs {
                    let elif_cond_code = self.transpile_expr(elif_cond)?;
                    code.push_str(&format!(" else if {} {{\n", elif_cond_code));
                    for s in elif_body {
                        code.push_str(&self.transpile_stmt(s, indent + 1)?);
                    }
                    code.push_str(&format!("{}}}", ind));
                }
                
                if let Some(else_stmts) = else_body {
                    code.push_str(" else {\n");
                    for s in else_stmts {
                        code.push_str(&self.transpile_stmt(s, indent + 1)?);
                    }
                    code.push_str(&format!("{}}}\n", ind));
                } else {
                    code.push_str("\n");
                }
                Ok(code)
            }
            Stmt::Fun { name, params, body, .. } => {
                let mut code = format!("{}let {} = |{}| {{\n", ind, name, params.join(", "));
                for s in body {
                    code.push_str(&self.transpile_stmt(s, indent + 1)?);
                }
                code.push_str(&format!("{}}};\n", ind));
                Ok(code)
            }
            _ => Ok(format!("{}// stmt not supported in Rust transpile\n", ind)),
        }
    }

    fn transpile_expr(&self, expr: &Expr) -> Result<String, String> {
        match expr {
            Expr::Int(val) => Ok(val.to_string()),
            Expr::Float(val) => Ok(val.to_string()),
            Expr::Char(val) => Ok(format!("'{}'", val)),
            Expr::Str(val) => {
                let escaped = val.replace("\"", "\\\"").replace("\n", "\\n");
                Ok(format!("\"{}\"", escaped))
            }
            Expr::Bool(val) => Ok(val.to_string()),
            Expr::None => Ok("None".to_string()),
            Expr::Void => Ok("()".to_string()),
            Expr::Var(name) => Ok(name.clone()),
            Expr::TemplateStr(parts) => {
                let mut format_str = String::new();
                let mut format_args = Vec::new();
                for part in parts {
                    match part {
                        TemplatePart::Text(t) => format_str.push_str(&t.replace("{", "{{").replace("}", "}}")),
                        TemplatePart::Var(v) => {
                            format_str.push_str("{}");
                            format_args.push(v.clone());
                        }
                    }
                }
                if format_args.is_empty() {
                    Ok(format!("\"{}\"", format_str))
                } else {
                    Ok(format!("format!(\"{}\", {})", format_str, format_args.join(", ")))
                }
            }
            Expr::UnaryOp { op, expr } => {
                let expr_code = self.transpile_expr(expr)?;
                let rust_op = match op.as_str() {
                    "not" | "NOT" | "!" => "!",
                    o => o,
                };
                Ok(format!("({}{})", rust_op, expr_code))
            }
            Expr::BinOp { left, op, right } => {
                let left_code = self.transpile_expr(left)?;
                let right_code = self.transpile_expr(right)?;
                let rust_op = match op.as_str() {
                    "eq" | "EQ" => "==",
                    "neq" | "NEQ" => "!=",
                    "lt" | "LT" => "<",
                    "gt" | "GT" => ">",
                    "lte" | "LTE" => "<=",
                    "gte" | "GTE" => ">=",
                    "and" | "AND" => "&&",
                    "or" | "OR" => "||",
                    o => o,
                };
                Ok(format!("({} {} {})", left_code, rust_op, right_code))
            }
            Expr::Call { function, args } => {
                let mut arg_codes = Vec::new();
                for arg in args {
                    arg_codes.push(self.transpile_expr(arg)?);
                }
                if let Expr::Var(name) = &**function {
                    if name == "print" {
                        return Ok(format!("println!(\"{{:?}}\", {})", arg_codes.join(", ")));
                    }
                }
                let func_code = self.transpile_expr(function)?;
                Ok(format!("{}({})", func_code, arg_codes.join(", ")))
            }
            Expr::Array(items) => {
                let mut item_codes = Vec::new();
                for item in items {
                    item_codes.push(self.transpile_expr(item)?);
                }
                Ok(format!("vec![{}]", item_codes.join(", ")))
            }
            _ => Ok("None".to_string()),
        }
    }
}
