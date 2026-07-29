pub mod ast;
pub mod asm;
pub mod codegen;
pub mod parser;
pub mod optimizer;
pub mod mir;
pub mod hir;
pub mod hir_to_mir;
pub mod passes;
pub mod type_inference;
pub mod arena;
pub mod const_eval;
pub mod kotlin;
pub mod cpp_backend;
pub mod js_backend;
pub mod python_backend;
pub mod object_backend;
pub mod direct_elf;
pub mod go_backend;
pub mod rust_backend;
pub mod sh_backend;
pub mod jvm_backend;
pub mod target;
pub mod build;
pub mod parallel;
// FIXME: broken external dependencies, disabled until fixed
// pub mod llvm_backend;

use std::path::Path;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum OptLevel {
    #[default]
    None,
    O1,
    O2,
    O3,
    Os,
    Oz,
}

impl OptLevel {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "none" | "o0" | "O0" => OptLevel::None,
            "o1" | "O1" => OptLevel::O1,
            "o2" | "O2" => OptLevel::O2,
            "o3" | "O3" => OptLevel::O3,
            "os" | "Os" => OptLevel::Os,
            "oz" | "Oz" => OptLevel::Oz,
            _ => OptLevel::O2,
        }
    }

    pub fn gcc_flag(&self) -> &'static str {
        match self {
            OptLevel::None => "-O0",
            OptLevel::O1 => "-O1",
            OptLevel::O2 => "-O2",
            OptLevel::O3 => "-O3",
            OptLevel::Os => "-Os",
            OptLevel::Oz => "-Oz",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AsmSyntax {
    Intel,
    Att,
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

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TargetTriple {
    X86_64Linux,
    AArch64Linux,
    X86_64Windows,
    AArch64Windows,
    X86_64Macos,
    AArch64Macos,
    Custom(String),
}

impl TargetTriple {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "x86_64-linux" | "x86_64-unknown-linux-gnu" => TargetTriple::X86_64Linux,
            "aarch64-linux" | "aarch64-unknown-linux-gnu" | "arm64-linux" => TargetTriple::AArch64Linux,
            "x86_64-windows" | "x86_64-pc-windows-msvc" => TargetTriple::X86_64Windows,
            "aarch64-windows" | "aarch64-pc-windows-msvc" => TargetTriple::AArch64Windows,
            "x86_64-macos" | "x86_64-apple-darwin" => TargetTriple::X86_64Macos,
            "aarch64-macos" | "aarch64-apple-darwin" | "arm64-macos" => TargetTriple::AArch64Macos,
            s => TargetTriple::Custom(s.to_string()),
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            TargetTriple::X86_64Linux => "x86_64-unknown-linux-gnu",
            TargetTriple::AArch64Linux => "aarch64-unknown-linux-gnu",
            TargetTriple::X86_64Windows => "x86_64-pc-windows-msvc",
            TargetTriple::AArch64Windows => "aarch64-pc-windows-msvc",
            TargetTriple::X86_64Macos => "x86_64-apple-darwin",
            TargetTriple::AArch64Macos => "aarch64-apple-darwin",
            TargetTriple::Custom(s) => s,
        }
    }

    pub fn arch(&self) -> Arch {
        match self {
            TargetTriple::X86_64Linux | TargetTriple::X86_64Windows | TargetTriple::X86_64Macos => Arch::X86_64,
            TargetTriple::AArch64Linux | TargetTriple::AArch64Windows | TargetTriple::AArch64Macos => Arch::AArch64,
            TargetTriple::Custom(s) => {
                if s.starts_with("x86_64") || s.starts_with("amd64") { Arch::X86_64 }
                else { Arch::AArch64 }
            }
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum EmitFormat {
    Assembly,
    Object,
    Binary,
    SharedLibrary,
    C,
    Cpp,
    JavaScript,
    Python,
    Kotlin,
    BareMetal,  // Bare metal / no-std mode for kernels/drivers
    Go,
    Rust,
    Shell,
    Jvm,
}

impl EmitFormat {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "asm" | "s" | "assembly" => EmitFormat::Assembly,
            "obj" | "o" | "object" => EmitFormat::Object,
            "bin" | "binary" | "exe" => EmitFormat::Binary,
            "so" | "shared" | "dylib" | "dll" => EmitFormat::SharedLibrary,
            "c" => EmitFormat::C,
            "cpp" | "c++" | "cxx" => EmitFormat::Cpp,
            "js" | "javascript" => EmitFormat::JavaScript,
            "py" | "python" => EmitFormat::Python,
            "kt" | "kotlin" => EmitFormat::Kotlin,
            "go" | "golang" => EmitFormat::Go,
            "rs" | "rust" => EmitFormat::Rust,
            "sh" | "shell" | "bash" => EmitFormat::Shell,
            "jvm" | "java" | "class" => EmitFormat::Jvm,
            "bare" | "baremetal" | "nostd" | "no-std" => EmitFormat::BareMetal,
            _ => EmitFormat::Binary,
        }
    }

    pub fn from_extension(ext: &str) -> Self {
        match ext.to_lowercase().as_str() {
            "asm" | "s" => EmitFormat::Assembly,
            "o" | "obj" => EmitFormat::Object,
            "c" => EmitFormat::C,
            "cpp" | "cc" | "cxx" | "c++" => EmitFormat::Cpp,
            "js" => EmitFormat::JavaScript,
            "py" => EmitFormat::Python,
            "kt" | "kts" => EmitFormat::Kotlin,
            "go" => EmitFormat::Go,
            "rs" => EmitFormat::Rust,
            "sh" => EmitFormat::Shell,
            "class" | "java" => EmitFormat::Jvm,
            "so" | "dylib" | "dll" | "so.1" => EmitFormat::SharedLibrary,
            "elf" | "img" => EmitFormat::BareMetal,
            "bin" => EmitFormat::Binary,
            _ => EmitFormat::Binary,
        }
    }

    pub fn default_extension(&self) -> &'static str {
        match self {
            EmitFormat::Assembly => "s",
            EmitFormat::Object => "o",
            EmitFormat::Binary => "",
            EmitFormat::SharedLibrary => if cfg!(target_os = "windows") { "dll" } else if cfg!(target_os = "macos") { "dylib" } else { "so" },
            EmitFormat::C => "c",
            EmitFormat::Cpp => "cpp",
            EmitFormat::JavaScript => "js",
            EmitFormat::Python => "py",
            EmitFormat::Kotlin => "kt",
            EmitFormat::Go => "go",
            EmitFormat::Rust => "rs",
            EmitFormat::Shell => "sh",
            EmitFormat::Jvm => "java",
            EmitFormat::BareMetal => "elf",
        }
    }
}

