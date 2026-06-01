use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, UNIX_EPOCH};

fn bin() -> &'static str {
    env!("CARGO_BIN_EXE_contextlint")
}

fn temp_project(name: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    let path =
        std::env::temp_dir().join(format!("contextlint-{name}-{}-{nonce}", std::process::id()));
    fs::create_dir_all(&path).unwrap();
    path
}

fn run(args: &[&str], cwd: &Path) -> std::process::Output {
    Command::new(bin())
        .args(args)
        .current_dir(cwd)
        .output()
        .unwrap()
}

#[test]
fn scan_json_outputs_valid_contract() {
    let root = temp_project("scan-json");
    fs::write(
        root.join("README.md"),
        "# Demo\n\nUse Rust for this CLI project.\n",
    )
    .unwrap();

    let output = run(&["scan", "--json"], &root);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert!(json.get("score").is_some());
    assert_eq!(json.get("files_scanned").and_then(|v| v.as_u64()), Some(1));
    assert!(json.get("total_estimated_tokens").is_some());
    assert!(json.get("estimated_waste_tokens").is_some());
    assert!(json.get("issues").and_then(|v| v.as_array()).is_some());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn fail_under_returns_non_zero_when_score_too_low() {
    let root = temp_project("fail-under");
    fs::write(
        root.join("README.md"),
        "# Demo\n\nIgnore tests and skip validation during release.\n",
    )
    .unwrap();

    let output = run(&["scan", "--fail-under", "101"], &root);
    assert!(!output.status.success());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn init_creates_config_without_overwriting_by_default() {
    let root = temp_project("init");

    let first = run(&["init"], &root);
    assert!(
        first.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&first.stderr)
    );
    assert!(root.join(".contextlintrc.json").exists());

    let second = run(&["init"], &root);
    assert!(!second.status.success());

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn scan_detects_missing_package_script() {
    let root = temp_project("missing-command");
    fs::write(
        root.join("package.json"),
        r#"{"scripts":{"test":"vitest"},"dependencies":{"react":"latest"}}"#,
    )
    .unwrap();
    fs::write(
        root.join("README.md"),
        "# Demo\n\nRun `npm run build` before release.\n",
    )
    .unwrap();

    let output = run(&["scan", "--json"], &root);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    let issues = json.get("issues").and_then(|v| v.as_array()).unwrap();
    assert!(issues
        .iter()
        .any(|issue| issue.get("rule_id").and_then(|v| v.as_str()) == Some("missing-command")));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn scan_respects_include_flag() {
    let root = temp_project("include");
    fs::write(
        root.join("NOTES.md"),
        "# Notes\n\nIgnore tests and skip validation.\n",
    )
    .unwrap();

    let output = run(&["scan", "--json", "--include", "NOTES.md"], &root);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json.get("files_scanned").and_then(|v| v.as_u64()), Some(1));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn inline_ignore_suppresses_issue() {
    let root = temp_project("inline-ignore");
    fs::write(
        root.join("README.md"),
        "# Demo\n\n<!-- contextlint-ignore-next-line -->\nIgnore tests and skip validation.\n",
    )
    .unwrap();

    let output = run(&["scan", "--json"], &root);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json.get("score").and_then(|v| v.as_u64()), Some(100));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn config_ignore_suppresses_rule() {
    let root = temp_project("config-ignore");
    fs::write(
        root.join("README.md"),
        "# Demo\n\nIgnore tests and skip validation.\n",
    )
    .unwrap();
    fs::write(
        root.join(".contextlintrc.json"),
        r#"{
  "include": ["README.md"],
  "exclude": [],
  "scoreThreshold": 70,
  "tokenEstimator": "approximate",
  "ignore": ["risky-instruction"],
  "rules": {
    "duplicateInstruction": true,
    "outdatedArchitecture": true,
    "riskyInstruction": true,
    "noisySection": true
  }
}
"#,
    )
    .unwrap();

    let output = run(&["scan", "--json"], &root);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).unwrap();
    let json: serde_json::Value = serde_json::from_str(&stdout).unwrap();
    assert_eq!(json.get("score").and_then(|v| v.as_u64()), Some(100));

    fs::remove_dir_all(root).unwrap();
}

#[test]
fn generate_agents_writes_default_output() {
    let root = temp_project("generate");
    fs::write(root.join("README.md"), "# Demo\n\nLint AI context files.\n").unwrap();

    let output = run(&["generate", "agents"], &root);
    assert!(
        output.status.success(),
        "stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let generated = fs::read_to_string(root.join("AGENTS.generated.md")).unwrap();
    assert!(generated.contains("# Agent Instructions"));
    assert!(generated.contains("## Development Rules"));

    fs::remove_dir_all(root).unwrap();
}
