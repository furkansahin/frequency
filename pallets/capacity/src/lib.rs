//! # Capacity Pallet
//! The Capacity pallet provides functionality for staking tokens to the Frequency network.
//!
//! - [Configuration: `Config`](Config)
//! - [Extrinsics: `Call`](Call)
//! - [Event Enum: `Event`](Event)
//! - [Error Enum: `Error`](Error)
//!
//! ## Overview
//! Capacity is a refillable resource that can be used to make transactions on the network.
//! Tokens may be staked to the network targeting a provider MSA account to which the network will grant Capacity.
//!
//! The network generates Capacity based on a Capacity-generating function that considers usage and network congestion.
//! When token is staked, the targeted provider MSA receives the Capacity generated.
//!
//! The staked amount may be increased, targeting either the same or a different target to receive the newly generated Capacity.
//! As a result, every time the network is staked to, the staked tokens are locked until unstaked.
//!
//! Unstaking schedules some amount of token to be unlocked. There is no limit on the amount of token that can be unstaked.
//! However, there is a a limit on how many concurrently scheduled unstaking requests can exist.
//! After scheduling tokens to be unlocked, they must undergo a thaw period before being withdrawn.
//!
//! After thawing, the tokens may be withdrawn using the withdraw_unstaked extrinsic.
//! On success, the tokens are unlocked and free up space to submit more unstaking request.
//!
//! The MSA pallet provides functions for:
//!
//! - staking and, updating,
//!
//! ## Terminology
//! * **Staker:** [insert description].
//! * **Target** [insert description].
//! * **Epoch Period:** [insert description here].
//! * **Capacity:** [insert description here].
//! * **Replenishable:** [insert description here].
//!

#![cfg_attr(not(feature = "std"), no_std)]
// Strong Documentation Lints
#![deny(
	rustdoc::broken_intra_doc_links,
	rustdoc::missing_crate_level_docs,
	rustdoc::invalid_codeblock_attributes,
	missing_docs
)]

use frame_support::{
	dispatch::DispatchResult,
	ensure,
	traits::{Currency, Get, Hooks, LockIdentifier, LockableCurrency, WithdrawReasons},
	weights::{constants::RocksDbWeight, Weight},
};

use sp_runtime::{
	traits::{CheckedAdd, Zero},
	ArithmeticError, DispatchError, Perbill, Saturating,
};

pub use common_primitives::{
	capacity::TargetValidator, msa::MessageSourceId, utils::wrap_binary_data,
};

#[cfg(feature = "runtime-benchmarks")]
use common_primitives::benchmarks::RegisterProviderBenchmarkHelper;

pub use pallet::*;
pub use types::*;
pub use weights::*;
pub mod types;

#[cfg(test)]
pub mod testing_utils;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[cfg(test)]
mod extrinsics_tests;
#[cfg(test)]
mod helpers_tests;
#[cfg(test)]
mod mock;

#[cfg(test)]
mod types_tests;

pub mod weights;

type BalanceOf<T> =
	<<T as Config>::Currency as Currency<<T as frame_system::Config>::AccountId>>::Balance;

const STAKING_ID: LockIdentifier = *b"netstkng";
#[frame_support::pallet]
pub mod pallet {
	use super::*;

	use frame_support::{pallet_prelude::*, Twox64Concat};
	use frame_system::pallet_prelude::*;
	use sp_runtime::traits::{AtLeast32BitUnsigned, MaybeDisplay};

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		/// Function that allows a balance to be locked.
		type Currency: LockableCurrency<Self::AccountId, Moment = Self::BlockNumber>;

		/// Function that checks if an MSA is a valid target.
		type TargetValidator: TargetValidator;

		/// The minimum required token amount to stake. It facilitates cleaning dust when unstaking.
		#[pallet::constant]
		type MinimumStakingAmount: Get<BalanceOf<Self>>;

		/// The maximum number of unlocking chunks a StakingAccountLedger can have. It determines how many concurrent unstaked chunks may exist.
		#[pallet::constant]
		type MaxUnlockingChunks: Get<u32>;