#[derive(Debug, Clone)]
pub struct CompileOptions {
    pub input_file: String,
    pub output_file: Option<String>,
    pub emit_format: EmitFormat,
    pub opt_level: OptLevel,
    pub target_triple: TargetTriple,
    pub arch: Arch,
    pub asm_syntax: AsmSyntax,
    pub debug_info: bool,
    pub shared: bool,
    pub verbose: bool,
    pub emit_asm_only: bool,
    // Bare metal / no-std options
    pub bare_metal: bool,
    pub no_std: bool,
    pub entry_point: Option<String>,
    pub linker_script: Option<String>,
    pub no_stdlib: bool,
    pub no_runtime: bool,
    pub no_gc: bool,
    pub direct: bool,
}

impl Default for CompileOptions {
    fn default() -> Self {
        Self {
            input_file: String::new(),
            output_file: None,
            emit_format: EmitFormat::Binary,
            opt_level: OptLevel::O2,
            target_triple: TargetTriple::X86_64Linux,
            arch: Arch::from_host(),
            asm_syntax: AsmSyntax::Att,
            debug_info: false,
            shared: false,
            verbose: false,
            emit_asm_only: false,
            bare_metal: false,
            no_std: false,
            entry_point: None,
            linker_script: None,
            no_stdlib: false,
            no_runtime: false,
            no_gc: false,
            direct: false,
        }
    }
}

