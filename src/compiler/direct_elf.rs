// src/compiler/direct_elf.rs
use crate::compiler::ast::{Expr, Stmt, Program};
use crate::compiler::parser::parse_program;
use std::collections::HashMap;

pub struct DirectCompiler {
    code: Vec<u8>,
    rodata: Vec<u8>,
    locals: HashMap<String, i32>,
    stack_offset: i32,
    relocations: Vec<(usize, usize)>, // (position in code, offset in rodata)
}

impl DirectCompiler {
    pub fn new() -> Self {
        Self {
            code: Vec::new(),
            rodata: Vec::new(),
            locals: HashMap::new(),
            stack_offset: 0,
            relocations: Vec::new(),
        }
    }

    pub fn compile_program_to_elf(program: &Program) -> Result<Vec<u8>, String> {
        let mut compiler = Self::new();
        compiler.compile(program)?;
        Ok(compiler.link_elf())
    }

    pub fn compile_program_to_pe(program: &Program) -> Result<Vec<u8>, String> {
        let mut compiler = Self::new();
        compiler.compile(program)?;
        Ok(compiler.link_pe())
    }

    pub fn compile_program_to_bin(program: &Program) -> Result<Vec<u8>, String> {
        let mut compiler = Self::new();
        compiler.compile(program)?;
        Ok(compiler.link_bin())
    }

    fn emit(&mut self, bytes: &[u8]) {
        self.code.extend_from_slice(bytes);
    }

    fn compile(&mut self, program: &Program) -> Result<(), String> {
        // Function Prologue
        // push rbp -> 55
        self.emit(&[0x55]);
        // mov rbp, rsp -> 48 89 E5
        self.emit(&[0x48, 0x89, 0xe5]);
        // Reserve stack space (e.g. 128 bytes)
        // sub rsp, 128 -> 48 81 ec 80 00 00 00
        self.emit(&[0x48, 0x81, 0xec, 0x80, 0x00, 0x00, 0x00]);

        for stmt in &program.stmts {
            self.compile_stmt(stmt)?;
        }

        // Implicit exit syscall (exit code 0)
        // mov rax, 60 (sys_exit) -> 48 c7 c0 3c 00 00 00
        self.emit(&[0x48, 0xc7, 0xc0, 0x3c, 0x00, 0x00, 0x00]);
        // mov rdi, 0 -> 48 c7 c7 00 00 00 00
        self.emit(&[0x48, 0xc7, 0xc7, 0x00, 0x00, 0x00, 0x00]);
        // syscall -> 0f 05
        self.emit(&[0x0f, 0x05]);

        Ok(())
    }

    fn compile_stmt(&mut self, stmt: &Stmt) -> Result<(), String> {
        match stmt {
            Stmt::Let { name, init, .. } => {
                if let Some(expr) = init {
                    self.compile_expr(expr)?;
                } else {
                    // Push 0 (nil) by default
                    self.emit(&[0x6a, 0x00]);
                }
                // pop rax -> 58
                self.emit(&[0x58]);
                self.stack_offset += 8;
                self.locals.insert(name.clone(), -self.stack_offset);
                // mov [rbp - offset], rax -> 48 89 45 [offset]
                let offset = (-self.stack_offset) as i8;
                self.emit(&[0x48, 0x89, 0x45, offset as u8]);
            }
            Stmt::Assign { target, value } => {
                if let Expr::Var(name) = target {
                    self.compile_expr(value)?;
                    // pop rax -> 58
                    self.emit(&[0x58]);
                    if let Some(&offset) = self.locals.get(name) {
                        // mov [rbp - offset], rax -> 48 89 45 [offset]
                        self.emit(&[0x48, 0x89, 0x45, offset as i8 as u8]);
                    } else {
                        return Err(format!("Undefined variable '{}'", name));
                    }
                } else {
                    return Err("Assignment target must be a variable".to_string());
                }
            }
            Stmt::Expr(expr) => {
                self.compile_expr(expr)?;
                // Pop the result to keep stack balanced
                // pop rax -> 58
                self.emit(&[0x58]);
            }
            Stmt::Return(opt_expr) => {
                if let Some(expr) = opt_expr {
                    self.compile_expr(expr)?;
                    // pop rdi -> 5f (return value as exit code)
                    self.emit(&[0x5f]);
                } else {
                    // mov rdi, 0 -> 48 c7 c7 00 00 00 00
                    self.emit(&[0x48, 0xc7, 0xc7, 0x00, 0x00, 0x00, 0x00]);
                }
                // mov rax, 60 (sys_exit) -> 48 c7 c0 3c 00 00 00
                self.emit(&[0x48, 0xc7, 0xc0, 0x3c, 0x00, 0x00, 0x00]);
                // syscall -> 0f 05
                self.emit(&[0x0f, 0x05]);
            }
            _ => {
                // Ignore other statement types for direct compilation demo
            }
        }
        Ok(())
    }

