// This file is part of Substrate.

// Copyright (C) Parity Technologies (UK) Ltd.
// SPDX-License-Identifier: Apache-2.0

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Autogenerated weights for pallet_fast_unstake
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-06-16, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-e8ezs4ez-project-145-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("dev"), DB CACHE: 1024

// Executed Command:
// ./target/production/substrate
// benchmark
// pallet
// --chain=dev
// --steps=50
// --repeat=20
// --pallet=pallet_fast_unstake
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --output=./frame/fast-unstake/src/weights.rs
// --header=./HEADER-APACHE2
// --template=./.maintain/frame-weight-template.hbs

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::{Weight, constants::RocksDbWeight}};
use core::marker::PhantomData;

/// Weight functions needed for pallet_fast_unstake.
pub trait WeightInfo {
	fn on_idle_unstake(b: u32, ) -> Weight;
	fn on_idle_check(v: u32, b: u32, ) -> Weight;
	fn register_fast_unstake() -> Weight;
	fn deregister() -> Weight;
	fn control() -> Weight;
}

/// Weights for pallet_fast_unstake using the Substrate node and recommended hardware.
pub struct SubstrateWeight<T>(PhantomData<T>);
impl<T: frame_system::Config> WeightInfo for SubstrateWeight<T> {
	/// Storage: FastUnstake ErasToCheckPerBlock (r:1 w:0)
	/// Proof: FastUnstake ErasToCheckPerBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking ValidatorCount (r:1 w:0)
	/// Proof: Staking ValidatorCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: FastUnstake Head (r:1 w:1)
	/// Proof: FastUnstake Head (max_values: Some(1), max_size: Some(5768), added: 6263, mode: MaxEncodedLen)
	/// Storage: FastUnstake CounterForQueue (r:1 w:0)
	/// Proof: FastUnstake CounterForQueue (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking SlashingSpans (r:64 w:0)
	/// Proof Skipped: Staking SlashingSpans (max_values: None, max_size: None, mode: Measured)
	/// Storage: Staking Bonded (r:64 w:64)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Validators (r:64 w:0)
	/// Proof: Staking Validators (max_values: None, max_size: Some(45), added: 2520, mode: MaxEncodedLen)
	/// Storage: Staking Nominators (r:64 w:0)
	/// Proof: Staking Nominators (max_values: None, max_size: Some(558), added: 3033, mode: MaxEncodedLen)
	/// Storage: System Account (r:64 w:64)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:64 w:64)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:64 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:0 w:64)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: Staking Payee (r:0 w:64)
	/// Proof: Staking Payee (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 64]`.
	fn on_idle_unstake(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1378 + b * (343 ±0)`
		//  Estimated: `7253 + b * (3774 ±0)`
		// Minimum execution time: 92_847_000 picoseconds.
		Weight::from_parts(42_300_813, 7253)
			// Standard Error: 40_514
			.saturating_add(Weight::from_parts(58_412_402, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(6_u64))
			.saturating_add(T::DbWeight::get().reads((7_u64).saturating_mul(b.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(T::DbWeight::get().writes((5_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 3774).saturating_mul(b.into()))
	}
	/// Storage: FastUnstake ErasToCheckPerBlock (r:1 w:0)
	/// Proof: FastUnstake ErasToCheckPerBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking ValidatorCount (r:1 w:0)
	/// Proof: Staking ValidatorCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: FastUnstake Head (r:1 w:1)
	/// Proof: FastUnstake Head (max_values: Some(1), max_size: Some(5768), added: 6263, mode: MaxEncodedLen)
	/// Storage: FastUnstake CounterForQueue (r:1 w:0)
	/// Proof: FastUnstake CounterForQueue (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking ErasStakers (r:257 w:0)
	/// Proof Skipped: Staking ErasStakers (max_values: None, max_size: None, mode: Measured)
	/// The range of component `v` is `[1, 256]`.
	/// The range of component `b` is `[1, 64]`.
	fn on_idle_check(v: u32, b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1546 + b * (48 ±0) + v * (10037 ±0)`
		//  Estimated: `7253 + b * (49 ±0) + v * (12513 ±0)`
		// Minimum execution time: 1_685_784_000 picoseconds.
		Weight::from_parts(1_693_370_000, 7253)
			// Standard Error: 13_295_842
			.saturating_add(Weight::from_parts(425_349_148, 0).saturating_mul(v.into()))
			// Standard Error: 53_198_180
			.saturating_add(Weight::from_parts(1_673_328_444, 0).saturating_mul(b.into()))
			.saturating_add(T::DbWeight::get().reads(7_u64))
			.saturating_add(T::DbWeight::get().reads((1_u64).saturating_mul(v.into())))
			.saturating_add(T::DbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 49).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 12513).saturating_mul(v.into()))
	}
	/// Storage: FastUnstake ErasToCheckPerBlock (r:1 w:0)
	/// Proof: FastUnstake ErasToCheckPerBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:1)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: FastUnstake Queue (r:1 w:1)
	/// Proof: FastUnstake Queue (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: FastUnstake Head (r:1 w:0)
	/// Proof: FastUnstake Head (max_values: Some(1), max_size: Some(5768), added: 6263, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Validators (r:1 w:0)
	/// Proof: Staking Validators (max_values: None, max_size: Some(45), added: 2520, mode: MaxEncodedLen)
	/// Storage: Staking Nominators (r:1 w:1)
	/// Proof: Staking Nominators (max_values: None, max_size: Some(558), added: 3033, mode: MaxEncodedLen)
	/// Storage: Staking CounterForNominators (r:1 w:1)
	/// Proof: Staking CounterForNominators (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: VoterList ListNodes (r:1 w:1)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:1 w:1)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: VoterList CounterForListNodes (r:1 w:1)
	/// Proof: VoterList CounterForListNodes (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: FastUnstake CounterForQueue (r:1 w:1)
	/// Proof: FastUnstake CounterForQueue (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn register_fast_unstake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1964`
		//  Estimated: `7253`
		// Minimum execution time: 125_512_000 picoseconds.
		Weight::from_parts(129_562_000, 7253)
			.saturating_add(T::DbWeight::get().reads(15_u64))
			.saturating_add(T::DbWeight::get().writes(9_u64))
	}
	/// Storage: FastUnstake ErasToCheckPerBlock (r:1 w:0)
	/// Proof: FastUnstake ErasToCheckPerBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:0)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: FastUnstake Queue (r:1 w:1)
	/// Proof: FastUnstake Queue (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: FastUnstake Head (r:1 w:0)
	/// Proof: FastUnstake Head (max_values: Some(1), max_size: Some(5768), added: 6263, mode: MaxEncodedLen)
	/// Storage: FastUnstake CounterForQueue (r:1 w:1)
	/// Proof: FastUnstake CounterForQueue (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn deregister() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1223`
		//  Estimated: `7253`
		// Minimum execution time: 43_943_000 picoseconds.
		Weight::from_parts(45_842_000, 7253)
			.saturating_add(T::DbWeight::get().reads(5_u64))
			.saturating_add(T::DbWeight::get().writes(2_u64))
	}
	/// Storage: FastUnstake ErasToCheckPerBlock (r:0 w:1)
	/// Proof: FastUnstake ErasToCheckPerBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn control() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_677_000 picoseconds.
		Weight::from_parts(2_849_000, 0)
			.saturating_add(T::DbWeight::get().writes(1_u64))
	}
}

