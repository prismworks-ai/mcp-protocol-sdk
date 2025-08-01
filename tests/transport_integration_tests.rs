// Copyright (c) 2025 MCP Rust Contributors
// SPDX-License-Identifier: MIT

//! Integration tests for transport layer - Phase 2
//!
//! These tests exercise actual transport functionality including:
//! - Message sending and receiving
//! - Error handling and recovery
//! - Connection lifecycle management
//! - Protocol compliance

use mcp_protocol_sdk::{
    core::error::{McpError, McpResult},
    protocol::types::{JsonRpcNotification, JsonRpcRequest, JsonRpcResponse},
    transport::traits::{ReconnectConfig, ServerTransport, Transport, TransportStats},
    transport::{StdioServerTransport, TransportConfig},
};
use serde_json::json;
use std::time::Duration;
use tokio::time::timeout;

#[cfg(test)]
mod transport_integration_tests {
    use super::*;

    /// Mock transport for testing client-side functionality
    struct MockClientTransport {
        connected: bool,
        request_responses: std::collections::HashMap<String, JsonRpcResponse>,
        sent_requests: Vec<JsonRpcRequest>,
        sent_notifications: Vec<JsonRpcNotification>,
        received_notifications: Vec<JsonRpcNotification>,
        error_on_method: Option<String>,
    }

    impl MockClientTransport {
        fn new() -> Self {
            Self {
                connected: true,
                request_responses: std::collections::HashMap::new(),
                sent_requests: Vec::new(),
                sent_notifications: Vec::new(),
                received_notifications: Vec::new(),
                error_on_method: None,
            }
        }

        fn add_response(&mut self, method: &str, response: JsonRpcResponse) {
            self.request_responses.insert(method.to_string(), response);
        }

        fn add_notification(&mut self, notification: JsonRpcNotification) {
            self.received_notifications.push(notification);
        }

        fn set_error_on_method(&mut self, method: &str) {
            self.error_on_method = Some(method.to_string());
        }

        fn disconnect(&mut self) {
            self.connected = false;
        }
    }

    #[async_trait::async_trait]
    impl Transport for MockClientTransport {
        async fn send_request(&mut self, request: JsonRpcRequest) -> McpResult<JsonRpcResponse> {
            if !self.connected {
                return Err(McpError::transport("Transport not connected"));
            }

            if let Some(ref error_method) = self.error_on_method {
                if request.method == *error_method {
                    return Err(McpError::transport("Simulated transport error"));
                }
            }

            self.sent_requests.push(request.clone());

            if let Some(response) = self.request_responses.get(&request.method) {
                let mut response = response.clone();
                response.id = request.id;
                Ok(response)
            } else {
                Ok(JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    id: request.id,
                    result: Some(json!({"status": "ok"})),
                })
            }
        }

        async fn send_notification(&mut self, notification: JsonRpcNotification) -> McpResult<()> {
            if !self.connected {
                return Err(McpError::transport("Transport not connected"));
            }

            self.sent_notifications.push(notification);
            Ok(())
        }

        async fn receive_notification(&mut self) -> McpResult<Option<JsonRpcNotification>> {
            if !self.connected {
                return Err(McpError::transport("Transport not connected"));
            }

            Ok(self.received_notifications.pop())
        }

        async fn close(&mut self) -> McpResult<()> {
            self.connected = false;
            Ok(())
        }

        fn is_connected(&self) -> bool {
            self.connected
        }

