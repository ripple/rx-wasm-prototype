use std::mem;
use serde_json::Value;

#[no_mangle]
pub extern fn allocate(size: usize) -> *mut u8 {
    let mut buffer = Vec::with_capacity(size);
    let pointer = buffer.as_mut_ptr();
    mem::forget(buffer);
    pointer
}

#[no_mangle]
pub extern fn compare_accountID(tx_json_ptr: *mut u8, tx_json_size: usize, lo_json_ptr: *mut u8, lo_json_size: usize) -> bool {
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
    tx_account == lo_account
}

//     let tx_json_value: Value = match serde_json::from_slice(tx_data.as_slice()) {
//         Ok(v) => v,
//         Err(_e) => {
//             println!("error parsing tx: {_e:?}");
//             return false;
//         }
//     };
//
//     let lo_json_value: Value = match serde_json::from_slice(lo_data.as_slice()) {
//         Ok(v) => v,
//         Err(_e) => {
//             println!("error parsing lo: {_e:?}");
//             return false;
//         }
//     };
//
//     println!("compare_accountID tx {:?}", tx_json_value);
//     println!("compare_accountID lo {:?}", lo_json_value);
//     let tx_account = match tx_json_value.get("Account") {
//         Some(v) => v,
//         None => {
//             println!("tx no account");
//             return false;
//         }
//     };
//
//     let lo_account = match lo_json_value.get("Account") {
//         Some(v) => v,
//         None => {
//             println!("lo no account");
//             return false;
//         }
//     };
//
//     tx_account == lo_account
// }
// true
// println!("compare_accountID tx {:?}", tx_data);
// println!("compare_accountID lo {:?}", lo_data);
// let tx_json_value: Value = serde_json::from_slice(tx_data.as_slice()).unwrap();
// let lo_json_value: Value = serde_json::from_slice(lo_data.as_slice()).unwrap();
// println!("compare_accountID tx {:?}", tx_json_value);
// println!("compare_accountID lo {:?}", lo_json_value);
// let tx_account = tx_json_value.get("Account").unwrap();
// let lo_account = lo_json_value.get("Account").unwrap();
// println!("compare_accountID tx {:?}", tx_account);
// println!("compare_accountID lo {:?}", lo_account);
// tx_account == lo_account
