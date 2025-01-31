#[no_mangle]
pub fn looping(to: i32) {
    let mut count = 0;
    loop {
        count += 1;
        if count == to {
            break;
        }
    }
}

#[no_mangle]
pub fn looping_host(to: i32) {
    let mut count = 0;
    loop {
        unsafe { let _ = host_lib::constInt(); }
        count += 1;
        if count == to {
            break;
        }
    }
}

pub mod host_lib {
    #[link(wasm_import_module = "host_lib")]
    extern "C" {
        pub fn constInt() -> i32;
    }
}