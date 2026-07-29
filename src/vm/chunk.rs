use super::opcode::*;
use super::value::Value;

/// A compiled chunk of bytecode — one per function / top-level script.
#[derive(Debug, Clone)]
pub struct Chunk {
    /// Flat array of instruction words (each is a `u32`).
    pub code: Vec<u32>,
    /// Constant pool — values referenced by `CONST` instructions.
    pub constants: Vec<Value>,
    /// Sub-function prototypes (for closures).
    pub functions: Vec<Chunk>,
    /// Line numbers for each instruction word (for error messages).
    pub lines: Vec<usize>,
    /// Number of local slots this chunk needs.
    pub local_count: usize,
    /// Number of upvalue slots this chunk needs.
    pub upvalue_count: usize,
    /// Name of this function (for debugging).
    pub name: String,
    /// Fast global variable cache.
    pub global_cache: Vec<Option<usize>>,
    /// Parameter names of the function.
    pub param_names: Vec<String>,
    /// Default values for parameters (if any).
    pub param_defaults: Vec<Option<Value>>,
    /// Whether the function has a variadic parameter.
    pub is_variadic: bool,
}

impl Chunk {
    pub fn new(name: impl Into<String>) -> Self {
        Chunk {
            code: Vec::new(),
            constants: Vec::new(),
            functions: Vec::new(),
            lines: Vec::new(),
            local_count: 0,
            upvalue_count: 0,
            name: name.into(),
            global_cache: Vec::new(),
            param_names: Vec::new(),
            param_defaults: Vec::new(),
            is_variadic: false,
        }
    }

    // ── Writing ────────────────────────────────────────────────────────────

    /// Write a single instruction word. `line` is the source line number.
    #[inline]
    pub fn write(&mut self, insn: u32, line: usize) {
        self.code.push(insn);
        self.lines.push(line);
    }

    /// Convenience: encode + write.
    #[inline]
    pub fn emit(&mut self, op: OpCode, a: u8, b: u8, line: usize) {
        self.write(encode(op, a, b), line);
    }

    /// Add a constant to the pool; returns its index.
    pub fn add_constant(&mut self, val: Value) -> usize {
        // Deduplicate strings to save space (optional optimisation).
        for (i, existing) in self.constants.iter().enumerate() {
            if *existing == val {
                return i;
            }
        }
        let idx = self.constants.len();
        self.constants.push(val);
        idx
    }

    /// Add a child function chunk; returns its index.
    pub fn add_function(&mut self, chunk: Chunk) -> usize {
        let idx = self.functions.len();
        self.functions.push(chunk);
        idx
    }

    // ── Reading ────────────────────────────────────────────────────────────

    #[inline]
    pub fn read(&self, ip: usize) -> u32 {
        self.code[ip]
    }

    #[inline]
    pub fn read_op(&self, ip: usize) -> OpCode {
        decode_op(self.code[ip])
    }

    #[inline]
    pub fn read_line(&self, ip: usize) -> usize {
        self.lines[ip]
    }

    // ── Disassembly ────────────────────────────────────────────────────────

    pub fn disassemble(&self, name: &str) -> String {
        let mut out = String::new();
        out.push_str(&format!("== {} ==\n", name));
        out.push_str(&format!("  local_count={} upvalue_count={}\n", self.local_count, self.upvalue_count));
        out.push_str(&format!("  constants={} functions={} code_size={}\n",
            self.constants.len(), self.functions.len(), self.code.len()));

        let mut ip = 0;
        while ip < self.code.len() {
            let line = self.lines[ip];
            let op = self.read_op(ip);
            let name = opcode_name(op);
            let width = opcode_width(op);

            match op {
                OpCode::Const => {
                    let idx = decode_a(self.code[ip]) as usize;
                    let val = &self.constants[idx];
                    out.push_str(&format!("  {:04x}  [{:>4}]  {:<16} {:>3} '{}'\n", ip, line, name, idx, val));
                }
                OpCode::GetLocal | OpCode::SetLocal | OpCode::InitLocal
                | OpCode::GetRefLocal | OpCode::GetRefGlobal
                | OpCode::GetUpvalue | OpCode::SetUpvalue
                | OpCode::GetGlobal | OpCode::SetGlobal | OpCode::DefineGlobal
                | OpCode::LoadInt | OpCode::Pop | OpCode::Call
                | OpCode::BuildList | OpCode::BuildTuple | OpCode::BuildMap => {
                    let a = decode_a(self.code[ip]);
                    out.push_str(&format!("  {:04x}  [{:>4}]  {:<16} {}\n", ip, line, name, a));
                }
                OpCode::Jump | OpCode::JumpIfFalse | OpCode::JumpIfTrue | OpCode::Loop => {
                    let offset = decode_offset(self.code[ip]);
                    let target = (ip as i32 + 1 + offset as i32) as usize;
                    out.push_str(&format!("  {:04x}  [{:>4}]  {:<16} {} -> {:04x}\n", ip, line, name, offset, target));
                }
                OpCode::Closure => {
                    let a = decode_a(self.code[ip]);
                    let b = decode_b(self.code[ip]);
                    out.push_str(&format!("  {:04x}  [{:>4}]  {:<16} {} ({} upvalues)\n", ip, line, name, a, b));
                    ip += b as usize;
                }
                OpCode::LoadIntLong => {
                    if ip + 2 < self.code.len() {
                        let lo = self.code[ip + 1] as u64;
                        let hi = self.code[ip + 2] as u64;
                        let val = (hi << 32) | lo;
                        out.push_str(&format!("  {:04x}  [{:>4}]  {:<16} {}\n", ip, line, name, i64::from_ne_bytes(val.to_ne_bytes())));
                    }
                }
                _ => {
                    out.push_str(&format!("  {:04x}  [{:>4}]  {:<16}\n", ip, line, name));
                }
            }
            ip += width;
        }
        out
    }
}
