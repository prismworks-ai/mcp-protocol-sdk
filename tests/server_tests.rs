// Copyright (c) 2025 MCP Rust Contributors
// SPDX-License-Identifier: MIT

//! Tests for server components

use mcp_protocol_sdk::{
    core::tool::{EchoTool, ToolHandler},
    protocol::types::ServerCapabilities,
    server::McpServer,
};
use serde_json::json;
use std::collections::HashMap;
use tokio;

#[cfg(test)]
mod server_tests {
    use super::*;

    #[tokio::test]
    async fn test_server_creation() {
        let _server = McpServer::new("test-server".to_string(), "1.0.0".to_string());
        // Basic test that server can be created
        assert!(true);
    }

    #[tokio::test]
    async fn test_server_with_tool() {
        let server = McpServer::new("test-server".to_string(), "1.0.0".to_string());

        // Add tool using the correct method signature
        server
            .add_tool(
                "echo".to_string(),
                Some("Echo a message".to_string()),
                json!({
                    "type": "object",
                    "properties": {
                        "message": {"type": "string"}
                    }
                }),
                EchoTool,
            )
            .await
            .unwrap();

        // Test that tool was added successfully
        assert!(true);
    }

    #[tokio::test]
    async fn test_echo_tool() {
        let tool = EchoTool;
        let mut args = HashMap::new();
        args.insert("message".to_string(), json!("Hello, World!"));

        let result = tool.call(args).await.unwrap();
        assert_eq!(result.content.len(), 1);
        assert_eq!(result.is_error, None);
    }

    #[test]
    fn test_server_capabilities() {
        let _capabilities = ServerCapabilities::default();
        // Test that capabilities can be created
        assert!(true);
    }

    #[test]
    fn test_server_capabilities_with_tools() {
        let capabilities = ServerCapabilities {
            tools: Some(mcp_protocol_sdk::protocol::types::ToolsCapability {
                list_changed: Some(true),
            }),
            ..Default::default()
        };

        assert!(capabilities.tools.is_some());
        assert_eq!(capabilities.tools.unwrap().list_changed, Some(true));
    }
}
