to set up wasm target in the Rust compiler toolchain:
```
rustup target add wasm32-unknown-unknown --toolchain nightly
```

to compile Rust to wasm:
```
cargo +nightly build --target wasm32-unknown-unknown
```

to shrink wasm module size, use `wasm-gc`:
```
cargo install wasm-gc
wasm-gc [ORIGINAL_FILE].wasm -o [TARGET_FILE].wasm
```
