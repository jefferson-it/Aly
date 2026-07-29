use crate::compiler::ast::*;
use super::chunk::Chunk;
use super::opcode::*;
use super::value::Value;

#[derive(Debug)]
pub enum CompileError {
    UnknownOperator(String),
    TooManyLocals,
    TooManyConstants,
    NotSupported(String),
}

impl std::fmt::Display for CompileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CompileError::UnknownOperator(op) => write!(f, "unknown operator '{op}'"),
            CompileError::TooManyLocals => write!(f, "too many local variables"),
            CompileError::TooManyConstants => write!(f, "too many constants"),
            CompileError::NotSupported(feat) => write!(f, "not supported yet: {feat}"),
        }
    }
}

type R<T> = std::result::Result<T, CompileError>;

pub fn compile_program(program: &Program) -> R<Chunk> {
    let mut c = Compiler::new("__main__");
    c.compile_program(program)?;
    Ok(c.chunk)
}

struct Local { name: String, #[allow(dead_code)] depth: usize, index: u8 }
struct Scope { locals: Vec<Local> }
struct Upvalue { is_local: bool, index: u8 }

struct Compiler {
    chunk: Chunk,
    scopes: Vec<Scope>,
    scope_depth: usize,
    upvalues: Vec<Upvalue>,
    loop_exits: Vec<Vec<usize>>,
    loop_continues: Vec<Vec<usize>>,
}

impl Compiler {
    fn new(name: &str) -> Self {
        Compiler {
            chunk: Chunk::new(name),
            scopes: vec![Scope { locals: Vec::new() }],
            scope_depth: 0,
            upvalues: Vec::new(),
            loop_exits: Vec::new(),
            loop_continues: Vec::new(),
        }
    }

    fn line(&self) -> usize { self.chunk.lines.last().copied().unwrap_or(0) }

    fn compile_program(&mut self, program: &Program) -> R<()> {
        for stmt in &program.stmts { self.compile_stmt(stmt)?; }
        self.emit(OpCode::Nil, 0, 0, 0);
        self.emit(OpCode::Return, 0, 0, 0);
        Ok(())
    }

