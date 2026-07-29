use eval::eval;

fn main() {
    let expr = r#""You typed: " + "Type here..kkl.""#;
    println!("Evaluating: {}", expr);
    match eval(expr) {
        Ok(val) => println!("Success: {}", val),
        Err(err) => println!("Error: {}", err),
    }
}
