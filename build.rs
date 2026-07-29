use std::fs;
use std::path::Path;

fn main() {
    // Ensure that src/embedded_script.txt exists so that the compiler does not fail
    // when using include_str!("embedded_script.txt")
    let script_path = Path::new("src/embedded_script.txt");
    if !script_path.exists() {
        fs::write(script_path, "").unwrap();
    }

    // Detect JOT source path for embed feature
    if let Ok(jot_path) = std::env::var("JOT_PATH") {
        if Path::new(&jot_path).exists() {
            println!("cargo:rustc-env=JOT_SOURCE_PATH={}", jot_path);
            println!("cargo:rustc-cfg=jot_available");
        }
    } else {
        let default_path = Path::new("../JOT");
        if default_path.exists() {
            println!("cargo:rustc-env=JOT_SOURCE_PATH={}", default_path.display());
            println!("cargo:rustc-cfg=jot_available");
        }
    }

    // Tell cargo to rerun if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-env-changed=JOT_PATH");
}
