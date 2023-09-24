use std::{fs::read_dir, os::linux::fs::MetadataExt, path::Path};

use anyhow::{anyhow, ensure, Context, Error, Result};
use clap::Parser;

use crate::{cli::Args, summary::Summary};

mod cli;
mod summary;

fn main() -> Result<()> {
    human_panic::setup_panic!();
    let args = Args::parse();

    let mut summary = Summary::default();
    for root in args.roots {
        fs_entry(&root, &mut summary);
    }

    if !args.raw {
        let (uf, gf) = summary.lookup_names();
        uf.into_iter()
            .for_each(|(uid, e)| print_err(e, format!("failed to get name for user {uid}")));
        gf.into_iter()
            .for_each(|(gid, e)| print_err(e, format!("failed to get name for group {gid}")));
    }
    let output = match args.json {
        false => summary.to_string(),
        true => serde_json::to_string_pretty(&summary).context("json serialization failed")?,
    };
    println!("{output}");

    Ok(())
}

/// Perform gid & uid gathering for a file, or a directory and its children.
fn fs_entry(entry: &Path, summary: &mut Summary) {
    let display = entry.display();
    if !entry.exists() && !entry.is_symlink() {
        print_root_err(format!("{display} doesn't exist"));
    }

    let meta = match entry.symlink_metadata() {
        Ok(meta) => meta,
        Err(e) => {
            print_err(e, format!("failed to get metadata for {display}"));
            return;
        }
    };

    summary.add_user(meta.st_uid());
    summary.add_group(meta.st_gid());

    if entry.is_dir() {
        let children = match read_dir(entry) {
            Ok(children) => children,
            Err(e) => {
                print_err(e, format!("failed to read dir {display}"));
                return;
            }
        };

        for child in children {
            match child {
                Ok(child) => fs_entry(&child.path(), summary),
                Err(e) => {
                    print_err(e, format!("invalid child for {display}"));
                    return;
                }
            }
        }
    }
}

fn print_root_err(message: String) {
    eprintln!("{:#}", anyhow!("{message}"));
}

fn print_err(err: impl Into<Error>, message: String) {
    eprintln!("{:#}", err.into().context(message));
}