impl CompileOptions {
    pub fn from_args(args: &[String]) -> Result<Self, String> {
        let mut opts = CompileOptions::default();
        let mut i = 0;
        let mut input_file = None;

        while i < args.len() {
            let arg = &args[i];
            match arg.as_str() {
                "-o" | "--output" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("Missing argument for -o/--output".to_string());
                    }
                    opts.output_file = Some(args[i].clone());
                }
                "-S" | "--assembly" => {
                    opts.emit_format = EmitFormat::Assembly;
                    opts.emit_asm_only = true;
                }
                "-emit" | "--emit" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("Missing argument for -emit/--emit".to_string());
                    }
                    opts.emit_format = EmitFormat::from_str(&args[i]);
                }
                "-O0" => opts.opt_level = OptLevel::None,
                "-O1" => opts.opt_level = OptLevel::O1,
                "-O2" => opts.opt_level = OptLevel::O2,
                "-O3" => opts.opt_level = OptLevel::O3,
                "-Os" => opts.opt_level = OptLevel::Os,
                "-Oz" => opts.opt_level = OptLevel::Oz,
                "--target" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("Missing argument for --target".to_string());
                    }
                    opts.target_triple = TargetTriple::from_str(&args[i]);
                    opts.arch = opts.target_triple.arch();
                }
                "--arch" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("Missing argument for --arch".to_string());
                    }
                    opts.arch = Arch::from_str(&args[i]);
                }
                "--masm" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("Missing argument for --masm".to_string());
                    }
                    opts.asm_syntax = AsmSyntax::from_str(&args[i]);
                }
                "-g" | "--debug" => {
                    opts.debug_info = true;
                }
                "--shared" | "-shared" => {
                    opts.shared = true;
                    opts.emit_format = EmitFormat::SharedLibrary;
                }
                "-v" | "--verbose" => {
                    opts.verbose = true;
                }
                "--direct" | "-direct" => {
                    opts.direct = true;
                }
                "-h" | "--help" => {
                    print_comp_help();
                    std::process::exit(0);
                }
                "--bare-metal" | "--no-std" | "--nostd" => {
                    opts.bare_metal = true;
                    opts.no_std = true;
                    opts.emit_format = EmitFormat::BareMetal;
                }
                "--no-stdlib" => {
                    opts.no_stdlib = true;
                    opts.no_std = true;
                }
                "--no-runtime" => {
                    opts.no_runtime = true;
                }
                "--no-gc" => {
                    opts.no_gc = true;
                }
                "--entry" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("Missing argument for --entry".to_string());
                    }
                    opts.entry_point = Some(args[i].clone());
                }
                "--ld-script" | "--linker-script" => {
                    i += 1;
                    if i >= args.len() {
                        return Err("Missing argument for --ld-script".to_string());
                    }
                    opts.linker_script = Some(args[i].clone());
                }
                "--no-runtime" => {
                    opts.no_runtime = true;
                }
                "--no-gc" => {
                    opts.no_gc = true;
                }
                arg if arg.starts_with('-') => {
                    return Err(format!("Unknown option: {}", arg));
                }
                _ => {
                    if input_file.is_none() {
                        input_file = Some(arg.clone());
                    } else {
                        return Err(format!("Unexpected argument: {}", arg));
                    }
                }
            }
            i += 1;
        }

        opts.input_file = input_file.ok_or("Missing input file")?;

        if opts.output_file.is_none() {
            let path = Path::new(&opts.input_file);
            let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("a.out");
            let ext = opts.emit_format.default_extension();
            if ext.is_empty() {
                opts.output_file = Some(stem.to_string());
            } else {
                opts.output_file = Some(format!("{}.{}", stem, ext));
            }
        }

        Ok(opts)
    }
}

