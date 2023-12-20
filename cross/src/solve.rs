use crate::cubie_converter::Cubie;
use crate::cubie_converter;
use crate::moves;

/*

choose_move.rs contains the logic for choosing what move should be made next in the cross solver algorith. 

This process will involve a process of identifying one of the 4 edges of the cross and determinuing if they are in the correct position. If they are not, then a series of moves 
will be calculated to bring the edges into the correct position.

The order in which the yellow edges will be placed into the cube is, yellow/red, yellow/blue, yellow/orange, yellow/green.

*/


//-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------Algorithm for finding the next move



// This function will use the find_edge function to find the index of the edge that needs to be placed.  find the index of the edge that needs to be placed. Then it will move 
// that edge to the top layer in a way that brings the yellow color to the top face. Then it will rotate the top face until the edge is above the correct position on the yellow face. 
// Then it will turn 180 degrees to bring the edge into the correct position.

// I will neeed to implement a check that ensures that if the edge is already in the correct position, then the function will not do anything.
// Additionally, for the case where the  edge is on the side of the cube, once the yellow face of the edge is looking upwards, the function will turn the U face and apply the 
// opposite move to the function so that if there is an edge already placed in that position, it does not disrupt it. 
// There will also need to be a check for when the edge is on the top layer but the yellow face is not facing upwards. In this case, the function will need to bring the edge 
// to one of the side faces with the yellow face not facing upwards. Then bringing the side


pub fn solve_edge(cubies : &[[[Cubie; 3]; 3]; 3], tensor_cube: &[[[i32; 5]; 5]; 5], actual_position : (usize, usize, usize), correct_position : (usize, usize, usize), color_combo : &(i32, i32)) -> String{

        // If it is the case that the edge is not is the correct location or not permuted correctly, then solve the edge
    
        if actual_position.1 == 0 {
            return top_layer_edge(cubies, tensor_cube, actual_position, correct_position, color_combo);
        }
        if actual_position.1  == 1 {
            return middle_layer_edge(cubies, tensor_cube, actual_position, correct_position, color_combo);
        }
        if actual_position.1  == 2 {
            return bottom_layer_edge(cubies, tensor_cube, actual_position, correct_position, color_combo)
        }
 
    // If the edge is already in the correct position, then return an empty string
    "".to_string()
}

//-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------Algorithms for solving an edge on a specific layer


/* If an edge of the cross is on the top layer of the cube, there are two scenarios that determine how it will be solved.

- If the yellow side is facing upwards. Then solving it just involves moving it above the correct position on the top face and 
  turning the other side the edge borders 180 degrees
- If the edge's yellow face is not facing upwards, then the edge will need to be moved to one of the side faces such that that face can be rotated,
    bringing the edge to the top layer with the yellow face facing upwards. Then the edge can be solved as described above.
  */
fn top_layer_edge(cubies : &[[[Cubie; 3]; 3]; 3], tensor_cube: &[[[i32; 5]; 5]; 5], actual_position : (usize, usize, usize), correct_position : (usize, usize, usize), color_combo : &(i32, i32)) -> String {

    println!("Top Layer Edge");

    let mut solution = "".to_string();

    // determine what direction the non yellow color is facing
    let yellow_face = yellow_direction(&cubies[actual_position.0][actual_position.1][actual_position.2]);
    println!("Yellow Face: {}", yellow_face);

    if yellow_face == 'U' { 
        // rotate the top face until the edge is above the correct position
        let turn_top = rotate_until_above_correct_position(actual_position, correct_position);
        solution.push_str(&turn_top);

        // Then rotate that side face 180 degrees
        let correct_face = bottom_top_edge_determine_face(correct_position);
        solution.push_str(&format!("{} {} ", correct_face, correct_face));
    }
    else if yellow_face != 'U' {
        // if the edge is not facing upwards, bring the edge to above the correct position on the top face
        let turn_top = rotate_until_above_correct_position(actual_position, correct_position);
        solution.push_str(&turn_top);

        // create a copy of the tensor cube to update as the solution is created
        let mut tensor_cube_copy = tensor_cube.clone();
        for turn in solution.split(" ") {
            moves::make_move(&mut tensor_cube_copy, &turn);
        }

        // convert this updated tensor cube into an updated cubie array
        let updated_cubies = cubie_converter::edge_converter(&tensor_cube_copy);
        let new_position = find_edge(&updated_cubies, color_combo.0, color_combo.1);


        // then rotate the top face so the yellow face "looks to its left" and rotate the face the yellow side is on clockwise
        let yellow_face = yellow_direction(&updated_cubies[new_position.0][new_position.1][new_position.2]);
        let left_face = face_left_of(yellow_face);
        solution.push_str(&format!("U {} {}' {}' ", left_face, yellow_face, left_face));
    }

    solution
}

