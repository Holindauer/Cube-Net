from cube_bindings import Cube
from model_cross import ConvLSTMClassifier, ConvLSTM
import torch
from train_cross import Trainer, TrainConfig
from early_stopping import EarlyStopping
import torch
import math as m
import os

if __name__ == "__main__":

    cube = Cube()

    config = TrainConfig(
        scramble_len=40,
        epochs=1,
        val_num_batches=10,
        batch_size=32,
        lr=0.001,
        device=torch.device("cuda" if torch.cuda.is_available() else "cpu"),
        optimizer=torch.optim.Adam,
        early_stopping=EarlyStopping(patience=10),
        num_classes=13
    )

    print(f"using device: {config.device}")

   # Initialize the ConvLSTM
    conv_lstm = ConvLSTM(input_channels=1, hidden_channels=[8, 16, 32, 32, 64], kernel_size=3)

    num_output_features = 64 * 5 * 5 * 5  # Replace with the correct size

    # Initialize ConvLSTMClassifier
    classifier = ConvLSTMClassifier(conv_lstm, num_output_features, num_classes=13)


    trainer = Trainer(cube, config, classifier)

    trainer.train()