use std::path::{Path, PathBuf};

use anyhow::Result;
use globset::{Glob, GlobSet, GlobSetBuilder};
use walkdir::{DirEntry, WalkDir};

use crate::config::ContextlintConfig;

pub fn discover_context_files(
    root: &Path,
    config: &ContextlintConfig,
    include: &[String],
    exclude: &[String],
) -> Result<Vec<PathBuf>> {
    let include_patterns = if include.is_empty() {
        config.include.clone()
    } else {
        let mut merged = config.include.clone();
        merged.extend_from_slice(include);
        merged
    };

    let mut exclude_patterns = config.exclude.clone();
    exclude_patterns.extend_from_slice(exclude);

    let include_set = build_globset(&include_patterns)?;
    let exclude_set = build_globset(&exclude_patterns)?;
    let mut files = Vec::new();

    for entry in WalkDir::new(root)
        .into_iter()
        .filter_entry(|entry| should_enter(entry))
        .filter_map(Result::ok)
    {
        if !entry.file_type().is_file() {
            continue;
        }

        let path = entry.path();
        let rel = path.strip_prefix(root).unwrap_or(path);
        let rel_string = normalize_path(rel);

        if exclude_set.is_match(&rel_string) {
            continue;
        }

        if include_set.is_match(&rel_string) {
            files.push(path.to_path_buf());
        }
    }

    files.sort();
    files.dedup();
    Ok(files)
}

fn should_enter(entry: &DirEntry) -> bool {
    let name = entry.file_name().to_string_lossy();
    !matches!(
        name.as_ref(),
        ".git" | "node_modules" | "dist" | "build" | ".next" | "coverage" | "target"
    )
}

fn build_globset(patterns: &[String]) -> Result<GlobSet> {
    let mut builder = GlobSetBuilder::new();
    for pattern in patterns {
        builder.add(Glob::new(pattern)?);
    }
    Ok(builder.build()?)
}

pub fn normalize_path(path: &Path) -> String {
    path.to_string_lossy().replace('\\', "/")
}
