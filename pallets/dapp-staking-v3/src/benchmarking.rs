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

use super::{Pallet as DappStaking, *};

use astar_primitives::Balance;
use frame_benchmarking::v2::*;

use frame_support::assert_ok;
use frame_system::{Pallet as System, RawOrigin};

// TODO: copy/paste from mock, make it more generic later

/// Run to the specified block number.
/// Function assumes first block has been initialized.
fn run_to_block<T: Config>(n: BlockNumberFor<T>) {
    while System::<T>::block_number() < n {
        DappStaking::<T>::on_finalize(System::<T>::block_number());
        System::<T>::set_block_number(System::<T>::block_number() + One::one());
        // This is performed outside of dapps staking but we expect it before on_initialize
        DappStaking::<T>::on_initialize(System::<T>::block_number());
    }
}

/// Run for the specified number of blocks.
/// Function assumes first block has been initialized.
fn run_for_blocks<T: Config>(n: BlockNumberFor<T>) {
    run_to_block::<T>(System::<T>::block_number() + n);
}

/// Advance blocks until the specified era has been reached.
///
/// Function has no effect if era is already passed.
pub(crate) fn advance_to_era<T: Config>(era: EraNumber) {
    assert!(era >= ActiveProtocolState::<T>::get().era);
    while ActiveProtocolState::<T>::get().era < era {
        run_for_blocks::<T>(One::one());
    }
}

/// Advance blocks until next era has been reached.
pub(crate) fn advance_to_next_era<T: Config>() {
    advance_to_era::<T>(ActiveProtocolState::<T>::get().era + 1);
}

/// Advance blocks until the specified period has been reached.
///
/// Function has no effect if period is already passed.
pub(crate) fn advance_to_period<T: Config>(period: PeriodNumber) {
    assert!(period >= ActiveProtocolState::<T>::get().period_number());
    while ActiveProtocolState::<T>::get().period_number() < period {
        run_for_blocks::<T>(One::one());
    }
}

/// Advance blocks until next period has been reached.
pub(crate) fn advance_to_next_period<T: Config>() {
    advance_to_period::<T>(ActiveProtocolState::<T>::get().period_number() + 1);
}

/// Advance blocks until next period type has been reached.
pub(crate) fn advance_to_next_subperiod<T: Config>() {
    let subperiod = ActiveProtocolState::<T>::get().subperiod();
    while ActiveProtocolState::<T>::get().subperiod() == subperiod {
        run_for_blocks::<T>(One::one());
    }
}

// All our networks use 18 decimals for native currency so this should be fine.
const UNIT: Balance = 1_000_000_000_000_000_000;

// Minimum amount that must be staked on a dApp to enter any tier
const MIN_TIER_THRESHOLD: Balance = 10 * UNIT;

const NUMBER_OF_SLOTS: u32 = 100;

const SEED: u32 = 9000;

/// Assert that the last event equals the provided one.
fn assert_last_event<T: Config>(generic_event: <T as Config>::RuntimeEvent) {
    frame_system::Pallet::<T>::assert_last_event(generic_event.into());
}

