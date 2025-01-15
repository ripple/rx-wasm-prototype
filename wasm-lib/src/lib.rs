use std::mem;
// use std::os::raw::c_void;

#[no_mangle]
pub extern fn allocate(size: usize) -> *mut u8 {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);
    // println!("allocate {:?}", pointer);
    pointer
        //as *mut c_void
}
//
// #[no_mangle]
// pub extern fn deallocate(pointer: *mut c_void, capacity: usize) {
//     unsafe {
//         // println!("deallocate {:?}", pointer);
//         let _ = Vec::from_raw_parts(pointer, 0, capacity);
//     }
// }

#[no_mangle]
pub extern fn check_accountID(pointer: *mut u8, capacity: usize) -> bool {
    let expected: Vec<u8> = Vec::from("rHb9CJAWyB4rj91VRWn96DkukG4bwdtyTh");
    unsafe {
        // println!("test {:?}", pointer);
        let v = Vec::from_raw_parts(pointer, capacity, capacity);
        // println!("test {:?}", v);
        // println!("test {:?}", v == expected);
        v == expected
    }
}
