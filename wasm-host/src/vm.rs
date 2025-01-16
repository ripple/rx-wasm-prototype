use wasmedge_sdk::{params, Vm, WasmVal, WasmEdgeResult, AsInstance};

pub fn run_func<T: wasmedge_sdk::vm::SyncInst>(
    vm : &mut Vm<T>,
    func_name: impl AsRef<str>,
    tx_json: Vec<u8>,
    lo_json: Vec<u8>,
) -> WasmEdgeResult<bool> {

    let tx_size = tx_json.len() as i32;
    let tx_pointer = match vm.run_func(None, "allocate", params!(tx_size)) {
        Ok(res) => res[0].to_i32(),
        Err(err) => {
            return Err(err);
        }
    };
    println!("host tx alloc {} {}", tx_pointer, tx_size);

    let lo_size = lo_json.len() as i32;
    let lo_pointer = match vm.run_func(None, "allocate", params!(lo_size)) {
        Ok(res) => res[0].to_i32(),
        Err(err) => {
            return Err(err);
        }
    };
    println!("host lo alloc {} {}", lo_pointer, lo_size);

    let mut memory = vm.active_module_mut().unwrap().get_memory_mut("memory")?;
    memory.set_data(tx_json, tx_pointer as u32).unwrap();
    memory.set_data(lo_json, lo_pointer as u32).unwrap();

    let rets = vm.run_func(None, func_name, params!(tx_pointer, tx_size, lo_pointer, lo_size))?;
    Ok(rets[0].to_i32() == 1)
}
