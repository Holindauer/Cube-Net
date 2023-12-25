use cube::cube::create_solved_cube;
use cube::moves::make_move;
use std::env;
use std::error::Error;

fn main() {
    let scrambles = match parse_scramble_args() {
        Ok(scrambles) => {
            scrambles
        }
        Err(e) => {
            eprintln!("Error: {}", e);
            return;
        }
    };

    let mut tensor_cubes = Vec::new();


    for scramble in scrambles {
        let mut tensor_cube = create_solved_cube();
        for turn in scramble {
            make_move(&mut tensor_cube, &turn);
        }
        tensor_cubes.push(tensor_cube);
    }
    
    let serialized = serde_json::to_string(&tensor_cubes).unwrap();

    // save serialized to file
    std::fs::write("../json/scrambled_cube_states.json", serialized).expect("Unable to write file");

}

fn parse_scramble_args() -> Result<Vec<Vec<String>>, Box<dyn Error>> {
    let args: Vec<String> = env::args().skip(1).collect();

    let scrambles = args.iter()
        .map(|arg| arg.split_whitespace().map(String::from).collect())
        .collect();

    Ok(scrambles)
}
