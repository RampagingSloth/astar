
// This file is part of Astar.

// Copyright (C) 2019-2023 Stake Technologies Pte.Ltd.
// SPDX-License-Identifier: GPL-3.0-or-later

// Astar is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Astar is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Astar. If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for pallet_unified_accounts
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-09-26, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `devserver-01`, CPU: `Intel(R) Xeon(R) E-2236 CPU @ 3.40GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("shibuya-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/astar-collator
// benchmark
// pallet
// --chain=shibuya-dev
// --steps=50
// --repeat=20
// --pallet=pallet_unified_accounts
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./benchmark-results/unified_accounts_weights.rs
// --template=./scripts/templates/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_unified_accounts.
pub trait WeightInfo {
	fn claim_evm_address() -> Weight;
	fn claim_default_evm_address() -> Weight;
	fn uam_to_account_id() -> Weight;
	fn uam_to_account_id_or_default() -> Weight;
	fn uam_to_h160() -> Weight;
	fn uam_to_h160_or_default() -> Weight;
}

/// Weights for pallet_unified_accounts using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: UnifiedAccounts EvmToNative (r:1 w:1)
	/// Proof: UnifiedAccounts EvmToNative (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: UnifiedAccounts NativeToEvm (r:1 w:1)
	/// Proof: UnifiedAccounts NativeToEvm (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: EVMChainId ChainId (r:1 w:0)
	/// Proof: EVMChainId ChainId (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System BlockHash (r:1 w:0)
	/// Proof: System BlockHash (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_evm_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `256`
		//  Estimated: `3593`
		// Minimum execution time: 64_843_000 picoseconds.
		Weight::from_parts(65_508_000, 3593)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: UnifiedAccounts EvmToNative (r:1 w:1)
	/// Proof: UnifiedAccounts EvmToNative (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: UnifiedAccounts NativeToEvm (r:1 w:1)
	/// Proof: UnifiedAccounts NativeToEvm (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn claim_default_evm_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3533`
		// Minimum execution time: 16_399_000 picoseconds.
		Weight::from_parts(16_806_000, 3533)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: UnifiedAccounts EvmToNative (r:1 w:0)
	/// Proof: UnifiedAccounts EvmToNative (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn uam_to_account_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `3533`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(7_000_000, 3533)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: UnifiedAccounts EvmToNative (r:1 w:0)
	/// Proof: UnifiedAccounts EvmToNative (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn uam_to_account_id_or_default() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `3533`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(7_000_000, 3533)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: UnifiedAccounts NativeToEvm (r:1 w:0)
	/// Proof: UnifiedAccounts NativeToEvm (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn uam_to_h160() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `3533`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 3533)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	/// Storage: UnifiedAccounts NativeToEvm (r:1 w:0)
	/// Proof: UnifiedAccounts NativeToEvm (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn uam_to_h160_or_default() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `3533`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 3533)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: UnifiedAccounts EvmToNative (r:1 w:1)
	/// Proof: UnifiedAccounts EvmToNative (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: UnifiedAccounts NativeToEvm (r:1 w:1)
	/// Proof: UnifiedAccounts NativeToEvm (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: EVMChainId ChainId (r:1 w:0)
	/// Proof: EVMChainId ChainId (max_values: Some(1), max_size: Some(8), added: 503, mode: MaxEncodedLen)
	/// Storage: System BlockHash (r:1 w:0)
	/// Proof: System BlockHash (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// Storage: System Account (r:1 w:0)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn claim_evm_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `256`
		//  Estimated: `3593`
		// Minimum execution time: 64_843_000 picoseconds.
		Weight::from_parts(65_508_000, 3593)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: UnifiedAccounts EvmToNative (r:1 w:1)
	/// Proof: UnifiedAccounts EvmToNative (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	/// Storage: UnifiedAccounts NativeToEvm (r:1 w:1)
	/// Proof: UnifiedAccounts NativeToEvm (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn claim_default_evm_address() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3533`
		// Minimum execution time: 16_399_000 picoseconds.
		Weight::from_parts(16_806_000, 3533)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: UnifiedAccounts EvmToNative (r:1 w:0)
	/// Proof: UnifiedAccounts EvmToNative (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn uam_to_account_id() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `3533`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(7_000_000, 3533)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: UnifiedAccounts EvmToNative (r:1 w:0)
	/// Proof: UnifiedAccounts EvmToNative (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn uam_to_account_id_or_default() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `3533`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(7_000_000, 3533)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: UnifiedAccounts NativeToEvm (r:1 w:0)
	/// Proof: UnifiedAccounts NativeToEvm (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn uam_to_h160() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `3533`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 3533)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	/// Storage: UnifiedAccounts NativeToEvm (r:1 w:0)
	/// Proof: UnifiedAccounts NativeToEvm (max_values: None, max_size: Some(68), added: 2543, mode: MaxEncodedLen)
	fn uam_to_h160_or_default() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `170`
		//  Estimated: `3533`
		// Minimum execution time: 6_000_000 picoseconds.
		Weight::from_parts(6_000_000, 3533)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
}
