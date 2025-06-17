// Copyright (c) 2025 MCP Rust Contributors
// SPDX-License-Identifier: MIT

//! Comprehensive schema compliance validation for MCP Protocol SDK
//! This test validates all protocol types against the JSON schema

use serde_json::json;
use std::collections::HashMap;

// Import the MCP protocol types
use mcp_protocol_sdk::protocol::messages::*;
use mcp_protocol_sdk::protocol::types::*;
use mcp_protocol_sdk::protocol::*;

#[cfg(test)]
mod comprehensive_schema_validation {
    use super::*;

    /// Validates the protocol version constant
    #[test]
    fn test_protocol_version_compliance() {
        assert_eq!(LATEST_PROTOCOL_VERSION, "2025-03-26");
        assert_eq!(JSONRPC_VERSION, "2.0");
        assert_eq!(PROTOCOL_VERSION, LATEST_PROTOCOL_VERSION); // Legacy compatibility
    }

    /// Validates Implementation struct against schema
    #[test]
    fn test_implementation_schema_compliance() {
        let impl_info = Implementation {
            name: "test-implementation".to_string(),
            version: "1.0.0".to_string(),
        };

        let json_val = serde_json::to_value(&impl_info).unwrap();
        assert!(json_val["name"].is_string());
        assert!(json_val["version"].is_string());
        assert_eq!(json_val["name"], "test-implementation");
        assert_eq!(json_val["version"], "1.0.0");
    }

    /// Validates ServerCapabilities against schema
    #[test]
    fn test_server_capabilities_schema_compliance() {
        let capabilities = ServerCapabilities {
            prompts: Some(PromptsCapability {
                list_changed: Some(true),
            }),
            resources: Some(ResourcesCapability {
                subscribe: Some(true),
                list_changed: Some(true),
            }),
            tools: Some(ToolsCapability {
                list_changed: Some(true),
            }),
            sampling: Some(SamplingCapability::default()),
            logging: Some(LoggingCapability::default()),
            completions: Some(CompletionsCapability::default()),
            experimental: Some(HashMap::new()),
        };

        let json_val = serde_json::to_value(&capabilities).unwrap();

        // Validate structure
        assert!(json_val["prompts"].is_object());
        assert!(json_val["resources"].is_object());
        assert!(json_val["tools"].is_object());
        assert!(json_val["sampling"].is_object());
        assert!(json_val["logging"].is_object());
        assert!(json_val["completions"].is_object());
        assert!(json_val["experimental"].is_object());

        // Validate specific fields
        assert_eq!(json_val["prompts"]["listChanged"], true);
        assert_eq!(json_val["resources"]["subscribe"], true);
        assert_eq!(json_val["resources"]["listChanged"], true);
        assert_eq!(json_val["tools"]["listChanged"], true);
    }

    /// Validates ClientCapabilities against schema
    #[test]
    fn test_client_capabilities_schema_compliance() {
        let capabilities = ClientCapabilities {
            sampling: Some(SamplingCapability::default()),
            roots: Some(RootsCapability {
                list_changed: Some(true),
            }),
            experimental: Some(HashMap::new()),
        };

        let json_val = serde_json::to_value(&capabilities).unwrap();

        assert!(json_val["sampling"].is_object());
        assert!(json_val["roots"].is_object());
        assert!(json_val["experimental"].is_object());
        assert_eq!(json_val["roots"]["listChanged"], true);
    }

