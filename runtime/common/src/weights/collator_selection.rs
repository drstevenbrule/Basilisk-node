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

//! Autogenerated weights for pallet_collator_selection
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-30, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/basilisk
// benchmark
// pallet
// --pallet=pallet-collator-selection
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// collator_selection.rs
// --template
// .maintain/pallet-weight-template-no-back.hbs

#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(clippy::unnecessary_cast)]

use frame_support::{
	traits::Get,
	weights::{constants::RocksDbWeight, Weight},
};
use sp_std::marker::PhantomData;

use pallet_collator_selection::weights::WeightInfo;

pub struct BasiliskWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for BasiliskWeight<T> {
	// Storage: Session NextKeys (r:50 w:0)
	// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	// Storage: CollatorSelection Invulnerables (r:0 w:1)
	// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(1601), added: 2096, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 50]`.
	fn set_invulnerables(b: u32) -> Weight {
		// Minimum execution time: 11_032 nanoseconds.
		Weight::from_ref_time(10_733_144 as u64) // Standard Error: 10_075
			.saturating_add(Weight::from_ref_time(2_119_006 as u64).saturating_mul(b as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(b as u64)))
			.saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: CollatorSelection DesiredCandidates (r:0 w:1)
	// Proof: CollatorSelection DesiredCandidates (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn set_desired_candidates() -> Weight {
		// Minimum execution time: 7_428 nanoseconds.
		Weight::from_ref_time(7_705_000 as u64).saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: CollatorSelection CandidacyBond (r:0 w:1)
	// Proof: CollatorSelection CandidacyBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	fn set_candidacy_bond() -> Weight {
		// Minimum execution time: 4_407 nanoseconds.
		Weight::from_ref_time(4_748_000 as u64).saturating_add(T::DbWeight::get().writes(1 as u64))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(961), added: 1456, mode: MaxEncodedLen)
	// Storage: CollatorSelection DesiredCandidates (r:1 w:0)
	// Proof: CollatorSelection DesiredCandidates (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(1601), added: 2096, mode: MaxEncodedLen)
	// Storage: Session NextKeys (r:1 w:0)
	// Proof Skipped: Session NextKeys (max_values: None, max_size: None, mode: Measured)
	// Storage: CollatorSelection CandidacyBond (r:1 w:0)
	// Proof: CollatorSelection CandidacyBond (max_values: Some(1), max_size: Some(16), added: 511, mode: MaxEncodedLen)
	// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// The range of component `c` is `[1, 19]`.
	fn register_as_candidate(c: u32) -> Weight {
		// Minimum execution time: 29_723 nanoseconds.
		Weight::from_ref_time(30_336_825 as u64) // Standard Error: 7_246
			.saturating_add(Weight::from_ref_time(125_221 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: CollatorSelection Candidates (r:1 w:1)
	// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(961), added: 1456, mode: MaxEncodedLen)
	// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	/// The range of component `c` is `[5, 20]`.
	fn leave_intent(c: u32) -> Weight {
		// Minimum execution time: 19_483 nanoseconds.
		Weight::from_ref_time(19_868_717 as u64) // Standard Error: 9_186
			.saturating_add(Weight::from_ref_time(65_444 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(1 as u64))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
	}
	// Storage: System Account (r:2 w:2)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: System BlockWeight (r:1 w:1)
	// Proof: System BlockWeight (max_values: Some(1), max_size: Some(48), added: 543, mode: MaxEncodedLen)
	// Storage: CollatorSelection LastAuthoredBlock (r:0 w:1)
	// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	fn note_author() -> Weight {
		// Minimum execution time: 16_572 nanoseconds.
		Weight::from_ref_time(17_060_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: CollatorSelection Candidates (r:1 w:0)
	// Proof: CollatorSelection Candidates (max_values: Some(1), max_size: Some(961), added: 1456, mode: MaxEncodedLen)
	// Storage: CollatorSelection LastAuthoredBlock (r:20 w:0)
	// Proof: CollatorSelection LastAuthoredBlock (max_values: None, max_size: Some(44), added: 2519, mode: MaxEncodedLen)
	// Storage: CollatorSelection Invulnerables (r:1 w:0)
	// Proof: CollatorSelection Invulnerables (max_values: Some(1), max_size: Some(1601), added: 2096, mode: MaxEncodedLen)
	// Storage: System BlockWeight (r:1 w:1)
	// Proof: System BlockWeight (max_values: Some(1), max_size: Some(48), added: 543, mode: MaxEncodedLen)
	// Storage: System Account (r:15 w:15)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// The range of component `r` is `[1, 20]`.
	/// The range of component `c` is `[1, 20]`.
	fn new_session(_r: u32, c: u32) -> Weight {
		// Minimum execution time: 14_855 nanoseconds.
		Weight::from_ref_time(27_820_837 as u64) // Standard Error: 748_906
			.saturating_add(Weight::from_ref_time(4_783_710 as u64).saturating_mul(c as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().reads((1 as u64).saturating_mul(c as u64)))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
			.saturating_add(T::DbWeight::get().writes((1 as u64).saturating_mul(c as u64)))
	}
}
