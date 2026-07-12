pub mod ast;
pub mod codegen;
pub mod parser;

pub fn compile_source(source: &str) -> Result<String, String> {
    let program = parser::parse_program(source);
    let mut generator = codegen::CodeGenerator::new();
    let c_code = generator.generate(&program);
    Ok(c_code)
}

pub fn compile_and_write_to_file(aly_path: &str, output_name: &str) -> Result<(), String> {
    let source = std::fs::read_to_string(aly_path)
        .map_err(|e| format!("Compiler Error: Failed to read source file '{}': {}", aly_path, e))?;

    let c_code = compile_source(&source)?;

    let output_path = format!("{}.c", output_name);
    std::fs::write(&output_path, c_code)
        .map_err(|e| format!("Compiler Error: Failed to write C file '{}': {}", output_path, e))?;

    let mut cmd = std::process::Command::new("gcc")
        .args([
            "-x", "c",
            "-static",
            "-w",
            "-o", output_name,
            &output_path,
            "-lm",
        ])
        .spawn()
        .map_err(|e| format!("Compiler Error: Failed to spawn gcc: {}", e))?;

    let status = cmd.wait()
        .map_err(|e| format!("Compiler Error: Failed to wait for gcc: {}", e))?;

    if !status.success() {
        let output = std::fs::read_to_string(&output_path)
            .unwrap_or_default();
        return Err(format!("Compilation failed.\nC code:\n{}\n", output));
    }

    Ok(())
}