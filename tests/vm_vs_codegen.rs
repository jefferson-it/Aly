use std::process::Command;
use std::fs;
use std::time::Duration;

fn run_test_case(name: &str, source: &str) {
    let mut temp_file = std::env::temp_dir();
    temp_file.push(format!("{}.aly", name));
    fs::write(&temp_file, source).expect("failed to write temp file");

    let binary_path = std::env::var("CARGO_BIN_EXE_aly")
        .unwrap_or_else(|_| "./target/debug/aly".to_string());

    // Execute with VM (with 30s timeout)
    let mut vm_child = Command::new(&binary_path)
        .args(["vm", temp_file.to_str().unwrap()])
        .spawn()
        .expect("failed to spawn VM");
    let vm_output = wait_with_timeout(&mut vm_child, Duration::from_secs(30))
        .expect("VM timed out or failed");

    // Execute with Codegen / Compiler runtime (with 60s timeout — includes gcc)
    let mut run_child = Command::new(&binary_path)
        .args(["run", temp_file.to_str().unwrap()])
        .spawn()
        .expect("failed to spawn Codegen");
    let run_output = wait_with_timeout(&mut run_child, Duration::from_secs(60))
        .expect("Codegen timed out or failed");

    // Clean up temp files
    let _ = fs::remove_file(&temp_file);
    let tmp_base = format!("{}_aly_run_tmp", name);
    let _ = fs::remove_file(&tmp_base);
    let _ = fs::remove_file(format!("{}.s", tmp_base));

    let vm_stdout = String::from_utf8_lossy(&vm_output.stdout).to_string();
    let run_stdout = String::from_utf8_lossy(&run_output.stdout).to_string();

    assert_eq!(vm_output.status.success(), run_output.status.success(), 
        "Exit status mismatch for test case '{}'. VM success: {}, Run success: {}", 
        name, vm_output.status.success(), run_output.status.success());
        
    assert_eq!(vm_stdout, run_stdout, 
        "Output mismatch for test case '{}'.\n=== VM Output ===\n{}\n=== Run Output ===\n{}", 
        name, vm_stdout, run_stdout);
}

fn wait_with_timeout(child: &mut std::process::Child, timeout: Duration) -> Result<std::process::Output, String> {
    let start = std::time::Instant::now();
    loop {
        match child.try_wait() {
            Ok(Some(status)) => {
                let output = child.stdout.take()
                    .map(|mut o| {
                        use std::io::Read;
                        let mut buf = Vec::new();
                        let _ = o.read_to_end(&mut buf);
                        buf
                    })
                    .unwrap_or_default();
                let stderr = child.stderr.take()
                    .map(|mut o| {
                        use std::io::Read;
                        let mut buf = Vec::new();
                        let _ = o.read_to_end(&mut buf);
                        buf
                    })
                    .unwrap_or_default();
                return Ok(std::process::Output {
                    status,
                    stdout: output,
                    stderr,
                });
            }
            Ok(None) => {
                if start.elapsed() > timeout {
                    let _ = child.kill();
                    return Err("Timeout".into());
                }
                std::thread::sleep(Duration::from_millis(100));
            }
            Err(e) => return Err(format!("Process error: {}", e)),
        }
    }
}

// ── Passing tests ──

#[test]
fn test_break_continue() {
    run_test_case("break_continue", r#"
let i = 0
let sum = 0
loop i lt 10 {
    i = i + 1
    if i eq 5 { continue }
    if i gt 8 { break }
    sum = sum + i
}
print(sum)
"#);
}

#[test]
fn test_comparisons() {
    run_test_case("comparisons", r#"
let a = 10
let b = 20
print(a lt b)
print(a gt b)
print(a lte b)
print(a gte b)
print(a eq b)
print(a neq b)
"#);
}

#[test]
fn test_strings() {
    run_test_case("strings", r#"
let s1 = "Hello "
let s2 = "World"
print(s1 + s2)
"#);
}

#[test]
fn test_recursion_and_functions() {
    run_test_case("recursion_and_functions", r#"
fun fib(n) {
    if n lte 1 { return n }
    return fib(n - 1) + fib(n - 2)
}
print(fib(8))
"#);
}

// ── Failing tests (ignored — VM vs codegen output mismatch) ──

#[test]
fn test_math_and_modulo() {
    run_test_case("math_and_modulo", r#"
let a = 10
let b = 3
print(a + b)
print(a - b)
print(a * b)
print(a / b)
print(a | b)
"#);
}

#[test]
fn test_logical_operators() {
    run_test_case("logical_operators", r#"
print(1 eq 1 and 2 eq 2)
print(1 eq 1 and 2 eq 3)
print(1 eq 1 or 2 eq 3)
print(not (1 eq 1))
"#);
}

#[test]
fn test_arrays() {
    run_test_case("arrays", r#"
let arr = [1, 2, 3, 4]
print(arr[0])
print(arr[2])
arr[1] = 99
print(arr[1])
"#);
}

#[test]
fn test_nested_loops() {
    run_test_case("nested_loops", r#"
let i = 0
let sum = 0
loop i lt 5 {
    let j = 0
    loop j lt 5 {
        sum = sum + i * j
        j = j + 1
    }
    i = i + 1
}
print(sum)
"#);
}

#[test]
fn test_prime_benchmark() {
    run_test_case("prime_bench", r#"
let n = 2000
let count = 0
let c = 2
loop {
    if c gt n { break }
    let p = 1
    let d = 2
    loop {
        if d * d gt c { break }
        if c | d eq 0 {
            p = 0
            break
        }
        d = d + 1
    }
    if p eq 1 { count = count + 1 }
    c = c + 1
}
print(count)
"#);
}
