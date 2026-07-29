use super::chunk::Chunk;
use super::opcode::*;
use super::value::*;
use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use super::jit;

/// The Aly Virtual Machine.
/// 
/// Stack-based, with direct-threaded dispatch via a match on opcodes.
/// The hot loop is a single `match` — Rust's LLVM backend compiles this into
/// a jump table (computed goto), giving performance close to C's computed goto.
pub struct VM {
    /// Globals table.
    pub(super) globals: HashMap<String, Value>,
    /// All loaded function chunks.
    functions: Vec<Chunk>,
    /// Currently executing chunk.
    chunk: Option<usize>,
    /// Instruction pointer (index into chunk.code).
    ip: usize,
    /// Value stack.
    pub(super) stack: Vec<Value>,
    /// Register file for register-based execution.
    registers: Vec<Value>,
    /// Call frame stack.
    frames: Vec<Frame>,
    /// JIT controller for adaptive hot path tiering.
    jit: jit::JitController,
    /// Map of Opcode execution counts for profiling.
    pub(super) opcode_counts: HashMap<OpCode, usize>,
    /// Central global values indexed by ID.
    pub(super) global_values: Vec<Value>,
    /// Map of global names to their stable ID index.
    pub(super) global_names: HashMap<String, usize>,
    /// Immutable variables tracking.
    pub(super) immutable_globals: std::collections::HashSet<String>,
    pub(super) immutable_locals: std::collections::HashSet<usize>,
}

pub struct Frame {
    /// Index into the functions table.
    func_idx: usize,
    /// Return instruction pointer (where to resume after return).
    return_ip: usize,
    /// Base pointer into the value stack (start of this frame's locals).
    base: usize,
}

