use std::time::Instant;
use wamr_rust_sdk::function::Function;
use wamr_rust_sdk::instance::Instance;
use wamr_rust_sdk::module::Module;
use wamr_rust_sdk::runtime::Runtime;

/*
(module
  ;; Define a function type that takes no parameters and returns an i32
  (type $t0 (func (result i32)))

  ;; Define the function ($f0 is a typical internal name)
  (func $f0 (type $t0)
    (i32.const 1)
  )

  ;; Export the function, linking the internal $f0 name to the external "finish" name
  (export "finish" (func $f0))
)
*/

fn main() {
    let wasm_code = "0061736d010000000105016000017f03020100070a010666696e69736800000a0601040041010b";
    let wasm_binary = hex::decode(wasm_code).expect("Invalid hex string");

    let start_all = Instant::now();

    let start = Instant::now();
    let runtime = Runtime::builder()
        .use_system_allocator()
        .run_as_interpreter()
        .build().unwrap();
    let duration_engine = start.elapsed();

    let start = Instant::now();
    let module = Module::from_vec(&runtime, wasm_binary, "return_1").unwrap();
    let duration_module = start.elapsed();

    let start = Instant::now();
    let instance = Instance::new(&runtime, &module, 1024 * 128).unwrap();
    let duration_instance = start.elapsed();

    let start = Instant::now();
    let func = Function::find_export_func(&instance, "finish").unwrap();
    let duration_func = start.elapsed();

    let start = Instant::now();
    let results = func.call(&instance, &vec![], Some(2_000_000_000)).unwrap();
    let duration_call = start.elapsed();

    println!("result {:?}", results.0[0]);
    println!("Fuel consumed: {}", 2_000_000_000 - results.1);

    println!("create engine   : {:?}", duration_engine );
    // println!("create store    : {:?}", duration_store );
    // println!("set fuel        : {:?}", duration_set_fuel );
    println!("load module     : {:?}", duration_module);
    println!("create instance : {:?}", duration_instance);
    println!("get function    : {:?}", duration_func);
    println!("call function   : {:?}", duration_call);
    // println!("get fuel        : {:?}", duration_get_fuel);
    println!("all with timer  : {:?}", start_all.elapsed());
}
