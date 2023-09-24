use std::{fs::read_dir, os::linux::fs::MetadataExt, path::Path};

use anyhow::{ensure, Context, Result};
use clap::Parser;

use crate::{cli::Args, id::Ids, name::Names, output::Output};

mod cli;
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
            fs_entry(&e.path(), summary)?;
        }
    }

    Ok(())
}
