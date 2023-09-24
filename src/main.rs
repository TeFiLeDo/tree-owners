use std::{fs::read_dir, os::linux::fs::MetadataExt, path::Path};

use anyhow::{anyhow, ensure, Context, Result};
use clap::Parser;

use crate::{cli::Args, summary::Summary};

mod cli;
mod summary;

fn main() -> Result<()> {
    human_panic::setup_panic!();
    let args = Args::parse();

    let mut summary = Summary::default();
    for root in args.roots {
        fs_entry(&root, &mut summary)?;
    }

    if !args.raw {
        let (uf, gf) = summary.lookup_names();

        for (uid, e) in uf {
            eprintln!(
                "{:#}",
                anyhow!(e).context(format!("failed to get name for user {uid}"))
            );
        }

        for (gid, e) in gf {
            eprintln!(
                "{:#}",
                anyhow!(e).context(format!("failed to get name for group {gid}"))
            );
        }
    }
    let output = match args.json {
        false => summary.to_string(),
        true => serde_json::to_string_pretty(&summary).context("json serialization failed")?,
    };
    println!("{output}");

    Ok(())
}

/// Perform gid & uid gathering for a file, or a directory and its children.
fn fs_entry(entry: &Path, summary: &mut Summary) -> Result<()> {
    let display = entry.display();
    ensure!(
        entry.is_symlink() || entry.exists(),
        format!("{} doesn't exist", display)
    );

    let meta = entry
        .symlink_metadata()
        .context(format!("failed to get metadata for {}", display))?;
    summary.add_user(meta.st_uid());
    summary.add_group(meta.st_gid());

    if entry.is_dir() {
        let children = read_dir(entry).context(format!("failed to read dir {}", display))?;
        for e in children {
            let e = e.context(format!("invalid child for {}", display))?;
            fs_entry(&e.path(), summary)?;
        }
    }

    Ok(())
}