    fn compile_stmt(&mut self, stmt: &Stmt) -> R<()> {
        match stmt {
            Stmt::Let { name, init, .. } => {
                if let Some(e) = init { self.compile_expr(e)?; }
                else { self.emit(OpCode::Nil, 0, 0, self.line()); }
                self.define_variable(name)?;
            }
            Stmt::Const { name, init, .. } => { self.compile_expr(init)?; self.define_variable(name)?; }
            Stmt::Assign { target, value } => { return self.compile_assign(target, value); }
            Stmt::Expr(expr) => {
                // Handle assignment-in-expression (parser wraps `i = i + 1` as BinOp with `=`)
                // The `=` operator has lowest precedence, so it becomes the root BinOp
                if let Expr::BinOp { left, op, right } = expr {
                    if op == "=" {
                        return self.compile_assign(left, right);
                    }
                }
                self.compile_expr(expr)?; self.emit(OpCode::Pop, 0, 0, self.line());
            }
            Stmt::Return(opt) => {
                if let Some(e) = opt { self.compile_expr(e)?; }
                else { self.emit(OpCode::Void, 0, 0, self.line()); }
                self.emit(OpCode::Return, 0, 0, self.line());
            }
            Stmt::Break => {
                if self.loop_exits.is_empty() {
                    return Err(CompileError::NotSupported("break outside of loop".into()));
                }
                let jmp_idx = self.chunk.code.len();
                let line = self.line();
                self.emit(OpCode::Jump, 0, 0, line);
                self.loop_exits.last_mut().unwrap().push(jmp_idx);
            }
            Stmt::Continue => {
                if self.loop_continues.is_empty() {
                    return Err(CompileError::NotSupported("continue outside of loop".into()));
                }
                let jmp_idx = self.chunk.code.len();
                let line = self.line();
                self.emit(OpCode::Jump, 0, 0, line);
                self.loop_continues.last_mut().unwrap().push(jmp_idx);
            }
            Stmt::If { condition, then_body, elifs, else_body } => {
                self.compile_if(condition, then_body, elifs, else_body)?;
            }
            Stmt::While { condition, body } => {
                let start = self.chunk.code.len();
                self.compile_expr(condition)?;
                let jmp = self.chunk.code.len();
                self.emit(OpCode::JumpIfFalse, 0, 0, self.line());
                self.emit(OpCode::Pop, 0, 0, self.line());
                
                self.loop_exits.push(Vec::new());
                self.loop_continues.push(Vec::new());
                self.push_scope();
                for s in body { self.compile_stmt(s)?; }
                self.pop_scope();
                let continues = self.loop_continues.pop().unwrap_or_default();
                let exits = self.loop_exits.pop().unwrap_or_default();
                
                for continue_idx in continues {
                    self.patch_offset_to(continue_idx, start);
                }
                
                self.patch_loop(start);
                self.patch_offset(jmp, 0);
                self.emit(OpCode::Pop, 0, 0, self.line());
                
                for exit_idx in exits {
                    self.patch_offset(exit_idx, 0);
                }
            }
            Stmt::For { init, condition, update, body } => {
                self.push_scope();
                if let Some(i) = init { self.compile_stmt(i)?; }
                let start = self.chunk.code.len();
                if let Some(c) = condition { self.compile_expr(c)?; }
                else { self.emit(OpCode::True, 0, 0, self.line()); }
                let jmp = self.chunk.code.len();
                self.emit(OpCode::JumpIfFalse, 0, 0, self.line());
                self.emit(OpCode::Pop, 0, 0, self.line());
                
                self.loop_exits.push(Vec::new());
                self.loop_continues.push(Vec::new());
                for s in body { self.compile_stmt(s)?; }
                
                let cont_target = self.chunk.code.len();
                if let Some(u) = update { self.compile_expr(u)?; self.emit(OpCode::Pop, 0, 0, self.line()); }
                let continues = self.loop_continues.pop().unwrap_or_default();
                for continue_idx in continues {
                    self.patch_offset_to(continue_idx, cont_target);
                }
                
                let exits = self.loop_exits.pop().unwrap_or_default();
                
                self.patch_loop(start);
                self.patch_offset(jmp, 0);
                self.emit(OpCode::Pop, 0, 0, self.line());
                self.pop_scope();
                
                for exit_idx in exits {
                    self.patch_offset(exit_idx, 0);
                }
            }
            Stmt::DoWhile { body, condition } => {
                let start = self.chunk.code.len();
                
                self.loop_exits.push(Vec::new());
                self.loop_continues.push(Vec::new());
                self.push_scope();
                for s in body { self.compile_stmt(s)?; }
                self.pop_scope();
                let continues = self.loop_continues.pop().unwrap_or_default();
                let exits = self.loop_exits.pop().unwrap_or_default();
                
                let cont_target = self.chunk.code.len();
                for continue_idx in continues {
                    self.patch_offset_to(continue_idx, cont_target);
                }
                
                self.compile_expr(condition)?;
                let jmp = self.chunk.code.len();
                self.emit(OpCode::JumpIfFalse, 0, 0, self.line());
                self.emit(OpCode::Pop, 0, 0, self.line());
                
                self.patch_loop(start);
                self.patch_offset(jmp, 0);
                self.emit(OpCode::Pop, 0, 0, self.line());
                
                for exit_idx in exits {
                    self.patch_offset(exit_idx, 0);
                }
            }
            Stmt::Foreach { item, collection, body } => {
                let id = self.chunk.code.len();
                let col_var_name = format!("_col_{}", id);
                let len_var_name = format!("_len_{}", id);
                let idx_var_name = format!("_idx_{}", id);
                
                let init_col = Stmt::Let {
                    name: col_var_name.clone(),
                    init: Some(collection.clone()),
                    attrs: vec![],
                };
                let init_len = Stmt::Let {
                    name: len_var_name.clone(),
                    init: Some(Expr::PropAccess {
                        object: Box::new(Expr::Var(col_var_name.clone())),
                        prop: "len".to_string(),
                    }),
                    attrs: vec![],
                };
                let init_idx = Stmt::Let {
                    name: idx_var_name.clone(),
                    init: Some(Expr::Int(0)),
                    attrs: vec![],
                };
                
                let cond = Expr::BinOp {
                    left: Box::new(Expr::Var(idx_var_name.clone())),
                    op: "lt".to_string(),
                    right: Box::new(Expr::Var(len_var_name.clone())),
                };
                
                let update = Expr::BinOp {
                    left: Box::new(Expr::Var(idx_var_name.clone())),
                    op: "=".to_string(),
                    right: Box::new(Expr::BinOp {
                        left: Box::new(Expr::Var(idx_var_name.clone())),
                        op: "+".to_string(),
                        right: Box::new(Expr::Int(1)),
                    }),
                };
                
                let item_decl = Stmt::Let {
                    name: item.clone(),
                    init: Some(Expr::Index {
                        object: Box::new(Expr::Var(col_var_name.clone())),
                        index: Box::new(Expr::Var(idx_var_name.clone())),
                    }),
                    attrs: vec![],
                };
                
                let mut for_body = vec![item_decl];
                for_body.extend(body.clone());
                
                let for_stmt = Stmt::For {
                    init: None,
                    condition: Some(cond),
                    update: Some(update),
                    body: for_body,
                };
                
                self.push_scope();
                self.compile_stmt(&init_col)?;
                self.compile_stmt(&init_len)?;
                self.compile_stmt(&init_idx)?;
                self.compile_stmt(&for_stmt)?;
                self.pop_scope();
            }
            Stmt::Fun { name, params, defaults, variadic, body, .. } => { self.compile_function(name, params, defaults, variadic.as_deref(), body)?; }
            Stmt::Match { scrutinee, arms } => {
                self.compile_expr(scrutinee)?;
                self.emit(OpCode::Pop, 0, 0, self.line());
                if let Some(arm) = arms.first() {
                    for s in &arm.body { self.compile_stmt(s)?; }
                }
            }

            Stmt::Destructure { .. } => {
                // Should have been desugared by the parser pass
                return Err(CompileError::NotSupported("destructure not desugared".into()));
            }
            Stmt::Lazy { name, init } => {
                // Compile lazy as a regular let for now (eager evaluation).
                // LazyCell infrastructure is in place for future thunking.
                self.compile_expr(init)?;
                self.define_variable(name)?;
            }
            Stmt::MacroDef { .. } => {
                // Macros are expanded at parse time; no runtime code needed
            }
            Stmt::Coroutine { name, params, body } => {
                // Compile coroutine as a function that can yield
                self.compile_function(name, params, &[], None, body)?;
            }
            Stmt::Yield(opt_expr) => {
                if let Some(e) = opt_expr { self.compile_expr(e)?; }
                else { self.emit(OpCode::Nil, 0, 0, self.line()); }
                // TODO: proper yield implementation for coroutines
                self.emit(OpCode::Return, 0, 0, self.line());
            }
            Stmt::Throw(_) => return Err(CompileError::NotSupported("throw".into())),
            Stmt::Try { .. } => return Err(CompileError::NotSupported("try/catch".into())),
        }
        Ok(())
    }

