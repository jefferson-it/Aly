use Aly::{
    aly::get_runtime,
    compiler::{compile_and_write_to_file, compile_with_pgo, CompileOptions},
};

#[cfg(feature = "android")]
use Aly::tools::{android_template::generate_android_template, android_build::build_android_project};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: aly [run|debug|comp|compile|build|compile-pgo|fmt|lint|doc|test|disassemble|android] <file.aly>");
        return;
    }

    let cmd = args[1].as_str();

    match cmd {
        #[cfg(feature = "android")]
        "android" => {
            if args.len() < 3 {
                println!("Usage: aly android [init|build|run|template|devices] [options]");
                return;
            }
            let subcmd = args[2].as_str();
            match subcmd {
                "init" => {
                    let package = args.get(3).map(|s| s.as_str()).unwrap_or("com.example.alyapp");
                    let name = args.get(4).map(|s| s.as_str()).unwrap_or("AlyApp");
                    let dir = args.get(5).map(|s| s.as_str()).unwrap_or(".");
                    println!("🤖 Initializing Android project: {} ({})", name, package);
                    if let Err(e) = generate_android_template(dir, package, name, name) {
                        eprintln!("Error: {}", e);
                    } else {
                        println!("✅ Android project created in: {}", dir);
                    }
                }
                "template" => {
                    let package = args.get(3).map(|s| s.as_str()).unwrap_or("com.example.alyapp");
                    let name = args.get(4).map(|s| s.as_str()).unwrap_or("AlyApp");
                    let dir = args.get(5).map(|s| s.as_str()).unwrap_or(".");
                    println!("📱 Generating Android template: {} ({})", name, package);
                    if let Err(e) = generate_android_template(dir, package, name, name) {
                        eprintln!("Error: {}", e);
                    } else {
                        println!("✅ Template generated in: {}", dir);
                    }
                }
                "build" => {
                    let project_dir = args.get(3).map(|s| s.as_str()).unwrap_or(".");
                    let release = args.contains(&"--release".to_string());
                    println!("🔨 Building Android project: {}", project_dir);
                    if let Err(e) = build_android_project(project_dir, "com.example.alyapp", release, None, ".") {
                        eprintln!("Error: {}", e);
                    } else {
                        println!("✅ Build completed!");
                    }
                }
                "run" => {
                    let project_dir = args.get(3).map(|s| s.as_str()).unwrap_or(".");
                    println!("▶️ Running Android app: {}", project_dir);
                    println!("Run functionality not yet implemented. Use 'adb install' and 'adb shell am start' manually.");
                }
                "devices" => {
                    println!("📱 Connected devices:");
                    println!("Use 'adb devices' to see connected devices.");
                }
                _ => {
                    println!("Unknown Android command: {}", subcmd);
                    println!("Available commands: init, template, build, run, devices");
                }
            }
        }
        #[cfg(not(feature = "android"))]
        "android" => {
            eprintln!("Error: Android functionality requires the 'android' feature (cargo build --features android)");
            return;
        }
        "fmt" => {
            if args.len() < 3 {
                eprintln!("Error: Script file name (.aly) required");
                return;
            }
            let file = &args[2];
            match std::fs::read_to_string(file) {
                Ok(source) => {
                    let formatted = Aly::tools::fmt::format_code(&source);
                    if let Err(e) = std::fs::write(file, formatted) {
                        eprintln!("Error writing formatted file: {}", e);
                    } else {
                        println!("File formatted successfully: {}", file);
                    }
                }
                Err(e) => eprintln!("Error reading file '{}': {}", file, e),
            }
        }
        "lint" => {
            if args.len() < 3 {
                eprintln!("Error: Script file name (.aly) required");
                return;
            }
            let file = &args[2];
            match std::fs::read_to_string(file) {
                Ok(source) => {
                    let diagnostics = Aly::tools::linter::lint_code(&source);
                    if diagnostics.is_empty() {
                        println!("No issues found in {}", file);
                    } else {
                        for d in diagnostics {
                            eprintln!("{}:{}: {}", file, d.line, d.message);
                        }
                    }
                }
                Err(e) => eprintln!("Error reading file '{}': {}", file, e),
            }
        }
        "doc" => {
            if args.len() < 3 {
                eprintln!("Error: Script file name (.aly) required");
                return;
            }
            let file = &args[2];
            match std::fs::read_to_string(file) {
                Ok(source) => {
                    let docs = Aly::tools::doc::generate_docs(&source);
                    println!("{}", docs);
                }
                Err(e) => eprintln!("Error reading file '{}': {}", file, e),
            }
        }
        "test" => {
            if args.len() < 3 {
                eprintln!("Error: Script file name (.aly) required");
                return;
            }
            let file = &args[2];
            match std::fs::read_to_string(file) {
                Ok(source) => {
                    let results = Aly::tools::test_runner::run_tests(&source);
                    let mut failed = false;
                    for r in results {
                        if r.passed {
                            println!("[PASS] test: {}", r.name);
                        } else {
                            println!("[FAIL] test: {} - {:?}", r.name, r.error);
                            failed = true;
                        }
                    }
                    if failed {
                        std::process::exit(1);
                    } else {
                        println!("All tests passed!");
                    }
                }
                Err(e) => eprintln!("Error reading file '{}': {}", file, e),
            }
        }
        "debug" => {
            let run = get_runtime();
            Aly::tools::debugger::inspect_runtime(run);
        }
        "disassemble" | "disasm" => {
            if args.len() < 3 {
                eprintln!("Error: Script file name (.aly) required");
                return;
            }
            let file = &args[2];
            match std::fs::read_to_string(file) {
                Ok(source) => {
                    match Aly::vm::disassemble(&source) {
                        Ok(d) => println!("{}", d),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                Err(e) => eprintln!("Error reading file '{}': {}", file, e),
            }
        }
        "comp" | "compile" | "build" => {
            let comp_args = &args[2..];
            if comp_args.is_empty() {
                print_comp_help();
                return;
            }
            let opts = match CompileOptions::from_args(comp_args) {
                Ok(opts) => opts,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    print_comp_help();
                    return;
                }
            };
            
            let source = match std::fs::read_to_string(&opts.input_file) {
                Ok(s) => s,
                Err(e) => {
                    eprintln!("Error reading file '{}': {}", opts.input_file, e);
                    return;
                }
            };
            
            if opts.verbose {
                println!("Compiling {} -> {:?}", opts.input_file, opts.output_file);
                println!("  Format: {:?}", opts.emit_format);
                println!("  Opt: {}", opts.opt_level.gcc_flag());
                println!("  Target: {}", opts.target_triple.as_str());
                println!("  Arch: {:?}", opts.arch);
                println!("  Debug: {}", opts.debug_info);
                println!("  Shared: {}", opts.shared);
            }
            
            match Aly::compiler::compile_with_options(&source, &opts) {
                Ok(_) => {
                    if let Some(out) = &opts.output_file {
                        println!("Compilation successful: {}", out);
                    } else {
                        println!("Compilation successful");
                    }
                }
                Err(e) => eprintln!("{}", e),
            }
        }
        "compile-pgo" | "build-pgo" | "pgo" => {
            if args.len() < 3 {
                eprintln!("Error: Script file name (.aly) required");
                return;
            }
            let file = &args[2];
            let out_name = file.strip_suffix(".aly").unwrap_or(file);
            match compile_with_pgo(file, out_name) {
                Ok(_) => println!("PGO compilation successful: {}", out_name),
                Err(e) => eprintln!("{}", e),
            }
        }
        "run" | "run-vm" | "vm" => {
            if args.len() < 3 {
                eprintln!("Error: Script file name (.aly) required");
                return;
            }
            let file = &args[2];
            match std::fs::read_to_string(file) {
                Ok(source) => {
                    if let Err(e) = Aly::vm::execute(&source) {
                        eprintln!("VM Error: {}", e);
                    }
                }
                Err(e) => eprintln!("Error reading file '{}': {}", file, e),
            }
        }
        _ => {
            let file = &args[1];
            match std::fs::read_to_string(file) {
                Ok(source) => {
                    if let Err(e) = Aly::vm::execute(&source) {
                        eprintln!("VM Error: {}", e);
                    }
                }
                Err(e) => eprintln!("Error reading file '{}': {}", file, e),
            }
        }
    }
}

fn print_comp_help() {
    println!("Usage: aly comp [options] <file.aly>");
    println!();
    println!("Options:");
    println!("  -o <file>          Specify output file");
    println!("  -S, --assembly     Emit assembly only (don't assemble/link)");
    println!("  -emit <format>     Override output format (asm, c, cpp, js, py, kt, obj, bin, so)");
    println!("  -O0, -O1, -O2, -O3, -Os, -Oz  Optimization level");
    println!("  --target <triple>  Target triple (x86_64-linux, aarch64-linux, x86_64-windows, etc.)");
    println!("  --arch <arch>      Target architecture (x86_64, aarch64)");
    println!("  --masm <intel|att> Assembly syntax style");
    println!("  -g, --debug        Emit debug information");
    println!("  --shared, -shared  Build shared library");
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
    println!("  (default) -> Binary");
}