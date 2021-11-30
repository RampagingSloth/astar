
//! Autogenerated weights for `pallet_dapps_staking`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2021-11-30, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 128

// Executed Command:
// ./target/release/astar-collator
// benchmark
// --chain
// dev
// --execution
// wasm
// --wasm-execution
// compiled
// --pallet
// pallet_dapps_staking
// --steps
// 20
// --repeat
// 10
// --extrinsic
// *
// --output
// .


#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `pallet_dapps_staking`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_dapps_staking::WeightInfo for WeightInfo<T> {
	// Storage: DappsStaking RegisteredDevelopers (r:1 w:1)
	// Storage: DappsStaking RegisteredDapps (r:1 w:1)
	// Storage: DappsStaking PreApprovalIsEnabled (r:1 w:0)
	fn register() -> Weight {
		(38_393_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: DappsStaking RegisteredDevelopers (r:1 w:1)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking ContractEraStake (r:3 w:1)
	// Storage: DappsStaking EraRewardsAndStakes (r:1 w:1)
	// Storage: System Account (r:1 w:1)
	// Storage: DappsStaking Ledger (r:25 w:25)
	// Storage: Balances Locks (r:25 w:25)
	fn unregister(n: u32, ) -> Weight {
		(0 as Weight)
			// Standard Error: 102_000
			.saturating_add((28_764_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().reads((3 as Weight).saturating_mul(n as Weight)))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
			.saturating_add(T::DbWeight::get().writes((3 as Weight).saturating_mul(n as Weight)))
	}
	// Storage: DappsStaking PreApprovalIsEnabled (r:0 w:1)
	fn enable_developer_pre_approval() -> Weight {
		(2_064_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: DappsStaking PreApprovedDevelopers (r:1 w:1)
	fn developer_pre_approval() -> Weight {
		(6_282_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(1 as Weight))
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking RegisteredDevelopers (r:1 w:0)
	// Storage: DappsStaking Ledger (r:1 w:1)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking ContractEraStake (r:1 w:1)
	// Storage: DappsStaking EraRewardsAndStakes (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	fn bond_and_stake() -> Weight {
		(223_367_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking RegisteredDevelopers (r:1 w:0)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking ContractEraStake (r:1 w:1)
	// Storage: DappsStaking Ledger (r:1 w:1)
	// Storage: Balances Locks (r:1 w:1)
	// Storage: DappsStaking EraRewardsAndStakes (r:1 w:1)
	fn unbond_and_unstake() -> Weight {
		(222_895_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(7 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	// Storage: DappsStaking Ledger (r:1 w:1)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: Balances Locks (r:1 w:1)
	fn withdraw_unbonded() -> Weight {
		(81_987_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: DappsStaking RegisteredDapps (r:1 w:0)
	// Storage: DappsStaking CurrentEra (r:1 w:0)
	// Storage: DappsStaking ContractEraStake (r:1 w:1)
	// Storage: DappsStaking EraRewardsAndStakes (r:1 w:0)
	// Storage: Balances TotalIssuance (r:1 w:1)
	fn claim(n: u32, ) -> Weight {
		(71_770_000 as Weight)
			// Standard Error: 20_000
			.saturating_add((5_254_000 as Weight).saturating_mul(n as Weight))
			.saturating_add(T::DbWeight::get().reads(5 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	// Storage: DappsStaking ForceEra (r:0 w:1)
	fn force_new_era() -> Weight {
		(1_824_000 as Weight)
			.saturating_add(T::DbWeight::get().writes(1 as Weight))
	}
}
