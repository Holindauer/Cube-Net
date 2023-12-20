#!/bin/bash

# Define the path to the directory containing your Rust project
PROJECT_DIR="scrambler"

# Change to the project directory
cd "$PROJECT_DIR" || { echo "Error: Directory '$PROJECT_DIR' not found."; exit 1; }

# Path to the precompiled Rust binary
RUST_PROGRAM="./target/release/scrambler"

# Pass all arguments to the Rust program
"$RUST_PROGRAM" "$@"
