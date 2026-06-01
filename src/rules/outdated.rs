use std::collections::HashSet;
use std::path::Path;

use crate::model::{ContextFile, Issue, Severity};

const MARKERS: &[&str] = &[
    "legacy",
    "deprecated",
    "old",
    "temporary",
    "todo later",
    "previous architecture",
];

const TECH_MAP: &[(&str, &str)] = &[
    ("express.js", "express"),
    ("express", "express"),
    ("nestjs", "@nestjs/core"),
    ("next.js", "next"),
    ("react", "react"),
    ("vue", "vue"),
    ("svelte", "svelte"),
    ("django", "django"),
    ("flask", "flask"),
    ("tailwind", "tailwindcss"),
];

pub fn detect(root: &Path, files: &[ContextFile]) -> Vec<Issue> {
    let manifests = read_manifest_dependencies(root);
    let mut issues = Vec::new();
    let mut issue_id = 1usize;

    for file in files {
        for (idx, line) in file.content.lines().enumerate() {
            let line_no = idx + 1;
            let lower = line.to_lowercase();

            for marker in MARKERS {
                if lower.contains(marker) {
                    issues.push(Issue {
                        id: format!("outdated-note-{issue_id}"),
                        rule_id: "outdated-architecture-note".into(),
                        severity: Severity::Low,
                        file_path: file.path.clone(),
                        start_line: Some(line_no),
                        end_line: Some(line_no),
                        message: format!("Potentially outdated marker found: '{marker}'."),
                        suggestion: Some(
                            "Verify this note is still current or move it to an archive.".into(),
                        ),
                        confidence: 0.65,
                    });
                    issue_id += 1;
                }
            }

            for reference in extract_backticks(line) {
                if looks_like_path(&reference) && !path_exists(root, &reference) {
                    issues.push(Issue {
                        id: format!("missing-file-reference-{issue_id}"),
                        rule_id: "missing-file-reference".into(),
                        severity: Severity::Medium,
                        file_path: file.path.clone(),
                        start_line: Some(line_no),
                        end_line: Some(line_no),
                        message: format!("Referenced path `{reference}` does not exist."),
                        suggestion: Some(
                            "Update the reference or create the missing file/folder.".into(),
                        ),
                        confidence: 0.85,
                    });
                    issue_id += 1;
                }
            }

            if !manifests.is_empty() {
                for (tech, dependency) in TECH_MAP {
                    if lower.contains(tech) && !manifests.contains(*dependency) {
                        issues.push(Issue {
                            id: format!("outdated-dependency-{issue_id}"),
                            rule_id: "outdated-architecture-note".into(),
                            severity: Severity::Medium,
                            file_path: file.path.clone(),
                            start_line: Some(line_no),
                            end_line: Some(line_no),
                            message: format!(
                                "Docs mention {tech}, but dependency `{dependency}` was not found."
                            ),
                            suggestion: Some(
                                "Verify technology stack docs against dependency manifests.".into(),
                            ),
                            confidence: 0.7,
                        });
                        issue_id += 1;
                    }
                }
            }
        }
    }

    issues
}

fn extract_backticks(line: &str) -> Vec<String> {
    let mut refs = Vec::new();
    let mut rest = line;
    while let Some(start) = rest.find('`') {
        let after_start = &rest[start + 1..];
        let Some(end) = after_start.find('`') else {
            break;
        };
        refs.push(after_start[..end].trim().to_string());
        rest = &after_start[end + 1..];
    }
    refs
}

fn looks_like_path(value: &str) -> bool {
    if value.is_empty()
        || value.contains(' ')
        || value.contains('*')
        || value.starts_with("http://")
        || value.starts_with("https://")
    {
        return false;
    }

    value.contains('/') || value.starts_with("./") || value.starts_with("../")
}

fn path_exists(root: &Path, reference: &str) -> bool {
    let cleaned = reference.trim_start_matches("./");
    root.join(cleaned).exists()
}

fn read_manifest_dependencies(root: &Path) -> HashSet<String> {
    let mut deps = HashSet::new();
    read_package_json(root, &mut deps);
    read_cargo_toml(root, &mut deps);
    read_go_mod(root, &mut deps);
    read_requirements(root, &mut deps);
    deps
}

fn read_package_json(root: &Path, deps: &mut HashSet<String>) {
    let path = root.join("package.json");
    let Ok(content) = std::fs::read_to_string(path) else {
        return;
    };
    let Ok(json) = serde_json::from_str::<serde_json::Value>(&content) else {
        return;
    };

    for section in [
        "dependencies",
        "devDependencies",
        "peerDependencies",
        "optionalDependencies",
    ] {
        if let Some(obj) = json.get(section).and_then(|value| value.as_object()) {
            deps.extend(obj.keys().map(|key| key.to_lowercase()));
        }
    }
}

fn read_cargo_toml(root: &Path, deps: &mut HashSet<String>) {
    let path = root.join("Cargo.toml");
    let Ok(content) = std::fs::read_to_string(path) else {
        return;
    };

    let mut in_deps = false;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with('[') {
            in_deps = matches!(
                trimmed,
                "[dependencies]" | "[dev-dependencies]" | "[build-dependencies]"
            );
            continue;
        }
        if !in_deps || trimmed.starts_with('#') || !trimmed.contains('=') {
            continue;
        }
        if let Some((name, _)) = trimmed.split_once('=') {
            deps.insert(name.trim().trim_matches('"').to_lowercase());
        }
    }
}

fn read_go_mod(root: &Path, deps: &mut HashSet<String>) {
    let path = root.join("go.mod");
    let Ok(content) = std::fs::read_to_string(path) else {
        return;
    };
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("require ") {
            if let Some(dep) = trimmed.split_whitespace().nth(1) {
                deps.insert(dep.to_lowercase());
            }
        }
    }
}

fn read_requirements(root: &Path, deps: &mut HashSet<String>) {
    let path = root.join("requirements.txt");
    let Ok(content) = std::fs::read_to_string(path) else {
        return;
    };
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        let name = trimmed
            .split(['=', '<', '>', '~', '!'])
            .next()
            .unwrap_or(trimmed)
            .trim()
            .to_lowercase();
        deps.insert(name);
    }
}
