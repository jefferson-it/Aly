use std::collections::HashMap;

use crate::compiler::ast::*;

pub struct MetaCompiler {
    macros: HashMap<String, MacroDef>,
    generated: Vec<Stmt>,
    compile_time_fns: HashMap<String, CompileTimeFn>,
    eval_depth: usize,
}

struct MacroDef {
    params: Vec<String>,
    body: Vec<Stmt>,
    is_variadic: bool,
}

struct CompileTimeFn {
    params: Vec<String>,
    body: Vec<Stmt>,
}

impl MetaCompiler {
    pub fn new() -> Self {
        MetaCompiler {
            macros: HashMap::new(),
            generated: Vec::new(),
            compile_time_fns: HashMap::new(),
            eval_depth: 0,
        }
    }

    pub fn register_macro(&mut self, name: &str, params: &[String], body: Vec<Stmt>, variadic: bool) {
        self.macros.insert(name.to_string(), MacroDef {
            params: params.to_vec(),
            body,
            is_variadic: variadic,
        });
    }

    pub fn register_compile_time_fn(&mut self, name: &str, params: &[String], body: Vec<Stmt>) {
        self.compile_time_fns.insert(name.to_string(), CompileTimeFn {
            params: params.to_vec(),
            body,
        });
    }

    pub fn expand_macro(&self, name: &str, args: &[Expr]) -> Result<Vec<Stmt>, String> {
        let def = self.macros.get(name).ok_or_else(|| format!("Macro '{}' not found", name))?;
        if !def.is_variadic && args.len() != def.params.len() {
            return Err(format!("Macro '{}' expects {} arguments, got {}", name, def.params.len(), args.len()));
        }
        let mut subst = HashMap::new();
        for (i, param) in def.params.iter().enumerate() {
            if i < args.len() {
                subst.insert(param.clone(), args[i].clone());
            }
        }
        Ok(self.substitute_stmts(&def.body, &subst))
    }

    fn substitute_stmts(&self, stmts: &[Stmt], subst: &HashMap<String, Expr>) -> Vec<Stmt> {
        stmts.iter().map(|s| self.substitute_stmt(s, subst)).collect()
    }

    fn substitute_stmt(&self, stmt: &Stmt, subst: &HashMap<String, Expr>) -> Stmt {
        match stmt {
            Stmt::Let { name, init, attrs } => Stmt::Let {
                name: name.clone(),
                init: init.as_ref().map(|e| self.substitute_expr(e, subst)),
                attrs: attrs.clone(),
            },
            Stmt::Expr(e) => Stmt::Expr(self.substitute_expr(e, subst)),
            Stmt::Return(e) => Stmt::Return(e.as_ref().map(|e| self.substitute_expr(e, subst))),
            Stmt::If { condition, then_body, elifs, else_body } => Stmt::If {
                condition: self.substitute_expr(condition, subst),
                then_body: self.substitute_stmts(then_body, subst),
                elifs: elifs.iter().map(|(c, b)| (self.substitute_expr(c, subst), self.substitute_stmts(b, subst))).collect(),
                else_body: else_body.as_ref().map(|b| self.substitute_stmts(b, subst)),
            },
            Stmt::While { condition, body } => Stmt::While {
                condition: self.substitute_expr(condition, subst),
                body: self.substitute_stmts(body, subst),
            },
            Stmt::For { init, condition, update, body } => Stmt::For {
                init: init.as_ref().map(|i| Box::new(self.substitute_stmt(i, subst))),
                condition: condition.as_ref().map(|c| self.substitute_expr(c, subst)),
                update: update.as_ref().map(|u| self.substitute_expr(u, subst)),
                body: self.substitute_stmts(body, subst),
            },
            _ => stmt.clone(),
        }
    }

    fn substitute_expr(&self, expr: &Expr, subst: &HashMap<String, Expr>) -> Expr {
        match expr {
            Expr::Var(name) => subst.get(name).cloned().unwrap_or_else(|| Expr::Var(name.clone())),
            Expr::BinOp { left, op, right } => Expr::BinOp {
                left: Box::new(self.substitute_expr(left, subst)),
                op: op.clone(),
                right: Box::new(self.substitute_expr(right, subst)),
            },
            Expr::UnaryOp { op, expr: inner } => Expr::UnaryOp {
                op: op.clone(),
                expr: Box::new(self.substitute_expr(inner, subst)),
            },
            Expr::Call { function, args } => Expr::Call {
                function: Box::new(self.substitute_expr(function, subst)),
                args: args.iter().map(|a| self.substitute_expr(a, subst)).collect(),
            },
            _ => expr.clone(),
        }
    }

