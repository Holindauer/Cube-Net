use std::env;
use std::error::Error;

pub fn parse_args() -> Result<Vec<String>, Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect(); // Skip the program name

    if args.len() != 1 {
        return Err("Exactly one argument is required.".into());
    }

    let first_arg_words: Vec<String> = args[0].split_whitespace().map(String::from).collect();

    Ok(first_arg_words)
}
