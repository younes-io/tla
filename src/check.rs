use crate::tooling::ensure_tool;
use anyhow::{Context, Result, anyhow};
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;
use tempfile::NamedTempFile;

pub fn run(spec: String, cfg: Option<PathBuf>) -> Result<()> {
    ensure_tool("tlc")?;

    let mut cmd = Command::new("tlc");
    cmd.arg(&spec);

    // TLC 2.19 errors if it cannot find <Spec>.cfg even when none is needed.
    // If the user did not provide a config, supply a minimal temp config that
    // points to the standard Init/Next operators. This avoids spurious
    // ConfigFileException while keeping explicit cfg handling intact.
    let _temp_cfg;
    if let Some(cfg_path) = cfg {
        cmd.arg("-config").arg(cfg_path);
    } else {
        let mut tmp = NamedTempFile::new()?;
        writeln!(tmp, "INIT Init")?;
        writeln!(tmp, "NEXT Next")?;
        _temp_cfg = tmp; // keep alive
        cmd.arg("-config").arg(_temp_cfg.path());
    }

    let status = cmd
        .status()
        .with_context(|| "failed to spawn tlc (is it on PATH?)")?;

    if !status.success() {
        return Err(anyhow!("tlc exited with {}", status));
    }

    Ok(())
}
