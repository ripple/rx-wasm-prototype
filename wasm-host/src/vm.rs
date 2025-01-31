use std::collections::HashMap;
use wasmedge_sdk::{Vm, WasmEdgeResult, AsInstance, Store, Module, Instance, CallingFrame,
                   WasmValue, ImportObjectBuilder};
use wasmedge_sdk::error::CoreError;
use wasmedge_sdk::vm::SyncInst;
use wasmedge_sdk::wasi::WasiModule;

#[derive(Clone, Debug)]
struct LedgerData {
    sqn: i32,
}

fn constInt(
    data: &mut LedgerData,
    _inst: &mut Instance,
    _caller: &mut CallingFrame,
    _input: Vec<WasmValue>,
) -> Result<Vec<WasmValue>, CoreError> {
    Ok(vec![WasmValue::from_i32(data.sqn)])
}

pub fn run_func(
    wasm_lib_file: &str,
    func_name: impl AsRef<str>,
) -> WasmEdgeResult<()> {
    let mut wasi_module = WasiModule::create(None, None, None)?;

    let ledger = LedgerData { sqn: 5 };

    let mut import_builder = ImportObjectBuilder::new("host_lib", ledger)?;
    import_builder
        .with_func::<(), i32>("constInt", constInt)
        .unwrap();
    let mut import_object = import_builder.build();

    let mut instances: HashMap<String, &mut dyn SyncInst> = HashMap::new();
    instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());
    instances.insert(import_object.name().unwrap(), &mut import_object);

    let mut vm = Vm::new(Store::new(None, instances)?);
    let wasm_module = Module::from_file(None, &wasm_lib_file)?;
    vm.register_module(None, wasm_module.clone())?;

    let args = vec![WasmValue::from_i32(10)];
    let _rets = vm.run_func(None, func_name, args)?;
    Ok(())
}
