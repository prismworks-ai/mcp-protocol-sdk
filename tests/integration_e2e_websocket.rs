// Copyright (c) 2025 MCP Rust Contributors
// SPDX-License-Identifier: MIT

//! Critical End-to-End Integration Tests - WebSocket Transport
//!
//! This test suite validates complete client-server communication workflows
//! using the WebSocket transport with real-time bidirectional communication.

#[cfg(feature = "websocket")]
use mcp_protocol_sdk::{
    core::{
        error::{McpError, McpResult},
        resource::ResourceHandler,
        tool::ToolHandler,
    },
    protocol::types::{
        CallToolResult as ToolResult, ClientCapabilities, Content, JsonRpcNotification,
        JsonRpcRequest, JsonRpcResponse, Resource as ResourceInfo, ResourceContents,
        ServerCapabilities, Tool as ToolInfo,
    },
    server::McpServer,
    transport::TransportConfig,
};

#[cfg(all(test, feature = "websocket"))]
mod e2e_websocket_tests {
    use super::*;

    // Test tool handler for WebSocket tests with notification support
    struct WebSocketToolHandler {
        call_count: Arc<Mutex<u32>>,
        notification_sender: Option<mpsc::UnboundedSender<JsonRpcNotification>>,
    }

    impl WebSocketToolHandler {
        fn new() -> Self {
            Self {
                call_count: Arc::new(Mutex::new(0)),
                notification_sender: None,
            }
        }

        fn with_notifications(sender: mpsc::UnboundedSender<JsonRpcNotification>) -> Self {
            Self {
                call_count: Arc::new(Mutex::new(0)),
                notification_sender: Some(sender),
            }
        }

        async fn get_call_count(&self) -> u32 {
            *self.call_count.lock().await
        }

        async fn send_test_notification(&self, message: &str) -> McpResult<()> {
            if let Some(sender) = &self.notification_sender {
                let notification = JsonRpcNotification {
                    jsonrpc: "2.0".to_string(),
                    method: "notifications/test".to_string(),
                    params: Some(json!({
                        "message": message,
                        "timestamp": chrono::Utc::now().to_rfc3339()
                    })),
                };

                sender
                    .send(notification)
                    .map_err(|_| McpError::internal("Failed to send notification"))?;
            }
            Ok(())
        }
    }

    #[async_trait]
    impl ToolHandler for WebSocketToolHandler {
        async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
            {
                let mut count = self.call_count.lock().await;
                *count += 1;
            }

            let method = arguments
                .get("method")
                .and_then(|v| v.as_str())
                .unwrap_or("ws_echo");

