pub mod duplicate;
pub mod noisy;
pub mod outdated;
pub mod risky;

use std::path::Path;

use anyhow::Result;

use crate::config::ContextlintConfig;
use crate::discovery::discover_context_files;
use crate::model::{ContextFile, ScanResult};
use crate::parser::parse_context_file;
use crate::score::{estimate_waste_tokens, score_issues};

pub fn scan_project(
    root: &Path,
    config: &ContextlintConfig,
    include: &[String],
    exclude: &[String],
) -> Result<ScanResult> {
    let paths = discover_context_files(root, config, include, exclude)?;
    let mut files: Vec<ContextFile> = Vec::new();

    for path in paths {
        files.push(parse_context_file(root, &path)?);
    }

    let mut issues = Vec::new();
    if config.rules.duplicate_instruction {
        issues.extend(duplicate::detect(&files));
    }
    if config.rules.noisy_section {
        issues.extend(noisy::detect(&files));
    }
    if config.rules.risky_instruction {
        issues.extend(risky::detect(&files));
    }
    if config.rules.outdated_architecture {
        issues.extend(outdated::detect(root, &files));
    }

    issues.sort_by(|a, b| {
        severity_rank(b.severity)
            .cmp(&severity_rank(a.severity))
            .then_with(|| a.file_path.cmp(&b.file_path))
            .then_with(|| a.start_line.cmp(&b.start_line))
    });

    let total_estimated_tokens = files.iter().map(|file| file.estimated_tokens).sum();
    let score = score_issues(&issues);
    let estimated_waste_tokens = estimate_waste_tokens(total_estimated_tokens, &issues);
    let file_summaries = files.iter().map(ContextFile::summary).collect();

    Ok(ScanResult {
        score,
        files_scanned: files.len(),
        total_estimated_tokens,
        estimated_waste_tokens,
        files: file_summaries,
        issues,
    })
}

fn severity_rank(severity: crate::model::Severity) -> u8 {
    match severity {
        crate::model::Severity::Critical => 4,
        crate::model::Severity::High => 3,
        crate::model::Severity::Medium => 2,
        crate::model::Severity::Low => 1,
    }
}
