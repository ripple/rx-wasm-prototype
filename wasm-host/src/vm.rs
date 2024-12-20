use wasmedge_sdk::{params, Vm, WasmVal, WasmEdgeResult, AsInstance, Module};

//TODO deallocate on early return
pub fn run_func<T: wasmedge_sdk::vm::SyncInst>(
    vm : &mut Vm<T>,
    // mod_name: Option<&str>,
    func_name: impl AsRef<str>,
    data: Vec<u8>,
    // module: Module,
) -> WasmEdgeResult<Vec<u8>> {
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

    let rvec = memory.get_data(pointer as u32, alloc_size as u32)?;
    println!("host run_func {:?}", rvec);

    let rets = vm.run_func(None, func_name, params!(pointer, alloc_size))?;

    let mut memory = vm.active_module_mut().unwrap().get_memory_mut("memory")?;
    let outter = rets[0].to_i32();
    let rvec = memory.get_data(outter as u32, 9)?;
    //TODO self.free(None, params!(rets[0].to_i32(), 9))?;

    let flag = rvec[0];
    let ret_pointer = i32::from_le_bytes(rvec[1..5].try_into().unwrap());
    let ret_len = i32::from_le_bytes(rvec[5..9].try_into().unwrap());
    println!("host run_func {} {} {}", flag, ret_pointer, ret_len);

    let memory = vm.active_module_mut().unwrap().get_memory_mut("memory")?;
    let rvec = memory.get_data(ret_pointer as u32, ret_len as u32)?;
    println!("host run_func {:?}", rvec);

    vm.run_func(None, "deallocate", params!(pointer, alloc_size))?;
    vm.run_func(None, "deallocate", params!(outter, 9))?;
    vm.run_func(None, "deallocate", params!(ret_pointer, ret_len))?;
    //match flag {
    // Ok(rvec)

    Ok(rvec)
}
