# Convolutional LSTM as Rubiks Cube Solver

This repository contains an attempt to create a Rubiks Cube Solver using a Convolutional LSTM Neural Network. 

I have been puzzled for some time now on how about how one might create a neural network that solves a rubiks cube. The task is not trivial and involves:
- Spatial awareness and reasoning,
- Developing a useful representation of the rubiks cube in memory,
- It is a sequential task where the order of the moves matter,
- And what I believe to be the most difficult part: It is difficult to create a loss function that is useful for training. This is because the ultimate goal is to create a correct solve, which is a series of moves. Predicating the loss on whether the model solved the cube is not useful because it does not inform the model on how to make useful individual moves.

After thinking about this for some time now, I have come up with the following solution:

## How to represent the rubiks cube data?
The rubiks cube will be represented in memory as an integer encoded 3D Tensor of size 5x5x5. The reason for being 5x5x5 is that although a rubiks cube has 3 "squares" per axis (when looking directly at a face), it should be noted that the "colors" of the "square" belong to a physical object. Corners and edges are single pieces. Neither a corner, nor an edge, can be disconnected from their adjacent colors. This means that a 3x3x3 representation of the rubiks cube is insufficient to represent the cube spatially in memory, it must be at least 5x5x5 to account for this geometry.

This representation will be created and "rotated" using a modification of a previous project of mine for creating [zero knowledge proofs of rubiks cube solutions](https://github.com/Holindauer/zk-Cube). This rust program is cabable of simulating all 12 moves of the rubiks cube in the 5x5x5 Tensor representation, as well as for verifying a complete solution.

## Surprise! We are not using a dataset to train the model... What? How?

Training a neural net without a dataset may initially seem impossible. Especially considering that machine learning is all about data. 

However, by taking a note from the reinforcement learning playbook, we can train a neural net by having it play against itself. But how can a rubiks cube play against itself? 

The answer is that we can compare the move made by the neural network to the output of a programatic/logical rubiks cube solver. This will work because the "mechanical" solver is deterministic. It will use the [CFOP](https://jperm.net/3x3/cfop) solving method to determine the next move that should be made given a cube state. Thus, an impromptu dataset will be created during training. Infinite data.

The idea of training, then, is to *encode the deterministic logical steps of the CFOP method into the neural network*!

## Model Architecture? Why ConvLSTM?

So we are using a sort of hybrid between supervised and reinforcement learning. Then  what model architecture should we use?

To address the the sequential and spatial nature of the task, we will use a Convolutional LSTM. The spatial nature of the task is addressed by the convolutional layers, and the sequential nature is addressed by the LSTM layers. 

So what is the output of the model? Its a 12 dimensional vector representing the probability of each move. The move with the highest probability is the move that the model will make.

The model will actually be *four models*. One for each of the four steps of the CFOP method. This design decision is to mitigate the vanishing/exploding gradient problem prevalent with RNNs. By breaking the problem into four smaller problems, we can train each model seperately and combine them for the final solver with more control. This will also allow us to train the model in stages, which will be useful for debugging.

Additionlly, each cube state will be appended into a time series that will inform the LSTM of the previous states of the cube when making its decision. This time series will be adjusted to maintain a cosntant maximum length.

## How to create a useful loss function?

Because the output of the model is a probability distribution that is argmaxed to determine the most likely move, we can use the cross entropy loss function. This is very useful because it means we don't have to predicate the loss on whether the model output an entire correct series of moves that solved the cube. We can simply predicate the loss on whether the model made the best move given the cube state according to CFOP. 

As well, another positive consequence of this is that it means there is more useful gradient information for the model to learn from. If the model makes a bad move, it will be penalized for it. If it makes a good move, it will be rewarded for it. This is a very useful property for training a neural network.
