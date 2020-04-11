//! # Plasm Lockdrop Module
//! This module held lockdrop event on live network.
//!
//! - [`plasm_lockdrop::Trait`](./trait.Trait.html)
//! - [`Call`](./enum.Call.html)
//!
//! ## Overview
//!
//!
//! ## Interface
//!
//! ### Dispatchable Functions
//!
//!
//! [`Call`]: ./enum.Call.html
//! [`Trait`]: ./trait.Trait.html

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Encode, Decode};
use sp_std::prelude::*;
use sp_core::{H256, ecdsa};
use sp_runtime::{
    RuntimeDebug, Perbill,
    traits::{
        Member, IdentifyAccount, BlakeTwo256, Hash, Saturating, AtLeast32Bit,
    },
    app_crypto::RuntimeAppPublic,
    transaction_validity::{
        ValidTransaction, InvalidTransaction,
        TransactionValidity, TransactionPriority,
        TransactionSource,
    },
};
use frame_support::{
    decl_module, decl_event, decl_storage, decl_error,
    debug, ensure, StorageValue, StorageMap,
    weights::{SimpleDispatchInfo, Weight},
    traits::{Get, Currency, Time},
    storage::IterableStorageMap,
    dispatch::Parameter,
};
use frame_system::{
    self as system, ensure_signed, ensure_none,
    offchain::SubmitUnsignedTransaction,
};

/// Bitcoin helpers.
mod btc_utils;
/// Ethereum helpers.
mod eth_utils;
/// Authority keys.
mod crypto;
/// Oracle traits.
mod oracle;

pub use oracle::{PriceOracle, ChainOracle};
pub use crypto::*;

#[cfg(test)]
mod mock;
#[cfg(test)]
mod tests;

pub type BalanceOf<T> =
    <<T as Trait>::Currency as Currency<<T as frame_system::Trait>::AccountId>>::Balance;

/// The module's main configuration trait.
pub trait Trait: system::Trait {
    /// The lockdrop balance.
    type Currency: Currency<Self::AccountId>;

    /// Bitcoin price oracle. 
    type BitcoinTicker: PriceOracle<Self::DollarRate>;

    /// Ethereum price oracle.
    type EthereumTicker: PriceOracle<Self::DollarRate>;

    /// Bitcoin transaction oracle.
    type BitcoinApi: ChainOracle<H256>;

    /// Ethereum transaction oracle.
    type EthereumApi: ChainOracle<H256>;

    /// How long dollar rate parameters valid in secs
    type MedianFilterExpire: Get<Self::Moment>;

    /// Width of dollar rate median filter.
    type MedianFilterWidth: Get<usize>;

    /// A dispatchable call type.
    type Call: From<Call<Self>>;

    /// Let's define the helper we use to create signed transactions.
    type SubmitTransaction: SubmitUnsignedTransaction<Self, <Self as Trait>::Call>;

    /// The identifier type for an authority.
    type AuthorityId: Member + Parameter + RuntimeAppPublic + Default + Ord;

    /// System level account type.
    /// This used for resolving account ID's of ECDSA lockdrop public keys.
    type Account: IdentifyAccount<AccountId=Self::AccountId> + From<ecdsa::Public>;

    /// Module that could provide timestamp.
    type Time: Time<Moment=Self::Moment>;

    /// Timestamp type.
    type Moment: Member + Parameter + Saturating + AtLeast32Bit
        + Copy + Default + From<u64> + Into<u64> + Into<u128>;

    /// Dollar rate number data type.
    type DollarRate: Member + Parameter + AtLeast32Bit
        + Copy + Default + Into<u128> + From<u64>;

    // XXX: I don't known how to convert into Balance from u128 without it
    // TODO: Should be removed
    type BalanceConvert: From<u128> +
        Into<<Self::Currency as Currency<<Self as frame_system::Trait>::AccountId>>::Balance>;

    /// The regular events type.
    type Event: From<Event<Self>> + Into<<Self as frame_system::Trait>::Event>;
}

