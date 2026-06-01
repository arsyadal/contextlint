use serde::Serialize;

#[derive(Debug, Clone)]
pub struct ContextFile {
    pub path: String,
    pub file_type: ContextFileType,
    pub content: String,
    pub estimated_tokens: usize,
    pub sections: Vec<ContextSection>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ContextFileType {
    Claude,
    Agents,
    Cursor,
    Readme,
    Docs,
    Copilot,
    Unknown,
}

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct ContextSection {
    pub id: String,
    pub file_path: String,
    pub heading: Option<String>,
    pub start_line: usize,
    pub end_line: usize,
    pub content: String,
    pub estimated_tokens: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct FileSummary {
    pub path: String,
    pub file_type: ContextFileType,
    pub estimated_tokens: usize,
    pub sections: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct Issue {
    pub id: String,
    pub rule_id: String,
    pub severity: Severity,
    pub file_path: String,
    pub start_line: Option<usize>,
    pub end_line: Option<usize>,
    pub message: String,
    pub suggestion: Option<String>,
    pub confidence: f32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum Severity {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, Serialize)]
pub struct ScanResult {
    pub score: u8,
    pub files_scanned: usize,
    pub total_estimated_tokens: usize,
    pub estimated_waste_tokens: usize,
    pub files: Vec<FileSummary>,
    pub issues: Vec<Issue>,
}

impl ContextFile {
    pub fn summary(&self) -> FileSummary {
        FileSummary {
            path: self.path.clone(),
            file_type: self.file_type.clone(),
            estimated_tokens: self.estimated_tokens,
            sections: self.sections.len(),
        }
    }
}
