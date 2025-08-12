#!/bin/bash
set -e

echo "🔧 Setting up build environment..."

# Add Rust target for WebAssembly
rustup target add wasm32-unknown-unknown

# Install trunk and wasm-bindgen-cli with specific versions
echo "📦 Installing trunk and wasm-bindgen-cli..."
cargo install trunk --version 0.21.14
cargo install wasm-bindgen-cli --version 0.2.100

# Build frontend first
echo "🎨 Building frontend..."
cd frontend
trunk build --release
cd ..

# Verify frontend was built
echo "✅ Verifying frontend build..."
if [ -f "frontend/dist/index.html" ]; then
    echo "Frontend built successfully!"
    ls -la frontend/dist/
else
    echo "❌ Frontend build failed!"
    exit 1
fi

# Build backend
echo "🦀 Building backend..."
cargo build --release

# Verify backend was built
echo "✅ Verifying backend build..."
if [ -f "target/release/ascii-converter" ]; then
    echo "Backend built successfully!"
    ls -la target/release/ascii-converter
else
    echo "❌ Backend build failed!"
    exit 1
fi

echo "🎉 Build completed successfully!"