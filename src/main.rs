mod check;
mod cli;
mod doctor;
mod fmt;
mod lint;
mod tla_parser;
mod tooling;

use anyhow::Result;
use clap::Parser;

fn main() -> Result<()> {
    let cmd = cli::Cli::parse();
    match cmd.command {
        cli::Command::Lint { paths, json } => lint::run(paths, json)?,
        cli::Command::Fmt { paths } => fmt::run(paths)?,
        cli::Command::Check { spec, cfg } => check::run(spec, cfg)?,
        cli::Command::Doctor {
            write_tlc_wrapper,
            jar,
        } => doctor::run(write_tlc_wrapper, jar)?,
    }
    Ok(())
}
