#!/bin/bash

# Build script for ASCII Converter full-stack application

set -e

echo "🦀 Building ASCII Converter..."

# Check if required tools are installed
if ! command -v trunk &> /dev/null; then
    echo "Installing Trunk..."
    cargo install trunk
fi

if ! command -v wasm-bindgen &> /dev/null; then
    echo "Installing wasm-bindgen-cli..."
    cargo install wasm-bindgen-cli
fi

# Add WASM target if not already added
rustup target add wasm32-unknown-unknown

echo "📦 Building frontend..."
cd frontend
trunk build --release --public-url /
cd ..

echo "🔧 Building backend..."
cargo build --release

echo "✅ Build complete!"
echo "Frontend files are in: frontend/dist/"
echo "Backend binary is in: target/release/ascii-converter"
echo ""
echo "To run locally:"
echo "  ./target/release/ascii-converter"
echo "  then open http://localhost:3000"
