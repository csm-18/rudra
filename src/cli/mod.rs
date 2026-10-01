mod arg_parser;
use arg_parser::parse;
use std::process::exit;

/// Parse and execute command-line arguments
pub fn run(args: &mut Vec<String>) {
    if !args.is_empty()
        && ["-build", "--build", "-b", "--b"].contains(&args[0].to_lowercase().as_str())
    {
        // Build mode
        println!("Build mode is not implemented yet!");
        exit(1);
    }

    parse(args);
}
