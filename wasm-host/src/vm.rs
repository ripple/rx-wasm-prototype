use std::collections::HashMap;
use wasmedge_sdk::{params, Vm, WasmEdgeResult, AsInstance, Store, Module, WasmVal};
use wasmedge_sdk::vm::SyncInst;
use wasmedge_sdk::wasi::WasiModule;

pub fn run_func_data(
    wasm_lib_file: &str,
    func_name: impl AsRef<str>,
    data: Vec<u8>,
) -> WasmEdgeResult<i32> {
    let mut wasi_module = WasiModule::create(None, None, None).unwrap();
    let mut instances: HashMap<String, &mut dyn SyncInst> = HashMap::new();
    instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());

    let mut vm = Vm::new(Store::new(None, instances)?);
    let wasm_module = Module::from_file(None, &wasm_lib_file)?;
    vm.register_module(None, wasm_module.clone())?;

    let alloc_size = data.len() as i32;
    let pointer = match vm.run_func(None, "allocate", params!(alloc_size)) {
        Ok(res) => res[0].to_i32(),
        Err(err) => {
            return Err(err);
        }
    };
    println!("host pointer {} {}", pointer, alloc_size);
    let mut memory = vm.active_module_mut().unwrap().get_memory_mut("memory")?;
    memory.set_data(data, pointer as u32)?;
    {
        let rvec = memory.get_data(pointer as u32, alloc_size as u32)?;
        println!("host get_data {:?}", rvec);
    }

    let rets = vm.run_func(None, func_name, params!(pointer, alloc_size))?;
    Ok(rets[0].to_i32())
}

pub fn run_func_int(
    wasm_lib_file: &str,
    func_name: impl AsRef<str>,
    input: i32,
) -> WasmEdgeResult<i32> {
    let mut wasi_module = WasiModule::create(None, None, None).unwrap();
    let mut instances: HashMap<String, &mut dyn SyncInst> = HashMap::new();
    instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());

    let mut vm = Vm::new(Store::new(None, instances)?);
    let wasm_module = Module::from_file(None, &wasm_lib_file)?;
    vm.register_module(None, wasm_module.clone())?;

    let res = match vm.run_func(None, func_name, params!(input)) {
        Ok(res) => res[0].to_i32(),
        Err(err) => {
            return Err(err);
        }
    };
    Ok(res)
}
