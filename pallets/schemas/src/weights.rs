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

//! Autogenerated weights for pallet_schemas
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2024-03-06, STEPS: `20`, REPEAT: `10`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `benchmark-runner-wc6w8-pv9rm`, CPU: `Intel(R) Xeon(R) Platinum 8375C CPU @ 2.90GHz`
//! EXECUTION: , WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_schemas
// --extrinsic
// *
// --chain=frequency-bench
// --heap-pages=4096
// --wasm-execution=compiled
// --additional-trie-layers=5
// --steps=20
// --repeat=10
// --output=./scripts/../pallets/schemas/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_schemas.
pub trait WeightInfo {
	fn create_schema(m: u32, ) -> Weight;
	fn create_schema_via_governance(m: u32, ) -> Weight;
	fn propose_to_create_schema(m: u32, ) -> Weight;
	fn create_schema_v2(m: u32, ) -> Weight;
	fn create_schema_v3(m: u32, ) -> Weight;
	fn set_max_schema_model_bytes() -> Weight;
	fn create_schema_via_governance_v2(m: u32, ) -> Weight;
	fn propose_to_create_schema_v2(m: u32, ) -> Weight;
	fn propose_to_create_schema_name() -> Weight;
	fn create_schema_name_via_governance() -> Weight;
}

