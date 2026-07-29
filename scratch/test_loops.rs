use Aly::compiler::{ast::*, parser::parse_program, codegen::CodeGenerator};

fn main() {
    let cases = vec![
        ("for_in", "loop x in arr { print(x) }"),
        ("for_of", "loop x of arr { print(x) }"),
        ("for_range", "loop range 0..5 { print(0) }"),
    ];
    for (name, src) in cases {
        match parse_program(src) {
            Ok(_) => println!("PASS {name}"),
            Err(e) => println!("FAIL {name}: {e}"),
        }
    }
}
