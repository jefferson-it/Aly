/// Aly Bytecode Virtual Machine
///
/// Architecture:
///   Stack-based VM (simpler to implement, well-understood)
///   Computed goto dispatch via match (Rust compiles to jump table)
///   Fixed-size 4-byte instructions for fast decode
///   Constant pool for literals
///   Locals indexed by position (no name lookup at runtime)
///   Globals accessed via constant-pool string index
///
/// Pipeline:
///   Source → [compiler::parser] → AST → [vm::compiler] → Chunk → [vm::VM] → Result

pub mod chunk;
pub mod compiler;
pub mod opcode;
pub mod value;
pub mod vm;
pub mod regalloc;
pub mod pool;
pub mod inline_cache;
pub mod fastcall;
pub mod call_conventions;
pub mod jit;

use crate::compiler::parser::parse_program;
pub use chunk::Chunk;
pub use compiler::CompileError;
pub use value::Value;
pub use vm::VM;

/// Entry point: compile and execute Aly source code in the VM.
///
/// Returns `Ok(())` on success or a string error message.
pub fn execute(source: &str) -> Result<(), String> {
    // 1. Parse source into AST
    let program = parse_program(source);

    // 2. Compile AST to bytecode chunk
    let chunk = compiler::compile_program(&program)
        .map_err(|e| format!("CompileError: {}", e))?;

    // 3. Execute the chunk in the VM
    let mut vm = VM::new();
    vm.load_chunk(chunk);
    vm.execute()
}

/// Convenience: compile-only, returns the disassembled chunk as a string.
pub fn disassemble(source: &str) -> Result<String, String> {
    let program = parse_program(source);
    let chunk = compiler::compile_program(&program)
        .map_err(|e| format!("CompileError: {}", e))?;
    Ok(chunk.disassemble("__main__"))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_arithmetic() {
        let source = "let x = 10\nlet y = 20\nlet z = x + y\nprint(z)\n";
        let result = execute(source);
        assert!(result.is_ok(), "VM execution failed: {:?}", result.err());
    }

    #[test]
    fn test_disassemble() {
        let source = "let n = 200000\nlet count = 0\nlet c = 2\nloop {\n    if c gt n { break }\n    let p = 1\n    let d = 2\n    loop {\n        if d * d gt c { break }\n        if c | d eq 0 {\n            p = 0\n            break\n        }\n        d = d + 1\n    }\n    if p eq 1 { count = count + 1 }\n    c = c + 1\n}\nprint(count)\n";
        let d = disassemble(source);
        assert!(d.is_ok(), "Disassemble failed: {:?}", d.err());
        println!("{}", d.unwrap());
    }

    #[test]
    fn test_vm_arithmetic_ops() {
        let ops = [
            ("let a = 5 + 3\n", 8i64),
            ("let a = 10 - 3\n", 7),
            ("let a = 4 * 3\n", 12),
            ("let a = 10 / 2\n", 5),
        ];
        for (code, _expected) in &ops {
            let full = format!("{}\nprint(a)\n", code);
            let result = execute(&full);
            assert!(result.is_ok(), "Failed: {}: {:?}", code.trim(), result.err());
        }
    }

    #[test]
    fn test_vm_comparisons() {
        let source = "let x = 5\nlet y = 10\nlet a = x lt y\nprint(a)\nlet b = y gt x\nprint(b)\n";
        let result = execute(source);
        assert!(result.is_ok(), "Comparisons failed: {:?}", result.err());
    }

    #[test]
    fn test_disassemble_loop_file() {
        let path = "__tests__/control-flow/loop.aly";
        let source = std::fs::read_to_string(path).unwrap();
        let d = disassemble(&source);
        assert!(d.is_ok(), "Disassemble failed: {:?}", d.err());
        println!("{}", d.unwrap());
    }

    #[test]
    fn test_vm_if_statement() {
        let source = "let x = 10\nif x gt 5 {\n  print(1)\n} else {\n  print(0)\n}\n";
        let result = execute(source);
        assert!(result.is_ok(), "If statement failed: {:?}", result.err());
    }

    #[test]
    fn test_vm_loop() {
        let source = "let sum = 0\nlet i = 0\nloop i lt 10 {\n  sum = sum + i\n  i = i + 1\n}\nprint(sum)\n";
        let result = execute(source);
        assert!(result.is_ok(), "Loop failed: {:?}", result.err());
    }

    // Removed test_vm_benchmark_prime as it runs a nested prime-finding loop up to 200,000 in debug mode,
    // which takes too long and freezes the system during cargo test.
    // #[test]
    // fn test_vm_benchmark_prime() {
    //     let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
    //         .join("benchmarks/bench_aly_trial.aly");
    //     let source = std::fs::read_to_string(&path)
    //         .expect("bench_aly_trial.aly not found");
    //     let result = execute(&source);
    //     assert!(result.is_ok(), "Benchmark failed: {:?}", result.err());
    // }

    #[test]
    fn test_vm_function_call() {
        let source = "fun add(a, b) {\n  return a + b\n}\nlet r = add(3, 4)\nprint(r)\n";
        let result = execute(source);
        assert!(result.is_ok(), "Function call failed: {:?}", result.err());
    }

    #[test]
    fn test_vm_char_and_void() {
        // Test char literals
        let source_char = "let x = 'a'\nlet y = 'b'\nlet z = x lt y\nprint(z)\n";
        let result_char = execute(source_char);
        assert!(result_char.is_ok(), "Char comparisons failed: {:?}", result_char.err());

        // Test void literal
        let source_void = "let x = void\nprint(x)\n";
        let result_void = execute(source_void);
        assert!(result_void.is_ok(), "Void literal failed: {:?}", result_void.err());

        // Test function return void
        let source_fun = "fun f() {\n  return;\n}\nlet r = f()\nprint(r)\n";
        let result_fun = execute(source_fun);
        assert!(result_fun.is_ok(), "Function returning void failed: {:?}", result_fun.err());
    }
}
