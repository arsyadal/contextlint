use crate::model::{ContextFile, Issue, Severity};

pub fn detect(files: &[ContextFile]) -> Vec<Issue> {
    let mut issues = Vec::new();
    let mut issue_id = 1usize;

    for file in files {
        if file.estimated_tokens > 4_000 {
            issues.push(Issue {
                id: format!("noisy-section-{issue_id}"),
                rule_id: "noisy-section".into(),
                severity: Severity::Low,
                file_path: file.path.clone(),
                start_line: Some(1),
                end_line: None,
                message: format!(
                    "File is large for agent context ({} estimated tokens).",
                    file.estimated_tokens
                ),
                suggestion: Some(
                    "Move long narrative/history into docs archive or generate compact AGENTS.md."
                        .into(),
                ),
                confidence: 0.9,
            });
            issue_id += 1;
        }

        for section in &file.sections {
            if section.estimated_tokens > 1_000 {
                let heading = section.heading.as_deref().unwrap_or("Untitled section");
                issues.push(Issue {
                    id: format!("noisy-section-{issue_id}"),
                    rule_id: "noisy-section".into(),
                    severity: Severity::Medium,
                    file_path: section.file_path.clone(),
                    start_line: Some(section.start_line),
                    end_line: Some(section.end_line),
                    message: format!(
                        "Section '{heading}' is noisy/long ({} estimated tokens).",
                        section.estimated_tokens
                    ),
                    suggestion: Some(
                        "Split, summarize, or keep only actionable instructions.".into(),
                    ),
                    confidence: 0.9,
                });
                issue_id += 1;
            }
        }
    }

    issues
}
