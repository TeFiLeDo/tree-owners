use std::{env, fs::create_dir};

use anyhow::{Context, Result};
use clap::CommandFactory;
use clap_complete::{
    generate_to,
    shells::{Bash, Fish, Zsh},
};

include!("src/cli.rs");

fn main() -> Result<()> {
    let out = if env::var("CI").map(|ci| ci == "true").unwrap_or_default() {
        create_dir("../ci-out").context("failed to create CI output directory")?;
        "../ci-out".to_string()
    } else {
        env::var("OUT_DIR").context("OUT_DIR not set")?
    };

    println!("cargo:rerun-if-changed=src/cli.rs");
    let mut cli = <Args as CommandFactory>::command();
    let _ = generate_to(Fish, &mut cli, env!("CARGO_PKG_NAME"), out.clone());
    let _ = generate_to(Bash, &mut cli, env!("CARGO_PKG_NAME"), out.clone());
    let _ = generate_to(Zsh, &mut cli, env!("CARGO_PKG_NAME"), out);

    Ok(())
}
