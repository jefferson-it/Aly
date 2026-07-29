// src/bin/alyc.rs
use Aly::compiler::{compile_with_options, CompileOptions, EmitFormat, TargetTriple, Arch, AsmSyntax, OptLevel};
use std::path::Path;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        print_help();
        return;
    }

    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print_help();
        return;
    }

    let comp_args = &args[1..];

    let mut opts = match CompileOptions::from_args(comp_args) {
        Ok(o) => o,
        Err(e) => {
            eprintln!("Error: {}", e);
            std::process::exit(1);
        }
    };
    let input_path = opts.input_file.clone();

    // Determine output file and auto-detect emit format if needed
    if opts.output_file.is_none() {
        let path = Path::new(&input_path);
        let stem = path.file_stem().and_then(|s| s.to_str()).unwrap_or("output");
        let ext = opts.emit_format.default_extension();
        let output = if ext.is_empty() {
            stem.to_string()
        } else {
            format!("{}.{}", stem, ext)
        };
        opts.output_file = Some(output);
    } else {
        // If format is still default (Binary) but output has a specific extension, auto-detect it
        if opts.emit_format == EmitFormat::Binary {
            if let Some(out_file) = &opts.output_file {
                if let Some(ext) = Path::new(out_file).extension().and_then(|e| e.to_str()) {
                    let detected = EmitFormat::from_extension(ext);
                    if detected != EmitFormat::Binary {
                        opts.emit_format = detected;
                    }
                }
            }
        }
    }

    if opts.verbose {
        println!("alyc compiler:");
        println!("  Input File:  {}", opts.input_file);
        println!("  Output File: {}", opts.output_file.as_ref().unwrap());
        println!("  Format:      {:?}", opts.emit_format);
        println!("  Direct MC:   {}", opts.direct);
        println!("  Asm Syntax:  {:?}", opts.asm_syntax);
        println!("  Arch:        {:?}", opts.arch);
        println!("  Target:      {}", opts.target_triple.as_str());
    }

    match compile_with_options(&opts.input_file, &opts) {
        Ok(_) => {
            println!("✅ Compilation successful: {}", opts.output_file.as_ref().unwrap());
        }
        Err(e) => {
            eprintln!("❌ Compilation failed: {}", e);
            std::process::exit(1);
        }
    }
}

fn print_help() {
    println!("alyc — Aly Compiler Interface");
    println!("Usage: alyc [options] <file.aly>");
    println!();
    println!("Options:");
    println!("  -o <file>             Specify the output file path");
    println!("  -emit <format>        Specify the target compilation format:");
    println!("                          asm   : x86-64 / AArch64 assembly text");
    println!("                          bin   : Direct machine code executable (ELF/PE/flat bin)");
    println!("                          c     : C source code");
    println!("                          cpp   : C++ source code");
    println!("                          js    : JavaScript source code");
    println!("                          py    : Python source code");
    println!("                          kt    : Kotlin source code");
    println!("                          go    : Go source code");
    println!("                          rs    : Rust source code");
    println!("                          sh    : Bash/Shell script");
    println!("                          jvm   : Java/JVM source code");
    println!("  --direct, -direct     Compile directly to machine code bytes bypassing C/ASM");
    println!("  --masm <intel|att>    Set assembly syntax formatting style (Intel or AT&T)");
    println!("  --target <triple>     Set target platform triple (e.g. x86_64-linux, x86_64-windows)");
    println!("  --arch <arch>         Set target instruction set architecture (x86_64, aarch64)");
    println!("  -O0, -O1, -O2, -O3    Set optimization level");
    println!("  -g, --debug           Include debugging symbols/metadata");
    println!("  -v, --verbose         Enable verbose log output");
    println!("  -h, --help            Show this help menu");
    println!();
    println!("Auto-detection of format from -o extension:");
    println!("  .s / .asm  -> Assembly");
    println!("  .go        -> Go code");
    println!("  .rs        -> Rust code");
    println!("  .sh        -> Shell script");
    println!("  .java      -> Java/JVM code");
    println!("  .js        -> JavaScript");
    println!("  .py        -> Python");
    println!("  .kt        -> Kotlin");
    println!("  .exe       -> Windows PE executable (compiled directly!)");
    println!("  .bin       -> Flat binary machine code (compiled directly!)");
}
