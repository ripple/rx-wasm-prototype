// pub mod alloc;

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

// use std::alloc::{alloc, Layout};
use std::mem;
// use std::os::raw::c_char;
use std::os::raw::c_void;

#[no_mangle]
pub extern fn allocate(size: usize) -> *mut c_void {
    // unsafe {
    // let mut pointer = alloc(Layout::from_size_align(size, 1).unwrap());
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);
    println!("allocate {:?}", pointer);
    pointer as *mut c_void
    // }
}

#[no_mangle]
pub extern fn deallocate(pointer: *mut c_void, capacity: usize) {
    unsafe {
        println!("deallocate {:?}", pointer);
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}

#[no_mangle]
pub extern fn test(pointer: *mut u8, capacity: usize) -> *mut c_void {
    unsafe {
        println!("test {:?}", pointer);
        let v = Vec::from_raw_parts(pointer, capacity, capacity);//TODO no need to deallocate??
        println!("test {:?}", v);
    }
    let mut buffer: Vec<u8> = Vec::with_capacity(9);
    buffer.push(7);
    {
        let mut data: Vec<u8> = Vec::with_capacity(6);
        data.append(&mut vec![1, 2, 3, 4, 5, 6]);
        let pointer = data.as_ptr();
        println!("test {:?}", pointer);
        let mut pointer_u32 = (pointer as usize).to_le_bytes().to_vec();
        buffer.append(&mut pointer_u32);
        let len = data.len();
        let mut len_u32 = len.to_le_bytes().to_vec();
        buffer.append(&mut len_u32);
        println!("test {} {:?} {}", buffer.len(), pointer, len);
        mem::forget(data);
    }
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);
    println!("{:?}", pointer);
    pointer as *mut c_void
}
