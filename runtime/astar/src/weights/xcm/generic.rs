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

//! Autogenerated weights for xcm_benchmarks_generic
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-03-27, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `gh-runner-01-ovh`, CPU: `Intel(R) Xeon(R) E-2236 CPU @ 3.40GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("astar-dev"), DB CACHE: 1024

// Executed Command:
// ./target/release/astar-collator
// benchmark
// pallet
// --chain=astar-dev
// --steps=50
// --repeat=20
// --pallet=xcm_benchmarks_generic
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./benchmark-results/astar-dev/benchmarks_generic_weights.rs
// --template=./scripts/templates/weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for xcm_benchmarks_generic.
pub trait WeightInfo {
	fn buy_execution() -> Weight;
	fn expect_pallet() -> Weight;
	fn exchange_asset() -> Weight;
	fn export_message() -> Weight;
	fn lock_asset() -> Weight;
	fn unlock_asset() -> Weight;
	fn note_unlockable() -> Weight;
	fn request_unlock() -> Weight;
	fn universal_origin() -> Weight;
	fn alias_origin() -> Weight;
	fn report_holding() -> Weight;
	fn query_response() -> Weight;
	fn transact() -> Weight;
	fn refund_surplus() -> Weight;
	fn set_error_handler() -> Weight;
	fn set_appendix() -> Weight;
	fn clear_error() -> Weight;
	fn descend_origin() -> Weight;
	fn clear_origin() -> Weight;
	fn report_error() -> Weight;
	fn claim_asset() -> Weight;
	fn trap() -> Weight;
	fn subscribe_version() -> Weight;
	fn unsubscribe_version() -> Weight;
	fn burn_asset() -> Weight;
	fn expect_asset() -> Weight;
	fn expect_origin() -> Weight;
	fn expect_error() -> Weight;
	fn expect_transact_status() -> Weight;
	fn query_pallet() -> Weight;
	fn report_transact_status() -> Weight;
	fn clear_transact_status() -> Weight;
	fn set_topic() -> Weight;
	fn clear_topic() -> Weight;
	fn set_fees_mode() -> Weight;
	fn unpaid_execution() -> Weight;
}

/// Weights for xcm_benchmarks_generic using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `XcAssetConfig::AssetLocationUnitsPerSecond` (r:1 w:0)
	/// Proof: `XcAssetConfig::AssetLocationUnitsPerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcAssetConfig::AssetLocationToId` (r:1 w:0)
	/// Proof: `XcAssetConfig::AssetLocationToId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(222), added: 2697, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn buy_execution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2296`
		//  Estimated: `5761`
		// Minimum execution time: 105_002_000 picoseconds.
		Weight::from_parts(108_931_000, 5761)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	fn expect_pallet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_852_000 picoseconds.
		Weight::from_parts(6_089_000, 0)
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn exchange_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn export_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn lock_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn unlock_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn note_unlockable() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn request_unlock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn universal_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn alias_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
	}
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn report_holding() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `74`
		//  Estimated: `3539`
		// Minimum execution time: 346_909_000 picoseconds.
		Weight::from_parts(352_602_000, 3539)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `PolkadotXcm::Queries` (r:1 w:0)
	/// Proof: `PolkadotXcm::Queries` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn query_response() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `3465`
		// Minimum execution time: 8_607_000 picoseconds.
		Weight::from_parts(8_813_000, 3465)
			.saturating_add(T::DbWeight::get().reads(1_u64))
	}
	fn transact() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_103_000 picoseconds.
		Weight::from_parts(12_304_000, 0)
	}
	fn refund_surplus() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_737_000 picoseconds.
		Weight::from_parts(4_943_000, 0)
	}
	fn set_error_handler() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_764_000 picoseconds.
		Weight::from_parts(2_853_000, 0)
	}
	fn set_appendix() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_793_000 picoseconds.
		Weight::from_parts(2_876_000, 0)
	}
	fn clear_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_746_000 picoseconds.
		Weight::from_parts(2_828_000, 0)
	}
	fn descend_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_348_000 picoseconds.
		Weight::from_parts(3_482_000, 0)
	}
	fn clear_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_730_000 picoseconds.
		Weight::from_parts(2_824_000, 0)
	}
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn report_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `74`
		//  Estimated: `3539`
		// Minimum execution time: 21_279_000 picoseconds.
		Weight::from_parts(21_644_000, 3539)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `PolkadotXcm::AssetTraps` (r:1 w:1)
	/// Proof: `PolkadotXcm::AssetTraps` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn claim_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `23`
		//  Estimated: `3488`
		// Minimum execution time: 12_317_000 picoseconds.
		Weight::from_parts(12_590_000, 3488)
			.saturating_add(T::DbWeight::get().reads(1_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn trap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_715_000 picoseconds.
		Weight::from_parts(2_780_000, 0)
	}
	/// Storage: `PolkadotXcm::VersionNotifyTargets` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionNotifyTargets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn subscribe_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3507`
		// Minimum execution time: 22_632_000 picoseconds.
		Weight::from_parts(22_943_000, 3507)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: `PolkadotXcm::VersionNotifyTargets` (r:0 w:1)
	/// Proof: `PolkadotXcm::VersionNotifyTargets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn unsubscribe_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_554_000 picoseconds.
		Weight::from_parts(4_749_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn burn_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 120_072_000 picoseconds.
		Weight::from_parts(123_411_000, 0)
	}
	fn expect_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_755_000 picoseconds.
		Weight::from_parts(11_914_000, 0)
	}
	fn expect_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_830_000 picoseconds.
		Weight::from_parts(2_913_000, 0)
	}
	fn expect_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_664_000 picoseconds.
		Weight::from_parts(2_789_000, 0)
	}
	fn expect_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_865_000 picoseconds.
		Weight::from_parts(2_977_000, 0)
	}
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn query_pallet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `74`
		//  Estimated: `3539`
		// Minimum execution time: 26_145_000 picoseconds.
		Weight::from_parts(26_530_000, 3539)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn report_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `74`
		//  Estimated: `3539`
		// Minimum execution time: 21_368_000 picoseconds.
		Weight::from_parts(22_050_000, 3539)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	fn clear_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_759_000 picoseconds.
		Weight::from_parts(2_849_000, 0)
	}
	fn set_topic() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_723_000 picoseconds.
		Weight::from_parts(2_797_000, 0)
	}
	fn clear_topic() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_684_000 picoseconds.
		Weight::from_parts(2_757_000, 0)
	}
	fn set_fees_mode() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_685_000 picoseconds.
		Weight::from_parts(2_790_000, 0)
	}
	fn unpaid_execution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_819_000 picoseconds.
		Weight::from_parts(2_895_000, 0)
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `XcAssetConfig::AssetLocationUnitsPerSecond` (r:1 w:0)
	/// Proof: `XcAssetConfig::AssetLocationUnitsPerSecond` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `XcAssetConfig::AssetLocationToId` (r:1 w:0)
	/// Proof: `XcAssetConfig::AssetLocationToId` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Assets::Asset` (r:1 w:1)
	/// Proof: `Assets::Asset` (`max_values`: None, `max_size`: Some(222), added: 2697, mode: `MaxEncodedLen`)
	/// Storage: `Assets::Account` (r:1 w:1)
	/// Proof: `Assets::Account` (`max_values`: None, `max_size`: Some(146), added: 2621, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn buy_execution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `2296`
		//  Estimated: `5761`
		// Minimum execution time: 105_002_000 picoseconds.
		Weight::from_parts(108_931_000, 5761)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	fn expect_pallet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 5_852_000 picoseconds.
		Weight::from_parts(6_089_000, 0)
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn exchange_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn export_message() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn lock_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn unlock_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn note_unlockable() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn request_unlock() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn universal_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
	}
	/// Storage: `Benchmark::Override` (r:0 w:0)
	/// Proof: `Benchmark::Override` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn alias_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 18_446_744_073_709_551_000 picoseconds.
		Weight::from_parts(18_446_744_073_709_551_000, 0)
	}
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn report_holding() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `74`
		//  Estimated: `3539`
		// Minimum execution time: 346_909_000 picoseconds.
		Weight::from_parts(352_602_000, 3539)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `PolkadotXcm::Queries` (r:1 w:0)
	/// Proof: `PolkadotXcm::Queries` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn query_response() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `3465`
		// Minimum execution time: 8_607_000 picoseconds.
		Weight::from_parts(8_813_000, 3465)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
	}
	fn transact() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 12_103_000 picoseconds.
		Weight::from_parts(12_304_000, 0)
	}
	fn refund_surplus() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_737_000 picoseconds.
		Weight::from_parts(4_943_000, 0)
	}
	fn set_error_handler() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_764_000 picoseconds.
		Weight::from_parts(2_853_000, 0)
	}
	fn set_appendix() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_793_000 picoseconds.
		Weight::from_parts(2_876_000, 0)
	}
	fn clear_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_746_000 picoseconds.
		Weight::from_parts(2_828_000, 0)
	}
	fn descend_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 3_348_000 picoseconds.
		Weight::from_parts(3_482_000, 0)
	}
	fn clear_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_730_000 picoseconds.
		Weight::from_parts(2_824_000, 0)
	}
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn report_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `74`
		//  Estimated: `3539`
		// Minimum execution time: 21_279_000 picoseconds.
		Weight::from_parts(21_644_000, 3539)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `PolkadotXcm::AssetTraps` (r:1 w:1)
	/// Proof: `PolkadotXcm::AssetTraps` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn claim_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `23`
		//  Estimated: `3488`
		// Minimum execution time: 12_317_000 picoseconds.
		Weight::from_parts(12_590_000, 3488)
			.saturating_add(RocksDbWeight::get().reads(1_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn trap() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_715_000 picoseconds.
		Weight::from_parts(2_780_000, 0)
	}
	/// Storage: `PolkadotXcm::VersionNotifyTargets` (r:1 w:1)
	/// Proof: `PolkadotXcm::VersionNotifyTargets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn subscribe_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `42`
		//  Estimated: `3507`
		// Minimum execution time: 22_632_000 picoseconds.
		Weight::from_parts(22_943_000, 3507)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: `PolkadotXcm::VersionNotifyTargets` (r:0 w:1)
	/// Proof: `PolkadotXcm::VersionNotifyTargets` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn unsubscribe_version() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 4_554_000 picoseconds.
		Weight::from_parts(4_749_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn burn_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 120_072_000 picoseconds.
		Weight::from_parts(123_411_000, 0)
	}
	fn expect_asset() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 11_755_000 picoseconds.
		Weight::from_parts(11_914_000, 0)
	}
	fn expect_origin() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_830_000 picoseconds.
		Weight::from_parts(2_913_000, 0)
	}
	fn expect_error() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_664_000 picoseconds.
		Weight::from_parts(2_789_000, 0)
	}
	fn expect_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_865_000 picoseconds.
		Weight::from_parts(2_977_000, 0)
	}
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn query_pallet() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `74`
		//  Estimated: `3539`
		// Minimum execution time: 26_145_000 picoseconds.
		Weight::from_parts(26_530_000, 3539)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `ParachainInfo::ParachainId` (r:1 w:0)
	/// Proof: `ParachainInfo::ParachainId` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `PolkadotXcm::SupportedVersion` (r:1 w:0)
	/// Proof: `PolkadotXcm::SupportedVersion` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::HostConfiguration` (r:1 w:0)
	/// Proof: `ParachainSystem::HostConfiguration` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `ParachainSystem::PendingUpwardMessages` (r:1 w:1)
	/// Proof: `ParachainSystem::PendingUpwardMessages` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	fn report_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `74`
		//  Estimated: `3539`
		// Minimum execution time: 21_368_000 picoseconds.
		Weight::from_parts(22_050_000, 3539)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	fn clear_transact_status() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_759_000 picoseconds.
		Weight::from_parts(2_849_000, 0)
	}
	fn set_topic() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_723_000 picoseconds.
		Weight::from_parts(2_797_000, 0)
	}
	fn clear_topic() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_684_000 picoseconds.
		Weight::from_parts(2_757_000, 0)
	}
	fn set_fees_mode() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_685_000 picoseconds.
		Weight::from_parts(2_790_000, 0)
	}
	fn unpaid_execution() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_819_000 picoseconds.
		Weight::from_parts(2_895_000, 0)
	}
}
