use crate::compiler::CompileOptions;

pub fn compile_to_cpp(source: &str, options: &CompileOptions) -> Result<String, String> {
    let c_code = crate::compiler::compile_to_c(source, options)?;
    
    let cpp_code = format!(
        "// Generated from Aly source\n// Target: {}\n// Opt: {}\n// Debug: {}\n\n\
        extern \"C\" {{\n    {}\n}}\n\n\
        int main(int argc, char** argv) {{\n    // C++ wrapper\n    return 0;\n}}\n",
        options.target_triple.as_str(),
        options.opt_level.gcc_flag(),
        options.debug_info,
        c_code.replace("int main", "int aly_main")
    );
    
    Ok(cpp_code)
}

pub fn compile_cpp_to_binary(source: &str, options: &CompileOptions, output_path: &str) -> Result<(), String> {
    let cpp_code = compile_to_cpp(source, options)?;
    
    let cpp_path = format!("{}_aly_tmp.cpp", output_path);
    std::fs::write(&cpp_path, cpp_code)
        .map_err(|e| format!("Failed to write C++ file: {}", e))?;
    
    let header = include_str!("runtime_aly.h");
    let header_path = format!("{}/runtime_aly.h", std::path::Path::new(&cpp_path).parent().unwrap_or(std::path::Path::new(".")).display());
    std::fs::write(&header_path, header)
        .map_err(|e| format!("Failed to write runtime header: {}", e))?;
    
    let mut cmd = std::process::Command::new("g++");
    cmd.args([options.opt_level.gcc_flag(), "-march=native", "-flto", "-std=c++17"]);
    
    if options.debug_info {
        cmd.arg("-g");
    }
    
    if options.shared {
        cmd.args(["-shared", "-fPIC", "-o", &format!("{}.so", output_path)]);
    } else {
        cmd.args(["-o", output_path]);
    }
    
    cmd.args([&cpp_path, "-lm"]);
    
    if options.verbose {
        println!("Compiling C++: {:?}", cmd);
    }
    
    let status = cmd.status()
        .map_err(|e| format!("Failed to run g++: {}", e))?;
    
    std::fs::remove_file(&cpp_path).ok();
    std::fs::remove_file(&header_path).ok();
    
    if !status.success() {
        return Err("C++ compilation failed".to_string());
    }
    
    Ok(())
}