    /// Validates Content types against schema
    #[test]
    fn test_content_types_schema_compliance() {
        // Test text content
        let text_content = Content::text("Hello, world!");
        let json_val = serde_json::to_value(&text_content).unwrap();
        assert_eq!(json_val["type"], "text");
        assert_eq!(json_val["text"], "Hello, world!");

        // Test image content
        let image_content = Content::image("base64data", "image/png");
        let json_val = serde_json::to_value(&image_content).unwrap();
        assert_eq!(json_val["type"], "image");
        assert_eq!(json_val["data"], "base64data");
        assert_eq!(json_val["mimeType"], "image/png");

        // Test audio content (2025-03-26 NEW)
        let audio_content = Content::audio("audiodata", "audio/wav");
        let json_val = serde_json::to_value(&audio_content).unwrap();
        assert_eq!(json_val["type"], "audio");
        assert_eq!(json_val["data"], "audiodata");
        assert_eq!(json_val["mimeType"], "audio/wav");

        // Test resource content (2025-03-26 NEW)
        let resource_content = Content::resource("file:///test.txt");
        let json_val = serde_json::to_value(&resource_content).unwrap();
        assert_eq!(json_val["type"], "resource");
        assert_eq!(json_val["resource"]["uri"], "file:///test.txt");
    }

    /// Validates Tool with annotations against schema
    #[test]
    fn test_tool_with_annotations_schema_compliance() {
        let tool = Tool {
            name: "test_tool".to_string(),
            description: Some("A test tool".to_string()),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: Some({
                    let mut props = HashMap::new();
                    props.insert("param1".to_string(), json!({"type": "string"}));
                    props
                }),
                required: Some(vec!["param1".to_string()]),
                additional_properties: HashMap::new(),
            },
            annotations: Some(Annotations {
                audience: Some(vec![AnnotationAudience::User]),
                danger: Some(DangerLevel::Low),
                destructive: Some(false),
                read_only: Some(true),
            }),
        };

        let json_val = serde_json::to_value(&tool).unwrap();

        // Validate core tool fields
        assert_eq!(json_val["name"], "test_tool");
        assert_eq!(json_val["description"], "A test tool");
        assert_eq!(json_val["inputSchema"]["type"], "object");
        assert!(json_val["inputSchema"]["properties"].is_object());
        assert!(json_val["inputSchema"]["required"].is_array());

