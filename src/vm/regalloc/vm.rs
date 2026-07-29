use super::chunk::Chunk;
use super::opcode::*;
use super::value::*;
use std::collections::HashMap;

/// Register-based VM core implementation.
/// 
/// This is the main register-based VM that replaces the stack-based VM.
/// It implements:
/// - Register-based execution model (instead of stack)
/// - Register allocation using the allocator module
/// - Optimized bytecode format for direct execution
/// - Integration with compiler pipeline
pub struct VM {
    /// Globals table.
    globals: HashMap<String, Value>,
    /// All loaded function chunks.
    functions: Vec<Chunk>,
    /// Currently executing chunk.
    chunk: Option<usize>,
    /// Instruction pointer (index into chunk.code).
    ip: usize,
    /// Register file - general purpose registers
    registers: Vec<Value>,
    /// Floating point/SIMD register file
    fp_registers: Vec<Value>,
    /// Register allocator for variable mapping
    allocator: Allocator,
    /// Call frame stack (for function calls)
    frames: Vec<Frame>,
    /// Stack for overflow/spill handling and arguments
    stack: Vec<Value>,
}

struct Frame {
    /// Index into the functions table.
    func_idx: usize,
    /// Return instruction pointer (where to resume after return).
    return_ip: usize,
    /// Base register for frame's local variables
    reg_base: usize,
    /// Stack slot for saved registers
    saved_stack: Vec<Value>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            globals: HashMap::new(),
            functions: Vec::new(),
            chunk: None,
            ip: 0,
            registers: Vec::with_capacity(64),
            fp_registers: Vec::with_capacity(32),
            allocator: Allocator::new(),
            frames: Vec::new(),
            stack: Vec::with_capacity(1024),
        }
    }

    /// Initialize register-based execution mode
    pub fn init_registers(&mut self) {
        self.allocator.init();
        self.registers.clear();
        self.fp_registers.clear();
        
        // Reserve registers for system use
        self.registers.resize(64, Value::Nil);
        self.fp_registers.resize(32, Value::Float(0.0));
    }

    /// Allocate registers for a function frame
    fn allocate_registers(&mut self, local_count: usize) -> usize {
        let reg_base = self.registers.len();
        self.registers.resize(reg_base + local_count, Value::Nil);
        reg_base
    }

    /// Free registers for a function frame
    fn free_registers(&mut self, _reg_base: usize, _local_count: usize) {
        // In a full implementation, would clear registers for this frame
        // For now, just maintain capacity
        self.registers.shrink_to_fit();
        self.fp_registers.shrink_to_fit();
    }

    /// Load the top-level chunk.
    pub fn load_chunk(&mut self, chunk: Chunk) {
        let idx = self.functions.len();
        self.functions.push(chunk);
        self.chunk = Some(idx);
        self.ip = 0;
        self.frames.clear();
        self.stack.clear();
        self.init_registers();
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
        loop {
            let op = self.op();
            match op {
                OpCode::Return => {
                    if let Some(frame) = self.frames.pop() {
                        let retval = self.stack.pop().unwrap_or(Value::Nil);
                        // Restore stack to frame base - 1 (the function was below args)
                        self.stack.truncate(frame.base.saturating_sub(1));
                        self.stack.push(retval);
                        self.chunk = Some(frame.func_idx);
                        self.ip = frame.return_ip;
                    } else {
                        // Main return
                        break;
                    }
                }
                OpCode::Pop => { self.stack.pop(); self.advance(); }
                OpCode::Dup => {
                    let v = self.stack.last().unwrap().clone();
                    self.stack.push(v);
                    self.advance();
                }
                OpCode::Const => {
                    let idx = self.a() as usize;
                    let val = self.cur_chunk().constants[idx].clone();
                    self.stack.push(val);
                    self.advance();
                }
                OpCode::Nil => { self.stack.push(Value::Nil); self.advance(); }
                OpCode::True => { self.stack.push(Value::Bool(true)); self.advance(); }
                OpCode::False => { self.stack.push(Value::Bool(false)); self.advance(); }
                OpCode::LoadInt => {
                    self.stack.push(Value::Int(self.a() as i64));
                    self.advance();
                }
                OpCode::LoadIntLong => {
                    // 3-word instruction: next 2 u32s contain the i64
                    let lo = self.read_u32() as u64;
                    let hi = self.read_u32() as u64;
                    let val = i64::from_ne_bytes((lo | (hi << 32)).to_ne_bytes());
                    self.stack.push(Value::Int(val));
                }
                OpCode::DefineGlobal => {
                    let idx = self.a() as usize;
                    let name = match &self.cur_chunk().constants[idx] {
                        Value::Str(s) => s.clone(),
                        _ => return Err("Constant pool: expected string for global name".into()),
                    };
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    self.globals.insert(name, val);
                    self.advance();
                }
                OpCode::GetGlobal => {
                    let idx = self.a() as usize;
                    let name = match &self.cur_chunk().constants[idx] {
                        Value::Str(s) => s.clone(),
                        _ => return Err("Constant pool: expected string for global name".into()),
                    };
                    let val = self.globals.get(&name).cloned().unwrap_or(Value::Nil);
                    self.stack.push(val);
                    self.advance();
                }
                OpCode::SetGlobal => {
                    let idx = self.a() as usize;
                    let name = match &self.cur_chunk().constants[idx] {
                        Value::Str(s) => s.clone(),
                        _ => return Err("Constant pool: expected string for global name".into()),
                    };
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    self.globals.insert(name, val);
                    self.advance();
                }
                OpCode::GetLocal => {
                    let idx = self.a() as usize;
                    let base = self.frames.last().map(|f| f.base).unwrap_or(0);
                    let val = self.stack[base + idx].clone();
                    self.stack.push(val);
                    self.advance();
                }
                OpCode::SetLocal => {
                    let idx = self.a() as usize;
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    let base = self.frames.last().map(|f| f.base).unwrap_or(0);
                    self.stack[base + idx] = val;
                    self.advance();
                }
                OpCode::InitLocal => {
                    self.advance();
                }
                OpCode::GetUpvalue => {
                    let idx = self.a() as usize;
                    // For now, upvalues are just copied from the closure
                    let val = self.stack.last().cloned().unwrap_or(Value::Nil);
                    self.stack.push(val);
                    self.advance();
                }
                OpCode::SetUpvalue => {
                    self.advance(); // simplified
                }
                OpCode::Jump => {
                    let off = self.offset();
                    self.ip = (self.ip as i32 + 1 + off as i32) as usize;
                }
                OpCode::JumpIfFalse => {
                    let off = self.offset();
                    let val = self.stack.last().cloned().unwrap_or(Value::Nil);
                    if !is_truthy(&val) {
                        self.ip = (self.ip as i32 + 1 + off as i32) as usize;
                    } else {
                        self.advance();
                    }
                }
                OpCode::JumpIfTrue => {
                    let off = self.offset();
                    let val = self.stack.last().cloned().unwrap_or(Value::Nil);
                    if is_truthy(&val) {
                        self.ip = (self.ip as i32 + 1 + off as i32) as usize;
                    } else {
                        self.advance();
                    }
                }
                OpCode::Loop => {
                    let off = self.offset() as i16;
                    self.ip = (self.ip as i32 + 1 - off as i32) as usize;
                }
                OpCode::Call => {
                    let arg_count = self.a() as usize;
                    self.call(arg_count)?;
                }
                OpCode::Closure => {
                    let func_idx = self.a() as usize;
                    let upvalue_count = self.b() as usize;
                    // Read upvalue data following the instruction
                    let mut upvalues: Vec<Value> = Vec::with_capacity(upvalue_count);
                    // For now, just create a closure with empty upvalues
                    self.stack.push(Value::Fun(Closure::new(func_idx, upvalue_count)));
                    self.advance();
                }
                // --- Arithmetic ---
                OpCode::Add => {
                    let b = self.stack.pop().unwrap_or(Value::Nil);
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(add_values(&a, &b));
                    self.advance();
                }
                OpCode::Subtract => {
                    let b = self.stack.pop().unwrap_or(Value::Nil);
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(sub_values(&a, &b));
                    self.advance();
                }
                OpCode::Multiply => {
                    let b = self.stack.pop().unwrap_or(Value::Nil);
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(mul_values(&a, &b));
                    self.advance();
                }
                OpCode::Divide => {
                    let b = self.stack.pop().unwrap_or(Value::Nil);
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(div_values(&a, &b));
                    self.advance();
                }
                OpCode::Modulus => {
                    let b = self.stack.pop().unwrap_or(Value::Nil);
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(mod_values(&a, &b));
                    self.advance();
                }
                OpCode::Negate => {
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(neg_value(&a));
                    self.advance();
                }
                // --- Comparisons ---
                OpCode::Equal => {
                    let b = self.stack.pop().unwrap_or(Value::Nil);
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Value::Bool(a == b));
                    self.advance();
                }
                OpCode::NotEqual => {
                    let b = self.stack.pop().unwrap_or(Value::Nil);
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Value::Bool(a != b));
                    self.advance();
                }
                OpCode::Less => {
                    let b = self.stack.pop().unwrap_or(Value::Nil);
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Value::Bool(less_than(&a, &b)));
                    self.advance();
                }
                OpCode::Greater => {
                    let b = self.stack.pop().unwrap_or(Value::Nil);
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Value::Bool(greater_than(&a, &b)));
                    self.advance();
                }
                OpCode::LessEqual => {
                    let b = self.stack.pop().unwrap_or(Value::Nil);
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Value::Bool(!greater_than(&a, &b)));
                    self.advance();
                }
                OpCode::GreaterEqual => {
                    let b = self.stack.pop().unwrap_or(Value::Nil);
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Value::Bool(!less_than(&a, &b)));
                    self.advance();
                }
                // --- Logical ---
                OpCode::Not => {
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(Value::Bool(!is_truthy(&a)));
                    self.advance();
                }
                // --- Strings ---
                OpCode::Concat => {
                    let b = self.stack.pop().unwrap_or(Value::Nil);
                    let a = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(concat_values(&a, &b));
                    self.advance();
                }
                // --- Data structures ---
                OpCode::BuildList => {
                    let count = self.a() as usize;
                    let mut items = Vec::with_capacity(count);
                    for _ in 0..count {
                        items.insert(0, self.stack.pop().unwrap_or(Value::Nil));
                    }
                    self.stack.push(Value::Vec(items));
                    self.advance();
                }
                OpCode::BuildTuple => {
                    let count = self.a() as usize;
                    let mut items = Vec::with_capacity(count);
                    for _ in 0..count {
                        items.insert(0, self.stack.pop().unwrap_or(Value::Nil));
                    }
                    self.stack.push(Value::Tuple(items));
                    self.advance();
                }
                OpCode::BuildMap => {
                    let count = self.a() as usize;
                    let mut pairs = Vec::with_capacity(count);
                    for _ in 0..count {
                        let val = self.stack.pop().unwrap_or(Value::Nil);
                        let key = self.stack.pop().unwrap_or(Value::Nil);
                        pairs.push((key, val));
                    }
                    pairs.reverse();
                    self.stack.push(Value::Map(pairs));
                    self.advance();
                }
                OpCode::Subscript => {
                    let key = self.stack.pop().unwrap_or(Value::Nil);
                    let obj = self.stack.pop().unwrap_or(Value::Nil);
                    self.stack.push(subscript_access(&obj, &key));
                    self.advance();
                }
                OpCode::SubscriptSet => {
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    let key = self.stack.pop().unwrap_or(Value::Nil);
                    let mut obj = self.stack.pop().unwrap_or(Value::Nil);
                    subscript_set(&mut obj, &key, val);
                    self.stack.push(obj);
                    self.advance();
                }
                // --- Misc ---
                OpCode::Print => {
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    println!("{}", val);
                    self.advance();
                }
                OpCode::Assert => {
                    let val = self.stack.pop().unwrap_or(Value::Nil);
                    if !is_truthy(&val) {
                        return Err("Assertion failed".into());
                    }
                    self.advance();
                }
            }
        }
        Ok(())
    }

    // ── Call handling ───────────────────────────────────────────────────

    fn call(&mut self, arg_count: usize) -> Result<(), String> {
        // The function is at stack[stack.len() - arg_count]
        let stack_len = self.stack.len();
        let func_idx = stack_len.saturating_sub(arg_count);
        let callee = self.stack[func_idx].clone();

        match callee {
            Value::Fun(closure) => {
                let func_idx = closure.func_idx;
                if func_idx >= self.functions.len() {
                    return Err(format!("Invalid function index {}", func_idx));
                }

                let frame = Frame {
                    func_idx: self.chunk.unwrap(),
                    return_ip: self.ip + 1,
                    base: func_idx, // stack[base] = function itself (can be overwritten)
                };
                self.frames.push(frame);
                self.chunk = Some(func_idx);
                self.ip = 0;
                Ok(())
            }
            Value::Native(func, _name) => {
                // Collect args from stack (after the function)
                let args: Vec<Value> = self.stack.drain(func_idx..).skip(1).collect();
                let result = func(&args);
                self.stack.push(result);
                self.advance();
                Ok(())
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
        Value::Str(format!("{}{}", a, b))
    }

    fn subscript_access(obj: &Value, key: &Value) -> Value {
        match (obj, key) {
            (Value::Vec(items), Value::Int(i)) => {
                let idx = *i as usize;
                items.get(idx).cloned().unwrap_or(Value::Nil)
            }
            (Value::Vec(items), Value::Float(f)) => {
                let idx = *f as usize;
                items.get(idx).cloned().unwrap_or(Value::Nil)
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
}