/// Weights for pallet_schemas using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:1 w:0)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:0 w:1)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaPayloads` (r:0 w:1)
	/// Proof: `Schemas::SchemaPayloads` (`max_values`: None, `max_size`: Some(65514), added: 67989, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `2974`
		// Minimum execution time: 15_235_000 picoseconds.
		Weight::from_parts(15_285_000, 2974)
			// Standard Error: 40
			.saturating_add(Weight::from_parts(34_297, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:1 w:0)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:0 w:1)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaPayloads` (r:0 w:1)
	/// Proof: `Schemas::SchemaPayloads` (`max_values`: None, `max_size`: Some(65514), added: 67989, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema_via_governance(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `2974`
		// Minimum execution time: 15_166_000 picoseconds.
		Weight::from_parts(15_283_000, 2974)
			// Standard Error: 43
			.saturating_add(Weight::from_parts(34_327, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[16, 65499]`.
	fn propose_to_create_schema(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `230`
		//  Estimated: `5180`
		// Minimum execution time: 20_977_000 picoseconds.
		Weight::from_parts(9_872_195, 5180)
			// Standard Error: 31
			.saturating_add(Weight::from_parts(3_075, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:1 w:0)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:0 w:1)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaPayloads` (r:0 w:1)
	/// Proof: `Schemas::SchemaPayloads` (`max_values`: None, `max_size`: Some(65514), added: 67989, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema_v2(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `2974`
		// Minimum execution time: 15_351_000 picoseconds.
		Weight::from_parts(15_374_000, 2974)
			// Standard Error: 40
			.saturating_add(Weight::from_parts(34_339, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(3_u64))
	}
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:1 w:0)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaNameToIds` (r:1 w:1)
	/// Proof: `Schemas::SchemaNameToIds` (`max_values`: None, `max_size`: Some(602), added: 3077, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:0 w:1)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaPayloads` (r:0 w:1)
	/// Proof: `Schemas::SchemaPayloads` (`max_values`: None, `max_size`: Some(65514), added: 67989, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema_v3(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `5552`
		// Minimum execution time: 24_266_000 picoseconds.
		Weight::from_parts(4_176_104, 5552)
			// Standard Error: 63
			.saturating_add(Weight::from_parts(34_827, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:0 w:1)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_max_schema_model_bytes() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_365_000 picoseconds.
		Weight::from_parts(6_787_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:1 w:0)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaNameToIds` (r:1 w:1)
	/// Proof: `Schemas::SchemaNameToIds` (`max_values`: None, `max_size`: Some(602), added: 3077, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:0 w:1)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaPayloads` (r:0 w:1)
	/// Proof: `Schemas::SchemaPayloads` (`max_values`: None, `max_size`: Some(65514), added: 67989, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema_via_governance_v2(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `5552`
		// Minimum execution time: 24_342_000 picoseconds.
		Weight::from_parts(4_617_130, 5552)
			// Standard Error: 61
			.saturating_add(Weight::from_parts(34_750, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(3_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[16, 65499]`.
	fn propose_to_create_schema_v2(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `230`
		//  Estimated: `5180`
		// Minimum execution time: 21_224_000 picoseconds.
		Weight::from_parts(9_820_214, 5180)
			// Standard Error: 32
			.saturating_add(Weight::from_parts(3_099, 0).saturating_mul(m.into()))
			.saturating_add(T::DbWeight::get().reads(4_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn propose_to_create_schema_name() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `433`
		//  Estimated: `5383`
		// Minimum execution time: 26_441_000 picoseconds.
		Weight::from_parts(27_274_000, 5383)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(4_u64))
	}
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaNameToIds` (r:1 w:1)
	/// Proof: `Schemas::SchemaNameToIds` (`max_values`: None, `max_size`: Some(602), added: 3077, mode: `MaxEncodedLen`)
	fn create_schema_name_via_governance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `203`
		//  Estimated: `5552`
		// Minimum execution time: 16_683_000 picoseconds.
		Weight::from_parts(17_491_000, 5552)
			.saturating_add(T::DbWeight::get().reads(2_u64))
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:1 w:0)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:0 w:1)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaPayloads` (r:0 w:1)
	/// Proof: `Schemas::SchemaPayloads` (`max_values`: None, `max_size`: Some(65514), added: 67989, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `2974`
		// Minimum execution time: 15_235_000 picoseconds.
		Weight::from_parts(15_285_000, 2974)
			// Standard Error: 40
			.saturating_add(Weight::from_parts(34_297, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:1 w:0)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:0 w:1)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaPayloads` (r:0 w:1)
	/// Proof: `Schemas::SchemaPayloads` (`max_values`: None, `max_size`: Some(65514), added: 67989, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema_via_governance(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `2974`
		// Minimum execution time: 15_166_000 picoseconds.
		Weight::from_parts(15_283_000, 2974)
			// Standard Error: 43
			.saturating_add(Weight::from_parts(34_327, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[16, 65499]`.
	fn propose_to_create_schema(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `230`
		//  Estimated: `5180`
		// Minimum execution time: 20_977_000 picoseconds.
		Weight::from_parts(9_872_195, 5180)
			// Standard Error: 31
			.saturating_add(Weight::from_parts(3_075, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:1 w:0)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:0 w:1)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaPayloads` (r:0 w:1)
	/// Proof: `Schemas::SchemaPayloads` (`max_values`: None, `max_size`: Some(65514), added: 67989, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema_v2(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `2974`
		// Minimum execution time: 15_351_000 picoseconds.
		Weight::from_parts(15_374_000, 2974)
			// Standard Error: 40
			.saturating_add(Weight::from_parts(34_339, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(3_u64))
	}
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:1 w:0)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaNameToIds` (r:1 w:1)
	/// Proof: `Schemas::SchemaNameToIds` (`max_values`: None, `max_size`: Some(602), added: 3077, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:0 w:1)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaPayloads` (r:0 w:1)
	/// Proof: `Schemas::SchemaPayloads` (`max_values`: None, `max_size`: Some(65514), added: 67989, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema_v3(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `5552`
		// Minimum execution time: 24_266_000 picoseconds.
		Weight::from_parts(4_176_104, 5552)
			// Standard Error: 63
			.saturating_add(Weight::from_parts(34_827, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:0 w:1)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	fn set_max_schema_model_bytes() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 6_365_000 picoseconds.
		Weight::from_parts(6_787_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
	/// Storage: `Schemas::GovernanceSchemaModelMaxBytes` (r:1 w:0)
	/// Proof: `Schemas::GovernanceSchemaModelMaxBytes` (`max_values`: Some(1), `max_size`: Some(4), added: 499, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::CurrentSchemaIdentifierMaximum` (r:1 w:1)
	/// Proof: `Schemas::CurrentSchemaIdentifierMaximum` (`max_values`: Some(1), `max_size`: Some(2), added: 497, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaNameToIds` (r:1 w:1)
	/// Proof: `Schemas::SchemaNameToIds` (`max_values`: None, `max_size`: Some(602), added: 3077, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaInfos` (r:0 w:1)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaPayloads` (r:0 w:1)
	/// Proof: `Schemas::SchemaPayloads` (`max_values`: None, `max_size`: Some(65514), added: 67989, mode: `MaxEncodedLen`)
	/// The range of component `m` is `[16, 65499]`.
	fn create_schema_via_governance_v2(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `136`
		//  Estimated: `5552`
		// Minimum execution time: 24_342_000 picoseconds.
		Weight::from_parts(4_617_130, 5552)
			// Standard Error: 61
			.saturating_add(Weight::from_parts(34_750, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(3_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// The range of component `m` is `[16, 65499]`.
	fn propose_to_create_schema_v2(m: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `230`
		//  Estimated: `5180`
		// Minimum execution time: 21_224_000 picoseconds.
		Weight::from_parts(9_820_214, 5180)
			// Standard Error: 32
			.saturating_add(Weight::from_parts(3_099, 0).saturating_mul(m.into()))
			.saturating_add(RocksDbWeight::get().reads(4_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Council::Members` (r:1 w:0)
	/// Proof: `Council::Members` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalOf` (r:1 w:1)
	/// Proof: `Council::ProposalOf` (`max_values`: None, `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Proposals` (r:1 w:1)
	/// Proof: `Council::Proposals` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::ProposalCount` (r:1 w:1)
	/// Proof: `Council::ProposalCount` (`max_values`: Some(1), `max_size`: None, mode: `Measured`)
	/// Storage: `Council::Voting` (r:0 w:1)
	/// Proof: `Council::Voting` (`max_values`: None, `max_size`: None, mode: `Measured`)
	fn propose_to_create_schema_name() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `433`
		//  Estimated: `5383`
		// Minimum execution time: 26_441_000 picoseconds.
		Weight::from_parts(27_274_000, 5383)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(4_u64))
	}
	/// Storage: `Schemas::SchemaInfos` (r:1 w:0)
	/// Proof: `Schemas::SchemaInfos` (`max_values`: None, `max_size`: Some(15), added: 2490, mode: `MaxEncodedLen`)
	/// Storage: `Schemas::SchemaNameToIds` (r:1 w:1)
	/// Proof: `Schemas::SchemaNameToIds` (`max_values`: None, `max_size`: Some(602), added: 3077, mode: `MaxEncodedLen`)
	fn create_schema_name_via_governance() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `203`
		//  Estimated: `5552`
		// Minimum execution time: 16_683_000 picoseconds.
		Weight::from_parts(17_491_000, 5552)
			.saturating_add(RocksDbWeight::get().reads(2_u64))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
