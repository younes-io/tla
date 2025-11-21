use crate::tooling::{tool_status, write_tlc_wrapper};
use anyhow::{Result, anyhow};
use std::path::PathBuf;

pub fn run(write_wrapper: Option<PathBuf>, jar: Option<PathBuf>) -> Result<()> {
    let tools = ["tlafmt", "tlc"];
    let mut missing = false;

    for tool in tools {
        let status = tool_status(tool);
        if status.present {
            if let Some(path) = status.path {
                println!("{tool}: OK ({})", path.display());
            } else {
                println!("{tool}: OK");
            }
        } else {
            missing = true;
            println!("{tool}: MISSING\n  {}", status.hint);
        }
    }

    if let Some(out_path) = write_wrapper {
        write_tlc_wrapper(out_path, jar)?;
    }

    if missing {
        Err(anyhow!("one or more external tools are missing"))
    } else {
        Ok(())
    }
}