/// Claim id is a hash of claim parameters.
pub type ClaimId = H256;

/// Type for enumerating claim proof votes. 
pub type AuthorityVote = u32;

/// Type for enumerating authorities in list (2^16 authorities is enough).
pub type AuthorityIndex = u16;

/// Plasm Lockdrop parameters.
#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, Clone)]
pub enum Lockdrop {
    /// Bitcoin lockdrop is pretty simple:
    /// transaction sended with time-lockding opcode,
    /// BTC token locked and could be spend some timestamp.
    /// Duration in blocks and value in shatoshi could be derived from BTC transaction.
    Bitcoin { public: ecdsa::Public, value: u128, duration: u64, transaction_hash: H256, },
    /// Ethereum lockdrop transactions is sended to pre-deployed lockdrop smart contract.
    Ethereum { public: ecdsa::Public, value: u128, duration: u64, transaction_hash: H256, },
}

impl Default for Lockdrop {
    fn default() -> Self {
        Lockdrop::Bitcoin {
            public: Default::default(),
            value: Default::default(),
            duration: Default::default(),
            transaction_hash: Default::default(),
        }
    }
}

/// Lockdrop claim request description.
#[cfg_attr(feature = "std", derive(PartialEq, Eq))]
#[derive(Encode, Decode, RuntimeDebug, Default, Clone)]
pub struct Claim {
    params:   Lockdrop,
    approve:  AuthorityVote,
    decline:  AuthorityVote,
    amount:   u128,
    complete: bool,
}

/// Lockdrop claim vote.
#[cfg_attr(feature = "std", derive(Eq))]
#[derive(Encode, Decode, RuntimeDebug, PartialEq, Clone)]
pub struct ClaimVote {
    claim_id:  ClaimId,
    approve:   bool,
    authority: AuthorityIndex,
}

/// Oracle dollar rate ticker.
#[cfg_attr(feature = "std", derive(Eq))]
#[derive(Encode, Decode, RuntimeDebug, PartialEq, Clone)]
pub struct TickerRate<DollarRate: Member + Parameter> {
    authority: AuthorityIndex,
    btc:       DollarRate,
    eth:       DollarRate,
}

decl_event!(
    pub enum Event<T>
    where <T as system::Trait>::AccountId,
          <T as Trait>::AuthorityId,
          <T as Trait>::DollarRate,
          Balance = BalanceOf<T>,
    {
        /// Lockdrop token claims requested by user
        ClaimRequest(ClaimId),
        /// Lockdrop token claims response by authority
        ClaimResponse(ClaimId, AuthorityId, bool),
        /// Lockdrop token claim paid
        ClaimComplete(ClaimId, AccountId, Balance),
        /// Dollar rate updated by oracle: BTC, ETH.
        NewDollarRate(DollarRate, DollarRate),
        /// New authority list registered
        NewAuthorities(Vec<AuthorityId>),
    }
);

decl_error! {
    pub enum Error for Module<T: Trait> {
    }
}

decl_storage! {
    trait Store for Module<T: Trait> as Provider {
        /// Offchain lock check requests made within this block execution.
        Requests get(fn requests): Vec<ClaimId>;
        /// List of lockdrop authority id's.
        Keys get(fn keys): Vec<T::AuthorityId>;
        /// Token claim requests.
        Claims get(fn claims):
            map hasher(blake2_128_concat) ClaimId
            => Claim;
        /// Double vote prevention registry.
        HasVote get(fn has_vote):
            double_map hasher(blake2_128_concat) T::AuthorityId, hasher(blake2_128_concat) ClaimId
            => bool;
        /// Lockdrop alpha parameter (fixed point at 1_000_000_000).
        /// α ∈ [0; 10]
        Alpha get(fn alpha) config(): Perbill;
        /// Lockdrop dollar rate parameter: BTC, ETH.
        DollarRate get(fn dollar_rate) config(): (T::DollarRate, T::DollarRate);
        /// Lockdrop dollar rate median filter table: Time, BTC, ETH.
        DollarRateF get(fn dollar_rate_f):
            map hasher(blake2_128_concat) T::AuthorityId
            => (T::Moment, T::DollarRate, T::DollarRate);
        /// How much authority votes module should receive to decide claim result.
        VoteThreshold get(fn vote_threshold) config(): AuthorityVote;
        /// How much positive votes requered to approve claim.
        ///   Positive votes = approve votes - decline votes.
        PositiveVotes get(fn positive_votes) config(): AuthorityVote;
        /// Ethereum lockdrop contract address.
        EthereumContract get(fn ethereum_contract) config(): String; 
        /// Timestamp of finishing lockdrop.
        LockdropEnd get(fn lockdrop_end) config(): T::Moment;
    }
}

