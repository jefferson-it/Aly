use crate::compiler::{CompileOptions, compile_to_assembly};

pub fn compile_to_object(source: &str, options: &CompileOptions, output_path: &str) -> Result<(), String> {
    let asm = compile_to_assembly(source, options)?;
    
    let asm_path = format!("{}.s", output_path);
    std::fs::write(&asm_path, asm)
        .map_err(|e| format!("Failed to write assembly: {}", e))?;
    
    let mut cmd = std::process::Command::new("gcc");
    cmd.args(["-c", &asm_path, "-o", output_path]);
    
    if options.debug_info {
        cmd.arg("-g");
    }
    
    if options.verbose {
        println!("Assembling: {:?}", cmd);
    }
    
    let status = cmd.status()
        .map_err(|e| format!("Failed to run assembler: {}", e))?;
    
    std::fs::remove_file(&asm_path).ok();
    
    if !status.success() {
        return Err("Assembly failed".to_string());
    }
    
    Ok(())
}

pub fn link_objects(object_files: &[&str], output_path: &str, options: &CompileOptions) -> Result<(), String> {
    let mut cmd = std::process::Command::new("gcc");
    
    if options.shared {
        cmd.args(["-shared", "-fPIC"]);
    }
    
    if options.debug_info {
        cmd.arg("-g");
    }
    
    cmd.args(object_files);
    
    if options.shared {
        let ext = if cfg!(target_os = "windows") { ".dll" }
                  else if cfg!(target_os = "macos") { ".dylib" }
                  else { ".so" };
        cmd.args(["-o", &format!("{}{}", output_path, ext)]);
    } else {
        cmd.args(["-o", output_path]);
    }
    
    cmd.arg("-lm");
    
    if options.verbose {
        println!("Linking: {:?}", cmd);
    }
    
    let status = cmd.status()
        .map_err(|e| format!("Failed to run linker: {}", e))?;
    
    if !status.success() {
        return Err("Linking failed".to_string());
    }
    
    Ok(())
}

pub fn create_static_library(object_files: &[&str], output_path: &str) -> Result<(), String> {
    let mut cmd = std::process::Command::new("ar");
    cmd.args(["rcs", output_path]);
    cmd.args(object_files);
    
    let status = cmd.status()
        .map_err(|e| format!("Failed to run ar: {}", e))?;
    
    if !status.success() {
        return Err("Static library creation failed".to_string());
    }
    
    Ok(())
}