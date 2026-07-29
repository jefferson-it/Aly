// src/compiler/sh_backend.rs
use crate::compiler::ast::{Expr, Stmt, Program, TemplatePart};

pub fn compile_to_sh(source: &str) -> Result<String, String> {
    let program = crate::compiler::parser::parse_program(source);
    let mut sh_code = String::new();
    
    sh_code.push_str("#!/bin/bash\n");
    sh_code.push_str("# Generated from Aly source\n\n");
    
    let transpiler = ShTranspiler;
    for stmt in &program.stmts {
        let stmt_code = transpiler.transpile_stmt(stmt, 0)?;
        sh_code.push_str(&stmt_code);
    }
    
    Ok(sh_code)
}

struct ShTranspiler;

impl ShTranspiler {
    fn transpile_stmt(&self, stmt: &Stmt, indent: usize) -> Result<String, String> {
        let ind = "  ".repeat(indent);
        match stmt {
            Stmt::Let { name, init, .. } => {
                if let Some(val) = init {
                    let expr_code = self.transpile_expr(val)?;
                    Ok(format!("{}{}={}\n", ind, name, expr_code))
                } else {
                    Ok(format!("{}{}=\"\"\n", ind, name))
                }
            }
            Stmt::Const { name, init, .. } => {
                let expr_code = self.transpile_expr(init)?;
                Ok(format!("{}readonly {}={}\n", ind, name, expr_code))
            }
            Stmt::Assign { target, value } => {
                let target_code = self.transpile_expr(target)?;
                let value_code = self.transpile_expr(value)?;
                Ok(format!("{}{}={}\n", ind, target_code, value_code))
            }
            Stmt::Expr(expr) => {
                let expr_code = self.transpile_expr(expr)?;
                Ok(format!("{}{}\n", ind, expr_code))
            }
            Stmt::Return(opt_expr) => {
                if let Some(expr) = opt_expr {
                    let expr_code = self.transpile_expr(expr)?;
                    Ok(format!("{}exit {}\n", ind, expr_code))
                } else {
                    Ok(format!("{}exit 0\n", ind))
                }
            }
            Stmt::If { condition, then_body, elifs, else_body } => {
                let cond_code = self.transpile_expr(condition)?;
                let mut code = format!("{}if [ {} ]; then\n", ind, cond_code);
                for s in then_body {
                    code.push_str(&self.transpile_stmt(s, indent + 1)?);
                }
                
                for (elif_cond, elif_body) in elifs {
                    let elif_cond_code = self.transpile_expr(elif_cond)?;
                    code.push_str(&format!("{}elif [ {} ]; then\n", ind, elif_cond_code));
                    for s in elif_body {
                        code.push_str(&self.transpile_stmt(s, indent + 1)?);
                    }
                }
                
                if let Some(else_stmts) = else_body {
                    code.push_str(&format!("{}else\n", ind));
                    for s in else_stmts {
                        code.push_str(&self.transpile_stmt(s, indent + 1)?);
                    }
                }
                code.push_str(&format!("{}fi\n", ind));
                Ok(code)
            }
            Stmt::Fun { name, params, body, .. } => {
                let mut code = format!("{}{} () {{\n", ind, name);
                // Bind positional parameters
                for (i, param) in params.iter().enumerate() {
                    code.push_str(&format!("{}  local {}={}\n", ind, param, format!("${}", i + 1)));
                }
                for s in body {
                    code.push_str(&self.transpile_stmt(s, indent + 1)?);
                }
                code.push_str(&format!("{}}}\n", ind));
                Ok(code)
            }
            _ => Ok(format!("{}# stmt not supported in bash transpile\n", ind)),
        }
    }

    fn transpile_expr(&self, expr: &Expr) -> Result<String, String> {
        match expr {
            Expr::Int(val) => Ok(val.to_string()),
            Expr::Float(val) => Ok(val.to_string()),
            Expr::Char(val) => Ok(format!("\"{}\"", val)),
            Expr::Str(val) => {
                let escaped = val.replace("\"", "\\\"").replace("\n", "\\n");
                Ok(format!("\"{}\"", escaped))
            }
            Expr::Bool(val) => Ok(if *val { "1" } else { "0" }.to_string()),
            Expr::None => Ok("\"\"".to_string()),
            Expr::Void => Ok("\"\"".to_string()),
            Expr::Var(name) => Ok(format!("${}", name)),
            Expr::TemplateStr(parts) => {
                let mut out = String::from("\"");
                for part in parts {
                    match part {
                        TemplatePart::Text(t) => out.push_str(&t.replace("\"", "\\\"")),
                        TemplatePart::Var(v) => out.push_str(&format!("${}", v)),
                    }
                }
                out.push('"');
                Ok(out)
            }
            Expr::UnaryOp { op, expr } => {
                let expr_code = self.transpile_expr(expr)?;
                Ok(format!("! {}", expr_code))
            }
            Expr::BinOp { left, op, right } => {
                let left_code = self.transpile_expr(left)?;
                let right_code = self.transpile_expr(right)?;
                let sh_op = match op.as_str() {
                    "eq" | "EQ" | "==" => "-eq",
                    "neq" | "NEQ" | "!=" => "-ne",
                    "lt" | "LT" | "<" => "-lt",
                    "gt" | "GT" | ">" => "-gt",
                    "lte" | "LTE" | "<=" => "-le",
                    "gte" | "GTE" | ">=" => "-ge",
                    _ => op.as_str(),
                };
                Ok(format!("$(( {} {} {} ))", left_code, sh_op, right_code))
            }
            Expr::Call { function, args } => {
                let mut arg_codes = Vec::new();
                for arg in args {
                    arg_codes.push(self.transpile_expr(arg)?);
                }
                if let Expr::Var(name) = &**function {
                    if name == "print" {
                        return Ok(format!("echo {}", arg_codes.join(" ")));
                    }
                }
                let func_code = self.transpile_expr(function)?;
                Ok(format!("{} {}", func_code, arg_codes.join(" ")))
            }
            _ => Ok("\"\"".to_string()),
        }
    }
}