fn print_comp_help() {
    println!("Usage: aly comp [options] <file.aly>");
    println!();
    println!("Options:");
    println!("  -o <file>          Specify output file");
    println!("  -S, --assembly     Emit assembly only (don't assemble/link)");
    println!("  -emit <format>     Override output format (asm, c, cpp, js, py, kt, obj, bin, so, bare)");
    println!("  -O0, -O1, -O2, -O3, -Os, -Oz  Optimization level");
    println!("  --target <triple>  Target triple (x86_64-linux, aarch64-linux, x86_64-windows, etc.)");
    println!("  --arch <arch>      Target architecture (x86_64, aarch64)");
    println!("  --masm <intel|att> Assembly syntax style");
    println!("  -g, --debug        Emit debug information");
    println!("  --shared, -shared  Build shared library");
    println!("  -v, --verbose      Verbose output");
    println!("  -h, --help         Show this help");
    println!();
    println!("Bare Metal / Kernel Options:");
    println!("  --bare-metal, --no-std, --nostd    Bare metal mode (no stdlib, no runtime)");
    println!("  --no-stdlib                        Disable standard library");
    println!("  --no-runtime                       Disable runtime (GC, runtime init)");
    println!("  --no-gc                            Disable garbage collector");
    println!("  --no-stdlib                        Disable standard library");
    println!("  --entry <name>                     Custom entry point (default: _start)");
    println!("  --ld-script, --linker-script <file> Custom linker script");
    println!("  -v, --verbose      Verbose output");
    println!("  -h, --help         Show this help");
    println!();
    println!("Examples:");
    println!("  aly comp hello.aly                    # Compile to binary");
    println!("  aly comp -o hello hello.aly           # Specify output");
    println!("  aly comp -S hello.aly                 # Emit assembly");
    println!("  aly comp -emit asm hello.aly          # Emit assembly");
    println!("  aly comp -emit c hello.aly            # Emit C code");
    println!("  aly comp -emit cpp hello.aly          # Emit C++ code");
    println!("  aly comp -emit js hello.aly           # Emit JavaScript");
    println!("  aly comp -emit py hello.aly           # Emit Python");
    println!("  aly comp -emit kt hello.aly           # Emit Kotlin");
    println!("  aly comp -O3 hello.aly                # Maximum optimization");
    println!("  aly comp --target aarch64-linux hello.aly  # Cross-compile");
    println!("  aly comp --shared hello.aly           # Build shared library");
    println!("  aly comp -g hello.aly                 # Debug build");
    println!("  aly comp --bare-metal --entry _start kernel.aly -o kernel.elf  # Kernel module");
    println!("  aly comp --bare-metal --entry _start boot.aly -o boot.bin  # Bootloader");
    println!("  aly comp --bare-metal --no-gc driver.aly -o driver.ko  # Kernel driver");
    println!();
    println!("Auto-detect format by extension:");
    println!("  .asm/.s   -> Assembly");
    println!("  .o        -> Object file");
    println!("  .c        -> C");
    println!("  .cpp      -> C++");
    println!("  .js       -> JavaScript");
    println!("  .py       -> Python");
    println!("  .kt       -> Kotlin");
    println!("  .so/.dylib/.dll -> Shared library");
    println!("  .bin/.elf -> Bare metal binary");
    println!("  (default) -> Binary");
}

pub fn compile_to_kotlin(source: &str) -> Result<String, String> {
    let program = parser::parse_program(source);
    let mut transpiler = kotlin::KotlinTranspiler::new();
    transpiler.transpile(&program)
}

pub fn compile_source(source: &str) -> Result<String, String> {
    let program = parser::parse_program(source);
    let chunk = crate::vm::compiler::compile_program(&program)
        .map_err(|e| format!("CompileError: {}", e))?;
    let mut generator = codegen::CodeGenerator::new();
    let c_code = generator.generate(&chunk);
    Ok(c_code)
}

pub fn compile_to_c(source: &str, options: &CompileOptions) -> Result<String, String> {
    let c_code = compile_source(source)?;
    
    if options.debug_info {
        Ok(format!("// Debug info enabled\n// Target: {}\n// Opt: {}\n\n{}", 
            options.target_triple.as_str(), options.opt_level.gcc_flag(), c_code))
    } else {
        Ok(c_code)
    }
}

