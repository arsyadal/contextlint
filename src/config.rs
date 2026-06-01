use std::path::Path;

use anyhow::{bail, Result};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default)]
pub struct ContextlintConfig {
    pub include: Vec<String>,
    pub exclude: Vec<String>,
    #[serde(rename = "scoreThreshold")]
    pub score_threshold: Option<u8>,
    #[serde(rename = "tokenEstimator")]
    pub token_estimator: String,
    pub rules: RuleConfig,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(default, rename_all = "camelCase")]
pub struct RuleConfig {
    pub duplicate_instruction: bool,
    pub outdated_architecture: bool,
    pub risky_instruction: bool,
    pub noisy_section: bool,
}

impl Default for ContextlintConfig {
    fn default() -> Self {
        Self {
            include: vec![
                "CLAUDE.md".into(),
                "AGENTS.md".into(),
                ".cursorrules".into(),
                "README.md".into(),
                "docs/**/*.md".into(),
                ".cursor/rules/*".into(),
                ".github/copilot-instructions.md".into(),
            ],
            exclude: vec![
                "node_modules/**".into(),
                ".git/**".into(),
                "dist/**".into(),
                "build/**".into(),
                ".next/**".into(),
                "coverage/**".into(),
                "target/**".into(),
            ],
            score_threshold: None,
            token_estimator: "approximate".into(),
            rules: RuleConfig::default(),
        }
    }
}

impl Default for RuleConfig {
    fn default() -> Self {
        Self {
            duplicate_instruction: true,
            outdated_architecture: true,
            risky_instruction: true,
            noisy_section: true,
        }
    }
}

pub fn load_config(root: &Path) -> Result<ContextlintConfig> {
    let path = root.join(".contextlintrc.json");
    if !path.exists() {
        return Ok(ContextlintConfig::default());
    }

    let content = std::fs::read_to_string(&path)?;
    let config = serde_json::from_str(&content)?;
    Ok(config)
}

pub fn init_config(root: &Path, force: bool) -> Result<()> {
    let path = root.join(".contextlintrc.json");
    if path.exists() && !force {
        bail!(
            "{} already exists; use --force to overwrite",
            path.display()
        );
    }

    let config = ContextlintConfig::default();
    let json = serde_json::to_string_pretty(&config)?;
    std::fs::write(&path, format!("{json}\n"))?;
    println!("Created {}", path.display());
    Ok(())
}
