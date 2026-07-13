mod apg;
mod error;
mod math_eval;
mod aly;
mod compiler;
mod runtime;
mod lexer;
mod native;
mod tokens;
mod validators;
mod schema;

use crate::apg::main as apg_main;

fn main() {
    apg_main();
}
