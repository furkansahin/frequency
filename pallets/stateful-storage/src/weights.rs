// This file is part of Substrate.

// Copyright (C) 2022 Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_stateful_storage
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-11-09, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-runner-44wtw-znkzv`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_stateful-storage
// --extrinsic
// *
// --chain=frequency-bench
// --heap-pages=4096
// --wasm-execution=compiled
// --additional-trie-layers=20
// --steps=20
// --repeat=10
// --output=./scripts/../pallets/stateful-storage/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_stateful_storage.
pub trait WeightInfo {
	fn apply_item_actions(s: u32, ) -> Weight;
	fn upsert_page(s: u32, ) -> Weight;
	fn delete_page() -> Weight;
	fn apply_item_actions_with_signature(s: u32, ) -> Weight;
	fn upsert_page_with_signature(s: u32, ) -> Weight;
	fn delete_page_with_signature() -> Weight;
}

/// Weights for pallet_stateful_storage using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Schemas::Schemas` (r:1 w:0)
	/// Proof: `Schemas::Schemas` (`max_values`: None, `max_size`: Some(65518), added: 67993, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// The range of component `s` is `[1, 5121]`.
	fn apply_item_actions(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `33370`
		//  Estimated: `77893`
		// Minimum execution time: 97_406_000 picoseconds.
		Weight::from_parts(96_006_676, 77893)
			// Standard Error: 336
			.saturating_add(Weight::from_parts(7_490, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Schemas::Schemas` (r:1 w:0)
	/// Proof: `Schemas::Schemas` (`max_values`: None, `max_size`: Some(65518), added: 67993, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// The range of component `s` is `[1, 1024]`.
	fn upsert_page(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `416`
		//  Estimated: `77893`
		// Minimum execution time: 30_102_000 picoseconds.
		Weight::from_parts(31_172_924, 77893)
			// Standard Error: 194
			.saturating_add(Weight::from_parts(1_216, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Schemas::Schemas` (r:1 w:0)
	/// Proof: `Schemas::Schemas` (`max_values`: None, `max_size`: Some(65518), added: 67993, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn delete_page() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1575`
		//  Estimated: `77893`
		// Minimum execution time: 36_093_000 picoseconds.
		Weight::from_parts(36_918_000, 77893)
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::Schemas` (r:1 w:0)
	/// Proof: `Schemas::Schemas` (`max_values`: None, `max_size`: Some(65518), added: 67993, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// The range of component `s` is `[1, 5121]`.
	fn apply_item_actions_with_signature(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `33377`
		//  Estimated: `77893`
		// Minimum execution time: 164_530_000 picoseconds.
		Weight::from_parts(161_735_871, 77893)
			// Standard Error: 458
			.saturating_add(Weight::from_parts(13_990, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::Schemas` (r:1 w:0)
	/// Proof: `Schemas::Schemas` (`max_values`: None, `max_size`: Some(65518), added: 67993, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// The range of component `s` is `[1, 1024]`.
	fn upsert_page_with_signature(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `349`
		//  Estimated: `77893`
		// Minimum execution time: 84_778_000 picoseconds.
		Weight::from_parts(90_533_245, 77893)
			// Standard Error: 827
			.saturating_add(Weight::from_parts(7_478, 0).saturating_mul(s.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::Schemas` (r:1 w:0)
	/// Proof: `Schemas::Schemas` (`max_values`: None, `max_size`: Some(65518), added: 67993, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn delete_page_with_signature() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1508`
		//  Estimated: `77893`
		// Minimum execution time: 90_748_000 picoseconds.
		Weight::from_parts(93_095_000, 77893)
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `Schemas::Schemas` (r:1 w:0)
	/// Proof: `Schemas::Schemas` (`max_values`: None, `max_size`: Some(65518), added: 67993, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// The range of component `s` is `[1, 5121]`.
	fn apply_item_actions(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `33370`
		//  Estimated: `77893`
		// Minimum execution time: 97_406_000 picoseconds.
		Weight::from_parts(96_006_676, 77893)
			// Standard Error: 336
			.saturating_add(Weight::from_parts(7_490, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Schemas::Schemas` (r:1 w:0)
	/// Proof: `Schemas::Schemas` (`max_values`: None, `max_size`: Some(65518), added: 67993, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// The range of component `s` is `[1, 1024]`.
	fn upsert_page(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `416`
		//  Estimated: `77893`
		// Minimum execution time: 30_102_000 picoseconds.
		Weight::from_parts(31_172_924, 77893)
			// Standard Error: 194
			.saturating_add(Weight::from_parts(1_216, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Schemas::Schemas` (r:1 w:0)
	/// Proof: `Schemas::Schemas` (`max_values`: None, `max_size`: Some(65518), added: 67993, mode: `MaxEncodedLen`)
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Msa::DelegatorAndProviderToDelegation` (r:1 w:0)
	/// Proof: `Msa::DelegatorAndProviderToDelegation` (`max_values`: None, `max_size`: Some(217), added: 2692, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn delete_page() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1575`
		//  Estimated: `77893`
		// Minimum execution time: 36_093_000 picoseconds.
		Weight::from_parts(36_918_000, 77893)
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::Schemas` (r:1 w:0)
	/// Proof: `Schemas::Schemas` (`max_values`: None, `max_size`: Some(65518), added: 67993, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0xbd1557c8db6bd8599a811a7175fbc2fc6400` (r:1 w:1)
	/// The range of component `s` is `[1, 5121]`.
	fn apply_item_actions_with_signature(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `33377`
		//  Estimated: `77893`
		// Minimum execution time: 164_530_000 picoseconds.
		Weight::from_parts(161_735_871, 77893)
			// Standard Error: 458
			.saturating_add(Weight::from_parts(13_990, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::Schemas` (r:1 w:0)
	/// Proof: `Schemas::Schemas` (`max_values`: None, `max_size`: Some(65518), added: 67993, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// The range of component `s` is `[1, 1024]`.
	fn upsert_page_with_signature(s: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `349`
		//  Estimated: `77893`
		// Minimum execution time: 84_778_000 picoseconds.
		Weight::from_parts(90_533_245, 77893)
			// Standard Error: 827
			.saturating_add(Weight::from_parts(7_478, 0).saturating_mul(s.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Msa::PublicKeyToMsaId` (r:1 w:0)
	/// Proof: `Msa::PublicKeyToMsaId` (`max_values`: None, `max_size`: Some(48), added: 2523, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::Schemas` (r:1 w:0)
	/// Proof: `Schemas::Schemas` (`max_values`: None, `max_size`: Some(65518), added: 67993, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	/// Proof: UNKNOWN KEY `0x0763c98381dc89abe38627fe2f98cb7af1577fbf1d628fdddb4ebfc6e8d95fb1` (r:1 w:1)
	fn delete_page_with_signature() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1508`
		//  Estimated: `77893`
		// Minimum execution time: 90_748_000 picoseconds.
		Weight::from_parts(93_095_000, 77893)
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
