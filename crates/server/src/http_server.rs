//! HTTP transport server
//!
//! This module provides HTTP Streamable transport for remote MCP client connections.
//! It integrates with the actual MCP server to handle real tool calls.

use axum::{Router, routing::post, extract::State, Json};
use tower_http::trace::TraceLayer;
use tracing::{info, instrument};
use tokio::net::TcpListener;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use serde_json::{json, Map};

use crate::server::AvaloniaMcpServer;
use rmcp::model::CallToolResult;
use rmcp::ServerHandler;

/// Shared server state with MCP server instance
#[derive(Clone)]
pub struct HttpServerState {
    pub mcp_server: Arc<AvaloniaMcpServer>,
}

/// HTTP JSON-RPC request structure
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub method: String,
    #[serde(default)]
    pub params: Option<serde_json::Value>,
    pub id: Option<serde_json::Value>,
}

/// HTTP JSON-RPC response structure
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRpcResponse {
    pub jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<JsonRpcError>,
    pub id: Option<serde_json::Value>,
}

/// JSON-RPC error structure
#[derive(Debug, Deserialize, Serialize)]
pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Run HTTP server with graceful shutdown
pub async fn run_http_server(
    host: &str,
    port: u16,
) -> Result<(), anyhow::Error> {
    let mcp_server = Arc::new(AvaloniaMcpServer::new());
    
    let state = HttpServerState {
        mcp_server,
    };

    let app = Router::new()
        .route("/mcp", post(handle_mcp_request))
        .route("/health", post(health_check))
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    let addr = format!("{}:{}", host, port);
    let listener = TcpListener::bind(&addr).await?;

    info!("AvaloniaUI MCP HTTP Server listening on {}", addr);
    eprintln!("AvaloniaUI MCP HTTP Server listening on {}", addr);

    // Use tokio::select! for graceful shutdown
    let server = axum::serve(listener, app);

    tokio::select! {
        biased;
        _ = tokio::signal::ctrl_c() => {
            info!("HTTP server shutdown requested");
            eprintln!("Shutdown requested, stopping server...");
        }
        result = server => {
            if let Err(e) = result {
                eprintln!("Server error: {:?}", e);
                return Err(e.into());
            }
        }
    }

    Ok(())
}

/// Health check endpoint
async fn health_check(
    State(_state): State<HttpServerState>,
) -> &'static str {
    "OK"
}

/// Handle MCP JSON-RPC request
#[instrument(skip(state))]
async fn handle_mcp_request(
    State(state): State<HttpServerState>,
    Json(request): Json<JsonRpcRequest>,
) -> Json<JsonRpcResponse> {
    info!(
        method = %request.method,
        "Handling MCP request"
    );

    let response = match request.method.as_str() {
        // JSON-RPC 2.0 methods
        "ping" => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: Some(serde_json::Value::Null),
            error: None,
            id: request.id,
        },
        
        // MCP Protocol methods
        "initialize" => {
            // Return server info for initialize
            let server_info = state.mcp_server.get_info();
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(json!(server_info)),
                error: None,
                id: request.id,
            }
        }
        
        "tools/list" => {
            // List all available tools
            match state.mcp_server.list_tools_for_http().await {
                Ok(tools) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: Some(json!(tools)),
                    error: None,
                    id: request.id,
                },
                Err(e) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32603,
                        message: format!("Internal error: {}", e),
                        data: None,
                    }),
                    id: request.id,
                }
            }
        }
        
        "tools/call" => {
            // Call a specific tool
            let params = request.params.unwrap_or(serde_json::Value::Null);
            
            // Extract name and arguments from params
            let name = params.get("name")
                .and_then(|v| v.as_str())
                .unwrap_or("")
                .to_string();
            
            let arguments = params.get("arguments")
                .and_then(|v| v.as_object())
                .cloned()
                .unwrap_or_else(|| Map::new());
            
            match state.mcp_server.call_tool_for_http(&name, &arguments).await {
                Ok(result) => {
                    // Convert CallToolResult to JSON
                    let result_json = call_tool_result_to_json(result);
                    JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        result: Some(result_json),
                        error: None,
                        id: request.id,
                    }
                },
                Err(e) => JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32603,
                        message: e,
                        data: None,
                    }),
                    id: request.id,
                }
            }
        }
        
        // Unknown method
        _ => JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            result: None,
            error: Some(JsonRpcError {
                code: -32601,
                message: format!("Method not found: {}", request.method),
                data: None,
            }),
            id: request.id,
        }
    };

    Json(response)
}

/// Convert CallToolResult to JSON value
fn call_tool_result_to_json(result: CallToolResult) -> serde_json::Value {
    json!({
        "content": result.content,
        "isError": result.is_error,
        "meta": result.meta
    })
}