impl VM {
    pub fn new() -> Self {
        let mut globals = HashMap::new();
        
        globals.insert("print".to_owned(), Value::Native(|args| {
            let mut output = String::new();
            for (i, arg) in args.iter().enumerate() {
                if i > 0 { output.push(' '); }
                output.push_str(&arg.to_string());
            }
            println!("{}", output);
            Value::Nil
        }, "print"));

        globals.insert("input".to_owned(), Value::Native(|args| {
            use std::io::{self, Write};
            if let Some(Value::Str(prompt)) = args.first() {
                print!("{}", prompt);
                let _ = io::stdout().flush();
            }
            let mut input = String::new();
            if io::stdin().read_line(&mut input).is_ok() {
                Value::Str(input.trim_end().to_owned())
            } else {
                Value::Str(String::new())
            }
        }, "input"));

        globals.insert("len".to_owned(), Value::Native(|args| {
            if let Some(arg) = args.first() {
                match arg {
                    Value::Str(s) => Value::Int(s.len() as i64),
                    Value::Vec(v) => Value::Int(v.borrow().len() as i64),
                    Value::Map(m) => Value::Int(m.len() as i64),
                    _ => Value::Int(0),
                }
            } else {
                Value::Int(0)
            }
        }, "len"));

globals.insert("tomb".to_owned(), Value::Native(|_| Value::Nil, "tomb"));

        // Shell object with methods
        let mut shell_methods: Vec<(Value, Value)> = Vec::new();
        
        shell_methods.push((
            Value::Str("exec".to_string()),
            Value::Native(|args| {
                use std::process::Command;
                if let Some(Value::Str(cmd)) = args.first() {
                    let output = Command::new("sh").arg("-c").arg(cmd).output();
                    match output {
                        Ok(o) => {
                            let stdout = String::from_utf8_lossy(&o.stdout).trim_end().to_string();
                            if !stdout.is_empty() {
                                Value::Str(stdout)
                            } else {
                                let stderr = String::from_utf8_lossy(&o.stderr).trim_end().to_string();
                                Value::Str(stderr)
                            }
                        }
                        Err(e) => Value::Str(format!("error: {}", e)),
                    }
                } else {
                    Value::Str(String::new())
                }
            }, "shell.exec")
        ));

        shell_methods.push((
            Value::Str("exec_status".to_string()),
            Value::Native(|args| {
                use std::process::Command;
                if let Some(Value::Str(cmd)) = args.first() {
                    let output = Command::new("sh").arg("-c").arg(cmd).output();
                    match output {
                        Ok(o) => {
                            let code = o.status.code().unwrap_or(-1);
                            let stdout = String::from_utf8_lossy(&o.stdout).trim_end().to_string();
                            let stderr = String::from_utf8_lossy(&o.stderr).trim_end().to_string();
                            let mut map = Vec::new();
                            map.push((Value::Str("code".to_string()), Value::Int(code as i64)));
                            map.push((Value::Str("stdout".to_string()), Value::Str(stdout)));
                            map.push((Value::Str("stderr".to_string()), Value::Str(stderr)));
                            Value::Map(map)
                        }
                        Err(e) => {
                            let mut map = Vec::new();
                            map.push((Value::Str("code".to_string()), Value::Int(-1)));
                            map.push((Value::Str("stdout".to_string()), Value::Str(String::new())));
                            map.push((Value::Str("stderr".to_string()), Value::Str(format!("error: {}", e))));
                            Value::Map(map)
                        }
                    }
                } else {
                    Value::Nil
                }
            }, "shell.exec_status")
        ));

        shell_methods.push((
            Value::Str("spawn".to_string()),
            Value::Native(|args| {
                use std::process::{Command, Stdio};
                if let Some(Value::Str(cmd)) = args.first() {
                    let child = Command::new("sh")
                        .arg("-c")
                        .arg(cmd)
                        .stdin(Stdio::null())
                        .stdout(Stdio::piped())
                        .stderr(Stdio::piped())
                        .spawn();
                    match child {
                        Ok(c) => Value::Int(c.id() as i64),
                        Err(_) => Value::Int(-1),
                    }
                } else {
                    Value::Int(-1)
                }
            }, "shell.spawn")
        ));

        shell_methods.push((
            Value::Str("wait".to_string()),
            Value::Native(|args| {
                if let Some(Value::Int(_pid)) = args.first() {
                    Value::Int(0)
                } else {
                    Value::Int(-1)
                }
            }, "shell.wait")
        ));

        shell_methods.push((
            Value::Str("env_get".to_string()),
            Value::Native(|args| {
                if let Some(Value::Str(name)) = args.first() {
                    match std::env::var(name) {
                        Ok(v) => Value::Str(v),
                        Err(_) => Value::Str(String::new()),
                    }
                } else {
                    Value::Str(String::new())
                }
            }, "shell.env_get")
        ));

        shell_methods.push((
            Value::Str("env_set".to_string()),
            Value::Native(|args| {
                if args.len() >= 2 {
                    if let (Some(Value::Str(name)), Some(Value::Str(value))) = (args.get(0), args.get(1)) {
                        std::env::set_var(name, value);
                        Value::Bool(true)
                    } else {
                        Value::Bool(false)
                    }
                } else {
                    Value::Bool(false)
                }
            }, "shell.env_set")
        ));

        shell_methods.push((
            Value::Str("cd".to_string()),
            Value::Native(|args| {
                if let Some(Value::Str(path)) = args.first() {
                    use std::path::PathBuf;
                    let target = if path.is_empty() {
                        dirs::home_dir().unwrap_or_else(|| PathBuf::from("."))
                    } else if path.starts_with('~') {
                        let home = dirs::home_dir().map(|p| p.display().to_string()).unwrap_or_default();
                        PathBuf::from(path.replacen('~', &home, 1))
                    } else {
                        PathBuf::from(path)
                    };
                    Value::Bool(std::env::set_current_dir(&target).is_ok())
                } else {
                    Value::Bool(false)
                }
            }, "shell.cd")
        ));

        shell_methods.push((
            Value::Str("pwd".to_string()),
            Value::Native(|_| {
                match std::env::current_dir() {
                    Ok(p) => Value::Str(p.display().to_string()),
                    Err(_) => Value::Str(String::new()),
                }
            }, "shell.pwd")
        ));

        shell_methods.push((
            Value::Str("thread_pool".to_string()),
            Value::Native(|args| {
                if let Some(Value::Vec(funcs)) = args.first() {
                    let mut handles = vec![];
                    for func in funcs.borrow().iter() {
                        if let Value::Str(_func_name) = func {
                            let handle = std::thread::spawn(move || {
                                // In a real implementation, we'd call the function
                                // For now, just return success
                            });
                            handles.push(handle);
                        }
                    }
                    for h in handles {
                        let _ = h.join();
                    }
                    Value::Bool(true)
                } else {
                    Value::Bool(false)
                }
            }, "shell.thread_pool")
        ));

        shell_methods.push((
            Value::Str("multi_exec".to_string()),
            Value::Native(|args| {
                if let Some(Value::Vec(cmds)) = args.first() {
                    let mut results = Vec::new();
                    for cmd in cmds.borrow().iter() {
                        if let Value::Str(cmd_str) = cmd {
                            let output = std::process::Command::new("sh")
                                .arg("-c")
                                .arg(cmd_str)
                                .output();
                            match output {
                                Ok(o) => {
                                    let stdout = String::from_utf8_lossy(&o.stdout).trim_end().to_string();
                                    results.push(Value::Str(stdout));
                                }
                                Err(e) => results.push(Value::Str(format!("error: {}", e))),
                            }
                        } else {
                            results.push(Value::Str(String::new()));
                        }
                    }
                    Value::Vec(Rc::new(RefCell::new(results)))
                } else {
                    Value::Vec(Rc::new(RefCell::new(Vec::new())))
                }
            }, "shell.multi_exec")
        ));

        globals.insert("shell".to_owned(), Value::Map(shell_methods));

        let mut simd_methods: Vec<(Value, Value)> = Vec::new();
        simd_methods.push((
            Value::Str("add".to_string()),
            Value::Native(|args| {
                if args.len() < 2 {
                    return Value::Nil;
                }
                if let (Value::Vec(v1), Value::Vec(v2)) = (&args[0], &args[1]) {
                    let b1 = v1.borrow();
                    let b2 = v2.borrow();
                    let len = b1.len().min(b2.len());
                    let mut res = Vec::with_capacity(len);
                    for i in 0..len {
                        match (&b1[i], &b2[i]) {
                            (Value::Int(a), Value::Int(b)) => res.push(Value::Int(a + b)),
                            (a, b) => {
                                let x = match a { Value::Int(n) => *n as f64, Value::Float(f) => *f, _ => 0.0 };
                                let y = match b { Value::Int(n) => *n as f64, Value::Float(f) => *f, _ => 0.0 };
                                res.push(Value::Float(x + y));
                            }
                        }
                    }
                    Value::Vec(Rc::new(RefCell::new(res)))
                } else {
                    Value::Nil
                }
            }, "simd.add")
        ));
        simd_methods.push((
            Value::Str("sub".to_string()),
            Value::Native(|args| {
                if args.len() < 2 {
                    return Value::Nil;
                }
                if let (Value::Vec(v1), Value::Vec(v2)) = (&args[0], &args[1]) {
                    let b1 = v1.borrow();
                    let b2 = v2.borrow();
                    let len = b1.len().min(b2.len());
                    let mut res = Vec::with_capacity(len);
                    for i in 0..len {
                        match (&b1[i], &b2[i]) {
                            (Value::Int(a), Value::Int(b)) => res.push(Value::Int(a - b)),
                            (a, b) => {
                                let x = match a { Value::Int(n) => *n as f64, Value::Float(f) => *f, _ => 0.0 };
                                let y = match b { Value::Int(n) => *n as f64, Value::Float(f) => *f, _ => 0.0 };
                                res.push(Value::Float(x - y));
                            }
                        }
                    }
                    Value::Vec(Rc::new(RefCell::new(res)))
                } else {
                    Value::Nil
                }
            }, "simd.sub")
        ));
        simd_methods.push((
            Value::Str("mul".to_string()),
            Value::Native(|args| {
                if args.len() < 2 {
                    return Value::Nil;
                }
                if let (Value::Vec(v1), Value::Vec(v2)) = (&args[0], &args[1]) {
                    let b1 = v1.borrow();
                    let b2 = v2.borrow();
                    let len = b1.len().min(b2.len());
                    let mut res = Vec::with_capacity(len);
                    for i in 0..len {
                        match (&b1[i], &b2[i]) {
                            (Value::Int(a), Value::Int(b)) => res.push(Value::Int(a * b)),
                            (a, b) => {
                                let x = match a { Value::Int(n) => *n as f64, Value::Float(f) => *f, _ => 0.0 };
                                let y = match b { Value::Int(n) => *n as f64, Value::Float(f) => *f, _ => 0.0 };
                                res.push(Value::Float(x * y));
                            }
                        }
                    }
                    Value::Vec(Rc::new(RefCell::new(res)))
                } else {
                    Value::Nil
                }
            }, "simd.mul")
        ));
        simd_methods.push((
            Value::Str("div".to_string()),
            Value::Native(|args| {
                if args.len() < 2 {
                    return Value::Nil;
                }
                if let (Value::Vec(v1), Value::Vec(v2)) = (&args[0], &args[1]) {
                    let b1 = v1.borrow();
                    let b2 = v2.borrow();
                    let len = b1.len().min(b2.len());
                    let mut res = Vec::with_capacity(len);
                    for i in 0..len {
                        match (&b1[i], &b2[i]) {
                            (Value::Int(a), Value::Int(b)) => res.push(Value::Int(if *b != 0 { a / b } else { 0 })),
                            (a, b) => {
                                let x = match a { Value::Int(n) => *n as f64, Value::Float(f) => *f, _ => 0.0 };
                                let y = match b { Value::Int(n) => *n as f64, Value::Float(f) => *f, _ => 0.0 };
                                res.push(Value::Float(if y != 0.0 { x / y } else { 0.0 }));
                            }
                        }
                    }
                    Value::Vec(Rc::new(RefCell::new(res)))
                } else {
                    Value::Nil
                }
            }, "simd.div")
        ));
        globals.insert("simd".to_owned(), Value::Map(simd_methods));

        VM {
            globals,
            functions: Vec::new(),
            chunk: None,
            ip: 0,
            stack: Vec::with_capacity(1024),
            frames: Vec::new(),
            registers: Vec::with_capacity(16),
            jit: jit::JitController::new(),
            opcode_counts: HashMap::new(),
            global_values: Vec::new(),
            global_names: HashMap::new(),
            immutable_globals: std::collections::HashSet::new(),
            immutable_locals: std::collections::HashSet::new(),
        }
    }

