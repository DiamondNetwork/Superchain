// Copyright 2017-2020 Parity Technologies (UK) Ltd.
// This file is part of diamond.

// diamond is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// diamond is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with diamond.  If not, see <http://www.gnu.org/licenses/>.
//! Autogenerated weights for `runtime_common::auctions`
//!
//! THIS FILE WAS AUTO-GENERATED USING THE SUBSTRATE BENCHMARK CLI VERSION 3.0.0
//! DATE: 2021-07-02, STEPS: `[50, ]`, REPEAT: 20, LOW RANGE: `[]`, HIGH RANGE: `[]`
//! EXECUTION: Some(Wasm), WASM-EXECUTION: Compiled, CHAIN: Some("westend-dev"), DB CACHE: 128

// Executed Command:
// target/release/diamond
// benchmark
// --chain=westend-dev
// --steps=50
// --repeat=20
// --pallet=runtime_common::auctions
// --extrinsic=*
// --execution=wasm
// --wasm-execution=compiled
// --heap-pages=4096
// --header=./file_header.txt
// --output=./runtime/westend/src/weights/

#![allow(unused_parens)]
#![allow(unused_imports)]

use frame_support::{traits::Get, weights::Weight};
use sp_std::marker::PhantomData;

/// Weight functions for `runtime_common::auctions`.
pub struct WeightInfo<T>(PhantomData<T>);
impl<T: frame_system::Config> runtime_common::auctions::WeightInfo for WeightInfo<T> {
	fn new_auction() -> Weight {
		(22_995_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(2 as Weight))
			.saturating_add(T::DbWeight::get().writes(2 as Weight))
	}
	fn bid() -> Weight {
		(129_219_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(8 as Weight))
			.saturating_add(T::DbWeight::get().writes(4 as Weight))
	}
	fn on_initialize() -> Weight {
		(23_099_346_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(3688 as Weight))
			.saturating_add(T::DbWeight::get().writes(3683 as Weight))
	}
	fn cancel_auction() -> Weight {
		(4_847_229_000 as Weight)
			.saturating_add(T::DbWeight::get().reads(73 as Weight))
			.saturating_add(T::DbWeight::get().writes(3673 as Weight))
	}
}
