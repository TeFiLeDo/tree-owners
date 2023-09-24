use std::{
    fs::read_dir,
    os::linux::fs::MetadataExt,
    path::{Path, PathBuf},
};

use anyhow::{ensure, Context, Result};
use clap::Parser;

use crate::{id::Ids, name::Names, output::Output};

mod id;
mod name;
mod output;

fn main() -> Result<()> {
    human_panic::setup_panic!();
    let args = Args::parse();

    let mut ids = Ids::default();
    for root in args.roots {
        fs_entry(&root, &mut ids)?;
    }

    let output: Box<dyn Output> = match args.raw {
        false => Box::new(Names::try_from(ids).context("failed to get names")?),
        true => Box::new(ids),
    };
    let output = match args.json {
        false => output.human_readable(),
        true => output.json().context("failed json serialization")?,
    };
    println!("{output}");

    Ok(())
}

/// Command line arguments.
#[derive(Debug, Parser)]
#[clap(author, about, version)]
struct Args {
    /// Whether to output data as json.
    #[clap(long)]
    pub json: bool,

    /// Whether to output raw uid and gid numbers.
    #[clap(long)]
    pub raw: bool,

    /// The roots to use for discovery.
    pub roots: Vec<PathBuf>,
}

/// Perform gid & uid gathering for a file, or a directory and its children.
fn fs_entry(entry: &Path, summary: &mut Ids) -> Result<()> {
    let display = entry.display();
    ensure!(
        entry.is_symlink() || entry.exists(),
        format!("{} doesn't exist", display)
    );

    let meta = entry
        .symlink_metadata()
        .context(format!("failed to get metadata for {}", display))?;
    summary.users.insert(meta.st_uid());
    summary.groups.insert(meta.st_gid());

    if entry.is_dir() {
        let children = read_dir(entry).context(format!("failed to read dir {}", display))?;
        for e in children {
            let e = e.context(format!("invalid child for {}", display))?;
            fs_entry(&e.path(), summary).context(format!("failed to read child of {}", display))?;
        }
    }

    Ok(())
}
