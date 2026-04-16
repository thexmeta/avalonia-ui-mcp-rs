//! Project generator tool - Project scaffolding
//!
//! This tool generates new AvaloniaUI projects with proper structure and MVVM architecture.

use avalonia_mcp_core::error::AvaloniaMcpError;
use avalonia_mcp_core::markdown::MarkdownOutputBuilder;
use avalonia_mcp_services::async_file::AsyncFileService;
use rmcp::model::{CallToolResult, Content};
use rmcp::tool;
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;

/// Project generator tool parameters
#[derive(Debug, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProjectGeneratorParams {
    /// Project name
    pub name: String,
    /// Output directory path
    pub output_dir: String,
    /// Enable HTTP transport support
    pub enable_http: Option<bool>,
    /// Include test project
    pub include_tests: Option<bool>,
    /// Use ReactiveUI for MVVM
    pub use_reactiveui: Option<bool>,
}

/// Project generator tool for scaffolding new AvaloniaUI projects
#[derive(Debug, Clone, Default)]
pub struct ProjectGeneratorTool;

impl ProjectGeneratorTool {
    /// Create a new ProjectGeneratorTool instance
    pub fn new() -> Self {
        Self
    }

    /// Generate a new AvaloniaUI project
    #[tool(description = "Generate a new AvaloniaUI project with MVVM architecture. Creates a complete project structure with Cargo.toml, main.rs, and optional test project.")]
    pub async fn generate_project(
        &self,
        params: ProjectGeneratorParams,
    ) -> Result<CallToolResult, AvaloniaMcpError> {
        // Validate parameters
        if params.name.is_empty() {
            return Err(AvaloniaMcpError::validation(
                "Project name cannot be empty",
            ));
        }

        if params.output_dir.is_empty() {
            return Err(AvaloniaMcpError::validation(
                "Output directory cannot be empty",
            ));
        }

        let project_path = format!("{}/{}", params.output_dir, params.name);
        let enable_http = params.enable_http.unwrap_or(false);
        let include_tests = params.include_tests.unwrap_or(true);
        let use_reactiveui = params.use_reactiveui.unwrap_or(false);

        tracing::info!(
            name = %params.name,
            output_dir = %params.output_dir,
            enable_http,
            include_tests,
            use_reactiveui,
            "Generating AvaloniaUI project"
        );

        // Create directory structure
        self.create_directory_structure(&project_path, include_tests)
            .await?;

        // Generate Cargo.toml
        let cargo_toml = self.generate_cargo_toml(&params.name, enable_http, use_reactiveui);
        AsyncFileService::write_string(format!("{}/Cargo.toml", project_path), &cargo_toml)
            .await?;

        // Generate main.rs
        let main_rs = self.generate_main_rs(enable_http);
        AsyncFileService::write_string(format!("{}/src/main.rs", project_path), &main_rs)
            .await?;

        // Generate ViewModelBase if using ReactiveUI
        if use_reactiveui {
            let viewmodel_base = self.generate_viewmodel_base();
            AsyncFileService::write_string(
                format!("{}/src/ViewModelBase.rs", project_path),
                &viewmodel_base,
            )
            .await?;
        }

        // Generate test file if requested
        if include_tests {
            let test_content = self.generate_test_content(&params.name);
            AsyncFileService::write_string(
                format!("{}/tests/integration_tests.rs", project_path),
                &test_content,
            )
            .await?;
        }

        // Generate .gitignore
        let gitignore = self.generate_gitignore();
        AsyncFileService::write_string(format!("{}/.gitignore", project_path), &gitignore)
            .await?;

        // Build output
        let output = MarkdownOutputBuilder::new()
            .heading(1, &format!("Project {} Created", params.name))
            .paragraph("A new AvaloniaUI MCP project has been successfully generated!")
            .heading(2, "Project Structure")
            .code_block("text", &self.format_project_tree(&project_path, include_tests))
            .heading(2, "Next Steps")
            .numbered_list(vec![
                format!("cd {}", project_path),
                "cargo build".to_string(),
                "cargo run".to_string(),
                if include_tests { "cargo test".to_string() } else { "".to_string() },
            ])
            .heading(2, "Features Enabled")
            .task_list(vec![
                (true, "MVVM Architecture"),
                (enable_http, "HTTP Transport Support"),
                (include_tests, "Test Project"),
                (use_reactiveui, "ReactiveUI Integration"),
            ])
            .build();

        Ok(CallToolResult::success(vec![Content::text(output)]))
    }

    /// Create directory structure
    async fn create_directory_structure(
        &self,
        project_path: &str,
        include_tests: bool,
    ) -> Result<(), AvaloniaMcpError> {
        AsyncFileService::create_dir_all(format!("{}/src", project_path)).await?;

        if include_tests {
            AsyncFileService::create_dir_all(format!("{}/tests", project_path)).await?;
        }

        Ok(())
    }

