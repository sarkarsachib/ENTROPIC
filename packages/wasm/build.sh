#!/bin/bash
set -e

echo "🔨 Building WASM validator..."

# Check if wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "❌ wasm-pack not found. Installing..."
    cargo install wasm-pack
fi

# Build WASM package
wasm-pack build --target bundler --out-dir pkg

echo "✅ WASM build complete! Output: packages/wasm/pkg/"