    fn compile_assign(&mut self, target: &Expr, value: &Expr) -> R<()> {
        match target {
            Expr::Var(name) => {
                self.compile_expr(value)?;
                if self.assign_to_local(name) { Ok(()) }
                else {
                    let idx = self.make_const(Value::Str(name.clone()));
                    self.emit(OpCode::SetGlobal, idx as u8, 0, self.line());
                    Ok(())
                }
            }
            Expr::Index { object, index } => {
                self.compile_expr(object)?; self.compile_expr(index)?; self.compile_expr(value)?;
                self.emit(OpCode::SubscriptSet, 0, 0, self.line());
                Ok(())
            }
            Expr::PropAccess { object, prop } => {
                self.compile_expr(object)?;
                let idx = self.make_const(Value::Str(prop.clone()));
                self.emit(OpCode::Const, idx as u8, 0, self.line());
                self.compile_expr(value)?;
                self.emit(OpCode::SubscriptSet, 0, 0, self.line());
                Ok(())
            }
            _ => Err(CompileError::NotSupported("complex assignment target".into())),
        }
    }

    fn compile_if(&mut self, condition: &Expr, then_body: &[Stmt],
                   elifs: &[(Expr, Vec<Stmt>)], else_body: &Option<Vec<Stmt>>) -> R<()> {
        self.compile_expr(condition)?;
        let mut end_jumps = Vec::new();
        let jmp = self.chunk.code.len();
        self.emit(OpCode::JumpIfFalse, 0, 0, self.line());
        self.emit(OpCode::Pop, 0, 0, self.line());
        self.push_scope();
        for s in then_body { self.compile_stmt(s)?; }
        self.pop_scope();
        end_jumps.push(self.chunk.code.len());
        self.emit(OpCode::Jump, 0, 0, self.line());
        self.patch_offset(jmp, 0);
        for (cond, body) in elifs {
            self.emit(OpCode::Pop, 0, 0, self.line());
            self.compile_expr(cond)?;
            let j = self.chunk.code.len();
            self.emit(OpCode::JumpIfFalse, 0, 0, self.line());
            self.emit(OpCode::Pop, 0, 0, self.line());
            self.push_scope();
            for s in body { self.compile_stmt(s)?; }
            self.pop_scope();
            end_jumps.push(self.chunk.code.len());
            self.emit(OpCode::Jump, 0, 0, self.line());
            self.patch_offset(j, 0);
        }
        self.emit(OpCode::Pop, 0, 0, self.line());
        if let Some(body) = else_body {
            self.push_scope();
            for s in body { self.compile_stmt(s)?; }
            self.pop_scope();
        }
        for j in end_jumps { self.patch_offset(j, 0); }
        Ok(())
    }

