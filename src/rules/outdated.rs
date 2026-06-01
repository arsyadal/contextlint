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

#[derive(Debug, Default)]
struct ManifestInfo {
    dependencies: HashSet<String>,
    package_scripts: HashSet<String>,
    make_targets: HashSet<String>,
    just_targets: HashSet<String>,
    has_cargo: bool,
    has_package_json: bool,
    has_makefile: bool,
    has_justfile: bool,
}

pub fn detect(root: &Path, files: &[ContextFile]) -> Vec<Issue> {
    let manifests = read_manifest_info(root);
    let mut issues = Vec::new();
    let mut issue_id = 1usize;

    for file in files {
        for (idx, line) in file.content.lines().enumerate() {
            let line_no = idx + 1;
            let lower = line.to_lowercase();

            for marker in MARKERS {
                if marker_in_line(&lower, marker) {
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

                if let Some(reason) = missing_command_reason(&reference, &manifests) {
                    issues.push(Issue {
                        id: format!("missing-command-{issue_id}"),
                        rule_id: "missing-command".into(),
                        severity: Severity::Medium,
                        file_path: file.path.clone(),
                        start_line: Some(line_no),
                        end_line: Some(line_no),
                        message: format!("Referenced command `{reference}` is not available: {reason}."),
                        suggestion: Some(
                            "Update docs to use an existing script/command or add the missing command."
                                .into(),
                        ),
                        confidence: 0.85,
                    });
                    issue_id += 1;
                }
            }

            if !manifests.dependencies.is_empty() {
                for (tech, dependency) in TECH_MAP {
                    if lower.contains(tech) && !manifests.dependencies.contains(*dependency) {
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

fn marker_in_line(line: &str, marker: &str) -> bool {
    if marker.contains(' ') {
        return line.contains(marker);
    }

    line.split(|ch: char| !ch.is_alphanumeric())
        .any(|word| word == marker)
}

fn missing_command_reason(command: &str, info: &ManifestInfo) -> Option<String> {
    let words: Vec<&str> = command.split_whitespace().collect();
    if words.is_empty() {
        return None;
    }

    match words.as_slice() {
        ["npm", "run", script, ..] => missing_npm_script(script, info),
        ["pnpm", "run", script, ..] => missing_npm_script(script, info),
        ["pnpm", script, ..] if !is_package_manager_builtin(script) => {
            missing_npm_script(script, info)
        }
        ["yarn", script, ..] if !is_package_manager_builtin(script) => {
            missing_npm_script(script, info)
        }
        ["cargo", subcommand, ..] => missing_cargo_subcommand(subcommand, info),
        ["make", target, ..] => missing_make_target(target, info),
        ["just", target, ..] => missing_just_target(target, info),
        _ => None,
    }
}

fn missing_npm_script(script: &str, info: &ManifestInfo) -> Option<String> {
    if !info.has_package_json || info.package_scripts.contains(script) {
        None
    } else {
        Some(format!("script `{script}` not found in package.json"))
    }
}

fn missing_cargo_subcommand(subcommand: &str, info: &ManifestInfo) -> Option<String> {
    if !info.has_cargo || is_cargo_builtin(subcommand) {
        None
    } else {
        Some(format!(
            "cargo subcommand `{subcommand}` is not a known built-in"
        ))
    }
}

fn missing_make_target(target: &str, info: &ManifestInfo) -> Option<String> {
    if !info.has_makefile || info.make_targets.contains(target) {
        None
    } else {
        Some(format!("target `{target}` not found in Makefile"))
    }
}

fn missing_just_target(target: &str, info: &ManifestInfo) -> Option<String> {
    if !info.has_justfile || info.just_targets.contains(target) {
        None
    } else {
        Some(format!("recipe `{target}` not found in justfile"))
    }
}

fn is_package_manager_builtin(script: &str) -> bool {
    matches!(
        script,
        "add"
            | "audit"
            | "config"
            | "create"
            | "dedupe"
            | "dlx"
            | "exec"
            | "install"
            | "link"
            | "list"
            | "outdated"
            | "remove"
            | "run"
            | "test"
            | "update"
            | "upgrade"
    )
}

fn is_cargo_builtin(subcommand: &str) -> bool {
    matches!(
        subcommand,
        "add"
            | "bench"
            | "build"
            | "check"
            | "clean"
            | "clippy"
            | "doc"
            | "fetch"
            | "fix"
            | "fmt"
            | "generate-lockfile"
            | "install"
            | "metadata"
            | "new"
            | "package"
            | "publish"
            | "remove"
            | "run"
            | "search"
            | "test"
            | "tree"
            | "update"
            | "vendor"
    )
}

fn read_manifest_info(root: &Path) -> ManifestInfo {
    let mut info = ManifestInfo::default();
    read_package_json(root, &mut info);
    read_cargo_toml(root, &mut info);
    read_go_mod(root, &mut info);
    read_requirements(root, &mut info);
    read_makefile(root, &mut info);
    read_justfile(root, &mut info);
    info
}

fn read_package_json(root: &Path, info: &mut ManifestInfo) {
    let path = root.join("package.json");
    let Ok(content) = std::fs::read_to_string(path) else {
        return;
    };
    info.has_package_json = true;
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
            info.dependencies
                .extend(obj.keys().map(|key| key.to_lowercase()));
        }
    }

    if let Some(obj) = json.get("scripts").and_then(|value| value.as_object()) {
        info.package_scripts
            .extend(obj.keys().map(|key| key.to_string()));
    }
}

fn read_cargo_toml(root: &Path, info: &mut ManifestInfo) {
    let path = root.join("Cargo.toml");
    let Ok(content) = std::fs::read_to_string(path) else {
        return;
    };
    info.has_cargo = true;

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
            info.dependencies
                .insert(name.trim().trim_matches('"').to_lowercase());
        }
    }
}

fn read_go_mod(root: &Path, info: &mut ManifestInfo) {
    let path = root.join("go.mod");
    let Ok(content) = std::fs::read_to_string(path) else {
        return;
    };
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.starts_with("require ") {
            if let Some(dep) = trimmed.split_whitespace().nth(1) {
                info.dependencies.insert(dep.to_lowercase());
            }
        }
    }
}

fn read_requirements(root: &Path, info: &mut ManifestInfo) {
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
        info.dependencies.insert(name);
    }
}

fn read_makefile(root: &Path, info: &mut ManifestInfo) {
    let Some(content) = read_first_existing(root, &["Makefile", "makefile"]) else {
        return;
    };
    info.has_makefile = true;
    for line in content.lines() {
        if line.starts_with('\t') || line.trim_start().starts_with('#') {
            continue;
        }
        if let Some((target, _)) = line.split_once(':') {
            let target = target.trim();
            if !target.is_empty() && !target.contains(' ') && !target.contains('$') {
                info.make_targets.insert(target.to_string());
            }
        }
    }
}

fn read_justfile(root: &Path, info: &mut ManifestInfo) {
    let Some(content) = read_first_existing(root, &["justfile", "Justfile", ".justfile"]) else {
        return;
    };
    info.has_justfile = true;
    for line in content.lines() {
        let trimmed = line.trim();
        if trimmed.is_empty()
            || trimmed.starts_with('#')
            || line.starts_with(' ')
            || line.starts_with('\t')
        {
            continue;
        }
        let recipe = trimmed
            .split([':', ' ', '('])
            .next()
            .unwrap_or(trimmed)
            .trim();
        if !recipe.is_empty() {
            info.just_targets.insert(recipe.to_string());
        }
    }
}

fn read_first_existing(root: &Path, names: &[&str]) -> Option<String> {
    names
        .iter()
        .find_map(|name| std::fs::read_to_string(root.join(name)).ok())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn detects_missing_npm_script() {
        let info = ManifestInfo {
            has_package_json: true,
            package_scripts: HashSet::from(["test".to_string()]),
            ..ManifestInfo::default()
        };
        assert!(missing_command_reason("npm run build", &info).is_some());
        assert!(missing_command_reason("npm run test", &info).is_none());
    }

    #[test]
    fn detects_unknown_cargo_subcommand() {
        let info = ManifestInfo {
            has_cargo: true,
            ..ManifestInfo::default()
        };
        assert!(missing_command_reason("cargo banana", &info).is_some());
        assert!(missing_command_reason("cargo test", &info).is_none());
    }
}
