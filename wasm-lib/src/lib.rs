// pub use xrpl_std::{allocate, deallocate};
// use xrpl_std::get_tx_account_id;
// use xrpl_std::{get_account_balance, get_current_escrow_account_id, get_current_escrow_data, get_current_escrow_destination, get_current_escrow_finish_after, get_tx_account_id, host_lib, print_data, print_number};

const RECUR: i32 = 100_000;
// #[no_mangle]
// fn recur(r: i32) {
//     if r > 0 {
//         recur(r - 1);
//     }
//     println!("{}",r);
// }

#[no_mangle]
fn recur(r: i32, count: &mut i32) {
    if r > 0 {
        *count += 1;
        recur(r - 1, count);
    }
    println!("{}",r);
}

#[no_mangle]
pub extern fn recursive() -> i32 {
    let input: i32 = RECUR;
    let mut count = 0;
    recur(input, &mut count);
    count
}

//
// #[no_mangle]
// pub extern fn ready() -> bool {
//     // unsafe {
//     //     let sender = get_tx_account_id();
//     // let owner = get_current_escrow_account_id();
//     // let dest = get_current_escrow_destination();
//     // let dest_balance = get_account_balance(&dest);
//     // let escrow_data = get_current_escrow_data();
//     // let ed_str = String::from_utf8(escrow_data.clone()).unwrap();
//     // let threshold_balance = ed_str.parse::<u64>().unwrap();
//     // let pl_time = host_lib::getParentLedgerTime();
//     // let e_time = get_current_escrow_finish_after();
//     //
//     // print_data(&sender);
//     // print_data(&owner);
//     // print_data(&dest);
//     // print_data(&escrow_data);
//     //
//     // print_number(&dest_balance);
//     // print_number(&pl_time);
//     // print_number(&e_time);
//     //
//     // sender == owner && dest_balance <= threshold_balance && pl_time >= e_time
//     true
//     // }
// }
