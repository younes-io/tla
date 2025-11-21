use crate::lint::types::{Diagnostic, Severity};
use anyhow::Result;
use serde::Serialize;
use std::io::{self, Write};

#[derive(Serialize)]
struct SerializableDiagnostic<'a> {
    file: &'a str,
    line: usize,
    column: usize,
    severity: &'a str,
    code: String,
    message: &'a str,
}

pub fn print_human(diags: &[Diagnostic]) -> Result<()> {
    let mut out = io::BufWriter::new(io::stdout());
    for d in diags {
        let sev = match d.severity {
            Severity::Warning => "WARN",
            Severity::Error => "ERROR",
        };
        writeln!(
            out,
            "{}:{}:{} [{} {}] {}",
            d.file,
            d.line,
            d.column,
            sev,
            format_code(&d.code),
            d.message
        )?;
    }
    out.flush()?;
    Ok(())
}

pub fn print_json(diags: &[Diagnostic]) -> Result<()> {
    let json = to_json(diags)?;
    println!("{json}");
    Ok(())
}

pub fn to_json(diags: &[Diagnostic]) -> Result<String> {
    let serializable: Vec<SerializableDiagnostic<'_>> = diags
        .iter()
        .map(|d| SerializableDiagnostic {
            file: &d.file,
            line: d.line,
            column: d.column,
            severity: match d.severity {
                Severity::Warning => "warning",
                Severity::Error => "error",
            },
            code: format_code(&d.code),
            message: &d.message,
        })
        .collect();
    Ok(serde_json::to_string_pretty(&serializable)?)
}

fn format_code(code: &crate::lint::types::RuleCode) -> String {
    format!("{:?}", code)
}