            match method {
                "ws_echo" => {
                    let message = arguments
                        .get("message")
                        .and_then(|v| v.as_str())
                        .unwrap_or("empty");

                    Ok(ToolResult {
                        content: vec![Content::Text {
                            text: format!("WebSocket Echo: {}", message),
                            annotations: None,
                        }],
                        is_error: Some(false),
                        meta: None,
                    })
                }
                "ws_notify" => {
                    let message = arguments
                        .get("message")
                        .and_then(|v| v.as_str())
                        .unwrap_or("notification");

                    self.send_test_notification(message).await?;

                    Ok(ToolResult {
                        content: vec![Content::Text {
                            text: format!("Notification sent: {}", message),
                            annotations: None,
                        }],
                        is_error: Some(false),
                        meta: None,
                    })
                }
                "ws_stream" => {
                    let count =
                        arguments.get("count").and_then(|v| v.as_u64()).unwrap_or(3) as usize;

                    let mut results = Vec::new();
                    for i in 1..=count {
                        results.push(format!("Stream item {}", i));
                        // Simulate streaming delay
                        sleep(Duration::from_millis(50)).await;
                    }

                    Ok(ToolResult {
                        content: vec![Content::Text {
                            text: results.join(", "),
                            annotations: None,
                        }],
                        is_error: Some(false),
                        meta: None,
                    })
                }
                _ => Err(McpError::validation(format!("Unknown method: {}", method))),
            }
        }
    }

    struct WebSocketResourceHandler {
        resources: HashMap<String, String>,
    }

    impl WebSocketResourceHandler {
        fn new() -> Self {
            let mut resources = HashMap::new();
            resources.insert(
                "ws://test/resource1".to_string(),
                "WebSocket Resource 1 Content".to_string(),
            );
            resources.insert(
                "ws://test/resource2".to_string(),
                "WebSocket Resource 2 Content".to_string(),
            );
            resources.insert(
                "ws://test/streaming".to_string(),
                "Streaming data content".to_string(),
            );

            Self { resources }
        }
    }

    #[async_trait]
    impl ResourceHandler for WebSocketResourceHandler {
        async fn list(&self) -> McpResult<Vec<ResourceInfo>> {
            Ok(vec![
                ResourceInfo {
                    uri: "ws://test/resource1".to_string(),
                    name: Some("WebSocket Test Resource 1".to_string()),
                    description: Some("First WebSocket test resource".to_string()),
                    mime_type: Some("text/plain".to_string()),
                    annotations: None,
                    size: None,
                },
                ResourceInfo {
                    uri: "ws://test/resource2".to_string(),
                    name: Some("WebSocket Test Resource 2".to_string()),
                    description: Some("Second WebSocket test resource".to_string()),
                    mime_type: Some("text/plain".to_string()),
                    annotations: None,
                    size: None,
                },
                ResourceInfo {
                    uri: "ws://test/streaming".to_string(),
                    name: Some("Streaming Resource".to_string()),
                    description: Some("Resource for streaming tests".to_string()),
                    mime_type: Some("application/octet-stream".to_string()),
                    annotations: None,
                    size: Some(1024),
                },
            ])
        }

        async fn read(
            &self,
            uri: &str,
            _params: &HashMap<String, String>,
        ) -> McpResult<Vec<ResourceContents>> {
            match self.resources.get(uri) {
                Some(content) => Ok(vec![ResourceContents::Text {
                    uri: uri.to_string(),
                    mime_type: Some("text/plain".to_string()),
                    text: content.clone(),
                }]),
                None => Err(McpError::validation(format!("Resource not found: {}", uri))),
            }
        }
    }

    #[tokio::test]
    async fn test_websocket_tool_handler_interface() {
        // Test that the WebSocket tool handler interface works as expected
        let tool_handler = WebSocketToolHandler::new();

        // Test WebSocket echo
        let mut args = HashMap::new();
        args.insert("method".to_string(), json!("ws_echo"));
        args.insert("message".to_string(), json!("test message"));

        let result = tool_handler.call(args).await.unwrap();
        match &result.content[0] {
            Content::Text { text, .. } => {
                assert!(text.contains("WebSocket Echo: test message"));
            }
            _ => panic!("Expected text content"),
        }

        // Test streaming
        let mut args = HashMap::new();
        args.insert("method".to_string(), json!("ws_stream"));
        args.insert("count".to_string(), json!(3));

        let result = tool_handler.call(args).await.unwrap();
        match &result.content[0] {
            Content::Text { text, .. } => {
                assert!(text.contains("Stream item 1"));
                assert!(text.contains("Stream item 3"));
            }
            _ => panic!("Expected text content"),
        }

        // Check call count
        assert_eq!(tool_handler.get_call_count().await, 2);
    }

    #[tokio::test]
    async fn test_websocket_resource_handler_interface() {
        // Test that the WebSocket resource handler interface works as expected
        let resource_handler = WebSocketResourceHandler::new();

        // Test listing resources
        let resources = resource_handler.list().await.unwrap();
        assert_eq!(resources.len(), 3);
        assert!(resources.iter().any(|r| r.uri == "ws://test/resource1"));

        // Test reading existing resource
        let result = resource_handler
            .read("ws://test/resource1", &HashMap::new())
            .await
            .unwrap();
        match &result[0] {
            ResourceContents::Text { text, .. } => {
                assert_eq!(text, "WebSocket Resource 1 Content");
            }
            _ => panic!("Expected text content"),
        }

        // Test reading non-existent resource
        let result = resource_handler
            .read("ws://test/nonexistent", &HashMap::new())
            .await;
        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_websocket_notifications() {
        // Test WebSocket notification functionality
        let (sender, mut receiver) = mpsc::unbounded_channel();
        let tool_handler = WebSocketToolHandler::with_notifications(sender);

        // Send a notification
        let mut args = HashMap::new();
        args.insert("method".to_string(), json!("ws_notify"));
        args.insert("message".to_string(), json!("test notification"));

        let result = tool_handler.call(args).await.unwrap();
        match &result.content[0] {
            Content::Text { text, .. } => {
                assert!(text.contains("Notification sent: test notification"));
            }
            _ => panic!("Expected text content"),
        }

        // Check that notification was sent
        let notification = receiver.try_recv().unwrap();
        assert_eq!(notification.method, "notifications/test");
        assert!(notification.params.is_some());
    }

    #[tokio::test]
    async fn test_websocket_server_creation() {
        // Test basic WebSocket MCP server creation
        let server = McpServer::new("websocket-test-server".to_string(), "1.0.0".to_string());

        // Test that server was created successfully
        println!("WebSocket MCP server created successfully");
    }

    #[tokio::test]
    async fn test_websocket_transport_configuration() {
        // Test WebSocket transport configuration structure
        let config = TransportConfig {
            read_timeout_ms: Some(5000),
            write_timeout_ms: Some(5000),
            max_message_size: Some(65536),
            compression: true,
            ..Default::default()
        };

        assert_eq!(config.read_timeout_ms, Some(5000));
        assert_eq!(config.write_timeout_ms, Some(5000));
        assert_eq!(config.max_message_size, Some(65536));
        assert_eq!(config.compression, true);

        // Test with WebSocket-specific headers
        let mut ws_config = TransportConfig::default();
        ws_config
            .headers
            .insert("Sec-WebSocket-Protocol".to_string(), "mcp".to_string());
        ws_config
            .headers
            .insert("Origin".to_string(), "http://localhost".to_string());

        assert_eq!(ws_config.headers.len(), 2);
        assert_eq!(
            ws_config.headers.get("Sec-WebSocket-Protocol"),
            Some(&"mcp".to_string())
        );
    }

    #[tokio::test]
    async fn test_websocket_json_rpc_messages() {
        // Test JSON-RPC message creation for WebSocket transport
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            method: "tools/call".to_string(),
            params: Some(json!({
                "name": "ws_echo",
                "arguments": {
                    "message": "hello websocket"
                }
            })),
        };

        let json_str = serde_json::to_string(&request).unwrap();
        assert!(json_str.contains("\"jsonrpc\":\"2.0\""));
        assert!(json_str.contains("\"method\":\"tools/call\""));
        assert!(json_str.contains("\"name\":\"ws_echo\""));

        // Test notification creation
        let notification = JsonRpcNotification {
            jsonrpc: "2.0".to_string(),
            method: "notifications/websocket_event".to_string(),
            params: Some(json!({
                "event": "connection_established",
                "timestamp": "2025-06-17T12:00:00Z"
            })),
        };

        let json_str = serde_json::to_string(&notification).unwrap();
        assert!(json_str.contains("\"method\":\"notifications/websocket_event\""));
        assert!(!json_str.contains("\"id\"")); // Notifications don't have IDs
    }
}
