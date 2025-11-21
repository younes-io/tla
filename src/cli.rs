use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "tla")]
#[command(about = "TLA+ CLI: lint, fmt, check", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand, Debug)]
pub enum Command {
    /// Run static analysis on TLA+ files
    Lint {
        /// Files or directories to lint
        #[arg(value_name = "PATH", default_value = ".")]
        paths: Vec<PathBuf>,

        /// Output JSON (for editors/CI)
        #[arg(long)]
        json: bool,
    },

    /// Format TLA+ files
    Fmt {
        #[arg(value_name = "PATH", default_value = ".")]
        paths: Vec<PathBuf>,
    },

    /// Run model checking via TLC
    Check {
        /// Spec module (without .tla)
        #[arg(long)]
        spec: String,

        /// TLC config file
        #[arg(long)]
        cfg: Option<PathBuf>,
    },
}
