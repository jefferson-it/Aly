use crate::compiler::ast::*;

pub struct CodeGenerator {
    code: String,
    indent: usize,
    label_count: usize,
}

impl CodeGenerator {
    pub fn new() -> Self {
        CodeGenerator {
            code: String::new(),
            indent: 0,
            label_count: 0,
        }
    }

    fn indent_str(&self) -> String {
        "    ".repeat(self.indent)
    }

    fn emit(&mut self, s: &str) {
        self.code.push_str(s);
    }

    fn emit_line(&mut self, s: &str) {
        self.code.push_str(&self.indent_str());
        self.code.push_str(s);
        self.code.push('\n');
    }

    fn new_label(&mut self) -> String {
        let label = format!("label_{}", self.label_count);
        self.label_count += 1;
        label
    }

    pub fn generate(&mut self, program: &Program) -> String {
        self.emit("#include \"runtime_aly.h\"\n\n");

        let funs: Vec<_> = program
            .stmts
            .iter()
            .filter(|s| matches!(s, Stmt::Fun { .. }))
            .collect();
        let others: Vec<_> = program
            .stmts
            .iter()
            .filter(|s| !matches!(s, Stmt::Fun { .. }))
            .collect();

        for stmt in &funs {
            self.gen_stmt_forward(stmt);
        }

        self.emit("int main(int argc, char** argv) {\n");
        self.indent += 1;
        self.emit_line("aly_init(argc, argv);");

        for stmt in &others {
            self.gen_stmt(stmt);
        }

        self.emit_line("aly_cleanup();");
        self.indent -= 1;
        self.emit("    return 0;\n}\n\n");

        for stmt in &funs {
            self.gen_stmt(stmt);
        }

        self.code.clone()
    }

