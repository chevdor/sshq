use clap::{crate_authors, crate_version, Parser, Subcommand};
use std::path::PathBuf;

/// `sshq` parses your ssh config and present the information back to you
#[derive(Debug, Parser)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Opts {
	#[clap(subcommand)]
	pub subcmd: SubCommand,

	#[clap(index = 1, global = true)]
	pub file: Option<PathBuf>,

	/// Output as json
	#[clap(short, long, global = true)]
	pub json: bool,
}

/// You can find all available commands below.
#[derive(Debug, Subcommand)]
pub enum SubCommand {
	#[clap(version = crate_version!(), author = crate_authors!())]
	List(ListOpts),

	#[clap(version = crate_version!(), author = crate_authors!())]
	Search(SearchOpts),
}

/// The `list` command returns the list of entries.
#[derive(Debug, Parser)]
pub struct ListOpts {}

/// The `search` command searches for a given pattern.
#[derive(Debug, Parser)]
pub struct SearchOpts {
	/// Search pattern
	#[clap(index = 1)]
	pub pattern: Option<String>,
}
