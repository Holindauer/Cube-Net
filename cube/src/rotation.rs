// rotation.rs contains functions that perform rotations on the face of the rubiks cube array. There are three individual functions
// that perform any one rotation. Corners_Clockwise, Corners_Counter_Clockwise, Edges_Clockwise, Edges_Counter_Clockwise, 
// Connected_Edges_Clockwise, and Connected_Edges_Counter_Clockwise. These functions are used by the functions in moves.rs to perform
// To perform the 12 possible rubiks cube moves: U U' R R' F F' L L' B B' D D'. Each of them uses a struct to store the 3D indicies of 
// the face edges, face corners, and edges connected to that face (which will also be turned in a rotation). These structs are defined
// in this file but are instantiated before the calls within the functions of moves.rs.





//-------------------------------------------------------------------------------------------------------------------------------- Face Corner Rotation Abstraction
// This struct contains the indices of the 3D cube array of the corners of a face of the cube when
// you look at it after rotating it directly from the red side to the face you are looking at.
pub struct FaceCorners {
    pub ul: (usize, usize, usize), // up left
    pub ur: (usize, usize, usize), // up right
    pub dl: (usize, usize, usize), // down left
    pub dr: (usize, usize, usize), // down right
}

// This function rotates the corners of the face clockwise
pub fn corners_clockwise(cube: &mut [[[i32; 5]; 5]; 5], corners : FaceCorners){

    // Store the value of the face's upper left corner as a temp
    let temp = cube[corners.ul.0][corners.ul.1][corners.ul.2]; 

    cube[corners.ul.0][corners.ul.1][corners.ul.2] = cube[corners.dl.0][corners.dl.1][corners.dl.2]; // Place down left corner into the upper left corner 
    cube[corners.dl.0][corners.dl.1][corners.dl.2] = cube[corners.dr.0][corners.dr.1][corners.dr.2]; // Place down right corner into the down left corner
    cube[corners.dr.0][corners.dr.1][corners.dr.2] = cube[corners.ur.0][corners.ur.1][corners.ur.2]; // Place up right corner into the down right corner

    // Place temp (upper left) into the up right corner
    cube[corners.ur.0][corners.ur.1][corners.ur.2] = temp; 
}

// This function rotates the corners of the face counter clockwise
pub fn corners_counter_clockwise(cube: &mut [[[i32; 5]; 5]; 5], corners : FaceCorners){

    // Store the value of the face's upper left corner as a temp
    let temp = cube[corners.ul.0][corners.ul.1][corners.ul.2]; 

    cube[corners.ul.0][corners.ul.1][corners.ul.2] = cube[corners.ur.0][corners.ur.1][corners.ur.2]; // Place up right corner into the upper left corner 
    cube[corners.ur.0][corners.ur.1][corners.ur.2] = cube[corners.dr.0][corners.dr.1][corners.dr.2]; // Place down right corner into the up right corner
    cube[corners.dr.0][corners.dr.1][corners.dr.2] = cube[corners.dl.0][corners.dl.1][corners.dl.2]; // Place down left corner into the down right corner

    // Place temp (upper left) into the down left corner
    cube[corners.dl.0][corners.dl.1][corners.dl.2] = temp; 
}

//-------------------------------------------------------------------------------------------------------------------------------- Face Edges Rotation Abstraction

// This struct contains the indices of the 3D cube array of the edges of a face of the cube when
// you look at it after rotating it directly from the red side to the face you are looking at.
pub struct FaceEdges {
    pub u : (usize, usize, usize), // up
    pub l : (usize, usize, usize), // left
    pub d : (usize, usize, usize), // down
    pub r : (usize, usize, usize), // right
}

// This function rotates the edges of the face clockwise
pub fn edges_clockwise(cube: &mut [[[i32; 5]; 5]; 5], edges : FaceEdges){

    // Store the value of the face's upper edge as a temp
    let temp = cube[edges.u.0][edges.u.1][edges.u.2]; 

    cube[edges.u.0][edges.u.1][edges.u.2] = cube[edges.l.0][edges.l.1][edges.l.2]; // Place left edge into the upper edge
    cube[edges.l.0][edges.l.1][edges.l.2] = cube[edges.d.0][edges.d.1][edges.d.2]; // Place down edge into the left edge
    cube[edges.d.0][edges.d.1][edges.d.2] = cube[edges.r.0][edges.r.1][edges.r.2]; // Place right edge into the down edge

    // Place temp (upper) into the right edge
    cube[edges.r.0][edges.r.1][edges.r.2] = temp;
}

pub fn edges_counter_clockwise(cube: &mut [[[i32; 5]; 5]; 5], edges : FaceEdges){

    // Store the value of the face's upper edge as a temp
    let temp = cube[edges.u.0][edges.u.1][edges.u.2]; 

    cube[edges.u.0][edges.u.1][edges.u.2] = cube[edges.r.0][edges.r.1][edges.r.2]; // Place right edge into the upper edge
    cube[edges.r.0][edges.r.1][edges.r.2] = cube[edges.d.0][edges.d.1][edges.d.2]; // Place down edge into the right edge
    cube[edges.d.0][edges.d.1][edges.d.2] = cube[edges.l.0][edges.l.1][edges.l.2]; // Place left edge into the down edge

    // Place temp (upper) into the left edge
    cube[edges.l.0][edges.l.1][edges.l.2] = temp;
}


//-------------------------------------------------------------------------------------------------------------------------------- Connected Edges Rotation Abstraction

// This struct contains the indicies of the 3D cube array of the edges connected to a face of the cube when
// you look at it after rotating it directly from the red side to the face you are looking at.
pub struct ConnectedEdges {
    pub ul: (usize, usize, usize), // up left
    pub um: (usize, usize, usize), // up middle
    pub ur: (usize, usize, usize), // up right

