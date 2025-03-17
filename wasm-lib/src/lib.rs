pub use helper_lib::{allocate, deallocate};
use helper_lib::*;

#[no_mangle]
pub extern fn ready() -> bool {
    unsafe {
        let sender = get_tx_account_id();
        let owner = get_current_escrow_account_id();
        let dest = get_current_escrow_destination();
        let dest_balance = get_account_balance(&dest);
        let escrow_data = get_current_escrow_data();
        let ed_str = String::from_utf8(escrow_data.clone()).unwrap();
        let threshold_balance = ed_str.parse::<u64>().unwrap();
        let pl_time = host_lib::getParentLedgerTime();
        let e_time = get_current_escrow_finish_after();

        print_data(&sender);
        print_data(&owner);
        print_data(&dest);
        print_data(&escrow_data);

        print_number(&dest_balance);
        print_number(&pl_time);
        print_number(&e_time);

        sender == owner && dest_balance <= threshold_balance && pl_time >= e_time
        // true
    }
}