// This function determines a sequence of moves to solve an edge in the cross that is on the middle layer of the cube
fn middle_layer_edge(cubies : &[[[Cubie; 3]; 3]; 3], tensor_cube: &[[[i32; 5]; 5]; 5], actual_position : (usize, usize, usize), correct_position : (usize, usize, usize), color_combo : &(i32, i32)) -> String {

    let mut solution = "".to_string();

    // determine what direction the non yellow color is facing
    let yellow_face = yellow_direction(&cubies[actual_position.0][actual_position.1][actual_position.2]);
    let non_yellow_face = non_yellow_direction(&cubies[actual_position.0][actual_position.1][actual_position.2]);

    let bring_to_top = bring_middle_to_top(yellow_face, non_yellow_face);
    solution.push_str(&format!("{} U U {}' ", bring_to_top, bring_to_top)); // <-- once on top, U U moves that edge out of the way so the bring_to_top face can be rotated back down


    // apply the above moves to a copy of the tensor cube
    let mut tensor_cube_copy = tensor_cube.clone();
    for turn in solution.split(" ") {
        moves::make_move(&mut tensor_cube_copy, &turn);
    }

    // convert this updated tensor cube into an updated cubie array
    let updated_cubies = cubie_converter::edge_converter(&tensor_cube_copy);
    let new_position = find_edge(&updated_cubies, color_combo.0, color_combo.1);
    
    // then rotate the top face until the edge is above the correct position
    let turn_top = rotate_until_above_correct_position(new_position, correct_position);
    solution.push_str(&turn_top);

    // Then rotate that side face 180 degrees
    let correct_face = bottom_top_edge_determine_face(correct_position);
    solution.push_str(&format!("{} {} ", correct_face, correct_face));

    solution
}


/* This function creates a series of moves to solve an edge of the cross that is on the bottom layer of the cube
 If a cubie is on the bottom layer of of the cube and not placed corrected, then that means there are 3 possible cases for this edge:

- It is in the wrong position but correctly permuted
- It is in the wrong position and incorrectly permuted
- It is in the correct position but incorrectly permuted
*/
fn bottom_layer_edge(cubies : &[[[Cubie; 3]; 3]; 3], tensor_cube: &[[[i32; 5]; 5]; 5], actual_position : (usize, usize, usize), correct_position : (usize, usize, usize), color_combo : &(i32, i32)) -> String{

    // determine placement status of the edge
    let edge_permutation = correct_edge_permutation(&cubies[actual_position.0][actual_position.1][actual_position.2]);
    let edge_placement = correct_edge_placement(actual_position, correct_position);

    // start a string that will hold the solution
    let mut solution = "".to_string();

    if !edge_permutation{

        /* We are creating a clone of tensor_cube here to update as we go becuase, unlike when the permuation is correct and incorrectly placed, the case where the 
        cube is incorrectly permuted requires updating the values as the solution is created in order to get accurate information on what the next move should be*/
        let mut tensor_cube_copy = tensor_cube.clone();

        // 1.) If the edge is in the wrong/right position and incorrectly permuted, then rotate the face it is on once clockwise and the face to the left of it counterclockwise
        let face = bottom_top_edge_determine_face(actual_position);
        let left_face = face_left_of(face);
        solution.push_str(&format!("{} {}' ", face, left_face)); // NOTE: the ' indicates a counterclockwise turn

        // apply the above moves to the tensor cube clone
        for turn in solution.split(" ") {
            moves::make_move(&mut tensor_cube_copy, &turn);
        }

        // convert this updated tensor cube into an updated cubie array
        let updated_cubies = cubie_converter::edge_converter(&tensor_cube_copy);

        // find the new position of the edge
        let new_position = find_edge(&updated_cubies, color_combo.0, color_combo.1);

        // 2.) then rotate the top face until the edge is above the correct position
        let turn_top = rotate_until_above_correct_position(new_position, correct_position);
        solution.push_str(&turn_top);

        if turn_top == "" { // In the case where the edge is above the correct position once immediately after the first two moves, then rotate the top face once clockwise
        // In order to move it out of the way so that the left_face that was brought up can be rotated back down. The next if statement will then rotate the top face back
            solution.push_str("U "); 
        }
        
        // 4.) reverse the second move
        solution.push_str(&format!("{} ", left_face)); // NOTE: the ' indicates a counterclockwise turn

        if turn_top == "" { // This is the second part of the case described above
            solution.push_str("U' "); 
        }

        // Then rotate that side face 180 degrees
        let correct_face = bottom_top_edge_determine_face(correct_position);
        solution.push_str(&format!("{} {} ", correct_face, correct_face));
    }
    else if !edge_placement && edge_permutation{
        // if the edge is in the wrong position but correctly permuted, then rotate the face 180 degrees
        let face = bottom_top_edge_determine_face(actual_position);
        solution.push_str(&format!("{} {} ", face, face));

        // then rotate the top face until the edge is above the correct position
        let turn_top = rotate_until_above_correct_position(actual_position, correct_position);
        solution.push_str(&turn_top);

        // Then rotate that side face 180 degrees
        let correct_face = bottom_top_edge_determine_face(correct_position);
        solution.push_str(&format!("{} {} ", correct_face, correct_face));
    }   
    solution
}