    fn gen_stmt_forward(&mut self, stmt: &Stmt) {
        if let Stmt::Fun { name, params, .. } = stmt {
            let c_params = if params.is_empty() {
                "void".to_string()
            } else {
                params
                    .iter()
                    .map(|p| format!("aly_value_t {p}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            };
            self.emit_line(&format!("aly_value_t {name}({c_params});"));
        }
    }

    fn gen_stmt(&mut self, stmt: &Stmt) {
        match stmt {
            Stmt::Let { name, init } => {
                match init {
                    Some(expr) => {
                        let c_expr = self.gen_expr(expr);
                        self.emit_line(&format!("aly_value_t {name} = {c_expr};"));
                    }
                    None => {
                        self.emit_line(&format!("aly_value_t {name} = aly_none();"));
                    }
                }
            }
            Stmt::Const { name, init } => {
                let c_expr = self.gen_expr(init);
                self.emit_line(&format!("aly_value_t {name} = {c_expr};"));
            }
            Stmt::Assign { target, value } => {
                let c_target = self.gen_expr(target);
                let c_value = self.gen_expr(value);
                self.emit_line(&format!("{c_target} = {c_value};"));
            }
            Stmt::Expr(expr) => {
                let c_expr = self.gen_expr(expr);
                self.emit_line(&format!("{c_expr};"));
            }
            Stmt::Return(expr) => {
                match expr {
                    Some(e) => {
                        let c_expr = self.gen_expr(e);
                        self.emit_line(&format!("return {c_expr};"));
                    }
                    None => {
                        self.emit_line("return aly_none();");
                    }
                }
            }
            Stmt::If {
                condition,
                then_body,
                elifs,
                else_body,
            } => {
                let c_cond = self.gen_expr(condition);
                self.emit_line(&format!("if ({c_cond}) {{"));
                self.indent += 1;
                for s in then_body {
                    self.gen_stmt(s);
                }
                self.indent -= 1;

                for (elif_cond, elif_body) in elifs {
                    let c_elif = self.gen_expr(elif_cond);
                    self.emit_line(&format!("}} else if ({c_elif}) {{"));
                    self.indent += 1;
                    for s in elif_body {
                        self.gen_stmt(s);
                    }
                    self.indent -= 1;
                }

                if let Some(eb) = else_body {
                    self.emit_line("} else {");
                    self.indent += 1;
                    for s in eb {
                        self.gen_stmt(s);
                    }
                    self.indent -= 1;
                }

                self.emit_line("}");
            }
            Stmt::While { condition, body } => {
                let c_cond = self.gen_expr(condition);
                self.emit_line(&format!("while ({c_cond}) {{"));
                self.indent += 1;
                for s in body {
                    self.gen_stmt(s);
                }
                self.indent -= 1;
                self.emit_line("}");
            }
            Stmt::For {
                init,
                condition,
                update,
                body,
            } => {
                self.emit("for (");
                if let Some(i) = init {
                    match i.as_ref() {
                        Stmt::Let { name, init: Some(expr) } => {
                            let c_expr = self.gen_expr(expr);
                            self.emit(&format!("aly_value_t {name} = {c_expr}"));
                        }
                        Stmt::Let { name, init: None } => {
                            self.emit(&format!("aly_value_t {name} = aly_none()"));
                        }
                        Stmt::Expr(e) => {
                            let c = self.gen_expr(e);
                            self.emit(&c);
                        }
                        _ => {}
                    }
                }
                self.emit(";");
                if let Some(c) = condition {
                    let cc = self.gen_expr(c);
                    self.emit(&format!(" {cc}"));
                }
                self.emit(";");
                if let Some(u) = update {
                    let cu = self.gen_expr(u);
                    self.emit(&format!(" {cu}"));
                }
                self.emit(") {\n");
                self.indent += 1;
                for s in body {
                    self.gen_stmt(s);
                }
                self.indent -= 1;
                self.emit_line("}");
            }
            Stmt::DoWhile { body, condition } => {
                self.emit_line("do {");
                self.indent += 1;
                for s in body {
                    self.gen_stmt(s);
                }
                self.indent -= 1;
                let c_cond = self.gen_expr(condition);
                self.emit_line(&format!("}} while ({c_cond});"));
            }
            Stmt::Fun { name, params, body } => {
                let c_params = if params.is_empty() {
                    "void".to_string()
                } else {
                    params
                        .iter()
                        .map(|p| format!("aly_value_t {p}"))
                        .collect::<Vec<_>>()
                        .join(", ")
                };
                self.emit_line(&format!("aly_value_t {name}({c_params}) {{"));
                self.indent += 1;

                for p in params {
                    self.emit_line(&format!("aly_ref({p});"));
                }

                for s in body {
                    self.gen_stmt(s);
                }

                self.emit_line("return aly_none();");
                self.indent -= 1;
                self.emit_line("}");
            }
            Stmt::Match { scrutinee, arms } => {
                let c_scrut = self.gen_expr(scrutinee);
                let scrut_var = format!("_match_val_{}", self.label_count);
                self.label_count += 1;
                self.emit_line(&format!("aly_value_t {scrut_var} = {c_scrut};"));

                let mut first = true;
                for arm in arms {
                    for pattern in &arm.patterns {
                        let prefix = if first { "if" } else { "} else if" };
                        first = false;

                        match pattern {
                            Pattern::Literal(lit) => {
                                let c_lit = self.gen_expr(lit);
                                self.emit_line(&format!(
                                    "{prefix} (aly_eq({scrut_var}, {c_lit})) {{"
                                ));
                            }
                            Pattern::Range(lo, hi) => {
                                let c_lo = self.gen_expr(lo);
                                let c_hi = self.gen_expr(hi);
                                self.emit_line(&format!(
                                    "{prefix} (aly_gte({scrut_var}, {c_lo}) && aly_lte({scrut_var}, {c_hi})) {{"
                                ));
                            }
                            Pattern::Wildcard => {
                                self.emit_line(&format!("{prefix} (1) {{"));
                            }
                            Pattern::Or(exprs) => {
                                let mut conditions = Vec::new();
                                for e in exprs {
                                    let c = self.gen_expr(e);
                                    conditions.push(format!("aly_eq({scrut_var}, {c})"));
                                }
                                let cond = conditions.join(" || ");
                                self.emit_line(&format!("{prefix} ({cond}) {{"));
                            }
                        }

                        self.indent += 1;
                        for s in &arm.body {
                            self.gen_stmt(s);
                        }
                        self.indent -= 1;
                    }
                }

                self.emit_line("}");
            }
        }
    }

    fn gen_expr(&mut self, expr: &Expr) -> String {
        match expr {
            Expr::Int(n) => format!("{n}"),
            Expr::Float(f) => format!("{f}"),
            Expr::Bool(b) => {
                if *b {
                    "1".to_string()
                } else {
                    "0".to_string()
                }
            }
            Expr::None => "aly_none()".to_string(),
            Expr::Var(name) => name.clone(),
            Expr::Str(lit) => lit.clone(),
            Expr::TemplateStr(parts) => self.gen_template(parts),
            Expr::UnaryOp { op, expr } => {
                let c_expr = self.gen_expr(expr);
                if op == "-" {
                    format!("aly_neg({c_expr})")
                } else if op == "!" {
                    format!("aly_not({c_expr})")
                } else {
                    format!("{op}{c_expr}")
                }
            }
            Expr::BinOp { left, op, right } => self.gen_binop(left, op, right),
            Expr::Call { function, args } => self.gen_call(function, args),
            Expr::Index { object, index } => {
                let c_obj = self.gen_expr(object);
                let c_idx = self.gen_expr(index);
                format!("aly_array_get({c_obj}, {c_idx})")
            }
            Expr::PropAccess { object, prop } => {
                let c_obj = self.gen_expr(object);
                format!("aly_object_get({c_obj}, \"{prop}\")")
            }
            Expr::Array(elements) => {
                if elements.is_empty() {
                    "aly_array_new(0)".to_string()
                } else {
                    let c_elements: Vec<String> =
                        elements.iter().map(|e| self.gen_expr(e)).collect();
                    let count = c_elements.len();
                    let elems = c_elements.join(", ");
                    format!("aly_array_init({count}, (aly_value_t[]){{{elems}}})")
                }
            }
            Expr::Object(entries) => {
                if entries.is_empty() {
                    "aly_object_new()".to_string()
                } else {
                    let mut obj_var = format!("_obj_{}", self.label_count);
                    self.label_count += 1;
                    let mut code = format!("aly_object_new()");
                    let mut stmts = vec![format!(
                        "aly_value_t {obj_var} = {code};"
                    )];
                    for (key, val) in entries {
                        let c_val = self.gen_expr(val);
                        stmts.push(format!(
                            "aly_object_set({obj_var}, \"{key}\", {c_val});"
                        ));
                    }
                    stmts.push(obj_var.clone());
                    code = stmts.join(" ");
                    code
                }
            }
            Expr::Percent(expr) => {
                let c_expr = self.gen_expr(expr);
                format!("aly_percent({c_expr})")
            }
        }
    }

    fn gen_binop(&mut self, left: &Expr, op: &str, right: &Expr) -> String {
        let c_left = self.gen_expr(left);
        let c_right = self.gen_expr(right);

        match op {
            "=" => format!("{c_left} = {c_right}"),
            "+" => format!("aly_add({c_left}, {c_right})"),
            "-" => format!("aly_sub({c_left}, {c_right})"),
            "*" => format!("aly_mul({c_left}, {c_right})"),
            "/" => format!("aly_div({c_left}, {c_right})"),
            "%" | "|" => format!("aly_mod({c_left}, {c_right})"),
            "eq" | "EQ" | "neq" | "NEQ" | "lt" | "LT" | "gt" | "GT" | "lte" | "LTE"
            | "gte" | "GTE" => {
                let cop = op_to_c(op);
                format!("aly_compare({c_left}, \"{cop}\", {c_right})")
            }
            "and" | "AND" => format!("aly_and({c_left}, {c_right})"),
            "or" | "OR" => format!("aly_or({c_left}, {c_right})"),
            "xor" | "XOR" => format!("aly_xor({c_left}, {c_right})"),
            _ => format!("{c_left} {op} {c_right}"),
        }
    }

    fn gen_call(&mut self, func: &Expr, args: &[Expr]) -> String {
        match func {
            Expr::PropAccess { object, prop } => {
                let c_obj = self.gen_expr(object);
                let c_args: Vec<String> = args.iter().map(|a| self.gen_expr(a)).collect();
                let args_str = if c_args.is_empty() {
                    String::new()
                } else {
                    c_args.join(", ")
                };
                format!("{c_obj}->vtable->{prop}({c_obj}, {args_str})")
            }
            Expr::Var(name) => {
                let c_args: Vec<String> = args.iter().map(|a| self.gen_expr(a)).collect();
                let args_str = if c_args.is_empty() {
                    String::new()
                } else {
                    c_args.join(", ")
                };
                format!("{name}({args_str})")
            }
            _ => {
                let c_func = self.gen_expr(func);
                let c_args: Vec<String> = args.iter().map(|a| self.gen_expr(a)).collect();
                let args_str = if c_args.is_empty() {
                    String::new()
                } else {
                    c_args.join(", ")
                };
                format!("{c_func}({args_str})")
            }
        }
    }

    fn gen_template(&mut self, parts: &[TemplatePart]) -> String {
        let mut has_vars = false;
        for p in parts {
            if matches!(p, TemplatePart::Var(_)) {
                has_vars = true;
                break;
            }
        }

        if !has_vars {
            let text: String = parts
                .iter()
                .filter_map(|p| match p {
                    TemplatePart::Text(s) => Some(s.as_str()),
                    _ => None,
                })
                .collect();
            return format!("\"{text}\"");
        }

        let mut fmt = String::new();
        let mut var_names = Vec::new();

        for part in parts {
            match part {
                TemplatePart::Text(s) => {
                    let escaped = s.replace('\\', "\\\\")
                        .replace('"', "\\\"")
                        .replace('\n', "\\n")
                        .replace('\t', "\\t");
                    fmt.push_str(&escaped);
                }
                TemplatePart::Var(name) => {
                    fmt.push_str("%s");
                    var_names.push(name.clone());
                }
            }
        }

        let buf_var = format!("_buf_{}", self.label_count);
        self.label_count += 1;

        if var_names.len() == 1 {
            format!(
                "({snprintf}({buf_var}, sizeof({buf_var}), \"{fmt}\", aly_to_str({var})))",
                snprintf = "snprintf",
                var = var_names[0]
            )
        } else {
            let vars: Vec<String> = var_names
                .iter()
                .map(|v| format!("aly_to_str({v})"))
                .collect();
            format!(
                "({snprintf}({buf_var}, sizeof({buf_var}), \"{fmt}\", {vars}))",
                snprintf = "snprintf",
                vars = vars.join(", ")
            )
        }
    }
}

pub fn op_to_c(op: &str) -> &str {
    crate::compiler::parser::op_to_c(op)
}
