//! AvaloniaUI MCP Server
//!
//! This is the main entry point for the AvaloniaUI MCP server,
//! providing development assistance tools via the Model Context Protocol.

#[cfg(feature = "http")]
mod http_server;
mod server;

use anyhow::Result;
use rmcp::{ServiceExt, transport::stdio};
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{layer::SubscriberExt, Registry};
use tracing_subscriber::filter::EnvFilter;
use crate::server::AvaloniaMcpServer;

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command-line arguments
    let args: Vec<String> = std::env::args().collect();
    
    // Detect transport mode from command line args or environment
    let use_http_transport = args.iter().any(|arg| arg == "--http")
        || std::env::var("AVALONIA_MCP_TRANSPORT")
            .map(|v| v.to_lowercase() == "http")
            .unwrap_or(false);

    if use_http_transport {
        // Run with HTTP transport
        run_http_server(&args).await
    } else {
        // Run with STDIO transport (default)
        run_stdio_server().await
    }
}

/// Runs the MCP server using STDIO transport (default for local development)
async fn run_stdio_server() -> Result<()> {
    // Initialize logging to stderr
    Registry::default()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with(tracing_subscriber::fmt::layer()
            .with_writer(std::io::stderr)
            .with_ansi(false)
            .with_line_number(true)
            .with_target(false))
        .init();

    tracing::info!("Starting AvaloniaUI MCP Server (STDIO)");

    // Create server instance
    let server = AvaloniaMcpServer::new();

    // Serve with STDIO transport
    let service = server
        .serve(stdio())
        .await
        .inspect_err(|e| tracing::error!("serving error: {:?}", e))?;

    // Set up graceful shutdown handling
    let shutdown_handle = tokio::spawn(async move {
        // Wait for SIGINT or SIGTERM
        match tokio::signal::ctrl_c().await {
            Ok(()) => {
                tracing::info!("Shutdown requested, stopping server gracefully...");
            }
            Err(e) => {
                tracing::error!("Error setting up signal handler: {:?}", e);
            }
        }
    });

    // Wait for service to complete or shutdown signal
    tokio::select! {
        result = service.waiting() => {
            if let Err(e) = result {
                tracing::error!("Server error: {:?}", e);
            }
        }
        _ = shutdown_handle => {
            tracing::info!("Received shutdown signal");
        }
    }

    tracing::info!("AvaloniaUI MCP Server shutdown complete");
    Ok(())
}

/// Runs the MCP server using HTTP Streamable transport (for remote access)
#[cfg(feature = "http")]
async fn run_http_server(args: &[String]) -> Result<()> {
    use crate::http_server::run_http_server;

    // Initialize logging to stderr (same as STDIO mode)
    Registry::default()
        .with(EnvFilter::from_default_env().add_directive(tracing::Level::INFO.into()))
        .with(tracing_subscriber::fmt::layer()
            .with_writer(std::io::stderr)
            .with_ansi(false)
            .with_line_number(true)
            .with_target(false))
        .init();

    // Parse port and host from arguments (support both --port=5000 and --port 5000)
    let mut port: Option<u16> = None;
    let mut host: Option<String> = None;
    
    let mut i = 1; // Skip program name
    while i < args.len() {
        let arg = &args[i];
        if arg.starts_with("--port=") {
            if let Some(p) = arg.strip_prefix("--port=").and_then(|s| s.parse::<u16>().ok()) {
                port = Some(p);
            }
        } else if arg == "--port" && i + 1 < args.len() {
            i += 1;
            port = args[i].parse::<u16>().ok();
        } else if arg.starts_with("--host=") {
            host = arg.strip_prefix("--host=").map(String::from);
        } else if arg == "--host" && i + 1 < args.len() {
            i += 1;
            host = Some(args[i].to_string());
        }
        i += 1;
    }

    // Get from environment or use defaults
    let host = host
        .or_else(|| std::env::var("AVALONIA_MCP_HOST").ok())
        .unwrap_or_else(|| "0.0.0.0".to_string());

    let port = port
        .or_else(|| std::env::var("AVALONIA_MCP_PORT").ok().and_then(|s| s.parse().ok()))
        .unwrap_or(5000);

    eprintln!("Starting AvaloniaUI MCP Server (HTTP) on {}:{}", host, port);
    tracing::info!(
        host = %host,
        port = %port,
        "Starting AvaloniaUI MCP Server (HTTP)"
    );

    run_http_server(&host, port).await?;

    Ok(())
}

/// Fallback when HTTP feature is not enabled
#[cfg(not(feature = "http"))]
async fn run_http_server(_args: &[String]) -> Result<()> {
    anyhow::bail!(
        "HTTP transport is not enabled. \
         Rebuild with --features http or use STDIO transport."
    );
}
