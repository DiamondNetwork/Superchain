// Copyright 2019-2021 Parity Technologies (UK) Ltd.
// This file is part of Parity Bridges Common.

// Parity Bridges Common is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Parity Bridges Common is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Parity Bridges Common.  If not, see <http://www.gnu.org/licenses/>.

//! Types used to connect to the gold chain.

use relay_substrate_client::{Chain, ChainBase};
use std::time::Duration;

/// gold header id.
pub type HeaderId = relay_utils::HeaderId<bp_gold::Hash, bp_gold::BlockNumber>;

/// gold chain definition
#[derive(Debug, Clone, Copy)]
pub struct gold;

impl ChainBase for gold {
	type BlockNumber = bp_gold::BlockNumber;
	type Hash = bp_gold::Hash;
	type Hasher = bp_gold::Hasher;
	type Header = bp_gold::Header;
}

impl Chain for gold {
	const NAME: &'static str = "gold";
	const AVERAGE_BLOCK_INTERVAL: Duration = Duration::from_secs(6);

	type AccountId = bp_gold::AccountId;
	type Index = bp_gold::Nonce;
	type SignedBlock = bp_gold::SignedBlock;
	type Call = ();
	type Balance = bp_gold::Balance;
}

/// gold header type used in headers sync.
pub type SyncHeader = relay_substrate_client::SyncHeader<bp_gold::Header>;