    fn compile_function(&mut self, name: &str, params: &[String], defaults: &[Option<Expr>], variadic: Option<&str>, body: &[Stmt]) -> R<()> {
        let mut func = Compiler::new(name);
        func.push_scope();

        let mut param_names = Vec::new();
        let mut param_defaults = Vec::new();

        for (i, param) in params.iter().enumerate() {
            param_names.push(param.clone());
            let def_val = if let Some(ref expr) = defaults.get(i).and_then(|d| d.as_ref()) {
                match expr {
                    Expr::Int(n) => Some(Value::Int(*n)),
                    Expr::Float(f) => Some(Value::Float(*f)),
                    Expr::Char(c) => Some(Value::Char(*c)),
                    Expr::Str(s) => Some(Value::Str(s.clone())),
                    Expr::Bool(b) => Some(Value::Bool(*b)),
                    Expr::None => Some(Value::Nil),
                    Expr::Void => Some(Value::Void),
                    _ => Some(Value::Nil),
                }
            } else {
                None
            };
            param_defaults.push(def_val);

            let local = Local { name: param.clone(), depth: func.scope_depth, index: func.current_scope().locals.len() as u8 };
            func.current_scope_mut().locals.push(local);
        }

        func.chunk.param_names = param_names;
        func.chunk.param_defaults = param_defaults;
        func.chunk.is_variadic = variadic.is_some();

        for s in body { func.compile_stmt(s)?; }
        func.emit(OpCode::Void, 0, 0, 0);
        func.emit(OpCode::Return, 0, 0, 0);
        func.chunk.local_count = func.current_scope().locals.len();
        func.chunk.upvalue_count = func.upvalues.len();
        let idx = self.chunk.add_function(func.chunk);
        let uv_count = func.upvalues.len() as u8;
        self.emit(OpCode::Closure, idx as u8, uv_count, self.line());
        for uv in &func.upvalues {
            let word = (if uv.is_local { 1u32 } else { 0u32 }) | ((uv.index as u32) << 8);
            self.chunk.write(word, self.line());
        }
        self.define_variable(name)?;
        Ok(())
    }

