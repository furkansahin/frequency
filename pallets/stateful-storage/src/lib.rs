//! # Stateful Storage Pallet
//! The Stateful Storage pallet provides functionality for reading and writing stateful data
//! representing stateful data for which we are only ever interested in the latest state.
//!
//! ## Overview
//! For state transitions for which we only care about the latest state, Stateful Storage provides a way to store and retrieve such data
//! outside of the existing Announcement mechanism, which would require the latest state to be tracked using some kind of 3rd-party indexer.
//!
//! This pallet supports two models for storing stateful data:
//! 1. **Itemized:** Data is stored in a single **page** (max size: `MaxItemizedPageSizeBytes`) containing multiple items (max item size `MaxItemizedBlobSizeBytes`) of the associated schema.
//! Useful for schemas with a relative small item size and higher potential item count.
//! 2. **Paginated:** Data is stored in multiple **pages** of size `MaxPaginatedPageSizeBytes`, each containing a single item of the associated schema.
//! Page IDs range from 0 .. `MaxPaginatedPageId` (implying there may be at most `MaxPaginatedPageId` + 1 pages per MSA+Schema at any given time, though
//! there may be holes in that range). Useful for schemas with a larger item size and smaller potential item count.
//!
//! ## Features
//! * Provide for storage of stateful data with flexible schemas on-chain.
//! * Data stored for one MSA does not have impact read/write access times or storage costs for any data stored for another MSA.
//! * High write throughput.
//! * High read throughput.
//! * Data race condition protection
//!
//! The Stateful Storage pallet provides functions for:
//!
//! - Appending items in an **itemized** model
//! - Removing items in an **itemized** model
//! - Upserting items in a **paginated** model
//! - Removing pages in a **paginated**  model
//!
//! ## Terminology
//! * **Item:** Data payload mapping to a defined schema
//! * **Page:** Block of on-chain data of a fixed size, containing one (Paginated model) or more (Itemized model) **items**.
//!

// Ensure we're `no_std` when compiling for Wasm.
#![cfg_attr(not(feature = "std"), no_std)]
// Strong Documentation Lints
// #![deny(
// 	rustdoc::broken_intra_doc_links,
// 	rustdoc::missing_crate_level_docs,
// 	rustdoc::invalid_codeblock_attributes,
// 	missing_docs
// )]

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;
use codec::{Decode, Encode};
#[cfg(feature = "runtime-benchmarks")]
use common_primitives::benchmarks::{MsaBenchmarkHelper, SchemaBenchmarkHelper};
use sp_std::prelude::*;

mod stateful_child_tree;
pub mod types;

pub mod weights;

use crate::{
	stateful_child_tree::{StatefulChildTree, StatefulPageKeyPart},
	types::*,
};
use common_primitives::{
	msa::{DelegatorId, MessageSourceId, MsaValidator, ProviderId, SchemaGrantValidator},
	schema::{PayloadLocation, SchemaId, SchemaProvider},
	stateful_storage::{
		ItemizedStoragePageResponse, ItemizedStorageResponse, PageId, PaginatedStorageResponse,
	},
};
use frame_support::{dispatch::DispatchResult, ensure, traits::Get};
pub use pallet::*;
use sp_runtime::DispatchError;
pub use weights::*;

#[frame_support::pallet]
pub mod pallet {
	use super::*;
	use crate::{
		stateful_child_tree::{StatefulChildTree, StatefulPageKeyPart},
		types::ItemAction,
	};
	use common_primitives::{
		msa::{MessageSourceId, MsaLookup, MsaValidator, SchemaGrantValidator},
		schema::{SchemaId, SchemaProvider},
		stateful_storage::PageId,
	};
	use frame_support::pallet_prelude::*;
	use frame_system::pallet_prelude::*;

	#[pallet::config]
	pub trait Config: frame_system::Config {
		/// The overarching event type.
		type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

		/// Weight information for extrinsics in this pallet.
		type WeightInfo: WeightInfo;

		/// A type that will supply MSA related information
		type MsaInfoProvider: MsaLookup + MsaValidator<AccountId = Self::AccountId>;

