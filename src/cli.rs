use std::path::PathBuf;

use clap::Parser;

/// Command line arguments.
#[derive(Debug, Parser)]
#[clap(author, about, version)]
pub(crate) struct Args {
    /// Output data as json.
    #[clap(long)]
    pub json: bool,

    /// Don't output names, only uid and gid.
    #[clap(long)]
    pub raw: bool,

    /// Paths from where to start discovery. Recursion is applied to paths that are directories.
    #[clap(default_value = ".")]
    pub roots: Vec<PathBuf>,
}
