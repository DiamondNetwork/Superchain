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

//! ruby-to-Millau headers sync entrypoint.

use crate::finality_pipeline::{SubstrateFinalitySyncPipeline, SubstrateFinalityToSubstrate};

use bp_header_chain::justification::GrandpaJustification;
use codec::Encode;
use relay_millau_client::{Millau, SigningParams as MillauSigningParams};
use relay_substrate_client::{Chain, TransactionSignScheme};
use relay_utils::metrics::MetricsParams;
use relay_ruby_client::{SyncHeader as rubySyncHeader, ruby};
use sp_core::{Bytes, Pair};

/// ruby-to-Millau finality sync pipeline.
pub(crate) type rubyFinalityToMillau = SubstrateFinalityToSubstrate<ruby, Millau, MillauSigningParams>;

impl SubstrateFinalitySyncPipeline for rubyFinalityToMillau {
	const BEST_FINALIZED_SOURCE_HEADER_ID_AT_TARGET: &'static str = bp_ruby::BEST_FINALIZED_ruby_HEADER_METHOD;

	type TargetChain = Millau;

	fn customize_metrics(params: MetricsParams) -> anyhow::Result<MetricsParams> {
		crate::chains::add_diamond_gold_price_metrics::<Self>(params)
	}

	fn transactions_author(&self) -> bp_millau::AccountId {
		(*self.target_sign.public().as_array_ref()).into()
	}

	fn make_submit_finality_proof_transaction(
		&self,
		transaction_nonce: <Millau as Chain>::Index,
		header: rubySyncHeader,
		proof: GrandpaJustification<bp_ruby::Header>,
	) -> Bytes {
		let call = millau_runtime::BridgeGrandparubyCall::<
			millau_runtime::Runtime,
			millau_runtime::rubyGrandpaInstance,
		>::submit_finality_proof(header.into_inner(), proof)
		.into();

		let genesis_hash = *self.target_client.genesis_hash();
		let transaction = Millau::sign_transaction(genesis_hash, &self.target_sign, transaction_nonce, call);

		Bytes(transaction.encode())
	}
}
