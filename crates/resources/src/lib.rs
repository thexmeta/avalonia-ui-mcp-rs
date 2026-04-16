//! AvaloniaUI MCP Resources Library
//!
//! This crate provides the knowledge base and resources for the MCP server,
//! including control references, XAML patterns, and migration guides.

pub mod loader;
pub mod types;

/// Re-export types for convenient access
pub use types::*;

/// Compile-time embedded knowledge base resources
pub mod data {
    include!(concat!(env!("OUT_DIR"), "/embedded_resources.rs"));
}
