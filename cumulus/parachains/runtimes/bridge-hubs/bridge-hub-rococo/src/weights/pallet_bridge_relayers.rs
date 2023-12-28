// Copyright (C) Parity Technologies (UK) Ltd.
// This file is part of Cumulus.

// Cumulus is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Cumulus is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Cumulus.  If not, see <http://www.gnu.org/licenses/>.

//! Autogenerated weights for `pallet_bridge_relayers`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 4.0.0-dev
//! DATE: 2023-07-31, STEPS: `50`, REPEAT: `20`, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! WORST CASE MAP SIZE: `1000000`
//! HOSTNAME: `runner-ynta1nyy-project-238-concurrent-0`, CPU: `Intel(R) Xeon(R) CPU @ 2.60GHz`
//! EXECUTION: ``, WASM-EXECUTION: `Compiled`, CHAIN: `Some("bridge-hub-rococo-dev")`, DB CACHE: 1024

// Executed Command:
// ./target/production/polkadot-parachain
// benchmark
// pallet
// --chain=bridge-hub-rococo-dev
// --wasm-execution=compiled
// --pallet=pallet_bridge_relayers
// --no-storage-info
// --no-median-slopes
// --no-min-squares
// --extrinsic=*
// --steps=50
// --repeat=20
// --json
// --header=./file_header.txt
// --output=./parachains/runtimes/bridge-hubs/bridge-hub-rococo/src/weights/

#![cfg_attr(rustfmt, rustfmt_skip)]
#![allow(unused_parens)]
#![allow(unused_imports)]
#![allow(missing_docs)]

use frame_support::{traits::Get, weights::Weight};
use core::marker::PhantomData;

/// Weight functions for `pallet_bridge_relayers`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> pallet_bridge_relayers::WeightInfo for WeightInfo<T> {
	/// Storage: `BridgeRelayers::RelayerRewards` (r:1 w:1)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn claim_rewards() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `207`
		//  Estimated: `3593`
		// Minimum execution time: 54_291_000 picoseconds.
		Weight::from_parts(55_145_000, 0)
			.saturating_add(Weight::from_parts(0, 3593))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `BridgeRelayers::RegisteredRelayers` (r:1 w:1)
	/// Proof: `BridgeRelayers::RegisteredRelayers` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: UNKNOWN KEY `0x1e8445dc201eeb8560e5579a5dd54655` (r:1 w:0)
	/// Proof: UNKNOWN KEY `0x1e8445dc201eeb8560e5579a5dd54655` (r:1 w:0)
	/// Storage: `Balances::Reserves` (r:1 w:1)
	/// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(1249), added: 3724, mode: `MaxEncodedLen`)
	fn register() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `61`
		//  Estimated: `4714`
		// Minimum execution time: 28_143_000 picoseconds.
		Weight::from_parts(28_920_000, 0)
			.saturating_add(Weight::from_parts(0, 4714))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `BridgeRelayers::RegisteredRelayers` (r:1 w:1)
	/// Proof: `BridgeRelayers::RegisteredRelayers` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Reserves` (r:1 w:1)
	/// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(1249), added: 3724, mode: `MaxEncodedLen`)
	fn deregister() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `160`
		//  Estimated: `4714`
		// Minimum execution time: 30_329_000 picoseconds.
		Weight::from_parts(30_646_000, 0)
			.saturating_add(Weight::from_parts(0, 4714))
			.saturating_add(T::DbWeight::get().reads(2))
			.saturating_add(T::DbWeight::get().writes(2))
	}
	/// Storage: `BridgeRelayers::RegisteredRelayers` (r:1 w:1)
	/// Proof: `BridgeRelayers::RegisteredRelayers` (`max_values`: None, `max_size`: Some(68), added: 2543, mode: `MaxEncodedLen`)
	/// Storage: `Balances::Reserves` (r:1 w:1)
	/// Proof: `Balances::Reserves` (`max_values`: None, `max_size`: Some(1249), added: 3724, mode: `MaxEncodedLen`)
	/// Storage: `System::Account` (r:1 w:1)
	/// Proof: `System::Account` (`max_values`: None, `max_size`: Some(128), added: 2603, mode: `MaxEncodedLen`)
	fn slash_and_deregister() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `263`
		//  Estimated: `4714`
		// Minimum execution time: 29_704_000 picoseconds.
		Weight::from_parts(30_269_000, 0)
			.saturating_add(Weight::from_parts(0, 4714))
			.saturating_add(T::DbWeight::get().reads(3))
			.saturating_add(T::DbWeight::get().writes(3))
	}
	/// Storage: `BridgeRelayers::RelayerRewards` (r:1 w:1)
	/// Proof: `BridgeRelayers::RelayerRewards` (`max_values`: None, `max_size`: Some(73), added: 2548, mode: `MaxEncodedLen`)
	fn register_relayer_reward() -> Weight {
		// Proof Size summary in bytes:
		//  Measured:  `6`
		//  Estimated: `3538`
		// Minimum execution time: 2_793_000 picoseconds.
		Weight::from_parts(2_999_000, 0)
			.saturating_add(Weight::from_parts(0, 3538))
			.saturating_add(T::DbWeight::get().reads(1))
			.saturating_add(T::DbWeight::get().writes(1))
	}
}
