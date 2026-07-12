use std::fs;
use std::path::Path;

fn main() {
    // Ensure thatsrc/embedded_script.txt exists so that the compiler does not fail
    // when using include_str!("embedded_script.txt")
    let script_path = Path::new("src/embedded_script.txt");
    if !script_path.exists() {
        fs::write(script_path, "").unwrap();
    }
    
    // Tell cargo to rerun if build.rs changes
    println!("cargo:rerun-if-changed=build.rs");
}
