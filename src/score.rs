use crate::model::{Issue, Severity};

pub fn score_issues(issues: &[Issue]) -> u8 {
    let penalty: i32 = issues.iter().map(|issue| weight(issue.severity)).sum();
    (100 - penalty).clamp(0, 100) as u8
}

pub fn estimate_waste_tokens(total_tokens: usize, issues: &[Issue]) -> usize {
    let severity_units: usize = issues
        .iter()
        .map(|issue| match issue.severity {
            Severity::Critical => 600,
            Severity::High => 350,
            Severity::Medium => 180,
            Severity::Low => 75,
        })
        .sum();
    severity_units.min(total_tokens / 2)
}

fn weight(severity: Severity) -> i32 {
    match severity {
        Severity::Critical => 20,
        Severity::High => 12,
        Severity::Medium => 6,
        Severity::Low => 2,
    }
}
