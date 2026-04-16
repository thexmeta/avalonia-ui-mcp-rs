//! AvaloniaUI MCP Core Library
//!
//! This crate provides the foundational types, traits, and error handling
//! for the AvaloniaUI MCP server implementation.
//!
//! # Modules
//!
//! - [`error`]: Core error types using `thiserror`
//! - [`types`]: Common types used across the server
//! - [`markdown`]: Markdown output builder for formatted responses

pub mod error;
pub mod markdown;
pub mod types;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::error::AvaloniaMcpError;
    pub use crate::types::{CommonParams, ToolResult, ValidationResult};
    pub use crate::markdown::MarkdownOutputBuilder;
}
