use model_context_protocol::{BoxFuture, McpTool, McpToolDefinition, ToolCallResult, ToolContent};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::sync::Arc;

use crate::mcp::server::McpState;
use crate::rules::scan_project;

pub struct ScanTool {
    pub state: Arc<McpState>,
}

#[derive(Deserialize)]
struct ScanParams {
    path: Option<String>,
}

#[derive(Serialize)]
struct ScanResponse {
    score: u8,
    files_scanned: usize,
    total_estimated_tokens: usize,
    issues_count: usize,
    status: String,
}

impl McpTool for ScanTool {
    fn definition(&self) -> McpToolDefinition {
        McpToolDefinition::new("scan")
            .with_description("Scan project for AI context waste and issues")
            .with_schema(serde_json::json!({
                "type": "object",
                "properties": {
                    "path": {
                        "type": "string",
                        "description": "Optional relative path to scan (defaults to project root)"
                    }
                }
            }))
    }

    fn call<'a>(&'a self, args: Value) -> BoxFuture<'a, ToolCallResult> {
        let state = Arc::clone(&self.state);
        Box::pin(async move {
            let params: ScanParams =
                serde_json::from_value(args).map_err(|e| format!("Invalid arguments: {}", e))?;

            let scan_path = params
                .path
                .map(|p| state.root.join(p))
                .unwrap_or_else(|| state.root.clone());

            let result = scan_project(&scan_path, &state.config, &[], &[])
                .map_err(|e| format!("Scan failed: {}", e))?;

            let status = match result.score {
                90..=100 => "Excellent",
                75..=89 => "Good",
                60..=74 => "Needs Cleanup",
                40..=59 => "Risky",
                _ => "Very Noisy",
            };

            let response = ScanResponse {
                score: result.score,
                files_scanned: result.files_scanned,
                total_estimated_tokens: result.total_estimated_tokens,
                issues_count: result.issues.len(),
                status: status.to_string(),
            };

            Ok(vec![ToolContent::text(
                serde_json::to_string_pretty(&response).unwrap(),
            )])
        })
    }
}
