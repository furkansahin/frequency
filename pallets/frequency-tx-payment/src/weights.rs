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

//! Autogenerated weights for pallet_frequency_tx_payment
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-25, STEPS: `20`, REPEAT: 10, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("frequency-bench"), DB CACHE: 1024

// Executed Command:
// ./scripts/../target/release/frequency
// benchmark
// pallet
// --pallet=pallet_frequency-tx-payment
// --extrinsic
// *
// --chain=frequency-bench
// --execution=wasm
// --heap-pages=4096
// --wasm-execution=compiled
// --steps=20
// --repeat=10
// --output=./scripts/../pallets/frequency-tx-payment/src/weights.rs
// --template=./scripts/../.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(
	rustdoc::all,
	missing_docs,
	unused_parens,
	unused_imports
)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_frequency_tx_payment.
pub trait WeightInfo {
	fn pay_with_capacity() -> Weight;
	fn pay_with_capacity_batch_all(n: u32, ) -> Weight;
}

/// Weights for pallet_frequency_tx_payment using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	fn pay_with_capacity() -> Weight {
		Weight::from_ref_time(7_243_000 as u64)
	}
	fn pay_with_capacity_batch_all(n: u32, ) -> Weight {
		Weight::from_ref_time(13_672_579 as u64)
			// Standard Error: 82_116
			.saturating_add(Weight::from_ref_time(4_935_376 as u64).saturating_mul(n as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn pay_with_capacity() -> Weight {
		Weight::from_ref_time(7_243_000 as u64)
	}
	fn pay_with_capacity_batch_all(n: u32, ) -> Weight {
		Weight::from_ref_time(13_672_579 as u64)
			// Standard Error: 82_116
			.saturating_add(Weight::from_ref_time(4_935_376 as u64).saturating_mul(n as u64))
	}
}
