# Use the official Rust image as the build stage
FROM rust:latest as build

# Set the working directory
WORKDIR /usr/src/cli-toool

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the project
RUN cargo build --release

# Use a minimal Ubuntu image for the final stage
FROM ubuntu:latest

# Install necessary packages
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /usr/local/bin

# Copy the compiled binary from the build stage
COPY --from=build /usr/src/cli-toool/target/release/cli-toool .

# Ensure the binary has execute permissions
RUN chmod +x cli-toool

# Set the entrypoint to the binary
ENTRYPOINT ["/bin/bash"]
