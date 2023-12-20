
extern crate cube_move_library;
use cube_move_library::{cube, moves};
mod verify;
mod parse;

fn main() {

    // Parse the command line arguments ----> 2 string vectors containing the scramble and solution
    let (scramble, solution) = match parse::parse_args() {
        Ok((first_arg_words, second_arg_words)) => {
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


    let mut solved_state = 0;

    // Check if the cube is solved. save to solve_status.json
    if verify::is_solved(&rubiks_cube) {    
        solved_state = 1;
    } 
    
    // save serialized to file
    let serialized = serde_json::to_string(&solved_state).unwrap();
    std::fs::write("../solve_status.json", serialized).expect("Unable to write file");
}

