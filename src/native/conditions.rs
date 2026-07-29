mod conditions {
    use crate::{
        lexer::Lexer,
        native::{process_value, types::{Validator, ValueData}},
        validators::str::remove_quoted_str,
    };

    pub fn exec_cond(expressions: Vec<Lexer>) -> Box<dyn Validator> {
        let mut tokens = vec![];

        // Check for leading `not` — negate the rest of the expression
        if expressions.first().map(|t| t.literal.as_str()) == Some("not") {
            let inner = exec_cond(expressions[1..].to_vec());
            let (_, val) = inner.valid();
            let is_true = match val {
                ValueData::Bool(b) => b,
                ValueData::Int(i) => i != 0,
                _ => val.to_string(false) == "true",
            };
            return Box::new((!is_true).to_string());
        }

        for exp in expressions {
            if exp.literal == "if"
                || exp.literal == "elif"
                || exp.literal == "loop"
                || exp.literal == "{"
                || exp.literal == "}"
            {
                continue;
            }
            tokens.push(exp);
        }

        if tokens.is_empty() {
            return Box::new("false".to_string());
        }

        // Handle simple single/binary string or numeric comparison: A op B
        let mut op_idx = None;
        let mut op_type = "";

        for (i, t) in tokens.iter().enumerate() {
            let lit = t.literal.trim();
            if lit == "eq" || lit == "EQ" || lit == "==" {
                op_idx = Some(i);
                op_type = "eq";
                break;
            } else if lit == "neq" || lit == "NEQ" || lit == "!=" {
                op_idx = Some(i);
                op_type = "neq";
                break;
            } else if lit == "lt" || lit == "LT" || lit == "<" {
                op_idx = Some(i);
                op_type = "lt";
                break;
            } else if lit == "gt" || lit == "GT" || lit == ">" {
                op_idx = Some(i);
                op_type = "gt";
                break;
            } else if lit == "lte" || lit == "LTE" || lit == "<=" {
                op_idx = Some(i);
                op_type = "lte";
                break;
            } else if lit == "gte" || lit == "GTE" || lit == ">=" {
                op_idx = Some(i);
                op_type = "gte";
                break;
            }
        }

        if let Some(idx) = op_idx {
            let left_tokens = tokens[..idx].to_vec();
            let right_tokens = tokens[idx + 1..].to_vec();

            let left_raw = remove_quoted_str(process_value(left_tokens).to_string(false));
            let right_raw = remove_quoted_str(process_value(right_tokens).to_string(false));

            // Strip underscores from numeric strings for comparison
            let left_val = left_raw.replace('_', "");
            let right_val = right_raw.replace('_', "");

            let res = match op_type {
                "eq" => left_val == right_val,
                "neq" => left_val != right_val,
                "lt" => {
                    let l: f64 = left_val.parse().unwrap_or(0.0);
                    let r: f64 = right_val.parse().unwrap_or(0.0);
                    l < r
                }
                "gt" => {
                    let l: f64 = left_val.parse().unwrap_or(0.0);
                    let r: f64 = right_val.parse().unwrap_or(0.0);
                    l > r
                }
                "lte" => {
                    let l: f64 = left_val.parse().unwrap_or(0.0);
                    let r: f64 = right_val.parse().unwrap_or(0.0);
                    l <= r
                }
                "gte" => {
                    let l: f64 = left_val.parse().unwrap_or(0.0);
                    let r: f64 = right_val.parse().unwrap_or(0.0);
                    l >= r
                }
                _ => false,
            };

            return Box::new(if res { "true".to_string() } else { "false".to_string() });
        }

        // Fallback: try to evaluate through the interpreter
        let res = process_value(tokens.clone());
        let _res_str = res.to_string(false);
        let is_truthy = match res {
            ValueData::Bool(b) => b,
            ValueData::Int(i) => i != 0,
            ValueData::String(ref s) => !s.is_empty() && s != "0" && s != "false",
            ValueData::Vec(ref v) => v.len() > 0,
            _ => true,
        };

        Box::new(if is_truthy { "true".to_string() } else { "false".to_string() })
    }
}

pub use conditions::*;