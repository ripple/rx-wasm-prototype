#[no_mangle]
pub fn ready() -> bool {
    let a = unsafe { host_lib::getLedgerSqn() };
    println!("ready: {:?} {}", a.a, a.b);
    a.a >= 5
}
#[repr(C)]
pub struct TwoI32 {
    a: i32,
    b: i32,
}
pub mod host_lib {
    use crate::TwoI32;

    #[link(wasm_import_module = "host_lib")]
    extern "C" {
        pub fn getLedgerSqn() -> TwoI32;
    }
}