//-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------Helper Functions

// This function will determine the non-yellow color's direction of an edge 
fn non_yellow_direction(cubie : &Cubie) -> char {
    if cubie.up != 0 && cubie.up != 4 {
        return 'U';
    }
    else if cubie.down != 0 && cubie.down != 4 {
        return 'D';
    }
    else if cubie.left != 0 && cubie.left != 4 {
        return 'L';
    }
    else if cubie.right != 0 && cubie.right != 4 {
        return 'R';
    }
    else if cubie.front != 0 && cubie.front != 4 {
        return 'F';
    }
    else if cubie.back != 0 && cubie.back != 4 {
        return 'B';
    }
    else {
        panic!("Non-yellow color not found");
    }
}

// This function will determine the direction of the yellow color on an edge
fn yellow_direction(cubie : &Cubie) -> char {
    if cubie.up == 4 {
        return 'U';
    }
    else if cubie.down == 4 {
        return 'D';
    }
    else if cubie.left == 4 {
        return 'L';
    }
    else if cubie.right == 4 {
        return 'R';
    }
    else if cubie.front == 4 {
        return 'F';
    }
    else if cubie.back == 4 {
        return 'B';
    }
    else {
        panic!("Yellow color not found");
    }
}

// This function is used to determine which face to turn in order to bring an edge of the yellow cross to the top layer with the yellow color facing upwards
fn bring_middle_to_top(yellow : char, other : char) -> String {
    if yellow == 'F' && other == 'R' {
        return "R".to_string();
    }
    else if yellow == 'R' && other == 'F' {
        return "F'".to_string();
    }
    else if yellow == 'L' && other == 'F' {
        return "F".to_string();
    }
    else if yellow == 'F' && other == 'L' {
        return "L'".to_string();
    }
    else if yellow == 'B' && other == 'L' {
        return "L".to_string();
    }
    else if yellow == 'L' && other == 'B' {
        return "B'".to_string();
    }
    else if yellow == 'R' && other == 'B' {
        return "B".to_string();
    }
    else if yellow == 'B' && other == 'R' {
        return "R'".to_string();
    }
    else {
        panic!("Invalid edge");
    }
}


// This function finds the face to the left of a given face on the cube. This function is intended for the middle strip of the cube when U and D are the top and bottom faces.
fn face_left_of(face : char) -> char {

    if face == 'F' {
        return 'L';
    }
    else if face == 'L' {
        return 'B';
    }
    else if face == 'B' {
        return 'R';
    }
    else if face == 'R' {
        return 'F';
    }
    else {
        panic!("Invalid face");
    }

}

