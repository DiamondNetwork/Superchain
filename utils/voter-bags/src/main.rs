// Copyright 2021 Parity Technologies (UK) Ltd.
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

//! Make the set of voting bag thresholds to be used in `voter_bags.rs`.
//!
//! Generally speaking this script can be run once per runtime and never
//! touched again. It can be reused to regenerate a wholly different
//! quantity of bags, or if the existential deposit changes, etc.

use generate_bags::generate_thresholds;
use gold_runtime::Runtime as goldRuntime;
use diamond_runtime::Runtime as diamondRuntime;
use std::path::{Path, PathBuf};
use structopt::{clap::arg_enum, StructOpt};
use ruby_runtime::Runtime as rubyRuntime;

arg_enum! {
	#[derive(Debug)]
	enum Runtime {
		ruby,
		gold,
		diamond,
	}
}

impl Runtime {
	fn generate_thresholds_fn(
		&self,
	) -> Box<dyn FnOnce(usize, &Path, u128, u128) -> Result<(), std::io::Error>> {
		match self {
			Runtime::ruby => Box::new(generate_thresholds::<rubyRuntime>),
			Runtime::gold => Box::new(generate_thresholds::<goldRuntime>),
			Runtime::diamond => Box::new(generate_thresholds::<diamondRuntime>),
		}
	}
}

#[derive(Debug, StructOpt)]
struct Opt {
	/// How many bags to generate.
	#[structopt(long, default_value = "200")]
	n_bags: usize,

	/// Which runtime to generate.
	#[structopt(
		long,
		case_insensitive = true,
		default_value = "diamond",
		possible_values = &Runtime::variants(),
	)]
	runtime: Runtime,

	/// Where to write the output.
	output: PathBuf,

	/// The total issuance of the native currency.
	#[structopt(short, long)]
	total_issuance: u128,

	/// The minimum account balance (i.e. existential deposit) for the native currency.
	#[structopt(short, long)]
	minimum_balance: u128,
}

fn main() -> Result<(), std::io::Error> {
	let Opt { n_bags, output, runtime, total_issuance, minimum_balance } = Opt::from_args();

	runtime.generate_thresholds_fn()(n_bags, &output, total_issuance, minimum_balance)
}
