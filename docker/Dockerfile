# Use the official Rust image as the base image
FROM rust:1.83-slim AS builder

# Install required packages for building
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    git \
    && rm -rf /var/lib/apt/lists/*

# Set the working directory
WORKDIR /app

# Copy the Cargo.toml and Cargo.lock files
COPY Cargo.toml Cargo.lock ./

# Copy the source code
COPY src ./src

# Build the application in release mode
RUN cargo build --release

# Create a new stage for the runtime image
FROM debian:bookworm-slim

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    git \
    && rm -rf /var/lib/apt/lists/*

# Create a non-root user
RUN useradd --create-home --shell /bin/bash app \
    && chown -R app:app /home/app

# Copy the built binary from the builder stage
COPY --from=builder /app/target/release/whisper /usr/local/bin/whisper

# Set the user
USER app

# Set the working directory
WORKDIR /home/app

# Set the entrypoint
ENTRYPOINT ["whisper"]

# Default command
CMD ["--help"]