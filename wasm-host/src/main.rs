mod vm;
use crate::vm::run_func;

fn main() {
    let wasm_lib_file = "/home/pwang/wasm/rx-wasm-prototype/wasm-lib/target/wasm32-wasip1/release/wasm_lib.wasm";
    let output = run_func(wasm_lib_file, "looping_host");
    println!("{:?}", output);
}
