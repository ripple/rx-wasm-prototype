use std::mem;

#[no_mangle]
pub extern fn allocate(size: usize) -> *mut u8 {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);
    println!("allocate {:?}", pointer);
    pointer
}

#[no_mangle]
pub extern fn deallocate(pointer: *mut u8, capacity: usize) {
    unsafe {
        println!("deallocate {:?}", pointer);
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}
