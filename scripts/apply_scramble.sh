#!/bin/bash

# Define the path to the directory containing your Rust project
PROJECT_DIR="cube"

cd ../cube

# Path to the precompiled Rust binary
RUST_PROGRAM="./target/release/scramble"

# Pass all arguments to the Rust program
"$RUST_PROGRAM" "$@"
