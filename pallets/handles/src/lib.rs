//! # Handles Pallet
//! The Handles pallet provides functionality for generating user handles.
//!
//! - [Configuration: `Config`](Config)
//! - [Extrinsics: `Call`](Call)
//! - [Event Enum: `Event`](Event)
//! - [Error Enum: `Error`](Error)
//!
//! ## Overview
//!
//! ## Features
//!
//! * Handle creation and retirement
//! * Shuffled sequence generation for handle suffixes
//! * Homoglyph detection
//!
//! ## Terminology
//! - Handle
//! - Suffix
//! - PRNG

// Ensure we're `no_std` when compiling for WASM.
#![cfg_attr(not(feature = "std"), no_std)]
// Strong Documentation Lints
#![deny(
	rustdoc::broken_intra_doc_links,
	rustdoc::missing_crate_level_docs,
	rustdoc::invalid_codeblock_attributes,
	missing_docs
)]

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
#[cfg(test)]
mod tests;

// pub mod weights;

use sp_std::prelude::*;

use common_primitives::{handles::*, msa::MessageSourceId};
use frame_support::{dispatch::DispatchResult, pallet_prelude::*, traits::Get};
use frame_system::pallet_prelude::*;
pub use pallet::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Weight information for extrinsics in this pallet.
		// type WeightInfo: WeightInfo;

		/// The minimum suffix value
		#[pallet::constant]
		type HandleSuffixMin: Get<u32>;

		/// The maximum suffix value
		#[pallet::constant]
		type HandleSuffixMax: Get<u32>;
	}

	// Simple declaration of the `Pallet` type. It is placeholder we use to implement traits and
	// method.
	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[derive(PartialEq, Eq)] // for testing
	#[pallet::error]
	pub enum Error<T> {
		/// Suffixes exhausted
		SuffixesExhausted,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub (super) fn deposit_event)]
	pub enum Event<T: Config> {
		/// Emitted when a schema is registered. [who, schemas id]
		HandleCreated {
			/// message source id of handle owner
			msa_id: MessageSourceId,
		},
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Claim handle
		///
		#[pallet::call_index(0)]
		#[pallet::weight(1000)]
		pub fn claim_handle(origin: OriginFor<T>, base_name: Vec<u8>) -> DispatchResult {
			ensure_signed(origin)?;
			if (base_name.len() as u32) < HANDLE_BASE_MIN ||
				(base_name.len() as u32) > HANDLE_BASE_MAX
			{
				()
			}
			Self::deposit_event(Event::HandleCreated { msa_id: 1 });
			Ok(())
		}
	}
}
