#!/bin/bash

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen target/wasm32-unknown-unknown/release/roguelike.wasm --out-dir wasm --no-modules --no-typescript
mv wasm/roguelike_bg.wasm wasm/project.wasm
mv wasm/roguelike.js wasm/project.js
