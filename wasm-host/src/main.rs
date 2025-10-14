use std::path::PathBuf;
use std::time::Instant;
use wasmtime::*;
use log::debug;
use clap::Parser;

#[derive(Parser)]
#[command(name = "wasm-host")]
#[command(about = "A WASM runtime host", long_about = None)]
struct Args {
    /// Path to the WASM file to execute
    wasm_file: PathBuf,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let wasm_path = args.wasm_file;
    
    debug!("Loading WASM module from file: {}", wasm_path.display());

    let mut config = Config::new();
    config.consume_fuel(true);
    // config.strategy(Strategy::Cranelift);
    // config.strategy(Strategy::Winch);
    // config.cranelift_opt_level(OptLevel::Speed);
    // config.cranelift_opt_level(OptLevel::None);
    // config.cranelift_opt_level(OptLevel::SpeedAndSize);
    let _ = config.target("pulley64").unwrap();

    let start = Instant::now();
    let engine = Engine::new(&config)?;
    let mut store = Store::new(&engine, ());
    store.set_fuel(1_000_000_000_000)?;
    let module = Module::from_file(store.engine(), &wasm_path)?;
    let instance = Instance::new(&mut store, &module, &[])?;
    debug!("Executing WASM function");
    let func = instance.get_typed_func::<(), i32>(&mut store, "finish")?;

    let start2 = Instant::now();
    let result = func.call(&mut store, ())?;
    let duration2 = start2.elapsed();
    let duration = start.elapsed();

    // 5. Get the amount of fuel consumed
    let consumed_fuel = 1_000_000_000_000 - store.get_fuel()?;

    println!("result {:?}", result);
    println!("load time:      {:?}", duration-duration2);
    println!("execution time: {:?}", duration2);
    println!("total time:     {:?}", duration);
    println!("Fuel consumed: {}", consumed_fuel);

    Ok(())
}