mod math {
    use std::time::{SystemTime, UNIX_EPOCH};

    use crate::{native::types::{Validator, ValueData}, validators::{conversor_to_float, str::put_quoted_str}};

    pub const PI: f64 = std::f64::consts::PI;
    pub const E: f64 = std::f64::consts::E;

    // Parse a native-function argument string into a list of f64 values.
    // Arguments come separated by whitespace and/or commas.
    fn parse_args(x: String) -> Vec<f64> {
        x.split(|c: char| c == ',' || c.is_whitespace())
            .map(|part| part.trim())
            .filter(|part| !part.is_empty())
            .map(|part| conversor_to_float(part.to_owned()) as f64)
            .collect()
    }

    // Return an Int when the float has no fractional part, otherwise a Float.
    fn to_value(result: f64) -> Box<dyn Validator> {
        if result.fract() == 0.0 && result.is_finite() {
            Box::new(ValueData::Int(result as i64))
        } else {
            Box::new(ValueData::Float(result))
        }
    }

    // A simple pseudo-random generator (xorshift) seeded by the system clock.
    fn random_f64() -> f64 {
        let nanos = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.subsec_nanos() as u64 + d.as_secs())
            .unwrap_or(0x2545F4914F6CDD1D);

        let mut state = nanos ^ 0x9E3779B97F4A7C15;
        state ^= state << 13;
        state ^= state >> 7;
        state ^= state << 17;

        // Map to the [0, 1) range.
        (state % 1_000_000) as f64 / 1_000_000.0
    }

    // pow(base, exponent)
    pub fn fun_pow(x: String) -> Box<dyn Validator> {
        let args = parse_args(x);
        if args.len() < 2 {
            eprintln!("TypeError [pow]: esperado pow(base, expoente).");
            return Box::new(ValueData::Float(0.0));
        }
        to_value(args[0].powf(args[1]))
    }

    // sqrt(value)
    pub fn fun_sqrt(x: String) -> Box<dyn Validator> {
        let args = parse_args(x);
        if args.is_empty() {
            eprintln!("TypeError [sqrt]: esperado sqrt(valor).");
            return Box::new(ValueData::Float(0.0));
        }
        to_value(args[0].sqrt())
    }

    // random() -> float in [0, 1)
    pub fn fun_random(_x: String) -> Box<dyn Validator> {
        Box::new(ValueData::Float(random_f64()))
    }

    // round(value) -> nearest integer
    pub fn fun_round(x: String) -> Box<dyn Validator> {
        let args = parse_args(x);
        if args.is_empty() {
            eprintln!("TypeError [round]: esperado round(valor).");
            return Box::new(ValueData::Int(0));
        }
        to_value(args[0].round())
    }

    // roundUp(value) -> ceil
    pub fn fun_round_up(x: String) -> Box<dyn Validator> {
        let args = parse_args(x);
        if args.is_empty() {
            eprintln!("TypeError [roundUp]: esperado roundUp(valor).");
            return Box::new(ValueData::Int(0));
        }
        to_value(args[0].ceil())
    }

    // roundDown(value) -> floor
    pub fn fun_round_down(x: String) -> Box<dyn Validator> {
        let args = parse_args(x);
        if args.is_empty() {
            eprintln!("TypeError [roundDown]: esperado roundDown(valor).");
            return Box::new(ValueData::Int(0));
        }
        to_value(args[0].floor())
    }

    // to_fixed(value, precision) -> string with fixed decimal places
    pub fn fun_to_fixed(x: String) -> Box<dyn Validator> {
        let args = parse_args(x);
        if args.len() < 2 {
            eprintln!("TypeError [to_fixed]: esperado to_fixed(valor, precisão).");
            return Box::new(put_quoted_str(String::new()));
        }
        let precision = args[1].max(0.0) as usize;
        let formatted = format!("{:.*}", precision, args[0]);
        Box::new(put_quoted_str(formatted))
    }

    // abs(value) -> absolute value
    pub fn fun_abs(x: String) -> Box<dyn Validator> {
        let args = parse_args(x);
        if args.is_empty() {
            eprintln!("TypeError [abs]: esperado abs(valor).");
            return Box::new(ValueData::Float(0.0));
        }
        to_value(args[0].abs())
    }

    // min(a, b) -> smaller of two values
    pub fn fun_min(x: String) -> Box<dyn Validator> {
        let args = parse_args(x);
        if args.len() < 2 {
            eprintln!("TypeError [min]: esperado min(a, b).");
            return Box::new(ValueData::Float(0.0));
        }
        to_value(args[0].min(args[1]))
    }

    // max(a, b) -> larger of two values
    pub fn fun_max(x: String) -> Box<dyn Validator> {
        let args = parse_args(x);
        if args.len() < 2 {
            eprintln!("TypeError [max]: esperado max(a, b).");
            return Box::new(ValueData::Float(0.0));
        }
        to_value(args[0].max(args[1]))
    }

    // sin(value) -> sine of angle in radians
    pub fn fun_sin(x: String) -> Box<dyn Validator> {
        let args = parse_args(x);
        if args.is_empty() {
            eprintln!("TypeError [sin]: esperado sin(valor).");
            return Box::new(ValueData::Float(0.0));
        }
        to_value(args[0].sin())
    }

    // cos(value) -> cosine of angle in radians
    pub fn fun_cos(x: String) -> Box<dyn Validator> {
        let args = parse_args(x);
        if args.is_empty() {
            eprintln!("TypeError [cos]: esperado cos(valor).");
            return Box::new(ValueData::Float(0.0));
        }
        to_value(args[0].cos())
    }

    // tan(value) -> tangent of angle in radians
    pub fn fun_tan(x: String) -> Box<dyn Validator> {
        let args = parse_args(x);
        if args.is_empty() {
            eprintln!("TypeError [tan]: esperado tan(valor).");
            return Box::new(ValueData::Float(0.0));
        }
        to_value(args[0].tan())
    }

    // log(x) -> base-10 logarithm
    pub fn fun_log(x: String) -> Box<dyn Validator> {
        let args = parse_args(x);
        if args.is_empty() {
            eprintln!("TypeError [log]: esperado log(valor).");
            return Box::new(ValueData::Float(0.0));
        }
        to_value(args[0].log10())
    }

    // ln(x) -> natural logarithm (base e)
    pub fn fun_ln(x: String) -> Box<dyn Validator> {
        let args = parse_args(x);
        if args.is_empty() {
            eprintln!("TypeError [ln]: esperado ln(valor).");
            return Box::new(ValueData::Float(0.0));
        }
        to_value(args[0].ln())
    }
}

pub use math::*;