// For backwards compatibility and tests
impl WeightInfo for () {
	/// Storage: FastUnstake ErasToCheckPerBlock (r:1 w:0)
	/// Proof: FastUnstake ErasToCheckPerBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking ValidatorCount (r:1 w:0)
	/// Proof: Staking ValidatorCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: FastUnstake Head (r:1 w:1)
	/// Proof: FastUnstake Head (max_values: Some(1), max_size: Some(5768), added: 6263, mode: MaxEncodedLen)
	/// Storage: FastUnstake CounterForQueue (r:1 w:0)
	/// Proof: FastUnstake CounterForQueue (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking SlashingSpans (r:64 w:0)
	/// Proof Skipped: Staking SlashingSpans (max_values: None, max_size: None, mode: Measured)
	/// Storage: Staking Bonded (r:64 w:64)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Validators (r:64 w:0)
	/// Proof: Staking Validators (max_values: None, max_size: Some(45), added: 2520, mode: MaxEncodedLen)
	/// Storage: Staking Nominators (r:64 w:0)
	/// Proof: Staking Nominators (max_values: None, max_size: Some(558), added: 3033, mode: MaxEncodedLen)
	/// Storage: System Account (r:64 w:64)
	/// Proof: System Account (max_values: None, max_size: Some(128), added: 2603, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:64 w:64)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:64 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:0 w:64)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: Staking Payee (r:0 w:64)
	/// Proof: Staking Payee (max_values: None, max_size: Some(73), added: 2548, mode: MaxEncodedLen)
	/// The range of component `b` is `[1, 64]`.
	fn on_idle_unstake(b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1378 + b * (343 ±0)`
		//  Estimated: `7253 + b * (3774 ±0)`
		// Minimum execution time: 92_847_000 picoseconds.
		Weight::from_parts(42_300_813, 7253)
			// Standard Error: 40_514
			.saturating_add(Weight::from_parts(58_412_402, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(6_u64))
			.saturating_add(RocksDbWeight::get().reads((7_u64).saturating_mul(b.into())))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(RocksDbWeight::get().writes((5_u64).saturating_mul(b.into())))
			.saturating_add(Weight::from_parts(0, 3774).saturating_mul(b.into()))
	}
	/// Storage: FastUnstake ErasToCheckPerBlock (r:1 w:0)
	/// Proof: FastUnstake ErasToCheckPerBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking ValidatorCount (r:1 w:0)
	/// Proof: Staking ValidatorCount (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: FastUnstake Head (r:1 w:1)
	/// Proof: FastUnstake Head (max_values: Some(1), max_size: Some(5768), added: 6263, mode: MaxEncodedLen)
	/// Storage: FastUnstake CounterForQueue (r:1 w:0)
	/// Proof: FastUnstake CounterForQueue (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: ElectionProviderMultiPhase CurrentPhase (r:1 w:0)
	/// Proof Skipped: ElectionProviderMultiPhase CurrentPhase (max_values: Some(1), max_size: None, mode: Measured)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking ErasStakers (r:257 w:0)
	/// Proof Skipped: Staking ErasStakers (max_values: None, max_size: None, mode: Measured)
	/// The range of component `v` is `[1, 256]`.
	/// The range of component `b` is `[1, 64]`.
	fn on_idle_check(v: u32, b: u32, ) -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1546 + b * (48 ±0) + v * (10037 ±0)`
		//  Estimated: `7253 + b * (49 ±0) + v * (12513 ±0)`
		// Minimum execution time: 1_685_784_000 picoseconds.
		Weight::from_parts(1_693_370_000, 7253)
			// Standard Error: 13_295_842
			.saturating_add(Weight::from_parts(425_349_148, 0).saturating_mul(v.into()))
			// Standard Error: 53_198_180
			.saturating_add(Weight::from_parts(1_673_328_444, 0).saturating_mul(b.into()))
			.saturating_add(RocksDbWeight::get().reads(7_u64))
			.saturating_add(RocksDbWeight::get().reads((1_u64).saturating_mul(v.into())))
			.saturating_add(RocksDbWeight::get().writes(1_u64))
			.saturating_add(Weight::from_parts(0, 49).saturating_mul(b.into()))
			.saturating_add(Weight::from_parts(0, 12513).saturating_mul(v.into()))
	}
	/// Storage: FastUnstake ErasToCheckPerBlock (r:1 w:0)
	/// Proof: FastUnstake ErasToCheckPerBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:1)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: FastUnstake Queue (r:1 w:1)
	/// Proof: FastUnstake Queue (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: FastUnstake Head (r:1 w:0)
	/// Proof: FastUnstake Head (max_values: Some(1), max_size: Some(5768), added: 6263, mode: MaxEncodedLen)
	/// Storage: Staking Bonded (r:1 w:0)
	/// Proof: Staking Bonded (max_values: None, max_size: Some(72), added: 2547, mode: MaxEncodedLen)
	/// Storage: Staking Validators (r:1 w:0)
	/// Proof: Staking Validators (max_values: None, max_size: Some(45), added: 2520, mode: MaxEncodedLen)
	/// Storage: Staking Nominators (r:1 w:1)
	/// Proof: Staking Nominators (max_values: None, max_size: Some(558), added: 3033, mode: MaxEncodedLen)
	/// Storage: Staking CounterForNominators (r:1 w:1)
	/// Proof: Staking CounterForNominators (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: VoterList ListNodes (r:1 w:1)
	/// Proof: VoterList ListNodes (max_values: None, max_size: Some(154), added: 2629, mode: MaxEncodedLen)
	/// Storage: VoterList ListBags (r:1 w:1)
	/// Proof: VoterList ListBags (max_values: None, max_size: Some(82), added: 2557, mode: MaxEncodedLen)
	/// Storage: VoterList CounterForListNodes (r:1 w:1)
	/// Proof: VoterList CounterForListNodes (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking CurrentEra (r:1 w:0)
	/// Proof: Staking CurrentEra (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Balances Locks (r:1 w:1)
	/// Proof: Balances Locks (max_values: None, max_size: Some(1299), added: 3774, mode: MaxEncodedLen)
	/// Storage: Balances Freezes (r:1 w:0)
	/// Proof: Balances Freezes (max_values: None, max_size: Some(49), added: 2524, mode: MaxEncodedLen)
	/// Storage: FastUnstake CounterForQueue (r:1 w:1)
	/// Proof: FastUnstake CounterForQueue (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn register_fast_unstake() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1964`
		//  Estimated: `7253`
		// Minimum execution time: 125_512_000 picoseconds.
		Weight::from_parts(129_562_000, 7253)
			.saturating_add(RocksDbWeight::get().reads(15_u64))
			.saturating_add(RocksDbWeight::get().writes(9_u64))
	}
	/// Storage: FastUnstake ErasToCheckPerBlock (r:1 w:0)
	/// Proof: FastUnstake ErasToCheckPerBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	/// Storage: Staking Ledger (r:1 w:0)
	/// Proof: Staking Ledger (max_values: None, max_size: Some(1091), added: 3566, mode: MaxEncodedLen)
	/// Storage: FastUnstake Queue (r:1 w:1)
	/// Proof: FastUnstake Queue (max_values: None, max_size: Some(56), added: 2531, mode: MaxEncodedLen)
	/// Storage: FastUnstake Head (r:1 w:0)
	/// Proof: FastUnstake Head (max_values: Some(1), max_size: Some(5768), added: 6263, mode: MaxEncodedLen)
	/// Storage: FastUnstake CounterForQueue (r:1 w:1)
	/// Proof: FastUnstake CounterForQueue (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn deregister() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `1223`
		//  Estimated: `7253`
		// Minimum execution time: 43_943_000 picoseconds.
		Weight::from_parts(45_842_000, 7253)
			.saturating_add(RocksDbWeight::get().reads(5_u64))
			.saturating_add(RocksDbWeight::get().writes(2_u64))
	}
	/// Storage: FastUnstake ErasToCheckPerBlock (r:0 w:1)
	/// Proof: FastUnstake ErasToCheckPerBlock (max_values: Some(1), max_size: Some(4), added: 499, mode: MaxEncodedLen)
	fn control() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `0`
		//  Estimated: `0`
		// Minimum execution time: 2_677_000 picoseconds.
		Weight::from_parts(2_849_000, 0)
			.saturating_add(RocksDbWeight::get().writes(1_u64))
	}
}
