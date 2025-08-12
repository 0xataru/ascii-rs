# Use official Rust image
FROM rust:1.89

# Install system dependencies
RUN apt-get update && apt-get install -y \
    build-essential \
    && rm -rf /var/lib/apt/lists/*

# Set working directory
WORKDIR /app

# Copy source code
COPY . .

# Add wasm32 target
RUN rustup target add wasm32-unknown-unknown

# Install trunk and wasm-bindgen-cli
RUN cargo install trunk --version 0.21.14
RUN cargo install wasm-bindgen-cli --version 0.2.100

# Build frontend
WORKDIR /app/frontend
RUN trunk build --release

# Build backend
WORKDIR /app
RUN cargo build --release

# Expose port
EXPOSE 10000

# Set environment variables
ENV PORT=10000
ENV RUST_LOG=info

# Run the application
CMD ["./target/release/ascii-converter"]