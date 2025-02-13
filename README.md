## compile the prototype

Install wasm target:
```
rustup target add wasm32-unknown-unknown
```
Compile the libs to wasm (in both of the libs' folds):
```
cargo build --target wasm32-unknown-unknown --release
```
Build the host (in the host's fold):
```
cargo build
```