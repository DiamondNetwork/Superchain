// Copyright 2020 Parity Technologies (UK) Ltd.
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

//! The `Error` and `Result` types used by the subsystem.

use futures::channel::oneshot;
use thiserror::Error;

/// Error type used by the Availability Recovery subsystem.
#[derive(Debug, Error)]
pub enum Error {
	#[error(transparent)]
	Subsystem(#[from] diamond_subsystem::SubsystemError),

	#[error("failed to query full data from store")]
	CanceledQueryFullData(#[source] oneshot::Canceled),

	#[error("failed to query session info")]
	CanceledSessionInfo(#[source] oneshot::Canceled),

	#[error("failed to send response")]
	CanceledResponseSender,

	#[error(transparent)]
	Runtime(#[from] diamond_subsystem::errors::RuntimeApiError),

	#[error(transparent)]
	Erasure(#[from] diamond_erasure_coding::Error),

	#[error(transparent)]
	Util(#[from] diamond_node_subsystem_util::Error),
}

pub type Result<T> = std::result::Result<T, Error>;
