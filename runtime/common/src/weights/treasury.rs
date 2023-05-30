// This file is part of Basilisk.

// Copyright (C) 2020-2023  Intergalactic, Limited (GIB).
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

//! Autogenerated weights for pallet_treasury
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-30, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/basilisk
// benchmark
// pallet
// --pallet=pallet-treasury
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// treasury.rs
// --template
// .maintain/pallet-weight-template.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

/// Weight functions needed for pallet_treasury.
pub trait WeightInfo {
	fn spend() -> Weight;
	fn propose_spend() -> Weight;
	fn reject_proposal() -> Weight;
	fn approve_proposal(p: u32) -> Weight;
	fn remove_approval() -> Weight;
	fn on_initialize_proposals(p: u32) -> Weight;
}

/// Weights for pallet_treasury using the hydraDX node and recommended hardware.
pub struct BasiliskWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for BasiliskWeight<T> {
	fn spend() -> Weight {
		// Minimum execution time: 76 nanoseconds.
		Weight::from_ref_time(92_000 as u64)
	}
	// Storage: Treasury ProposalCount (r:1 w:1)
	// Proof: Treasury ProposalCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Treasury Proposals (r:0 w:1)
	// Proof: Treasury Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	fn propose_spend() -> Weight {
		// Minimum execution time: 15_739 nanoseconds.
		Weight::from_ref_time(16_475_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Treasury Proposals (r:1 w:1)
	// Proof: Treasury Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn reject_proposal() -> Weight {
		// Minimum execution time: 21_827 nanoseconds.
		Weight::from_ref_time(22_257_000 as u64)
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: Treasury Proposals (r:1 w:0)
	// Proof: Treasury Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Treasury Approvals (r:1 w:1)
	// Proof: Treasury Approvals (max_values: Some(1), max_size: Some(402), added: 897, mode: MaxEncodedLen)
	/// The range of component `p` is `[0, 99]`.
	fn approve_proposal(p: u32) -> Weight {
		// Minimum execution time: 8_294 nanoseconds.
		Weight::from_ref_time(10_118_449 as u64) // Standard Error: 5_283
			.saturating_add(Weight::from_ref_time(41_829 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(2 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Treasury Approvals (r:1 w:1)
	// Proof: Treasury Approvals (max_values: Some(1), max_size: Some(402), added: 897, mode: MaxEncodedLen)
	fn remove_approval() -> Weight {
		// Minimum execution time: 6_801 nanoseconds.
		Weight::from_ref_time(7_023_000 as u64)
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: Treasury Deactivated (r:1 w:1)
	// Proof: Treasury Deactivated (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Balances InactiveIssuance (r:1 w:1)
	// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Treasury Approvals (r:1 w:1)
	// Proof: Treasury Approvals (max_values: Some(1), max_size: Some(402), added: 897, mode: MaxEncodedLen)
	// Storage: Treasury Proposals (r:100 w:100)
	// Proof: Treasury Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: System Account (r:200 w:200)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `p` is `[0, 100]`.
	fn on_initialize_proposals(p: u32) -> Weight {
		// Minimum execution time: 14_340 nanoseconds.
		Weight::from_ref_time(15_410_600 as u64) // Standard Error: 24_878
			.saturating_add(Weight::from_ref_time(16_795_486 as u64).saturating_mul(p as u64))
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().reads((3 as u64).saturating_mul(p as u64)))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(p as u64)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	fn spend() -> Weight {
		// Minimum execution time: 76 nanoseconds.
		Weight::from_ref_time(92_000 as u64)
	}
	// Storage: Treasury ProposalCount (r:1 w:1)
	// Proof: Treasury ProposalCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: Treasury Proposals (r:0 w:1)
	// Proof: Treasury Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	fn propose_spend() -> Weight {
		// Minimum execution time: 15_739 nanoseconds.
		Weight::from_ref_time(16_475_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Treasury Proposals (r:1 w:1)
	// Proof: Treasury Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: System Account (r:1 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	fn reject_proposal() -> Weight {
		// Minimum execution time: 21_827 nanoseconds.
		Weight::from_ref_time(22_257_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
	}
	// Storage: Treasury Proposals (r:1 w:0)
	// Proof: Treasury Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: Treasury Approvals (r:1 w:1)
	// Proof: Treasury Approvals (max_values: Some(1), max_size: Some(402), added: 897, mode: MaxEncodedLen)
	/// The range of component `p` is `[0, 99]`.
	fn approve_proposal(p: u32) -> Weight {
		// Minimum execution time: 8_294 nanoseconds.
		Weight::from_ref_time(10_118_449 as u64)
			// Standard Error: 5_283
			.saturating_add(Weight::from_ref_time(41_829 as u64).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(2 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Treasury Approvals (r:1 w:1)
	// Proof: Treasury Approvals (max_values: Some(1), max_size: Some(402), added: 897, mode: MaxEncodedLen)
	fn remove_approval() -> Weight {
		// Minimum execution time: 6_801 nanoseconds.
		Weight::from_ref_time(7_023_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(1 as u64))
			.saturating_add(RocksDbWeight::get().writes(1 as u64))
	}
	// Storage: Treasury Deactivated (r:1 w:1)
	// Proof: Treasury Deactivated (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Balances InactiveIssuance (r:1 w:1)
	// Proof: Balances InactiveIssuance (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: Treasury Approvals (r:1 w:1)
	// Proof: Treasury Approvals (max_values: Some(1), max_size: Some(402), added: 897, mode: MaxEncodedLen)
	// Storage: Treasury Proposals (r:100 w:100)
	// Proof: Treasury Proposals (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: System Account (r:200 w:200)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `p` is `[0, 100]`.
	fn on_initialize_proposals(p: u32) -> Weight {
		// Minimum execution time: 14_340 nanoseconds.
		Weight::from_ref_time(15_410_600 as u64)
			// Standard Error: 24_878
			.saturating_add(Weight::from_ref_time(16_795_486 as u64).saturating_mul(p as u64))
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().reads((3 as u64).saturating_mul(p as u64)))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
			.saturating_add(RocksDbWeight::get().writes((3 as u64).saturating_mul(p as u64)))
	}
}
