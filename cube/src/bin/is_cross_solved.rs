// Importing local modules from library crate
use cube::moves::make_move;
use cube::cube::create_solved_cube; 
use cube::parse::parse_scramble_args;
use cube::verify_cross::verify_cross;

// Importing external crates
use serde_json; 

fn main() {

    // Retrieve the scramble from the CLI
    let scramble = match parse_scramble_args() {
        Ok(first_arg_words) => {
            first_arg_words
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    // Recreate the cube state (Tensor Representation) using the cube_move_library crate
    let mut tensor_cube = create_solved_cube();
    for turn in scramble {
        make_move(&mut tensor_cube, &turn);
    }

    let mut solved_state = 0;

    // Check if the cube is solved. save to solve_status.json
    if verify_cross(&mut tensor_cube) {    
        solved_state = 1;
    }

    // save serialized to file
    let serialized = serde_json::to_string(&solved_state).unwrap();
    std::fs::write("../json/cross_solve_status.json", serialized).expect("Unable to write file");
}