decl_module! {
    pub struct Module<T: Trait> for enum Call where origin: T::Origin {
        type Error = Error<T>;

        /// Initializing events
        fn deposit_event() = default;

        /// Clean the state on initialisation of a block
        fn on_initialize(_now: T::BlockNumber) -> Weight {
            // At the beginning of each block execution, system triggers all
            // `on_initialize` functions, which allows us to set up some temporary state or - like
            // in this case - clean up other states
            <Requests>::kill();

            50_000
        }

        /// Request authorities to check locking transaction. 
        #[weight = SimpleDispatchInfo::FixedNormal(1_000_000)]
        fn request(
            origin,
            params: Lockdrop,
        ) {
            let _ = ensure_signed(origin)?;
            let claim_id = BlakeTwo256::hash_of(&params);
            ensure!(!<Claims>::get(claim_id).complete, "claim should not be already paid");

            if !<Claims>::contains_key(claim_id) {
                let amount = match params {
                    Lockdrop::Bitcoin { value, duration, .. } => {
                        // Average block duration in BTC is 10 min = 600 sec
                        let duration_sec = duration * 600;
                        // Cast bitcoin value to PLM order:
                        // satoshi = BTC * 10^9;
                        // PLM unit = PLM * 10^15;
                        // (it also helps to make evaluations more precise)
                        let value_btc = value * 1_000_000;
                        Self::btc_issue_amount(value_btc, duration_sec.into())
                    },
                    Lockdrop::Ethereum { value, duration, .. } => {
                        // Cast bitcoin value to PLM order:
                        // satoshi = ETH * 10^18;
                        // PLM unit = PLM * 10^15;
                        // (it also helps to make evaluations more precise)
                        let value_eth = value / 1_000;
                        Self::eth_issue_amount(value_eth, duration.into())
                    }
                };
                let claim = Claim { params, amount, .. Default::default() };
                <Claims>::insert(claim_id, claim);
            }

            <Requests>::mutate(|requests| requests.push(claim_id));
            Self::deposit_event(RawEvent::ClaimRequest(claim_id));
        }

        /// Claim tokens according to lockdrop procedure.
        #[weight = SimpleDispatchInfo::FixedNormal(1_000_000)]
        fn claim(
            origin,
            claim_id: ClaimId,
        ) {
            let _ = ensure_signed(origin)?;
            let claim = <Claims>::get(claim_id);
            ensure!(!claim.complete, "claim should be already paid"); 

            if claim.approve + claim.decline < <VoteThreshold>::get() {
                Err("this request don't get enough authority votes")?
            }

            if claim.approve.saturating_sub(claim.decline) < <PositiveVotes>::get() {
                Err("this request don't approved by authorities")?
            }

            // Deposit lockdrop tokens on locking public key.
            let account = match claim.params {
                Lockdrop::Bitcoin { public, .. }  => T::Account::from(public).into_account(),
                Lockdrop::Ethereum { public, .. } => T::Account::from(public).into_account(),
            };
            let amount: BalanceOf<T> = T::BalanceConvert::from(claim.amount).into();
            T::Currency::deposit_creating(&account, amount);

            // Finalize claim request
            <Claims>::mutate(claim_id, |claim| claim.complete = true);
            Self::deposit_event(RawEvent::ClaimComplete(claim_id, account, amount));
        }

        /// Vote for claim request according to check results. (for authorities only) 
        #[weight = SimpleDispatchInfo::default()]
        fn vote(
            origin,
            vote: ClaimVote,
            // since signature verification is done in `validate_unsigned`
            // we can skip doing it here again.
            _signature: <T::AuthorityId as RuntimeAppPublic>::Signature,
        ) {
            ensure_none(origin)?;

            <Claims>::mutate(&vote.claim_id, |claim|
                if vote.approve { claim.approve += 1 }
                else { claim.decline += 1 }
            );

            let keys = Keys::<T>::get();
            if let Some(authority) = keys.get(vote.authority as usize) {
                <HasVote<T>>::insert(authority, &vote.claim_id, true);
                Self::deposit_event(RawEvent::ClaimResponse(vote.claim_id, authority.clone(), vote.approve));
            } else {
                return Err("unable get authority by index")?;
            }
        }

        /// Dollar Rate oracle entrypoint. (for authorities only)
        #[weight = SimpleDispatchInfo::default()]
        fn set_dollar_rate(
            origin,
            rate: TickerRate<T::DollarRate>,
            // since signature verification is done in `validate_unsigned`
            // we can skip doing it here again.
            _signature: <T::AuthorityId as RuntimeAppPublic>::Signature,
        ) {
            ensure_none(origin)?;

            let now = T::Time::now();

            let keys = Keys::<T>::get();
            if let Some(authority) = keys.get(rate.authority as usize) {
                DollarRateF::<T>::insert(authority, (now, rate.btc, rate.eth));
            } else {
                return Err("unable to get authority by index")?;
            }

            let expire = T::MedianFilterExpire::get();
            let mut btc_filter = median::Filter::new(T::MedianFilterWidth::get());
            let mut eth_filter = median::Filter::new(T::MedianFilterWidth::get());
            let (mut btc_filtered_rate, mut eth_filtered_rate) = <DollarRate<T>>::get();
            for (a, item) in <DollarRateF<T>>::iter() {
                if now.saturating_sub(item.0) < expire {
                    // Use value in filter when not expired
                    btc_filtered_rate = btc_filter.consume(item.1);
                    eth_filtered_rate = eth_filter.consume(item.2);
                } else {
                    // Drop value when expired
                    <DollarRateF<T>>::remove(a);
                }
            }

            <DollarRate<T>>::put((btc_filtered_rate, eth_filtered_rate));
            Self::deposit_event(RawEvent::NewDollarRate(btc_filtered_rate, eth_filtered_rate));
        }

        // Runs after every block within the context and current state of said block.
        fn offchain_worker(_now: T::BlockNumber) {
            debug::RuntimeLogger::init();

            if sp_io::offchain::is_validator() {
                match Self::offchain() {
                    Err(e) => debug::error!(
                        target: "lockdrop-offchain-worker",
                        "lockdrop worker fails: {}", e
                    ),
                    _ => (),
                }
            }
        }
    }
}

