use candid::{candid_method, export_service, CandidType, Nat, Principal};
use std::collections::HashMap;
use ic_cdk::caller;
use crate::utils;
// use crate::utils::zero;

pub type DepositReceipt = Result<Nat, DepositErr>;

pub type OrderId = u32;

#[derive(Default)]
pub struct Balances(pub HashMap<Principal, HashMap<Principal, Nat>>);

// impl Balances {
//     pub fn add_balance(&mut self, owner: &Principal, token_canister_id: &Principal, delta: Nat) {
//         let balances = self.0.entry(*owner).or_insert_with(HashMap::new);
//
//         if let Some(x) = balances.get_mut(token_canister_id) {
//             *x += delta;
//         } else {
//             balances.insert(*token_canister_id, delta);
//         }
//     }
//
//     // Tries to substract balance from user account. Checks for overflows
//     pub fn subtract_balance(
//         &mut self,
//         owner: &Principal,
//         token_canister_id: &Principal,
//         delta: Nat,
//     ) -> bool {
//         if let Some(balances) = self.0.get_mut(owner) {
//             if let Some(x) = balances.get_mut(token_canister_id) {
//                 if *x >= delta {
//                     *x -= delta;
//                     // no need to keep an empty token record
//                     if *x == zero() {
//                         balances.remove(token_canister_id);
//                     }
//                     return true;
//                 } else {
//                     return false;
//                 }
//             }
//         }
//
//         false
//     }
// }

// owner -> token_canister_id -> amount
type Orders = HashMap<OrderId, Order>;

#[allow(non_snake_case)]
#[derive(CandidType, Clone)]
pub struct Order {
    pub id: OrderId,
    pub owner: Principal,
    pub from: Principal,
    pub fromAmount: Nat,
    pub to: Principal,
    pub toAmount: Nat,
}

#[derive(Default)]
pub struct State {
    pub(crate) owner: Option<Principal>,
    pub(crate) ledger: Option<Principal>,
    pub(crate) exchange: Exchange,
}

#[derive(CandidType)]
pub struct Balance {
    pub owner: Principal,
    pub token: Principal,
    pub amount: Nat,
}

#[derive(Default)]
pub struct Exchange {
    pub next_id: OrderId,
    pub balances: Balances,
    pub orders: Orders,
}

impl Exchange {
    pub fn get_balance(&self, token_canister_id: Principal) -> Nat {
        self.balances
            .0
            .get(&caller())
            .and_then(|v| v.get(&token_canister_id))
            .map_or(utils::zero(), |v| v.to_owned())
    }
    pub fn get_balances(&self) -> Vec<Balance> {
        match self.balances.0.get(&caller()) {
            None => Vec::new(),
            Some(v) => v
                .iter()
                .map(|(token_canister_id, amount)| Balance {
                    owner: caller(),
                    token: *token_canister_id,
                    amount: amount.to_owned(),
                })
                .collect(),
        }
    }
}

#[derive(CandidType)]
pub enum DepositErr {
    BalanceLow,
    TransferFailure,
}

pub const ICP_FEE: u64 = 10_000;
