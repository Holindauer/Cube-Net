
extern crate cube_move_library;
use cube_move_library::{cube, moves, rotation};
mod verify;
mod parse;

fn main() {

    // Parse the command line arguments ----> 2 string vectors containing the scramble and solution
    let (scramble, solution) = match parse::parse_args() {
        Ok((first_arg_words, second_arg_words)) => {
            println!("Scramble: {:?}", first_arg_words);
            println!("Proposed Solution: {:?}", second_arg_words);
            (first_arg_words, second_arg_words)
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            // Handle the error appropriately, maybe exit the program
            return;
        }
    };

    // Create a solved 3x3 rubiks cube array --- 5x5x5 in memory
    let mut rubiks_cube = cube::create_solved_cube();

    // iterate through the scramble vector, passing each string to the make_move function
    for move_str in &scramble {
        moves::make_move(&mut rubiks_cube, move_str);
    }

    // iterate through the solution vector, passing each string to the make_move function
    for move_str in &solution {
        moves::make_move(&mut rubiks_cube, move_str);
    }

    // Check if the cube is solved
    if verify::is_solved(&rubiks_cube) {
        println!("Cube is solved!");
    } else {
        println!("Cube is not solved!");
    }
}

