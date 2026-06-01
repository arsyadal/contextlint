pub mod duplicate;
pub mod noisy;
pub mod outdated;
pub mod risky;

use std::collections::HashMap;
use std::path::Path;

use anyhow::Result;
use globset::Glob;

use crate::config::ContextlintConfig;
use crate::discovery::discover_context_files;
use crate::model::{ContextFile, Issue, ScanResult};
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

    issues.retain(|issue| !is_ignored(issue, &files, config));

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

fn is_ignored(issue: &Issue, files: &[ContextFile], config: &ContextlintConfig) -> bool {
    is_config_ignored(issue, &config.ignore) || is_inline_ignored(issue, files)
}

fn is_config_ignored(issue: &Issue, patterns: &[String]) -> bool {
    patterns.iter().any(|pattern| {
        let pattern = pattern.trim();
        if pattern.is_empty() {
            return false;
        }

        if pattern == issue.rule_id || pattern == issue.file_path {
            return true;
        }

        if let Some((rule, path_pattern)) = pattern.split_once(':') {
            return rule == issue.rule_id && glob_matches(path_pattern, &issue.file_path);
        }

        glob_matches(pattern, &issue.file_path)
    })
}

fn glob_matches(pattern: &str, value: &str) -> bool {
    Glob::new(pattern)
        .ok()
        .and_then(|glob| glob.compile_matcher().is_match(value).then_some(()))
        .is_some()
}

fn is_inline_ignored(issue: &Issue, files: &[ContextFile]) -> bool {
    let Some(line_no) = issue.start_line else {
        return false;
    };
    let Some(file) = files.iter().find(|file| file.path == issue.file_path) else {
        return false;
    };

    let lines: HashMap<usize, &str> = file
        .content
        .lines()
        .enumerate()
        .map(|(idx, line)| (idx + 1, line))
        .collect();

    let same_line = lines
        .get(&line_no)
        .is_some_and(|line| line.contains("contextlint-ignore"));
    let previous_line = line_no > 1
        && lines
            .get(&(line_no - 1))
            .is_some_and(|line| line.contains("contextlint-ignore-next-line"));

    same_line || previous_line
}

fn severity_rank(severity: crate::model::Severity) -> u8 {
    match severity {
        crate::model::Severity::Critical => 4,
        crate::model::Severity::High => 3,
        crate::model::Severity::Medium => 2,
        crate::model::Severity::Low => 1,
    }
}
