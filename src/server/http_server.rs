//! HTTP-specific MCP server implementation
//!
//! This module provides a specialized MCP server that integrates directly with HTTP transport.

use crate::core::error::McpResult;
use crate::protocol::types::{JsonRpcRequest, JsonRpcResponse};
use crate::server::mcp_server::McpServer;
use crate::transport::http::HttpServerTransport;
use crate::transport::traits::ServerTransport;
use std::sync::Arc;
use tokio::sync::Mutex;

/// HTTP-specific MCP server that properly integrates with HTTP transport
pub struct HttpMcpServer {
    server: Arc<Mutex<McpServer>>,
    transport: Option<HttpServerTransport>,
}

impl HttpMcpServer {
    /// Create a new HTTP MCP server
    pub fn new(name: String, version: String) -> Self {
        Self {
            server: Arc::new(Mutex::new(McpServer::new(name, version))),
            transport: None,
        }
    }

    /// Get a reference to the underlying MCP server
    pub async fn server(&self) -> Arc<Mutex<McpServer>> {
        self.server.clone()
    }

    /// Start the HTTP server with proper request handling integration
    pub async fn start(&mut self, mut transport: HttpServerTransport) -> McpResult<()> {
        // Set up the request handler to use the MCP server
        let server_clone = self.server.clone();

        transport
            .set_request_handler(move |request: JsonRpcRequest| {
                let server = server_clone.clone();
                let (tx, rx) = tokio::sync::oneshot::channel();

                tokio::spawn(async move {
                    let server_guard = server.lock().await;
                    let response = server_guard
                        .handle_request(request)
                        .await
                        .unwrap_or_else(|e| {
                            tracing::error!("Error handling HTTP request: {}", e);
                            JsonRpcResponse {
                                jsonrpc: "2.0".to_string(),
                                id: serde_json::Value::Null,
                                result: Some(serde_json::json!({
                                    "error": {
                                        "code": -32603,
                                        "message": e.to_string()
                                    }
                                })),
                            }
                        });
                    let _ = tx.send(response);
                });

                rx
            })
            .await;

        // Start the transport
        transport.start().await?;

        self.transport = Some(transport);
        Ok(())
    }

    /// Stop the HTTP server
    pub async fn stop(&mut self) -> McpResult<()> {
        if let Some(transport) = &mut self.transport {
            transport.stop().await?;
        }
        self.transport = None;
        Ok(())
    }

    /// Check if the server is running
    pub fn is_running(&self) -> bool {
        self.transport.as_ref().map_or(false, |t| t.is_running())
    }
}
