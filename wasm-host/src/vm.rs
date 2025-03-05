use wasmedge_sdk::{Vm, WasmEdgeResult};

pub fn run_func<T: wasmedge_sdk::vm::SyncInst>(
    vm : &mut Vm<T>,
    func_name: impl AsRef<str>,
) -> WasmEdgeResult<bool> {
    let rets = vm.run_func(None, func_name, vec![])?;
    Ok(rets[0].to_i32() == 1)
}
