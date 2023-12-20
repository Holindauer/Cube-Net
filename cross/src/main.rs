extern crate cube_move_library;
use cube_move_library::{cube, moves};//{cube, moves, rotation};
mod parse;
mod verify_cross; 
mod cubie_converter;
mod solve;


// Cross is a program that reads in a scramble string from the CLI and finds a set of moves that solves for the cross on the yellow face.


fn main() {

    // Retrieve the scramble from the CLI
    let scramble = match parse::parse_args() {
        Ok(first_arg_words) => {
            println!("Scramble: {:?}", first_arg_words);
            first_arg_words
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    // Recreate the cube state (Tensor Representation) using the cube_move_library crate
    let mut tensor_cube = cube::create_solved_cube();
    for turn in scramble {
        moves::make_move(&mut tensor_cube, &turn);
    }

    // parallel arrays of tuples that hold the (integer encoded) color combos and positions for each of the edges
    let color_combos: [(i32, i32); 4] = [(1, 4), (3, 4), (5, 4), (6, 4)];  // order is: red/yellow, green/yellow, blue/yellow, orange/yellow
    let correct_placements: [(usize, usize, usize); 4] = [(0, 2, 1), (1, 2, 0), (1, 2, 2), (2, 2, 1)];

    // Start a string that will hold the solution
    let mut full_solution = "".to_string();

    // Iterate through each color combination and find a solution for each edge
    for (index, color_combo) in color_combos.iter().enumerate() {

        if !verify_cross::verify_cross(&mut tensor_cube) { // <--- If the cross is not solved, continue finding a solution

            // Convert the Tensor Representaion into the Cubie Struct Array Represention
            let cubies = cubie_converter::edge_converter(&tensor_cube);

            // Find the edge with the current color combination (tuple)
            let actual_position = solve::find_edge(&cubies, color_combo.0, color_combo.1);  

            // Print the coordinates of the edge
            println!("Edge Coordinates: ({}, {}, {})", actual_position.0, actual_position.1, actual_position.2);

            // Get a reference to the edge
            let cubie = &cubies[actual_position.0][actual_position.1][actual_position.2];

            if !solve::correct_edge_placement(actual_position, correct_placements[index]) || !solve::correct_edge_permutation(cubie) {

                // Solve the edge
                let edge_solution = solve::solve_edge(&cubies, &tensor_cube, actual_position, correct_placements[index], color_combo);
                
                // append the edge solution to the solution string
                full_solution.push_str(&edge_solution);

                // Apply the edge solution to the cube.
                for turn in edge_solution.split(" ") {
                    moves::make_move(&mut tensor_cube, &turn);
                }
            }
        }
    }

    // Print the solution
    println!("Solution: {}", full_solution);
}