impl<T: Trait> Module<T> {
    /// The main offchain worker entry point.
    fn offchain() -> Result<(), String> {
        // TODO: add delay to prevent frequent transaction sending
        Self::send_dollar_rate(
            T::BitcoinTicker::fetch()?,
            T::EthereumTicker::fetch()?,
        )?;

        // TODO: use permanent storage to track request when temporary failed
        Self::claim_request_oracle()
    }

    fn claim_request_oracle() -> Result<(), String> {
        for claim_id in Self::requests() {
            debug::debug!(
                target: "lockdrop-offchain-worker",
                "new claim request: id = {}", claim_id 
            );

            let approve = Self::check_lock(claim_id)?;
            debug::info!(
                target: "lockdrop-offchain-worker",
                "claim id {} => check result: {}", claim_id, approve 
            );

            for key in T::AuthorityId::all() {
                if let Some(authority) = Self::authority_index_of(&key) {
                    let vote = ClaimVote { authority, claim_id, approve };
                    let signature = key.sign(&vote.encode()).ok_or("signing error")?;
                    let call = Call::vote(vote, signature);
                    debug::debug!(
                        target: "lockdrop-offchain-worker",
                        "claim id {} => vote extrinsic: {:?}", claim_id, call
                    );

                    let res = T::SubmitTransaction::submit_unsigned(call);
                    debug::debug!(
                        target: "lockdrop-offchain-worker",
                        "claim id {} => vote extrinsic send: {:?}", claim_id, res
                    );
                }
            }
        }

        Ok(())
    }

