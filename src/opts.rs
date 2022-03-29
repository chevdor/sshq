use clap::{crate_authors, crate_version, Parser};

/// `subwasm` allows fetching, parsing and calling some methods on WASM runtimes of Substrate based chains.
#[derive(Parser)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Opts {
	/// Output as json
	#[clap(index = 1)]
	pub pattern: Option<String>,
}
