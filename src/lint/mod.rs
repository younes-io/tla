use crate::lint::types::{Diagnostic, RuleCode, Severity};
use crate::tla_parser::TlaParser;
use anyhow::{Result, anyhow};
use std::fs;
use std::path::{Path, PathBuf};
use walkdir::WalkDir;

pub mod reporter;
pub mod rules;
pub mod types;

#[derive(Debug)]
pub struct FileContext {
    pub path: PathBuf,
    pub src: String,
    line_starts: Vec<usize>,
}

impl FileContext {
    pub fn new(path: PathBuf, src: String) -> Self {
        let mut line_starts = vec![0];
        for (idx, ch) in src.char_indices() {
            if ch == '\n' {
                line_starts.push(idx + 1);
            }
        }
        Self {
            path,
            src,
            line_starts,
        }
    }

    pub fn position(&self, byte_offset: usize) -> (usize, usize) {
        // line index via binary search
        let line_idx = match self.line_starts.binary_search(&byte_offset) {
            Ok(i) => i,
            Err(i) => i.saturating_sub(1),
        };
        let line_start = *self.line_starts.get(line_idx).unwrap_or(&0);
        let col_chars = self.src[line_start..byte_offset].chars().count() + 1;
        (line_idx + 1, col_chars)
    }
}

pub fn run(paths: Vec<PathBuf>, json: bool) -> Result<()> {
    let diagnostics = collect_diagnostics(paths)?;

    if json {
        reporter::print_json(&diagnostics)?;
    } else {
        reporter::print_human(&diagnostics)?;
    }

    if diagnostics.iter().any(|d| d.severity == Severity::Error) {
        return Err(anyhow!("lint errors"));
    }

    Ok(())
}

pub fn collect_diagnostics(paths: Vec<PathBuf>) -> Result<Vec<Diagnostic>> {
    let files = collect_tla_files(paths);
    let mut parser = TlaParser::new()?;
    let mut diagnostics = Vec::new();

    for path in files {
        let src = fs::read_to_string(&path)?;
        let tree = match parser.parse(&src) {
            Some(t) => t,
            None => {
                diagnostics.push(Diagnostic {
                    file: path.to_string_lossy().into_owned(),
                    line: 0,
                    column: 0,
                    severity: Severity::Error,
                    code: RuleCode::TLA000,
                    message: "Failed to parse TLA+ file".to_string(),
                });
                continue;
            }
        };

        let ctx = FileContext::new(path.clone(), src);
        rules::run_all_rules(&ctx, &tree, &mut diagnostics);
    }

    Ok(diagnostics)
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