		/// A type that will validate schema grants
		type SchemaGrantValidator: SchemaGrantValidator<Self::BlockNumber>;

		/// A type that will supply schema related information.
		type SchemaProvider: SchemaProvider<SchemaId>;

		/// The maximum size of a page (in bytes) for an Itemized storage model
		#[pallet::constant]
		type MaxItemizedPageSizeBytes: Get<u32> + Default;

		/// The maximum size of a page (in bytes) for a Paginated storage model
		#[pallet::constant]
		type MaxPaginatedPageSizeBytes: Get<u32>;

		/// The maximum size of a single item in an itemized storage model (in bytes)
		#[pallet::constant]
		type MaxItemizedBlobSizeBytes: Get<u32>;

		/// The maximum number of pages in a Paginated storage model
		#[pallet::constant]
		type MaxPaginatedPageId: Get<u32>;

		/// The maximum number of actions in itemized actions
		#[pallet::constant]
		type MaxItemizedActionsCount: Get<u32>;

		#[cfg(feature = "runtime-benchmarks")]
		/// A set of helper functions for benchmarking.
		type MsaBenchmarkHelper: MsaBenchmarkHelper<Self::AccountId>;

		#[cfg(feature = "runtime-benchmarks")]
		/// A set of helper functions for benchmarking.
		type SchemaBenchmarkHelper: SchemaBenchmarkHelper;
	}

	// Simple declaration of the `Pallet` type. It is placeholder we use to implement traits and
	// method.
	#[pallet::pallet]
	#[pallet::generate_store(pub (super) trait Store)]
	pub struct Pallet<T>(_);

	#[pallet::error]
	pub enum Error<T> {
		/// Item payload exceeds max item blob size
		ItemExceedsMaxBlobSizeBytes,

		/// Page would exceed the highest allowable PageId
		PageIdExceedsMaxAllowed,

		/// Page size exceeds max allowable page size
		PageExceedsMaxPageSizeBytes,

		/// Invalid SchemaId or Schema not found
		InvalidSchemaId,

		/// Schema is not valid for storage model
		SchemaPayloadLocationMismatch,

		/// Invalid Message Source Account
		InvalidMessageSourceAccount,

		/// UnAuthorizedDelegate
		UnAuthorizedDelegate,

		/// Corrupted State
		CorruptedState,

		/// Invalid item action
		InvalidItemAction,
	}

	#[pallet::event]
	#[pallet::generate_deposit(pub(super) fn deposit_event)]
	pub enum Event<T: Config> {
		ItemizedPageUpdated { msa_id: MessageSourceId, schema_id: SchemaId },
		ItemizedPageDeleted { msa_id: MessageSourceId, schema_id: SchemaId },
		PaginatedPageUpdated { msa_id: MessageSourceId, schema_id: SchemaId, page_id: PageId },
		PaginatedPageDeleted { msa_id: MessageSourceId, schema_id: SchemaId, page_id: PageId },
	}

