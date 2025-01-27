use std::collections::HashMap;
use wasmedge_sdk::{params, Vm, WasmEdgeResult, AsInstance, Store, Module, Instance, CallingFrame,
                   WasmValue, ImportObjectBuilder};
use wasmedge_sdk::error::CoreError;
use wasmedge_sdk::vm::SyncInst;
use wasmedge_sdk::wasi::WasiModule;

#[derive(Clone, Debug)]
struct LedgerData {
    sqn: i32,
}

fn get_ledger_sqn(
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
) -> WasmEdgeResult<bool> {
    let mut wasi_module = WasiModule::create(None, None, None).unwrap();

    let ledger = LedgerData { sqn: 5 };

    let mut import_builder = ImportObjectBuilder::new("host_lib", ledger)?;
    import_builder
        .with_func::<(), i32>("get_ledger_sqn", get_ledger_sqn)
        .unwrap();
    let mut import_object = import_builder.build();

    // let mut instances = HashMap::new();
    let mut instances: HashMap<String, &mut dyn SyncInst> = HashMap::new();
    instances.insert(wasi_module.name().to_string(), wasi_module.as_mut());
    instances.insert(import_object.name().unwrap(), &mut import_object);

    let mut vm = Vm::new(Store::new(None, instances).unwrap());
    let wasm_module = Module::from_file(None, &wasm_lib_file).unwrap();
    vm.register_module(None, wasm_module.clone()).unwrap();

    let rets = vm.run_func(None, func_name, params!())?;
    Ok(rets[0].to_i32() == 1)
}
