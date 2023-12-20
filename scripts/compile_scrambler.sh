#!/bin/bash

# Define the path to the directory containing your Rust project
PROJECT_DIR="scrambler"

# Change to the project directory
cd "$PROJECT_DIR" || { echo "Error: Directory '$PROJECT_DIR' not found."; exit 1; }

# Check if Cargo.toml exists (indicating it's a Rust project)
if [ ! -f "Cargo.toml" ]; then
    echo "Error: 'Cargo.toml' not found. Make sure you are in a Rust project directory."
    exit 1
fi

# Compile the Rust program in release mode using Cargo
echo "Compiling the scrambler Rust program in release mode..."
cargo build --release

# Check if compilation was successful
if [ $? -eq 0 ]; then
    echo "Compilation successful."
else
    echo "Compilation failed."
fi
