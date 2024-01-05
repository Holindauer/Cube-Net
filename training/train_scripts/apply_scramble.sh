#!/bin/bash

cd cube/target/release

# Path to the precompiled Rust binary
RUST_PROGRAM="./scramble"

# Pass all arguments to the Rust program
"$RUST_PROGRAM" "$@"
