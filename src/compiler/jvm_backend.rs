// src/compiler/jvm_backend.rs
use crate::compiler::ast::{Expr, Stmt, Program, TemplatePart};

pub fn compile_to_jvm(source: &str) -> Result<String, String> {
    let program = crate::compiler::parser::parse_program(source);
    let mut java_code = String::new();
    
    java_code.push_str("// Generated from Aly source\n");
    java_code.push_str("public class Main {\n");
    java_code.push_str("    pub static Object None = null;\n\n");
    java_code.push_str("    public static void main(String[] args) {\n");
    
    let transpiler = JavaTranspiler;
    for stmt in &program.stmts {
        let stmt_code = transpiler.transpile_stmt(stmt, 2)?;
        java_code.push_str(&stmt_code);
    }
    
    java_code.push_str("    }\n");
    java_code.push_str("}\n");
    
    Ok(java_code)
}

struct JavaTranspiler;

impl JavaTranspiler {
    fn transpile_stmt(&self, stmt: &Stmt, indent: usize) -> Result<String, String> {
        let ind = "    ".repeat(indent);
        match stmt {
            Stmt::Let { name, init, .. } => {
                if let Some(val) = init {
                    let expr_code = self.transpile_expr(val)?;
                    Ok(format!("{}Object {} = {};\n", ind, name, expr_code))
                } else {
                    Ok(format!("{}Object {} = null;\n", ind, name))
                }
            }
            Stmt::Const { name, init, .. } => {
                let expr_code = self.transpile_expr(init)?;
                Ok(format!("{}final Object {} = {};\n", ind, name, expr_code))
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
                let mut code = format!("{}if (Boolean.TRUE.equals({})) {{\n", ind, cond_code);
                for s in then_body {
                    code.push_str(&self.transpile_stmt(s, indent + 1)?);
                }
                code.push_str(&format!("{}}}", ind));
                
                for (elif_cond, elif_body) in elifs {
                    let elif_cond_code = self.transpile_expr(elif_cond)?;
                    code.push_str(&format!(" else if (Boolean.TRUE.equals({})) {{\n", elif_cond_code));
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
                // Java lambda function syntax
                let mut code = format!("{}java.util.function.Function<Object, Object> {} = ({}) -> {{\n", ind, name, params.join(", "));
                for s in body {
                    code.push_str(&self.transpile_stmt(s, indent + 1)?);
                }
                code.push_str(&format!("{}}};\n", ind));
                Ok(code)
            }
            _ => Ok(format!("{}// stmt not supported in Java/JVM transpile\n", ind)),
        }
    }

    fn transpile_expr(&self, expr: &Expr) -> Result<String, String> {
        match expr {
            Expr::Int(val) => Ok(format!("(Object){}L", val)),
            Expr::Float(val) => Ok(format!("(Object){}D", val)),
            Expr::Char(val) => Ok(format!("(Object)'{}'", val)),
            Expr::Str(val) => {
                let escaped = val.replace("\"", "\\\"").replace("\n", "\\n");
                Ok(format!("(Object)\"{}\"", escaped))
            }
            Expr::Bool(val) => Ok(if *val { "Boolean.TRUE" } else { "Boolean.FALSE" }.to_string()),
            Expr::None => Ok("null".to_string()),
            Expr::Void => Ok("null".to_string()),
            Expr::Var(name) => Ok(name.clone()),
            Expr::TemplateStr(parts) => {
                let mut out = String::from("String.format(\"");
                let mut java_format = String::new();
                let mut java_args = Vec::new();
                for part in parts {
                    match part {
                        TemplatePart::Text(t) => java_format.push_str(&t.replace("%", "%%").replace("\"", "\\\"")),
                        TemplatePart::Var(v) => {
                            java_format.push_str("%s");
                            java_args.push(v.clone());
                        }
                    }
                }
                out.push_str(&java_format);
                out.push_str("\"");
                for arg in java_args {
                    out.push_str(&format!(", {}", arg));
                }
                out.push(')');
                Ok(out)
            }
            Expr::UnaryOp { op, expr } => {
                let expr_code = self.transpile_expr(expr)?;
                let java_op = match op.as_str() {
                    "not" | "NOT" | "!" => "!",
                    o => o,
                };
                Ok(format!("({}{})", java_op, expr_code))
            }
            Expr::BinOp { left, op, right } => {
                let left_code = self.transpile_expr(left)?;
                let right_code = self.transpile_expr(right)?;
                // Handle basic math on Object (requires cast to long or double, for simple transpile we treat as Long)
                let java_op = match op.as_str() {
                    "eq" | "EQ" | "==" => "==",
                    "neq" | "NEQ" | "!=" => "!=",
                    "lt" | "LT" | "<" => "<",
                    "gt" | "GT" | ">" => ">",
                    "lte" | "LTE" | "<=" => "<=",
                    "gte" | "GTE" | ">=" => ">=",
                    o => o,
                };
                if ["+", "-", "*", "/", "%"].contains(&java_op) {
                    Ok(format!("(((Long){}) {} ((Long){}))", left_code, java_op, right_code))
                } else {
                    Ok(format!("({} {} {})", left_code, java_op, right_code))
                }
            }
            Expr::Call { function, args } => {
                let mut arg_codes = Vec::new();
                for arg in args {
                    arg_codes.push(self.transpile_expr(arg)?);
                }
                if let Expr::Var(name) = &**function {
                    if name == "print" {
                        return Ok(format!("System.out.println({})", arg_codes.join(" + \" \" + ")));
                    }
                }
                let func_code = self.transpile_expr(function)?;
                Ok(format!("{}.apply({})", func_code, arg_codes.join(", ")))
            }
            _ => Ok("null".to_string()),
        }
    }
}