    fn compile_expr(&mut self, expr: &Expr) -> R<()> {
        match expr {
            Expr::Int(n) => {
                if *n >= 0 && *n <= 255 {
                    self.emit(OpCode::LoadInt, *n as u8, 0, self.line());
                } else {
                    let idx = self.make_const(Value::Int(*n));
                    self.emit(OpCode::Const, idx as u8, 0, self.line());
                }
            }
            Expr::Float(f) => { let idx = self.make_const(Value::Float(*f)); self.emit(OpCode::Const, idx as u8, 0, self.line()); }
            Expr::Char(c) => {
                let idx = self.make_const(Value::Char(*c));
                self.emit(OpCode::Const, idx as u8, 0, self.line());
            }
            Expr::Str(s) => { 
                    let idx = self.make_const(Value::Str(s.clone())); 
                    self.emit(OpCode::Const, idx as u8, 0, self.line()); 
                }
            Expr::Bool(b) => { if *b { self.emit(OpCode::True, 0, 0, self.line()); } else { self.emit(OpCode::False, 0, 0, self.line()); } }
            Expr::None => { self.emit(OpCode::Nil, 0, 0, self.line()); }
            Expr::Void => { self.emit(OpCode::Void, 0, 0, self.line()); }
            Expr::Var(name) => {
                if name.starts_with('&') {
                    let var_name = &name[1..];
                    if let Some(local_idx) = self.resolve_local(var_name) {
                        self.emit(OpCode::GetRefLocal, local_idx, 0, self.line());
                    } else {
                        let idx = self.make_const(Value::Str(var_name.to_owned()));
                        self.emit(OpCode::GetRefGlobal, idx as u8, 0, self.line());
                    }
                } else {
                    if !self.load_local(name) {
                        let idx = self.make_const(Value::Str(name.clone()));
                        self.emit(OpCode::GetGlobal, idx as u8, 0, self.line());
                    }
                }
            }
            Expr::TemplateStr(parts) => { self.compile_template(parts)?; }
            Expr::UnaryOp { op, expr } => {
                self.compile_expr(expr)?;
                match op.as_str() {
                    "-" => self.emit(OpCode::Negate, 0, 0, self.line()),
                    "not" | "NOT" | "!" => self.emit(OpCode::Not, 0, 0, self.line()),
                    _ => return Err(CompileError::UnknownOperator(op.clone())),
                }
            }
            Expr::BinOp { left, op, right } => {
                if op == "=" {
                    self.compile_assign(left, right)?;
                    self.emit(OpCode::Nil, 0, 0, self.line());
                    return Ok(());
                }
                return self.compile_binary(left, op, right);
            }
            Expr::Call { function, args } => {
                if let Expr::Var(name) = &**function {
                    if name == "print" {
                        if args.len() == 1 {
                            self.compile_expr(&args[0])?;
                            self.emit(OpCode::Print, 0, 0, self.line());
                            self.emit(OpCode::Nil, 0, 0, self.line());
                            return Ok(());
                        }
                    }
                }
                self.compile_expr(function)?;
                let mut total_slots = 0;
                let mut named_count = 0;
                for arg in args {
                    if let Expr::BinOp { left, op, right } = arg {
                        if op == "=" {
                            if let Expr::Var(ref name) = &**left {
                                self.compile_expr(right)?;
                                let name_const = self.make_const(Value::Str(name.clone()));
                                self.emit(OpCode::Const, name_const as u8, 0, self.line());
                                total_slots += 2;
                                named_count += 1;
                                continue;
                            }
                        }
                    }
                    self.compile_expr(arg)?;
                    total_slots += 1;
                }
                self.emit(OpCode::Call, (total_slots + 1) as u8, named_count as u8, self.line());
            }
            Expr::Index { object, index } => {
                self.compile_expr(object)?; self.compile_expr(index)?;
                self.emit(OpCode::Subscript, 0, 0, self.line());
            }
            Expr::PropAccess { object, prop } => {
                self.compile_expr(object)?;
                let idx = self.make_const(Value::Str(prop.clone()));
                self.emit(OpCode::Const, idx as u8, 0, self.line());
                self.emit(OpCode::Subscript, 0, 0, self.line());
            }
            Expr::Array(items) => {
                for item in items { self.compile_expr(item)?; }
                self.emit(OpCode::BuildList, items.len() as u8, 0, self.line());
            }
            Expr::Tuple(items) => {
                for item in items { self.compile_expr(item)?; }
                self.emit(OpCode::BuildTuple, items.len() as u8, 0, self.line());
            }
            Expr::Object(pairs) => {
                for (key, val) in pairs {
                    let k = self.make_const(Value::Str(key.clone()));
                    self.emit(OpCode::Const, k as u8, 0, self.line());
                    self.compile_expr(val)?;
                }
                self.emit(OpCode::BuildMap, pairs.len() as u8, 0, self.line());
            }
            Expr::MacroExpand { name, args } => {
                // Macros are expanded at parse time; treat as function call
                let func_expr = Expr::Var(name.clone());
                self.compile_expr(&func_expr)?;
                for arg in args { self.compile_expr(arg)?; }
                self.emit(OpCode::Call, (args.len() + 1) as u8, 0, self.line());
            }
            Expr::YieldExpr(e) => {
                self.compile_expr(e)?;
                self.emit(OpCode::Return, 0, 0, self.line());
            }
            Expr::Percent(e) => { self.compile_expr(e)?; }
            Expr::AnonymousFun { params, defaults, variadic, body } => {
                let name = format!("__anon_fun_{}", self.chunk.functions.len());
                self.compile_function(&name, params, defaults, variadic.as_deref(), body)?;
                if let Some(local_idx) = self.resolve_local(&name) {
                    self.emit(OpCode::GetLocal, local_idx, 0, self.line());
                } else {
                    let idx = self.make_const(Value::Str(name));
                    self.emit(OpCode::GetGlobal, idx as u8, 0, self.line());
                }
            }
        }
        Ok(())
    }

