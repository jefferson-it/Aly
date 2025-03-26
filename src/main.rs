use std::env;

use aly::get_runtime;
use validators::path::validation_file;

pub mod aly;
pub mod lexer;
pub mod native;
pub mod tokens;
pub mod validators;

pub mod runtime {
    pub mod interpreter;
    pub mod parser;
}

#[derive(Clone)]
pub enum Act {
    Run,
    Cli,
    Comp,
}

fn main() {
    let argv: Vec<String> = env::args().collect();
    let argc = argv.len();
    let mut action = Act::Cli;
    let mut file_run = String::from("main.ba");

    if argc > 1 {
        if argv[1] != "run" && argv[1] != "comp" && argv[1] != "cli" {
            println!("Error: Invalid action {}!", argv[1]);

            return;
        }

        action = if &argv[1] == "run" {
            Act::Run
        } else if &argv[1] == "comp" {
            Act::Comp
        } else {
            Act::Cli
        };

        if argc > 2 {
            file_run = validation_file(argv[2].as_str());
        }
    }

    let runtime = get_runtime();

    runtime.def_action(action);

    runtime.run(file_run);
}