pub fn initial_config<T: Config>() {
    let era_length = T::StandardEraLength::get();
    let voting_period_length_in_eras = T::StandardErasPerVotingSubperiod::get();

    // Init protocol state
    ActiveProtocolState::<T>::put(ProtocolState {
        era: 1,
        next_era_start: era_length.saturating_mul(voting_period_length_in_eras.into()) + One::one(),
        period_info: PeriodInfo {
            number: 1,
            subperiod: Subperiod::Voting,
            subperiod_end_era: 2,
        },
        maintenance: false,
    });

    // Init tier params
    let tier_params = TierParameters::<T::NumberOfTiers> {
        reward_portion: BoundedVec::try_from(vec![
            Permill::from_percent(40),
            Permill::from_percent(30),
            Permill::from_percent(20),
            Permill::from_percent(10),
        ])
        .unwrap(),
        slot_distribution: BoundedVec::try_from(vec![
            Permill::from_percent(10),
            Permill::from_percent(20),
            Permill::from_percent(30),
            Permill::from_percent(40),
        ])
        .unwrap(),
        tier_thresholds: BoundedVec::try_from(vec![
            TierThreshold::DynamicTvlAmount {
                amount: 100 * UNIT,
                minimum_amount: 80 * UNIT,
            },
            TierThreshold::DynamicTvlAmount {
                amount: 50 * UNIT,
                minimum_amount: 40 * UNIT,
            },
            TierThreshold::DynamicTvlAmount {
                amount: 20 * UNIT,
                minimum_amount: 20 * UNIT,
            },
            TierThreshold::FixedTvlAmount {
                amount: MIN_TIER_THRESHOLD,
            },
        ])
        .unwrap(),
    };

    // Init tier config, based on the initial params
    let init_tier_config = TiersConfiguration::<T::NumberOfTiers> {
        number_of_slots: NUMBER_OF_SLOTS.try_into().unwrap(),
        slots_per_tier: BoundedVec::try_from(vec![10, 20, 30, 40]).unwrap(),
        reward_portion: tier_params.reward_portion.clone(),
        tier_thresholds: tier_params.tier_thresholds.clone(),
    };

    assert!(tier_params.is_valid());
    assert!(init_tier_config.is_valid());

    StaticTierParams::<T>::put(tier_params);
    TierConfig::<T>::put(init_tier_config.clone());
    NextTierConfig::<T>::put(init_tier_config);
}

fn max_number_of_contracts<T: Config>() -> u32 {
    T::MaxNumberOfContracts::get().min(NUMBER_OF_SLOTS).into()
}

#[benchmarks]
mod benchmarks {
    use super::*;

    #[benchmark]
    fn maintenance_mode() {
        initial_config::<T>();

        #[extrinsic_call]
        _(RawOrigin::Root, true);

        assert_last_event::<T>(Event::<T>::MaintenanceMode { enabled: true }.into());
    }

    #[benchmark]
    fn register() {
        initial_config::<T>();

        let account: T::AccountId = account("dapp_owner", 0, SEED);
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);

        #[extrinsic_call]
        _(RawOrigin::Root, account.clone(), smart_contract.clone());

