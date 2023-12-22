// moves.rs contains the functions for performing the U, U', B, B', D, D', L, and L' moves on the rubiks cube array.
// The six below functions set the indicies of the rubiks ccube array abstraction (cube.rs) and then call the functions from
// rotation.rs in order to either perform a clockwise or counter clockwise roation on the cube (depending on the direction 
// specified in the char argument).

use crate::rotation::{corners_clockwise, 
    corners_counter_clockwise, 
    edges_clockwise, 
    edges_counter_clockwise, 
    connected_edges_clockwise, 
    connected_edges_counter_clockwise,
    FaceCorners,
    FaceEdges,
    ConnectedEdges,
};

//-------------------------------------------------------------------------------------------------------------------------------- Move Decision Switch Statement

// make_move() contains a switch statement that recieves the array of strings arguments and then calls the appropriate move function
pub fn make_move(cube: &mut [[[i32; 5]; 5]; 5], move_string: &str) {
    match move_string {
        "U" => u(cube, 'c'),
        "U'" => u(cube, 'p'),
        "B" => b(cube, 'c'),
        "B'" => b(cube, 'p'),
        "D" => d(cube, 'c'),
        "D'" => d(cube, 'p'),
        "L" => l(cube, 'c'),
        "L'" => l(cube, 'p'),
        "F" => f(cube, 'c'),
        "F'" => f(cube, 'p'),
        "R" => r(cube, 'c'),
        "R'" => r(cube, 'p'),
        _ => (),
    }
}

//-------------------------------------------------------------------------------------------------------------------------------- Face Rotations Abstraction

// This function directions recieves the corners, edge, and sides structs after they are set by the U, B, D, and L functions.
// It also recieves the direction specifier and a mutable reference to the rubiks cube array. The function then calls the rotaions
// functions required to perform the specified move on the rubiks cube array.
fn rotate_in_direction(cube: &mut [[[i32; 5]; 5]; 5], corners: FaceCorners, edges : FaceEdges, sides : ConnectedEdges, direction: char){

    if direction == 'c'{
        corners_clockwise(cube, corners);
        edges_clockwise(cube, edges);
        connected_edges_clockwise(cube, sides);
    }
    else if direction == 'p'{
        corners_counter_clockwise(cube, corners);
        edges_counter_clockwise(cube, edges);
        connected_edges_counter_clockwise(cube, sides);
    }
    else{
        println!("Invalid Direction --- direction char must either be 'c' or 'p'");
    }
}

// This function performs the U move on the rubiks cube 3D array abstraction. The function accepts a mutable reference to the
// 3D array. Depending on if direction is set to 'c' or 'p' (clockwise or prime) the function will perform the U or U' move.
pub fn u(cube: &mut [[[i32; 5]; 5]; 5], direction: char){

    // Set the indicies of the corners of the U face
    let corners = FaceCorners{
        ul: (3, 0, 1),
        ur: (3, 0, 3),
        dl: (1, 0, 1),
        dr: (1, 0, 3),

    };

    // Set the indicies of the edges of the U face
    let edges = FaceEdges{
        u: (3, 0, 2),
        l: (2, 0, 1),
        d: (1, 0, 2),
        r: (2, 0, 3),
    };

    // Set the indicies of the connected edges of the U face
    let sides = ConnectedEdges{
        ul: (4, 1, 1),
        um: (4, 1, 2),
        ur: (4, 1, 3),

        lu: (3, 1, 0),
        lm: (2, 1, 0),
        ld: (1, 1, 0),

        dl: (0, 1, 1),
        dm: (0, 1, 2), 
        dr: (0, 1, 3),

        ru: (3, 1, 4), 
        rm: (2, 1, 4), 
        rd: (1, 1, 4), 
    };
    
    // Call the rotate_in_direction function to perform the U or U' move on the rubiks cube array
    rotate_in_direction(cube, corners, edges, sides, direction);
}


// This function performs the B move on the rubiks cube 3D array abstraction. The function accepts a mutable reference to the
// 3D array. Depending on if direction is set to 'c' or 'p' (clockwise or prime) the function will perform the B or B' move.
pub fn b(cube :&mut [[[i32; 5]; 5]; 5], direction: char){

    // Set the indicies of the corners of the B face
    let corners = FaceCorners{
        ul: (4, 1, 3),
        ur: (4, 1, 1),
        dl: (4, 3, 3),
        dr: (4, 3, 1), 
    };

    // Set the indicies of the edges of the B face
    let edges = FaceEdges{
        u: (4, 1, 2),
        l: (4, 2, 3),
        d: (4, 3, 2),
        r: (4, 2, 1),
    };

    // Set the indicies of the connected edges of the B face
    let sides = ConnectedEdges{
        ul: (3, 0, 3),
        um: (3, 0, 2),
        ur: (3, 0, 1),

        lu: (3, 1, 4), 
        lm: (3, 2, 4),
        ld: (3, 3, 4),

        dl: (3, 4, 3),
        dm: (3, 4, 2), 
        dr: (3, 4, 1),

        ru: (3, 1, 0), 
        rm: (3, 2, 0),
        rd: (3, 3, 0),
    };

    // Call the rotate_in_direction function to perform the B or B' move on the rubiks cube array
    rotate_in_direction(cube, corners, edges, sides, direction);
}