	#[pallet::call]
	impl<T: Config> Pallet<T> {
		/// Applies the Add or Delete Actions on the requested Itemized page
		#[pallet::call_index(0)]
		#[pallet::weight(T::WeightInfo::apply_item_actions( actions.len() as u32 ,
			actions.iter().fold(0, |acc, a| acc + match a {
				ItemAction::Add { data } => data.len() as u32,
				_ => 0,
			})
		))]
		pub fn apply_item_actions(
			origin: OriginFor<T>,
			#[pallet::compact] state_owner_msa_id: MessageSourceId,
			#[pallet::compact] schema_id: SchemaId,
			actions: BoundedVec<ItemAction, T::MaxItemizedActionsCount>,
		) -> DispatchResult {
			let provider_key = ensure_signed(origin)?;
			ensure!(
				actions.as_slice().iter().all(|a| match a {
					ItemAction::Add { data } =>
						data.len() <= T::MaxItemizedBlobSizeBytes::get() as usize,
					_ => true,
				}),
				Error::<T>::ItemExceedsMaxBlobSizeBytes
			);

			Self::check_schema_and_grants(
				provider_key,
				state_owner_msa_id,
				schema_id,
				PayloadLocation::Itemized,
			)?;

			let storage_key = &schema_id.encode()[..];
			let keys = vec![storage_key.to_vec()];
			let updated_page =
				StatefulChildTree::try_read::<ItemizedPage<T>>(&state_owner_msa_id, &keys)
					.map_err(|_| {
						log::warn!(
							"failed decoding Itemized msa={:?} schema_id={:?}",
							state_owner_msa_id,
							schema_id
						);
						Error::<T>::CorruptedState
					})?
					.unwrap_or_default()
					.apply_item_actions(&actions[..])
					.map_err(|_| Error::<T>::InvalidItemAction)?;

			match updated_page.is_empty() {
				true => {
					StatefulChildTree::kill(&state_owner_msa_id, &keys);
					Self::deposit_event(Event::ItemizedPageDeleted {
						msa_id: state_owner_msa_id,
						schema_id,
					});
				},
				false => {
					StatefulChildTree::write(&state_owner_msa_id, &keys, updated_page);
					Self::deposit_event(Event::ItemizedPageUpdated {
						msa_id: state_owner_msa_id,
						schema_id,
					});
				},
			};
			Ok(())
		}

		/// Creates or updates an Paginated storage with new payload
		#[pallet::call_index(1)]
		#[pallet::weight(T::WeightInfo::upsert_page(payload.len() as u32))]
		pub fn upsert_page(
			origin: OriginFor<T>,
			#[pallet::compact] state_owner_msa_id: MessageSourceId,
			#[pallet::compact] schema_id: SchemaId,
			#[pallet::compact] page_id: PageId,
			payload: Vec<u8>,
		) -> DispatchResult {
			let provider_key = ensure_signed(origin)?;
			let page = PaginatedPage::<T>::try_from(payload)
				.map_err(|_| Error::<T>::PageExceedsMaxPageSizeBytes)?;
			ensure!(
				page_id as u32 <= T::MaxPaginatedPageId::get(),
				Error::<T>::PageIdExceedsMaxAllowed
			);

			Self::check_schema_and_grants(
				provider_key,
				state_owner_msa_id,
				schema_id,
				PayloadLocation::Paginated,
			)?;

			let schema_key: StatefulPageKeyPart = schema_id.encode();
			let page_key: StatefulPageKeyPart = page_id.encode();

			StatefulChildTree::write(&state_owner_msa_id, &[schema_key, page_key], page);
			Self::deposit_event(Event::PaginatedPageUpdated {
				msa_id: state_owner_msa_id,
				schema_id,
				page_id,
			});
			Ok(())
		}

		/// Deletes a Paginated storage
		#[pallet::call_index(2)]
		#[pallet::weight(T::WeightInfo::delete_page())]
		pub fn delete_page(
			origin: OriginFor<T>,
			#[pallet::compact] state_owner_msa_id: MessageSourceId,
			#[pallet::compact] schema_id: SchemaId,
			#[pallet::compact] page_id: PageId,
		) -> DispatchResult {
			let provider_key = ensure_signed(origin)?;
			ensure!(
				page_id as u32 <= T::MaxPaginatedPageId::get(),
				Error::<T>::PageIdExceedsMaxAllowed
			);
			Self::check_schema_and_grants(
				provider_key,
				state_owner_msa_id,
				schema_id,
				PayloadLocation::Paginated,
			)?;

			let schema_key = schema_id.encode();
			let page_key = page_id.encode();

			StatefulChildTree::kill(&state_owner_msa_id, &[schema_key, page_key]);
			Self::deposit_event(Event::PaginatedPageDeleted {
				msa_id: state_owner_msa_id,
				schema_id,
				page_id,
			});
			Ok(())
		}
	}
}

