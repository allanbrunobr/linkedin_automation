# Use an official Rust image as a parent image
FROM rust:latest

# Set the working directory inside the container
WORKDIR /usr/src/backend

# Copy the current directory contents into the container at /usr/src/backend
COPY . .

# Install dependencies and compile the project
RUN cargo build --release

# Set the entry point for the backend
CMD ["cargo", "run", "--release"]
