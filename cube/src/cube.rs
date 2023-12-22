// cube.rs contains the function for creating a solved rubiks cube array and the function for displaying it to the terminal.


/* 
This function creates a solved rubik's cube represented as a 3D array. The color of each face is represented by a number.
The dimmensions of the cube are 5x5 in order to represent the edges of the cube in a way that is easy to understand.

So How Do I Understand This Abstraction?
--------------------------------------------------------------------------------------------------------------------------
- Hold a rubiks cube in front of your face with the red side facing you and the white side facing up. The Red side is the 
  the first nested 2D array of the 3D array (the 2D array with ones in the middle). Each subsequent 2D array represents a 
  cross section of the rubiks cube as you move from the red side to the orange side. 

- The white and yellow sides are the represented within the middle 3 of these cross sections as 2 and 4 respectively.
- The green and blue sides are represented within the middle 3 of these cross sections as 3 and 5 respectively.

//  1 = red, 2 = white, 3 = green, 4 = yellow, 5 = blue, 6 = orange. 
*/
pub fn create_solved_cube() -> [[[i32; 5]; 5]; 5] {
    [
        [
            [0, 0, 0, 0, 0],  // <---  
            [0, 1, 1, 1, 0],  // <--- Red side of the cube facing you when looking at red with white on top.
            [0, 1, 1, 1, 0],  // <---
            [0, 1, 1, 1, 0],  // <---
            [0, 0, 0, 0, 0],  // <---
        ],

        [
            [0, 2, 2, 2, 0],  // <--- Top of cube 
            [3, 0, 0, 0, 5], 
            [3, 0, 0, 0, 5],  
            [3, 0, 0, 0, 5],
            [0, 4, 4, 4, 0],  // <--- Bottom of cube
        ],
        [
            [0, 2, 2, 2, 0],  
            [3, 0, 0, 0, 5],  // <--- 3's and 5's are the sides of the cube 
            [3, 0, 0, 0, 5],  // <---
            [3, 0, 0, 0, 5],  // <---
            [0, 4, 4, 4, 0],
        ],
        [
            [0, 2, 2, 2, 0],
            [3, 0, 0, 0, 5],
            [3, 0, 0, 0, 5],
            [3, 0, 0, 0, 5],
            [0, 4, 4, 4, 0],
        ],
        [
            [0, 0, 0, 0, 0], // <--- 
            [0, 6, 6, 6, 0], // <--- Back of the cube (orange) 
            [0, 6, 6, 6, 0], // <---
            [0, 6, 6, 6, 0], // <---
            [0, 0, 0, 0, 0], // <---
        ],
    ]
}


// This function is used to display "Cross Sections" of the the rubiks cube array. The 
// first dim spans the cube from the red side to the orange side (with white on top).
pub fn display_cube(array: &[[[i32; 5]; 5]; 5]) {
    for i in 0..array.len() {

        println!("Slice 1 {}:", i + 1);
        for j in 0..array[i].len() {
            for k in 0..array[i][j].len() {
                print!("{:4}", array[i][j][k]);
            }
            println!();
        }
        println!();
    }
}