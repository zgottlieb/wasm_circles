# wasm_circles

This repository provides a small example of using Rust to compile to WebAssembly and doing Javascript interop via the WebAssembly memory. It is based on this tutorial of using C to compile to wasm: https://egghead.io/courses/get-started-using-webassembly-wasm. The WebGL portion of this repo is taken directly from that project.

to set up wasm target in the Rust compiler toolchain:
```
rustup target add wasm32-unknown-unknown --toolchain nightly
```

to compile Rust to wasm:
```
cargo +nightly build --target wasm32-unknown-unknown --release
```

to shrink wasm module size, use `wasm-gc`:
```
cargo install wasm-gc
wasm-gc [ORIGINAL_FILE].wasm -o [TARGET_FILE].wasm
```
