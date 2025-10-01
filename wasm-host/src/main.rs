use std::fs;
use sha2::{Digest, Sha512};
use soroban_wasmi::{
    core::{Trap, },
    Caller, Config, Engine, Extern, Func, Linker, Module, Store,
};
use std::path::PathBuf;
use std::time::Instant;

// This hashing function remains unchanged.
pub fn sha512_half(data: &[u8]) -> Vec<u8> {
    let mut hasher = Sha512::new();
    hasher.update(data);
    let result = hasher.finalize();
    result[..32].to_vec()
}

// Helper to read data from Wasm memory.
// Note: In `wasmi`, it's often cleaner to get the memory once inside the main host function
// and pass it to helpers, rather than resolving the export multiple times.
fn get_data_from_memory(
    memory: &soroban_wasmi::Memory,
    store: &impl soroban_wasmi::AsContext,
    pointer: u32,
    len: u32,
) -> Result<Vec<u8>, Trap> {
    let pointer = pointer as usize;
    let len = len as usize;

    // `wasmi` provides a `read` method that performs bounds checking for you.
    let mut buffer = vec![0u8; len];
    memory.read(store, pointer, &mut buffer).unwrap();

    Ok(buffer)
}

// Helper to write data to Wasm memory.
fn set_data_in_memory(
    memory: &soroban_wasmi::Memory,
    store: &mut impl soroban_wasmi::AsContextMut,
    data: &[u8],
    output_pointer: u32,
) -> Result<(), Trap> {
    // `wasmi` provides a `write` method that performs bounds checking.
    memory.write(store, output_pointer as usize, data).unwrap();
    Ok(())
}

/// The host function, adapted for the `soroban-wasmi` API.
pub fn compute_sha512_half(
    mut caller: Caller<'_, ()>,
    in_buf_ptr: u32,
    in_buf_len: u32,
    out_buf_ptr: u32,
    out_buf_cap: u32,
) -> i32 {
    // --- soroban-wasmi Change: Host Function Fuel Metering ---
    // `soroban-wasmi` provides a direct `consume_fuel` method.
    // This is much cleaner than the get/set pattern in wasmtime.
    // This will return an error if there isn't enough fuel.
    caller.set_fuel(caller.get_fuel().unwrap().saturating_sub( 1000)).unwrap();
    // let custom_cost = 2_000;
    // let mut have = caller.get_fuel().unwrap();
    //
    // if have < custom_cost {
    //     // If fuel is insufficient, we panic. The VM will catch this and trap execution.
    //     panic!("Host function trap: Out of fuel.");
    // }
    //
    // have = have.saturating_sub(custom_cost);
    //
    // caller.set_fuel(have).unwrap();
    //     // consume_fuel(custom_cot)?;

    if 32 > out_buf_cap {
        return -1;
        // We return a `Trap` to signal an error to the Wasm module.
        // return Err(Trap::new(TrapCode::UnreachableCodeReached));
            // .with_message("Output buffer capacity is less than 32"));
    }

    if in_buf_len > 1024 {
        return -1;
            // .with_message("Input buffer length exceeds 1024"));
    }

    // --- soroban-wasmi Change: Memory Access ---
    // Get the exported memory from the caller. The name "mem" must match the export name in your Wasm.
    let memory = caller
        .get_export("mem")
        .and_then(Extern::into_memory).unwrap();
        // .ok_or_else(|| Trap::new(TrapCode::MemoryOutOfBounds));

    // Read the input data from Wasm memory.
    let data = get_data_from_memory(&memory, &caller, in_buf_ptr, in_buf_len).unwrap();

    // Perform the computation.
    let hash_half = sha512_half(&data);

    // Write the result back to Wasm memory.
    set_data_in_memory(&memory, &mut caller, &hash_half, out_buf_ptr).unwrap();

    // Return the number of bytes written.
    32
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();

    // --- soroban-wasmi Change: Setup & Configuration ---
    // 1. Create a `Config` and enable fuel consumption.
    let mut config = Config::default();
    config.consume_fuel(true);
    config.floats(false);

    // 2. Create the `Engine` and a `Store`. The store holds the fuel.
    let engine = Engine::new(&config);
    let mut store = Store::new(&engine, ());

    // 3. Set the initial fuel. This is the "gas limit" for the transaction.
    let initial_fuel = 18_000;
    store.set_fuel(initial_fuel)?;
    println!("Initial fuel limit set to: {}", initial_fuel);

    // --- soroban-wasmi Change: Linking ---
    // 4. Create a `Linker` and define the host function import.
    let mut linker = Linker::new(&engine);
    linker.func_wrap(
        "host_lib", // Module name in Wasm
        "compute_sha512_half", // Function name in Wasm
        compute_sha512_half,
    )?;

    // 5. Load and compile the Wasm module.
    let wasm_file = PathBuf::from("/Users/pwang/wasm/rx-wasm-prototype/wat/test.wasm"); // Make sure test.wasm is in the project root
    println!("Loading WASM module from: {:?}", wasm_file);
    let wasm_bytes = fs::read(wasm_file)?;
    let module = Module::new(&engine, &wasm_bytes)?;

    let start = Instant::now();

    // 6. Instantiate the module, linking the host functions.
    let instance = linker.instantiate(&mut store, &module)?.start(&mut store)?;

    // 7. Get the exported Wasm function you want to call.
    let func: Func = instance.get_func(&mut store, "finish").ok_or("finish export not found")?;

    let start2 = Instant::now();
    let mut result_buffer = [soroban_wasmi::Val::I32(0)];
    func.call(&mut store, &[], &mut result_buffer)?;
    let duration2 = start2.elapsed();
    let duration = start.elapsed();

    let result = result_buffer[0].i32().ok_or("Invalid result type")?;

    // --- soroban-wasmi Change: Reading Consumed Fuel ---
    // `wasmi` tracks both remaining and consumed fuel.
    // let consumed_fuel = store.fuel_consumed().unwrap();
    let remaining_fuel = store.get_fuel().unwrap();

    println!("\n--- Results ---");
    println!("Wasm function returned: {:?}", result);
    println!("Total execution time:    {:?}", duration);
    println!("Function execution time: {:?}", duration2);
    println!("Remaining fuel: {} units", remaining_fuel);
    println!("Fuel consumed: {} units", initial_fuel - remaining_fuel);


    Ok(())
}
