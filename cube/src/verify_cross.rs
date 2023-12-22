use crate::moves;


// verify_cross.rs contains the code that verifies that the cross is solved.

// The yellow cross is solved if the yellow face has a yellow cross on it and the yellow cross is 
// oriented correctly with regards to the connected edges. The function will rotate the yellow face 
// up to 4 times to check if the cross is solved but not oriented correctly.

// The function returns true if the cross is solved and false otherwise.
pub fn verify_cross(cube: &mut [[[i32; 5]; 5]; 5]) -> bool {

    // create a for loop with 4 iterations
    for i in 0..4 {

        // check if the cross is solved
        if check_cross(cube) == 4 {
            return true;
        }

        // rotate the D face and check again
        moves::d(cube, 'c')
    }

    // if the cross is not solved return false
    return false;

    
}

// This function checks if each of the edges of the cross of the yellow face are in place.
// The function uses the cube abstaction from the cube_move_library/cube.rs file.
pub fn check_cross(cube: &mut [[[i32; 5]; 5]; 5]) -> i32 {

    let mut correct_edges = 0;

    // check that red/yellow edge is in place
    if cube[0][3][2] == 1 && cube[1][4][2] == 4{
        correct_edges += 1;
    }

    // check that green/yellow edge is in place
    if cube[2][3][0] == 3 && cube [2][4][1] == 4{
        correct_edges += 1;
    }

    // check that blue/yellow edge is in place
    if cube[2][3][4] == 5 && cube[2][4][3] == 4{
        correct_edges += 1;
    }

    // check that orange/yellow edge is in place
    if cube[4][3][2] == 6 && cube[3][4][2] == 4{
        correct_edges += 1;
    }

    return correct_edges;
}