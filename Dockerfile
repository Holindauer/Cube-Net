FROM python:3.9-slim

WORKDIR /usr/src/app

# Copy just the requirements.txt first to leverage Docker cache
COPY requirements.txt ./


RUN pip install -r requirements.txt

# replace the above line with this one below for smaller image size
#RUN pip install --no-cache-dir -r requirements.txt

# Copy the rest of your application code
COPY . .

# Install Rust and build tools
RUN apt-get update && apt-get install -y curl build-essential
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y
ENV PATH="/root/.cargo/bin:${PATH}"


RUN pwd
RUN ls


# Run docker_training.py when the container launches
CMD ["python3", "docker_training.py"]
