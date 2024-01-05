# Stage 1: Build Rust application in release mode
FROM rust:1.58 as rust-builder

# Copy the source code of Rust application
COPY ./cube /cube
WORKDIR /cube

# Build the Rust application
RUN cargo build --release

# Final stage with PyTorch and Python environment
FROM pytorch/pytorch:latest

# Switch to root user for installation
USER root

# Install necessary packages (if any)
RUN apt-get update && apt-get install -y curl build-essential

# Copy the built Rust binaries from the builder stage
# Replace 'binary1', 'binary2', etc. with your actual binary names
COPY --from=rust-builder /cube/target/release/is_cross_solved /cube/target/release/is_cross_solved
COPY --from=rust-builder /cube/target/release/scramble /cube/target/release/scramble
COPY --from=rust-builder /cube/target/release/solve_cross /cube/target/release/solve_cross
COPY --from=rust-builder /cube/target/release/verify_solution /cube/target/release/verify_solution

# Switch back to non-root user if necessary
# USER [your-non-root-user]

# Copy Python code or other necessary files
COPY ./training ./training

# Run training script when the container launches
CMD ["python3", "training/train_cross.py"]