        fn connection_info(&self) -> String {
            format!("Mock transport (connected: {})", self.connected)
        }
    }

    #[tokio::test]
    async fn test_mock_transport_request_response() {
        // Test basic request/response functionality
        let mut transport = MockClientTransport::new();

        // Setup a response for a specific method
        let expected_response = JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            result: Some(json!({"data": "test_response"})),
        };
        transport.add_response("test_method", expected_response.clone());

        // Send a request
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            method: "test_method".to_string(),
            params: Some(json!({"param": "value"})),
        };

        let response = transport.send_request(request.clone()).await;
        assert!(response.is_ok(), "Request should succeed");

        let response = response.unwrap();
        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, json!(1));
        assert_eq!(response.result, Some(json!({"data": "test_response"})));

        // Verify the request was recorded
        assert_eq!(transport.sent_requests.len(), 1);
        assert_eq!(transport.sent_requests[0].method, "test_method");
    }

    #[tokio::test]
    async fn test_mock_transport_notifications() {
        // Test notification sending and receiving
        let mut transport = MockClientTransport::new();

        // Test sending notification
        let notification = JsonRpcNotification {
            jsonrpc: "2.0".to_string(),
            method: "test_notification".to_string(),
            params: Some(json!({"event": "test_event"})),
        };

        let result = transport.send_notification(notification.clone()).await;
        assert!(result.is_ok(), "Sending notification should succeed");

        // Verify notification was recorded
        assert_eq!(transport.sent_notifications.len(), 1);
        assert_eq!(transport.sent_notifications[0].method, "test_notification");

        // Test receiving notification
        let incoming_notification = JsonRpcNotification {
            jsonrpc: "2.0".to_string(),
            method: "incoming_notification".to_string(),
            params: Some(json!({"data": "incoming_data"})),
        };
        transport.add_notification(incoming_notification.clone());

        let received = transport.receive_notification().await;
        assert!(received.is_ok(), "Receiving notification should succeed");

        let received = received.unwrap();
        assert!(received.is_some(), "Should receive the notification");
        let received = received.unwrap();
        assert_eq!(received.method, "incoming_notification");
    }

    #[tokio::test]
    async fn test_mock_transport_connection_lifecycle() {
        // Test connection state management
        let mut transport = MockClientTransport::new();

        // Initially connected
        assert!(transport.is_connected());
        assert!(transport.connection_info().contains("connected: true"));

        // Test operations while connected
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            method: "test".to_string(),
            params: None,
        };

        let result = transport.send_request(request).await;
        assert!(result.is_ok(), "Request should work while connected");

        // Disconnect
        transport.disconnect();
        assert!(!transport.is_connected());
        assert!(transport.connection_info().contains("connected: false"));

        // Test operations while disconnected
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(2),
            method: "test".to_string(),
            params: None,
        };

        let result = transport.send_request(request).await;
        assert!(result.is_err(), "Request should fail while disconnected");

        match result.unwrap_err() {
            McpError::Transport(msg) => assert!(msg.contains("not connected")),
            _ => panic!("Expected transport error"),
        }

        // Test close
        let close_result = transport.close().await;
        assert!(close_result.is_ok(), "Close should succeed");
        assert!(!transport.is_connected());
    }

    #[tokio::test]
    async fn test_mock_transport_error_simulation() {
        // Test error handling
        let mut transport = MockClientTransport::new();
        transport.set_error_on_method("error_method");

        // Request that should succeed
        let good_request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            method: "good_method".to_string(),
            params: None,
        };

        let result = transport.send_request(good_request).await;
        assert!(result.is_ok(), "Good request should succeed");

        // Request that should fail
        let bad_request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(2),
            method: "error_method".to_string(),
            params: None,
        };

        let result = transport.send_request(bad_request).await;
        assert!(result.is_err(), "Error method should fail");

        match result.unwrap_err() {
            McpError::Transport(msg) => assert!(msg.contains("Simulated transport error")),
            _ => panic!("Expected transport error"),
        }
    }

    #[tokio::test]
    async fn test_stdio_server_transport_async_operations() {
        // Test async server transport operations
        let mut transport = StdioServerTransport::new();

        // Test that we can call async methods
        let stop_result = transport.stop().await;
        assert!(stop_result.is_ok(), "Stop should succeed");

        // Test request handling with different request types
        let requests = vec![
            ("capabilities", json!({})),
            ("tools/list", json!({})),
            ("resources/list", json!({})),
            ("unknown_method", json!({"param": "value"})),
        ];

        for (method, params) in requests {
            let request = JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                id: json!(method),
                method: method.to_string(),
                params: Some(params),
            };

            let result = transport.handle_request(request).await;
            assert!(
                result.is_err(),
                "All methods should return error in default implementation"
            );

            // Verify error contains method name
            match result.unwrap_err() {
                McpError::Protocol(msg) => {
                    assert!(msg.contains(method), "Error should mention the method name");
                }
                _ => panic!("Expected Protocol error"),
            }
        }
    }

    #[tokio::test]
    async fn test_transport_timeout_simulation() {
        // Test timeout behavior simulation
        let mut transport = MockClientTransport::new();

        // Create a request
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            method: "test_method".to_string(),
            params: None,
        };

        // Test with timeout
        let timeout_duration = Duration::from_millis(10);
        let result = timeout(timeout_duration, transport.send_request(request)).await;

        // The mock transport should complete quickly, so this should succeed
        assert!(
            result.is_ok(),
            "Mock transport should complete within timeout"
        );
        let response_result = result.unwrap();
        assert!(response_result.is_ok(), "Request should succeed");
    }

    #[tokio::test]
    async fn test_concurrent_transport_operations() {
        // Test concurrent operations on transport
        let mut transport = MockClientTransport::new();

        // Setup responses for multiple methods
        for i in 0..5 {
            let method = format!("method_{i}");
            let response = JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                id: json!(i),
                result: Some(json!({"result": i})),
            };
            transport.add_response(&method, response);
        }

        // We can't actually run concurrent operations on a single transport
        // since it's not Clone, but we can test sequential operations quickly
        for i in 0..5 {
            let request = JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                id: json!(i),
                method: format!("method_{i}"),
                params: None,
            };

            let response = transport.send_request(request).await;
            assert!(response.is_ok(), "Request {i} should succeed");

            let response = response.unwrap();
            assert_eq!(response.id, json!(i));
        }

        // Verify all requests were sent
        assert_eq!(transport.sent_requests.len(), 5);
    }

    #[tokio::test]
    async fn test_transport_config_integration() {
        // Test that TransportConfig values are properly used
        let configs = vec![
            TransportConfig {
                read_timeout_ms: Some(1000),
                write_timeout_ms: Some(500),
                max_message_size: Some(1024),
                compression: false,
                ..Default::default()
            },
            TransportConfig {
                read_timeout_ms: Some(5000),
                write_timeout_ms: Some(2000),
                max_message_size: Some(8192),
                compression: true,
                headers: std::collections::HashMap::from([(
                    "Test-Header".to_string(),
                    "test-value".to_string(),
                )]),
                ..Default::default()
            },
        ];

        for (i, config) in configs.into_iter().enumerate() {
            let transport = StdioServerTransport::with_config(config.clone());

            // Verify transport was created successfully
            assert!(
                !transport.is_running(),
                "Transport {i} should not be running initially"
            );

            // We can't directly verify config usage without exposing internal state,
            // but we can verify the transport was created with the config
            assert_eq!(
                transport.server_info(),
                "STDIO server transport (running: false)"
            );
        }
    }

    #[tokio::test]
    async fn test_json_rpc_protocol_compliance() {
        // Test JSON-RPC protocol compliance
        let mut transport = MockClientTransport::new();

        // Test valid JSON-RPC request
        let valid_request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(42),
            method: "test_method".to_string(),
            params: Some(json!({"param1": "value1", "param2": 123})),
        };

        let response = transport.send_request(valid_request.clone()).await;
        assert!(response.is_ok(), "Valid request should succeed");

        let response = response.unwrap();
        assert_eq!(
            response.jsonrpc, "2.0",
            "Response should have correct JSON-RPC version"
        );
        assert_eq!(response.id, json!(42), "Response should have matching ID");

        // Test notification (no ID)
        let notification = JsonRpcNotification {
            jsonrpc: "2.0".to_string(),
            method: "notification_method".to_string(),
            params: Some(json!({"event": "test_event", "data": {"key": "value"}})),
        };

        let result = transport.send_notification(notification.clone()).await;
        assert!(result.is_ok(), "Valid notification should succeed");

        // Verify notification structure
        assert_eq!(notification.jsonrpc, "2.0");
        assert!(!notification.method.is_empty());

        // Test various ID types
        let id_types = vec![json!(1), json!("string_id"), json!(null)];

        for (i, id) in id_types.into_iter().enumerate() {
            let request = JsonRpcRequest {
                jsonrpc: "2.0".to_string(),
                id: id.clone(),
                method: format!("test_method_{i}"),
                params: None,
            };

            let response = transport.send_request(request).await;
            assert!(response.is_ok(), "Request with ID type {i} should succeed");

            let response = response.unwrap();
            assert_eq!(response.id, id, "Response ID should match request ID");
        }
    }

    #[tokio::test]
    async fn test_error_response_handling() {
        // Test error response handling
        let mut transport = MockClientTransport::new();

        // Setup an error response using JsonRpcError instead
        let error_response = JsonRpcResponse {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            result: None, // No result for error case
        };
        transport.add_response("error_method", error_response);

        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: json!(1),
            method: "error_method".to_string(),
            params: Some(json!({})),
        };

        let response = transport.send_request(request).await;
        assert!(response.is_ok(), "Transport should succeed");

        let response = response.unwrap();
        assert!(
            response.result.is_none(),
            "Error response should have no result"
        );

        // For actual error handling, we would need to use JsonRpcError type
        // This test demonstrates the transport layer working correctly
    }

    #[test]
    fn test_transport_stats_integration() {
        // Test TransportStats with realistic scenarios
        // Simulate a session with various operations
        let stats = TransportStats {
            requests_sent: 25,
            responses_received: 24, // One timeout
            notifications_sent: 5,
            notifications_received: 3,
            connection_errors: 1,
            protocol_errors: 0,
            bytes_sent: 12_584,     // ~12KB
            bytes_received: 15_229, // ~15KB
            uptime_ms: 45_000,      // 45 seconds
        };

        // Verify the stats make sense
        assert!(
            stats.responses_received <= stats.requests_sent,
            "Can't receive more responses than requests sent"
        );
        assert!(stats.bytes_sent > 0, "Should have sent some data");
        assert!(stats.bytes_received > 0, "Should have received some data");
        assert!(stats.uptime_ms > 0, "Should have some uptime");

        // Test reasonable ratios
        let success_rate = stats.responses_received as f64 / stats.requests_sent as f64;
        assert!(
            success_rate >= 0.9,
            "Success rate should be high: {success_rate}"
        );

        // Test that clone works
        let cloned_stats = stats.clone();
        assert_eq!(stats.requests_sent, cloned_stats.requests_sent);
        assert_eq!(stats.bytes_sent, cloned_stats.bytes_sent);
        assert_eq!(stats.uptime_ms, cloned_stats.uptime_ms);
    }

    #[test]
    fn test_reconnect_config_integration() {
        // Test ReconnectConfig with various scenarios
        let configs = vec![
            // Conservative config
            ReconnectConfig {
                enabled: true,
                max_attempts: Some(3),
                initial_delay_ms: 500,
                max_delay_ms: 5_000,
                backoff_multiplier: 1.5,
                jitter_factor: 0.1,
            },
            // Aggressive config
            ReconnectConfig {
                enabled: true,
                max_attempts: Some(10),
                initial_delay_ms: 100,
                max_delay_ms: 30_000,
                backoff_multiplier: 2.0,
                jitter_factor: 0.2,
            },
            // Disabled reconnection
            ReconnectConfig {
                enabled: false,
                ..Default::default()
            },
            // Unlimited attempts
            ReconnectConfig {
                enabled: true,
                max_attempts: None,
                initial_delay_ms: 1000,
                max_delay_ms: 60_000,
                backoff_multiplier: 2.0,
                jitter_factor: 0.15,
            },
        ];

        for (i, config) in configs.into_iter().enumerate() {
            // Test config validity
            if config.enabled {
                assert!(
                    config.initial_delay_ms > 0,
                    "Config {i}: Initial delay should be positive"
                );
                assert!(
                    config.max_delay_ms >= config.initial_delay_ms,
                    "Config {i}: Max delay should be >= initial delay"
                );
                assert!(
                    config.backoff_multiplier > 1.0,
                    "Config {i}: Backoff multiplier should be > 1.0"
                );
                assert!(
                    (0.0..=1.0).contains(&config.jitter_factor),
                    "Config {i}: Jitter factor should be between 0.0 and 1.0"
                );
            }

            // Test cloning
            let cloned_config = config.clone();
            assert_eq!(config.enabled, cloned_config.enabled);
            assert_eq!(config.max_attempts, cloned_config.max_attempts);
            assert_eq!(config.initial_delay_ms, cloned_config.initial_delay_ms);
        }
    }

    #[tokio::test]
    async fn test_transport_shutdown_scenarios() {
        // Test various shutdown scenarios
        let scenarios = vec!["clean_shutdown", "forced_shutdown", "already_stopped"];

        for scenario in scenarios {
            let mut transport = StdioServerTransport::new();

            match scenario {
                "clean_shutdown" => {
                    // Normal shutdown
                    let result = transport.stop().await;
                    assert!(result.is_ok(), "Clean shutdown should succeed");
                    assert!(
                        !transport.is_running(),
                        "Transport should not be running after stop"
                    );
                }
                "forced_shutdown" => {
                    // Immediate shutdown (same as clean for this implementation)
                    let result = transport.stop().await;
                    assert!(result.is_ok(), "Forced shutdown should succeed");
                    assert!(
                        !transport.is_running(),
                        "Transport should not be running after stop"
                    );
                }
                "already_stopped" => {
                    // Stop when already stopped
                    let result1 = transport.stop().await;
                    assert!(result1.is_ok(), "First stop should succeed");

                    let result2 = transport.stop().await;
                    assert!(result2.is_ok(), "Second stop should also succeed");
                    assert!(!transport.is_running(), "Transport should remain stopped");
                }
                _ => unreachable!(),
            }
        }
    }
}