    fn compile_expr(&mut self, expr: &Expr) -> Result<(), String> {
        match expr {
            Expr::Int(n) => {
                let val = *n;
                if val >= -128 && val <= 127 {
                    // push imm8 -> 6a [val]
                    self.emit(&[0x6a, val as u8]);
                } else if val >= -2147483648 && val <= 2147483647 {
                    // push imm32 -> 68 [4 bytes val]
                    self.emit(&[0x68]);
                    self.emit(&(val as i32).to_le_bytes());
                } else {
                    // mov rax, imm64 -> 48 b8 [8 bytes val]
                    self.emit(&[0x48, 0xb8]);
                    self.emit(&val.to_le_bytes());
                    // push rax -> 50
                    self.emit(&[0x50]);
                }
            }
            Expr::Str(s) => {
                let offset = self.rodata.len();
                self.rodata.extend_from_slice(s.as_bytes());
                self.rodata.push(0); // null terminator

                // mov rsi, imm64 (placeholder address) -> 48 be [8 bytes]
                self.emit(&[0x48, 0xbe]);
                let relo_pos = self.code.len();
                self.emit(&[0; 8]); // address placeholder
                self.relocations.push((relo_pos, offset));

                // push rsi -> 56
                self.emit(&[0x56]);
            }
            Expr::Var(name) => {
                if let Some(&offset) = self.locals.get(name) {
                    // mov rax, [rbp - offset] -> 48 8b 45 [offset]
                    self.emit(&[0x48, 0x8b, 0x45, offset as i8 as u8]);
                    // push rax -> 50
                    self.emit(&[0x50]);
                } else {
                    return Err(format!("Undefined variable '{}'", name));
                }
            }
            Expr::BinOp { left, op, right } => {
                self.compile_expr(left)?;
                self.compile_expr(right)?;
                // pop rcx -> 59
                self.emit(&[0x59]);
                // pop rax -> 58
                self.emit(&[0x58]);

                match op.as_str() {
                    "+" => {
                        // add rax, rcx -> 48 01 c8
                        self.emit(&[0x48, 0x01, 0xc8]);
                    }
                    "-" => {
                        // sub rax, rcx -> 48 29 c8
                        self.emit(&[0x48, 0x29, 0xc8]);
                    }
                    "*" => {
                        // imul rax, rcx -> 48 0f af c1
                        self.emit(&[0x48, 0x0f, 0xaf, 0xc1]);
                    }
                    "/" => {
                        // cqo -> 48 99
                        self.emit(&[0x48, 0x99]);
                        // idiv rcx -> 48 f7 f9
                        self.emit(&[0x48, 0xf7, 0xf9]);
                    }
                    _ => return Err(format!("Unsupported operator '{}' in direct compilation", op)),
                }
                // push rax -> 50
                self.emit(&[0x50]);
            }
            Expr::Call { function, args } => {
                if let Expr::Var(name) = &**function {
                    if name == "print" && args.len() == 1 {
                        self.compile_expr(&args[0])?;
                        // pop rsi (pointer to string) -> 5e
                        self.emit(&[0x5e]);
                        
                        // Calculate length of string (basic helper inline assembly: find null byte)
                        // For simplicity, we assume writing up to 128 characters or inline length find.
                        // Let's implement a quick strlen in assembly:
                        // mov rdx, 0
                        // loop: cmp byte ptr [rsi + rdx], 0; je done; inc rdx; jmp loop; done:
                        // mov rax, 1 (sys_write)
                        // mov rdi, 1 (stdout)
                        // syscall
                        
                        // mov rdx, 0 -> 48 c7 c2 00 00 00 00
                        self.emit(&[0x48, 0xc7, 0xc2, 0x00, 0x00, 0x00, 0x00]);
                        // L_loop: cmp byte ptr [rsi + rdx], 0 -> 80 3c 16 00
                        let loop_start = self.code.len();
                        self.emit(&[0x80, 0x3c, 0x16, 0x00]);
                        // je done -> 74 05 (jump 5 bytes forward)
                        self.emit(&[0x74, 0x05]);
                        // inc rdx -> 48 ff c2
                        self.emit(&[0x48, 0xff, 0xc2]);
                        // jmp L_loop -> eb [relative offset]
                        let offset = (loop_start as i32 - (self.code.len() as i32 + 2)) as i8;
                        self.emit(&[0xeb, offset as u8]);
                        
                        // L_done: mov rax, 1 (sys_write) -> 48 c7 c0 01 00 00 00
                        self.emit(&[0x48, 0xc7, 0xc0, 0x01, 0x00, 0x00, 0x00]);
                        // mov rdi, 1 (stdout) -> 48 c7 c7 01 00 00 00
                        self.emit(&[0x48, 0xc7, 0xc7, 0x01, 0x00, 0x00, 0x00]);
                        // syscall -> 0f 05
                        self.emit(&[0x0f, 0x05]);
                        
                        // push 0 (as expression value) -> 6a 00
                        self.emit(&[0x6a, 0x00]);
                        return Ok(());
                    }
                }
                return Err("Unsupported function call in direct compilation".to_string());
            }
            _ => return Err("Unsupported expression type in direct compilation".to_string()),
        }
        Ok(())
    }

