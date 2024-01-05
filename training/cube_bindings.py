import subprocess
import os
import random
import json
import torch
from torch import Tensor
from typing import Tuple

'''
cube_bindings.py contains a a class that provides python bindings to the "cube" program written in rust.
'''

class Cube:
    def __init__(self) -> None:
        print(os.getcwd())

    def generate_data(self, batch_size : int, scramble_len : int) -> Tuple[Tensor, list[str]]:
        # Generate a random 20 move scramble of the possible moves
        moves = ["U", "U'", "D", "D'", "L", "L'", "R", "R'", "F", "F'", "B", "B'"]
        generate_scramble = lambda :  " ".join([random.choice(moves) for _ in range(scramble_len)])
        scrambles = [generate_scramble() for _ in range(batch_size)]

        # call apply_scramble.sh
        subprocess.run(["training/train_scripts/apply_scramble.sh", *scrambles])

        # read the output of apply_scramble.sh 
        with open("training/json/scrambled_cube_states.json", "r") as f:
            data = json.load(f)
            
        return torch.tensor(data), scrambles
    
    
    def apply_moves(self, moves_list : list[str]) -> Tensor:
        """
        apply_moves() accepts a list of scrambles and applies them to the cube using the apply_scramble.sh script.
        """
        subprocess.run(["training/train_scripts/apply_scramble.sh", *moves_list])

        with open("training/json/scrambled_cube_states.json", "r") as f:
            data = json.load(f)

        return torch.tensor(data)
    
    def is_solved(self, scramble : str, solution : str) -> bool:

        subprocess.run(["training/train_scripts/check_solved.sh", scramble, solution])

        # read the stdout of check_solved.sh
        with open("training/json/solve_status.json", "r") as f:
           solved = json.load(f)

        if solved:
            return True
        else:
            return False

    def is_cross_solved(self, scramble : str) -> bool:

        subprocess.run(["training/train_scripts/is_cross_solved.sh", scramble])

        # read the stdout of check_solved.sh
        with open("training/json/cross_solve_status.json", "r") as f:
           solved = json.load(f)

        if solved:
            return True
        else:
            return False

    def solve_cross(self, scramble : str) -> str:

        subprocess.run(["training/train_scripts/solve_cross.sh", scramble])

        # read the stdout of check_solved.sh
        with open("training/json/cross_solution.json", "r") as f:
           solution = json.load(f)

        return solution