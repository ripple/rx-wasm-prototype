## Mock Math Library

This project exists to simulate a simple math library implemented in C++ so simulate a WASM host function.
The intention here is mimic host functions that will ultimately be available to the WASM VM when it's running inside of
`rippled`.


# Build

Install wasm target:

```
rustup target add wasm32-unknown-unknown
```

Compile to WASM:

```
cargo build --target wasm32-unknown-unknown --release
```