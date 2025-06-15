//! Echo Server Example
//!
//! A simple MCP server that provides an echo tool to demonstrate
//! basic server functionality.

use async_trait::async_trait;
use serde_json::{json, Value};
use std::collections::HashMap;

use mcp_protocol_sdk::{
    core::{error::McpResult, tool::ToolHandler},
    protocol::types::{Content, ToolResult},
    server::McpServer,
    transport::stdio::StdioServerTransport,
};

/// Simple echo tool handler
struct EchoHandler;

#[async_trait]
impl ToolHandler for EchoHandler {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        let message = arguments
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("Hello, World!");

        let repeat_count = arguments
            .get("repeat")
            .and_then(|v| v.as_u64())
            .unwrap_or(1)
            .min(10); // Limit to prevent spam

        let separator = arguments
            .get("separator")
            .and_then(|v| v.as_str())
            .unwrap_or(" ");

        let mut responses = Vec::new();
        for _ in 0..repeat_count {
            responses.push(message.to_string());
        }

        let result = responses.join(separator);

        Ok(ToolResult {
            content: vec![Content::text(result)],
            is_error: None,
            meta: None,
        })
    }
}

#[tokio::main]
async fn main() -> McpResult<()> {
    // Initialize logging (only if tracing-subscriber feature is enabled)
    #[cfg(feature = "tracing-subscriber")]
    #[cfg(feature = "tracing-subscriber")]
    tracing_subscriber::fmt::init();

    let mut server = McpServer::new("echo-server".to_string(), "1.0.0".to_string());

    // Add the echo tool
    server
        .add_tool(
            "echo".to_string(),
            Some("Echo a message with optional repetition".to_string()),
            json!({
                "type": "object",
                "properties": {
                    "message": {
                        "type": "string",
                        "description": "The message to echo"
                    },
                    "repeat": {
                        "type": "integer",
                        "description": "Number of times to repeat the message (max 10)",
                        "minimum": 1,
                        "maximum": 10,
                        "default": 1
                    },
                    "separator": {
                        "type": "string",
                        "description": "Separator between repeated messages",
                        "default": " "
                    }
                },
                "required": ["message"]
            }),
            EchoHandler,
        )
        .await?;

    // Start the server
    let transport = StdioServerTransport::new();
    server.start(transport).await?;

    // Keep running until interrupted
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl+c");
    server.stop().await?;

    Ok(())
}
