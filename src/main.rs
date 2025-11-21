mod check;
mod cli;
mod fmt;
mod lint;
mod tla_parser;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cmd = cli::Cli::parse();
    match cmd.command {
        cli::Command::Lint { paths, json } => lint::run(paths, json)?,
        cli::Command::Fmt { paths } => fmt::run(paths)?,
        cli::Command::Check { spec, cfg } => check::run(spec, cfg)?,
    }
    Ok(())
}
