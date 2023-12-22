import torch
import torch.nn as nn
from torch.autograd import Variable

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
            self.Wci = nn.Parameter(torch.zeros(1, hidden, shape[0], shape[1], shape[2])).cpu()
            self.Wcf = nn.Parameter(torch.zeros(1, hidden, shape[0], shape[1], shape[2])).cpu()
            self.Wco = nn.Parameter(torch.zeros(1, hidden, shape[0], shape[1], shape[2])).cpu()
        else:
            # Check if the input shape matches the expected shape
            assert shape[0] == self.Wci.size()[2], 'Input Depth Mismatched!'
            assert shape[1] == self.Wci.size()[3], 'Input Height Mismatched!'
            assert shape[2] == self.Wci.size()[4], 'Input Width Mismatched!'
        return (Variable(torch.zeros(batch_size, hidden, shape[0], shape[1], shape[2])).cpu(),
                Variable(torch.zeros(batch_size, hidden, shape[0], shape[1], shape[2])).cpu())

class ConvLSTM(nn.Module):
    """
    ConvLSTM implements a multi-layer convolutional LSTM.

    Attributes:
    input_channels (int): Number of channels in the input tensor.
    hidden_channels (list of int): Number of channels in hidden states for each layer.
    kernel_size (int): Size of the kernel in convolutions.
    step (int): Number of time steps to unroll the LSTM.
    effective_step (list of int): Time steps at which outputs are recorded.
    """
    def __init__(self, input_channels, hidden_channels, kernel_size, step=1, effective_step=[1]):
        super(ConvLSTM, self).__init__()
        self.input_channels = [input_channels] + hidden_channels
        self.hidden_channels = hidden_channels
        self.kernel_size = kernel_size
        self.num_layers = len(hidden_channels)
        self.step = step
        self.effective_step = effective_step
        self._all_layers = []
        for i in range(self.num_layers):
            # Create and add ConvLSTM cells for each layer
            name = 'cell{}'.format(i)
            cell = ConvLSTMCell(self.input_channels[i], self.hidden_channels[i], self.kernel_size)
            setattr(self, name, cell)
            self._all_layers.append(cell)

    def forward(self, input):
        """
        Defines the forward pass of the ConvLSTM.

        Args:
        input (Tensor): The input tensor with shape [batch-size, time-series-steps, channels, depth, height, width].

        Returns:
        list of Tensor: Recorded outputs at effective steps.
        (Tensor, Tensor): The last hidden state and cell state.
        """
        internal_state = []
        outputs = []

        # Iterate over time series steps
        for time_step in range(input.size(1)):

            # Extract the data for the current time step
            x = input[:, time_step, :, :, :, :]

            for layer_idx in range(self.num_layers):
                name = 'cell{}'.format(layer_idx)
                # Initialize internal states for the first time step
                if time_step == 0:
                    bsize, _, depth, height, width = x.size()
                    (h, c) = getattr(self, name).init_hidden(batch_size=bsize, hidden=self.hidden_channels[layer_idx],
                                                             shape=(depth, height, width))
                    internal_state.append((h, c))

                # Forward pass through each ConvLSTM cell
                (h, c) = internal_state[layer_idx]
                x, new_c = getattr(self, name)(x, h, c)
                internal_state[layer_idx] = (x, new_c)

            # Record outputs at specified effective steps
            if time_step in self.effective_step:
                outputs.append(x)

        return outputs, (x, new_c)
    
class ConvLSTMClassifier(nn.Module):
    """
    ConvLSTMClassifier combines the ConvLSTM with a classification head.

    Attributes:
    conv_lstm (ConvLSTM): The ConvLSTM model.
    num_classes (int): Number of classes for classification.
    """
    def __init__(self, conv_lstm, num_output_features, num_classes=12):
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
        conv_lstm_output, _ = self.conv_lstm(x)
        # Take the last output for classification
        x = conv_lstm_output[-1]  # Assuming we use the last time step for classification

        # Flatten and pass through the classification layers
        x = self.flatten(x)
        x = self.fc(x)
        return x
    
if __name__ == '__main__':
    # Initialize ConvLSTM
    conv_lstm = ConvLSTM(input_channels=1, hidden_channels=[8, 16, 32, 32, 32], kernel_size=3, step=5, effective_step=[4]).cpu()

    num_output_features = 32 * 16 * 16 * 16  # Replace with the correct size

    # Initialize ConvLSTMClassifier
    classifier = ConvLSTMClassifier(conv_lstm, num_output_features, num_classes=12).cpu()

    # Example input and target data for classification
    input = Variable(torch.randn(1, 5, 1, 16, 16, 16)).cpu()
    target = Variable(torch.empty(1, dtype=torch.long).random_(12)).cpu()  # Random target classes

    # Forward pass through the classifier
    output = classifier(input)

    # argmax the output probability vector
    prediction = output.argmax(dim=1)

    # Print shapes for verification
    print('Input size:', input.shape)
    print('Output size:', output.shape)
    print('Prediction size:', prediction.shape)
    print('Target size:', target.shape)

    # Loss and gradient check (for classification task)
    loss_fn = torch.nn.CrossEntropyLoss()
    loss = loss_fn(output, target)
    loss.backward()

    # # Uncomment below to perform gradient check
    # res = torch.autograd.gradcheck(loss_fn, (output, target), eps=1e-6, raise_exception=True)
    # print(res)