    /// Send dollar rate as unsigned extrinsic from authority.
    fn send_dollar_rate(btc: T::DollarRate, eth: T::DollarRate) -> Result<(), String> { 
        for key in T::AuthorityId::all() {
            if let Some(authority) = Self::authority_index_of(&key) {
                let rate = TickerRate { authority, btc, eth };
                let signature = key.sign(&rate.encode()).ok_or("signing error")?;
                let call = Call::set_dollar_rate(rate, signature);
                debug::debug!(
                    target: "lockdrop-offchain-worker",
                    "dollar rate extrinsic: {:?}", call
                );

                let res = T::SubmitTransaction::submit_unsigned(call);
                debug::debug!(
                    target: "lockdrop-offchain-worker",
                    "dollar rate extrinsic send: {:?}", res
                );
            }
        }
        Ok(())
    }

    /// Check locking parameters of given claim.
    fn check_lock(claim_id: ClaimId) -> Result<bool, String> {
        let Claim { params, .. } = Self::claims(claim_id);
        match params {
            Lockdrop::Bitcoin { public, value, duration, transaction_hash } => {
                let tx = T::BitcoinApi::fetch(transaction_hash)?;
                debug::debug!(
                    target: "lockdrop-offchain-worker",
                    "claim id {} => fetched transaction: {:?}", claim_id, tx
                );

                let lock_script = btc_utils::lock_script(public, duration);
                debug::debug!(
                    target: "lockdrop-offchain-worker",
                    "claim id {} => desired lock script: {}", claim_id, hex::encode(lock_script.clone())
                );

                let script = btc_utils::p2sh(&btc_utils::script_hash(&lock_script[..]));
                debug::debug!(
                    target: "lockdrop-offchain-worker",
                    "claim id {} => desired P2HS script: {}", claim_id, hex::encode(script.clone())
                );

                let valid = tx.confirmations > 8
                         && tx.value == value
                         && tx.script == script;
                Ok(valid)
            },
            Lockdrop::Ethereum { public, value, duration, transaction_hash } => {
                let tx = T::EthereumApi::fetch(transaction_hash)?; 
                debug::debug!(
                    target: "lockdrop-offchain-worker",
                    "claim id {} => fetched transaction: {:?}", claim_id, tx
                );

                let script = eth_utils::lock_method(duration);
                debug::debug!(
                    target: "lockdrop-offchain-worker",
                    "claim id {} => desired lock script: {}", claim_id, hex::encode(script.clone())
                );

                let valid = tx.confirmations > 10 
                         && tx.value == value
                         && tx.script == script
                         && tx.recipient == <EthereumContract>::get()
                         && tx.sender == hex::encode(eth_utils::to_address(public));
                Ok(valid)
            }
        }
    }

    /// PLM issue amount for given BTC value and locking duration.
    fn btc_issue_amount(value: u128, duration: T::Moment) -> u128 {
        // https://medium.com/stake-technologies/plasm-lockdrop-introduction-99fa2dfc37c0
        let rate = Self::alpha() * Self::dollar_rate().0 * Self::time_bonus(duration).into();
        rate.into() * value * 10
    }

    /// PLM issue amount for given ETH value and locking duration.
    fn eth_issue_amount(value: u128, duration: T::Moment) -> u128 {
        // https://medium.com/stake-technologies/plasm-lockdrop-introduction-99fa2dfc37c0
        let rate = Self::alpha() * Self::dollar_rate().1 * Self::time_bonus(duration).into();
        rate.into() * value * 10
    }

