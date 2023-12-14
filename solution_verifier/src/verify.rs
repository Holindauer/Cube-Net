use crate::cube;

// Write a function that determines if the cube is solved or not by iterating through the cube array of the scramble+solution cube and comparing it a new solve cube array
// If the cube is solved, print "Cube is solved!"
pub fn is_solved(cube: &[[[i32; 5]; 5]; 5]) -> bool {
    let solved_cube = cube::create_solved_cube();
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                if cube[i][j][k] != solved_cube[i][j][k] {
                    return false;
                }
            }
        }
    }
    true
}