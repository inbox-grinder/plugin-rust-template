#!/usr/bin/env bash
# Build the Rust plugin as both a native binary and a WASM module.
# Run from the rust-template directory.
set -euo pipefail

cd "$(dirname "$0")"

echo "==> Building binary (local-binary runtime)..."
cargo build --release --bin plugin
cp target/release/plugin plugin
echo "    plugin ($(du -sh plugin | cut -f1))"

echo "==> Building WASM (wasm-component runtime)..."
# Requires: rustup target add wasm32-unknown-unknown
cargo build --release --target wasm32-unknown-unknown
cp target/wasm32-unknown-unknown/release/rust_template.wasm plugin.wasm
echo "    plugin.wasm ($(du -sh plugin.wasm | cut -f1))"

echo ""
echo "Done. To switch between runtimes, copy the matching manifest:"
echo "  cp manifest.json manifest-binary.json      # for local-binary"
echo "  cp manifest-wasm.json manifest.json        # for wasm-component"
