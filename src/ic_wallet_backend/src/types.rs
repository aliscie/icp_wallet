use std::collections::HashMap;
use candid::{candid_method, export_service, Nat, CandidType, Principal};

#[derive(CandidType, Clone)]
pub struct DepositReceipt {
    amount: u64,
}

impl DepositReceipt {
    pub fn Ok(amount: u64) -> Self {
        DepositReceipt { amount }
    }
}

pub type OrderId = u32;

#[derive(Default)]
pub struct Balances(pub HashMap<Principal, HashMap<Principal, Nat>>);

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
    owner: Option<Principal>,
    pub(crate) ledger: Option<Principal>,
    pub(crate) exchange: Exchange,
}

#[derive(Default)]
pub struct Exchange {
    pub next_id: OrderId,
    pub balances: Balances,
    pub orders: Orders,
}