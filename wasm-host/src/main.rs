mod vm;

use std::collections::HashMap;
use wasmedge_sdk::{wasi::WasiModule, Module, Store, Vm};
use crate::vm::run_func;

fn main() {
    let mut wasi_module = WasiModule::create(None, None, None).unwrap();
    let mut instances = HashMap::new();
    instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());
    let mut vm = Vm::new(Store::new(None, instances).unwrap());

    let wasm_lib_file = "/home/pwang/wasm/rx-wasm-prototype/wasm-lib/target/wasm32-unknown-unknown/release/wasm_lib.wasm";
    let wasm_module = Module::from_file(None, &wasm_lib_file).unwrap();
    vm.register_module(None, wasm_module.clone()).unwrap();

    let output = run_func(&mut vm, "compare_accountID");
    println!("{:?}", output);
}
