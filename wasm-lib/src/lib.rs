#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
    unsafe { math_lib::real_add(a, b) }
}

pub mod math_lib {
    #[link(wasm_import_module = "mock_math_lib")]
    extern "C" {
        pub fn real_add(a: i32, b: i32) -> i32;
    }
}