    /// Lockdrop bonus depends of lockding duration.
    fn time_bonus(duration: T::Moment) -> u16 {
        let days: u64 = 24 * 60 * 60; // One day in seconds
        let duration_sec = Into::<u64>::into(duration);
        if duration_sec < 30 * days {
            0 // Dont permit to participate with locking less that one month
        } else if duration_sec < 100 * days {
            24
        } else if duration_sec < 300 * days {
            100
        } else if duration_sec < 1000 * days {
            360
        } else {
            1600
        }
    }

    /// Check that authority key list contains given account
    fn authority_index_of(public: &T::AuthorityId) -> Option<AuthorityIndex> {
        let keys = Keys::<T>::get();
        // O(n) is ok because of short list
        for (i, elem) in keys.iter().enumerate() {
            if elem.eq(public) {
                return Some(i as AuthorityIndex);
            }
        }
        None
    }
}

impl<T: Trait> sp_runtime::BoundToRuntimeAppPublic for Module<T> {
    type Public = T::AuthorityId;
}

impl<T: Trait> pallet_session::OneSessionHandler<T::AccountId> for Module<T> {
    type Key = T::AuthorityId;

    fn on_genesis_session<'a, I: 'a>(validators: I)
        where I: Iterator<Item=(&'a T::AccountId, T::AuthorityId)>
    {
        // Init authorities on genesis session.
        let authorities: Vec<_> = validators.map(|x| x.1).collect();
        Keys::<T>::put(authorities.clone());
        Self::deposit_event(RawEvent::NewAuthorities(authorities));
    }

    fn on_new_session<'a, I: 'a>(_changed: bool, validators: I, _queued_validators: I)
        where I: Iterator<Item=(&'a T::AccountId, T::AuthorityId)>
    {
        // Remember who the authorities are for the new session.
        let authorities: Vec<_> = validators.map(|x| x.1).collect();
        Keys::<T>::put(authorities.clone());
        Self::deposit_event(RawEvent::NewAuthorities(authorities));
    }

    fn on_before_session_ending() { }
    fn on_disabled(_i: usize) { }
}

impl<T: Trait> frame_support::unsigned::ValidateUnsigned for Module<T> {
    type Call = Call<T>;

    fn validate_unsigned(
        _source: TransactionSource,
        call: &Self::Call
    ) -> TransactionValidity {
        match call {
            Call::vote(vote, signature) => {
                // Verify call params
                if !<Claims>::contains_key(vote.claim_id.clone()) {
                    return InvalidTransaction::Call.into();
                }

                vote.using_encoded(|encoded_vote| {
                    // Verify authority
                    let keys = Keys::<T>::get();
                    if let Some(authority) = keys.get(vote.authority as usize) {
                        // Check that sender is authority 
                        if !authority.verify(&encoded_vote, &signature) {
                            return InvalidTransaction::BadProof.into();
                        }
                        // Double voting guard
                        if <HasVote<T>>::get(authority, vote.claim_id.clone()) {
                            return InvalidTransaction::Call.into();
                        }
                    } else {
                        return InvalidTransaction::BadProof.into();
                    }

                    Ok(ValidTransaction {
                        priority: TransactionPriority::max_value(),
                        requires: vec![],
                        provides: vec![encoded_vote.to_vec()],
                        longevity: 64_u64,
                        propagate: true,
                    })
                })
            },

            Call::set_dollar_rate(rate, signature) => {
                rate.using_encoded(|encoded_rate| {
                    let keys = Keys::<T>::get();
                    if let Some(authority) = keys.get(rate.authority as usize) {
                        // Check that sender is authority 
                        if !authority.verify(&encoded_rate, &signature) {
                            return InvalidTransaction::BadProof.into();
                        }
                    } else {
                        return InvalidTransaction::BadProof.into();
                    }

                    Ok(ValidTransaction {
                        priority: TransactionPriority::max_value(),
                        requires: vec![],
                        provides: vec![encoded_rate.to_vec()],
                        longevity: 64_u64,
                        propagate: true,
                    })
                })
            },

            _ => InvalidTransaction::Call.into(),
        }
    }
}
