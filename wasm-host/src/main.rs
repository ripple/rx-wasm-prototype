mod vm;
use crate::vm::{run_func_int};
// const RECUR: i32 = 2_000_000_000;
//
//
// // #[no_mangle]
// fn recur(r: i32, count: &mut i32) {
//     if r > 0 {
//         *count += 1;
//         recur(r - 1, count);
//     }
// }

fn main() {
    // let mut count = 0;
    // recur(RECUR, &mut count);
    // println!("{:?}", count);
    // recur(RECUR);

    let wasm_lib_file = "/home/pwang/wasm/rx-wasm-prototype/wasm-lib/target/wasm32-wasip1/release/wasm_lib.wasm";
    let output = run_func_int(wasm_lib_file, "recursive", 100_000);
    println!("{:?}", output);
}

