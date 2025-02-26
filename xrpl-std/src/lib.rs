use std::mem;
use std::collections::HashMap;

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

pub type AccountID = [u8; 20];
pub type Hash256 = [u8; 32];
pub type PublicKey = [u8; 33];

pub enum HostType{
    Number(i64),
    Str(String),
    AccID(AccountID),
    Hash(Hash256),
    PubKey(PublicKey),
}

pub type HostObject = HashMap<String, HostType>;

pub fn getTxAccount() -> Option<AccountID>{
    None
}
pub fn getTx() -> Option<HostObject>{
    None
}

pub fn getLedgerObjectAccount() -> Option<AccountID>{
    None
}
pub fn getLedgerObject() -> Option<HostObject>{
    None
}

pub fn getForeignLedgerObject(_key : Hash256) -> Option<HostObject>{
    None
}