impl<T: Config> Pallet<T> {
	/// This function returns all the paginated storages associated with `msa_id` and `schema_id`
	///
	/// [Warning] since this function iterates over all the potential keys it should never called
	/// from runtime.
	pub fn get_paginated_storages(
		msa_id: MessageSourceId,
		schema_id: SchemaId,
	) -> Result<Vec<PaginatedStorageResponse>, DispatchError> {
		let schema =
			T::SchemaProvider::get_schema_by_id(schema_id).ok_or(Error::<T>::InvalidSchemaId)?;
		ensure!(
			schema.payload_location == PayloadLocation::Paginated,
			Error::<T>::SchemaPayloadLocationMismatch
		);
		let keys: Vec<StatefulPageKeyPart> = vec![schema_id.encode().to_vec()];
		Ok(StatefulChildTree::prefix_iterator::<PaginatedPage<T>>(&msa_id, &keys[..])
			.filter_map(|(key, v)| match <PageId>::decode(&mut &key[..]) {
				Ok(k) =>
					Some(PaginatedStorageResponse::new(k, msa_id, schema_id, v.data.into_inner())),
				Err(_) => {
					log::debug!(
						"error parsing pageId from key {:?} for msa_id {} and schema_id {}",
						key,
						msa_id,
						schema_id
					);
					None
				},
			})
			.collect())
	}

	/// This function returns all the itemized storages associated with `msa_id` and `schema_id`
	pub fn get_itemized_storages(
		msa_id: MessageSourceId,
		schema_id: SchemaId,
	) -> Result<ItemizedStoragePageResponse, DispatchError> {
		let schema =
			T::SchemaProvider::get_schema_by_id(schema_id).ok_or(Error::<T>::InvalidSchemaId)?;
		ensure!(
			schema.payload_location == PayloadLocation::Itemized,
			Error::<T>::SchemaPayloadLocationMismatch
		);
		let keys: Vec<StatefulPageKeyPart> = vec![schema_id.encode().to_vec()];
		let items: Vec<ItemizedStorageResponse> =
			StatefulChildTree::try_read::<ItemizedPage<T>>(&msa_id, &keys[..])
				.map_err(|_| Error::<T>::CorruptedState)?
				.unwrap_or_default()
				.parse_as_itemized(false)
				.map_err(|_| Error::<T>::CorruptedState)?
				.items
				.iter()
				.map(|(key, v)| ItemizedStorageResponse::new(*key, v.to_vec()))
				.collect();
		Ok(ItemizedStoragePageResponse::new(msa_id, schema_id, items))
	}

	fn check_schema_and_grants(
		provider_key: T::AccountId,
		state_owner_msa_id: MessageSourceId,
		schema_id: SchemaId,
		payload_location: PayloadLocation,
	) -> DispatchResult {
		let provider_msa_id = T::MsaInfoProvider::ensure_valid_msa_key(&provider_key)
			.map_err(|_| Error::<T>::InvalidMessageSourceAccount)?;
		let schema =
			T::SchemaProvider::get_schema_by_id(schema_id).ok_or(Error::<T>::InvalidSchemaId)?;
		ensure!(
			schema.payload_location == payload_location,
			Error::<T>::SchemaPayloadLocationMismatch
		);

		let current_block = frame_system::Pallet::<T>::block_number();
		Ok(T::SchemaGrantValidator::ensure_valid_schema_grant(
			ProviderId(provider_msa_id),
			DelegatorId(state_owner_msa_id),
			schema_id,
			current_block,
		)
		.map_err(|_| Error::<T>::UnAuthorizedDelegate)?)
	}

	#[cfg(feature = "runtime-benchmarks")]
	pub fn get_itemized_page(
		msa_id: MessageSourceId,
		schema_id: SchemaId,
	) -> Option<ItemizedPage<T>> {
		let storage_key = &schema_id.encode()[..];
		let keys = vec![storage_key.to_vec()];
		StatefulChildTree::try_read::<ItemizedPage<T>>(&msa_id, &keys).unwrap_or(None)
	}

	#[cfg(feature = "runtime-benchmarks")]
	pub fn get_paginated_page(
		msa_id: MessageSourceId,
		schema_id: SchemaId,
		page_id: PageId,
	) -> Option<PaginatedPage<T>> {
		let schema_key: StatefulPageKeyPart = schema_id.encode();
		let page_key: StatefulPageKeyPart = page_id.encode();
		StatefulChildTree::try_read::<PaginatedPage<T>>(&msa_id, &[schema_key, page_key])
			.unwrap_or(None)
	}
}