mod types;

use std::cell::RefCell;
use std::env;
use std::fs::{create_dir_all, write};
use std::path::{Path, PathBuf};
use candid::{Nat, Principal};
use ic_cdk::caller;

use ic_cdk::export::candid::{
    candid_method, CandidType, check_prog, Deserialize, export_service, IDLProg, TypeEnv,
};
use ic_ledger_types::{
    AccountIdentifier, Memo, Tokens, DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID,
};


use ic_cdk_macros::*;
use crate::types::*;

thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}

#[update]
#[candid_method(update)]
pub async fn deposit(token_canister_id: Principal) -> DepositReceipt {
    // let caller = caller();
    // let ledger_canister_id = STATE
    //     .with(|s| s.borrow().ledger)
    //     .unwrap_or(MAINNET_LEDGER_CANISTER_ID);
    //
    // let amount = if token_canister_id == ledger_canister_id {
    //     deposit_icp(caller).await?
    // } else {
    //     deposit_token(caller, token_canister_id).await?
    // };
    // STATE.with(|s| {
    //     s.borrow_mut()
    //         .exchange
    //         .balances
    //         .add_balance(&caller, &token_canister_id, amount.to_owned())
    // });
    DepositReceipt::Ok(10 as u64)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn save_candid() {
        #[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
        fn export_candid() -> String {
            export_service!();
            __export_service()
        }

        let current_dir = env::current_dir().expect("Failed to get current directory");
        let folder_name = current_dir
            .file_name()
            .expect("Failed to get folder name")
            .to_str()
            .expect("Failed to convert folder name to string");

        let dir = PathBuf::from(&current_dir);
        match create_dir_all(&dir) {
            Ok(_) => println!("Successfully created directory"),
            Err(e) => println!("Failed to create directory: {}", e),
        }
        let file = format!("{}.did", folder_name);
        let res = write(dir.join(file), export_candid());
        println!("-------- Wrote to {:?}", dir);
        println!("-------- res {:?}", res);
    }
}