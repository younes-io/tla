use anyhow::{Context, Result, anyhow};
use std::env;
use std::fs;
#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;
use std::path::PathBuf;

pub struct ToolStatus {
    pub present: bool,
    pub path: Option<PathBuf>,
    pub hint: String,
}

pub fn tool_status(name: &'static str) -> ToolStatus {
    let found = which::which(name).ok();
    let hint = missing_hint(name);
    ToolStatus {
        present: found.is_some(),
        path: found,
        hint,
    }
}

pub fn ensure_tool(name: &'static str) -> Result<()> {
    let status = tool_status(name);
    if status.present {
        return Ok(());
    }
    Err(anyhow!(status.hint))
}

pub fn write_tlc_wrapper(out_path: PathBuf, jar: Option<PathBuf>) -> Result<()> {
    let jar_path = jar
        .or_else(|| env::var("TLA_TOOLS_JAR").ok().map(PathBuf::from))
        .ok_or_else(|| anyhow!("--jar <path> or TLA_TOOLS_JAR must be provided"))?;

    let jar_abs = fs::canonicalize(&jar_path)
        .with_context(|| format!("cannot resolve jar path: {}", jar_path.display()))?;

    if !jar_abs.is_file() {
        return Err(anyhow!("tla2tools.jar not found at {}", jar_abs.display()));
    }

    let script = format!(
        "#!/bin/sh\nexec java -cp \"{}\" tlc2.TLC \"$@\"\n",
        jar_abs.display()
    );

    fs::write(&out_path, script.as_bytes())
        .with_context(|| format!("failed to write wrapper to {}", out_path.display()))?;

    #[cfg(unix)]
    {
        let perm = fs::Permissions::from_mode(0o755);
        fs::set_permissions(&out_path, perm).with_context(|| {
            format!(
                "failed to mark wrapper executable at {}",
                out_path.display()
            )
        })?;
    }

    println!(
        "Created tlc wrapper at {} (uses {})",
        out_path.display(),
        jar_abs.display()
    );

    Ok(())
}

fn missing_hint(name: &str) -> String {
    match (env::consts::OS, name) {
        ("linux", "tlafmt") => "Missing tlafmt: install with `cargo install tlafmt` and ensure it is on PATH.".to_string(),
        ("linux", "tlc") => "Missing tlc: download the latest TLA+ tools (tla2tools.jar) from GitHub releases, then add a wrapper script `tlc` that runs `java -cp /path/to/tla2tools.jar tlc2.TLC` and put it on PATH. Snap `tlaplus` installs the Toolbox GUI only, not the `tlc` CLI.".to_string(),
        (_os, _) => format!("Missing {name}: install it for your OS and ensure it is on PATH (fill in host-specific steps)."),
    }
}
