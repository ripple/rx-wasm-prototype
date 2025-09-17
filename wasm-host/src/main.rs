use std::path::PathBuf;
use std::time::Instant;
use wasmtime::*;
use log::debug;

fn main() -> Result<()> {
    // 1. Set up the Wasmtime environment with fuel enabled
    // let wasm_file = "/home/pwang/wasm/rx-wasm-prototype/wat/test.wasm";
    let wasm_file = "/Users/pwang/wasm/rx-wasm-prototype/wat/test.wasm";
    debug!("Loading WASM module from file: {}", wasm_file);
    let wasm_path = PathBuf::from(wasm_file);

    let mut config = Config::new();
    config.consume_fuel(true);
    let _ = config.target("pulley64").unwrap();
    // config.strategy(Strategy::Cranelift);
    // config.cranelift_opt_level(OptLevel::Speed);


    let start = Instant::now();

    let engine = Engine::new(&config)?;
    // The Store holds our fuel counter
    let mut store = Store::new(&engine, ());

    // 2. Set the initial fuel limit for this execution
    // You can set this to any large number. 1_000_000_000 is a good starting point.
    store.set_fuel(1_000_000_000_000)?;

    // 3. Load the module and create an instance
    let module = Module::from_file(store.engine(), &wasm_path)?;
    let instance = Instance::new(&mut store, &module, &[])?;

    // 4. Find and call the exported function
    debug!("Executing WASM function");
    let func = instance.get_typed_func::<(), i32>(&mut store, "finish")?;

    let start2 = Instant::now();
    let result = func.call(&mut store, ())?;
    let duration2 = start2.elapsed();
    let duration = start.elapsed();

    // 5. Get the amount of fuel consumed
    let consumed_fuel = 1_000_000_000_000 - store.get_fuel()?;

    println!("result {:?}", result);
    println!("total execution time:    {:?}", duration);
    println!("function execution time: {:?}", duration2);
    println!("Fuel consumed: {}", consumed_fuel);

    Ok(())
}