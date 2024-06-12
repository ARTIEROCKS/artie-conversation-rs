# Use the official Rust image as the base
FROM rust:1.71 as builder

# Install protoc (Protocol Buffers compiler)
RUN apt-get update && apt-get install -y protobuf-compiler

# Set the working directory
WORKDIR /usr/src/artie-conversation

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Create an empty directory for build dependencies
RUN mkdir src
RUN echo "fn main() {}" > src/main.rs

# Compile the dependencies
RUN cargo build --release
RUN rm -r src

# Copy the rest of the project files
COPY . .

# Compile the project
RUN cargo build --release

# Create a new stage for a lighter image
FROM debian:buster-slim

# Install necessary dependencies to run the binary
RUN apt-get update && apt-get install -y ca-certificates && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/local/bin

# Copy the compiled binary from the previous stage
COPY --from=builder /usr/src/artie-conversation/target/release/artie-conversation .

# Expose the port where the gRPC service will be listening
EXPOSE 50051

# Set environment variables
ENV RUST_LOG=debug

# Command to run the binary
CMD ["./artie-conversation"]