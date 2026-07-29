use crate::vm::chunk::Chunk;
use crate::vm::opcode::*;
use crate::vm::value::Value;
use std::collections::HashMap;
use std::fmt::Write;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Arch {
    X86_64,
    AArch64,
}

impl Arch {
    pub fn from_host() -> Self {
        #[cfg(target_arch = "x86_64")]
        return Arch::X86_64;
        #[cfg(target_arch = "aarch64")]
        return Arch::AArch64;
        #[cfg(not(any(target_arch = "x86_64", target_arch = "aarch64")))]
        return Arch::X86_64;
    }

    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "x86_64" | "x64" | "amd64" => Arch::X86_64,
            "aarch64" | "arm64" => Arch::AArch64,
            _ => Arch::from_host(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OptLevel {
    None,
    Fast,
    Moderate,
    Aggressive,
    Size,
    SizeZ,
}

impl OptLevel {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "none" | "o0" | "O0" => OptLevel::None,
            "fast" | "o1" | "O1" => OptLevel::Fast,
            "moderate" | "o2" | "O2" => OptLevel::Moderate,
            "aggressive" | "o3" | "O3" => OptLevel::Aggressive,
            "size" | "os" | "Os" => OptLevel::Size,
            "sizez" | "oz" | "Oz" => OptLevel::SizeZ,
            _ => OptLevel::Moderate,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmitType {
    Assembly,
    Object,
}

impl EmitType {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "asm" | "s" => EmitType::Assembly,
            "obj" | "o" => EmitType::Object,
            _ => EmitType::Assembly,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AsmSyntax {
    #[default]
    Att,
    Intel,
}

impl AsmSyntax {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "intel" => AsmSyntax::Intel,
            "att" | "at&t" => AsmSyntax::Att,
            _ => AsmSyntax::Att,
        }
    }
}

pub struct AsmBackend {
    arch: Arch,
    opt_level: OptLevel,
    emit_type: EmitType,
    target_triple: String,
    output: String,
    labels: HashMap<usize, String>,
    label_counter: usize,
    stack_slots: HashMap<usize, i32>,
    stack_size: i32,
    globals: HashMap<String, i32>,
    functions: HashMap<String, String>,
    current_function: Option<String>,
    chunk: Option<Chunk>,
    ip: usize,
    asm_syntax: AsmSyntax,
    // Bare metal support
    bare_metal: bool,
    no_std: bool,
    entry_point: Option<String>,
}

impl AsmBackend {
    pub fn new(arch: Arch) -> Self {
        let target_triple = match arch {
            Arch::X86_64 => "x86_64-unknown-linux-gnu".to_string(),
            Arch::AArch64 => "aarch64-unknown-linux-gnu".to_string(),
        };

        AsmBackend {
            arch,
            opt_level: OptLevel::Moderate,
            emit_type: EmitType::Assembly,
            target_triple,
            output: String::new(),
            labels: HashMap::new(),
            label_counter: 0,
            stack_slots: HashMap::new(),
            stack_size: 0,
            globals: HashMap::new(),
            functions: HashMap::new(),
            current_function: None,
            chunk: None,
            ip: 0,
            asm_syntax: AsmSyntax::Att,
            bare_metal: false,
            no_std: false,
            entry_point: None,
        }
    }

    pub fn set_opt_level(&mut self, level: OptLevel) {
        self.opt_level = level;
    }

    pub fn set_target_triple(&mut self, triple: &str) {
        self.target_triple = triple.to_string();
    }

    pub fn set_emit_type(&mut self, emit: EmitType) {
        self.emit_type = emit;
    }

    pub fn set_asm_syntax(&mut self, syntax: AsmSyntax) {
        self.asm_syntax = syntax;
    }

    // Bare metal support methods
    pub fn set_bare_metal(&mut self, bare: bool) {
        self.bare_metal = bare;
    }

    pub fn set_no_std(&mut self, no_std: bool) {
        self.no_std = no_std;
    }

    pub fn set_entry_point(&mut self, entry: &str) {
        self.entry_point = Some(entry.to_string());
    }

    pub fn compile_chunk(&mut self, chunk: &Chunk, fn_name: &str) -> Result<(), String> {
        self.chunk = Some(chunk.clone());
        self.ip = 0;
        self.current_function = Some(fn_name.to_string());
        self.output.clear();
        self.labels.clear();
        self.label_counter = 0;
        self.stack_slots.clear();
        self.stack_size = 0;

        self.emit_prelude(fn_name);
        self.emit_function_body(chunk, fn_name)?;
        self.emit_epilogue(fn_name);

        self.functions.insert(fn_name.to_string(), self.output.clone());
        Ok(())
    }

    fn emit_prelude(&mut self, fn_name: &str) {
        let syntax = self.asm_syntax;
        let is_entry = self.bare_metal && fn_name == self.entry_point.as_deref().unwrap_or("_start");
        
        match self.arch {
            Arch::X86_64 => {
                if self.asm_syntax == AsmSyntax::Intel {
                    writeln!(self.output, ".intel_syntax noprefix").unwrap();
                }
                writeln!(self.output, ".text").unwrap();
                
                if is_entry {
                    // Bare metal entry point - no standard prologue
                    writeln!(self.output, ".global _start").unwrap();
                    writeln!(self.output, "_start:").unwrap();
                    // Set up stack pointer if needed
                    let rsp = Self::reg(syntax, "%rsp", "rsp");
                    writeln!(self.output, "    mov {}, 0x7c00", rsp).unwrap(); // Default stack for bootloader
                } else {
                    writeln!(self.output, ".global {}", fn_name).unwrap();
                    writeln!(self.output, ".type {}, @function", fn_name).unwrap();
                    writeln!(self.output, "{}:", fn_name).unwrap();
                    let rbp = Self::reg(syntax, "%rbp", "rbp");
                    let rsp = Self::reg(syntax, "%rsp", "rsp");
                    writeln!(self.output, "    push {}", rbp).unwrap();
                    writeln!(self.output, "    mov {}, {}", rsp, rbp).unwrap();
                    if self.stack_size > 0 {
                        writeln!(self.output, "    sub ${}, {}", self.align_stack(self.stack_size), rsp).unwrap();
                    }
                }
            }
            Arch::AArch64 => {
                writeln!(self.output, ".text").unwrap();
                
                if is_entry {
                    writeln!(self.output, ".global _start").unwrap();
                    writeln!(self.output, "_start:").unwrap();
                    // Set up stack pointer for bare metal
                    writeln!(self.output, "    mov sp, #0x80000").unwrap();
                } else {
                    writeln!(self.output, ".global {}", fn_name).unwrap();
                    writeln!(self.output, ".type {}, %function", fn_name).unwrap();
                    writeln!(self.output, "{}:", fn_name).unwrap();
                    writeln!(self.output, "    stp x29, x30, [sp, #-16]!").unwrap();
                    writeln!(self.output, "    mov x29, sp").unwrap();
                    if self.stack_size > 0 {
                        writeln!(self.output, "    sub sp, sp, #{}", self.align_stack(self.stack_size)).unwrap();
                    }
                }
            }
        }
    }

    fn emit_function_body(&mut self, chunk: &Chunk, fn_name: &str) -> Result<(), String> {
        self.precompute_labels(chunk);
        self.calculate_stack_slots(chunk);

        while self.ip < chunk.code.len() {
            let insn = chunk.code[self.ip];
            let op = decode_op(insn);
            let a = decode_a(insn);
            let b = decode_b(insn);
            let width = opcode_width(op);

            if let Some(label) = self.labels.get(&self.ip) {
                writeln!(self.output, "{}:", label).unwrap();
            }

            match self.emit_instruction(op, a, b, chunk) {
                Ok(()) => {}
                Err(e) => {
                    eprintln!("Warning: Failed to emit {:?}: {}", op, e);
                }
            }

            self.ip += width;
            if op == OpCode::Closure {
                self.ip += b as usize;
            }
        }

        Ok(())
    }

    fn precompute_labels(&mut self, chunk: &Chunk) {
        let mut ip = 0;
        while ip < chunk.code.len() {
            let insn = chunk.code[ip];
            let op = decode_op(insn);
            let width = opcode_width(op);

            match op {
                OpCode::Jump | OpCode::JumpIfFalse | OpCode::JumpIfTrue | OpCode::Loop => {
                    let offset = decode_offset(insn);
                    let target = (ip as i32 + 1 + offset as i32) as usize;
                    self.get_or_create_label(target);
                }
                _ => {}
            }

            ip += width;
            if op == OpCode::Closure {
                let b = decode_b(insn);
                ip += b as usize;
            }
        }
    }

    fn calculate_stack_slots(&mut self, chunk: &Chunk) {
        let mut max_stack = 0;
        let mut current_stack = 0;
        let mut ip = 0;

        while ip < chunk.code.len() {
            let insn = chunk.code[ip];
            let op = decode_op(insn);
            let a = decode_a(insn);
            let width = opcode_width(op);

            let stack_effect = self.stack_effect(op, a);
            current_stack = (current_stack as i32 + stack_effect).max(0) as usize;
            max_stack = max_stack.max(current_stack);

            ip += width;
            if op == OpCode::Closure {
                let b = decode_b(insn);
                ip += b as usize;
            }
        }

        self.stack_size = (max_stack * 8) as i32;
    }

    fn stack_effect(&self, op: OpCode, a: u8) -> i32 {
        match op {
            OpCode::Nil | OpCode::True | OpCode::False | OpCode::LoadInt | OpCode::Const
            | OpCode::GetLocal | OpCode::GetGlobal | OpCode::Closure | OpCode::BuildList
            | OpCode::BuildTuple | OpCode::BuildMap => 1,
            OpCode::Pop => -1,
            OpCode::Add | OpCode::Subtract | OpCode::Multiply | OpCode::Divide
            | OpCode::Modulus | OpCode::Equal | OpCode::NotEqual | OpCode::Less
            | OpCode::Greater | OpCode::LessEqual | OpCode::GreaterEqual | OpCode::Concat
            | OpCode::Subscript => -1,
            OpCode::Negate | OpCode::Not => 0,
            OpCode::Call => -(a as i32),
            OpCode::Return => 0,
            OpCode::DefineGlobal | OpCode::SetGlobal | OpCode::SetLocal => -1,
            OpCode::SubscriptSet => -3,
            OpCode::Print | OpCode::Assert => -1,
            OpCode::ForceLazy | OpCode::Yield | OpCode::MakeCoroutine | OpCode::Resume => 0,
            OpCode::Jump | OpCode::JumpIfFalse | OpCode::JumpIfTrue | OpCode::Loop => 0,
            _ => 0,
        }
    }

    fn get_or_create_label(&mut self, ip: usize) -> String {
        if let Some(label) = self.labels.get(&ip) {
            label.clone()
        } else {
            let label = format!("L{}", self.label_counter);
            self.label_counter += 1;
            self.labels.insert(ip, label.clone());
            label
        }
    }

    fn emit_instruction(&mut self, op: OpCode, a: u8, b: u8, chunk: &Chunk) -> Result<(), String> {
        match op {
            OpCode::Nil => self.emit_push_nil(),
            OpCode::True => self.emit_push_bool(true),
            OpCode::False => self.emit_push_bool(false),
            OpCode::LoadInt => self.emit_push_int(a as i64),
            OpCode::Const => self.emit_push_const(&chunk.constants[a as usize]),
            OpCode::Add => self.emit_binary_op("add"),
            OpCode::Subtract => self.emit_binary_op("sub"),
            OpCode::Multiply => self.emit_binary_op("imul"),
            OpCode::Divide => self.emit_binary_op("idiv"),
            OpCode::Modulus => self.emit_binary_op("mod"),
            OpCode::Negate => self.emit_negate(),
            OpCode::Equal => self.emit_compare("eq"),
            OpCode::NotEqual => self.emit_compare("ne"),
            OpCode::Less => self.emit_compare("lt"),
            OpCode::Greater => self.emit_compare("gt"),
            OpCode::LessEqual => self.emit_compare("le"),
            OpCode::GreaterEqual => self.emit_compare("ge"),
            OpCode::Not => self.emit_not(),
            OpCode::Concat => self.emit_concat(),
            OpCode::Pop => self.emit_pop(),
            OpCode::Print => self.emit_print(),
            OpCode::Assert => self.emit_assert(),
            OpCode::Jump => {
                let target = self.get_or_create_label(self.jump_target(chunk.code[self.ip]));
                self.emit_jump(&target)
            }
            OpCode::JumpIfFalse => {
                let target = self.get_or_create_label(self.jump_target(chunk.code[self.ip]));
                self.emit_jump_if_false(&target)
            }
            OpCode::JumpIfTrue => {
                let target = self.get_or_create_label(self.jump_target(chunk.code[self.ip]));
                self.emit_jump_if_true(&target)
            }
            OpCode::Loop => {
                let target = self.get_or_create_label(self.jump_target(chunk.code[self.ip]));
                self.emit_jump(&target)
            }
            OpCode::Call => self.emit_call(a),
            OpCode::Return => self.emit_return(),
            OpCode::GetLocal => self.emit_load_local(a),
            OpCode::SetLocal => self.emit_store_local(a),
            OpCode::DefineGlobal => self.emit_define_global(a, chunk),
            OpCode::GetGlobal => self.emit_get_global(a, chunk),
            OpCode::SetGlobal => self.emit_set_global(a, chunk),
            OpCode::BuildList => self.emit_build_list(a),
            OpCode::BuildTuple => self.emit_build_tuple(a),
            OpCode::BuildMap => self.emit_build_map(a),
            OpCode::Subscript => self.emit_subscript(),
            OpCode::SubscriptSet => self.emit_subscript_set(),
            OpCode::Closure => self.emit_closure(a, b, chunk),
            OpCode::ForceLazy => self.emit_force_lazy(),
            OpCode::Yield => self.emit_yield(),
            OpCode::MakeCoroutine => self.emit_make_coroutine(),
            OpCode::Resume => self.emit_resume(),
            _ => Ok(()),
        }
    }

    fn jump_target(&self, insn: u32) -> usize {
        let offset = decode_offset(insn);
        (self.ip as i32 + 1 + offset as i32) as usize
    }

    fn emit_push_nil(&mut self) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rax = Self::reg(syntax, "%rax", "rax");
                writeln!(self.output, "    xor {}, {}", rax, rax).unwrap();
                writeln!(self.output, "    push {}", rax).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    mov x0, #0").unwrap();
                writeln!(self.output, "    str x0, [sp, #-16]!").unwrap();
            }
        }
        Ok(())
    }

    fn emit_push_bool(&mut self, val: bool) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rax = Self::reg(syntax, "%rax", "rax");
                writeln!(self.output, "    mov ${}, {}", if val { 1 } else { 0 }, rax).unwrap();
                writeln!(self.output, "    push {}", rax).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    mov x0, #{}", if val { 1 } else { 0 }).unwrap();
                writeln!(self.output, "    str x0, [sp, #-16]!").unwrap();
            }
        }
        Ok(())
    }

    fn emit_push_int(&mut self, val: i64) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rax = Self::reg(syntax, "%rax", "rax");
                writeln!(self.output, "    mov ${}, {}", val, rax).unwrap();
                writeln!(self.output, "    push {}", rax).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    mov x0, #{}", val).unwrap();
                writeln!(self.output, "    str x0, [sp, #-16]!").unwrap();
            }
        }
        Ok(())
    }

    fn emit_push_const(&mut self, val: &Value) -> Result<(), String> {
        match val {
            Value::Nil => self.emit_push_nil(),
            Value::Bool(b) => self.emit_push_bool(*b),
            Value::Int(i) => self.emit_push_int(*i),
            Value::Float(f) => self.emit_push_float(*f),
            Value::Str(s) => self.emit_push_string(s),
            _ => self.emit_push_nil(),
        }
    }

    fn emit_push_float(&mut self, val: f64) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let bits = val.to_bits();
                let rax = Self::reg(syntax, "%rax", "rax");
                writeln!(self.output, "    mov ${}, {}", bits, rax).unwrap();
                writeln!(self.output, "    push {}", rax).unwrap();
            }
            Arch::AArch64 => {
                let bits = val.to_bits();
                writeln!(self.output, "    mov x0, #{}", bits).unwrap();
                writeln!(self.output, "    str x0, [sp, #-16]!").unwrap();
            }
        }
        Ok(())
    }

    fn emit_push_string(&mut self, s: &str) -> Result<(), String> {
        let syntax = self.asm_syntax;
        let label = format!(".Lstr{}", self.label_counter);
        self.label_counter += 1;

        match self.arch {
            Arch::X86_64 => {
                let rax = Self::reg(syntax, "%rax", "rax");
                writeln!(self.output, "    .section .rodata").unwrap();
                writeln!(self.output, "{}:", label).unwrap();
                writeln!(self.output, "    .string \"{}\"", s.escape_default()).unwrap();
                writeln!(self.output, "    .section .text").unwrap();
                writeln!(self.output, "    lea {}(%rip), {}", label, rax).unwrap();
                writeln!(self.output, "    push {}", rax).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    .section .rodata").unwrap();
                writeln!(self.output, "{}:", label).unwrap();
                writeln!(self.output, "    .string \"{}\"", s.escape_default()).unwrap();
                writeln!(self.output, "    .section .text").unwrap();
                writeln!(self.output, "    adrp x0, {}", label).unwrap();
                writeln!(self.output, "    add x0, x0, :lo12:{}", label).unwrap();
                writeln!(self.output, "    str x0, [sp, #-16]!").unwrap();
            }
        }
        Ok(())
    }

    fn emit_pop(&mut self) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rax = Self::reg(syntax, "%rax", "rax");
                writeln!(self.output, "    pop {}", rax).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    ldr x0, [sp], #16").unwrap();
            }
        }
        Ok(())
    }

    fn emit_binary_op(&mut self, op: &str) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rcx = Self::reg(syntax, "%rcx", "rcx");
                let rax = Self::reg(syntax, "%rax", "rax");
                let rdx = Self::reg(syntax, "%rdx", "rdx");
                writeln!(self.output, "    pop {}", rcx).unwrap();
                writeln!(self.output, "    pop {}", rax).unwrap();
                match op {
                    "add" => writeln!(self.output, "    add {}, {}", rcx, rax).unwrap(),
                    "sub" => writeln!(self.output, "    sub {}, {}", rcx, rax).unwrap(),
                    "imul" => writeln!(self.output, "    imul {}, {}", rcx, rax).unwrap(),
                    "idiv" => {
                        writeln!(self.output, "    cqo").unwrap();
                        writeln!(self.output, "    idiv {}", rcx).unwrap();
                    }
                    "mod" => {
                        writeln!(self.output, "    cqo").unwrap();
                        writeln!(self.output, "    idiv {}", rcx).unwrap();
                        writeln!(self.output, "    mov {}, {}", Self::reg(syntax, "%rdx", "rdx"), rax).unwrap();
                    }
                    _ => {}
                }
                writeln!(self.output, "    push {}", rax).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    ldr x1, [sp], #16").unwrap();
                writeln!(self.output, "    ldr x0, [sp], #16").unwrap();
                match op {
                    "add" => writeln!(self.output, "    add x0, x0, x1").unwrap(),
                    "sub" => writeln!(self.output, "    sub x0, x0, x1").unwrap(),
                    "imul" => writeln!(self.output, "    mul x0, x0, x1").unwrap(),
                    "idiv" => writeln!(self.output, "    sdiv x0, x0, x1").unwrap(),
                    "mod" => {
                        writeln!(self.output, "    sdiv x2, x0, x1").unwrap();
                        writeln!(self.output, "    msub x0, x2, x1, x0").unwrap();
                    }
                    _ => {}
                }
                writeln!(self.output, "    str x0, [sp, #-16]!").unwrap();
            }
        }
        Ok(())
    }

    fn emit_negate(&mut self) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rax = Self::reg(syntax, "%rax", "rax");
                writeln!(self.output, "    pop {}", rax).unwrap();
                writeln!(self.output, "    neg {}", rax).unwrap();
                writeln!(self.output, "    push {}", rax).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    ldr x0, [sp], #16").unwrap();
                writeln!(self.output, "    neg x0, x0").unwrap();
                writeln!(self.output, "    str x0, [sp, #-16]!").unwrap();
            }
        }
        Ok(())
    }

    fn emit_not(&mut self) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rax = Self::reg(syntax, "%rax", "rax");
                let al = Self::reg(syntax, "%al", "al");
                writeln!(self.output, "    pop {}", rax).unwrap();
                writeln!(self.output, "    test {}, {}", rax, rax).unwrap();
                writeln!(self.output, "    sete {}", al).unwrap();
                writeln!(self.output, "    movzx {}, {}", al, rax).unwrap();
                writeln!(self.output, "    push {}", rax).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    ldr x0, [sp], #16").unwrap();
                writeln!(self.output, "    cmp x0, #0").unwrap();
                writeln!(self.output, "    cset x0, eq").unwrap();
                writeln!(self.output, "    str x0, [sp, #-16]!").unwrap();
            }
        }
        Ok(())
    }

    fn emit_compare(&mut self, cond: &str) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rcx = Self::reg(syntax, "%rcx", "rcx");
                let rax = Self::reg(syntax, "%rax", "rax");
                let al = Self::reg(syntax, "%al", "al");
                writeln!(self.output, "    pop {}", rcx).unwrap();
                writeln!(self.output, "    pop {}", rax).unwrap();
                writeln!(self.output, "    cmp {}, {}", rcx, rax).unwrap();
                match cond {
                    "eq" => writeln!(self.output, "    sete {}", Self::reg(syntax, "%al", "al")).unwrap(),
                    "ne" => writeln!(self.output, "    setne {}", Self::reg(syntax, "%al", "al")).unwrap(),
                    "lt" => writeln!(self.output, "    setl {}", Self::reg(syntax, "%al", "al")).unwrap(),
                    "gt" => writeln!(self.output, "    setg {}", Self::reg(syntax, "%al", "al")).unwrap(),
                    "le" => writeln!(self.output, "    setle {}", Self::reg(syntax, "%al", "al")).unwrap(),
                    "ge" => writeln!(self.output, "    setge {}", Self::reg(syntax, "%al", "al")).unwrap(),
                    _ => {}
                }
                let al_reg = Self::reg(syntax, "%al", "al");
                let rax = Self::reg(syntax, "%rax", "rax");
                writeln!(self.output, "    movzx {}, {}", al_reg, rax).unwrap();
                writeln!(self.output, "    push {}", rax).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    ldr x1, [sp], #16").unwrap();
                writeln!(self.output, "    ldr x0, [sp], #16").unwrap();
                writeln!(self.output, "    cmp x0, x1").unwrap();
                match cond {
                    "eq" => writeln!(self.output, "    cset x0, eq").unwrap(),
                    "ne" => writeln!(self.output, "    cset x0, ne").unwrap(),
                    "lt" => writeln!(self.output, "    cset x0, lt").unwrap(),
                    "gt" => writeln!(self.output, "    cset x0, gt").unwrap(),
                    "le" => writeln!(self.output, "    cset x0, le").unwrap(),
                    "ge" => writeln!(self.output, "    cset x0, ge").unwrap(),
                    _ => {}
                }
                writeln!(self.output, "    str x0, [sp, #-16]!").unwrap();
            }
        }
        Ok(())
    }

    fn emit_concat(&mut self) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rdi = Self::reg(syntax, "%rdi", "rdi");
                let rsi = Self::reg(syntax, "%rsi", "rsi");
                let rax = Self::reg(syntax, "%rax", "rax");
                writeln!(self.output, "    pop {}", rdi).unwrap();
                writeln!(self.output, "    pop {}", rsi).unwrap();
                writeln!(self.output, "    call aly_string_concat@PLT").unwrap();
                writeln!(self.output, "    push {}", rax).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    ldr x1, [sp], #16").unwrap();
                writeln!(self.output, "    ldr x0, [sp], #16").unwrap();
                writeln!(self.output, "    bl aly_string_concat").unwrap();
                writeln!(self.output, "    str x0, [sp, #-16]!").unwrap();
            }
        }
        Ok(())
    }

    fn emit_print(&mut self) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rdi = Self::reg(syntax, "%rdi", "rdi");
                writeln!(self.output, "    pop {}", rdi).unwrap();
                writeln!(self.output, "    call aly_print@PLT").unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    ldr x0, [sp], #16").unwrap();
                writeln!(self.output, "    bl aly_print").unwrap();
            }
        }
        Ok(())
    }

    fn emit_assert(&mut self) -> Result<(), String> {
        let syntax = self.asm_syntax;
        let fail_label = format!(".Lassert_fail_{}", self.label_counter);
        self.label_counter += 1;

        match self.arch {
            Arch::X86_64 => {
                let rax = Self::reg(syntax, "%rax", "rax");
                let rdi = Self::reg(syntax, "%rdi", "rdi");
                writeln!(self.output, "    pop {}", rax).unwrap();
                writeln!(self.output, "    test {}, {}", rax, rax).unwrap();
                writeln!(self.output, "    jne {}", fail_label).unwrap();
                writeln!(self.output, "    lea .Lassert_msg(%rip), {}", rdi).unwrap();
                writeln!(self.output, "    call puts@PLT").unwrap();
                writeln!(self.output, "    mov ${}, {}", 1, rdi).unwrap();
                writeln!(self.output, "    call exit@PLT").unwrap();
                writeln!(self.output, "{}:", fail_label).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    ldr x0, [sp], #16").unwrap();
                writeln!(self.output, "    cbnz x0, {}", fail_label).unwrap();
                writeln!(self.output, "    adrp x0, .Lassert_msg").unwrap();
                writeln!(self.output, "    add x0, x0, :lo12:.Lassert_msg").unwrap();
                writeln!(self.output, "    bl puts").unwrap();
                writeln!(self.output, "    mov x0, #1").unwrap();
                writeln!(self.output, "    bl exit").unwrap();
                writeln!(self.output, "{}:", fail_label).unwrap();
            }
        }
        Ok(())
    }

    fn emit_jump(&mut self, target: &str) -> Result<(), String> {
        match self.arch {
            Arch::X86_64 => writeln!(self.output, "    jmp {}", target).unwrap(),
            Arch::AArch64 => writeln!(self.output, "    b {}", target).unwrap(),
        }
        Ok(())
    }

    fn emit_jump_if_false(&mut self, target: &str) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rax = Self::reg(syntax, "%rax", "rax");
                writeln!(self.output, "    pop {}", rax).unwrap();
                writeln!(self.output, "    test {}, {}", rax, rax).unwrap();
                writeln!(self.output, "    je {}", target).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    ldr x0, [sp], #16").unwrap();
                writeln!(self.output, "    cbz x0, {}", target).unwrap();
            }
        }
        Ok(())
    }

    fn emit_jump_if_true(&mut self, target: &str) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rax = Self::reg(syntax, "%rax", "rax");
                writeln!(self.output, "    pop {}", rax).unwrap();
                writeln!(self.output, "    test {}, {}", rax, rax).unwrap();
                writeln!(self.output, "    jne {}", target).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    ldr x0, [sp], #16").unwrap();
                writeln!(self.output, "    cbnz x0, {}", target).unwrap();
            }
        }
        Ok(())
    }

    fn emit_call(&mut self, num_args: u8) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rax = Self::reg(syntax, "%rax", "rax");
                writeln!(self.output, "    mov ${}, {}", num_args, rax).unwrap();
                writeln!(self.output, "    call aly_call@PLT").unwrap();
                writeln!(self.output, "    push {}", rax).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    mov x0, #{}", num_args).unwrap();
                writeln!(self.output, "    bl aly_call").unwrap();
                writeln!(self.output, "    str x0, [sp, #-16]!").unwrap();
            }
        }
        Ok(())
    }

    fn emit_return(&mut self) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                if self.stack_size > 0 {
                    let rsp = Self::reg(syntax, "%rsp", "rsp");
                    writeln!(self.output, "    add ${}, {}", self.align_stack(self.stack_size), rsp).unwrap();
                }
                let rbp = Self::reg(syntax, "%rbp", "rbp");
                writeln!(self.output, "    pop {}", rbp).unwrap();
                writeln!(self.output, "    ret").unwrap();
            }
            Arch::AArch64 => {
                if self.stack_size > 0 {
                    writeln!(self.output, "    add sp, sp, #{}", self.align_stack(self.stack_size)).unwrap();
                }
                writeln!(self.output, "    ldp x29, x30, [sp], #16").unwrap();
                writeln!(self.output, "    ret").unwrap();
            }
        }
        Ok(())
    }

    fn emit_load_local(&mut self, slot: u8) -> Result<(), String> {
        let syntax = self.asm_syntax;
        let offset = (slot as i32) * 8;
        match self.arch {
            Arch::X86_64 => {
                let rbp = Self::reg(syntax, "%rbp", "rbp");
                let rax = Self::reg(syntax, "%rax", "rax");
                writeln!(self.output, "    mov {}({}), {}", -offset - 8, rbp, rax).unwrap();
                writeln!(self.output, "    push {}", rax).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    ldr x0, [x29, #-{}]", offset + 16).unwrap();
                writeln!(self.output, "    str x0, [sp, #-16]!").unwrap();
            }
        }
        Ok(())
    }

    fn emit_store_local(&mut self, slot: u8) -> Result<(), String> {
        let syntax = self.asm_syntax;
        let offset = (slot as i32) * 8;
        match self.arch {
            Arch::X86_64 => {
                let rax = Self::reg(syntax, "%rax", "rax");
                let rbp = Self::reg(syntax, "%rbp", "rbp");
                writeln!(self.output, "    pop {}", rax).unwrap();
                writeln!(self.output, "    mov {}, {}({})", rax, -offset - 8, rbp).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    ldr x0, [sp], #16").unwrap();
                writeln!(self.output, "    str x0, [x29, #-{}]", offset + 16).unwrap();
            }
        }
        Ok(())
    }

    fn emit_define_global(&mut self, idx: u8, chunk: &Chunk) -> Result<(), String> {
        if let Value::Str(name) = &chunk.constants[idx as usize] {
            self.globals.insert(name.clone(), self.globals.len() as i32);
        }
        self.emit_pop()
    }

    fn emit_get_global(&mut self, idx: u8, chunk: &Chunk) -> Result<(), String> {
        let syntax = self.asm_syntax;
        if let Value::Str(name) = &chunk.constants[idx as usize] {
            match self.arch {
                Arch::X86_64 => {
                    let rax = Self::reg(syntax, "%rax", "rax");
                    writeln!(self.output, "    lea {}(%rip), {}", name, rax).unwrap();
                    writeln!(self.output, "    push {}", rax).unwrap();
                }
                Arch::AArch64 => {
                    writeln!(self.output, "    adrp x0, {}", name).unwrap();
                    writeln!(self.output, "    add x0, x0, :lo12:{}", name).unwrap();
                    writeln!(self.output, "    str x0, [sp, #-16]!").unwrap();
                }
            }
        } else {
            self.emit_push_nil()?;
        }
        Ok(())
    }

    fn emit_set_global(&mut self, idx: u8, chunk: &Chunk) -> Result<(), String> {
        let syntax = self.asm_syntax;
        if let Value::Str(name) = &chunk.constants[idx as usize] {
            match self.arch {
                Arch::X86_64 => {
                    let rax = Self::reg(syntax, "%rax", "rax");
                    writeln!(self.output, "    pop {}", rax).unwrap();
                    writeln!(self.output, "    mov {}, {}(%rip)", rax, name).unwrap();
                }
                Arch::AArch64 => {
                    writeln!(self.output, "    ldr x0, [sp], #16").unwrap();
                    writeln!(self.output, "    adrp x1, {}", name).unwrap();
                    writeln!(self.output, "    add x1, x1, :lo12:{}", name).unwrap();
                    writeln!(self.output, "    str x0, [x1]").unwrap();
                }
            }
        }
        Ok(())
    }

    fn emit_build_list(&mut self, count: u8) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rdi = Self::reg(syntax, "%rdi", "rdi");
                let rax = Self::reg(syntax, "%rax", "rax");
                writeln!(self.output, "    mov ${}, {}", count, rdi).unwrap();
                writeln!(self.output, "    call aly_array_new@PLT").unwrap();
                writeln!(self.output, "    push {}", rax).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    mov x0, #{}", count).unwrap();
                writeln!(self.output, "    bl aly_array_new").unwrap();
                writeln!(self.output, "    str x0, [sp, #-16]!").unwrap();
            }
        }
        Ok(())
    }

    fn emit_build_tuple(&mut self, count: u8) -> Result<(), String> {
        self.emit_build_list(count)
    }

    fn emit_build_map(&mut self, count: u8) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rdi = Self::reg(syntax, "%rdi", "rdi");
                let rax = Self::reg(syntax, "%rax", "rax");
                writeln!(self.output, "    mov ${}, {}", count, rdi).unwrap();
                writeln!(self.output, "    call aly_object_new@PLT").unwrap();
                writeln!(self.output, "    push {}", rax).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    mov x0, #{}", count).unwrap();
                writeln!(self.output, "    bl aly_object_new").unwrap();
                writeln!(self.output, "    str x0, [sp, #-16]!").unwrap();
            }
        }
        Ok(())
    }

    fn emit_subscript(&mut self) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rsi = Self::reg(syntax, "%rsi", "rsi");
                let rdi = Self::reg(syntax, "%rdi", "rdi");
                let rax = Self::reg(syntax, "%rax", "rax");
                writeln!(self.output, "    pop {}", rsi).unwrap();
                writeln!(self.output, "    pop {}", rdi).unwrap();
                writeln!(self.output, "    call aly_subscript_get@PLT").unwrap();
                writeln!(self.output, "    push {}", rax).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    ldr x1, [sp], #16").unwrap();
                writeln!(self.output, "    ldr x0, [sp], #16").unwrap();
                writeln!(self.output, "    bl aly_subscript_get").unwrap();
                writeln!(self.output, "    str x0, [sp, #-16]!").unwrap();
            }
        }
        Ok(())
    }

    fn emit_subscript_set(&mut self) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rdx = Self::reg(syntax, "%rdx", "rdx");
                let rsi = Self::reg(syntax, "%rsi", "rsi");
                let rdi = Self::reg(syntax, "%rdi", "rdi");
                writeln!(self.output, "    pop {}", rdx).unwrap();
                writeln!(self.output, "    pop {}", rsi).unwrap();
                writeln!(self.output, "    pop {}", rdi).unwrap();
                writeln!(self.output, "    call aly_subscript_set@PLT").unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    ldr x2, [sp], #16").unwrap();
                writeln!(self.output, "    ldr x1, [sp], #16").unwrap();
                writeln!(self.output, "    ldr x0, [sp], #16").unwrap();
                writeln!(self.output, "    bl aly_subscript_set").unwrap();
            }
        }
        Ok(())
    }

    fn emit_closure(&mut self, func_idx: u8, num_upvalues: u8, chunk: &Chunk) -> Result<(), String> {
        let child_id = self.current_function.as_ref().unwrap().clone() + &format!("_fn{}", func_idx);
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rax = Self::reg(syntax, "%rax", "rax");
                writeln!(self.output, "    lea {}(%rip), {}", child_id, rax).unwrap();
                writeln!(self.output, "    push {}", rax).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    adrp x0, {}", child_id).unwrap();
                writeln!(self.output, "    add x0, x0, :lo12:{}", child_id).unwrap();
                writeln!(self.output, "    str x0, [sp, #-16]!").unwrap();
            }
        }
        Ok(())
    }

    fn emit_force_lazy(&mut self) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rdi = Self::reg(syntax, "%rdi", "rdi");
                let rax = Self::reg(syntax, "%rax", "rax");
                writeln!(self.output, "    pop {}", rdi).unwrap();
                writeln!(self.output, "    call aly_force_lazy@PLT").unwrap();
                writeln!(self.output, "    push {}", rax).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    ldr x0, [sp], #16").unwrap();
                writeln!(self.output, "    bl aly_force_lazy").unwrap();
                writeln!(self.output, "    str x0, [sp, #-16]!").unwrap();
            }
        }
        Ok(())
    }

    fn emit_yield(&mut self) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rax = Self::reg(syntax, "%rax", "rax");
                writeln!(self.output, "    pop {}", rax).unwrap();
                writeln!(self.output, "    ret").unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    ldr x0, [sp], #16").unwrap();
                writeln!(self.output, "    ret").unwrap();
            }
        }
        Ok(())
    }

    fn emit_make_coroutine(&mut self) -> Result<(), String> {
        Ok(())
    }

    fn emit_resume(&mut self) -> Result<(), String> {
        let syntax = self.asm_syntax;
        match self.arch {
            Arch::X86_64 => {
                let rdi = Self::reg(syntax, "%rdi", "rdi");
                let rax = Self::reg(syntax, "%rax", "rax");
                writeln!(self.output, "    pop {}", rdi).unwrap();
                writeln!(self.output, "    call aly_coroutine_resume@PLT").unwrap();
                writeln!(self.output, "    push {}", rax).unwrap();
            }
            Arch::AArch64 => {
                writeln!(self.output, "    ldr x0, [sp], #16").unwrap();
                writeln!(self.output, "    bl aly_coroutine_resume").unwrap();
                writeln!(self.output, "    str x0, [sp, #-16]!").unwrap();
            }
        }
        Ok(())
    }

    fn emit_epilogue(&mut self, fn_name: &str) {
        let is_entry = self.bare_metal && fn_name == self.entry_point.as_deref().unwrap_or("_start");
        
        if !is_entry {
            match self.arch {
                Arch::X86_64 => {
                    writeln!(self.output, ".size {}, .-{0}", fn_name).unwrap();
                    writeln!(self.output, "").unwrap();
                    writeln!(self.output, ".section .rodata").unwrap();
                    writeln!(self.output, ".Lassert_msg:").unwrap();
                    writeln!(self.output, "    .string \"Assertion failed\"").unwrap();
                    writeln!(self.output, "").unwrap();
                }
                Arch::AArch64 => {
                    writeln!(self.output, "    .size {}, .-{0}", fn_name).unwrap();
                    writeln!(self.output, "").unwrap();
                    writeln!(self.output, ".section .rodata").unwrap();
                    writeln!(self.output, ".Lassert_msg:").unwrap();
                    writeln!(self.output, "    .string \"Assertion failed\"").unwrap();
                    writeln!(self.output, "").unwrap();
                }
            }
        }
    }

    fn align_stack(&self, size: i32) -> i32 {
        (size + 15) & !15
    }

    fn reg(asm_syntax: AsmSyntax, att: &'static str, intel: &'static str) -> &'static str {
        match asm_syntax {
            AsmSyntax::Att => att,
            AsmSyntax::Intel => intel,
        }
    }

    fn imm(asm_syntax: AsmSyntax, att: &'static str, intel: &'static str) -> &'static str {
        match asm_syntax {
            AsmSyntax::Att => att,
            AsmSyntax::Intel => intel,
        }
    }

    pub fn get_output(&self) -> &str {
        &self.output
    }

    pub fn take_output(&mut self) -> String {
        std::mem::take(&mut self.output)
    }

    pub fn get_functions(&self) -> &HashMap<String, String> {
        &self.functions
    }

    pub fn emit_to_file(&self, path: &str) -> Result<(), String> {
        std::fs::write(path, &self.output)
            .map_err(|e| format!("Failed to write assembly file: {}", e))
    }
}

pub fn compile_chunk_to_asm(
    chunk: &Chunk,
    output_name: &str,
    arch: &str,
    opt_level: &str,
    emit: &str,
) -> Result<(), String> {
    let arch = Arch::from_str(arch);
    let opt = OptLevel::from_str(opt_level);
    let emit_type = EmitType::from_str(emit);

    let mut backend = AsmBackend::new(arch);
    backend.set_opt_level(opt);
    backend.set_emit_type(emit_type);

    backend.compile_chunk(chunk, "aly_main")?;

    let output = backend.take_output();
    let ext = match emit_type {
        EmitType::Assembly => "s",
        EmitType::Object => "o",
    };
    let out_path = format!("{}.{}", output_name, ext);

    std::fs::write(&out_path, output)
        .map_err(|e| format!("Failed to write output: {}", e))?;

    if emit_type == EmitType::Object {
        let status = std::process::Command::new("as")
            .args(["-o", &format!("{}.o", output_name), &out_path])
            .spawn()
            .map_err(|e| format!("Failed to spawn as: {}", e))?
            .wait()
            .map_err(|e| format!("Failed to wait for as: {}", e))?;

        if !status.success() {
            return Err("Assembly failed".to_string());
        }
    }

    Ok(())
}