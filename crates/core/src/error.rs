//! Core error types for AvaloniaUI MCP Server
//!
//! This module defines all error types used throughout the server,
//! providing detailed error information and proper error propagation.

use thiserror::Error;

/// Core error types for AvaloniaUI MCP Server
#[derive(Error, Debug, Clone)]
pub enum AvaloniaMcpError {
    /// Validation error for invalid parameters
    #[error("Validation error: {0}")]
    ValidationError(String),

    /// Cache operation error
    #[error("Cache error: {0}")]
    CacheError(String),

    /// Telemetry operation error
    #[error("Telemetry error: {0}")]
    TelemetryError(String),

    /// File I/O error
    #[error("File I/O error: {0}")]
    FileError(String),

    /// Tool execution error
    #[error("Tool execution error: {0}")]
    ToolError(String),

    /// Resource not found error
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    /// JSON serialization/deserialization error
    #[error("JSON error: {0}")]
    JsonError(String),

    /// HTTP client error
    #[error("HTTP error: {0}")]
    HttpError(String),

    /// Internal server error
    #[error("Internal error: {0}")]
    InternalError(String),

    /// MCP protocol error
    #[error("MCP protocol error: {0}")]
    McpProtocolError(String),
}

impl From<tokio::io::Error> for AvaloniaMcpError {
    fn from(err: tokio::io::Error) -> Self {
        Self::FileError(err.to_string())
    }
}

impl From<serde_json::Error> for AvaloniaMcpError {
    fn from(err: serde_json::Error) -> Self {
        Self::JsonError(err.to_string())
    }
}

impl From<reqwest::Error> for AvaloniaMcpError {
    fn from(err: reqwest::Error) -> Self {
        Self::HttpError(err.to_string())
    }
}

impl From<AvaloniaMcpError> for rmcp::ErrorData {
    fn from(err: AvaloniaMcpError) -> Self {
        rmcp::ErrorData::internal_error(err.to_string(), None)
    }
}

impl AvaloniaMcpError {
    /// Create a validation error
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::ValidationError(msg.into())
    }

    /// Create a cache error
    pub fn cache(msg: impl Into<String>) -> Self {
        Self::CacheError(msg.into())
    }

    /// Create a file error
    pub fn file(msg: impl Into<String>) -> Self {
        Self::FileError(msg.into())
    }

    /// Create a tool error
    pub fn tool(msg: impl Into<String>) -> Self {
        Self::ToolError(msg.into())
    }

    /// Create a resource not found error
    pub fn not_found(resource: impl Into<String>) -> Self {
        Self::ResourceNotFound(resource.into())
    }

    /// Create an internal error
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::InternalError(msg.into())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_display() {
        let err = AvaloniaMcpError::validation("test validation error");
        assert!(err.to_string().contains("Validation error"));
        assert!(err.to_string().contains("test validation error"));
    }

    #[test]
    fn test_error_from_io_error() {
        let io_err = tokio::io::Error::new(
            tokio::io::ErrorKind::NotFound,
            "file not found",
        );
        let err: AvaloniaMcpError = io_err.into();
        assert!(matches!(err, AvaloniaMcpError::FileError(_)));
    }

    #[test]
    fn test_error_constructors() {
        assert!(matches!(
            AvaloniaMcpError::validation("test"),
            AvaloniaMcpError::ValidationError(_)
        ));
        assert!(matches!(
            AvaloniaMcpError::cache("test"),
            AvaloniaMcpError::CacheError(_)
        ));
        assert!(matches!(
            AvaloniaMcpError::file("test"),
            AvaloniaMcpError::FileError(_)
        ));
        assert!(matches!(
            AvaloniaMcpError::tool("test"),
            AvaloniaMcpError::ToolError(_)
        ));
        assert!(matches!(
            AvaloniaMcpError::not_found("test"),
            AvaloniaMcpError::ResourceNotFound(_)
        ));
        assert!(matches!(
            AvaloniaMcpError::internal("test"),
            AvaloniaMcpError::InternalError(_)
        ));
    }
}
