import torch
import torch.nn as nn

"""
Credit for this code goes to https://github.com/automan000/Convolutional_LSTM_PyTorch/blob/master/convolution_lstm.py

I have modified it a bit to use 3D conv layers, but the core logic of the code is still here. Muchos appreciados.


"""

class ConvLSTMCell(nn.Module):
    """
    ConvLSTMCell implements a single cell of ConvLSTM.

    Attributes:
    input_channels (int): Number of channels of the input tensor.
    hidden_channels (int): Number of channels of the hidden state.
    kernel_size (int): Size of the kernel in convolutions.
    """
    def __init__(self, input_channels, hidden_channels, kernel_size):
        super(ConvLSTMCell, self).__init__()

        # Ensure that the number of hidden channels is even for architectural consistency
        assert hidden_channels % 2 == 0

        self.input_channels = input_channels
        self.hidden_channels = hidden_channels
        self.kernel_size = kernel_size
        self.num_features = 4  # LSTM has four gates

        # Calculate padding size to keep the spatial dimensions constant
        self.padding = int((kernel_size - 1) / 2)

        # Define the convolutional layers for the LSTM gates
        # Conv3d is used to process 3D data (e.g., volumetric data or sequences of images)
        self.Wxi = nn.Conv3d(self.input_channels, self.hidden_channels, self.kernel_size, 1, self.padding, bias=True)
        self.Whi = nn.Conv3d(self.hidden_channels, self.hidden_channels, self.kernel_size, 1, self.padding, bias=False)
        self.Wxf = nn.Conv3d(self.input_channels, self.hidden_channels, self.kernel_size, 1, self.padding, bias=True)
        self.Whf = nn.Conv3d(self.hidden_channels, self.hidden_channels, self.kernel_size, 1, self.padding, bias=False)
        self.Wxc = nn.Conv3d(self.input_channels, self.hidden_channels, self.kernel_size, 1, self.padding, bias=True)
        self.Whc = nn.Conv3d(self.hidden_channels, self.hidden_channels, self.kernel_size, 1, self.padding, bias=False)
        self.Wxo = nn.Conv3d(self.input_channels, self.hidden_channels, self.kernel_size, 1, self.padding, bias=True)
        self.Who = nn.Conv3d(self.hidden_channels, self.hidden_channels, self.kernel_size, 1, self.padding, bias=False)

        # Peephole connections (optional in LSTM), initialized later
        self.Wci = None
        self.Wcf = None
        self.Wco = None

    def forward(self, x, h, c):
        """
        Defines the forward pass for the ConvLSTM cell.

        Args:
        x (Tensor): The input tensor.
        h (Tensor): The hidden state from the previous time step.
        c (Tensor): The cell state from the previous time step.

        Returns:
        (Tensor, Tensor): The new hidden state and cell state.
        """
        # Compute the LSTM gates with convolutional operations
        ci = torch.sigmoid(self.Wxi(x) + self.Whi(h) + c * self.Wci)
        cf = torch.sigmoid(self.Wxf(x) + self.Whf(h) + c * self.Wcf)
        cc = cf * c + ci * torch.tanh(self.Wxc(x) + self.Whc(h))
        co = torch.sigmoid(self.Wxo(x) + self.Who(h) + cc * self.Wco)
        ch = co * torch.tanh(cc)
        return ch, cc

    def init_hidden(self, batch_size, hidden, shape):
        """
        Initializes hidden states for the first time step.

        Args:
        batch_size (int): Size of the batch.
        hidden (int): Number of hidden channels.
        shape (tuple): Shape of the spatial dimensions.

        Returns:
        (Tensor, Tensor): Initialized hidden state and cell state.
        """
        if self.Wci is None:
            # Initialize peephole connections
            self.Wci = nn.Parameter(torch.zeros(1, hidden, shape[0], shape[1], shape[2]))
            self.Wcf = nn.Parameter(torch.zeros(1, hidden, shape[0], shape[1], shape[2]))
            self.Wco = nn.Parameter(torch.zeros(1, hidden, shape[0], shape[1], shape[2]))
        else:
            # Check if the input shape matches the expected shape
            assert shape[0] == self.Wci.size()[2], 'Input Depth Mismatched!'
            assert shape[1] == self.Wci.size()[3], 'Input Height Mismatched!'
            assert shape[2] == self.Wci.size()[4], 'Input Width Mismatched!'
        return ((torch.zeros(batch_size, hidden, shape[0], shape[1], shape[2])), 
                torch.zeros(batch_size, hidden, shape[0], shape[1], shape[2]))

