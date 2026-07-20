/// Aly-lang — Entry point.
///
/// Delegates to the APG (Aly Package Manager) CLI which handles
/// running scripts, REPL, compilation, and package management.
use Aly::apg::main::main as apg_main;

fn main() {
    apg_main();
}