    pub fn expand_ast(&mut self, program: &Program) -> Program {
        let mut stmts = Vec::new();
        let mut i = 0;
        while i < program.stmts.len() {
            match &program.stmts[i] {
                Stmt::MacroDef { name, patterns, bodies } => {
                    let params: Vec<String> = patterns.first()
                        .map(|p| p.iter().filter_map(|e| if let Expr::Var(v) = e { Some(v.clone()) } else { None }).collect())
                        .unwrap_or_default();
                    let body = bodies.first().cloned().unwrap_or_default();
                    self.register_macro(name, &params, body, false);
                    stmts.push(program.stmts[i].clone());
                }
                Stmt::Expr(Expr::MacroExpand { name, args }) => {
                    match self.expand_macro(name, args) {
                        Ok(expanded) => stmts.extend(expanded),
                        Err(e) => eprintln!("Macro expansion error: {}", e),
                    }
                }
                _ => stmts.push(program.stmts[i].clone()),
            }
            i += 1;
        }
        Program { stmts }
    }

    pub fn eval_compile_time(&self, stmts: &[Stmt]) -> Vec<Stmt> {
        let mut result = Vec::new();
        for stmt in stmts {
            if let Stmt::Expr(Expr::Call { function, args }) = stmt {
                if let Expr::Var(name) = function.as_ref() {
                    if let Some(ct_fn) = self.compile_time_fns.get(name) {
                        if let Ok(expanded) = self.expand_macro(name, args) {
                            result.extend(expanded);
                            continue;
                        }
                    }
                }
            }
            result.push(stmt.clone());
        }
        result
    }

    // Generate getters/setters for a struct-like definition
    pub fn generate_accessors(&self, struct_name: &str, fields: &[&str]) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        for field in fields {
            let getter = Stmt::Fun {
                name: format!("get_{}", field),
                params: vec!["self".to_string()],
                defaults: vec![],
                variadic: None,
                body: vec![Stmt::Return(Some(Expr::PropAccess {
                    object: Box::new(Expr::Var("self".to_string())),
                    prop: field.to_string(),
                }))],
                attrs: vec![],
            };
            let setter = Stmt::Fun {
                name: format!("set_{}", field),
                params: vec!["self".to_string(), "value".to_string()],
                defaults: vec![],
                variadic: None,
                body: vec![Stmt::Expr(Expr::BinOp {
                    left: Box::new(Expr::PropAccess {
                        object: Box::new(Expr::Var("self".to_string())),
                        prop: field.to_string(),
                    }),
                    op: "=".to_string(),
                    right: Box::new(Expr::Var("value".to_string())),
                })],
                attrs: vec![],
            };
            stmts.push(getter);
            stmts.push(setter);
        }
        stmts
    }

    // Generate a builder pattern for a struct
    pub fn generate_builder(&self, struct_name: &str, fields: &[(&str, &str)]) -> Vec<Stmt> {
        let mut stmts = Vec::new();
        let builder_name = format!("{}Builder", struct_name);

        // Builder struct constructor
        let mut builder_body = Vec::new();
        for (field, default) in fields {
            builder_body.push(Stmt::Let {
                name: field.to_string(),
                init: Some(Expr::Str(default.to_string())),
                attrs: vec![],
            });
        }
        stmts.push(Stmt::Fun {
            name: format!("{}::new", builder_name),
            params: vec![],
            defaults: vec![],
            variadic: None,
            body: builder_body,
            attrs: vec![],
        });

        for (field, _) in fields {
            let set_body = vec![
                Stmt::Expr(Expr::BinOp {
                    left: Box::new(Expr::PropAccess {
                        object: Box::new(Expr::Var("self".to_string())),
                        prop: field.to_string(),
                    }),
                    op: "=".to_string(),
                    right: Box::new(Expr::Var("value".to_string())),
                }),
                Stmt::Return(Some(Expr::Var("self".to_string()))),
            ];
            stmts.push(Stmt::Fun {
                name: format!("{}::{}", builder_name, field),
                params: vec!["self".to_string(), "value".to_string()],
                defaults: vec![],
                variadic: None,
                body: set_body,
                attrs: vec![],
            });
        }

        // Build method
        let mut build_body = Vec::new();
        for (field, _) in fields {
            build_body.push(Stmt::Let {
                name: format!("{}_val", field),
                init: Some(Expr::PropAccess {
                    object: Box::new(Expr::Var("self".to_string())),
                    prop: field.to_string(),
                }),
                attrs: vec![],
            });
        }
        stmts.push(Stmt::Fun {
            name: format!("{}::build", builder_name),
            params: vec!["self".to_string()],
            defaults: vec![],
            variadic: None,
            body: build_body,
            attrs: vec![],
        });

        stmts
    }
}
