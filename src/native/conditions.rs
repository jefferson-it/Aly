mod conditions {
    use crate::{aly::Aly, lexer::Lexer, native::{exec_rust, process_value, types::Validator}};

    pub fn exec_cond(run: &mut Aly, expressions: Vec<Lexer>) -> Box<dyn Validator> {
        let find: Vec<Vec<&str>> = vec![
            vec!["lte", "<="],
            vec!["gte", ">="],
            vec!["neq", "!="],
            vec!["eq", "=="],
            vec!["lt", "<"],
            vec!["gt", ">"],
            vec!["and", "&&"],
            vec!["or", "||"],
            vec!["xor", "^"],
        ];

        let mut expr = vec![];

        for exp in expressions {
            if exp.token.id() == "reference" {
                let item = process_value(run, [exp].to_vec());
                expr.push(item.to_string(false));
            } else {
                expr.push(exp.literal);
            }
        }

        let mut expr_final = expr.join(""); 

        for item in &find {
            if expr_final.contains(item[0]) {
                expr_final = expr_final.replace(item[0], item[1]);
            } else if expr_final.contains(&item[0].to_uppercase()) {
                expr_final = expr_final.replace(&item[0].to_uppercase(), item[1]);
            }
        }
        
        let res = match exec_rust(expr_final) {
            Ok(boolean) => boolean,
            Err(_) => String::from("false"),
        };

        return Box::new(res.clone());
    }
}

pub use conditions::*;