
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

//! Autogenerated weights for oracle_benchmarks
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-03-18, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `gh-runner-01-ovh`, CPU: `Intel(R) Xeon(R) E-2236 CPU @ 3.40GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("shibuya-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/astar-collator
// benchmark
// pallet
// --chain=shibuya-dev
// --steps=50
// --repeat=20
// --pallet=oracle_benchmarks
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./benchmark-results/shibuya-dev/benchmarks_weights.rs
// --template=./scripts/templates/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;
use orml_oracle::WeightInfo;

/// Weights for oracle_benchmarks using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `OracleMembership::Members` (r:1 w:0)
	/// Proof: `OracleMembership::Members` (`max_values`: Some(1), `max_size`: Some(513), added: 1008, mode: `MaxEncodedLen`)
	/// Storage: `Oracle::HasDispatched` (r:1 w:1)
	/// Proof: `Oracle::HasDispatched` (`max_values`: Some(1), `max_size`: Some(257), added: 752, mode: `MaxEncodedLen`)
	/// Storage: `Timestamp::Now` (r:1 w:0)
	/// Proof: `Timestamp::Now` (`max_values`: Some(1), `max_size`: Some(8), added: 503, mode: `MaxEncodedLen`)
	/// Storage: `Oracle::RawValues` (r:3 w:1)
	/// Proof: `Oracle::RawValues` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `Oracle::Values` (r:1 w:0)
	/// Proof: `Oracle::Values` (`max_values`: None, `max_size`: Some(33), added: 2508, mode: `MaxEncodedLen`)
	/// The range of component `x` is `[1, 2]`.
	fn feed_values(x: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `279`
		//  Estimated: `8634`
		// Minimum execution time: 28_512_000 picoseconds.
		Weight::from_parts(20_589_375, 8634)
			// Standard Error: 94_515
			.saturating_add(Weight::from_parts(8_962_312, 0).saturating_mul(x.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `Oracle::HasDispatched` (r:0 w:1)
	/// Proof: `Oracle::HasDispatched` (`max_values`: Some(1), `max_size`: Some(257), added: 752, mode: `MaxEncodedLen`)
	fn on_finalize() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 812_000 picoseconds.
		Weight::from_parts(885_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}
