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

//! Autogenerated weights for pallet_nft
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-05-30, STEPS: 5, REPEAT: 20, LOW RANGE: [], HIGH RANGE: []
//! EXECUTION: None, WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// target/release/basilisk
// benchmark
// pallet
// --pallet=pallet-nft
// --chain=dev
// --extrinsic=*
// --steps=5
// --repeat=20
// --output
// nft.rs
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

/// Weight functions needed for pallet_nft.
pub trait WeightInfo {
	fn create_collection() -> Weight;
	fn mint() -> Weight;
	fn transfer() -> Weight;
	fn destroy_collection() -> Weight;
	fn burn() -> Weight;
}

/// Weights for pallet_nft using the hydraDX node and recommended hardware.
pub struct BasiliskWeight<T>(PhantomData<T>);

impl<T: frame_system::Config> WeightInfo for BasiliskWeight<T> {
	// Storage: Uniques Class (r:1 w:1)
	// Proof: Uniques Class (max_values: None, max_size: Some(190), added: 2665, mode: MaxEncodedLen)
	// Storage: AssetRegistry NextAssetId (r:1 w:0)
	// Proof Skipped: AssetRegistry NextAssetId (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: AssetRegistry LocationAssets (r:1 w:0)
	// Proof Skipped: AssetRegistry LocationAssets (max_values: None, max_size: None, mode: Measured)
	// Storage: Uniques ClassAccount (r:0 w:1)
	// Proof: Uniques ClassAccount (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	// Storage: NFT Collections (r:0 w:1)
	// Proof: NFT Collections (max_values: None, max_size: Some(99), added: 2574, mode: MaxEncodedLen)
	fn create_collection() -> Weight {
		// Minimum execution time: 19_285 nanoseconds.
		Weight::from_ref_time(19_971_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(3 as u64))
	}
	// Storage: NFT Collections (r:1 w:0)
	// Proof: NFT Collections (max_values: None, max_size: Some(99), added: 2574, mode: MaxEncodedLen)
	// Storage: Uniques Class (r:1 w:1)
	// Proof: Uniques Class (max_values: None, max_size: Some(190), added: 2665, mode: MaxEncodedLen)
	// Storage: Uniques Asset (r:1 w:1)
	// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	// Proof: Uniques CollectionMaxSupply (max_values: None, max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	// Storage: AssetRegistry NextAssetId (r:1 w:0)
	// Proof Skipped: AssetRegistry NextAssetId (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: AssetRegistry LocationAssets (r:1 w:0)
	// Proof Skipped: AssetRegistry LocationAssets (max_values: None, max_size: None, mode: Measured)
	// Storage: Uniques Account (r:0 w:1)
	// Proof: Uniques Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	// Storage: NFT Items (r:0 w:1)
	// Proof: NFT Items (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	fn mint() -> Weight {
		// Minimum execution time: 30_873 nanoseconds.
		Weight::from_ref_time(31_502_000 as u64)
			.saturating_add(T::DbWeight::get().reads(6 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: NFT Collections (r:1 w:0)
	// Proof: NFT Collections (max_values: None, max_size: Some(99), added: 2574, mode: MaxEncodedLen)
	// Storage: Uniques Asset (r:1 w:1)
	// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	// Storage: Uniques Class (r:1 w:0)
	// Proof: Uniques Class (max_values: None, max_size: Some(190), added: 2665, mode: MaxEncodedLen)
	// Storage: Uniques Account (r:0 w:2)
	// Proof: Uniques Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(113), added: 2588, mode: MaxEncodedLen)
	fn transfer() -> Weight {
		// Minimum execution time: 28_617 nanoseconds.
		Weight::from_ref_time(28_876_000 as u64)
			.saturating_add(T::DbWeight::get().reads(3 as u64))
			.saturating_add(T::DbWeight::get().writes(4 as u64))
	}
	// Storage: NFT Collections (r:1 w:1)
	// Proof: NFT Collections (max_values: None, max_size: Some(99), added: 2574, mode: MaxEncodedLen)
	// Storage: Uniques Class (r:1 w:1)
	// Proof: Uniques Class (max_values: None, max_size: Some(190), added: 2665, mode: MaxEncodedLen)
	// Storage: Uniques Asset (r:1 w:0)
	// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	// Storage: AssetRegistry NextAssetId (r:1 w:0)
	// Proof Skipped: AssetRegistry NextAssetId (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: AssetRegistry LocationAssets (r:1 w:0)
	// Proof Skipped: AssetRegistry LocationAssets (max_values: None, max_size: None, mode: Measured)
	// Storage: Uniques ClassAccount (r:0 w:1)
	// Proof: Uniques ClassAccount (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	// Storage: Uniques ClassMetadataOf (r:0 w:1)
	// Proof: Uniques ClassMetadataOf (max_values: None, max_size: Some(123), added: 2598, mode: MaxEncodedLen)
	// Storage: Uniques CollectionMaxSupply (r:0 w:1)
	// Proof: Uniques CollectionMaxSupply (max_values: None, max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	fn destroy_collection() -> Weight {
		// Minimum execution time: 37_550 nanoseconds.
		Weight::from_ref_time(38_201_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
	// Storage: NFT Collections (r:1 w:0)
	// Proof: NFT Collections (max_values: None, max_size: Some(99), added: 2574, mode: MaxEncodedLen)
	// Storage: Uniques Asset (r:1 w:1)
	// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	// Storage: Uniques Class (r:1 w:1)
	// Proof: Uniques Class (max_values: None, max_size: Some(190), added: 2665, mode: MaxEncodedLen)
	// Storage: AssetRegistry NextAssetId (r:1 w:0)
	// Proof Skipped: AssetRegistry NextAssetId (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: AssetRegistry LocationAssets (r:1 w:0)
	// Proof Skipped: AssetRegistry LocationAssets (max_values: None, max_size: None, mode: Measured)
	// Storage: Uniques Account (r:0 w:1)
	// Proof: Uniques Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(113), added: 2588, mode: MaxEncodedLen)
	// Storage: NFT Items (r:0 w:1)
	// Proof: NFT Items (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	fn burn() -> Weight {
		// Minimum execution time: 31_433 nanoseconds.
		Weight::from_ref_time(31_954_000 as u64)
			.saturating_add(T::DbWeight::get().reads(5 as u64))
			.saturating_add(T::DbWeight::get().writes(5 as u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	// Storage: Uniques Class (r:1 w:1)
	// Proof: Uniques Class (max_values: None, max_size: Some(190), added: 2665, mode: MaxEncodedLen)
	// Storage: AssetRegistry NextAssetId (r:1 w:0)
	// Proof Skipped: AssetRegistry NextAssetId (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: AssetRegistry LocationAssets (r:1 w:0)
	// Proof Skipped: AssetRegistry LocationAssets (max_values: None, max_size: None, mode: Measured)
	// Storage: Uniques ClassAccount (r:0 w:1)
	// Proof: Uniques ClassAccount (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	// Storage: NFT Collections (r:0 w:1)
	// Proof: NFT Collections (max_values: None, max_size: Some(99), added: 2574, mode: MaxEncodedLen)
	fn create_collection() -> Weight {
		// Minimum execution time: 19_285 nanoseconds.
		Weight::from_ref_time(19_971_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(3 as u64))
	}
	// Storage: NFT Collections (r:1 w:0)
	// Proof: NFT Collections (max_values: None, max_size: Some(99), added: 2574, mode: MaxEncodedLen)
	// Storage: Uniques Class (r:1 w:1)
	// Proof: Uniques Class (max_values: None, max_size: Some(190), added: 2665, mode: MaxEncodedLen)
	// Storage: Uniques Asset (r:1 w:1)
	// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	// Storage: Uniques CollectionMaxSupply (r:1 w:0)
	// Proof: Uniques CollectionMaxSupply (max_values: None, max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	// Storage: AssetRegistry NextAssetId (r:1 w:0)
	// Proof Skipped: AssetRegistry NextAssetId (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: AssetRegistry LocationAssets (r:1 w:0)
	// Proof Skipped: AssetRegistry LocationAssets (max_values: None, max_size: None, mode: Measured)
	// Storage: Uniques Account (r:0 w:1)
	// Proof: Uniques Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	// Storage: NFT Items (r:0 w:1)
	// Proof: NFT Items (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	fn mint() -> Weight {
		// Minimum execution time: 30_873 nanoseconds.
		Weight::from_ref_time(31_502_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(6 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: NFT Collections (r:1 w:0)
	// Proof: NFT Collections (max_values: None, max_size: Some(99), added: 2574, mode: MaxEncodedLen)
	// Storage: Uniques Asset (r:1 w:1)
	// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	// Storage: Uniques Class (r:1 w:0)
	// Proof: Uniques Class (max_values: None, max_size: Some(190), added: 2665, mode: MaxEncodedLen)
	// Storage: Uniques Account (r:0 w:2)
	// Proof: Uniques Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(113), added: 2588, mode: MaxEncodedLen)
	fn transfer() -> Weight {
		// Minimum execution time: 28_617 nanoseconds.
		Weight::from_ref_time(28_876_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(3 as u64))
			.saturating_add(RocksDbWeight::get().writes(4 as u64))
	}
	// Storage: NFT Collections (r:1 w:1)
	// Proof: NFT Collections (max_values: None, max_size: Some(99), added: 2574, mode: MaxEncodedLen)
	// Storage: Uniques Class (r:1 w:1)
	// Proof: Uniques Class (max_values: None, max_size: Some(190), added: 2665, mode: MaxEncodedLen)
	// Storage: Uniques Asset (r:1 w:0)
	// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	// Storage: AssetRegistry NextAssetId (r:1 w:0)
	// Proof Skipped: AssetRegistry NextAssetId (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: AssetRegistry LocationAssets (r:1 w:0)
	// Proof Skipped: AssetRegistry LocationAssets (max_values: None, max_size: None, mode: Measured)
	// Storage: Uniques ClassAccount (r:0 w:1)
	// Proof: Uniques ClassAccount (max_values: None, max_size: Some(80), added: 2555, mode: MaxEncodedLen)
	// Storage: Uniques ClassMetadataOf (r:0 w:1)
	// Proof: Uniques ClassMetadataOf (max_values: None, max_size: Some(123), added: 2598, mode: MaxEncodedLen)
	// Storage: Uniques CollectionMaxSupply (r:0 w:1)
	// Proof: Uniques CollectionMaxSupply (max_values: None, max_size: Some(36), added: 2511, mode: MaxEncodedLen)
	fn destroy_collection() -> Weight {
		// Minimum execution time: 37_550 nanoseconds.
		Weight::from_ref_time(38_201_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
	// Storage: NFT Collections (r:1 w:0)
	// Proof: NFT Collections (max_values: None, max_size: Some(99), added: 2574, mode: MaxEncodedLen)
	// Storage: Uniques Asset (r:1 w:1)
	// Proof: Uniques Asset (max_values: None, max_size: Some(146), added: 2621, mode: MaxEncodedLen)
	// Storage: Uniques Class (r:1 w:1)
	// Proof: Uniques Class (max_values: None, max_size: Some(190), added: 2665, mode: MaxEncodedLen)
	// Storage: AssetRegistry NextAssetId (r:1 w:0)
	// Proof Skipped: AssetRegistry NextAssetId (max_values: Some(1), max_size: None, mode: Measured)
	// Storage: AssetRegistry LocationAssets (r:1 w:0)
	// Proof Skipped: AssetRegistry LocationAssets (max_values: None, max_size: None, mode: Measured)
	// Storage: Uniques Account (r:0 w:1)
	// Proof: Uniques Account (max_values: None, max_size: Some(112), added: 2587, mode: MaxEncodedLen)
	// Storage: Uniques ItemPriceOf (r:0 w:1)
	// Proof: Uniques ItemPriceOf (max_values: None, max_size: Some(113), added: 2588, mode: MaxEncodedLen)
	// Storage: NFT Items (r:0 w:1)
	// Proof: NFT Items (max_values: None, max_size: Some(122), added: 2597, mode: MaxEncodedLen)
	fn burn() -> Weight {
		// Minimum execution time: 31_433 nanoseconds.
		Weight::from_ref_time(31_954_000 as u64)
			.saturating_add(RocksDbWeight::get().reads(5 as u64))
			.saturating_add(RocksDbWeight::get().writes(5 as u64))
	}
}
