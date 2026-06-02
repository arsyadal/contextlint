use std::path::PathBuf;
use std::sync::Arc;

use anyhow::Result;
use model_context_protocol::McpServerConfig;
use model_context_protocol::McpStdioServer;

use crate::config::ContextlintConfig;
use crate::mcp::tools::ScanTool;

pub struct McpState {
    pub root: PathBuf,
    pub config: ContextlintConfig,
}

pub async fn run_mcp_server(root: PathBuf) -> Result<()> {
    let config = crate::config::load_config(&root).unwrap_or_default();
    let state = Arc::new(McpState {
        root: root.clone(),
        config,
    });

    let config = McpServerConfig::builder()
        .name("contextlint")
        .version(env!("CARGO_PKG_VERSION"))
        .with_tool(ScanTool { state })
        .build();

    McpStdioServer::run(config).await.map_err(|e| anyhow::anyhow!("MCP Server error: {}", e))?;

    Ok(())
}
