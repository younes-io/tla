use anyhow::{Context, Result, anyhow};
use std::path::PathBuf;
use std::process::Command;

pub fn run(spec: String, cfg: Option<PathBuf>) -> Result<()> {
    let mut cmd = Command::new("tlc");
    cmd.arg(spec);
    if let Some(cfg) = cfg {
        cmd.arg("-config").arg(cfg);
    }

    let status = cmd
        .status()
        .with_context(|| "failed to spawn tlc (is it on PATH?)")?;

    if !status.success() {
        return Err(anyhow!("tlc exited with {}", status));
    }

    Ok(())
}
