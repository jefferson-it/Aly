mod timer_mod {
    use crate::native::types::{Validator, ValueData};
    use crate::native::std::{split_args, arg as std_arg};

    // timer.sleep(ms) -> blocks for ms milliseconds
    pub fn timer_sleep(x: String) -> Box<dyn Validator> {
        let ms: u64 = std_arg(&split_args(&x, 1), 0).trim().parse().unwrap_or(0);
        std::thread::sleep(std::time::Duration::from_millis(ms));
        Box::new("None".to_owned())
    }

    // timer.set_timeout(ms) -> blocks for ms milliseconds (alias for sleep)
    pub fn timer_set_timeout(x: String) -> Box<dyn Validator> {
        timer_sleep(x)
    }

    // timer.set_interval(fn_name, ms) -> calls the function by name every ms
    // Uses the runtime to look up and execute the function.
    // Runs until the runtime errors or forever.
    pub fn timer_set_interval(x: String) -> Box<dyn Validator> {
        use crate::tokens::Tokens;
        let args = split_args(&x, 2);
        let fn_name = std_arg(&args, 0);
        let ms: u64 = std_arg(&args, 1).trim().parse().unwrap_or(1000);

        let duration = std::time::Duration::from_millis(ms);
        loop {
            std::thread::sleep(duration);
            let run = crate::aly::get_runtime();
            let fake = crate::lexer::Lexer::new(Tokens::Reference, fn_name.clone(), 0);
            if let Ok(var) = run.get_var(fake) {
                if let ValueData::Function(_, params, body) = var.get_value() {
                    let old = run.get_vars().clone();
                    for p in &params {
                        run.register_var(crate::native::vars::Var::new(p.literal.clone(), ValueData::String("None".to_owned()), true));
                    }
                    let stmts = crate::runtime::interpreter::split_statements(&body);
                    let mut v: Box<dyn Validator> = Box::new(ValueData::String("None".to_owned()));
                    for s in stmts {
                        let mut s2 = s;
                        if crate::runtime::interpreter::exec(&mut s2, &mut v) {
                            break;
                        }
                    }
                } else if let ValueData::NativeFunction(f) = var.get_value() {
                    let _ = f(String::new());
                }
            }
        }
    }
}

pub use timer_mod::*;
