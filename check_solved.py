import torch
import subprocess   
import json
import os

'''
The Check class in check_solved.py is responsible for creating a python object that can call on various rust programs in order to determine whether 
or not the rubiks cube is solved. The Check class will also be able to check for different milestones within the training process, such as if the 
cross is solved via calling the different rust programs.
'''


class Check:

    def __init__(self):
        subprocess.run(["./scripts/compile_solution_verifier.sh"])

    # this function checks if the entire cube is solved. It recieves a string of 
    # all moves that have been made to the cube. Both the scramble and current solution
    def is_solved(self, scramble : str, solution : str) -> int:

        
        os.chdir("scripts")
        subprocess.run(["./check_solved.sh", scramble, solution])

        # read the stdout of check_solved.sh
        with open("../solve_status.json", "r") as f:
           solved = json.load(f)

        return solved

