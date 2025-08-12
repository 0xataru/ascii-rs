# Multi-stage build for Rust full-stack application
FROM rust:1.75 as builder

# Install required tools
RUN cargo install trunk wasm-bindgen-cli
RUN rustup target add wasm32-unknown-unknown

WORKDIR /app

# Copy Cargo files
COPY Cargo.toml Cargo.lock ./
COPY frontend/Cargo.toml frontend/Cargo.lock ./frontend/

# Create dummy source files to cache dependencies
RUN mkdir src && echo "fn main() {}" > src/main.rs
RUN mkdir frontend/src && echo "use wasm_bindgen::prelude::*; #[wasm_bindgen(start)] pub fn main() {}" > frontend/src/lib.rs

# Build dependencies (cached layer)
RUN cargo build --release
RUN cd frontend && cargo build --release --target wasm32-unknown-unknown

# Copy actual source code
COPY . .

# Build frontend
RUN cd frontend && trunk build --release --public-url /

# Build backend
RUN cargo build --release

# Runtime image
FROM debian:bookworm-slim

# Install CA certificates and clean up
RUN apt-get update && \
    apt-get install -y ca-certificates && \
    rm -rf /var/lib/apt/lists/*

WORKDIR /app

# Copy built application and frontend assets
COPY --from=builder /app/target/release/ascii-converter /usr/local/bin/ascii-converter
COPY --from=builder /app/frontend/dist ./frontend/dist

# Set environment variables
ENV PORT=10000
ENV RUST_LOG=info

# Expose port
EXPOSE 10000

# Health check
HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
  CMD curl -f http://localhost:${PORT}/health || exit 1

# Run the application
CMD ["ascii-converter"]
