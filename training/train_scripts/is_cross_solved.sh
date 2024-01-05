#!/bin/bash
pwd

cd cube/target/release

pwd

# Path to the precompiled Rust binary
RUST_PROGRAM="./is_cross_solved"

# Pass all arguments to the Rust program
"$RUST_PROGRAM" "$@"
