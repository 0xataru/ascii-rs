#!/bin/bash

# Check if frontend was built
if [ ! -f "frontend/dist/index.html" ]; then
    echo "⚠️ Frontend not found, using static fallback"
    mkdir -p frontend/dist
    cp static/index.html frontend/dist/index.html
fi

# Start the server
echo "🚀 Starting ASCII Converter server..."
exec ./target/release/ascii-converter
