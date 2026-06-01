use crate::model::{ContextFile, Issue, Severity};

const PHRASES: &[(&str, Severity)] = &[
    ("delete all", Severity::Critical),
    ("drop database", Severity::Critical),
    ("drop table", Severity::Critical),
    ("wipe production", Severity::Critical),
    ("disable auth", Severity::High),
    ("use production database", Severity::High),
    ("hardcode token", Severity::High),
    ("ignore tests", Severity::Medium),
    ("skip validation", Severity::Medium),
    ("bypass security", Severity::Medium),
];

pub fn detect(files: &[ContextFile]) -> Vec<Issue> {
    let mut issues = Vec::new();
    let mut issue_id = 1usize;

    for file in files {
        for (idx, line) in file.content.lines().enumerate() {
            let lower = line.to_lowercase();
            for (phrase, severity) in PHRASES {
                if lower.contains(phrase) {
                    issues.push(Issue {
                        id: format!("risky-instruction-{issue_id}"),
                        rule_id: "risky-instruction".into(),
                        severity: *severity,
                        file_path: file.path.clone(),
                        start_line: Some(idx + 1),
                        end_line: Some(idx + 1),
                        message: format!("Risky instruction contains '{phrase}'."),
                        suggestion: Some(
                            "Remove unsafe instruction or add explicit review/backup requirement."
                                .into(),
                        ),
                        confidence: 0.95,
                    });
                    issue_id += 1;
                }
            }
        }
    }

    issues
}
