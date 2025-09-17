use std::path::PathBuf;
use std::time::Instant;
use wasmtime::*;
use log::debug;
use sha2::{Digest, Sha512};

pub fn sha512_half(data: &[u8]) -> Vec<u8> {

    let mut hasher = Sha512::new();
    hasher.update(data);
    let result = hasher.finalize();
    result[..32].to_vec()
}

fn get_data(caller: &mut Caller<'_, ()>, pointer_a: u32, len_a: u32) -> Vec<u8>{
    // 1. Get the Memory object from the caller.
    let memory = match caller.get_export("mem") {
        Some(Extern::Memory(mem)) => mem,
        _ => panic!("failed to find host memory"),
    };

    // 2. Access the memory and get a slice.
    // The `data_mut` method provides a mutable slice to the entire linear memory.
    let memory_slice = memory.data_mut(caller);

    // 3. Use the Wasm-provided pointer and length to create a sub-slice.
    // This sub-slice points to the specific data you need.
    let data = &memory_slice[(pointer_a as usize)..((pointer_a + len_a) as usize)];

    // // 4. Now you can use the `data` slice in your Rust code.
    // println!("Read data from Wasm: {:?}", data);
    // println!("Number of bytes read: {}", data.len());
    // println!("Data as a string: {}", String::from_utf8_lossy(data));

    data.to_vec()
}

pub fn set_data(
    caller: &mut Caller<'_, ()>,
    data: Vec<u8>,
    output_pointer: u32,
    output_len: u32,
) -> Result<()> {
    // Check if the provided data size matches the specified output length.
    if data.len() as u32 != output_len {
        return Err(anyhow::anyhow!("Data size does not match specified length."));
    }

    // 1. Get a mutable handle to the Wasm module's linear memory.
    let memory = match caller.get_export("mem") {
        Some(Extern::Memory(mem)) => mem,
        _ => return Err(anyhow::anyhow!("failed to find host memory")),
    };

    // 2. Get a mutable slice to the entire memory.
    let memory_slice = memory.data_mut(caller);

    // 3. Perform a critical bounds check.
    // Ensure the requested write operation is within the bounds of the memory slice.
    let end_index = (output_pointer as usize)
        .checked_add(output_len as usize)
        .ok_or_else(|| anyhow::anyhow!("Output pointer and length overflow"))?;

    if end_index > memory_slice.len() {
        return Err(anyhow::anyhow!("Write operation is out of memory bounds."));
    }

    // 4. Create a mutable sub-slice to the destination in Wasm memory.
    let dest_slice = &mut memory_slice[output_pointer as usize..end_index];

    // 5. Copy the data from the host's `Vec` into the Wasm memory.
    dest_slice.copy_from_slice(&data);

    Ok(())
}
pub fn compute_sha512_half(
    mut caller: Caller<'_, ()>,
    in_buf_ptr: u32,
    in_buf_len: u32,
    out_buf_ptr: u32,
    out_buf_cap: u32,
) -> i32 {
    if 32 > out_buf_cap {
        return -1i32;
    }
    if in_buf_len > 1024 {
        return -2i32;
    }
    let custom_cost = 2_000;
    caller.set_fuel(caller.get_fuel().unwrap() - custom_cost).expect("TODO: panic message");

    let data = get_data(&mut caller, in_buf_ptr, in_buf_len);
    let hash_half = sha512_half(&data);
    set_data(&mut caller, hash_half, out_buf_ptr, out_buf_cap).expect("TODO: panic message");
    32i32
}

fn main() -> Result<()> {
    // 1. Set up the Wasmtime environment with fuel enabled
    // let wasm_file = "/home/pwang/wasm/rx-wasm-prototype/wat/test.wasm";
    let wasm_file = "/Users/pwang/wasm/rx-wasm-prototype/wat/test.wasm";
    debug!("Loading WASM module from file: {}", wasm_file);
    let wasm_path = PathBuf::from(wasm_file);

    let mut config = Config::new();
    config.consume_fuel(true);
    // let _ = config.target("pulley64").unwrap();
    // config.strategy(Strategy::Cranelift);
    config.cranelift_opt_level(OptLevel::Speed);

    let start = Instant::now();

    let engine = Engine::new(&config)?;
    // The Store holds our fuel counter
    let mut store = Store::new(&engine, ());
    let mut linker = Linker::new(&engine);
    linker.func_wrap("host_lib", "compute_sha512_half",
                     compute_sha512_half)?;

    // 2. Set the initial fuel limit for this execution
    // You can set this to any large number. 1_000_000_000 is a good starting point.
    store.set_fuel(1_000_000_000_000)?;

    // 3. Load the module and create an instance
    let module = Module::from_file(store.engine(), &wasm_path)?;
    // let instance = Instance::new(&mut store, &module, &[])?;
    let instance = linker.instantiate(&mut store, &module)?;

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