class ConvLSTM(nn.Module):
    """
    ConvLSTM implements a multi-layer convolutional LSTM network.

    This ConvLSTM model is designed to process a sequence of 3D data (such as a sequence of images)
    and output the state from the last time step. It's suitable for tasks where the final state after
    processing the entire sequence is important, such as in sequence-to-one modeling.

    Attributes:
        input_channels (int): Number of channels in the input tensor.
        hidden_channels (list of int): Number of channels in hidden states for each layer.
        kernel_size (int): Size of the kernel in convolutions.
        num_layers (int): Number of layers in the ConvLSTM.
        _all_layers (list): Internal list that stores all the ConvLSTMCell layers.
    """

    def __init__(self, input_channels, hidden_channels, kernel_size):
        """
        Initializes the ConvLSTM network.

        Args:
            input_channels (int): Number of channels in the input tensor.
            hidden_channels (list of int): Number of channels in hidden states for each layer.
            kernel_size (int): Size of the kernel in convolutions.
        """
        super(ConvLSTM, self).__init__()
        self.input_channels = [input_channels] + hidden_channels
        self.hidden_channels = hidden_channels
        self.kernel_size = kernel_size
        self.num_layers = len(hidden_channels)
        self._all_layers = []
        for i in range(self.num_layers):
            name = 'cell{}'.format(i)
            cell = ConvLSTMCell(self.input_channels[i], self.hidden_channels[i], self.kernel_size)
            setattr(self, name, cell)
            self._all_layers.append(cell)

    def forward(self, input):
        """
        Defines the forward pass of the ConvLSTM.

        Processes the input through each time step and layer of ConvLSTM cells, and returns only the final output
        after the last time step. This output reflects the state of the network after processing the entire sequence.

        Args:
            input (Tensor): The input tensor with shape [batch size, time-series steps, channels, depth, height, width].

        Returns:
            Tensor: The output tensor from the last time step with shape [batch size, channels, depth, height, width].
        """
        internal_state = []

        # Iterate over time series steps
        for time_step in range(input.size(1)):
            x = input[:, time_step, :, :, :, :].float()

            for layer_idx in range(self.num_layers):
                name = 'cell{}'.format(layer_idx)
                if time_step == 0:
                    bsize, _, depth, height, width = x.size()
                    (h, c) = getattr(self, name).init_hidden(bsize, self.hidden_channels[layer_idx], (depth, height, width))
                    internal_state.append((h, c))

                (h, c) = internal_state[layer_idx]
                x, new_c = getattr(self, name)(x, h, c)
                internal_state[layer_idx] = (x, new_c)

        # Return the output from the last time step
        return x


class ConvLSTMClassifier(nn.Module):
    """
    ConvLSTMClassifier combines the ConvLSTM with a classification head.

    Attributes:
    conv_lstm (ConvLSTM): The ConvLSTM model.
    num_classes (int): Number of classes for classification.
    """
    def __init__(self, conv_lstm, num_output_features, num_classes=13):
        super(ConvLSTMClassifier, self).__init__()
        self.conv_lstm = conv_lstm
        self.num_classes = num_classes

        # The final output of ConvLSTM has shape [batch size, channels, depth, height, width]
        # We need to add layers to transform this to a 12-class output
        # Example: a simple classifier with a flatten layer and a fully connected layer
        # Adjust the in_features depending on the output size of your ConvLSTM
        self.flatten = nn.Flatten()
        self.fc = nn.Linear(in_features=num_output_features, out_features=num_classes)  # Replace '...' with the correct size

    def forward(self, x):
        # Forward pass through ConvLSTM
        conv_lstm_output = self.conv_lstm(x)
        
        # Flatten and pass through the classification layers
        x = self.flatten(conv_lstm_output)
        x = self.fc(x)
        return x
    