    fn compile_binary(&mut self, left: &Expr, op: &str, right: &Expr) -> R<()> {
        if let Some(folded) = self.try_fold_constants(left, op, right) {
            match folded {
                Value::Int(n) => {
                    if n >= 0 && n <= 255 {
                        self.emit(OpCode::LoadInt, n as u8, 0, self.line());
                    } else {
                        let idx = self.make_const(Value::Int(n));
                        self.emit(OpCode::Const, idx as u8, 0, self.line());
                    }
                }
                Value::Float(f) => {
                    let idx = self.make_const(Value::Float(f));
                    self.emit(OpCode::Const, idx as u8, 0, self.line());
                }
                Value::Str(s) => {
                    let idx = self.make_const(Value::Str(s));
                    self.emit(OpCode::Const, idx as u8, 0, self.line());
                }
                Value::Bool(b) => {
                    if b {
                        self.emit(OpCode::True, 0, 0, self.line());
                    } else {
                        self.emit(OpCode::False, 0, 0, self.line());
                    }
                }
                _ => {}
            }
            return Ok(());
        }

        self.compile_expr(left)?;
        match op {
            "and" | "AND" => {
                let jmp = self.chunk.code.len();
                self.emit(OpCode::JumpIfFalse, 0, 0, self.line());
                self.emit(OpCode::Pop, 0, 0, self.line());
                self.compile_expr(right)?;
                self.patch_offset(jmp, 0);
            }
            "or" | "OR" => {
                let jmp = self.chunk.code.len();
                self.emit(OpCode::JumpIfTrue, 0, 0, self.line());
                self.emit(OpCode::Pop, 0, 0, self.line());
                self.compile_expr(right)?;
                self.patch_offset(jmp, 0);
            }
            _ => {
                self.compile_expr(right)?;
                match op {
                    "+" => self.emit(OpCode::Add, 0, 0, self.line()),
                    "-" => self.emit(OpCode::Subtract, 0, 0, self.line()),
                    "*" => self.emit(OpCode::Multiply, 0, 0, self.line()),
                    "/" => self.emit(OpCode::Divide, 0, 0, self.line()),
                    "%" | "|" => self.emit(OpCode::Modulus, 0, 0, self.line()),
                    "eq" | "EQ" | "==" => self.emit(OpCode::Equal, 0, 0, self.line()),
                    "neq" | "NEQ" | "!=" => self.emit(OpCode::NotEqual, 0, 0, self.line()),
                    "lt" | "LT" => self.emit(OpCode::Less, 0, 0, self.line()),
                    "gt" | "GT" => self.emit(OpCode::Greater, 0, 0, self.line()),
                    "lte" | "LTE" | "<=" => self.emit(OpCode::LessEqual, 0, 0, self.line()),
                    "gte" | "GTE" | ">=" => self.emit(OpCode::GreaterEqual, 0, 0, self.line()),
                    "." => self.emit(OpCode::Concat, 0, 0, self.line()),
                    _ => return Err(CompileError::UnknownOperator(op.into())),
                }
            }
        };
        Ok(())
    }