		#[cfg(feature = "runtime-benchmarks")]
		/// A set of helper functions for benchmarking.
		type BenchmarkHelper: RegisterProviderBenchmarkHelper;

		/// The number of Epochs before you can unlock tokens after unstaking.
		#[pallet::constant]
		type UnstakingThawPeriod: Get<u16>;

		/// Maximum number of blocks an epoch can be
		/// currently used as the actual value of epoch length.
		#[pallet::constant]
		type MaxEpochLength: Get<Self::BlockNumber>;

		/// A type that provides an Epoch number
		/// traits pulled from frame_system::Config::BlockNumber
		type EpochNumber: Parameter
			+ Member
			+ MaybeSerializeDeserialize
			+ MaybeDisplay
			+ AtLeast32BitUnsigned
			+ Default
			+ Copy
			+ sp_std::hash::Hash
			+ MaxEncodedLen
			+ TypeInfo;
	}

	/// Storage for keeping a ledger of staked token amounts for accounts.
	/// - Keys: AccountId
	/// - Value: [`StakingAccountDetails`](types::StakingAccountDetails)
	#[pallet::storage]
	#[pallet::getter(fn get_staking_account_for)]
	pub type StakingAccountLedger<T: Config> =
		StorageMap<_, Twox64Concat, T::AccountId, StakingAccountDetails<T>>;

	/// Storage to record how many tokens were targeted to an MSA.
	/// - Keys: AccountId, MSA Id
	/// - Value: [`StakingTargetDetails`](types::StakingTargetDetails)
	#[pallet::storage]
	#[pallet::getter(fn get_target_for)]
	pub type StakingTargetLedger<T: Config> = StorageDoubleMap<
		_,
		Twox64Concat,
		T::AccountId,
		Twox64Concat,
		MessageSourceId,
		StakingTargetDetails<BalanceOf<T>>,
	>;

	/// Storage for target Capacity usage.
	/// - Keys: MSA Id
	/// - Value: [`CapacityDetails`](types::CapacityDetails)
	#[pallet::storage]
	#[pallet::getter(fn get_capacity_for)]
	pub type CapacityLedger<T: Config> =
		StorageMap<_, Twox64Concat, MessageSourceId, CapacityDetails<BalanceOf<T>, T::BlockNumber>>;

	/// Storage for the current epoch number
	#[pallet::storage]
	#[pallet::whitelist_storage]
	#[pallet::getter(fn get_current_epoch)]
	pub type CurrentEpoch<T: Config> = StorageValue<_, T::EpochNumber, ValueQuery>;

	/// Storage for the current epoch info
	#[pallet::storage]
	#[pallet::getter(fn get_current_epoch_info)]
	pub type CurrentEpochInfo<T: Config> = StorageValue<_, EpochInfo<T::BlockNumber>, ValueQuery>;

	/// Storage for the current epoch number
	#[pallet::storage]
	#[pallet::getter(fn get_current_epoch_used_capacity)]
	pub type CurrentEpochUsedCapacity<T: Config> = StorageValue<_, BalanceOf<T>, ValueQuery>;

	// Simple declaration of the `Pallet` type. It is placeholder we use to implement traits and
	// method.
	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Tokens have been staked to the Frequency network.
		Staked {
			/// The token account that staked tokens to the network.
			account: T::AccountId,
			/// The MSA that a token account targeted to receive Capacity based on this staking amount.
			target: MessageSourceId,
			/// An amount that was staked.
			amount: BalanceOf<T>,
			/// The Capacity amount issued to the target as a result of the stake.
			capacity: BalanceOf<T>,
		},
		/// Unsstaked token that has thawed was unlocked for the given account
		StakeWithdrawn {
			/// the account that withdrew its stake
			account: T::AccountId,
			/// the total amount withdrawn, i.e. put back into free balance.
			amount: BalanceOf<T>,
		},
		/// A token account has unstaked the Frequency network.
		UnStaked {
			/// The token account that staked tokens to the network.
			account: T::AccountId,
			/// The MSA that a token account targeted to receive Capacity to unstake from.
			target: MessageSourceId,
			/// An amount that was unstaked.
			amount: BalanceOf<T>,
			/// The Capacity amount that was reduced from a target.
			capacity: BalanceOf<T>,
		},
	}

	#[pallet::error]
	pub enum Error<T> {
		/// Staker attempted to stake to an invalid staking target.
		InvalidTarget,
		/// Staker has insufficient balance to cover the amount wanting to stake.
		InsufficientBalance,
		/// Staker is attempting to stake an amount below the minimum amount.
		InsufficientStakingAmount,
		/// Staker is attempting to stake a zero amount.
		ZeroAmountNotAllowed,
		/// Origin has no Staking Account
		NotAStakingAccount,
		/// No staked value is available for withdrawal; either nothing is being unstaked,
		/// or nothing has passed the thaw period.
		NoUnstakedTokensAvailable,
		/// Unstaking amount should be greater than zero.
		UnstakedAmountIsZero,
		/// Amount to unstake is greater than the amount staked.
		AmountToUnstakeExceedsAmountStaked,
		/// Attempting to unstake from a target that has not been staked to.
		StakingAccountNotFound,
		/// Attempting to get a staker / target relationship that does not exist.
		StakerTargetRelationshipNotFound,
		/// Attempting to get the target's capacity that does not exist.
		TargetCapacityNotFound,
		/// Staker reached the limit number for the allowed amount of unlocking chunks.
		MaxUnlockingChunksExceeded,
	}

	#[pallet::hooks]
	impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
		fn on_initialize(current: T::BlockNumber) -> Weight {
			Self::start_new_epoch_if_needed(current)
		}
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Stakes some amount of tokens to the network and generates Capacity.
		///
		/// ### Errors
		///
		/// - Returns Error::ZeroAmountNotAllowed if the staker is attempting to stake a zero amount.
		/// - Returns Error::InvalidTarget if attempting to stake to an invalid target.
		/// - Returns Error::InsufficientStakingAmount if attempting to stake an amount below the minimum amount.
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::stake())]
		pub fn stake(
			origin: OriginFor<T>,
			target: MessageSourceId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let staker = ensure_signed(origin)?;

			let (mut staking_account, actual_amount) =
				Self::ensure_can_stake(&staker, target, amount)?;

			let capacity = Self::increase_stake_and_issue_capacity(
				&staker,
				&mut staking_account,
				target,
				actual_amount,
			)?;

			Self::deposit_event(Event::Staked {
				account: staker,
				amount: actual_amount,
				target,
				capacity,
			});

			Ok(())
		}

		/// removes all thawed UnlockChunks from caller's StakingAccount and unlocks the sum of the thawed values
		/// in the caller's token account.
		///
		/// ### Errors
		///   - Returns `Error::NotAStakingAccount` if no StakingAccountDetails are found for `origin`.
		///   - Returns `Error::NoUnstakedTokensAvailable` if the account has no unstaking chunks or none are thawed.
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::withdraw_unstaked())]
		pub fn withdraw_unstaked(origin: OriginFor<T>) -> DispatchResult {
			let staker = ensure_signed(origin)?;

			let mut staking_account =
				Self::get_staking_account_for(&staker).ok_or(Error::<T>::NotAStakingAccount)?;
			let current_block = frame_system::Pallet::<T>::block_number();

			let amount_withdrawn = staking_account.reap_thawed(current_block);
			ensure!(!amount_withdrawn.is_zero(), Error::<T>::NoUnstakedTokensAvailable);

			Self::update_or_delete_staking_account(&staker, &mut staking_account);
			Self::deposit_event(Event::<T>::StakeWithdrawn {
				account: staker,
				amount: amount_withdrawn,
			});
			Ok(())
		}

		/// Schedules an amount of the stake to be unlocked.
		/// ### Errors
		///
		/// - Returns `Error::UnstakedAmountIsZero` if `amount` is not greater than zero.
		/// - Returns `Error::MaxUnlockingChunksExceeded` if attempting to unlock more times than config::MaxUnlockingChunks.
		/// - Returns `Error::AmountToUnstakeExceedsAmountStaked` if `amount` exceeds the amount currently staked.
		/// - Returns `Error::InvalidTarget` if `target` is not a valid staking target
		/// - Returns `Error:: NotAStakingAccount` if `origin` has nothing staked
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::unstake())]
		pub fn unstake(
			origin: OriginFor<T>,
			target: MessageSourceId,
			amount: BalanceOf<T>,
		) -> DispatchResult {
			let unstaker = ensure_signed(origin)?;

			ensure!(T::TargetValidator::validate(target), Error::<T>::InvalidTarget);
			ensure!(amount > Zero::zero(), Error::<T>::UnstakedAmountIsZero);

			Self::decrease_active_staking_balance(&unstaker, amount)?;
			let capacity_reduction = Self::reduce_capacity(&unstaker, target, amount)?;

			Self::deposit_event(Event::UnStaked {
				account: unstaker,
				target,
				amount,
				capacity: capacity_reduction,
			});
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// Checks to see if staker has sufficient free-balance to stake the minimum required staking amount.
	///
	/// # Errors
	/// * [`Error::ZeroAmountNotAllowed`]
	/// * [`Error::InvalidTarget`]
	///
	fn ensure_can_stake(
		staker: &T::AccountId,
		target: MessageSourceId,
		amount: BalanceOf<T>,
	) -> Result<(StakingAccountDetails<T>, BalanceOf<T>), DispatchError> {
		ensure!(amount > Zero::zero(), Error::<T>::ZeroAmountNotAllowed);
		ensure!(T::TargetValidator::validate(target), Error::<T>::InvalidTarget);

		let staking_account = Self::get_staking_account_for(&staker).unwrap_or_default();
		let stakable_amount = staking_account.get_stakable_amount_for(&staker, amount);

		let new_active_staking_amount = staking_account
			.active
			.checked_add(&stakable_amount)
			.ok_or(ArithmeticError::Overflow)?;

		ensure!(
			new_active_staking_amount >= T::MinimumStakingAmount::get(),
			Error::<T>::InsufficientStakingAmount
		);

		Ok((staking_account, stakable_amount))
	}

	/// Increase a staking account and target account balances by amount.
	/// Additionally, it issues Capacity to the MSA target.
	fn increase_stake_and_issue_capacity(
		staker: &T::AccountId,
		staking_account: &mut StakingAccountDetails<T>,
		target: MessageSourceId,
		amount: BalanceOf<T>,
	) -> Result<BalanceOf<T>, DispatchError> {
		staking_account.increase_by(amount).ok_or(ArithmeticError::Overflow)?;

		let capacity = Self::calculate_capacity(amount);
		let mut target_details = Self::get_target_for(&staker, &target).unwrap_or_default();
		target_details.increase_by(amount, capacity).ok_or(ArithmeticError::Overflow)?;

		let mut capacity_details = Self::get_capacity_for(target).unwrap_or_default();
		capacity_details
			.increase_by(amount, frame_system::Pallet::<T>::block_number())
			.ok_or(ArithmeticError::Overflow)?;

		Self::set_staking_account(&staker, staking_account);
		Self::set_target_details_for(&staker, target, target_details);
		Self::set_capacity_for(target, capacity_details);

		Ok(capacity)
	}

	/// Sets staking account details.
	fn set_staking_account(staker: &T::AccountId, staking_account: &StakingAccountDetails<T>) {
		T::Currency::set_lock(STAKING_ID, &staker, staking_account.total, WithdrawReasons::all());
		StakingAccountLedger::<T>::insert(staker, staking_account);
	}

	/// Deletes staking account details
	fn delete_staking_account(staker: &T::AccountId) {
		T::Currency::remove_lock(STAKING_ID, &staker);
		StakingAccountLedger::<T>::remove(&staker);
	}

	/// If the staking account total is zero we reap storage, otherwise set the acount to the new details.
	fn update_or_delete_staking_account(
		staker: &<T>::AccountId,
		staking_account: &StakingAccountDetails<T>,
	) {
		if staking_account.total.is_zero() {
			Self::delete_staking_account(&staker);
		} else {
			Self::set_staking_account(&staker, &staking_account)
		}
	}

	/// Sets target account details.
	fn set_target_details_for(
		staker: &T::AccountId,
		target: MessageSourceId,
		target_details: StakingTargetDetails<BalanceOf<T>>,
	) {
		StakingTargetLedger::<T>::insert(staker, target, target_details);
	}

	/// Sets targets Capacity.
	fn set_capacity_for(
		target: MessageSourceId,
		capacity_details: CapacityDetails<BalanceOf<T>, T::BlockNumber>,
	) {
		CapacityLedger::<T>::insert(target, capacity_details);
	}

	/// Calculates Capacity.
	fn calculate_capacity(amount: BalanceOf<T>) -> BalanceOf<T> {
		amount
	}

	/// Decrease a staking account's active token and create an unlocking chunk to be thawed at some future block.
	fn decrease_active_staking_balance(
		unstaker: &T::AccountId,
		amount: BalanceOf<T>,
	) -> DispatchResult {
		let mut staking_account_details =
			Self::get_staking_account_for(unstaker).ok_or(Error::<T>::StakingAccountNotFound)?;
		ensure!(
			amount <= staking_account_details.active,
			Error::<T>::AmountToUnstakeExceedsAmountStaked
		);

		let current_block: T::BlockNumber = frame_system::Pallet::<T>::block_number();
		let thaw_at =
			current_block.saturating_add(T::BlockNumber::from(T::UnstakingThawPeriod::get()));

		staking_account_details.decrease_by(amount, thaw_at)?;
		Self::set_staking_account(&unstaker, &staking_account_details);

		Ok(())
	}

	/// Reduce available capacity of target and return the amount of capacity reduction.
	fn reduce_capacity(
		unstaker: &T::AccountId,
		target: MessageSourceId,
		amount: BalanceOf<T>,
	) -> Result<BalanceOf<T>, DispatchError> {
		let mut staking_target_details = Self::get_target_for(&unstaker, &target)
			.ok_or(Error::<T>::StakerTargetRelationshipNotFound)?;
		let mut capacity_details =
			Self::get_capacity_for(target).ok_or(Error::<T>::TargetCapacityNotFound)?;
		let capacity_reduction = Self::calculate_capacity_reduction(
			amount,
			capacity_details.total_tokens_staked,
			capacity_details.total_available,
		);
		staking_target_details.decrease_by(amount, capacity_reduction);
		capacity_details.decrease_by(capacity_reduction, amount);

		Self::set_capacity_for(target, capacity_details);
		Self::set_target_details_for(unstaker, target, staking_target_details);

		Ok(capacity_reduction)
	}

	/// Determine the capacity reduction when given total_capacity, unstaking_amount, and total_amount_staked.
	fn calculate_capacity_reduction(
		unstaking_amount: BalanceOf<T>,
		total_amount_staked: BalanceOf<T>,
		total_capacity: BalanceOf<T>,
	) -> BalanceOf<T> {
		let rate = Perbill::from_rational(unstaking_amount, total_amount_staked);
		total_capacity.saturating_sub(rate.mul_ceil(total_capacity))
	}

	/// Get current epoch length in blocks.
	fn get_epoch_length() -> T::BlockNumber {
		<T>::MaxEpochLength::get()
	}

	fn start_new_epoch_if_needed(current_block: T::BlockNumber) -> Weight {
		if Self::get_current_epoch_info()
			.epoch_start
			.saturating_add(Self::get_epoch_length())
			.eq(&current_block)
		{
			let current_epoch = Self::get_current_epoch();
			CurrentEpoch::<T>::set(current_epoch.saturating_add(1u32.into()));
			CurrentEpochInfo::<T>::set(EpochInfo { epoch_start: current_block });
			CurrentEpochUsedCapacity::<T>::set(0u32.into());
			T::WeightInfo::on_initialize()
		} else {
			RocksDbWeight::get().reads(1u64)
		}
	}
}