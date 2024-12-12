use std::collections::HashMap;
use wasmedge_sdk::{params, wasi::WasiModule, Module, Store, Vm, WasmVal};

fn main() {
    let mut wasi_module = WasiModule::create(None, None, None).unwrap();
    let mut instances = HashMap::new();
    instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());
    let mut vm = Vm::new(Store::new(None, instances).unwrap());

    //TODO filename in command line args	
    let math_wasm_lib_file = "/home/pwang/wasm/rx-wasm-prototype/mock-math-lib/target/wasm32-wasip1/release/mock_math_lib.wasm";
    let wasm_lib_file = "/home/pwang/wasm/rx-wasm-prototype/wasm-lib/target/wasm32-wasip1/release/wasm_lib.wasm";
    let math_module = Module::from_file(None, &math_wasm_lib_file).unwrap();
    let wasm_module = Module::from_file(None, &wasm_lib_file).unwrap();
    vm.register_module(Some("mock_math_lib"), math_module).unwrap();
    vm.register_module(Some("wasm-lib"), wasm_module).unwrap();

    let a: i32 = 5;
    let b: i32 = 3;
    let res = vm.run_func(Some("wasm-lib"), "add", params!(a, b)).unwrap();
    println!("add({}, {}) = {}", a, b, res[0].to_i32());
}