    fn link_elf(&mut self) -> Vec<u8> {
        let header_size = 120;
        let rodata_offset = header_size + self.code.len();
        let rodata_vaddr = 0x400000 + rodata_offset;

        // Resolve relocations (replace 8-byte placeholders with actual absolute virtual addresses of rodata strings)
        for &(relo_pos, rodata_off) in &self.relocations {
            let addr = (rodata_vaddr + rodata_off) as u64;
            let bytes = addr.to_le_bytes();
            for i in 0..8 {
                self.code[relo_pos + i] = bytes[i];
            }
        }

        // Build the ELF64 binary
        let file_size = header_size + self.code.len() + self.rodata.len();
        let code_vaddr = 0x400000 + header_size;

        let mut elf = Vec::with_capacity(file_size);

        // 1. ELF Header (64 bytes)
        elf.extend_from_slice(&[
            0x7f, 0x45, 0x4c, 0x46, // Magic
            2,                      // 64-bit
            1,                      // Little Endian
            1,                      // Version
            0,                      // System V ABI
            0,                      // ABI Version
            0, 0, 0, 0, 0, 0, 0,    // Padding
        ]);

        elf.extend_from_slice(&2u16.to_le_bytes()); // e_type: Executable (2)
        elf.extend_from_slice(&0x3eu16.to_le_bytes()); // e_machine: x86-64 (0x3e)
        elf.extend_from_slice(&1u32.to_le_bytes()); // e_version: 1
        elf.extend_from_slice(&code_vaddr.to_le_bytes()); // e_entry
        elf.extend_from_slice(&64u64.to_le_bytes()); // e_phoff: Program header offset (64)
        elf.extend_from_slice(&0u64.to_le_bytes()); // e_shoff
        elf.extend_from_slice(&0u32.to_le_bytes()); // e_flags
        elf.extend_from_slice(&64u16.to_le_bytes()); // e_ehsize: 64
        elf.extend_from_slice(&56u16.to_le_bytes()); // e_phentsize: 56
        elf.extend_from_slice(&1u16.to_le_bytes()); // e_phnum: 1
        elf.extend_from_slice(&0u16.to_le_bytes());
        elf.extend_from_slice(&0u16.to_le_bytes());
        elf.extend_from_slice(&0u16.to_le_bytes());

        // 2. Program Header (56 bytes)
        elf.extend_from_slice(&1u32.to_le_bytes()); // p_type: PT_LOAD (1)
        elf.extend_from_slice(&7u32.to_le_bytes()); // p_flags: PF_R | PF_W | PF_X (7)
        elf.extend_from_slice(&0u64.to_le_bytes()); // p_offset: 0
        elf.extend_from_slice(&0x400000u64.to_le_bytes()); // p_vaddr
        elf.extend_from_slice(&0x400000u64.to_le_bytes()); // p_paddr
        elf.extend_from_slice(&(file_size as u64).to_le_bytes()); // p_filesz
        elf.extend_from_slice(&(file_size as u64).to_le_bytes()); // p_memsz
        elf.extend_from_slice(&4096u64.to_le_bytes()); // p_align: 4096

        // 3. Append code & rodata segments
        elf.extend_from_slice(&self.code);
        elf.extend_from_slice(&self.rodata);

        elf
    }

