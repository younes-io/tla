use crate::tooling::ensure_tool;
use anyhow::{Context, Result, anyhow};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use walkdir::WalkDir;

pub fn run(paths: Vec<PathBuf>) -> Result<()> {
    ensure_tool("tlafmt")?;

    let files = collect_tla_files(paths);
    let mut failed = false;

    for file in files {
        let output = Command::new("tlafmt")
            .arg(&file)
            .output()
            .with_context(|| "failed to spawn tlafmt (is it on PATH?)")?;

        if !output.status.success() {
            failed = true;
            eprintln!("tlafmt failed on {}: {}", file.display(), output.status);
            if !output.stdout.is_empty() {
                eprintln!("{}", String::from_utf8_lossy(&output.stdout));
            }
            if !output.stderr.is_empty() {
                eprintln!("{}", String::from_utf8_lossy(&output.stderr));
            }
        }
    }

    if failed {
        return Err(anyhow!("formatter errors"));
    }

    Ok(())
}

fn collect_tla_files(paths: Vec<PathBuf>) -> Vec<PathBuf> {
    let mut out = Vec::new();
    for path in paths {
        if path.is_dir() {
            let walker = WalkDir::new(&path)
                .follow_links(false)
                .into_iter()
                .filter_entry(|e| !is_hidden_entry(e) && !e.file_type().is_symlink());
            for entry in walker.filter_map(Result::ok) {
                let p = entry.path();
                if is_tla_file(p) {
                    out.push(p.to_path_buf());
                }
            }
        } else if is_tla_file(&path) && !is_hidden(&path) && !is_symlink_path(&path) {
            out.push(path);
        }
    }
    out.sort();
    out
}

fn is_tla_file(path: &Path) -> bool {
    path.extension()
        .and_then(|s| s.to_str())
        .map(|ext| ext.eq_ignore_ascii_case("tla"))
        .unwrap_or(false)
}

fn is_hidden(path: &Path) -> bool {
    path.file_name()
        .and_then(|n| n.to_str())
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}

fn is_symlink_path(path: &Path) -> bool {
    fs::symlink_metadata(path)
        .map(|m| m.file_type().is_symlink())
        .unwrap_or(false)
}

fn is_hidden_entry(entry: &walkdir::DirEntry) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|name| name.starts_with('.'))
        .unwrap_or(false)
}