    pub lu: (usize, usize, usize), // left up
    pub lm: (usize, usize, usize), // left middle
    pub ld: (usize, usize, usize), // left down

    pub dl: (usize, usize, usize), // down left
    pub dm: (usize, usize, usize), // down middle
    pub dr: (usize, usize, usize), // down right

    pub ru: (usize, usize, usize), // right up
    pub rm: (usize, usize, usize), // right middle
    pub rd: (usize, usize, usize), // right down
}


// This function rotates the connected edges of the face clockwise. There are 9 total connected edges
pub fn connected_edges_clockwise(cube: &mut [[[i32; 5]; 5]; 5], sides : ConnectedEdges){

    // Set 3 temps for the upper connected edges
    let temp1 = cube[sides.ul.0][sides.ul.1][sides.ul.2];  // upper left
    let temp2 = cube[sides.um.0][sides.um.1][sides.um.2];  // upper middle
    let temp3 = cube[sides.ur.0][sides.ur.1][sides.ur.2];  // upper right


    // place left connected edges into the upper connected edges
    cube[sides.ul.0][sides.ul.1][sides.ul.2] = cube[sides.ld.0][sides.ld.1][sides.ld.2]; // upper left <---- left down
    cube[sides.um.0][sides.um.1][sides.um.2] = cube[sides.lm.0][sides.lm.1][sides.lm.2]; // upper middle <---- left middle
    cube[sides.ur.0][sides.ur.1][sides.ur.2] = cube[sides.lu.0][sides.lu.1][sides.lu.2]; // upper right <---- left up

    // place down connected edges into the left connected edges
    cube[sides.ld.0][sides.ld.1][sides.ld.2] = cube[sides.dr.0][sides.dr.1][sides.dr.2]; // left down <---- down right
    cube[sides.lm.0][sides.lm.1][sides.lm.2] = cube[sides.dm.0][sides.dm.1][sides.dm.2]; // left middle <---- down middle
    cube[sides.lu.0][sides.lu.1][sides.lu.2] = cube[sides.dl.0][sides.dl.1][sides.dl.2]; // left up <---- down left

    // place right connected edges into the down connected edges
    cube[sides.dr.0][sides.dr.1][sides.dr.2] = cube[sides.ru.0][sides.ru.1][sides.ru.2]; // down right <---- right up
    cube[sides.dm.0][sides.dm.1][sides.dm.2] = cube[sides.rm.0][sides.rm.1][sides.rm.2]; // down middle <---- right middle
    cube[sides.dl.0][sides.dl.1][sides.dl.2] = cube[sides.rd.0][sides.rd.1][sides.rd.2]; // down left <---- right down

    // place upper connected edges into the right connected edges
    cube[sides.ru.0][sides.ru.1][sides.ru.2] = temp1; // right up <---- upper left
    cube[sides.rm.0][sides.rm.1][sides.rm.2] = temp2; // right middle <---- upper middle
    cube[sides.rd.0][sides.rd.1][sides.rd.2] = temp3; // right down <---- upper right
}

// This function rotates the connected edges of the face counter clockwise. There are 9 total connected edges
pub fn connected_edges_counter_clockwise(cube: &mut [[[i32; 5]; 5]; 5], sides : ConnectedEdges){

    // Set 3 temps for the upper connected edges
    let temp1 = cube[sides.ul.0][sides.ul.1][sides.ul.2];  // upper left
    let temp2 = cube[sides.um.0][sides.um.1][sides.um.2];  // upper middle
    let temp3 = cube[sides.ur.0][sides.ur.1][sides.ur.2];  // upper right

    // place right connected edges into the upper connected edges
    cube[sides.ul.0][sides.ul.1][sides.ul.2] = cube[sides.ru.0][sides.ru.1][sides.ru.2]; // upper left <---- right up
    cube[sides.um.0][sides.um.1][sides.um.2] = cube[sides.rm.0][sides.rm.1][sides.rm.2]; // upper middle <---- right middle
    cube[sides.ur.0][sides.ur.1][sides.ur.2] = cube[sides.rd.0][sides.rd.1][sides.rd.2]; // upper right <---- right down

    // place down connected edges into the right connected edges
    cube[sides.ru.0][sides.ru.1][sides.ru.2] = cube[sides.dr.0][sides.dr.1][sides.dr.2]; // right up <---- down right
    cube[sides.rm.0][sides.rm.1][sides.rm.2] = cube[sides.dm.0][sides.dm.1][sides.dm.2]; // right middle <---- down middle
    cube[sides.rd.0][sides.rd.1][sides.rd.2] = cube[sides.dl.0][sides.dl.1][sides.dl.2]; // right down <---- down left

    // place left connected edges into the down connected edges
    cube[sides.dr.0][sides.dr.1][sides.dr.2] = cube[sides.ld.0][sides.ld.1][sides.ld.2]; // down right <---- left down
    cube[sides.dm.0][sides.dm.1][sides.dm.2] = cube[sides.lm.0][sides.lm.1][sides.lm.2]; // down middle <---- left middle
    cube[sides.dl.0][sides.dl.1][sides.dl.2] = cube[sides.lu.0][sides.lu.1][sides.lu.2]; // down left <---- left up

    // place upper connected edges into the left connected edges
    cube[sides.ld.0][sides.ld.1][sides.ld.2] = temp1; // left down <---- upper left
    cube[sides.lm.0][sides.lm.1][sides.lm.2] = temp2; // left middle <---- upper middle
    cube[sides.lu.0][sides.lu.1][sides.lu.2] = temp3; // left up <---- upper right
}
