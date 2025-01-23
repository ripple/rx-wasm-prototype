use std::{mem, u32};
// use std::str::FromStr;
use serde_json::Value;

#[no_mangle]
pub extern fn allocate(size: usize) -> *mut u8 {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);
    pointer
}

#[no_mangle]
pub extern fn deallocate(pointer: *mut u8, capacity: usize) {
    unsafe {
        // println!("deallocate {:?}", pointer);
        let _ = Vec::from_raw_parts(pointer, 0, capacity);
    }
}

#[no_mangle]
pub extern fn compare_accountID(tx_json_ptr: *mut u8, tx_json_size: usize, lo_json_ptr: *mut u8, lo_json_size: usize)
                                -> *mut u8
{
    let tx_data;
    let lo_data;
    unsafe {
        tx_data = Vec::from_raw_parts(tx_json_ptr, tx_json_size, tx_json_size);
        lo_data = Vec::from_raw_parts(lo_json_ptr, lo_json_size, lo_json_size);
    }

    let tx_json_value: Value = serde_json::from_slice(tx_data.as_slice()).unwrap();
    let lo_json_value: Value = serde_json::from_slice(lo_data.as_slice()).unwrap();
    let tx_account = tx_json_value.get("Account").unwrap();
    let lo_account = lo_json_value.get("Account").unwrap();

    let data_field = lo_json_value.get("Data").unwrap();
    let data = data_field.as_str().unwrap();
    let mut counter = u32::from_str_radix(data, 10).unwrap();

    let good =
        if tx_account == lo_account {
            counter = counter - 1;
            if counter == 0 {
                1
            } else {
                0
            }
        } else {
            0
        };
    println!("test good = {:?}, counter = {}", good, counter);

    // pack data
    let mut buffer: Vec<u8> = Vec::with_capacity(9);
    buffer.push(good);
    {
        let data = u32::to_string(&counter);
        let pointer = data.as_ptr();
        let mut pointer_u32 = (pointer as u32).to_le_bytes().to_vec();
        let pointer_u32_len = pointer_u32.len();
        buffer.append(&mut pointer_u32);
        let len = data.len() as u32;
        let mut len_u32 = len.to_le_bytes().to_vec();
        buffer.append(&mut len_u32);
        println!("test data {}, pointer = {:?}, pointer_u32_len = {}, len = {}",
                 data, pointer as u32, pointer_u32_len, len);
        mem::forget(data);
    }
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);
    pointer
}

