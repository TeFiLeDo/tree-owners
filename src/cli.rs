use std::path::PathBuf;

use clap::Parser;

/// Command line arguments.
#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub(crate) struct Args {
    /// Whether to output data as json.
    #[clap(long)]
    pub json: bool,

    /// Whether to output raw uid and gid numbers.
    #[clap(long)]
    pub raw: bool,

    /// The roots to use for discovery.
    #[clap(default_value = ".")]
    pub roots: Vec<PathBuf>,
}