pub fn compile_to_cpp(source: &str, options: &CompileOptions) -> Result<String, String> {
    let c_code = compile_source(source)?;
    
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

pub fn compile_to_js(source: &str, _options: &CompileOptions) -> Result<String, String> {
    let program = parser::parse_program(source);
    let chunk = crate::vm::compiler::compile_program(&program)
        .map_err(|e| format!("CompileError: {}", e))?;
    
    let mut js = String::new();
    js.push_str("// Generated from Aly source\n");
    js.push_str("const aly_runtime = require('./runtime_aly.js');\n\n");
    js.push_str("function aly_main() {\n");
    js.push_str("    // TODO: Implement JS codegen from chunk\n");
    js.push_str("    console.log('Aly program compiled to JS');\n");
    js.push_str("}\n\n");
    js.push_str("aly_main();\n");
    
    Ok(js)
}

pub fn compile_to_python(source: &str, _options: &CompileOptions) -> Result<String, String> {
    let program = parser::parse_program(source);
    let chunk = crate::vm::compiler::compile_program(&program)
        .map_err(|e| format!("CompileError: {}", e))?;
    
    let mut py = String::new();
    py.push_str("# Generated from Aly source\n");
    py.push_str("import sys\n");
    py.push_str("sys.path.insert(0, '.')\n");
    py.push_str("import runtime_aly\n\n");
    py.push_str("def aly_main():\n");
    py.push_str("    # TODO: Implement Python codegen from chunk\n");
    py.push_str("    print('Aly program compiled to Python')\n\n");
    py.push_str("if __name__ == '__main__':\n");
    py.push_str("    aly_main()\n");
    
    Ok(py)
}

pub fn compile_to_assembly(source: &str, options: &CompileOptions) -> Result<String, String> {
    let program = parser::parse_program(source);
    let chunk = crate::vm::compiler::compile_program(&program)
        .map_err(|e| format!("CompileError: {}", e))?;
    
    let arch = match options.arch {
        Arch::X86_64 => asm::Arch::X86_64,
        Arch::AArch64 => asm::Arch::AArch64,
    };
    let mut backend = asm::AsmBackend::new(arch);
    backend.set_opt_level(match options.opt_level {
        OptLevel::None => asm::OptLevel::None,
        OptLevel::O1 => asm::OptLevel::Fast,
        OptLevel::O2 => asm::OptLevel::Moderate,
        OptLevel::O3 => asm::OptLevel::Aggressive,
        OptLevel::Os => asm::OptLevel::Size,
        OptLevel::Oz => asm::OptLevel::SizeZ,
    });
    backend.set_target_triple(options.target_triple.as_str());
    backend.set_emit_type(asm::EmitType::Assembly);
    backend.set_asm_syntax(match options.asm_syntax {
        AsmSyntax::Intel => asm::AsmSyntax::Intel,
        AsmSyntax::Att => asm::AsmSyntax::Att,
    });
    
    backend.compile_chunk(&chunk, "main")
        .map_err(|e| format!("Assembly generation failed: {}", e))?;
    
    Ok(backend.take_output())
}

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

pub fn compile_to_binary(source: &str, options: &CompileOptions, output_path: &str) -> Result<(), String> {
    let c_code = compile_to_c(source, options)?;
    
    let c_path = format!("{}_aly_tmp.c", output_path);
    std::fs::write(&c_path, c_code)
        .map_err(|e| format!("Failed to write C file: {}", e))?;
    
    let header = include_str!("runtime_aly.h");
    let header_path = format!("{}/runtime_aly.h", Path::new(&c_path).parent().unwrap_or(Path::new(".")).display());
    std::fs::write(&header_path, header)
        .map_err(|e| format!("Failed to write runtime header: {}", e))?;
    
    let mut cmd = std::process::Command::new("gcc");
    cmd.args([options.opt_level.gcc_flag(), "-march=native", "-flto"]);
    
    if options.debug_info {
        cmd.arg("-g");
    }
    
    if options.shared {
        cmd.args(["-shared", "-fPIC", "-o", &format!("{}.so", output_path)]);
    } else {
        cmd.args(["-o", output_path]);
    }
    
    cmd.args([&c_path, "-lm"]);
    
    if options.verbose {
        println!("Compiling: {:?}", cmd);
    }
    
    let status = cmd.status()
        .map_err(|e| format!("Failed to run gcc: {}", e))?;
    
    std::fs::remove_file(&c_path).ok();
    std::fs::remove_file(&header_path).ok();
    
    if !status.success() {
        return Err("Compilation failed".to_string());
    }
    
    Ok(())
}

pub fn compile_and_write_to_file(aly_path: &str, output_name: &str) -> Result<(), String> {
    let source = std::fs::read_to_string(aly_path)
        .map_err(|e| format!("Compiler Error: Failed to read source file '{}': {}", aly_path, e))?;

    let c_code = compile_source(&source)?;

    let tmp_c = format!("{}_aly_tmp.c", output_name);
    let asm_path = format!("{}.s", output_name);
    let obj_path = format!("{}_aly_tmp.o", output_name);

    std::fs::write(&tmp_c, &c_code)
        .map_err(|e| format!("Compiler Error: Failed to write temp C file '{}': {}", tmp_c, e))?;

    let header = include_str!("runtime_aly.h");
    let tmp_dir = std::path::Path::new(&tmp_c)
        .parent()
        .unwrap_or(std::path::Path::new("."));
    let header_path = format!("{}/runtime_aly.h", tmp_dir.display());
    std::fs::write(&header_path, header)
        .map_err(|e| format!("Compiler Error: Failed to write runtime_aly.h '{}': {}", header_path, e))?;

    let status = std::process::Command::new("gcc")
        .args(["-O3", "-march=native", "-flto", "-S", "-x", "c", "-std=gnu11", "-w", "-o", &asm_path, &tmp_c])
        .spawn()
        .map_err(|e| format!("Compiler Error: Failed to spawn gcc (C→asm): {}", e))?
        .wait()
        .map_err(|e| format!("Compiler Error: Failed to wait for gcc (C→asm): {}", e))?;

    if !status.success() {
        let c_out = std::fs::read_to_string(&tmp_c).unwrap_or_default();
        let _ = std::fs::remove_file(&tmp_c);
        let _ = std::fs::remove_file(&header_path);
        return Err(format!("Compilation to assembly failed.\nC code:\n{}\n", c_out));
    }

    let _ = std::fs::remove_file(&tmp_c);
    let _ = std::fs::remove_file(&header_path);

    let status = std::process::Command::new("as")
        .args(["-o", &obj_path, &asm_path])
        .spawn()
        .map_err(|e| format!("Compiler Error: Failed to spawn as: {}", e))?
        .wait()
        .map_err(|e| format!("Compiler Error: Failed to wait for as: {}", e))?;

    if !status.success() {
        let _ = std::fs::remove_file(&obj_path);
        return Err("Assembly failed.".to_string());
    }

    let status = std::process::Command::new("gcc")
        .args(["-O3", "-march=native", "-flto", "-o", output_name, &obj_path, "-lm"])
        .spawn()
        .map_err(|e| format!("Compiler Error: Failed to spawn gcc (link): {}", e))?
        .wait()
        .map_err(|e| format!("Compiler Error: Failed to wait for gcc (link): {}", e))?;

    let _ = std::fs::remove_file(&obj_path);

    if !status.success() {
        return Err("Linking failed.".to_string());
    }

    Ok(())
}

pub fn compile_with_options(aly_path: &str, options: &CompileOptions) -> Result<(), String> {
    let source = std::fs::read_to_string(aly_path)
        .map_err(|e| format!("Compiler Error: Failed to read source file '{}': {}", aly_path, e))?;
    
    let output_name = options.output_file.as_ref().unwrap();
    
    match options.emit_format {
        EmitFormat::Assembly => {
            let asm = compile_to_assembly(&source, options)?;
            std::fs::write(output_name, asm)
                .map_err(|e| format!("Failed to write assembly: {}", e))?;
            if options.verbose {
                println!("Assembly written to: {}", output_name);
            }
            Ok(())
        }
        EmitFormat::Object => {
            compile_to_object(&source, options, output_name)
        }
        EmitFormat::Binary => {
            if options.direct || output_name.ends_with(".exe") || output_name.ends_with(".bin") || output_name.ends_with(".img") {
                compile_to_direct_machine_code(&source, options, output_name)
            } else {
                compile_to_binary(&source, options, output_name)
            }
        }
        EmitFormat::SharedLibrary => {
            let mut opts = options.clone();
            opts.shared = true;
            compile_to_binary(&source, &opts, output_name)
        }
        EmitFormat::C => {
            let c_code = compile_to_c(&source, options)?;
            std::fs::write(output_name, c_code)
                .map_err(|e| format!("Failed to write C file: {}", e))?;
            if options.verbose {
                println!("C code written to: {}", output_name);
            }
            Ok(())
        }
        EmitFormat::Cpp => {
            let cpp_code = compile_to_cpp(&source, options)?;
            std::fs::write(output_name, cpp_code)
                .map_err(|e| format!("Failed to write C++ file: {}", e))?;
            if options.verbose {
                println!("C++ code written to: {}", output_name);
            }
            Ok(())
        }
        EmitFormat::JavaScript => {
            let js_code = compile_to_js(&source, options)?;
            std::fs::write(output_name, js_code)
                .map_err(|e| format!("Failed to write JS file: {}", e))?;
            if options.verbose {
                println!("JavaScript written to: {}", output_name);
            }
            Ok(())
        }
        EmitFormat::Python => {
            let py_code = compile_to_python(&source, options)?;
            std::fs::write(output_name, py_code)
                .map_err(|e| format!("Failed to write Python file: {}", e))?;
            if options.verbose {
                println!("Python written to: {}", output_name);
            }
            Ok(())
        }
        EmitFormat::Kotlin => {
            let kt_code = compile_to_kotlin(&source)?;
            std::fs::write(output_name, kt_code)
                .map_err(|e| format!("Failed to write Kotlin file: {}", e))?;
            if options.verbose {
                println!("Kotlin written to: {}", output_name);
            }
            Ok(())
        }
        EmitFormat::BareMetal => {
            compile_bare_metal_to_binary(&source, options, output_name)
        }
        EmitFormat::Go => {
            let go_code = go_backend::compile_to_go(&source)?;
            std::fs::write(output_name, go_code)
                .map_err(|e| format!("Failed to write Go file: {}", e))?;
            if options.verbose {
                println!("Go code written to: {}", output_name);
            }
            Ok(())
        }
        EmitFormat::Rust => {
            let rust_code = rust_backend::compile_to_rust(&source)?;
            std::fs::write(output_name, rust_code)
                .map_err(|e| format!("Failed to write Rust file: {}", e))?;
            if options.verbose {
                println!("Rust code written to: {}", output_name);
            }
            Ok(())
        }
        EmitFormat::Shell => {
            let sh_code = sh_backend::compile_to_sh(&source)?;
            std::fs::write(output_name, sh_code)
                .map_err(|e| format!("Failed to write Shell file: {}", e))?;
            if options.verbose {
                println!("Shell script written to: {}", output_name);
            }
            Ok(())
        }
        EmitFormat::Jvm => {
            let jvm_code = jvm_backend::compile_to_jvm(&source)?;
            std::fs::write(output_name, jvm_code)
                .map_err(|e| format!("Failed to write Java/JVM file: {}", e))?;
            if options.verbose {
                println!("Java/JVM code written to: {}", output_name);
            }
            Ok(())
        }
    }
}

pub fn compile_to_bare_metal(source: &str, options: &CompileOptions) -> Result<String, String> {
    let program = parser::parse_program(source);
    let chunk = crate::vm::compiler::compile_program(&program)
        .map_err(|e| format!("CompileError: {}", e))?;
    
    let arch = match options.arch {
        Arch::X86_64 => asm::Arch::X86_64,
        Arch::AArch64 => asm::Arch::AArch64,
    };
    let mut backend = asm::AsmBackend::new(arch);
    backend.set_opt_level(match options.opt_level {
        OptLevel::None => asm::OptLevel::None,
        OptLevel::O1 => asm::OptLevel::Fast,
        OptLevel::O2 => asm::OptLevel::Moderate,
        OptLevel::O3 => asm::OptLevel::Aggressive,
        OptLevel::Os => asm::OptLevel::Size,
        OptLevel::Oz => asm::OptLevel::SizeZ,
    });
    backend.set_target_triple(options.target_triple.as_str());
    backend.set_emit_type(asm::EmitType::Assembly);
    backend.set_asm_syntax(match options.asm_syntax {
        AsmSyntax::Intel => asm::AsmSyntax::Intel,
        AsmSyntax::Att => asm::AsmSyntax::Att,
    });
    
    // Bare metal mode: no runtime, no GC, custom entry point
    if options.bare_metal {
        backend.set_bare_metal(true);
        backend.set_no_std(options.no_std);
        if let Some(entry) = &options.entry_point {
            backend.set_entry_point(entry);
        }
    }
    
    backend.compile_chunk(&chunk, "main")
        .map_err(|e| format!("Assembly generation failed: {}", e))?;
    
    Ok(backend.take_output())
}

pub fn compile_bare_metal_to_binary(source: &str, options: &CompileOptions, output_path: &str) -> Result<(), String> {
    let asm = compile_to_bare_metal(source, options)?;
    
    let asm_path = format!("{}.s", output_path);
    std::fs::write(&asm_path, asm)
        .map_err(|e| format!("Failed to write assembly: {}", e))?;
    
    let mut cmd = std::process::Command::new("gcc");
    
    // Bare metal compilation flags
    cmd.args([
        "-c", &asm_path, "-o", &format!("{}.o", output_path),
        "-nostdlib", "-nostartfiles", "-nodefaultlibs",
        "-ffreestanding", "-fno-builtin", "-nostdlib",
    ]);
    
    if options.debug_info {
        cmd.arg("-g");
    }
    
    if options.verbose {
        println!("Assembling bare metal: {:?}", cmd);
    }
    
    let status = cmd.status()
        .map_err(|e| format!("Failed to run assembler: {}", e))?;
    
    std::fs::remove_file(&asm_path).ok();
    
    if !status.success() {
        return Err("Bare metal assembly failed".to_string());
    }
    
    // Link with custom linker script if provided
    let mut link_cmd = std::process::Command::new("ld");
    let obj_path = format!("{}.o", output_path);
    
    if let Some(script) = &options.linker_script {
        if std::path::Path::new(script).exists() {
            link_cmd.args(["-T", script]);
        }
    } else {
        // Default minimal linker script for bare metal
        let default_script = format!("{}.ld", output_path);
        let script_content = r#"
ENTRY(_start)
SECTIONS {
    . = 0x100000;
    .text : { *(.text*) }
    .rodata : { *(.rodata*) }
    .data : { *(.data*) }
    .bss : { *(.bss*) }
}
"#;
        std::fs::write(&default_script, script_content).ok();
        link_cmd.args(["-T", &default_script]);
    }
    
    link_cmd.args(["-o", output_path, &obj_path, "--oformat=elf64-x86-64"]);
    
    if options.verbose {
        println!("Linking bare metal: {:?}", link_cmd);
    }
    
    let status = link_cmd.status()
        .map_err(|e| format!("Failed to run linker: {}", e))?;
    
    std::fs::remove_file(&format!("{}.o", output_path)).ok();
    
    if !status.success() {
        return Err("Bare metal linking failed".to_string());
    }
    
    Ok(())
}

pub fn compile_with_pgo(aly_path: &str, output_name: &str) -> Result<(), String> {
    let source = std::fs::read_to_string(aly_path)
        .map_err(|e| format!("Compiler Error: Failed to read source file '{}': {}", aly_path, e))?;

    let c_code = compile_source(&source)?;

    let tmp_c = format!("{}_aly_tmp.c", output_name);
    
    std::fs::write(&tmp_c, &c_code)
        .map_err(|e| format!("Compiler Error: Failed to write temp C file '{}': {}", tmp_c, e))?;

    let header = include_str!("runtime_aly.h");
    let tmp_dir = std::path::Path::new(&tmp_c)
        .parent()
        .unwrap_or(std::path::Path::new("."));
    let header_path = format!("{}/runtime_aly.h", tmp_dir.display());
    std::fs::write(&header_path, header)
        .map_err(|e| format!("Compiler Error: Failed to write runtime_aly.h '{}': {}", header_path, e))?;

    let status = std::process::Command::new("gcc")
        .args(["-O3", "-march=native", "-fprofile-generate", "-o", output_name, &tmp_c, "-lm"])
        .spawn()
        .map_err(|e| format!("Compiler Error (PGO Gen): Failed to spawn gcc: {}", e))?
        .wait()
        .map_err(|e| format!("Compiler Error (PGO Gen): Failed to wait for gcc: {}", e))?;

    if !status.success() {
        let _ = std::fs::remove_file(&tmp_c);
        let _ = std::fs::remove_file(&header_path);
        return Err("Instrumented compilation failed.".to_string());
    }

    let run_path = if output_name.starts_with('/') {
        output_name.to_string()
    } else {
        format!("./{}", output_name)
    };
    
    let _ = std::process::Command::new(&run_path)
        .spawn()
        .map_err(|e| format!("Compiler Error (PGO Run): Failed to run instrumented binary: {}", e))?
        .wait();

    let status = std::process::Command::new("gcc")
        .args(["-O3", "-march=native", "-flto", "-fprofile-use", "-o", output_name, &tmp_c, "-lm"])
        .spawn()
        .map_err(|e| format!("Compiler Error (PGO Use): Failed to spawn gcc: {}", e))?
        .wait()
        .map_err(|e| format!("Compiler Error (PGO Use): Failed to wait for gcc: {}", e))?;

    let _ = std::fs::remove_file(&tmp_c);
    let _ = std::fs::remove_file(&header_path);
    
    let gcda_file = format!("{}_aly_tmp.gcda", output_name);
    let gcno_file = format!("{}_aly_tmp.gcno", output_name);
    let _ = std::fs::remove_file(&gcda_file);
    let _ = std::fs::remove_file(&gcno_file);

    if !status.success() {
        return Err("Profile-guided compilation failed.".to_string());
    }

    Ok(())
}

pub fn compile_to_direct_machine_code(source: &str, options: &CompileOptions, output_path: &str) -> Result<(), String> {
    if output_path.ends_with(".exe") || matches!(options.target_triple, TargetTriple::X86_64Windows | TargetTriple::AArch64Windows) {
        direct_elf::compile_direct_to_pe(source, output_path)
    } else if output_path.ends_with(".bin") || output_path.ends_with(".img") {
        direct_elf::compile_direct_to_bin(source, output_path)
    } else {
        direct_elf::compile_direct_to_elf(source, output_path)
    }
}