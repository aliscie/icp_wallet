use std::cell::RefCell;
use std::env;
use std::fmt::format;
use std::fs::{create_dir_all, write};
use std::path::{Path, PathBuf};

use candid::{candid_method, export_service, Nat, Principal};
use ic_cdk::api::{call::call, caller};
use ic_cdk::export::candid::{CandidType, check_prog, Deserialize, IDLProg, TypeEnv};
use ic_cdk_macros::*;
use ic_ledger_types::{account_balance, AccountBalanceArgs, AccountIdentifier, BlockIndex, DEFAULT_FEE, DEFAULT_SUBACCOUNT, MAINNET_LEDGER_CANISTER_ID, Memo, Tokens, TransferArgs};
use num_bigint::BigUint;

use crate::types::*;
use crate::utils::principal_to_subaccount;

// use crate::utils::principal_to_subaccount;

mod types;
mod utils;


async fn transfer_to_caller(to: AccountIdentifier, amount: Tokens) -> BlockIndex {
    ic_ledger_types::transfer(
        MAINNET_LEDGER_CANISTER_ID,
        TransferArgs {
            memo: Memo(0),
            amount,
            fee: DEFAULT_FEE,
            from_subaccount: Some(DEFAULT_SUBACCOUNT), // can this automatically recognize the current user?
            to,
            created_at_time: None,
        },
    )
        .await
        .expect("call to ledger failed")
        .expect("transfer failed")
}

#[update]
#[candid_method(update)]
pub async fn withdraw(_to: String, _amount: u64) -> BlockIndex {
    let to = Principal::from_text(_to).unwrap();
    let amount = Tokens::from_e8s(_amount);
    let to: AccountIdentifier = AccountIdentifier::new(&to, &DEFAULT_SUBACCOUNT);
    transfer_to_caller(to, amount).await
}

#[query]
#[candid_method(query)]
pub async fn test_connection() -> bool { true }


#[update(name = "getDepositAddress")]
#[candid_method(update, rename = "getDepositAddress")]
pub fn get_deposit_address() -> String {
    let user = &caller();
    if format!("{}", user).as_str() == "2vxsx-fae" {
        return "Antonyms user.".to_string();
    }
    // to make a deposit you can get your address then transfer to it from another wallet.
    let canister_id = ic_cdk::api::id();
    let subaccount = principal_to_subaccount(user);
    format!("{}", AccountIdentifier::new(&canister_id, &subaccount)).as_str().to_string()
}


// #[init]
// fn init(ledger: Option<Principal>) {
//     ic_cdk::setup();
//     STATE.with(|s| {
//         s.borrow_mut().owner = Some(caller());
//         s.borrow_mut().ledger = ledger;
//     });
// }


thread_local! {
    static STATE: RefCell<State> = RefCell::new(State::default());
}




#[query(name = "getBalances")]
#[candid_method(query, rename = "getBalances")]
pub fn get_balances() -> Vec<Balance> {
    STATE.with(|s| s.borrow().exchange.get_balances())
}


#[cfg(test)]
mod tests {
    use ic_cdk::export::candid::{
        candid_method, CandidType, check_prog, Deserialize, export_service, IDLProg, TypeEnv,
    };

    use super::*;

    #[test]
    fn save_candid_2() {
        #[ic_cdk_macros::query(name = "__get_candid_interface_tmp_hack")]
        fn export_candid() -> String {
            export_service!();
            __export_service()
        }

        let dir = PathBuf::from("/Users/ahmed/Desktop/ic_wallet/src/ic_wallet_backend");
        match create_dir_all(&dir) {
            Ok(_) => println!("Successfully created directory"),
            Err(e) => println!("Failed to create directory: {}", e),
        }

        let res = write(dir.join("ic_wallet_backend.did"), export_candid());
        println!("-------- Wrote to {:?}", dir);
        println!("-------- res {:?}", res);
    }
}