        assert_last_event::<T>(
            Event::<T>::DAppRegistered {
                owner: account,
                smart_contract,
                dapp_id: 0,
            }
            .into(),
        );
    }

    #[benchmark]
    fn set_dapp_reward_beneficiary() {
        initial_config::<T>();

        let owner: T::AccountId = whitelisted_caller();
        let beneficiary: Option<T::AccountId> = Some(account("beneficiary", 0, SEED));
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            owner.clone().into(),
            smart_contract.clone(),
        ));

        #[extrinsic_call]
        _(
            RawOrigin::Signed(owner),
            smart_contract.clone(),
            beneficiary.clone(),
        );

        assert_last_event::<T>(
            Event::<T>::DAppRewardDestinationUpdated {
                smart_contract,
                beneficiary,
            }
            .into(),
        );
    }

    #[benchmark]
    fn set_dapp_owner() {
        initial_config::<T>();

        let init_owner: T::AccountId = whitelisted_caller();
        let new_owner: T::AccountId = account("dapp_owner", 0, SEED);
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            init_owner.clone().into(),
            smart_contract.clone(),
        ));

        #[extrinsic_call]
        _(
            RawOrigin::Signed(init_owner),
            smart_contract.clone(),
            new_owner.clone(),
        );

        assert_last_event::<T>(
            Event::<T>::DAppOwnerChanged {
                smart_contract,
                new_owner,
            }
            .into(),
        );
    }

    #[benchmark]
    fn unregister() {
        initial_config::<T>();

        let owner: T::AccountId = whitelisted_caller();
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            owner.clone().into(),
            smart_contract.clone(),
        ));

        #[extrinsic_call]
        _(RawOrigin::Root, smart_contract.clone());

        assert_last_event::<T>(
            Event::<T>::DAppUnregistered {
                smart_contract,
                era: ActiveProtocolState::<T>::get().era,
            }
            .into(),
        );
    }

    #[benchmark]
    fn lock() {
        initial_config::<T>();

        let staker: T::AccountId = whitelisted_caller();
        let owner: T::AccountId = account("dapp_owner", 0, SEED);
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            owner.clone().into(),
            smart_contract.clone(),
        ));

        let amount = T::MinimumLockedAmount::get();
        T::Currency::make_free_balance_be(&staker, amount);

        #[extrinsic_call]
        _(RawOrigin::Signed(staker.clone()), amount);

        assert_last_event::<T>(
            Event::<T>::Locked {
                account: staker,
                amount,
            }
            .into(),
        );
    }

    #[benchmark]
    fn unlock() {
        initial_config::<T>();

        let staker: T::AccountId = whitelisted_caller();
        let owner: T::AccountId = account("dapp_owner", 0, SEED);
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            owner.clone().into(),
            smart_contract.clone(),
        ));

        let amount = T::MinimumLockedAmount::get() * 2;
        T::Currency::make_free_balance_be(&staker, amount);
        assert_ok!(DappStaking::<T>::lock(
            RawOrigin::Signed(staker.clone()).into(),
            amount,
        ));

        #[extrinsic_call]
        _(RawOrigin::Signed(staker.clone()), 1);

        assert_last_event::<T>(
            Event::<T>::Unlocking {
                account: staker,
                amount: 1,
            }
            .into(),
        );
    }

    // TODO: maybe this is not needed. Compare it after running benchmarks to the 'not-full' unlock
    #[benchmark]
    fn full_unlock() {
        initial_config::<T>();

        let staker: T::AccountId = whitelisted_caller();
        let owner: T::AccountId = account("dapp_owner", 0, SEED);
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            owner.clone().into(),
            smart_contract.clone(),
        ));

        let amount = T::MinimumLockedAmount::get() * 2;
        T::Currency::make_free_balance_be(&staker, amount);
        assert_ok!(DappStaking::<T>::lock(
            RawOrigin::Signed(staker.clone()).into(),
            amount,
        ));

        #[extrinsic_call]
        unlock(RawOrigin::Signed(staker.clone()), amount);

        assert_last_event::<T>(
            Event::<T>::Unlocking {
                account: staker,
                amount,
            }
            .into(),
        );
    }

    #[benchmark]
    fn claim_unlocked(x: Linear<0, { T::MaxNumberOfStakedContracts::get() }>) {
        // Prepare staker account and lock some amount
        let staker: T::AccountId = whitelisted_caller();
        let amount = (T::MinimumStakeAmount::get() + 1)
            * Into::<Balance>::into(max_number_of_contracts::<T>())
            + 1;
        T::Currency::make_free_balance_be(&staker, amount);
        assert_ok!(DappStaking::<T>::lock(
            RawOrigin::Signed(staker.clone()).into(),
            amount,
        ));

        // Move over to the build&earn subperiod to ensure 'non-loyal' staking.
        // This is needed so we can achieve staker entry cleanup after claiming unlocked tokens.
        advance_to_next_subperiod::<T>();
        assert_eq!(
          ActiveProtocolState::<T>::get().subperiod(),
          Subperiod::BuildAndEarn,
          "Sanity check - we need to stake during build&earn for entries to be cleaned up in the next era."
        );

        // Register required number of contracts and have staker stake on them.
        // This is needed to achieve the cleanup functionality.
        for x in 0..x {
            let smart_contract = T::BenchmarkHelper::get_smart_contract(x as u32);
            let owner: T::AccountId = account("dapp_owner", x.into(), SEED);

            assert_ok!(DappStaking::<T>::register(
                RawOrigin::Root.into(),
                owner.clone().into(),
                smart_contract.clone(),
            ));

            assert_ok!(DappStaking::<T>::stake(
                RawOrigin::Signed(staker.clone()).into(),
                smart_contract,
                T::MinimumStakeAmount::get() + 1,
            ));
        }

        // Finally, unlock some amount.
        let unlock_amount = 1;
        assert_ok!(DappStaking::<T>::unlock(
            RawOrigin::Signed(staker.clone()).into(),
            unlock_amount,
        ));

        // Advance to next period to ensure the old stake entries are cleaned up.
        advance_to_next_period::<T>();

        // Additionally, ensure enough blocks have passed so that the unlocking chunk can be claimed.
        let unlock_block = Ledger::<T>::get(&staker).unlocking[0].unlock_block;
        run_to_block::<T>(unlock_block);

        #[extrinsic_call]
        _(RawOrigin::Signed(staker.clone()));

        assert_last_event::<T>(
            Event::<T>::ClaimedUnlocked {
                account: staker,
                amount: unlock_amount,
            }
            .into(),
        );
    }

    #[benchmark]
    fn relock_unlocking() {
        initial_config::<T>();

        let staker: T::AccountId = whitelisted_caller();
        let owner: T::AccountId = account("dapp_owner", 0, SEED);
        let smart_contract = T::BenchmarkHelper::get_smart_contract(1);
        assert_ok!(DappStaking::<T>::register(
            RawOrigin::Root.into(),
            owner.clone().into(),
            smart_contract.clone(),
        ));

        let amount = T::MinimumLockedAmount::get() * 2;
        T::Currency::make_free_balance_be(&staker, amount);
        assert_ok!(DappStaking::<T>::lock(
            RawOrigin::Signed(staker.clone()).into(),
            amount,
        ));

        let unlock_amount = 1;
        assert_ok!(DappStaking::<T>::unlock(
            RawOrigin::Signed(staker.clone()).into(),
            unlock_amount,
        ));

        #[extrinsic_call]
        _(RawOrigin::Signed(staker.clone()));

        assert_last_event::<T>(
            Event::<T>::Relock {
                account: staker,
                amount: unlock_amount,
            }
            .into(),
        );
    }

    // TODO: investigate why the PoV size is so large here, evne after removing read of `IntegratedDApps` storage.
    // Relevant file: polkadot-sdk/substrate/utils/frame/benchmarking-cli/src/pallet/writer.rs
    // UPDATE: after some investigation, it seems that PoV size benchmarks are very unprecise
    // - the worst case measured is usually very far off the actual value that is consumed on chain.
    // There's an ongoing item to improve it (mentioned on roundtable meeting).
    #[benchmark]
    fn dapp_tier_assignment(x: Linear<0, { max_number_of_contracts::<T>() }>) {
        // Prepare init config (protocol state, tier params & config, etc.)
        initial_config::<T>();

        let developer: T::AccountId = whitelisted_caller();
        for id in 0..x {
            let smart_contract = T::BenchmarkHelper::get_smart_contract(id as u32);
            assert_ok!(DappStaking::<T>::register(
                RawOrigin::Root.into(),
                developer.clone().into(),
                smart_contract,
            ));
        }

        // TODO: try to make this more "shuffled" so the generated vector ends up being more random
        let mut amount = 1000 * MIN_TIER_THRESHOLD;
        for id in 0..x {
            let staker = account("staker", id.into(), 1337);
            T::Currency::make_free_balance_be(&staker, amount);
            assert_ok!(DappStaking::<T>::lock(
                RawOrigin::Signed(staker.clone()).into(),
                amount,
            ));

            let smart_contract = T::BenchmarkHelper::get_smart_contract(id as u32);
            assert_ok!(DappStaking::<T>::stake(
                RawOrigin::Signed(staker.clone()).into(),
                smart_contract,
                amount,
            ));

            // Slowly decrease the stake amount
            amount.saturating_reduce(UNIT);
        }

        // Advance to next era
        advance_to_next_era::<T>();

        let reward_era = ActiveProtocolState::<T>::get().era;
        let reward_period = ActiveProtocolState::<T>::get().period_number();
        let reward_pool = Balance::from(10_000 * UNIT as u128);

        #[block]
        {
            let dapp_tiers =
                Pallet::<T>::get_dapp_tier_assignment(reward_era, reward_period, reward_pool);
            // TODO: how to move this outside of the 'block'? Cannot declare it outside, and then use it inside.
            assert_eq!(dapp_tiers.dapps.len(), x as usize);
        }
    }

    impl_benchmark_test_suite!(
        Pallet,
        crate::benchmarking::tests::new_test_ext(),
        crate::test::mock::Test,
    );
}

#[cfg(test)]
mod tests {
    use crate::test::mock;
    use sp_io::TestExternalities;

    pub fn new_test_ext() -> TestExternalities {
        mock::ExtBuilder::build()
    }
}
