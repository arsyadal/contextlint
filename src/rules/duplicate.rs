use std::collections::{HashMap, HashSet};

use crate::model::{ContextFile, Issue, Severity};

#[derive(Debug, Clone)]
struct Candidate {
    file_path: String,
    line: usize,
    norm: String,
}

pub fn detect(files: &[ContextFile]) -> Vec<Issue> {
    let candidates = collect_candidates(files);
    let mut issues = Vec::new();
    let mut issue_id = 1usize;
    let mut grouped: HashMap<String, Vec<&Candidate>> = HashMap::new();

    for candidate in &candidates {
        grouped
            .entry(candidate.norm.clone())
            .or_default()
            .push(candidate);
    }

    let mut duplicate_norms = HashSet::new();
    for (norm, group) in grouped.iter().filter(|(_, group)| group.len() > 1) {
        duplicate_norms.insert(norm.clone());
        let files_count = unique_files(group);
        issues.push(Issue {
            id: format!("duplicate-instruction-{issue_id}"),
            rule_id: "duplicate-instruction".into(),
            severity: Severity::Medium,
            file_path: group[0].file_path.clone(),
            start_line: Some(group[0].line),
            end_line: Some(group[0].line),
            message: format!("Duplicate instruction found in {files_count} files."),
            suggestion: Some(
                "Keep this instruction in AGENTS.md when possible and remove duplicates.".into(),
            ),
            confidence: 1.0,
        });
        issue_id += 1;
    }

    let mut seen_pairs = HashSet::new();
    for i in 0..candidates.len() {
        for j in (i + 1)..candidates.len() {
            let left = &candidates[i];
            let right = &candidates[j];
            if left.file_path == right.file_path || duplicate_norms.contains(&left.norm) {
                continue;
            }
            let key = ordered_pair_key(&left.norm, &right.norm);
            if !seen_pairs.insert(key) {
                continue;
            }

            let similarity = jaccard(&left.norm, &right.norm);
            if similarity >= 0.82 {
                issues.push(Issue {
                    id: format!("duplicate-instruction-{issue_id}"),
                    rule_id: "duplicate-instruction".into(),
                    severity: Severity::Medium,
                    file_path: left.file_path.clone(),
                    start_line: Some(left.line),
                    end_line: Some(left.line),
                    message: format!(
                        "Near-duplicate instruction found in {} and {}.",
                        left.file_path, right.file_path
                    ),
                    suggestion: Some(
                        "Merge these instructions into one canonical agent rule.".into(),
                    ),
                    confidence: similarity as f32,
                });
                issue_id += 1;
            }
        }
    }

    issues
}

fn collect_candidates(files: &[ContextFile]) -> Vec<Candidate> {
    let mut candidates = Vec::new();

    for file in files {
        let mut in_code = false;
        for (idx, line) in file.content.lines().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("```") {
                in_code = !in_code;
                continue;
            }
            if in_code || trimmed.starts_with('#') || trimmed.is_empty() {
                continue;
            }

            let text = strip_list_prefix(trimmed).trim().to_string();
            let words = text.split_whitespace().count();
            if !(8..=80).contains(&words) || text.chars().count() < 25 {
                continue;
            }

            let norm = normalize(&text);
            if norm.split_whitespace().count() < 8 {
                continue;
            }

            candidates.push(Candidate {
                file_path: file.path.clone(),
                line: idx + 1,
                norm,
            });
        }
    }

    candidates
}

fn strip_list_prefix(line: &str) -> &str {
    let line = line.trim_start_matches(['-', '*', '+']).trim_start();
    let mut chars = line.char_indices();
    let mut end_digits = 0usize;
    while let Some((idx, ch)) = chars.next() {
        if ch.is_ascii_digit() {
            end_digits = idx + ch.len_utf8();
            continue;
        }
        if ch == '.' && end_digits > 0 {
            return line[end_digits + 1..].trim_start();
        }
        break;
    }
    line
}

fn normalize(text: &str) -> String {
    let lowered = text.to_lowercase();
    let cleaned: String = lowered
        .chars()
        .map(|ch| {
            if ch.is_alphanumeric() || ch.is_whitespace() {
                ch
            } else {
                ' '
            }
        })
        .collect();
    cleaned.split_whitespace().collect::<Vec<_>>().join(" ")
}

fn jaccard(left: &str, right: &str) -> f64 {
    let left: HashSet<&str> = left.split_whitespace().collect();
    let right: HashSet<&str> = right.split_whitespace().collect();
    let union = left.union(&right).count();
    if union == 0 {
        return 0.0;
    }
    left.intersection(&right).count() as f64 / union as f64
}

fn unique_files(group: &[&Candidate]) -> usize {
    group
        .iter()
        .map(|candidate| candidate.file_path.as_str())
        .collect::<HashSet<_>>()
        .len()
}

fn ordered_pair_key(left: &str, right: &str) -> String {
    if left <= right {
        format!("{left}\n{right}")
    } else {
        format!("{right}\n{left}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn normalizes_text() {
        assert_eq!(
            normalize("Use TypeScript, for all new files!"),
            "use typescript for all new files"
        );
    }

    #[test]
    fn computes_jaccard() {
        assert!(jaccard("use rust for cli", "use rust for cli") > 0.99);
    }
}
