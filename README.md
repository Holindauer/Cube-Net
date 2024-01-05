# Cube-Net

## What is Cube-Net?
Cube-Net is an experimental project attempting to create an existence proof for **Neural Network Compilation**. The program being compiled , it is an attempt to "compile" an inneficient, but competent program for solving a rubiks cube into a neural network while maintaining functionality and increasing efficiency. 

Let's unpack that statement.

I'm using the term **Neural Network Compilation** to mean the process of encoding a program written in source code into a neural network such that inference with the trained model preserves the correct execution of the program.

I'm not sure the extent of potential use cases, but I believe this idea could be useful for a kind of *refactoring by optimization*. Meaning that if you had a program that worked, but was also inneficient, you could make it faster by representing it as a neural network. The reason I think this is because supervised machine learning takes a feature vector and a target vector and literally optimizes a model to be an efficient computable representation of the functional relation defined by the dataset of such vectors. Thus, if you rigged up a training pipeline such that each feature vector was an expected input for a program and the target vector was an actual output of the program, the model would be optimized to find an efficient representation of the program. This strikes me as a sort of offshoot/hybrid of reinforcement and supervised learning.

## Project Structure

As mentioned above, the program being compiled is a rubiks cube solver. I chose this specific program mostly out of personal interest in the idea. To read more on the model choice and architecture design, [read this](cube-net-theory.md). The rubiks solver program is written in rust and is located in [Cube-Net/cube](cube) directory. Training files can be found in [Cube-Net/training](training).

## Project Status + Contributing
At this stage, the project is still in the early development stages. I have been building out the training pipeline mentioned above but have not yet begun training. If you find the idea compelling and would like to contribute, feel free to email me at holindauer@gmail.com

