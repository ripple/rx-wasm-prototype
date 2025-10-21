use std::time::Instant;
use wasmtime::*;

fn main() -> Result<()> {
    let mut config = Config::new();
    config.consume_fuel(true);

    let wasm_code = "0061736d010000000105016000017f03020100070a010666696e69736800000a0601040041010b";
    let wasm_binary = hex::decode(wasm_code).expect("Invalid hex string");

    let start_all = Instant::now();

    let start = Instant::now();
    let engine = Engine::new(&config)?;
    let duration_engine = start.elapsed();

    let start = Instant::now();
    let mut store = Store::new(&engine, ());
    let duration_store = start.elapsed();

    let start = Instant::now();
    store.set_fuel(1_000_000_000_000)?;
    let duration_set_fuel = start.elapsed();

    let start = Instant::now();
    let module = Module::from_binary(store.engine(), &wasm_binary)?;
    let duration_module = start.elapsed();

    let start = Instant::now();
    let instance = Instance::new(&mut store, &module, &[])?;
    let duration_instance = start.elapsed();

    let start = Instant::now();
    let func = instance.get_typed_func::<(), i32>(&mut store, "finish")?;
    let duration_func = start.elapsed();

    let start = Instant::now();
    let result = func.call(&mut store, ())?;
    let duration_call = start.elapsed();

    let start = Instant::now();
    let consumed_fuel = 1_000_000_000_000 - store.get_fuel()?;
    let duration_get_fuel = start.elapsed();

    println!("result {:?}", result);
    println!("Fuel consumed: {}", consumed_fuel);

    println!("create engine   : {:?}", duration_engine );
    println!("create store    : {:?}", duration_store );
    println!("set fuel        : {:?}", duration_set_fuel );
    println!("load module     : {:?}", duration_module);
    println!("create instance : {:?}", duration_instance);
    println!("get function    : {:?}", duration_func);
    println!("call function   : {:?}", duration_call);
    println!("get fuel        : {:?}", duration_get_fuel);
    println!("all with timer  : {:?}", start_all.elapsed());

    Ok(())
}