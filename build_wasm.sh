#!/bin/bash
set -e

echo "Building Perspecta for WebAssembly..."

# Build for wasm32
cargo build --release --target wasm32-unknown-unknown

# Generate JS bindings
wasm-bindgen --no-typescript --target web \
    --out-dir ./wasm/ \
    --out-name "perspecta" \
    ./target/wasm32-unknown-unknown/release/perspecta.wasm

echo "Build complete! WASM files are in ./wasm/"
echo "To test locally, run: python3 -m http.server --directory wasm 8080"
echo "Then open http://localhost:8080 in your browser"
