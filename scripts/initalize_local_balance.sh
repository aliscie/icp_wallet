#!/bin/bash

dfx identity use default
II_ACCOUNT_ID_HEX=$(python3 ./scripts/principal_to_default_account_id.py $1)
II_ACCOUNT_ID=$(python3 -c 'print("vec{" + ";".join([str(b) for b in bytes.fromhex("'$II_ACCOUNT_ID_HEX'")]) + "}")')
dfx canister call ledger transfer "(record { amount = record { e8s = 10000000 }; to = $II_ACCOUNT_ID; fee = record { e8s = 10000}; memo = 1;})"