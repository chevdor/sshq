use clap::{crate_authors, crate_version, Parser};

/// `sshq` parses your ssh config and present the information back to you
#[derive(Parser)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Opts {
	/// Search pattern
	#[clap(index = 1)]
	pub pattern: Option<String>,
}
