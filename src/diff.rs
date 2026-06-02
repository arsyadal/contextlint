use std::collections::HashSet;
use std::path::Path;
use std::process::Command;

use anyhow::{Context, Result};

use crate::config::ContextlintConfig;
use crate::discovery::discover_context_files;
use crate::model::{ContextFile, Issue, ScanResult, Severity};
use crate::parser::parse_context_file_from_content;
use crate::rules::scan_files;

pub struct DiffResult {
    pub base_commit: String,
    pub current: ScanResult,
    pub base: ScanResult,
    pub token_delta: i64,
    pub token_delta_percent: f32,
    pub new_issues: Vec<Issue>,
    pub resolved_issues: Vec<Issue>,
    pub score_delta: i64,
}

pub fn diff_project(
    root: &Path,
    base_commit: &str,
    config: &ContextlintConfig,
) -> Result<DiffResult> {
    verify_git_repo(root)?;

    let current_paths = discover_context_files(root, config, &[], &[])?;
    let current_files: Vec<ContextFile> = current_paths
        .iter()
        .filter_map(|path| parse_context_file(root, path).ok())
        .collect();

    let base_files: Vec<ContextFile> = current_paths
        .iter()
        .filter_map(|path| {
            let rel = path.strip_prefix(root).unwrap_or(path);
            let rel_str = rel.to_string_lossy().replace('\\', "/");
            let content = git_show_file(root, base_commit, &rel_str).ok()?;
            Some(parse_context_file_from_content(&rel_str, &content))
        })
        .collect();

    let current_result = scan_files(root, &current_files, config)?;
    let base_result = scan_files(root, &base_files, config)?;

    let current_issues_set: HashSet<String> = current_result
        .issues
        .iter()
        .map(issue_fingerprint)
        .collect();
    let base_issues_set: HashSet<String> =
        base_result.issues.iter().map(issue_fingerprint).collect();

    let new_issues: Vec<Issue> = current_result
        .issues
        .iter()
        .filter(|issue| !base_issues_set.contains(&issue_fingerprint(issue)))
        .cloned()
        .collect();

    let resolved_issues: Vec<Issue> = base_result
        .issues
        .iter()
        .filter(|issue| !current_issues_set.contains(&issue_fingerprint(issue)))
        .cloned()
        .collect();

    let token_delta =
        current_result.total_estimated_tokens as i64 - base_result.total_estimated_tokens as i64;
    let token_delta_percent = if base_result.total_estimated_tokens > 0 {
        (token_delta as f32 / base_result.total_estimated_tokens as f32) * 100.0
    } else {
        0.0
    };

    let score_delta = current_result.score as i64 - base_result.score as i64;

    Ok(DiffResult {
        base_commit: base_commit.to_string(),
        current: current_result,
        base: base_result,
        token_delta,
        token_delta_percent,
        new_issues,
        resolved_issues,
        score_delta,
    })
}

fn verify_git_repo(root: &Path) -> Result<()> {
    let output = Command::new("git")
        .args(["-C", &root.to_string_lossy(), "rev-parse", "--git-dir"])
        .output()
        .context("Failed to run git. Is git installed?")?;

    if !output.status.success() {
        anyhow::bail!("Not a git repository: {}", root.display());
    }
    Ok(())
}

fn git_show_file(root: &Path, commit: &str, rel_path: &str) -> Result<String> {
    let spec = format!("{commit}:{rel_path}");
    let output = Command::new("git")
        .args(["-C", &root.to_string_lossy(), "show", &spec])
        .output()
        .with_context(|| format!("Failed to run git show {spec}"))?;

    if !output.status.success() {
        anyhow::bail!("File not found at {commit}: {rel_path}");
    }
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

fn parse_context_file(root: &Path, path: &Path) -> Result<ContextFile> {
    let content = std::fs::read_to_string(path)?;
    let rel = path.strip_prefix(root).unwrap_or(path);
    let rel_str = rel.to_string_lossy().replace('\\', "/");
    Ok(parse_context_file_from_content(&rel_str, &content))
}

fn issue_fingerprint(issue: &Issue) -> String {
    format!(
        "{}|{}|{}|{}",
        issue.rule_id,
        issue.file_path,
        issue.start_line.unwrap_or(0),
        issue.message
    )
}

pub fn print_terminal(root: &Path, diff: &DiffResult) {
    println!("ContextLint Diff");
    println!();
    println!("Project: {}", root.display());
    println!("Base: {} → HEAD", diff.base_commit);
    println!();

    let token_sign = if diff.token_delta >= 0 { "+" } else { "" };
    println!(
        "Context tokens: {}{} ({} tokens / {:.1}%)",
        token_sign, diff.token_delta, diff.base.total_estimated_tokens, diff.token_delta_percent
    );

    let score_sign = if diff.score_delta >= 0 { "+" } else { "" };
    println!(
        "Score: {}{} ({} → {})",
        score_sign, diff.score_delta, diff.base.score, diff.current.score
    );
    println!();

    if !diff.new_issues.is_empty() {
        println!("New issues:");
        for issue in &diff.new_issues {
            let line = issue
                .start_line
                .map(|line| format!(":{line}"))
                .unwrap_or_default();
            println!(
                "[{}] {} — {}{}",
                severity_label(issue.severity),
                issue.message,
                issue.file_path,
                line
            );
        }
        println!();
    }

    if !diff.resolved_issues.is_empty() {
        println!("Resolved issues:");
        for issue in &diff.resolved_issues {
            let line = issue
                .start_line
                .map(|line| format!(":{line}"))
                .unwrap_or_default();
            println!(
                "[{}] {} — {}{}",
                severity_label(issue.severity),
                issue.message,
                issue.file_path,
                line
            );
        }
        println!();
    }

    if diff.new_issues.is_empty() && diff.resolved_issues.is_empty() {
        println!("No context changes detected.");
    }
}

fn severity_label(severity: Severity) -> &'static str {
    match severity {
        Severity::Low => "LOW",
        Severity::Medium => "MEDIUM",
        Severity::High => "HIGH",
        Severity::Critical => "CRITICAL",
    }
}
