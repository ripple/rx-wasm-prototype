pub use helper_lib::{allocate, deallocate};

#[no_mangle]
pub extern fn add_bytes(pointer: *mut u8, capacity: usize) -> i32 {
    unsafe {
        let mut r = 0 ;
        println!("add_bytes {:?}", pointer);
        let v = Vec::from_raw_parts(pointer, capacity, capacity);
        println!("add_bytes {:?}", v);
        for i in v{
            r += i as i32;
        }
        r
    }
}
