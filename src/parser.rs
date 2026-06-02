use std::path::Path;

use anyhow::Result;

use crate::discovery::normalize_path;
use crate::model::{ContextFile, ContextFileType, ContextSection};
use crate::token::estimate_tokens;

pub fn parse_context_file(root: &Path, path: &Path) -> Result<ContextFile> {
    let content = std::fs::read_to_string(path)?;
    let rel = path.strip_prefix(root).unwrap_or(path);
    let rel_path = normalize_path(rel);
    Ok(parse_context_file_from_content(&rel_path, &content))
}

pub fn parse_context_file_from_content(rel_path: &str, content: &str) -> ContextFile {
    let file_type = classify_file(rel_path);
    let sections = parse_sections(rel_path, content);
    let estimated_tokens = estimate_tokens(content);

    ContextFile {
        path: rel_path.to_string(),
        file_type,
        content: content.to_string(),
        estimated_tokens,
        sections,
    }
}

pub fn classify_file(rel_path: &str) -> ContextFileType {
    match rel_path {
        "CLAUDE.md" => ContextFileType::Claude,
        "AGENTS.md" => ContextFileType::Agents,
        ".cursorrules" => ContextFileType::Cursor,
        "README.md" => ContextFileType::Readme,
        ".github/copilot-instructions.md" => ContextFileType::Copilot,
        _ if rel_path.starts_with("docs/") => ContextFileType::Docs,
        _ if rel_path.starts_with(".cursor/rules/") => ContextFileType::Cursor,
        _ => ContextFileType::Unknown,
    }
}

pub fn parse_sections(file_path: &str, content: &str) -> Vec<ContextSection> {
    let lines: Vec<&str> = content.lines().collect();
    if lines.is_empty() {
        return vec![ContextSection {
            id: format!("{file_path}:1"),
            file_path: file_path.to_string(),
            heading: None,
            start_line: 1,
            end_line: 1,
            content: String::new(),
            estimated_tokens: 0,
        }];
    }

    let mut sections = Vec::new();
    let mut start_line = 1usize;
    let mut heading: Option<String> = None;
    let mut buffer: Vec<&str> = Vec::new();

    for (idx, line) in lines.iter().enumerate() {
        let line_no = idx + 1;
        if is_heading(line) && !buffer.is_empty() {
            push_section(
                &mut sections,
                file_path,
                heading.take(),
                start_line,
                line_no - 1,
                &buffer,
            );
            buffer.clear();
            start_line = line_no;
        }

        if is_heading(line) {
            heading = Some(line.trim_start_matches('#').trim().to_string());
        }
        buffer.push(line);
    }

    if !buffer.is_empty() {
        push_section(
            &mut sections,
            file_path,
            heading,
            start_line,
            lines.len(),
            &buffer,
        );
    }

    sections
}

fn is_heading(line: &str) -> bool {
    let trimmed = line.trim_start();
    trimmed.starts_with('#') && trimmed.chars().take_while(|c| *c == '#').count() <= 6
}

fn push_section(
    sections: &mut Vec<ContextSection>,
    file_path: &str,
    heading: Option<String>,
    start_line: usize,
    end_line: usize,
    lines: &[&str],
) {
    let content = lines.join("\n");
    sections.push(ContextSection {
        id: format!("{file_path}:{start_line}"),
        file_path: file_path.to_string(),
        heading,
        start_line,
        end_line,
        estimated_tokens: estimate_tokens(&content),
        content,
    });
}
