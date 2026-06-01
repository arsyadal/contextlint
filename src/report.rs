use std::path::Path;

use anyhow::Result;

use crate::model::{ScanResult, Severity};

pub fn print_json(result: &ScanResult) -> Result<()> {
    println!("{}", serde_json::to_string_pretty(result)?);
    Ok(())
}

pub fn print_terminal(root: &Path, result: &ScanResult) {
    println!("ContextLint v{}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("Project: {}", root.display());
    println!();

    println!("Files scanned: {}", result.files_scanned);
    for file in &result.files {
        println!("✓ {} ({} tokens)", file.path, file.estimated_tokens);
    }
    println!();

    println!("Score: {}/100", result.score);
    println!("Status: {}", score_status(result.score));
    println!();

    println!("Token Summary:");
    println!(
        "- Total estimated tokens: {}",
        result.total_estimated_tokens
    );
    println!("- Estimated waste: {}", result.estimated_waste_tokens);
    if result.total_estimated_tokens > 0 {
        let reduction = result.estimated_waste_tokens * 100 / result.total_estimated_tokens;
        println!("- Potential reduction: {reduction}%");
    }
    println!();

    if result.issues.is_empty() {
        println!("No issues found.");
        return;
    }

    println!("Top Issues:");
    for issue in result.issues.iter().take(20) {
        let line = issue
            .start_line
            .map(|line| format!(":{line}"))
            .unwrap_or_default();
        println!();
        println!("[{}] {}", severity_label(issue.severity), issue.message);
        println!("File: {}{}", issue.file_path, line);
        if let Some(suggestion) = &issue.suggestion {
            println!("Suggestion: {suggestion}");
        }
    }

    if result.issues.len() > 20 {
        println!();
        println!("... {} more issues", result.issues.len() - 20);
    }
}

pub fn render_markdown(root: &Path, result: &ScanResult) -> String {
    let mut out = String::new();
    out.push_str("# ContextLint Report\n\n");
    out.push_str(&format!("Project: `{}`\n\n", root.display()));
    out.push_str("## Summary\n\n");
    out.push_str(&format!(
        "- Score: **{}/100** ({})\n",
        result.score,
        score_status(result.score)
    ));
    out.push_str(&format!("- Files scanned: {}\n", result.files_scanned));
    out.push_str(&format!(
        "- Total estimated tokens: {}\n",
        result.total_estimated_tokens
    ));
    out.push_str(&format!(
        "- Estimated waste tokens: {}\n\n",
        result.estimated_waste_tokens
    ));

    out.push_str("## Files\n\n");
    for file in &result.files {
        out.push_str(&format!(
            "- `{}` — {} tokens\n",
            file.path, file.estimated_tokens
        ));
    }

    out.push_str("\n## Issues\n\n");
    if result.issues.is_empty() {
        out.push_str("No issues found.\n");
        return out;
    }

    for issue in &result.issues {
        let line = issue
            .start_line
            .map(|line| format!(":{line}"))
            .unwrap_or_default();
        out.push_str(&format!(
            "### [{}] {}\n\n",
            severity_label(issue.severity),
            issue.message
        ));
        out.push_str(&format!("- Rule: `{}`\n", issue.rule_id));
        out.push_str(&format!("- Location: `{}{}`\n", issue.file_path, line));
        out.push_str(&format!("- Confidence: {:.2}\n", issue.confidence));
        if let Some(suggestion) = &issue.suggestion {
            out.push_str(&format!("- Suggestion: {}\n", suggestion));
        }
        out.push('\n');
    }

    out
}

fn score_status(score: u8) -> &'static str {
    match score {
        90..=100 => "Excellent",
        75..=89 => "Good",
        60..=74 => "Needs Cleanup",
        40..=59 => "Risky",
        _ => "Very Noisy",
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