    fn link_pe(&mut self) -> Vec<u8> {
        let header_size = 512;
        let rodata_offset = header_size + self.code.len();
        let rodata_vaddr = 0x400000 + 0x1000 + rodata_offset;

        for &(relo_pos, rodata_off) in &self.relocations {
            let addr = (rodata_vaddr + rodata_off) as u64;
            let bytes = addr.to_le_bytes();
            for i in 0..8 {
                self.code[relo_pos + i] = bytes[i];
            }
        }

        let mut pe = Vec::new();
        pe.extend_from_slice(&[
            0x4d, 0x5a,
            0x90, 0x00, 0x03, 0x00, 0x00, 0x00, 0x04, 0x00, 0x00, 0x00, 0xff, 0xff, 0x00, 0x00,
            0xb8, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x80, 0x00, 0x00, 0x00,
        ]);
        pe.extend_from_slice(&[0; 64]);
        pe.extend_from_slice(&[0x50, 0x45, 0x00, 0x00]);
        pe.extend_from_slice(&0x8664u16.to_le_bytes());
        pe.extend_from_slice(&1u16.to_le_bytes());
        pe.extend_from_slice(&0u32.to_le_bytes());
        pe.extend_from_slice(&0u32.to_le_bytes());
        pe.extend_from_slice(&0u32.to_le_bytes());
        pe.extend_from_slice(&240u16.to_le_bytes());
        pe.extend_from_slice(&0x0022u16.to_le_bytes());

        pe.extend_from_slice(&0x020bu16.to_le_bytes());
        pe.push(1);
        pe.push(0);
        pe.extend_from_slice(&((self.code.len() + self.rodata.len()) as u32).to_le_bytes());
        pe.extend_from_slice(&0u32.to_le_bytes());
        pe.extend_from_slice(&0u32.to_le_bytes());
        pe.extend_from_slice(&0x1000u32.to_le_bytes());
        pe.extend_from_slice(&0x1000u32.to_le_bytes());

        pe.extend_from_slice(&0x400000u64.to_le_bytes());
        pe.extend_from_slice(&0x1000u32.to_le_bytes());
        pe.extend_from_slice(&0x200u32.to_le_bytes());
        pe.extend_from_slice(&4u16.to_le_bytes());
        pe.extend_from_slice(&0u16.to_le_bytes());
        pe.extend_from_slice(&0u16.to_le_bytes());
        pe.extend_from_slice(&0u16.to_le_bytes());
        pe.extend_from_slice(&4u16.to_le_bytes());
        pe.extend_from_slice(&0u16.to_le_bytes());
        pe.extend_from_slice(&0u32.to_le_bytes());

        let file_size = header_size + self.code.len() + self.rodata.len();
        pe.extend_from_slice(&0x2000u32.to_le_bytes());
        pe.extend_from_slice(&(header_size as u32).to_le_bytes());
        pe.extend_from_slice(&0u32.to_le_bytes());
        pe.extend_from_slice(&3u16.to_le_bytes());
        pe.extend_from_slice(&0u16.to_le_bytes());
        pe.extend_from_slice(&0x100000u64.to_le_bytes());
        pe.extend_from_slice(&0x1000u64.to_le_bytes());
        pe.extend_from_slice(&0x100000u64.to_le_bytes());
        pe.extend_from_slice(&0x1000u64.to_le_bytes());
        pe.extend_from_slice(&0u32.to_le_bytes());
        pe.extend_from_slice(&16u32.to_le_bytes());

        for _ in 0..16 {
            pe.extend_from_slice(&[0; 8]);
        }

        pe.extend_from_slice(b".text\0\0\0");
        pe.extend_from_slice(&((self.code.len() + self.rodata.len()) as u32).to_le_bytes());
        pe.extend_from_slice(&0x1000u32.to_le_bytes());
        pe.extend_from_slice(&((self.code.len() + self.rodata.len()) as u32).to_le_bytes());
        pe.extend_from_slice(&0x200u32.to_le_bytes());
        pe.extend_from_slice(&[0; 12]);
        pe.extend_from_slice(&0x60000020u32.to_le_bytes());

        while pe.len() < 512 {
            pe.push(0);
        }

        pe.extend_from_slice(&self.code);
        pe.extend_from_slice(&self.rodata);
        pe
    }