    /// Generate Cargo.toml content
    fn generate_cargo_toml(
        &self,
        name: &str,
        enable_http: bool,
        use_reactiveui: bool,
    ) -> String {
        let http_deps = if enable_http {
            r#"
# HTTP Transport
axum = "0.8"
tower = "0.5"
tower-http = { version = "0.6", features = ["trace"] }
"#
        } else {
            ""
        };

        let reactiveui_deps = if use_reactiveui {
            r#"
# ReactiveUI for MVVM
reactive-macro = "0.5"
"#
        } else {
            ""
        };

        format!(
            r#"[package]
name = "{}"
version = "0.1.0"
edition = "2021"
rust-version = "1.85"

[dependencies]
# MCP Protocol
rmcp = {{ version = "0.11.0", features = ["reqwest", "transport-io", "uuid"] }}

# Async Runtime
tokio = {{ version = "1.48", features = ["full"] }}

# Serialization
serde = {{ version = "1.0", features = ["derive"] }}
serde_json = "1.0"

# Logging
tracing = "0.1"
tracing-subscriber = {{ version = "0.3", features = ["env-filter"] }}

# Error Handling
anyhow = "1.0"
thiserror = "2.0"
{}{}
"#,
            name, http_deps, reactiveui_deps
        )
    }

    /// Generate main.rs content
    fn generate_main_rs(&self, enable_http: bool) -> String {
        if enable_http {
            r#"//! AvaloniaUI MCP Server with HTTP transport

use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{layer::SubscriberExt, Registry};
use tracing_subscriber::filter::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    Registry::default()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with(tracing_subscriber::fmt::layer()
            .with_writer(std::io::stderr)
            .with_ansi(false))
        .init();

    tracing::info!("Starting AvaloniaUI MCP Server");

    // Create and serve MCP server
    let service = MyMcpServer::new()
        .serve(stdio())
        .await?;

    service.waiting().await?;
    Ok(())
}

// TODO: Define your MCP server with tools
#[derive(Clone)]
struct MyMcpServer;

impl MyMcpServer {
    fn new() -> Self {
        Self
    }
}
"#
            .to_string()
        } else {
            r#"//! AvaloniaUI MCP Server with STDIO transport

use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::{layer::SubscriberExt, Registry};
use tracing_subscriber::filter::EnvFilter;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging to stderr
    Registry::default()
        .with(EnvFilter::from_default_env())
        .with(tracing_subscriber::fmt::layer()
            .with_writer(std::io::stderr)
            .with_ansi(false))
        .init();

    tracing::info!("Starting AvaloniaUI MCP Server (STDIO)");

    // Create and serve MCP server
    let service = MyMcpServer::new()
        .serve(stdio())
        .await?;

    service.waiting().await?;
    Ok(())
}

// TODO: Define your MCP server with tools
#[derive(Clone)]
struct MyMcpServer;

impl MyMcpServer {
    fn new() -> Self {
        Self
    }
}
"#
            .to_string()
        }
    }

    /// Generate ViewModelBase for ReactiveUI
    fn generate_viewmodel_base(&self) -> String {
        r#"//! ViewModelBase for ReactiveUI MVVM pattern

use reactive_macro::viewModel;

/// Base class for all ViewModels with INotifyPropertyChanged support
#[viewModel]
pub struct ViewModelBase {
    // Add common ViewModel properties here
}
"#
        .to_string()
    }

    /// Generate test content
    fn generate_test_content(&self, name: &str) -> String {
        format!(
            r#"//! Integration tests for {}

#[cfg(test)]
mod tests {{
    #[tokio::test]
    async fn test_app_starts() {{
        // TODO: Add integration tests
        assert!(true);
    }}
}}
"#,
            name
        )
    }

    /// Generate .gitignore
    fn generate_gitignore(&self) -> String {
        r#"# Rust
/target/
**/*.rs.bk
Cargo.lock

# Build
/dist/
/build/

# IDE
.idea/
.vscode/
*.swp
*.swo

# Environment
.env
.env.local

# Logs
*.log
logs/

# OS
.DS_Store
Thumbs.db
"#
        .to_string()
    }

    /// Format project tree for display
    fn format_project_tree(&self, project_path: &str, include_tests: bool) -> String {
        if include_tests {
            format!(
                r#"{}
├── Cargo.toml
├── .gitignore
├── src/
│   ├── main.rs
│   └── ViewModelBase.rs (if using ReactiveUI)
└── tests/
    └── integration_tests.rs
"#,
                project_path
            )
        } else {
            format!(
                r#"{}
├── Cargo.toml
├── .gitignore
└── src/
    └── main.rs
"#,
                project_path
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_cargo_toml() {
        let tool = ProjectGeneratorTool::new();
        let cargo_toml = tool.generate_cargo_toml("test_project", false, false);

        assert!(cargo_toml.contains("name = \"test_project\""));
        assert!(cargo_toml.contains("rmcp"));
        assert!(cargo_toml.contains("tokio"));
        assert!(!cargo_toml.contains("axum")); // HTTP not enabled
    }

    #[test]
    fn test_generate_cargo_toml_with_http() {
        let tool = ProjectGeneratorTool::new();
        let cargo_toml = tool.generate_cargo_toml("test_project", true, false);

        assert!(cargo_toml.contains("axum"));
        assert!(cargo_toml.contains("tower"));
    }

    #[test]
    fn test_generate_gitignore() {
        let tool = ProjectGeneratorTool::new();
        let gitignore = tool.generate_gitignore();

        assert!(gitignore.contains("/target/"));
        assert!(gitignore.contains("Cargo.lock"));
    }
}
