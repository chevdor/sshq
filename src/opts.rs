use clap::{crate_authors, crate_version, Parser, Subcommand};

/// `sshq` parses your ssh config and present the information back to you
#[derive(Parser)]
#[clap(version = crate_version!(), author = crate_authors!())]
pub struct Opts {
    #[clap(subcommand)]
    pub subcmd: SubCommand,

    /// Output as json
    #[clap(short, long, global = true)]
    pub json: bool,
}

/// You can find all available commands below.
#[derive(Subcommand)]
pub enum SubCommand {
    #[clap(version = crate_version!(), author = crate_authors!())]
    List(ListOpts),

    #[clap(version = crate_version!(), author = crate_authors!())]
    Search(SearchOpts),
}

/// The `info` command returns summarized information about a runtime.
#[derive(Parser)]
pub struct ListOpts {}

/// The `info` command returns summarized information about a runtime.
#[derive(Parser)]
pub struct SearchOpts {
    /// Search pattern
    #[clap(index = 1)]
    pub pattern: Option<String>,
}
