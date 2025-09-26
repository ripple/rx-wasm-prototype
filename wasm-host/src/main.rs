use std::path::PathBuf;
use std::time::Instant;
use log::debug;
use wamr_rust_sdk::function::Function;
use wamr_rust_sdk::instance::Instance;
use wamr_rust_sdk::module::Module;
use wamr_rust_sdk::runtime::Runtime;

fn main() {
    // let wasm_file = "/home/pwang/wasm/rx-wasm-prototype/wat/test.wasm";
    let wasm_file = "/Users/pwang/wasm/rx-wasm-prototype/wat/test.wasm";

    debug!("Loading WASM module from file: {}", wasm_file);
    let wasm_path = PathBuf::from(wasm_file);

    let runtime = Runtime::builder()
        .use_system_allocator()
        .run_as_interpreter()
        .build().unwrap();
    let module = Module::from_file(&runtime, wasm_path.as_path()).unwrap();
    let instance = Instance::new(&runtime, &module, 1024 * 128).unwrap();
    let gas_cap: Option<u32> = Some(2_000_000_000);
    debug!("Executing WASM function");
    let func = Function::find_export_func(&instance, "finish").unwrap();
    let start = Instant::now();
    let results = func.call(&instance, &vec![], gas_cap).unwrap();
    let duration = start.elapsed();

    println!("result {:?}, gas {}", results.0, 2_000_000_000-results.1);
    println!("Execution time: {:?}", duration);
    // println!()
}
