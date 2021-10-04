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

//! Error types for the subsystem requests.

use crate::JaegerError;

/// A description of an error causing the runtime API request to be unservable.
#[derive(Debug, Clone)]
pub struct RuntimeApiError(String);

impl From<String> for RuntimeApiError {
	fn from(s: String) -> Self {
		RuntimeApiError(s)
	}
}

impl core::fmt::Display for RuntimeApiError {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
		write!(f, "{}", self.0)
	}
}

impl std::error::Error for RuntimeApiError {}

/// A description of an error causing the chain API request to be unservable.
#[derive(Debug, Clone)]
pub struct ChainApiError {
	msg: String,
}

impl From<&str> for ChainApiError {
	fn from(s: &str) -> Self {
		s.to_owned().into()
	}
}

impl From<String> for ChainApiError {
	fn from(msg: String) -> Self {
		Self { msg }
	}
}

impl core::fmt::Display for ChainApiError {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
		write!(f, "{}", self.msg)
	}
}

impl std::error::Error for ChainApiError {}

/// An error that may happen during Availability Recovery process.
#[derive(PartialEq, Debug, Clone)]
pub enum RecoveryError {
	/// A chunk is recovered but is invalid.
	Invalid,

	/// A requested chunk is unavailable.
	Unavailable,
}

impl std::fmt::Display for RecoveryError {
	fn fmt(&self, f: &mut core::fmt::Formatter) -> Result<(), core::fmt::Error> {
		write!(f, "{}", self)
	}
}

impl std::error::Error for RecoveryError {}

/// An error type that describes faults that may happen
///
/// These are:
///   * Channels being closed
///   * Subsystems dying when they are not expected to
///   * Subsystems not dying when they are told to die
///   * etc.
#[derive(thiserror::Error, Debug)]
#[allow(missing_docs)]
pub enum SubsystemError {
	#[error(transparent)]
	NotifyCancellation(#[from] futures::channel::oneshot::Canceled),

	#[error(transparent)]
	QueueError(#[from] futures::channel::mpsc::SendError),

	#[error(transparent)]
	Io(#[from] std::io::Error),

	#[error(transparent)]
	Infallible(#[from] std::convert::Infallible),

	#[error(transparent)]
	Prometheus(#[from] substrate_prometheus_endpoint::PrometheusError),

	#[error(transparent)]
	Jaeger(#[from] JaegerError),

	#[error("Failed to {0}")]
	Context(String),

	#[error("Subsystem stalled: {0}")]
	SubsystemStalled(&'static str),

	/// Generated by the `#[overseer(..)]` proc-macro
	#[error(transparent)]
	Generated(#[from] ::diamond_overseer_gen::OverseerError),

	/// Per origin (or subsystem) annotations to wrap an error.
	#[error("Error originated in {origin}")]
	FromOrigin {
		/// An additional annotation tag for the origin of `source`.
		origin: &'static str,
		/// The wrapped error. Marked as source for tracking the error chain.
		#[source]
		source: Box<dyn 'static + std::error::Error + Send + Sync>,
	},
}

// impl AnnotateErrorOrigin for SubsystemError {
// 	fn with_origin(self, origin: &'static str) -> Self {
// 		Self::FromOrigin {
// 			origin,
// 			source: Box::new(self),
// 		}
// 	}
// }

impl SubsystemError {
	/// Adds a `str` as `origin` to the given error `err`.
	pub fn with_origin<E: 'static + Send + Sync + std::error::Error>(
		origin: &'static str,
		err: E,
	) -> Self {
		Self::FromOrigin { origin, source: Box::new(err) }
	}
}

/// Ease the use of subsystem errors.
pub type SubsystemResult<T> = Result<T, self::SubsystemError>;