// This function is used in the situation that a cubie is on the top face and the yellow side is correctly permuted upwward but the cubie is not
// directly above the correct position (such that rotating the connected side face of that edge w/ the top would put it in place).
// This function assumes that the edge is on the top layer in the position described.
fn rotate_until_above_correct_position(actual : (usize, usize, usize), correct : (usize, usize, usize)) -> String{

    let correct_face = bottom_top_edge_determine_face(correct);
    let actual_face = bottom_top_edge_determine_face(actual);

    let mut solution = "".to_string();

    if actual_face == correct_face {
        // if the edge is already above the correct position, then return the empty string
        return solution;
    }
    else {
        // if the edge is not above the correct position, then rotate the top face until it is above the correct position
        let distance = cyclic_distance(actual_face, correct_face);
        for _ in 0..distance {
            solution.push_str("U ");
        }
        return solution;
    }
}

// this helper function computes the cyclic distance between sides of a face of the cube. It is used in 
// the rotate_until_above_correct_position function to determin how many times to rotate the top face
fn cyclic_distance(start: char, end: char) -> usize {
    let mut map = std::collections::HashMap::new();
    map.insert('F', 0); // top
    map.insert('L', 1); // right
    map.insert('B', 2); // bottom
    map.insert('R', 3); // left

    let start_pos = map[&start];
    let end_pos = map[&end];

    (end_pos + 4 - start_pos) % 4
}


// This function is used to determine what face of te cube an edge that is on the bottom layer is on
fn bottom_top_edge_determine_face(position : (usize, usize, usize)) -> char{

    if position.0 == 0 {
        return 'F'; // front 
    }
    else if position.0 == 1 && position.2 == 0 { 
        return 'L'; // left 
    }
    else if position.0 == 1 && position.2 == 2 {
        return 'R'; // right
    }
    else if position.0 == 2 {
        return 'B'; // back
    }
    else {
        panic!("Edge is neither on the bottom nor the top layer");
    }
}


// assuming the an edge of the cross is in the location, then all that is need to check if the edge is permuted correctly is to check if the yellow color is facing downwards
pub fn correct_edge_permutation(cubie : &Cubie) -> bool{
    if cubie.down == 4 {
        return true;
    }
    false
}

// This function will check if the edge is in the correct place
pub fn correct_edge_placement(actual : (usize, usize, usize), correct : (usize, usize, usize)) -> bool {
    if actual == correct {
        return true;
    }
    false
}


//-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------Algorithm for finding some edge
// This function will find a specific edge is within the cubie array.
pub fn find_edge(cubies: &[[[Cubie; 3]; 3]; 3], color_1 :i32, color_2 :i32) -> (usize, usize, usize) {

    for i in 0..3 { // Itereate through the cubie array
        for j in 0..3 {
            for k in 0..3 {

                // check if the edge is in the cubie
                if search_cubie_edge(&cubies[i][j][k], color_1, color_2) {
                    return (i, j, k);
                }
            }
        }
    }

    // if the edge is not found return an error
    panic!("Edge not found");
}


// This function will recieve a single Cubie (which in this case represents an edge) from the Cubies array and will determine if the specified colors are inside of it
// Because the cubie structs can store up to six integers in their members, the function will also keep track of how many colors are in the cubie in order to ensure
// That the cubie is not a corner piece.
fn search_cubie_edge(cubie :&Cubie, color_1:i32, color_2:i32) -> bool {

    // ensure the cubie is not an edge piece
    if count_colors(&cubie) != 2 {
        return false;
    }
    // Determine if both of the specified colors are in the cube
    if search_color(cubie, color_1) && search_color(cubie, color_2) {
        return true;
    } 
    false // return false if the colors are not found
}

// This function is used to search for a specific color in a cubie. 
fn search_color(cubie: &Cubie, color: i32) -> bool {
    cubie.up == color || cubie.down == color || cubie.left == color || cubie.right == color || cubie.front == color || cubie.back == color
}

// This function is used to count the number of colors in a cubie in order to determine if it is a corner or an edge piece
fn count_colors(cubie: &Cubie) -> usize {
    let fields = [
        cubie.up,
        cubie.down,
        cubie.left,
        cubie.right,
        cubie.front,
        cubie.back,
    ];

    fields.iter().filter(|&&field| field != 0).count()
}
