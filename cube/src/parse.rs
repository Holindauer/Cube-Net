// parse.rs is used to parse two command line arguments (scrambled and solution) for the program into a list of strings that will be used to 
// first scramble the rubiks cube and then verify if the solution is correct.

use std::env;
use std::error::Error;

// This function parses a scramble and a solution from the CLI and returns a tuple of two vectors of strings.
pub fn parse_scramble_solution_args() -> Result<(Vec<String>, Vec<String>), Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect(); // Skip the program name

    // Check if there are exactly two arguments
    if args.len() != 2 {
        return Err("Exactly two arguments are required.".into());
    }

    // Split each argument into words
    let first_arg_words: Vec<String> = args[0].split_whitespace().map(String::from).collect();
    let second_arg_words: Vec<String> = args[1].split_whitespace().map(String::from).collect();

    Ok((first_arg_words, second_arg_words))
}

// This function parses a scramble from the CLI and returns a vector of strings.
pub fn parse_scramble_args() -> Result<Vec<String>, Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect(); // Skip the program name

    if args.len() != 1 {
        return Err("Exactly one argument is required.".into());
    }

    let first_arg_words: Vec<String> = args[0].split_whitespace().map(String::from).collect();

    Ok(first_arg_words)
}

