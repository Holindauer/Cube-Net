/*
I am realizing that the cross and F2L algorithms will be the most difficult part of this determinisitc Rubiks cube algorithm.
The cross and  F2L both are more flexible in terms of how you solve them than OLL and PLL. OLL and PLL will just involve checking
for a particalur state and then applying the correct algotithm for that state. However, cross and F2L logic will need to be 
subdivided into multiple steps in order to determine how to deterministically solve the more open problem of the cross and F2L.
*/


/*
The Cubie struct is used to store a single cubie of the rubiks cube such that the orientationof the colors is implicit by way of 
which members contain values > 0. The 6 integer members represent the color of a sticker on an edge. Although there are 6, only 2 
or 3 members will be populated at a time depending on if it is a corner or an edge.

NOTE: the orientation of these edges is the same as how standard cubing notation does it. The point of reference is the red face, 
with the white face on top.
*/
#[derive(Copy, Clone)]
pub struct Cubie {
    pub up: i32,
    pub down: i32,
    pub left: i32,
    pub right: i32,
    pub front: i32,
    pub back: i32,
}

impl Cubie {
    pub fn new() -> Self {
        Cubie {
            up: 0,
            down: 0,
            left: 0,
            right: 0,
            front: 0,
            back: 0,
        }
    }
}


// This function will be used to convert the cube state in its 5x5x5 Tensor Representation into the 3x3x3 Cubie Represenation.
// The scramble argument is used to recreate a given rubiks cube state using the mechanics of the cube_move_library crate.
pub fn edge_converter(tensor_cube: &[[[i32; 5]; 5]; 5] )  -> [[[Cubie; 3]; 3]; 3] {

    // Create a 3x3x3 array of Cubie structs
    let mut cubies = [[[Cubie::new(); 3]; 3]; 3]; 

    // convert the edges of the tensor cube into the cubie representation
    populate_edges(&mut cubies, &tensor_cube);
    
    cubies
}

// This fucntion is called within the edge_converter function and is used to populate the edges of the tensor cube representaiton
// This is done in a seperate function so it can be reused for the F2L algorithm in addition to the analogous function for corners.
// It returns a struct of the indicies of the edges in the cubies array.
pub fn populate_edges(cubies : &mut [[[Cubie; 3]; 3]; 3], tensor_cube : &[[[i32; 5]; 5]; 5]) {

    // front upper edge
    cubies[0][0][1].up = tensor_cube[1][0][2];
    cubies[0][0][1].front = tensor_cube[0][1][2];

    // front left edge
    cubies[0][1][0].left = tensor_cube[1][2][0];
    cubies[0][1][0].front = tensor_cube[0][2][1];

    // front down edge 
    cubies[0][2][1].down = tensor_cube[1][4][2];
    cubies[0][2][1].front = tensor_cube[0][3][2];

    // front right edge 
    cubies[0][1][2].right = tensor_cube[1][2][4];
    cubies[0][1][2].front = tensor_cube[0][2][3];

    // middle upper left edge
    cubies[1][0][0].up = tensor_cube[2][0][1];
    cubies[1][0][0].left = tensor_cube[2][1][0];

    // middle upper right edge
    cubies[1][0][2].up = tensor_cube[2][0][3];
    cubies[1][0][2].right = tensor_cube[2][1][4];

    // middle down right edge
    cubies[1][2][2].down = tensor_cube[2][4][3];
    cubies[1][2][2].right = tensor_cube[2][3][4];

    // middle down left edge
    cubies[1][2][0].down = tensor_cube[2][4][1];
    cubies[1][2][0].left = tensor_cube[2][3][0];

    // NOTE: these back edges are being input from the perspective of the front face.
    // For example, the front left edge is on the same face as the back left edge.

    // back upper edge
    cubies[2][0][1].up = tensor_cube[3][0][2];
    cubies[2][0][1].back = tensor_cube[4][1][2];

    // back left edge 
    cubies[2][1][0].left = tensor_cube[3][2][0];
    cubies[2][1][0].back = tensor_cube[4][2][1];

    // back down edge
    cubies[2][2][1].down = tensor_cube[3][4][2];
    cubies[2][2][1].back = tensor_cube[4][3][2];

    // back right edge
    cubies[2][1][2].right = tensor_cube[3][2][4];
    cubies[2][1][2].back = tensor_cube[4][2][3];
}