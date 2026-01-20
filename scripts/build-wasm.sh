#!/bin/bash
set -e

# Ensure wasm-pack is installed
if ! command -v wasm-pack &> /dev/null; then
    echo "wasm-pack could not be found. Please install it with 'curl https://rustwasm.github.io/wasm-pack/installer/init.sh -sSf | sh'"
    exit 1
fi

echo "Building WASM package..."
cd crates/wasm
wasm-pack build --target web --out-dir ../../packages/wasm-pkg --scope my-scope
echo "WASM build complete. Package located at packages/wasm-pkg"