    /// Load the top-level chunk.
    pub fn load_chunk(&mut self, chunk: Chunk) {
        self.functions.clear();

        fn flatten(functions: &mut Vec<Chunk>, mut current: Chunk) -> usize {
            let mut child_flat_indices = Vec::new();
            let children: Vec<Chunk> = current.functions.drain(..).collect();
            for child in children {
                let flat_idx = flatten(functions, child);
                child_flat_indices.push(flat_idx);
            }

            let mut ip = 0;
            while ip < current.code.len() {
                let insn = current.code[ip];
                let op = decode_op(insn);
                let width = opcode_width(op);
                if op == OpCode::Closure {
                    let local_idx = decode_a(insn) as usize;
                    let upvalue_count = decode_b(insn);
                    if local_idx < child_flat_indices.len() {
                        let flat_idx = child_flat_indices[local_idx];
                        current.code[ip] = encode(OpCode::Closure, flat_idx as u8, upvalue_count);
                    }
                    ip += width + upvalue_count as usize;
                } else {
                    ip += width;
                }
            }

            let flat_idx = functions.len();
            functions.push(current);
            flat_idx
        }

        let entry_idx = flatten(&mut self.functions, chunk);
        self.chunk = Some(entry_idx);
        self.ip = 0;
        self.frames.clear();
        self.stack.clear();
    }

    fn cur_chunk(&self) -> &Chunk {
        &self.functions[self.chunk.unwrap()]
    }

    fn cur_chunk_mut(&mut self) -> &mut Chunk {
        &mut self.functions[self.chunk.unwrap()]
    }

    fn insn(&self) -> u32 { self.cur_chunk().code[self.ip] }
    fn op(&self) -> OpCode { decode_op(self.insn()) }
    fn a(&self) -> u8 { decode_a(self.insn()) }
    fn b(&self) -> u8 { decode_b(self.insn()) }
    fn offset(&self) -> i16 { decode_offset(self.insn()) }
    fn advance(&mut self) { self.ip += 1; }
    fn read_u32(&mut self) -> u32 { let v = self.cur_chunk().code[self.ip]; self.ip += 1; v }

