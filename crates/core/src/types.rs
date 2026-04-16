//! Common types used across the AvaloniaUI MCP server
//!
//! This module defines shared types for parameters, validation results,
//! and tool execution results.

use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Common parameters for tool validation
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct CommonParams {
    /// Optional project path for context-aware operations
    pub project_path: Option<String>,
    /// Enable verbose output
    pub verbose: Option<bool>,
}

/// Validation result with detailed feedback
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ValidationResult {
    /// Whether the validation passed
    pub is_valid: bool,
    /// List of validation errors
    pub errors: Vec<String>,
    /// List of validation warnings
    pub warnings: Vec<String>,
}

impl ValidationResult {
    /// Create a successful validation result
    pub fn success() -> Self {
        Self {
            is_valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        }
    }

    /// Create a failed validation result with errors
    pub fn failure(errors: Vec<String>) -> Self {
        Self {
            is_valid: false,
            errors,
            warnings: Vec::new(),
        }
    }

    /// Add a warning to the result
    pub fn with_warning(mut self, warning: impl Into<String>) -> Self {
        self.warnings.push(warning.into());
        self
    }

    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }

    /// Check if there are any warnings
    pub fn has_warnings(&self) -> bool {
        !self.warnings.is_empty()
    }

    /// Merge two validation results
    pub fn merge(mut self, other: ValidationResult) -> Self {
        self.is_valid = self.is_valid && other.is_valid;
        self.errors.extend(other.errors);
        self.warnings.extend(other.warnings);
        self
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        Self::success()
    }
}

/// Tool execution result
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ToolResult {
    /// Whether the tool execution was successful
    pub success: bool,
    /// Tool output as a string
    pub output: String,
    /// Optional metadata associated with the result
    pub metadata: Option<serde_json::Value>,
}

impl ToolResult {
    /// Create a successful tool result
    pub fn success(output: impl Into<String>) -> Self {
        Self {
            success: true,
            output: output.into(),
            metadata: None,
        }
    }

    /// Create a successful tool result with metadata
    pub fn success_with_metadata(
        output: impl Into<String>,
        metadata: serde_json::Value,
    ) -> Self {
        Self {
            success: true,
            output: output.into(),
            metadata: Some(metadata),
        }
    }

    /// Create a failed tool result
    pub fn failure(output: impl Into<String>) -> Self {
        Self {
            success: false,
            output: output.into(),
            metadata: None,
        }
    }

    /// Convert to an MCP CallToolResult
    pub fn to_mcp_result(self) -> rmcp::model::CallToolResult {
        if self.success {
            rmcp::model::CallToolResult::success(vec![rmcp::model::Content::text(self.output)])
        } else {
            // For errors, return success with error message in content
            // MCP doesn't have a direct error constructor
            rmcp::model::CallToolResult::success(vec![
                rmcp::model::Content::text(format!("Error: {}", self.output))
            ])
        }
    }
}

/// Server information structure
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ServerInfo {
    /// Server version
    pub version: String,
    /// Supported transport modes
    pub transports: Vec<String>,
    /// Available tools count
    pub tools_count: usize,
    /// Available resources count
    pub resources_count: usize,
    /// Server capabilities
    pub capabilities: ServerCapabilities,
}

/// Server capabilities
#[derive(Debug, Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct ServerCapabilities {
    /// Tools capability
    pub tools: bool,
    /// Resources capability
    pub resources: bool,
    /// Prompts capability
    pub prompts: bool,
    /// Logging capability
    pub logging: bool,
}

impl ServerCapabilities {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_tools(mut self) -> Self {
        self.tools = true;
        self
    }

    pub fn with_resources(mut self) -> Self {
        self.resources = true;
        self
    }

    pub fn with_prompts(mut self) -> Self {
        self.prompts = true;
        self
    }

    pub fn with_logging(mut self) -> Self {
        self.logging = true;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_validation_result_success() {
        let result = ValidationResult::success();
        assert!(result.is_valid);
        assert!(result.errors.is_empty());
        assert!(result.warnings.is_empty());
    }

    #[test]
    fn test_validation_result_failure() {
        let result = ValidationResult::failure(vec!["error1".to_string()]);
        assert!(!result.is_valid);
        assert_eq!(result.errors.len(), 1);
        assert!(result.has_errors());
    }

    #[test]
    fn test_validation_result_merge() {
        let result1 = ValidationResult::success().with_warning("warning1");
        let result2 = ValidationResult::failure(vec!["error1".to_string()]);
        let merged = result1.merge(result2);
        assert!(!merged.is_valid);
        assert_eq!(merged.errors.len(), 1);
        assert_eq!(merged.warnings.len(), 1);
    }

    #[test]
    fn test_tool_result_success() {
        let result = ToolResult::success("test output");
        assert!(result.success);
        assert_eq!(result.output, "test output");
        assert!(result.metadata.is_none());
    }

    #[test]
    fn test_tool_result_failure() {
        let result = ToolResult::failure("error message");
        assert!(!result.success);
        assert_eq!(result.output, "error message");
    }

    #[test]
    fn test_server_capabilities_builder() {
        let caps = ServerCapabilities::new()
            .with_tools()
            .with_resources()
            .with_logging();
        assert!(caps.tools);
        assert!(caps.resources);
        assert!(!caps.prompts);
        assert!(caps.logging);
    }
}
