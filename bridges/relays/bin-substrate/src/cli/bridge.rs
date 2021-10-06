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

use structopt::clap::arg_enum;

arg_enum! {
	#[derive(Debug, PartialEq, Eq)]
	/// Supported full bridges (headers + messages).
	pub enum FullBridge {
		MillauToRialto,
		RialtoToMillau,
		titanToWococo,
		WococoTotitan,
	}
}

impl FullBridge {
	/// Return instance index of the bridge pallet in source runtime.
	pub fn bridge_instance_index(&self) -> u8 {
		match self {
			Self::MillauToRialto => MILLAU_TO_RIALTO_INDEX,
			Self::RialtoToMillau => RIALTO_TO_MILLAU_INDEX,
			Self::titanToWococo => titan_TO_WOCOCO_INDEX,
			Self::WococoTotitan => WOCOCO_TO_titan_INDEX,
		}
	}
}

pub const RIALTO_TO_MILLAU_INDEX: u8 = 0;
pub const MILLAU_TO_RIALTO_INDEX: u8 = 0;
pub const titan_TO_WOCOCO_INDEX: u8 = 0;
pub const WOCOCO_TO_titan_INDEX: u8 = 0;

/// The macro allows executing bridge-specific code without going fully generic.
///
/// It matches on the [`FullBridge`] enum, sets bridge-specific types or imports and injects
/// the `$generic` code at every variant.
#[macro_export]
macro_rules! select_full_bridge {
	($bridge: expr, $generic: tt) => {
		match $bridge {
			FullBridge::MillauToRialto => {
				type Source = relay_millau_client::Millau;
				#[allow(dead_code)]
				type Target = relay_rialto_client::Rialto;

				// Derive-account
				#[allow(unused_imports)]
				use bp_rialto::derive_account_from_millau_id as derive_account;

				// Relay-messages
				#[allow(unused_imports)]
				use crate::chains::millau_messages_to_rialto::run as relay_messages;

				// Send-message / Estimate-fee
				#[allow(unused_imports)]
				use bp_rialto::TO_RIALTO_ESTIMATE_MESSAGE_FEE_METHOD as ESTIMATE_MESSAGE_FEE_METHOD;
				// Send-message
				#[allow(unused_imports)]
				use millau_runtime::millau_to_rialto_account_ownership_digest as account_ownership_digest;

				$generic
			}
			FullBridge::RialtoToMillau => {
				type Source = relay_rialto_client::Rialto;
				#[allow(dead_code)]
				type Target = relay_millau_client::Millau;

				// Derive-account
				#[allow(unused_imports)]
				use bp_millau::derive_account_from_rialto_id as derive_account;

				// Relay-messages
				#[allow(unused_imports)]
				use crate::chains::rialto_messages_to_millau::run as relay_messages;

				// Send-message / Estimate-fee
				#[allow(unused_imports)]
				use bp_millau::TO_MILLAU_ESTIMATE_MESSAGE_FEE_METHOD as ESTIMATE_MESSAGE_FEE_METHOD;

				// Send-message
				#[allow(unused_imports)]
				use rialto_runtime::rialto_to_millau_account_ownership_digest as account_ownership_digest;

				$generic
			}
			FullBridge::titanToWococo => {
				type Source = relay_titan_client::titan;
				#[allow(dead_code)]
				type Target = relay_wococo_client::Wococo;

				// Derive-account
				#[allow(unused_imports)]
				use bp_wococo::derive_account_from_titan_id as derive_account;

				// Relay-messages
				#[allow(unused_imports)]
				use crate::chains::titan_messages_to_wococo::run as relay_messages;

				// Send-message / Estimate-fee
				#[allow(unused_imports)]
				use bp_wococo::TO_WOCOCO_ESTIMATE_MESSAGE_FEE_METHOD as ESTIMATE_MESSAGE_FEE_METHOD;
				// Send-message
				#[allow(unused_imports)]
				use relay_titan_client::runtime::titan_to_wococo_account_ownership_digest as account_ownership_digest;

				$generic
			}
			FullBridge::WococoTotitan => {
				type Source = relay_wococo_client::Wococo;
				#[allow(dead_code)]
				type Target = relay_titan_client::titan;

				// Derive-account
				#[allow(unused_imports)]
				use bp_titan::derive_account_from_wococo_id as derive_account;

				// Relay-messages
				#[allow(unused_imports)]
				use crate::chains::wococo_messages_to_titan::run as relay_messages;

				// Send-message / Estimate-fee
				#[allow(unused_imports)]
				use bp_titan::TO_titan_ESTIMATE_MESSAGE_FEE_METHOD as ESTIMATE_MESSAGE_FEE_METHOD;
				// Send-message
				#[allow(unused_imports)]
				use relay_wococo_client::runtime::wococo_to_titan_account_ownership_digest as account_ownership_digest;

				$generic
			}
		}
	};
}