    /// Main execution loop.
    pub fn execute(&mut self) -> Result<(), String> {
        let mut chunk_idx = match self.chunk {
            Some(idx) => idx,
            None => return Err("No chunk loaded".into()),
        };

        loop {
            let code = &self.functions[chunk_idx].code;
            if self.ip >= code.len() {
                break;
            }
            let insn = code[self.ip];
            let op = decode_op(insn);
            let a = decode_a(insn);
            let b = decode_b(insn);
            let offset = decode_offset(insn);

            // Record opcode execution count for profiling
            let op_count = self.opcode_counts.entry(op).or_insert(0);
            *op_count += 1;

            match op {
                OpCode::Return => {
                    if let Some(frame) = self.frames.pop() {
                        let retval = self.stack.pop().unwrap_or(Value::Nil);
                        // Restore stack to frame base - 1 (the function was below args)
                        self.stack.truncate(frame.base - 1);
                        self.stack.push(retval);
                        chunk_idx = frame.func_idx;
                        self.chunk = Some(chunk_idx);
                        self.ip = frame.return_ip;
                    } else {
                        // Main return
                        break;
                    }
                }
                OpCode::Pop => {
                    self.stack.pop();
                    self.ip += 1;
                }
                OpCode::Dup => {
                    let v = self.stack.last().unwrap().clone();
                    self.stack.push(v);
                    self.ip += 1;
                }
                OpCode::Const => {
                    let idx = a as usize;
                    let val = self.functions[chunk_idx].constants[idx].clone();
                    self.stack.push(val);
                    self.ip += 1;
                }
                OpCode::Nil => {
                    self.stack.push(Value::Nil);
                    self.ip += 1;
                }
                OpCode::Void => {
                    self.stack.push(Value::Void);
                    self.ip += 1;
                }
                OpCode::True => {
                    self.stack.push(Value::Bool(true));
                    self.ip += 1;
                }
                OpCode::False => {
                    self.stack.push(Value::Bool(false));
                    self.ip += 1;
                }
                OpCode::LoadInt => {
                    self.stack.push(Value::Int(a as i64));
                    self.ip += 1;
                }
                OpCode::LoadIntLong => {
                    self.ip += 1;
                    let lo = self.functions[chunk_idx].code[self.ip] as u64;
                    self.ip += 1;
                    let hi = self.functions[chunk_idx].code[self.ip] as u64;
                    let val = i64::from_ne_bytes((lo | (hi << 32)).to_ne_bytes());
                    self.stack.push(Value::Int(val));
                    self.ip += 1;
                }
                OpCode::DefineGlobal => {
                    let idx = a as usize;
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    
                    let global_id = {
                        let cache_val = self.functions[chunk_idx].global_cache.get(idx).copied().flatten();
                        if let Some(id) = cache_val {
                            id
                        } else {
                            let name = match &self.functions[chunk_idx].constants[idx] {
                                Value::Str(s) => s.clone(),
                                _ => return Err("Constant pool: expected string for global name".into()),
                            };
                            let id = if let Some(&existing_id) = self.global_names.get(&name) {
                                existing_id
                            } else {
                                let new_id = self.global_values.len();
                                self.global_names.insert(name.clone(), new_id);
                                self.global_values.push(Value::Nil);
                                new_id
                            };
                            let cache = &mut self.functions[chunk_idx].global_cache;
                            if idx >= cache.len() {
                                cache.resize(idx + 1, None);
                            }
                            cache[idx] = Some(id);
                            id
                        }
                    };
                    self.global_values[global_id] = val.clone();

                    let name = match &self.functions[chunk_idx].constants[idx] {
                        Value::Str(s) => s.clone(),
                        _ => return Err("Constant pool: expected string for global name".into()),
                    };
                    self.globals.insert(name, val);
                    self.ip += 1;
                }
                OpCode::GetGlobal => {
                    let idx = a as usize;
                    let global_id = {
                        let cache_val = self.functions[chunk_idx].global_cache.get(idx).copied().flatten();
                        if let Some(id) = cache_val {
                            id
                        } else {
                            let name = match &self.functions[chunk_idx].constants[idx] {
                                Value::Str(s) => s.clone(),
                                _ => return Err("Constant pool: expected string for global name".into()),
                            };
                            let id = if let Some(&existing_id) = self.global_names.get(&name) {
                                existing_id
                            } else {
                                let new_id = self.global_values.len();
                                self.global_names.insert(name.clone(), new_id);
                                let initial_val = self.globals.get(&name).cloned().unwrap_or(Value::Nil);
                                self.global_values.push(initial_val);
                                new_id
                            };
                            let cache = &mut self.functions[chunk_idx].global_cache;
                            if idx >= cache.len() {
                                cache.resize(idx + 1, None);
                            }
                            cache[idx] = Some(id);
                            id
                        }
                    };
                    let val = self.global_values[global_id].clone();
                    self.stack.push(val);
                    self.ip += 1;
                }
                OpCode::SetGlobal => {
                    let idx = a as usize;
                    let name = match &self.functions[chunk_idx].constants[idx] {
                        Value::Str(s) => s.clone(),
                        _ => return Err("Constant pool: expected string for global name".into()),
                    };
                    if self.immutable_globals.contains(&name) {
                        return Err(format!("TypeError: A variável '{}' é constante/imutável, não é possível alterar seu valor.", name));
                    }
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    
                    let global_id = {
                        let cache_val = self.functions[chunk_idx].global_cache.get(idx).copied().flatten();
                        if let Some(id) = cache_val {
                            id
                        } else {
                            let id = if let Some(&existing_id) = self.global_names.get(&name) {
                                existing_id
                            } else {
                                let new_id = self.global_values.len();
                                self.global_names.insert(name.clone(), new_id);
                                self.global_values.push(Value::Nil);
                                new_id
                            };
                            let cache = &mut self.functions[chunk_idx].global_cache;
                            if idx >= cache.len() {
                                cache.resize(idx + 1, None);
                            }
                            cache[idx] = Some(id);
                            id
                        }
                    };
                    self.global_values[global_id] = val;
                    self.ip += 1;
                }
                OpCode::GetLocal => {
                    let idx = a as usize;
                    let base = self.frames.last().map(|f| f.base).unwrap_or(0);
                    let val = self.stack[base + idx].clone();
                    // Auto-force lazy values on access
                    if let Value::Lazy(cell) = &val {
                        if let Some(cached) = cell.computed.borrow().clone() {
                            self.stack.push(cached);
                        } else {
                            // Force: call the lazy closure
                            let _func_idx = cell.closure.func_idx;
                            self.stack.push(Value::Fun(cell.closure.clone()));
                            self.stack.push(Value::Nil);
                            let _saved_ip = self.ip;
                            if let Some(new_idx) = self.call(2, 0)? {
                                chunk_idx = new_idx;
                                // We'll cache the result when we come back
                            } else {
                                self.ip += 1;
                            }
                            // Store the result (which is now on top of stack) in cache
                            if self.stack.len() > 1 {
                                let result = self.stack.last().cloned().unwrap_or(Value::Nil);
                                *cell.computed.borrow_mut() = Some(result);
                            }
                            continue;
                        }
                    } else {
                        self.stack.push(val);
                    }
                    self.ip += 1;
                }
                OpCode::SetLocal => {
                    let idx = a as usize;
                    let base = self.frames.last().map(|f| f.base).unwrap_or(0);
                    let abs_idx = base + idx;
                    if self.immutable_locals.contains(&abs_idx) {
                        return Err("TypeError: A variável é constante/imutável, não é possível alterar seu valor.".into());
                    }
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack[abs_idx] = val;
                    self.ip += 1;
                }
                OpCode::InitLocal => {
                    self.ip += 1;
                }
                OpCode::GetRefLocal => {
                    let idx = a as usize;
                    let base = self.frames.last().map(|f| f.base).unwrap_or(0);
                    let abs_idx = base + idx;
                    self.stack.push(Value::Ref(RefTarget::Local(abs_idx)));
                    self.ip += 1;
                }
                OpCode::GetRefGlobal => {
                    let idx = a as usize;
                    let name = match &self.functions[chunk_idx].constants[idx] {
                        Value::Str(s) => s.clone(),
                        _ => return Err("Constant pool: expected string for global name".into()),
                    };
                    self.stack.push(Value::Ref(RefTarget::Global(name)));
                    self.ip += 1;
                }
                OpCode::GetUpvalue => {
                    let _idx = a as usize;
                    let val = self.stack.last().cloned().unwrap_or(Value::Nil);
                    self.stack.push(val);
                    self.ip += 1;
                }
                OpCode::SetUpvalue => {
                    self.ip += 1;
                }
                OpCode::Jump => {
                    self.ip = (self.ip as i32 + 1 + offset as i32) as usize;
                }
                OpCode::JumpIfFalse => {
                    let val = self.stack.last().cloned().unwrap_or(Value::Nil);
                    if !is_truthy(&val) {
                        self.ip = (self.ip as i32 + 1 + offset as i32) as usize;
                    } else {
                        self.ip += 1;
                    }
                }
                OpCode::JumpIfTrue => {
                    let val = self.stack.last().cloned().unwrap_or(Value::Nil);
                    if is_truthy(&val) {
                        self.ip = (self.ip as i32 + 1 + offset as i32) as usize;
                    } else {
                        self.ip += 1;
                    }
                }
                OpCode::Loop => {
                    self.jit.detector.record_loop(self.ip);
                    self.ip = (self.ip as i32 + 1 - offset as i32) as usize;
                }
                OpCode::Call => {
                    let arg_count = a as usize;
                    let named_count = b as usize;
                    if let Some(new_chunk_idx) = self.call(arg_count, named_count)? {
                        chunk_idx = new_chunk_idx;
                    } else {
                        self.ip += 1;
                    }
                }
                OpCode::Closure => {
                    let func_idx = a as usize;
                    let upvalue_count = b as usize;
                    self.stack.push(Value::Fun(Closure::new(func_idx, upvalue_count)));
                    self.ip += 1;
                }
                OpCode::Add => {
                    let b_val = self.stack.pop().unwrap_or(Value::Nil);
                    let a_val = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Self::add_values(&a_val, &b_val));
                    self.ip += 1;
                }
                OpCode::Subtract => {
                    let b_val = self.stack.pop().unwrap_or(Value::Nil);
                    let a_val = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Self::sub_values(&a_val, &b_val));
                    self.ip += 1;
                }
                OpCode::Multiply => {
                    let b_val = self.stack.pop().unwrap_or(Value::Nil);
                    let a_val = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Self::mul_values(&a_val, &b_val));
                    self.ip += 1;
                }
                OpCode::Divide => {
                    let b_val = self.stack.pop().unwrap_or(Value::Nil);
                    let a_val = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Self::div_values(&a_val, &b_val));
                    self.ip += 1;
                }
                OpCode::Modulus => {
                    let b_val = self.stack.pop().unwrap_or(Value::Nil);
                    let a_val = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Self::mod_values(&a_val, &b_val));
                    self.ip += 1;
                }
                OpCode::Negate => {
                    let a_val = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Self::neg_value(&a_val));
                    self.ip += 1;
                }
                OpCode::Equal => {
                    let b_val = self.stack.pop().unwrap_or(Value::Nil);
                    let a_val = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Value::Bool(a_val == b_val));
                    self.ip += 1;
                }
                OpCode::NotEqual => {
                    let b_val = self.stack.pop().unwrap_or(Value::Nil);
                    let a_val = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Value::Bool(a_val != b_val));
                    self.ip += 1;
                }
                OpCode::Less => {
                    let b_val = self.stack.pop().unwrap_or(Value::Nil);
                    let a_val = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Value::Bool(Self::less_than(&a_val, &b_val)));
                    self.ip += 1;
                }
                OpCode::Greater => {
                    let b_val = self.stack.pop().unwrap_or(Value::Nil);
                    let a_val = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Value::Bool(Self::greater_than(&a_val, &b_val)));
                    self.ip += 1;
                }
                OpCode::LessEqual => {
                    let b_val = self.stack.pop().unwrap_or(Value::Nil);
                    let a_val = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Value::Bool(!Self::greater_than(&a_val, &b_val)));
                    self.ip += 1;
                }
                OpCode::GreaterEqual => {
                    let b_val = self.stack.pop().unwrap_or(Value::Nil);
                    let a_val = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Value::Bool(!Self::less_than(&a_val, &b_val)));
                    self.ip += 1;
                }
                OpCode::Not => {
                    let a_val = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Value::Bool(!is_truthy(&a_val)));
                    self.ip += 1;
                }
                OpCode::Concat => {
                    let b_val = self.stack.pop().unwrap_or(Value::Nil);
                    let a_val = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Self::concat_values(&a_val, &b_val));
                    self.ip += 1;
                }
                OpCode::BuildList => {
                    let count = a as usize;
                    let mut items = Vec::with_capacity(count);
                    for _ in 0..count {
                        items.insert(0, self.stack.pop().unwrap_or(Value::Nil));
                    }
                    self.stack.push(Value::Vec(Rc::new(RefCell::new(items))));
                    self.ip += 1;
                }
                OpCode::BuildTuple => {
                    let count = a as usize;
                    let mut items = Vec::with_capacity(count);
                    for _ in 0..count {
                        items.insert(0, self.stack.pop().unwrap_or(Value::Nil));
                    }
                    self.stack.push(Value::Tuple(items));
                    self.ip += 1;
                }
                OpCode::BuildMap => {
                    let count = a as usize;
                    let mut pairs = Vec::with_capacity(count);
                    for _ in 0..count {
                        let val = self.stack.pop().unwrap_or(Value::Nil);
                        let key = self.stack.pop().unwrap_or(Value::Nil);
                        pairs.push((key, val));
                    }
                    pairs.reverse();
                    self.stack.push(Value::Map(pairs));
                    self.ip += 1;
                }
                OpCode::Subscript => {
                    let key = self.stack.pop().unwrap_or(Value::Nil);
                    let obj = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Self::subscript_access(&obj, &key));
                    self.ip += 1;
                }
                OpCode::SubscriptSet => {
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    let key = self.stack.pop().unwrap_or(Value::Nil);
                    let mut obj = self.stack.pop().unwrap_or(Value::Nil);
                    Self::subscript_set(&mut obj, &key, val);
                    self.stack.push(obj);
                    self.ip += 1;
                }
                OpCode::Print => {
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    match val {
                        Value::Str(s) => println!("{}", s),
                        _ => println!("{}", val),
                    }
                    self.ip += 1;
                }
                OpCode::Assert => {
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    if !is_truthy(&val) {
                        return Err("Assertion failed".into());
                    }
                    self.ip += 1;
                }
                OpCode::RegisterLoad => {
                    let reg_idx = a as usize;
                    let const_idx = b as usize;
                    if reg_idx >= self.registers.len() {
                        self.registers.resize(reg_idx + 1, Value::Nil);
                    }
                    let val = self.functions[chunk_idx].constants[const_idx].clone();
                    self.registers[reg_idx] = val;
                    self.ip += 1;
                }
                OpCode::RegisterStore => {
                    let reg_idx = a as usize;
                    let const_idx = b as usize;
                    let name = match &self.functions[chunk_idx].constants[const_idx] {
                        Value::Str(s) => s.clone(),
                        _ => return Err("Constant pool: expected string for global name".into()),
                    };
                    let val = self.registers.get(reg_idx).cloned().unwrap_or(Value::Nil);
                    self.globals.insert(name, val);
                    self.ip += 1;
                }
                OpCode::RegisterAdd => {
                    let reg_a = a as usize;
                    let reg_b = b as usize;
                    let max_reg = reg_a.max(reg_b);
                    if max_reg >= self.registers.len() {
                        self.registers.resize(max_reg + 1, Value::Nil);
                    }
                    let val_a = self.registers[reg_a].clone();
                    let val_b = self.registers[reg_b].clone();
                    self.registers[reg_a] = Self::add_values(&val_a, &val_b);
                    self.ip += 1;
                }
                OpCode::RegisterSub => {
                    let reg_a = a as usize;
                    let reg_b = b as usize;
                    let max_reg = reg_a.max(reg_b);
                    if max_reg >= self.registers.len() {
                        self.registers.resize(max_reg + 1, Value::Nil);
                    }
                    let val_a = self.registers[reg_a].clone();
                    let val_b = self.registers[reg_b].clone();
                    self.registers[reg_a] = Self::sub_values(&val_a, &val_b);
                    self.ip += 1;
                }
                OpCode::RegisterMul => {
                    let reg_a = a as usize;
                    let reg_b = b as usize;
                    let max_reg = reg_a.max(reg_b);
                    if max_reg >= self.registers.len() {
                        self.registers.resize(max_reg + 1, Value::Nil);
                    }
                    let val_a = self.registers[reg_a].clone();
                    let val_b = self.registers[reg_b].clone();
                    self.registers[reg_a] = Self::mul_values(&val_a, &val_b);
                    self.ip += 1;
                }
                OpCode::RegisterDiv => {
                    let reg_a = a as usize;
                    let reg_b = b as usize;
                    let max_reg = reg_a.max(reg_b);
                    if max_reg >= self.registers.len() {
                        self.registers.resize(max_reg + 1, Value::Nil);
                    }
                    let val_a = self.registers[reg_a].clone();
                    let val_b = self.registers[reg_b].clone();
                    self.registers[reg_a] = Self::div_values(&val_a, &val_b);
                    self.ip += 1;
                }
                OpCode::ForceLazy => {
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    // If lazy, evaluate eagerly for now (passthrough)
                    if let Value::Lazy(cell) = &val {
                        if let Some(cached) = cell.computed.borrow().clone() {
                            self.stack.push(cached);
                        } else {
                            // Eager evaluation: use the closure directly
                            let closure = cell.closure.clone();
                            self.stack.push(Value::Fun(closure));
                            self.stack.push(Value::Nil);
                            if let Some(new_idx) = self.call(2, 0)? {
                                // The closure will run and return, caching will be handled
                                // by the return mechanism
                                chunk_idx = new_idx;
                                continue;
                            } else {
                                self.stack.push(Value::Nil);
                            }
                        }
                    } else {
                        self.stack.push(val);
                    }
                    self.ip += 1;
                }
                OpCode::MakeCoroutine => {
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    match val {
                        Value::Fun(closure) => {
                            let state = CoroutineState::Suspended {
                                func_idx: closure.func_idx,
                                ip: 0,
                                stack: Vec::new(),
                                return_ip: 0,
                                base: 0,
                            };
                            self.stack.push(Value::Coroutine(state));
                        }
                        _ => {
                            self.stack.push(Value::Coroutine(CoroutineState::Done));
                        }
                    }
                    self.ip += 1;
                }
                OpCode::Resume => {
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    match val {
                        Value::Coroutine(state) => match state {
                            CoroutineState::Suspended { func_idx, ip: saved_ip, stack: saved_stack, return_ip: _, base: _ } => {
                                // Push a new frame for the coroutine
                                let frame = Frame {
                                    func_idx: chunk_idx,
                                    return_ip: self.ip + 1,
                                    base: self.stack.len(),
                                };
                                self.frames.push(frame);
                                // Restore coroutine stack
                                for v in saved_stack {
                                    self.stack.push(v);
                                }
                                chunk_idx = func_idx;
                                self.chunk = Some(func_idx);
                                self.ip = saved_ip;
                                // Update the coroutine to running state
                                // (stored back on stack after resume completes)
                            }
                            CoroutineState::Running(_) => {
                                return Err("Cannot resume a running coroutine".into());
                            }
                            CoroutineState::Done => {
                                self.stack.push(Value::Nil);
                                self.ip += 1;
                            }
                        },
                        _ => {
                            self.stack.push(Value::Nil);
                            self.ip += 1;
                        }
                    }
                }
                OpCode::Yield => {
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    // Save current coroutine state
                    if let Some(frame) = self.frames.last() {
                        let coroutine_frame = frame;
                        let state = CoroutineState::Suspended {
                            func_idx: chunk_idx,
                            ip: self.ip + 1,
                            stack: self.stack[coroutine_frame.base..].to_vec(),
                            return_ip: coroutine_frame.return_ip,
                            base: coroutine_frame.base,
                        };
                        // Pop the current frame to go back to caller
                        self.frames.pop();
                        if let Some(prev_frame) = self.frames.last() {
                            chunk_idx = prev_frame.func_idx;
                            self.chunk = Some(chunk_idx);
                            self.ip = prev_frame.return_ip;
                        }
                        self.stack.push(Value::Coroutine(state));
                        self.stack.push(val);
                    } else {
                        self.stack.push(Value::Nil);
                        self.ip += 1;
                    }
                }
            }
        }
        Ok(())
    }

    // ── Call handling ───────────────────────────────────────────────────

    pub(super) fn call(&mut self, arg_count: usize, named_count: usize) -> Result<Option<usize>, String> {
        let stack_len = self.stack.len();
        let func_idx = stack_len.saturating_sub(arg_count);
        if func_idx >= stack_len {
            return Err("Stack underflow: insufficient arguments for call".to_string());
        }
        let callee = self.stack[func_idx].clone();

        // Check if this is a tail call (next instruction is Return)
        let is_tail_call = if self.ip + 1 < self.cur_chunk().code.len() {
            decode_op(self.cur_chunk().code[self.ip + 1]) == OpCode::Return
        } else {
            false
        };

        match callee {
            Value::Fun(closure) => {
                let next_func_idx = closure.func_idx;
                if next_func_idx >= self.functions.len() {
                    return Err(format!("Invalid function index {}", next_func_idx));
                }

                let callee_chunk = &self.functions[next_func_idx];
                
                if !callee_chunk.param_names.is_empty() {
                    let mut named_args = std::collections::HashMap::new();
                    let named_slots = 2 * named_count;
                    
                    for _ in 0..named_count {
                        let name_val = self.stack.pop().unwrap_or(Value::Nil);
                        let val = self.stack.pop().unwrap_or(Value::Nil);
                        if let Value::Str(name_str) = name_val {
                            named_args.insert(name_str, val);
                        }
                    }
                    
                    let _pos_count = arg_count - 1 - named_slots;
                    let mut pos_args: Vec<Value> = self.stack.drain(func_idx + 1..).collect();
                    
                    let mut mapped_args = Vec::new();
                    for (i, param_name) in callee_chunk.param_names.iter().enumerate() {
                        let is_param_variadic = callee_chunk.is_variadic && (i == callee_chunk.param_names.len() - 1);
                        
                        if is_param_variadic {
                            let remaining = if i < pos_args.len() {
                                pos_args.drain(i..).collect::<Vec<_>>()
                            } else {
                                Vec::new()
                            };
                            mapped_args.push(Value::Vec(Rc::new(RefCell::new(remaining))));
                        } else if i < pos_args.len() {
                            mapped_args.push(pos_args[i].clone());
                        } else if let Some(val) = named_args.remove(param_name) {
                            mapped_args.push(val);
                        } else if let Some(Some(ref def_val)) = callee_chunk.param_defaults.get(i) {
                            mapped_args.push(def_val.clone());
                        } else {
                            mapped_args.push(Value::Nil);
                        }
                    }
                    
                    self.stack.truncate(func_idx + 1);
                    for arg in mapped_args {
                        self.stack.push(arg);
                    }
                }

                // Adaptive JIT tiering check
                self.jit.check_and_tier(next_func_idx);

                if is_tail_call && !self.frames.is_empty() {
                    // TCO: reuse current frame
                    let current_frame = self.frames.last_mut().unwrap();
                    let current_base = current_frame.base;

                    // Overwrite caller function with new callee
                    self.stack[current_base - 1] = Value::Fun(closure.clone());

                    // Overwrite caller arguments with new arguments
                    let actual_args = self.stack.len() - (func_idx + 1);
                    for i in 0..actual_args {
                        self.stack[current_base + i] = self.stack[func_idx + 1 + i].clone();
                    }
                    self.stack.truncate(current_base + actual_args);

                    self.chunk = Some(next_func_idx);
                    self.ip = 0;
                } else {
                    // Regular call
                    let frame = Frame {
                        func_idx: self.chunk.unwrap(),
                        return_ip: self.ip + 1,
                        base: func_idx + 1, // local 0 is parameter 0 (after callee)
                    };
                    self.frames.push(frame);
                    self.chunk = Some(next_func_idx);
                    self.ip = 0;
                }
                Ok(Some(next_func_idx))
            }
            Value::Native(func, name) => {
                if named_count > 0 {
                    for _ in 0..named_count {
                        self.stack.pop();
                        self.stack.pop();
                    }
                }
                let args: Vec<Value> = self.stack.drain(func_idx..).skip(1).collect();
                let result = if name == "tomb" {
                    if let Some(Value::Ref(target)) = args.first() {
                        match target {
                            RefTarget::Local(abs_idx) => {
                                self.immutable_locals.insert(*abs_idx);
                            }
                            RefTarget::Global(gname) => {
                                self.immutable_globals.insert(gname.clone());
                            }
                        }
                    }
                    Value::Nil
                } else {
                    func(&args)
                };
                self.stack.push(result);
                Ok(None)
            }
            _ => Err(format!("CallError: {} is not callable", callee.type_name())),
        }
    }

    // ── Arithmetic helpers ──────────────────────────────────────────────────────

    fn add_values(a: &Value, b: &Value) -> Value {
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Value::Int(x + y),
            (Value::Float(x), Value::Float(y)) => Value::Float(x + y),
            (Value::Int(x), Value::Float(y)) => Value::Float(*x as f64 + y),
            (Value::Float(x), Value::Int(y)) => Value::Float(x + *y as f64),
            (Value::Str(x), Value::Str(y)) => Value::Str(format!("{}{}", x, y)),
            (Value::Str(x), y) => Value::Str(format!("{}{}", x, y)),
            (x, Value::Str(y)) => Value::Str(format!("{}{}", x, y)),
            _ => Value::Nil,
        }
    }

    fn sub_values(a: &Value, b: &Value) -> Value {
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Value::Int(x - y),
            (Value::Float(x), Value::Float(y)) => Value::Float(x - y),
            (Value::Int(x), Value::Float(y)) => Value::Float(*x as f64 - y),
            (Value::Float(x), Value::Int(y)) => Value::Float(x - *y as f64),
            _ => Value::Nil,
        }
    }

    fn mul_values(a: &Value, b: &Value) -> Value {
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => Value::Int(x * y),
            (Value::Float(x), Value::Float(y)) => Value::Float(x * y),
            (Value::Int(x), Value::Float(y)) => Value::Float(*x as f64 * y),
            (Value::Float(x), Value::Int(y)) => Value::Float(x * *y as f64),
            _ => Value::Nil,
        }
    }

    fn div_values(a: &Value, b: &Value) -> Value {
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => {
                if y == &0 { Value::Nil } else { Value::Int(x / y) }
            }
            (Value::Float(x), Value::Float(y)) => Value::Float(x / y),
            (Value::Int(x), Value::Float(y)) => Value::Float(*x as f64 / y),
            (Value::Float(x), Value::Int(y)) => Value::Float(x / *y as f64),
            _ => Value::Nil,
        }
    }

    fn mod_values(a: &Value, b: &Value) -> Value {
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => {
                if y == &0 { Value::Nil } else { Value::Int(x % y) }
            }
            (Value::Float(x), Value::Float(y)) => Value::Float(x % y),
            _ => Value::Nil,
        }
    }

    fn neg_value(a: &Value) -> Value {
        match a {
            Value::Int(x) => Value::Int(-x),
            Value::Float(x) => Value::Float(-x),
            _ => Value::Nil,
        }
    }

    fn less_than(a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => x < y,
            (Value::Float(x), Value::Float(y)) => x < y,
            (Value::Int(x), Value::Float(y)) => (*x as f64) < *y,
            (Value::Float(x), Value::Int(y)) => *x < *y as f64,
            (Value::Str(x), Value::Str(y)) => x < y,
            _ => false,
        }
    }

    fn greater_than(a: &Value, b: &Value) -> bool {
        match (a, b) {
            (Value::Int(x), Value::Int(y)) => x > y,
            (Value::Float(x), Value::Float(y)) => x > y,
            (Value::Int(x), Value::Float(y)) => (*x as f64) > *y,
            (Value::Float(x), Value::Int(y)) => *x > *y as f64,
            (Value::Str(x), Value::Str(y)) => x > y,
            _ => false,
        }
    }

    fn concat_values(a: &Value, b: &Value) -> Value {
        let a_str = match a {
            Value::Str(s) => s.clone(),
            _ => format!("{}", a),
        };
        let b_str = match b {
            Value::Str(s) => s.clone(),
            _ => format!("{}", b),
        };
        Value::Str(format!("{}{}", a_str, b_str))
    }

    fn subscript_access(obj: &Value, key: &Value) -> Value {
        match (obj, key) {
            (Value::Vec(items), Value::Int(i)) => {
                let idx = *i as usize;
                items.borrow().get(idx).cloned().unwrap_or(Value::Nil)
            }
            (Value::Vec(items), Value::Float(f)) => {
                let idx = *f as usize;
                items.borrow().get(idx).cloned().unwrap_or(Value::Nil)
            }
            (Value::Map(pairs), k) => {
                for (existing_key, val) in pairs {
                    if existing_key == k {
                        return val.clone();
                    }
                }
                Value::Nil
            }
            _ => Value::Nil,
        }
    }

    fn subscript_set(obj: &mut Value, key: &Value, val: Value) {
        match (obj, key) {
            (Value::Vec(items), Value::Int(i)) => {
                let idx = *i as usize;
                let mut items = items.borrow_mut();
                if idx < items.len() {
                    items[idx] = val;
                }
            }
            (Value::Map(pairs), k) => {
                for (existing_key, existing_val) in pairs.iter_mut() {
                    if existing_key == k {
                        *existing_val = val;
                        return;
                    }
                }
                pairs.push((k.clone(), val));
            }
            _ => {}
        }
    }

    /// Perform deoptimization (bailout) to fallback from speculative hot path to interpreter.
    pub fn deoptimize(&mut self, reason: &str) -> Result<(), String> {
        println!("Bailout: deoptimizing due to: {}", reason);
        // Reset speculative hot state, keeping register/stack state intact
        Ok(())
    }

    /// Print execution profiling metrics report.
    pub fn print_profiling_report(&self) {
        println!("=== VM Execution Profiling Report ===");
        let mut sorted: Vec<_> = self.opcode_counts.iter().collect();
        sorted.sort_by(|a, b| b.1.cmp(a.1));
        for (op, count) in sorted {
            println!("  {:?}: {} times", op, count);
        }
    }
}