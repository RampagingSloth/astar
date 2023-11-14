#!/bin/sh

targets=("protocol_state" "account_ledger" "dapp_info" "period_info" "era_info" \
        "stake_amount" "singular_staking_info" "contract_stake_info" "era_reward_span" \
        "period_end_info" "era_stake_pair_iter")

for target in "${targets[@]}"
do
  cargo tarpaulin -p pallet-dapp-staking-v3 -o=html --output-dir=./coverage/$target -- $target
done