    fn try_fold_constants(&self, left: &Expr, op: &str, right: &Expr) -> Option<Value> {
        match (left, right) {
            (Expr::Int(l), Expr::Int(r)) => {
                match op {
                    "+" => Some(Value::Int(l.checked_add(*r)?)),
                    "-" => Some(Value::Int(l.checked_sub(*r)?)),
                    "*" => Some(Value::Int(l.checked_mul(*r)?)),
                    "/" => if *r == 0 { None } else { Some(Value::Int(l / r)) },
                    "%" => if *r == 0 { None } else { Some(Value::Int(l % r)) },
                    "==" => Some(Value::Bool(l == r)),
                    "!=" => Some(Value::Bool(l != r)),
                    "<" | "lt" | "LT" => Some(Value::Bool(l < r)),
                    ">" | "gt" | "GT" => Some(Value::Bool(l > r)),
                    "<=" | "lte" | "LTE" => Some(Value::Bool(l <= r)),
                    ">=" | "gte" | "GTE" => Some(Value::Bool(l >= r)),
                    _ => None,
                }
            }
            (Expr::Float(l), Expr::Float(r)) => {
                match op {
                    "+" => Some(Value::Float(l + r)),
                    "-" => Some(Value::Float(l - r)),
                    "*" => Some(Value::Float(l * r)),
                    "/" => Some(Value::Float(l / r)),
                    "==" => Some(Value::Bool((l - r).abs() < f64::EPSILON)),
                    "!=" => Some(Value::Bool((l - r).abs() >= f64::EPSILON)),
                    "<" | "lt" | "LT" => Some(Value::Bool(l < r)),
                    ">" | "gt" | "GT" => Some(Value::Bool(l > r)),
                    "<=" | "lte" | "LTE" => Some(Value::Bool(l <= r)),
                    ">=" | "gte" | "GTE" => Some(Value::Bool(l >= r)),
                    _ => None,
                }
            }
            (Expr::Str(l), Expr::Str(r)) => {
                match op {
                    "+" | "." => Some(Value::Str(format!("{}{}", l, r))),
                    "==" => Some(Value::Bool(l == r)),
                    "!=" => Some(Value::Bool(l != r)),
                    _ => None,
                }
            }
            (Expr::Char(l), Expr::Char(r)) => {
                match op {
                    "==" => Some(Value::Bool(l == r)),
                    "!=" => Some(Value::Bool(l != r)),
                    "<" | "lt" | "LT" => Some(Value::Bool(l < r)),
                    ">" | "gt" | "GT" => Some(Value::Bool(l > r)),
                    "<=" | "lte" | "LTE" => Some(Value::Bool(l <= r)),
                    ">=" | "gte" | "GTE" => Some(Value::Bool(l >= r)),
                    _ => None,
                }
            }
            (Expr::Void, Expr::Void) => {
                match op {
                    "==" => Some(Value::Bool(true)),
                    "!=" => Some(Value::Bool(false)),
                    _ => None,
                }
            }
            _ => None,
        }
    }

    fn compile_template(&mut self, parts: &[TemplatePart]) -> R<()> {
        if parts.is_empty() {
            let idx = self.make_const(Value::Str(String::new()));
            self.emit(OpCode::Const, idx as u8, 0, self.line());
            return Ok(());
        }
        let mut first = true;
        for part in parts {
            match part {
                TemplatePart::Text(s) => {
                    let idx = self.make_const(Value::Str(s.clone()));
                    self.emit(OpCode::Const, idx as u8, 0, self.line());
                }
                TemplatePart::Var(name) => {
                    if !self.load_local(name) {
                        let idx = self.make_const(Value::Str(name.clone()));
                        self.emit(OpCode::GetGlobal, idx as u8, 0, self.line());
                    }
                }
            }
            if !first { self.emit(OpCode::Concat, 0, 0, self.line()); }
            first = false;
        }
        Ok(())
    }

