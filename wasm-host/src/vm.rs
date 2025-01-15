use wasmedge_sdk::{params, Vm, WasmVal, WasmEdgeResult, AsInstance};

pub fn run_func<T: wasmedge_sdk::vm::SyncInst>(
    vm : &mut Vm<T>,
    func_name: impl AsRef<str>,
    data: Vec<u8>,
) -> WasmEdgeResult<bool> {
    let alloc_size = data.len() as i32;
    let pointer = match vm.run_func(None, "allocate", params!(alloc_size)) {
        Ok(res) => res[0].to_i32(),
        Err(err) => {
            return Err(err);
        }
    };
    println!("host run_func {} {}", pointer, alloc_size);

    let mut memory = vm.active_module_mut().unwrap().get_memory_mut("memory")?;
    memory.set_data(data, pointer as u32).unwrap();

    let rets = vm.run_func(None, func_name, params!(pointer, alloc_size))?;
    Ok(rets[0].to_i32() == 1)
}
