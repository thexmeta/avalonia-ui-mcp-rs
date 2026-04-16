//! Echo tool - Basic connectivity testing
//!
//! This tool provides basic echo functionality for testing MCP server connectivity.

use avalonia_mcp_core::error::AvaloniaMcpError;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Echo tool parameters
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct EchoParams {
    /// Message to echo back
    pub message: String,
}

/// Echo tool for basic connectivity testing
#[derive(Debug, Clone, Default)]
pub struct EchoTool;

impl EchoTool {
    /// Create a new EchoTool instance
    pub fn new() -> Self {
        Self
    }

    /// Echo the message back to the client
    #[tool(description = "Echoes the message back to the client. Use this to test MCP server connectivity.")]
    pub async fn echo(&self, params: EchoParams) -> Result<CallToolResult, AvaloniaMcpError> {
        tracing::info!(message = %params.message, "Echo tool called");

        Ok(CallToolResult::success(vec![Content::text(format!(
            "Hello from AvaloniaUI MCP Server (Rust): {}",
            params.message
        ))]))
    }

    /// Get server information
    #[tool(description = "Gets information about the AvaloniaUI MCP server including version and capabilities")]
    pub async fn server_info(&self) -> Result<CallToolResult, AvaloniaMcpError> {
        let info = concat!(
            "AvaloniaUI MCP Server v1.0 (Rust)\n\n",
            "Capabilities:\n",
            "- Project generation with MVVM architecture\n",
            "- XAML syntax validation\n",
            "- WPF to Avalonia migration assistance\n",
            "- Security pattern generation\n",
            "- Performance analysis\n",
            "- Accessibility compliance checks\n\n",
            "Built with:\n",
            "- Rust programming language\n",
            "- tokio async runtime\n",
            "- rmcp (Model Context Protocol) framework\n\n",
            "Transport: STDIO (default) or HTTP Streamable"
        );

        Ok(CallToolResult::success(vec![Content::text(info)]))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_echo_success() {
        let tool = EchoTool::new();
        let params = EchoParams {
            message: "test message".to_string(),
        };

        let result = tool.echo(params).await.unwrap();
        assert!(result.is_error.is_none() || result.is_error == Some(false));
        assert!(result.content.len() == 1);
        // Content is now a string directly
        let content_str = format!("{:?}", result.content[0]);
        assert!(content_str.contains("test message"));
    }

    #[tokio::test]
    async fn test_server_info() {
        let tool = EchoTool::new();
        let result = tool.server_info().await.unwrap();

        assert!(result.is_error.is_none() || result.is_error == Some(false));
        assert!(result.content.len() == 1);
        let content_str = format!("{:?}", result.content[0]);
        assert!(content_str.contains("AvaloniaUI MCP Server"));
    }
}