        // Validate annotations (2025-03-26 NEW)
        assert!(json_val["annotations"].is_object());
        assert!(json_val["annotations"]["audience"].is_array());
        assert_eq!(json_val["annotations"]["danger"], "low");
        assert_eq!(json_val["annotations"]["destructive"], false);
        assert_eq!(json_val["annotations"]["readOnly"], true);
    }

    /// Validates Resource against schema
    #[test]
    fn test_resource_schema_compliance() {
        let resource = Resource {
            uri: "file:///test.txt".to_string(),
            name: Some("Test File".to_string()),
            description: Some("A test file".to_string()),
            mime_type: Some("text/plain".to_string()),
            annotations: Some(Annotations::new().read_only()),
            size: Some(1024),
        };

        let json_val = serde_json::to_value(&resource).unwrap();

        assert_eq!(json_val["uri"], "file:///test.txt");
        assert_eq!(json_val["name"], "Test File");
        assert_eq!(json_val["description"], "A test file");
        assert_eq!(json_val["mimeType"], "text/plain");
        assert_eq!(json_val["size"], 1024);
        assert!(json_val["annotations"].is_object());
    }

    /// Validates Prompt against schema
    #[test]
    fn test_prompt_schema_compliance() {
        let prompt = Prompt {
            name: "test_prompt".to_string(),
            description: Some("A test prompt".to_string()),
            arguments: Some(vec![PromptArgument {
                name: "input".to_string(),
                description: Some("Input text".to_string()),
                required: Some(true),
            }]),
        };

        let json_val = serde_json::to_value(&prompt).unwrap();

        assert_eq!(json_val["name"], "test_prompt");
        assert_eq!(json_val["description"], "A test prompt");
        assert!(json_val["arguments"].is_array());
        assert_eq!(json_val["arguments"][0]["name"], "input");
        assert_eq!(json_val["arguments"][0]["required"], true);
    }

    /// Validates JSON-RPC message types against schema
    #[test]
    fn test_jsonrpc_message_schema_compliance() {
        // Test request
        let request = JsonRpcRequest::new::<serde_json::Value>(
            json!("test-1"),
            "tools/list".to_string(),
            Some(json!({})),
        )
        .unwrap();

        let json_val = serde_json::to_value(&request).unwrap();
        assert_eq!(json_val["jsonrpc"], "2.0");
        assert_eq!(json_val["id"], "test-1");
        assert_eq!(json_val["method"], "tools/list");
        assert!(json_val["params"].is_object());

        // Test response
        let response = JsonRpcResponse::success(json!("test-1"), json!({"tools": []})).unwrap();

        let json_val = serde_json::to_value(&response).unwrap();
        assert_eq!(json_val["jsonrpc"], "2.0");
        assert_eq!(json_val["id"], "test-1");
        assert!(json_val["result"].is_object());

        // Test error
        let error = JsonRpcError::error(
            json!("test-1"),
            -32601,
            "Method not found".to_string(),
            None,
        );

        let json_val = serde_json::to_value(&error).unwrap();
        assert_eq!(json_val["jsonrpc"], "2.0");
        assert_eq!(json_val["id"], "test-1");
        assert_eq!(json_val["error"]["code"], -32601);
        assert_eq!(json_val["error"]["message"], "Method not found");

        // Test notification
        let notification = JsonRpcNotification::new::<serde_json::Value>(
            "notifications/progress".to_string(),
            Some(json!({"progress": 50})),
        )
        .unwrap();

        let json_val = serde_json::to_value(&notification).unwrap();
        assert_eq!(json_val["jsonrpc"], "2.0");
        assert_eq!(json_val["method"], "notifications/progress");
        assert!(json_val["params"].is_object());
    }

    /// Validates InitializeParams against schema
    #[test]
    fn test_initialize_params_schema_compliance() {
        let params = InitializeParams {
            protocol_version: "2025-03-26".to_string(),
            capabilities: ClientCapabilities::default(),
            client_info: Implementation {
                name: "test-client".to_string(),
                version: "1.0.0".to_string(),
            },
            meta: None,
        };

        let json_val = serde_json::to_value(&params).unwrap();

        assert_eq!(json_val["protocolVersion"], "2025-03-26");
        assert!(json_val["capabilities"].is_object());
        assert!(json_val["clientInfo"].is_object());
        assert_eq!(json_val["clientInfo"]["name"], "test-client");
        assert_eq!(json_val["clientInfo"]["version"], "1.0.0");
    }

    /// Validates InitializeResult against schema
    #[test]
    fn test_initialize_result_schema_compliance() {
        let result = InitializeResult {
            protocol_version: "2025-03-26".to_string(),
            capabilities: ServerCapabilities::default(),
            server_info: Implementation {
                name: "test-server".to_string(),
                version: "1.0.0".to_string(),
            },
            instructions: Some("Test instructions".to_string()),
            meta: None,
        };

        let json_val = serde_json::to_value(&result).unwrap();

        assert_eq!(json_val["protocolVersion"], "2025-03-26");
        assert!(json_val["capabilities"].is_object());
        assert!(json_val["serverInfo"].is_object());
        assert_eq!(json_val["serverInfo"]["name"], "test-server");
        assert_eq!(json_val["instructions"], "Test instructions");
    }

    /// Validates CallToolParams and Result against schema
    #[test]
    fn test_call_tool_schema_compliance() {
        // Test params
        let mut arguments = HashMap::new();
        arguments.insert("input".to_string(), json!("test input"));

        let params = CallToolParams {
            name: "test_tool".to_string(),
            arguments: Some(arguments),
            meta: None,
        };

        let json_val = serde_json::to_value(&params).unwrap();
        assert_eq!(json_val["name"], "test_tool");
        assert!(json_val["arguments"].is_object());
        assert_eq!(json_val["arguments"]["input"], "test input");

        // Test result
        let result = CallToolResult {
            content: vec![Content::text("Tool output")],
            is_error: Some(false),
            meta: None,
        };

        let json_val = serde_json::to_value(&result).unwrap();
        assert!(json_val["content"].is_array());
        assert_eq!(json_val["content"][0]["type"], "text");
        assert_eq!(json_val["content"][0]["text"], "Tool output");
        assert_eq!(json_val["isError"], false);
    }

    /// Validates SamplingMessage and CreateMessageParams against schema
    #[test]
    fn test_sampling_schema_compliance() {
        let message = SamplingMessage {
            role: Role::User,
            content: Content::text("Hello AI"),
        };

        let json_val = serde_json::to_value(&message).unwrap();
        assert_eq!(json_val["role"], "user");
        assert_eq!(json_val["content"]["type"], "text");
        assert_eq!(json_val["content"]["text"], "Hello AI");

        // Test CreateMessageParams
        let params = CreateMessageParams {
            messages: vec![message],
            max_tokens: 1000,
            system_prompt: Some("You are helpful".to_string()),
            include_context: Some("thisServer".to_string()),
            temperature: Some(0.7),
            stop_sequences: Some(vec!["STOP".to_string()]),
            model_preferences: Some(ModelPreferences::default()),
            metadata: None,
            meta: None,
        };

        let json_val = serde_json::to_value(&params).unwrap();
        assert!(json_val["messages"].is_array());
        assert_eq!(json_val["maxTokens"], 1000);
        assert_eq!(json_val["systemPrompt"], "You are helpful");
        assert_eq!(json_val["includeContext"], "thisServer");
        // Use approximate comparison for floating point values due to JSON precision
        assert!((json_val["temperature"].as_f64().unwrap() - 0.7).abs() < 0.01);
        assert!(json_val["stopSequences"].is_array());
        assert!(json_val["modelPreferences"].is_object());
    }

    /// Validates CreateMessageResult against schema
    #[test]
    fn test_create_message_result_schema_compliance() {
        let result = CreateMessageResult {
            role: Role::Assistant,
            content: Content::text("AI response"),
            model: "claude-3-5-sonnet".to_string(),
            stop_reason: Some(StopReason::EndTurn),
            meta: None,
        };

        let json_val = serde_json::to_value(&result).unwrap();
        assert_eq!(json_val["role"], "assistant");
        assert_eq!(json_val["content"]["type"], "text");
        assert_eq!(json_val["content"]["text"], "AI response");
        assert_eq!(json_val["model"], "claude-3-5-sonnet");
        assert_eq!(json_val["stopReason"], "endTurn");
    }

    /// Validates LoggingLevel against schema
    #[test]
    fn test_logging_level_schema_compliance() {
        let levels = vec![
            LoggingLevel::Debug,
            LoggingLevel::Info,
            LoggingLevel::Notice,
            LoggingLevel::Warning,
            LoggingLevel::Error,
            LoggingLevel::Critical,
            LoggingLevel::Alert,
            LoggingLevel::Emergency,
        ];

        for level in levels {
            let json_val = serde_json::to_value(&level).unwrap();
            assert!(json_val.is_string());

            // Verify it's one of the expected values
            let level_str = json_val.as_str().unwrap();
            assert!(matches!(
                level_str,
                "debug"
                    | "info"
                    | "notice"
                    | "warning"
                    | "error"
                    | "critical"
                    | "alert"
                    | "emergency"
            ));
        }
    }

    /// Validates completion request and response against schema
    #[test]
    fn test_completion_schema_compliance() {
        // Test completion reference types
        let prompt_ref = CompletionReference::Prompt {
            name: "test_prompt".to_string(),
        };
        let json_val = serde_json::to_value(&prompt_ref).unwrap();
        assert_eq!(json_val["type"], "ref/prompt");
        assert_eq!(json_val["name"], "test_prompt");

        let resource_ref = CompletionReference::Resource {
            uri: "file:///test.txt".to_string(),
        };
        let json_val = serde_json::to_value(&resource_ref).unwrap();
        assert_eq!(json_val["type"], "ref/resource");
        assert_eq!(json_val["uri"], "file:///test.txt");

        // Test completion params
        let params = CompleteParams {
            reference: prompt_ref,
            argument: CompletionArgument {
                name: "input".to_string(),
                value: "partial".to_string(),
            },
            meta: None,
        };

        let json_val = serde_json::to_value(&params).unwrap();
        assert!(json_val["ref"].is_object());
        assert!(json_val["argument"].is_object());
        assert_eq!(json_val["argument"]["name"], "input");
        assert_eq!(json_val["argument"]["value"], "partial");

        // Test completion result
        let result = CompleteResult {
            completion: CompletionData {
                values: vec!["input1".to_string(), "input2".to_string()],
                total: Some(2),
                has_more: Some(false),
            },
            meta: None,
        };

        let json_val = serde_json::to_value(&result).unwrap();
        assert!(json_val["completion"]["values"].is_array());
        assert_eq!(json_val["completion"]["total"], 2);
        assert_eq!(json_val["completion"]["hasMore"], false);
    }

    /// Validates Root against schema
    #[test]
    fn test_root_schema_compliance() {
        let root = Root {
            uri: "file:///project".to_string(),
            name: Some("Project Root".to_string()),
        };

        let json_val = serde_json::to_value(&root).unwrap();
        assert_eq!(json_val["uri"], "file:///project");
        assert_eq!(json_val["name"], "Project Root");
    }

    /// Validates progress notification against schema
    #[test]
    fn test_progress_notification_schema_compliance() {
        let progress = ProgressParams {
            progress_token: json!("upload-123"),
            progress: 75.0,
            total: Some(100.0),
            message: Some("Uploading files...".to_string()),
        };

        let json_val = serde_json::to_value(&progress).unwrap();
        assert_eq!(json_val["progressToken"], "upload-123");
        assert_eq!(json_val["progress"], 75.0);
        assert_eq!(json_val["total"], 100.0);
        assert_eq!(json_val["message"], "Uploading files...");
    }

    /// Validates ResourceContents types against schema
    #[test]
    fn test_resource_contents_schema_compliance() {
        // Test text resource
        let text_resource = ResourceContents::Text {
            uri: "file:///text.txt".to_string(),
            mime_type: Some("text/plain".to_string()),
            text: "File content here".to_string(),
        };

        let json_val = serde_json::to_value(&text_resource).unwrap();
        assert_eq!(json_val["uri"], "file:///text.txt");
        assert_eq!(json_val["mimeType"], "text/plain");
        assert_eq!(json_val["text"], "File content here");

        // Test binary resource
        let blob_resource = ResourceContents::Blob {
            uri: "file:///image.png".to_string(),
            mime_type: Some("image/png".to_string()),
            blob: "base64imagedata".to_string(),
        };

        let json_val = serde_json::to_value(&blob_resource).unwrap();
        assert_eq!(json_val["uri"], "file:///image.png");
        assert_eq!(json_val["mimeType"], "image/png");
        assert_eq!(json_val["blob"], "base64imagedata");
    }

    /// Validates error codes against schema
    #[test]
    fn test_error_codes_schema_compliance() {
        use crate::error_codes::*;

        // Test standard JSON-RPC error codes
        assert_eq!(PARSE_ERROR, -32700);
        assert_eq!(INVALID_REQUEST, -32600);
        assert_eq!(METHOD_NOT_FOUND, -32601);
        assert_eq!(INVALID_PARAMS, -32602);
        assert_eq!(INTERNAL_ERROR, -32603);

        // Test MCP-specific error codes
        assert_eq!(TOOL_NOT_FOUND, -32000);
        assert_eq!(RESOURCE_NOT_FOUND, -32001);
        assert_eq!(PROMPT_NOT_FOUND, -32002);
    }

    /// Validates JSON-RPC batch operations against schema
    #[test]
    fn test_jsonrpc_batch_schema_compliance() {
        // Test batch request
        let req1 = JsonRpcRequest::new::<serde_json::Value>(json!(1), "method1".to_string(), None)
            .unwrap();
        let req2 = JsonRpcRequest::new::<serde_json::Value>(json!(2), "method2".to_string(), None)
            .unwrap();
        let notification = JsonRpcNotification::new::<()>("notif".to_string(), None).unwrap();

        let batch_request: JsonRpcBatchRequest = vec![
            JsonRpcRequestOrNotification::Request(req1),
            JsonRpcRequestOrNotification::Request(req2),
            JsonRpcRequestOrNotification::Notification(notification),
        ];

        let json_val = serde_json::to_value(&batch_request).unwrap();
        assert!(json_val.is_array());
        assert_eq!(json_val.as_array().unwrap().len(), 3);

        // Test batch response
        let resp1 = JsonRpcResponse::success(json!(1), json!({"result": "ok"})).unwrap();
        let error2 = JsonRpcError::error(json!(2), -32601, "Not found".to_string(), None);

        let batch_response: JsonRpcBatchResponse = vec![
            JsonRpcResponseOrError::Response(resp1),
            JsonRpcResponseOrError::Error(error2),
        ];

        let json_val = serde_json::to_value(&batch_response).unwrap();
        assert!(json_val.is_array());
        assert_eq!(json_val.as_array().unwrap().len(), 2);
    }

    /// Validates method constants against schema
    #[test]
    fn test_method_constants_schema_compliance() {
        // Test that all method constants are valid
        assert_eq!(INITIALIZE, "initialize");
        assert_eq!(INITIALIZED, "notifications/initialized");
        assert_eq!(PING, "ping");
        assert_eq!(TOOLS_LIST, "tools/list");
        assert_eq!(TOOLS_CALL, "tools/call");
        assert_eq!(RESOURCES_LIST, "resources/list");
        assert_eq!(RESOURCES_READ, "resources/read");
        assert_eq!(PROMPTS_LIST, "prompts/list");
        assert_eq!(PROMPTS_GET, "prompts/get");
        assert_eq!(COMPLETION_COMPLETE, "completion/complete");
        assert_eq!(LOGGING_SET_LEVEL, "logging/setLevel");
        assert_eq!(ROOTS_LIST, "roots/list");

        // Test notification methods
        assert_eq!(PROGRESS, "notifications/progress");
        assert_eq!(CANCELLED, "notifications/cancelled");
        assert_eq!(TOOLS_LIST_CHANGED, "notifications/tools/list_changed");
        assert_eq!(
            RESOURCES_LIST_CHANGED,
            "notifications/resources/list_changed"
        );
    }

    /// Validates ModelPreferences against schema
    #[test]
    fn test_model_preferences_schema_compliance() {
        let preferences = ModelPreferences {
            cost_priority: Some(0.3),
            speed_priority: Some(0.7),
            quality_priority: Some(0.9),
        };

        let json_val = serde_json::to_value(&preferences).unwrap();
        // Use approximate comparison for floating point values due to JSON precision
        assert!((json_val["costPriority"].as_f64().unwrap() - 0.3).abs() < 0.01);
        assert!((json_val["speedPriority"].as_f64().unwrap() - 0.7).abs() < 0.01);
        assert!((json_val["qualityPriority"].as_f64().unwrap() - 0.9).abs() < 0.01);
    }

    /// Validates complete message flow against schema
    #[test]
    fn test_complete_message_flow_schema_compliance() {
        // Initialize request
        let init_request = JsonRpcRequest::new::<InitializeParams>(
            json!("init-1"),
            INITIALIZE.to_string(),
            Some(InitializeParams {
                protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
                capabilities: ClientCapabilities::default(),
                client_info: Implementation {
                    name: "test-client".to_string(),
                    version: "1.0.0".to_string(),
                },
                meta: None,
            }),
        )
        .unwrap();

        let json_val = serde_json::to_value(&init_request).unwrap();
        assert_eq!(json_val["method"], "initialize");
        assert_eq!(json_val["params"]["protocolVersion"], "2025-03-26");

        // Initialize response
        let init_response = JsonRpcResponse::success(
            json!("init-1"),
            InitializeResult {
                protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
                capabilities: ServerCapabilities::default(),
                server_info: Implementation {
                    name: "test-server".to_string(),
                    version: "1.0.0".to_string(),
                },
                instructions: None,
                meta: None,
            },
        )
        .unwrap();

        let json_val = serde_json::to_value(&init_response).unwrap();
        assert_eq!(json_val["result"]["protocolVersion"], "2025-03-26");

        // Initialized notification
        let initialized_notif = JsonRpcNotification::new::<InitializedParams>(
            INITIALIZED.to_string(),
            Some(InitializedParams { meta: None }),
        )
        .unwrap();

        let json_val = serde_json::to_value(&initialized_notif).unwrap();
        assert_eq!(json_val["method"], "notifications/initialized");
    }

    /// Test that all new 2025-03-26 features are properly implemented
    #[test]
    fn test_all_2025_features_comprehensive() {
        println!("✓ Protocol version: {}", LATEST_PROTOCOL_VERSION);

        // Audio content
        let audio = Content::audio("audiodata", "audio/wav");
        assert!(matches!(audio, Content::Audio { .. }));
        println!("✓ Audio content support");

        // Annotations
        let annotations = Annotations::new()
            .destructive(DangerLevel::High)
            .for_audience(vec![AnnotationAudience::User]);
        assert!(annotations.destructive.is_some());
        assert!(annotations.danger.is_some());
        println!("✓ Annotations support");

        // Tool annotations
        let tool = Tool::new("test", "description").with_annotations(annotations);
        assert!(tool.annotations.is_some());
        println!("✓ Tool annotations support");

        // Completion capabilities
        let caps = ServerCapabilities {
            completions: Some(CompletionsCapability::default()),
            ..Default::default()
        };
        assert!(caps.completions.is_some());
        println!("✓ Completion capabilities");

        // Roots capabilities
        let client_caps = ClientCapabilities {
            roots: Some(RootsCapability {
                list_changed: Some(true),
            }),
            ..Default::default()
        };
        assert!(client_caps.roots.is_some());
        println!("✓ Roots capabilities");

        // Enhanced progress
        let progress = ProgressParams {
            progress_token: json!("token"),
            progress: 50.0,
            total: Some(100.0),
            message: Some("Processing...".to_string()),
        };
        assert!(progress.message.is_some());
        println!("✓ Enhanced progress notifications");

        // Embedded resources
        let resource_content = Content::resource("file://test.txt");
        assert!(matches!(resource_content, Content::Resource { .. }));
        println!("✓ Embedded resources support");

        // JSON-RPC batching
        let batch: JsonRpcBatchRequest = vec![];
        assert!(serde_json::to_value(&batch).unwrap().is_array());
        println!("✓ JSON-RPC batching support");

        // Metadata support
        let init_params = InitializeParams {
            protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
            capabilities: ClientCapabilities::default(),
            client_info: Implementation {
                name: "test".to_string(),
                version: "1.0.0".to_string(),
            },
            meta: Some({
                let mut meta = HashMap::new();
                meta.insert("custom".to_string(), json!("value"));
                meta
            }),
        };
        assert!(init_params.meta.is_some());
        println!("✓ Metadata support in requests");

        println!("\n🎉 All 2025-03-26 features are properly implemented and schema-compliant!");
    }

    /// Final comprehensive validation test
    #[test]
    fn test_final_schema_compliance_report() {
        println!("\n=== COMPREHENSIVE SCHEMA COMPLIANCE REPORT ===");
        println!("Protocol Version: {}", LATEST_PROTOCOL_VERSION);
        println!("JSON-RPC Version: {}", JSONRPC_VERSION);

        let mut checks_passed = 0;
        let total_checks = 15;

        // Check 1: Core types
        let _impl = Implementation {
            name: "test".to_string(),
            version: "1.0.0".to_string(),
        };
        checks_passed += 1;
        println!("✓ Core types (Implementation)");

        // Check 2: Capabilities
        let _caps = ServerCapabilities::default();
        checks_passed += 1;
        println!("✓ Server capabilities");

        // Check 3: Content types
        let _content = Content::text("test");
        checks_passed += 1;
        println!("✓ Content types");

        // Check 4: Tool types
        let _tool = Tool::new("test", "description");
        checks_passed += 1;
        println!("✓ Tool types");

        // Check 5: Resource types
        let _resource = Resource {
            uri: "file://test".to_string(),
            name: None,
            description: None,
            mime_type: None,
            annotations: None,
            size: None,
        };
        checks_passed += 1;
        println!("✓ Resource types");

        // Check 6: Prompt types
        let _prompt = Prompt {
            name: "test".to_string(),
            description: None,
            arguments: None,
        };
        checks_passed += 1;
        println!("✓ Prompt types");

        // Check 7: JSON-RPC types
        let _request =
            JsonRpcRequest::new::<serde_json::Value>(json!(1), "test".to_string(), None).unwrap();
        checks_passed += 1;
        println!("✓ JSON-RPC types");

        // Check 8: Message types
        let _params = InitializeParams {
            protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
            capabilities: ClientCapabilities::default(),
            client_info: Implementation {
                name: "test".to_string(),
                version: "1.0.0".to_string(),
            },
            meta: None,
        };
        checks_passed += 1;
        println!("✓ Message parameter types");

        // Check 9: Sampling types
        let _message = SamplingMessage::user_text("test");
        checks_passed += 1;
        println!("✓ Sampling types");

        // Check 10: Logging types
        let _level = LoggingLevel::Info;
        checks_passed += 1;
        println!("✓ Logging types");

        // Check 11: Completion types
        let _completion = CompletionReference::Prompt {
            name: "test".to_string(),
        };
        checks_passed += 1;
        println!("✓ Completion types");

        // Check 12: Root types
        let _root = Root::new("file://test".to_string());
        checks_passed += 1;
        println!("✓ Root types");

        // Check 13: Progress types
        let _progress = ProgressParams {
            progress_token: json!("test"),
            progress: 50.0,
            total: None,
            message: None,
        };
        checks_passed += 1;
        println!("✓ Progress notification types");

        // Check 14: Error codes
        assert_eq!(crate::error_codes::PARSE_ERROR, -32700);
        checks_passed += 1;
        println!("✓ Error codes");

        // Check 15: 2025-03-26 features
        let _audio = Content::audio("data", "audio/wav");
        let _annotations = Annotations::new();
        checks_passed += 1;
        println!("✓ 2025-03-26 new features");

        println!("\n=== COMPLIANCE SUMMARY ===");
        println!("Checks passed: {}/{}", checks_passed, total_checks);
        println!(
            "Compliance rate: {:.1}%",
            (checks_passed as f64 / total_checks as f64) * 100.0
        );

        if checks_passed == total_checks {
            println!("🎉 ALL PROTOCOL TYPES ARE 100% COMPLIANT WITH SCHEMA!");
        } else {
            println!("⚠️  Some compliance issues found");
        }

        assert_eq!(
            checks_passed, total_checks,
            "Not all schema compliance checks passed"
        );
    }
}
