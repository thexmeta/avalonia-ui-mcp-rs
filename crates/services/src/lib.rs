//! AvaloniaUI MCP Services Library
//!
//! This crate provides core services used throughout the MCP server:
//!
//! - [`error_handling`]: Async-safe error handling with tracing
//! - [`input_validation`]: Parameter validation service
//! - [`cache`]: Async resource caching with TTL support
//! - [`telemetry`]: Telemetry and metrics collection
//! - [`async_file`]: Async file operations
//! - [`markdown`]: Markdown output utilities (re-exports from core)

pub mod async_file;
pub mod cache;
pub mod error_handling;
pub mod input_validation;
pub mod markdown;
pub mod telemetry;

/// Prelude module for convenient imports
pub mod prelude {
    pub use crate::async_file::AsyncFileService;
    pub use crate::cache::ResourceCacheService;
    pub use crate::error_handling::ErrorHandlingService;
    pub use crate::input_validation::InputValidationService;
    pub use crate::telemetry::TelemetryService;
    pub use avalonia_mcp_core::markdown::MarkdownOutputBuilder;
}
