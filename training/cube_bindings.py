import subprocess
import os
import random
import json
import torch
from dataclasses import dataclass

'''
cube_bindings.py contains a a class that provides python bindings to the "cube" program written in rust.
'''


@dataclass
class TrainConfig:
    batch_size :int
    scramble_len :int

class Cube:
    def __init__(self, config :TrainConfig):

        # compile the scrambler by running compile_rust_cube.sh
        subprocess.run(['../scripts/compile_rust_cube.sh'])

        self.batch_size = config.batch_size
        self.scramble_len = config.scramble_len

    def generate_data(self):
        # Generate a random 20 move scramble of the possible moves
        moves = ["U", "U'", "D", "D'", "L", "L'", "R", "R'", "F", "F'", "B", "B'"]
        generate_scramble = lambda :  " ".join([random.choice(moves) for _ in range(self.scramble_len)])
        scrambles = [generate_scramble() for _ in range(self.batch_size)]

        # call apply_scramble.sh
        subprocess.run(["../scripts/apply_scramble.sh", *scrambles])

        # read the output of apply_scramble.sh 
        with open("scrambled_cube_states.json", "r") as f:
            data = json.load(f)
            
        return torch.tensor(data), scrambles
    
    def is_solved(self, scramble : str, solution : str) -> int:

        subprocess.run(["../scripts/check_solved.sh", scramble, solution])

        # read the stdout of check_solved.sh
        with open("solve_status.json", "r") as f:
           solved = json.load(f)

        return solved

    def is_cross_solved(self, scramble : str) -> int:

        subprocess.run(["../scripts/is_cross_solved.sh", scramble])

        # read the stdout of check_solved.sh
        with open("cross_solve_status.json", "r") as f:
           solved = json.load(f)

        return solved

    def solve_cross(self, scramble : str) -> str:

        subprocess.run(["../scripts/solve_cross.sh", scramble])

        # read the stdout of check_solved.sh
        with open("cross_solution.json", "r") as f:
           solution = json.load(f)

        return solution