    fn link_bin(&mut self) -> Vec<u8> {
        let rodata_offset = self.code.len();
        for &(relo_pos, rodata_off) in &self.relocations {
            let addr = (rodata_offset + rodata_off) as u64;
            let bytes = addr.to_le_bytes();
            for i in 0..8 {
                self.code[relo_pos + i] = bytes[i];
            }
        }
        let mut bin = Vec::new();
        bin.extend_from_slice(&self.code);
        bin.extend_from_slice(&self.rodata);
        bin
    }
}

pub fn compile_direct_to_elf(source: &str, output_path: &str) -> Result<(), String> {
    let program = parse_program(source);
    let bytes = DirectCompiler::compile_program_to_elf(&program)?;
    std::fs::write(output_path, bytes)
        .map_err(|e| format!("Failed to write direct ELF executable: {}", e))?;
    
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = std::fs::metadata(output_path) {
            let mut perms = metadata.permissions();
            perms.set_mode(0o755);
            std::fs::set_permissions(output_path, perms).ok();
        }
    }
    Ok(())
}

pub fn compile_direct_to_pe(source: &str, output_path: &str) -> Result<(), String> {
    let program = parse_program(source);
    let bytes = DirectCompiler::compile_program_to_pe(&program)?;
    std::fs::write(output_path, bytes)
        .map_err(|e| format!("Failed to write direct PE executable: {}", e))?;
    Ok(())
}

pub fn compile_direct_to_bin(source: &str, output_path: &str) -> Result<(), String> {
    let program = parse_program(source);
    let bytes = DirectCompiler::compile_program_to_bin(&program)?;
    std::fs::write(output_path, bytes)
        .map_err(|e| format!("Failed to write direct binary file: {}", e))?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_direct_elf_compilation() {
        let source = "let msg = \"Hello Direct Machine Code!\\n\"\nprint(msg)\nlet x = 10\nlet y = 20\nlet z = x + y * 2\nreturn z\n";
        let temp_bin = "test_direct_elf.bin";
        let result = compile_direct_to_elf(source, temp_bin);
        assert!(result.is_ok(), "Direct ELF compilation failed: {:?}", result.err());

        // Clean up
        std::fs::remove_file(temp_bin).ok();
    }
}
