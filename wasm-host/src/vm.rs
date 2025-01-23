use wasmedge_sdk::{params, Vm, WasmVal, WasmEdgeResult, AsInstance};

pub fn run_func<T: wasmedge_sdk::vm::SyncInst>(
    vm : &mut Vm<T>,
    func_name: impl AsRef<str>,
    tx_json: Vec<u8>,
    lo_json: Vec<u8>,
) -> WasmEdgeResult<(bool, String)> {

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

    {
        let mut memory = vm.active_module_mut().unwrap().get_memory_mut("memory")?;
        memory.set_data(tx_json, tx_pointer as u32).unwrap();
        memory.set_data(lo_json, lo_pointer as u32).unwrap();
    }

    let rets = vm.run_func(None, func_name, params!(tx_pointer, tx_size, lo_pointer, lo_size))?;

    {
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
        let fn_re = String::from_utf8(rvec).unwrap();
        println!("host run_func {:?}", fn_re);

        vm.run_func(None, "deallocate", params!(outter, 9))?;
        vm.run_func(None, "deallocate", params!(ret_pointer, ret_len))?;
        Ok((flag == 1, fn_re))
    }
}
