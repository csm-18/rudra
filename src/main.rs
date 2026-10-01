mod cli;
mod diagnostics;
mod utils;
fn main() {
    // Command-line arguments
    let mut args: Vec<String> = std::env::args().skip(1).collect();

    cli::run(&mut args);
}