// This function performs the D move on the rubiks cube 3D array abstraction. The function accepts a mutable reference to the
// 3D array. Depending on if direction is set to 'c' or 'p' (clockwise or prime) the function will perform the D or D' move.
pub fn d(cube :&mut [[[i32; 5]; 5]; 5], direction: char){

    // Set the indicies of the corners of the D face
    let corners = FaceCorners{
        ul: (1, 4, 1),
        ur: (1, 4, 3),
        dl: (3, 4, 1),
        dr: (3, 4, 3),
    };

    // Set the indicies of the edges of the D face
    let edges = FaceEdges{
        u: (1, 4, 2),
        l: (2, 4, 1),
        d: (3, 4, 2),
        r: (2, 4, 3), 
    };

    // Set the indicies of the connected edges of the D face
    let sides = ConnectedEdges{
        ul: (0, 3, 1),
        um: (0, 3, 2),
        ur: (0, 3, 3), 

        lu: (1, 3, 0),
        lm: (2, 3, 0),
        ld: (3, 3, 0), 

        dl: (4, 3, 1),
        dm: (4, 3, 2), 
        dr: (4, 3, 3), 

        ru: (1, 3, 4),
        rm: (2, 3, 4),
        rd: (3, 3, 4),
    };

    // Call the rotate_in_direction function to perform the D or D' move on the rubiks cube array
    rotate_in_direction(cube, corners, edges, sides, direction);
}


// This function performs the L move on the rubiks cube 3D array abstraction. The function accepts a mutable reference to the
// 3D array. Depending on if direction is set to 'c' or 'p' (clockwise or prime) the function will perform the L or L' move.
pub fn l(cube :&mut [[[i32; 5]; 5]; 5], direction: char){

    // Set the indicies of the corners of the L face
    let corners = FaceCorners{
        ul: (3, 1, 0),
        ur: (1, 1, 0), 
        dl: (3, 3, 0),
        dr: (1, 3, 0), 
    };

    // Set the indicies of the edges of the L face
    let edges = FaceEdges{
        u: (2, 1, 0),
        l: (3, 2, 0), 
        d: (2, 3, 0),   
        r: (1, 2, 0),
    };

    // Set the indicies of the connected edges of the L face
    let sides = ConnectedEdges{
        ul: (3, 0, 1),
        um: (2, 0, 1),
        ur: (1, 0, 1), 

        lu: (4, 1, 1),
        lm: (4, 2, 1), 
        ld: (4, 3, 1), 

        dl: (3, 4, 1), 
        dm: (2, 4, 1), 
        dr: (1, 4, 1),

        ru: (0, 1, 1), 
        rm: (0, 2, 1),
        rd: (0, 3, 1), 
    };

    // Call the rotate_in_direction function to perform the L or L' move on the rubiks cube array
    rotate_in_direction(cube, corners, edges, sides, direction);
}

// This function performs the L move on the rubiks cube 3D array abstraction. The function accepts a mutable reference to the
// 3D array. Depending on if direction is set to 'c' or 'p' (clockwise or prime) the function will perform the R or R' move.
pub fn r(cube :&mut [[[i32; 5]; 5]; 5], direction: char){

    // Set the indicies of the corners of the R face
    let corners = FaceCorners{
        ul: (1, 1, 4),
        ur: (3, 1, 4),
        dl: (1, 3, 4), 
        dr: (3, 3, 4),
    };

    // Set the indicies of the edges of the R face
    let edges = FaceEdges{
        u: (2, 1, 4), 
        l: (1, 2, 4),
        d: (2, 3, 4),   
        r: (3, 2, 4), 
    };

    // Set the indicies of the connected edges of the R face
    let sides = ConnectedEdges{
        ul: (1, 0, 3),
        um: (2, 0, 3),
        ur: (3, 0, 3),

        lu: (0, 1, 3),
        lm: (0, 2, 3),  
        ld: (0, 3, 3),

        dl: (1, 4, 3), 
        dm: (2, 4, 3),
        dr: (3, 4, 3),

        ru: (4, 1, 3), 
        rm: (4, 2, 3),
        rd: (4, 3, 3),
    };

    // Call the rotate_in_direction function to perform the R or R' move on the rubiks cube array
    rotate_in_direction(cube, corners, edges, sides, direction);
}

// This function performs the F move on the rubiks cube 3D array abstraction. The function accepts a mutable reference to the
// 3D array. Depending on if direction is set to 'c' or 'p' (clockwise or prime) the function will perform the F or F' move.
pub fn f(cube :&mut [[[i32; 5]; 5]; 5], direction: char){

    // Set the indicies of the corners of the F face
    let corners = FaceCorners{
        ul: (0, 1, 1),
        ur: (0, 1, 3),
        dl: (0, 3, 1), 
        dr: (0, 3, 3), 
    };

    // Set the indicies of the edges of the F face
    let edges = FaceEdges{
        u: (0, 1, 2), 
        l: (0, 2, 1),
        d: (0, 3, 2),
        r: (0, 2, 3),  
    };

    // Set the indicies of the connected edges of the F face
    let sides = ConnectedEdges{
        ul: (1, 0, 1),
        um: (1, 0, 2),
        ur: (1, 0, 3), 

        lu: (1, 1, 0),
        lm: (1, 2, 0),
        ld: (1, 3, 0), 

        dl: (1, 4, 1),
        dm: (1, 4, 2),
        dr: (1, 4, 3), 

        ru: (1, 1, 4), 
        rm: (1, 2, 4), 
        rd: (1, 3, 4),
    };

    // Call the rotate_in_direction function to perform the F or F' move on the rubiks cube array
    rotate_in_direction(cube, corners, edges, sides, direction);
}
