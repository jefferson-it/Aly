mod console_mod {
    use crossterm::style::{self, Color, Stylize};
    use crate::native::types::{Validator, ValueData};
    use crate::validators::str::remove_quoted_str;
    use crate::native::std::{split_args, arg as std_arg};

    // console.log(text) -> prints text with default color
    pub fn console_log(x: String) -> Box<dyn Validator> {
        let text = std_arg(&split_args(&x, 1), 0);
        println!("{}", text);
        Box::new("None".to_owned())
    }

    // console.warn(text) -> prints yellow text
    pub fn console_warn(x: String) -> Box<dyn Validator> {
        let text = std_arg(&split_args(&x, 1), 0);
        println!("{}", text.yellow());
        Box::new("None".to_owned())
    }

    // console.error(text) -> prints red text
    pub fn console_error(x: String) -> Box<dyn Validator> {
        let text = std_arg(&split_args(&x, 1), 0);
        eprintln!("{}", text.red());
        Box::new("None".to_owned())
    }

    // console.info(text) -> prints blue text
    pub fn console_info(x: String) -> Box<dyn Validator> {
        let text = std_arg(&split_args(&x, 1), 0);
        println!("{}", text.blue());
        Box::new("None".to_owned())
    }

    // console.success(text) -> prints green text
    pub fn console_success(x: String) -> Box<dyn Validator> {
        let text = std_arg(&split_args(&x, 1), 0);
        println!("{}", text.green());
        Box::new("None".to_owned())
    }

    // console.progress(current, total, label) -> prints a progress bar line
    // console.progress(current, total) or console.progress(current, total, "Progress")
    pub fn console_progress(x: String) -> Box<dyn Validator> {
        let args = split_args(&x, 3);
        let current: usize = std_arg(&args, 0).trim().parse().unwrap_or(0);
        let total: usize = std_arg(&args, 1).trim().parse().unwrap_or(100);
        let label = if args.len() > 2 && !args[2].trim().is_empty() {
            remove_quoted_str(args[2].trim().to_string())
        } else {
            "Progress".to_string()
        };

        if total == 0 {
            return Box::new("None".to_owned());
        }

        let pct = (current as f64 / total as f64 * 100.0) as usize;
        let bar_width = 30;
        let filled = (pct * bar_width / 100).min(bar_width);
        let empty = bar_width - filled;

        let bar = format!(
            "\r{}: [{}{}] {}%",
            label,
            "=".repeat(filled),
            " ".repeat(empty),
            pct
        );

        if current >= total {
            println!("{}", bar.green());
        } else {
            print!("{}", bar.cyan());
        }

        use std::io::{Write, stdout};
        let _ = stdout().flush();

        Box::new("None".to_owned())
    }
}

pub use console_mod::*;
