import numpy as np
import matplotlib.pyplot as plt
from mpl_toolkits.mplot3d import Axes3D
from typing import NoReturn

class Plot:
    def __init__(self):
        pass

    @staticmethod
    def show_rubiks_(array: np.ndarray) -> NoReturn:
        """
        Plots a 3D numpy array as a 3D grid, with different colors for different values.

        Args:
        array (numpy.ndarray): A 3D numpy array.

        The function plots the array in 3D space, using different colors for different values:
        - 0: Not displayed
        - 1: Red
        - 2: White
        - 3: Green
        - 4: Yellow
        - 5: Blue
        - 6: Orange
        """

        # Define the color map
        color_map = {1: 'red', 2: 'white', 3: 'green', 4: 'yellow', 5: 'blue', 6: 'orange'}

        fig = plt.figure()
        ax = fig.add_subplot(111, projection='3d')

        # Get the shape of the array
        nx, ny, nz = array.shape

        # Iterate through the array and plot the points
        for x in range(nx):
            for y in range(ny):
                for z in range(nz):
                    if array[x, y, z] != 0:
                        # with different shape points
                        ax.scatter(x, y, z, color=color_map.get(array[x, y, z], 'black'), s=1000, marker='h', edgecolors='black')

        # Set labels and title
        ax.set_xlabel('X axis')
        ax.set_ylabel('Y axis')
        ax.set_zlabel('Z axis')
        ax.set_title('Numerical Rubiks Cube Visualization')

        plt.show()
