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

//! Autogenerated weights for pallet_route_executor
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-30, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/basilisk
// benchmark
// pallet
// --pallet=pallet-route-executor
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// route_executor.rs
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

/// Weight functions needed for pallet_route_executor.
pub trait WeightInfo {
	fn sell(n: u32) -> Weight;
	fn buy(n: u32) -> Weight;
}

/// Weights for pallet_route_executor using the hydraDX node and recommended hardware.
pub struct BasiliskWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for BasiliskWeight<T> {
	// Storage: Tokens Accounts (r:16 w:16)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: System Account (r:6 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: XYK ShareToken (r:5 w:0)
	// Proof: XYK ShareToken (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:6 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:0)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:5 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 5]`.
	fn sell(n: u32) -> Weight {
		// Minimum execution time: 99_600 nanoseconds.
		Weight::from_ref_time(50_779_650 as u64) // Standard Error: 136_028
			.saturating_add(Weight::from_ref_time(48_214_210 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().reads((7 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(n as u64)))
	}
	// Storage: Tokens Accounts (r:16 w:16)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: System Account (r:6 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: XYK ShareToken (r:5 w:0)
	// Proof: XYK ShareToken (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:6 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:0)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:5 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 5]`.
	fn buy(n: u32) -> Weight {
		// Minimum execution time: 95_816 nanoseconds.
		Weight::from_ref_time(48_035_270 as u64) // Standard Error: 114_009
			.saturating_add(Weight::from_ref_time(48_391_870 as u64).saturating_mul(n as u64))
			.saturating_add(T::DbWeight::get().reads(4 as u64))
			.saturating_add(T::DbWeight::get().reads((7 as u64).saturating_mul(n as u64)))
			.saturating_add(T::DbWeight::get().writes(2 as u64))
			.saturating_add(T::DbWeight::get().writes((3 as u64).saturating_mul(n as u64)))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Tokens Accounts (r:16 w:16)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: System Account (r:6 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: XYK ShareToken (r:5 w:0)
	// Proof: XYK ShareToken (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:6 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:0)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:5 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 5]`.
	fn sell(n: u32) -> Weight {
		// Minimum execution time: 99_600 nanoseconds.
		Weight::from_ref_time(50_779_650 as u64)
			// Standard Error: 136_028
			.saturating_add(Weight::from_ref_time(48_214_210 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().reads((7 as u64).saturating_mul(n as u64)))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
			.saturating_add(RocksDbWeight::get().writes((3 as u64).saturating_mul(n as u64)))
	}
	// Storage: Tokens Accounts (r:16 w:16)
	// Proof: Tokens Accounts (max_values: None, max_size: Some(108), added: 2583, mode: MaxEncodedLen)
	// Storage: System Account (r:6 w:1)
	// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	// Storage: XYK ShareToken (r:5 w:0)
	// Proof: XYK ShareToken (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: AssetRegistry Assets (r:6 w:0)
	// Proof Skipped: AssetRegistry Assets (max_values: None, max_size: None, mode: Measured)
	// Storage: MultiTransactionPayment AccountCurrencyMap (r:1 w:0)
	// Proof: MultiTransactionPayment AccountCurrencyMap (max_values: None, max_size: Some(52), added: 2527, mode: MaxEncodedLen)
	// Storage: MultiTransactionPayment AcceptedCurrencies (r:5 w:0)
	// Proof: MultiTransactionPayment AcceptedCurrencies (max_values: None, max_size: Some(28), added: 2503, mode: MaxEncodedLen)
	/// The range of component `n` is `[1, 5]`.
	fn buy(n: u32) -> Weight {
		// Minimum execution time: 95_816 nanoseconds.
		Weight::from_ref_time(48_035_270 as u64)
			// Standard Error: 114_009
			.saturating_add(Weight::from_ref_time(48_391_870 as u64).saturating_mul(n as u64))
			.saturating_add(RocksDbWeight::get().reads(4 as u64))
			.saturating_add(RocksDbWeight::get().reads((7 as u64).saturating_mul(n as u64)))
			.saturating_add(RocksDbWeight::get().writes(2 as u64))
			.saturating_add(RocksDbWeight::get().writes((3 as u64).saturating_mul(n as u64)))
	}
}