    fn get_total_locals(&self) -> usize {
        self.scopes.iter().map(|s| s.locals.len()).sum()
    }

    fn push_scope(&mut self) {
        self.scopes.push(Scope { locals: Vec::new() });
        self.scope_depth += 1;
    }

    fn pop_scope(&mut self) {
        if let Some(scope) = self.scopes.pop() {
            let pop_count = scope.locals.len();
            for _ in 0..pop_count {
                self.emit(OpCode::Pop, 0, 0, self.line());
            }
        }
        self.scope_depth -= 1;
    }

    fn current_scope(&self) -> &Scope {
        self.scopes.last().unwrap()
    }

    fn current_scope_mut(&mut self) -> &mut Scope {
        self.scopes.last_mut().unwrap()
    }

    fn define_variable(&mut self, name: &str) -> R<()> {
        if self.scope_depth == 0 {
            let idx = self.make_const(Value::Str(name.to_owned()));
            self.emit(OpCode::DefineGlobal, idx as u8, 0, self.line());
        } else {
            let depth = self.scope_depth;
            let index = self.get_total_locals() as u8;
            let locals = &mut self.current_scope_mut().locals;
            if index >= 255 { return Err(CompileError::TooManyLocals); }
            locals.push(Local { name: name.to_owned(), depth, index });
            self.emit(OpCode::InitLocal, index, 0, self.line());
        }
        Ok(())
    }

    fn resolve_local(&self, name: &str) -> Option<u8> {
        for scope in self.scopes.iter().rev() {
            for local in scope.locals.iter().rev() {
                if local.name == name {
                    return Some(local.index);
                }
            }
        }
        None
    }

    fn load_local(&mut self, name: &str) -> bool {
        for scope in self.scopes.iter().rev() {
            for local in scope.locals.iter().rev() {
                if local.name == name {
                    self.emit(OpCode::GetLocal, local.index, 0, self.line());
                    return true;
                }
            }
        }
        false
    }

    fn assign_to_local(&mut self, name: &str) -> bool {
        for scope in self.scopes.iter().rev() {
            for local in scope.locals.iter().rev() {
                if local.name == name {
                    self.emit(OpCode::SetLocal, local.index, 0, self.line());
                    return true;
                }
            }
        }
        false
    }

    fn make_const(&mut self, val: Value) -> usize {
        self.chunk.add_constant(val)
    }

    fn patch_offset(&mut self, idx: usize, extra: usize) {
        let from = idx + 1;
        let to = self.chunk.code.len() + extra;
        let offset = (to - from) as i16;
        let enc = offset.to_le_bytes();
        self.chunk.code[idx] = self.chunk.code[idx] & 0x0000_00FF
            | ((enc[0] as u32) << 8) | ((enc[1] as u32) << 16);
    }

    fn patch_offset_to(&mut self, idx: usize, target: usize) {
        let from = idx + 1;
        let to = target;
        let offset = (to as i32 - from as i32) as i16;
        let enc = offset.to_le_bytes();
        self.chunk.code[idx] = self.chunk.code[idx] & 0x0000_00FF
            | ((enc[0] as u32) << 8) | ((enc[1] as u32) << 16);
    }

    fn patch_loop(&mut self, start: usize) {
        let offset = (self.chunk.code.len() - start + 1) as i16;
        let enc = offset.to_le_bytes();
        let _idx = self.chunk.code.len();
        self.chunk.code.push(encode(OpCode::Loop, enc[0], enc[1]));
        self.chunk.lines.push(self.line());
    }

    fn emit(&mut self, op: OpCode, a: u8, b: u8, line: usize) {
        self.chunk.emit(op, a, b, line);
    }
}
