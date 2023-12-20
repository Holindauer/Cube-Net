import random
import subprocess
import json
import torch

''' 
The BatchGenerator class is used to generate a batch of scrambled rubiks cube states.

This is a two part process:
- First, we must generate a rubiks cube scramble, which is just a random list of the 12 possible moves that can be made on a rubiks cube.
- Then, the BatchGenerator will pass the scramble into the the rust program 'scrambler' in order to generate a scrambled rubiks cube state 
  that is represented as a 3D (5, 5, 5) array. These outputs will be retrieved by the BatchGenerator, collated into a Batch usable for training
  and returned to the caller.
'''

class BatchGenerator:
    def __init__(self, batch_size : int):
        self.batch_size = batch_size
        
        # compile the scrambler by runnign compile_scrambler.sh
        subprocess.run(["./scripts/compile_scrambler.sh"])

    def __call__(self):
    
        # generate a batch of scrambles
        scrambles = [self.generate_scramble() for _ in range(self.batch_size)]

        # call apply_scramble.sh
        subprocess.run(["./scripts/apply_scramble.sh", *scrambles])

        # read the output of apply_scramble.sh
        with open("scrambled_cube_states.json", "r") as f:
            data = json.load(f)
            
        return torch.tensor(data), scrambles
        
        
    def generate_scramble(self):
        # Generate a random 20 move scramble of the possible moves
        moves = ["U", "U'", "D", "D'", "L", "L'", "R", "R'", "F", "F'", "B", "B'"]
        return " ".join([random.choice(moves) for _ in range(40)])
    

        