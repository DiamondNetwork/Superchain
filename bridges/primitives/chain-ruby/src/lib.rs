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

#![cfg_attr(not(feature = "std"), no_std)]
// RuntimeApi generated functions
#![allow(clippy::too_many_arguments)]
// Runtime-generated DecodeLimit::decode_all_with_depth_limit
#![allow(clippy::unnecessary_mut_passed)]

use bp_messages::{LaneId, MessageDetails, MessageNonce, UnrewardedRelayersState};
use bp_runtime::Chain;
use sp_std::prelude::*;
use sp_version::RuntimeVersion;

pub use bp_diamond_core::*;

/// ruby Chain
pub type ruby = diamondLike;

pub type UncheckedExtrinsic = bp_diamond_core::UncheckedExtrinsic<Call>;

// NOTE: This needs to be kept up to date with the ruby runtime found in the diamond repo.
pub const VERSION: RuntimeVersion = RuntimeVersion {
	spec_name: sp_version::create_runtime_str!("ruby"),
	impl_name: sp_version::create_runtime_str!("parity-ruby"),
	authoring_version: 2,
	spec_version: 51,
	impl_version: 0,
	apis: sp_version::create_apis_vec![[]],
	transaction_version: 5,
};

/// ruby Runtime `Call` enum.
///
/// The enum represents a subset of possible `Call`s we can send to ruby chain.
/// Ideally this code would be auto-generated from Metadata, because we want to
/// avoid depending directly on the ENTIRE runtime just to get the encoding of `Dispatchable`s.
///
/// All entries here (like pretty much in the entire file) must be kept in sync with ruby
/// `construct_runtime`, so that we maintain SCALE-compatibility.
///
/// See: https://github.com/paritytech/diamond/blob/master/runtime/ruby/src/lib.rs
#[derive(parity_scale_codec::Encode, parity_scale_codec::Decode, Debug, PartialEq, Eq, Clone)]
pub enum Call {
	/// titan bridge pallet.
	#[codec(index = 40)]
	BridgeGrandpatitan(BridgeGrandpatitanCall),
}

#[derive(parity_scale_codec::Encode, parity_scale_codec::Decode, Debug, PartialEq, Eq, Clone)]
#[allow(non_camel_case_types)]
pub enum BridgeGrandpatitanCall {
	#[codec(index = 0)]
	submit_finality_proof(
		<diamondLike as Chain>::Header,
		bp_header_chain::justification::GrandpaJustification<<diamondLike as Chain>::Header>,
	),
	#[codec(index = 1)]
	initialize(bp_header_chain::InitializationData<<diamondLike as Chain>::Header>),
}

impl sp_runtime::traits::Dispatchable for Call {
	type Origin = ();
	type Config = ();
	type Info = ();
	type PostInfo = ();

	fn dispatch(self, _origin: Self::Origin) -> sp_runtime::DispatchResultWithInfo<Self::PostInfo> {
		unimplemented!("The Call is not expected to be dispatched.")
	}
}

// We use this to get the account on ruby (target) which is derived from titan's (source)
// account.
pub fn derive_account_from_titan_id(id: bp_runtime::SourceAccount<AccountId>) -> AccountId {
	let encoded_id = bp_runtime::derive_account_id(bp_runtime::titan_CHAIN_ID, id);
	AccountIdConverter::convert(encoded_id)
}

/// Name of the `rubyFinalityApi::best_finalized` runtime method.
pub const BEST_FINALIZED_ruby_HEADER_METHOD: &str = "rubyFinalityApi_best_finalized";
/// Name of the `rubyFinalityApi::is_known_header` runtime method.
pub const IS_KNOWN_ruby_HEADER_METHOD: &str = "rubyFinalityApi_is_known_header";

/// Name of the `TorubyOutboundLaneApi::estimate_message_delivery_and_dispatch_fee` runtime method.
pub const TO_ruby_ESTIMATE_MESSAGE_FEE_METHOD: &str =
	"TorubyOutboundLaneApi_estimate_message_delivery_and_dispatch_fee";
