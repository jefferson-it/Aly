mod conditions {
    use crate::{aly::Aly, lexer::Lexer, native::{exec_rust, process_value, types::Validator}};

    pub fn exec_cond(run: &mut Aly, expressions: Vec<Lexer>) -> Box<dyn Validator> {
        let mut expr = vec![];

        for exp in expressions {
            if exp.token.id() == "reference" {
                let item = process_value(run, [exp].to_vec());

                expr.push(item.to_string(false));
            } else {
                expr.push(exp.literal);
            }
        }

        let res = match exec_rust(expr.join(" ")) {
            Ok(boolean) => boolean,
            Err(_) => String::from("false"),
        };

        return Box::new(res.clone());
    }
}

pub use conditions::*;