/// Name of the `TorubyOutboundLaneApi::message_details` runtime method.
pub const TO_ruby_MESSAGE_DETAILS_METHOD: &str = "TorubyOutboundLaneApi_message_details";
/// Name of the `TorubyOutboundLaneApi::latest_generated_nonce` runtime method.
pub const TO_ruby_LATEST_GENERATED_NONCE_METHOD: &str = "TorubyOutboundLaneApi_latest_generated_nonce";
/// Name of the `TorubyOutboundLaneApi::latest_received_nonce` runtime method.
pub const TO_ruby_LATEST_RECEIVED_NONCE_METHOD: &str = "TorubyOutboundLaneApi_latest_received_nonce";

/// Name of the `FromrubyInboundLaneApi::latest_received_nonce` runtime method.
pub const FROM_ruby_LATEST_RECEIVED_NONCE_METHOD: &str = "FromrubyInboundLaneApi_latest_received_nonce";
/// Name of the `FromrubyInboundLaneApi::latest_onfirmed_nonce` runtime method.
pub const FROM_ruby_LATEST_CONFIRMED_NONCE_METHOD: &str = "FromrubyInboundLaneApi_latest_confirmed_nonce";
/// Name of the `FromrubyInboundLaneApi::unrewarded_relayers_state` runtime method.
pub const FROM_ruby_UNREWARDED_RELAYERS_STATE: &str = "FromrubyInboundLaneApi_unrewarded_relayers_state";

/// The target length of a session (how often authorities change) on ruby measured in of number of
/// blocks.
///
/// Note that since this is a target sessions may change before/after this time depending on network
/// conditions.
pub const SESSION_LENGTH: BlockNumber = 10 * time_units::MINUTES;

sp_api::decl_runtime_apis! {
	/// API for querying information about the finalized ruby headers.
	///
	/// This API is implemented by runtimes that are bridging with the ruby chain, not the
	/// ruby runtime itself.
	pub trait rubyFinalityApi {
		/// Returns number and hash of the best finalized header known to the bridge module.
		fn best_finalized() -> (BlockNumber, Hash);
		/// Returns true if the header is known to the runtime.
		fn is_known_header(hash: Hash) -> bool;
	}

	/// Outbound message lane API for messages that are sent to ruby chain.
	///
	/// This API is implemented by runtimes that are sending messages to ruby chain, not the
	/// ruby runtime itself.
	pub trait TorubyOutboundLaneApi<OutboundMessageFee: Parameter, OutboundPayload: Parameter> {
		/// Estimate message delivery and dispatch fee that needs to be paid by the sender on
		/// this chain.
		///
		/// Returns `None` if message is too expensive to be sent to ruby from this chain.
		///
		/// Please keep in mind that this method returns the lowest message fee required for message
		/// to be accepted to the lane. It may be good idea to pay a bit over this price to account
		/// future exchange rate changes and guarantee that relayer would deliver your message
		/// to the target chain.
		fn estimate_message_delivery_and_dispatch_fee(
			lane_id: LaneId,
			payload: OutboundPayload,
		) -> Option<OutboundMessageFee>;
		/// Returns dispatch weight, encoded payload size and delivery+dispatch fee of all
		/// messages in given inclusive range.
		///
		/// If some (or all) messages are missing from the storage, they'll also will
		/// be missing from the resulting vector. The vector is ordered by the nonce.
		fn message_details(
			lane: LaneId,
			begin: MessageNonce,
			end: MessageNonce,
		) -> Vec<MessageDetails<OutboundMessageFee>>;
		/// Returns nonce of the latest message, received by bridged chain.
		fn latest_received_nonce(lane: LaneId) -> MessageNonce;
		/// Returns nonce of the latest message, generated by given lane.
		fn latest_generated_nonce(lane: LaneId) -> MessageNonce;
	}

	/// Inbound message lane API for messages sent by ruby chain.
	///
	/// This API is implemented by runtimes that are receiving messages from ruby chain, not the
	/// ruby runtime itself.
	pub trait FromrubyInboundLaneApi {
		/// Returns nonce of the latest message, received by given lane.
		fn latest_received_nonce(lane: LaneId) -> MessageNonce;
		/// Nonce of the latest message that has been confirmed to the bridged chain.
		fn latest_confirmed_nonce(lane: LaneId) -> MessageNonce;
		/// State of the unrewarded relayers set at given lane.
		fn unrewarded_relayers_state(lane: LaneId) -> UnrewardedRelayersState;
	}
}
