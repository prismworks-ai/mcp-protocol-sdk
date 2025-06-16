//! Comprehensive unit tests for MCP Protocol SDK Schema (2025-03-26)
//!
//! This test suite covers all aspects of the MCP protocol implementation,
//! focusing on schema validation, type safety, and 2025-03-26 features.

use mcp_protocol_sdk::protocol::{messages::*, types::*, validation::*};
use serde_json::json;
use std::collections::HashMap;

// ============================================================================
// Test Module: Core Protocol Constants and Types
// ============================================================================

#[cfg(test)]
mod protocol_constants {
    use super::*;

    #[test]
    fn test_protocol_version_constants() {
        assert_eq!(LATEST_PROTOCOL_VERSION, "2025-03-26");
        assert_eq!(JSONRPC_VERSION, "2.0");
    }

    #[test]
    fn test_method_constants_complete_coverage() {
        use mcp_protocol_sdk::protocol::messages::methods;

        // Core protocol methods
        assert_eq!(methods::INITIALIZE, "initialize");
        assert_eq!(methods::INITIALIZED, "notifications/initialized");
        assert_eq!(methods::PING, "ping");

        // Tool methods
        assert_eq!(methods::TOOLS_LIST, "tools/list");
        assert_eq!(methods::TOOLS_CALL, "tools/call");
        assert_eq!(
            methods::TOOLS_LIST_CHANGED,
            "notifications/tools/list_changed"
        );

        // Resource methods
        assert_eq!(methods::RESOURCES_LIST, "resources/list");
        assert_eq!(methods::RESOURCES_READ, "resources/read");
        assert_eq!(methods::RESOURCES_SUBSCRIBE, "resources/subscribe");
        assert_eq!(methods::RESOURCES_UNSUBSCRIBE, "resources/unsubscribe");
        assert_eq!(
            methods::RESOURCES_UPDATED,
            "notifications/resources/updated"
        );
        assert_eq!(
            methods::RESOURCES_LIST_CHANGED,
            "notifications/resources/list_changed"
        );

        // New in 2025-03-26: Resource templates
        assert_eq!(
            methods::RESOURCES_TEMPLATES_LIST,
            "resources/templates/list"
        );

        // Prompt methods
        assert_eq!(methods::PROMPTS_LIST, "prompts/list");
        assert_eq!(methods::PROMPTS_GET, "prompts/get");
        assert_eq!(
            methods::PROMPTS_LIST_CHANGED,
            "notifications/prompts/list_changed"
        );

        // Sampling methods
        assert_eq!(methods::SAMPLING_CREATE_MESSAGE, "sampling/createMessage");

        // New in 2025-03-26: Roots management
        assert_eq!(methods::ROOTS_LIST, "roots/list");
        assert_eq!(
            methods::ROOTS_LIST_CHANGED,
            "notifications/roots/list_changed"
        );

        // New in 2025-03-26: Autocompletion
        assert_eq!(methods::COMPLETION_COMPLETE, "completion/complete");

        // Logging methods
        assert_eq!(methods::LOGGING_SET_LEVEL, "logging/setLevel");
        assert_eq!(methods::LOGGING_MESSAGE, "notifications/logging/message");

        // Progress and cancellation
        assert_eq!(methods::PROGRESS, "notifications/progress");
        assert_eq!(methods::CANCELLED, "notifications/cancelled");
    }

    #[test]
    fn test_error_codes() {
        use mcp_protocol_sdk::protocol::types::error_codes;

        // Standard JSON-RPC error codes
        assert_eq!(error_codes::PARSE_ERROR, -32700);
        assert_eq!(error_codes::INVALID_REQUEST, -32600);
        assert_eq!(error_codes::METHOD_NOT_FOUND, -32601);
        assert_eq!(error_codes::INVALID_PARAMS, -32602);
        assert_eq!(error_codes::INTERNAL_ERROR, -32603);

        // MCP-specific error codes
        assert_eq!(error_codes::TOOL_NOT_FOUND, -32000);
        assert_eq!(error_codes::RESOURCE_NOT_FOUND, -32001);
        assert_eq!(error_codes::PROMPT_NOT_FOUND, -32002);
    }
}

// ============================================================================
// Test Module: Content Types (2025-03-26 Enhanced)
// ============================================================================

#[cfg(test)]
mod content_types {
    use super::*;

    #[test]
    fn test_text_content_creation_and_serialization() {
        let content = Content::text("Hello, world!");

        // Test the content without moving it
        let json = serde_json::to_value(&content).unwrap();
        assert_eq!(json["type"], "text");
        assert_eq!(json["text"], "Hello, world!");

        // Now match on the content
        match content {
            Content::Text { text, annotations } => {
                assert_eq!(text, "Hello, world!");
                assert!(annotations.is_none());
            }
            _ => panic!("Expected text content"),
        }
    }

    #[test]
    fn test_image_content_creation_and_validation() {
        let content = Content::image("base64encodeddata", "image/png");

        let json = serde_json::to_value(&content).unwrap();
        assert_eq!(json["type"], "image");
        assert_eq!(json["data"], "base64encodeddata");
        assert_eq!(json["mimeType"], "image/png");

        // Test validation
        assert!(validate_content(&content).is_ok());

        // Test invalid MIME type
        let invalid_content = Content::Image {
            data: "data".to_string(),
            mime_type: "text/plain".to_string(),
            annotations: None,
        };
        assert!(validate_content(&invalid_content).is_err());
    }

    #[test]
    fn test_audio_content_new_in_2025() {
        // Test audio content creation (NEW in 2025-03-26)
        let audio_content = Content::audio("base64audiodata", "audio/wav");

        let json = serde_json::to_value(&audio_content).unwrap();
        assert_eq!(json["type"], "audio");
        assert_eq!(json["data"], "base64audiodata");
        assert_eq!(json["mimeType"], "audio/wav");

        // Test validation
        assert!(validate_content(&audio_content).is_ok());

        // Test various audio MIME types
        let audio_formats = vec![
            "audio/wav",
            "audio/mp3",
            "audio/ogg",
            "audio/aac",
            "audio/flac",
        ];

        for format in audio_formats {
            let content = Content::audio("data", format);
            assert!(
                validate_content(&content).is_ok(),
                "Failed for format: {}",
                format
            );
        }

        // Test invalid MIME type for audio
        let invalid_audio = Content::Audio {
            data: "data".to_string(),
            mime_type: "video/mp4".to_string(),
            annotations: None,
        };
        assert!(validate_content(&invalid_audio).is_err());
    }

    #[test]
    fn test_resource_content_new_in_2025() {
        // Test embedded resource content (NEW in 2025-03-26)
        let resource_content = Content::resource("file:///test.txt");

        let json = serde_json::to_value(&resource_content).unwrap();
        assert_eq!(json["type"], "resource");
        assert_eq!(json["resource"]["uri"], "file:///test.txt");

        // Test validation
        assert!(validate_content(&resource_content).is_ok());

        // Test invalid URI
        let invalid_resource = Content::Resource {
            resource: ResourceReference {
                uri: "".to_string(),
            },
            annotations: None,
        };
        assert!(validate_content(&invalid_resource).is_err());
    }

    #[test]
    fn test_content_with_annotations() {
        let annotations = Annotations::new()
            .for_audience(vec![AnnotationAudience::Developer])
            .with_danger_level(DangerLevel::Safe);

        let content = Content::Text {
            text: "Debug information".to_string(),
            annotations: Some(annotations),
        };

        let json = serde_json::to_value(&content).unwrap();
        assert_eq!(json["type"], "text");
        assert_eq!(json["annotations"]["audience"][0], "developer");
        assert_eq!(json["annotations"]["danger"], "safe");
    }
}

// ============================================================================
// Test Module: Annotations System (2025-03-26 NEW)
// ============================================================================

#[cfg(test)]
mod annotations_system {
    use super::*;

    #[test]
    fn test_annotation_creation_and_builders() {
        let annotations = Annotations::new()
            .read_only()
            .for_audience(vec![
                AnnotationAudience::User,
                AnnotationAudience::Developer,
            ])
            .with_danger_level(DangerLevel::Safe);

        assert_eq!(annotations.read_only, Some(true));
        assert_eq!(annotations.destructive, Some(false));
        assert_eq!(annotations.danger, Some(DangerLevel::Safe));
        assert_eq!(annotations.audience.as_ref().unwrap().len(), 2);
    }

    #[test]
    fn test_annotation_audience_enum() {
        let audiences = vec![
            AnnotationAudience::User,
            AnnotationAudience::Developer,
            AnnotationAudience::Admin,
        ];

        for audience in audiences {
            let json = serde_json::to_value(&audience).unwrap();
            match audience {
                AnnotationAudience::User => assert_eq!(json, "user"),
                AnnotationAudience::Developer => assert_eq!(json, "developer"),
                AnnotationAudience::Admin => assert_eq!(json, "admin"),
            }
        }
    }

    #[test]
    fn test_danger_level_enum() {
        let levels = vec![
            DangerLevel::Safe,
            DangerLevel::Low,
            DangerLevel::Medium,
            DangerLevel::High,
            DangerLevel::Critical,
        ];

        for level in levels {
            let json = serde_json::to_value(&level).unwrap();
            match level {
                DangerLevel::Safe => assert_eq!(json, "safe"),
                DangerLevel::Low => assert_eq!(json, "low"),
                DangerLevel::Medium => assert_eq!(json, "medium"),
                DangerLevel::High => assert_eq!(json, "high"),
                DangerLevel::Critical => assert_eq!(json, "critical"),
            }
        }
    }

    #[test]
    fn test_destructive_tool_annotations() {
        let destructive_annotations = Annotations::new().destructive(DangerLevel::High);

        assert_eq!(destructive_annotations.destructive, Some(true));
        assert_eq!(destructive_annotations.read_only, Some(false));
        assert_eq!(destructive_annotations.danger, Some(DangerLevel::High));
    }

    #[test]
    fn test_annotation_validation() {
        let valid_annotations = Annotations::new()
            .read_only()
            .for_audience(vec![AnnotationAudience::User]);

        assert!(validate_annotations(&valid_annotations).is_ok());
        assert!(validate_tool_annotations(&valid_annotations).is_ok());
    }
}

// ============================================================================
// Test Module: Tool System with Annotations (2025-03-26)
// ============================================================================

#[cfg(test)]
mod tool_system {
    use super::*;

    #[test]
    fn test_tool_creation_with_annotations() {
        let tool = Tool::new("file_reader", "Safely read files").with_annotations(
            Annotations::new()
                .read_only()
                .for_audience(vec![AnnotationAudience::User])
                .with_danger_level(DangerLevel::Safe),
        );

        assert_eq!(tool.name, "file_reader");
        assert!(tool.description.is_some());
        assert!(tool.annotations.is_some());

        let annotations = tool.annotations.unwrap();
        assert_eq!(annotations.read_only, Some(true));
        assert_eq!(annotations.danger, Some(DangerLevel::Safe));
    }

    #[test]
    fn test_tool_validation() {
        let valid_tool = Tool {
            name: "test_tool".to_string(),
            description: Some("A test tool".to_string()),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: None,
                required: None,
                additional_properties: HashMap::new(),
            },
            annotations: Some(Annotations::new().read_only()),
        };

        assert!(validate_tool_info(&valid_tool).is_ok());

        // Test invalid tool (empty name)
        let invalid_tool = Tool {
            name: "".to_string(),
            description: None,
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: None,
                required: None,
                additional_properties: HashMap::new(),
            },
            annotations: None,
        };

        assert!(validate_tool_info(&invalid_tool).is_err());
    }

    #[test]
    fn test_call_tool_result_with_metadata() {
        let content = vec![Content::text("Operation successful")];
        let result = CallToolResult::success(content);

        assert_eq!(result.is_error, Some(false));
        assert!(!result.content.is_empty());

        // Test with metadata
        let mut meta = HashMap::new();
        meta.insert("execution_time_ms".to_string(), json!(150));
        meta.insert("memory_used_mb".to_string(), json!(2.5));

        let result_with_meta =
            CallToolResult::success(vec![Content::text("Done")]).with_metadata(meta);

        assert!(result_with_meta.meta.is_some());
        assert_eq!(
            result_with_meta.meta.unwrap().get("execution_time_ms"),
            Some(&json!(150))
        );
    }
}

// ============================================================================
// Test Module: JSON-RPC Enhanced (2025-03-26)
// ============================================================================

#[cfg(test)]
mod jsonrpc_enhanced {
    use super::*;

    #[test]
    fn test_jsonrpc_request_creation() {
        let request = JsonRpcRequest::new(
            json!(1),
            "test_method".to_string(),
            Some(json!({"param": "value"})),
        )
        .unwrap();

        assert_eq!(request.jsonrpc, "2.0");
        assert_eq!(request.id, json!(1));
        assert_eq!(request.method, "test_method");
        assert!(request.params.is_some());
    }

    #[test]
    fn test_jsonrpc_response_creation() {
        let response = JsonRpcResponse::success(json!(1), json!({"status": "ok"})).unwrap();

        assert_eq!(response.jsonrpc, "2.0");
        assert_eq!(response.id, json!(1));
        assert!(response.result.is_some());
    }

    #[test]
    fn test_jsonrpc_error_creation() {
        let error = JsonRpcError::error(
            json!(1),
            -32601,
            "Method not found".to_string(),
            Some(json!({"method": "unknown_method"})),
        );

        assert_eq!(error.jsonrpc, "2.0");
        assert_eq!(error.id, json!(1));
        assert_eq!(error.error.code, -32601);
        assert_eq!(error.error.message, "Method not found");
        assert!(error.error.data.is_some());
    }

    #[test]
    fn test_jsonrpc_message_validation() {
        // Valid request
        let valid_request = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "test_method",
            "params": {"key": "value"}
        });
        assert!(validate_jsonrpc_message(&valid_request).is_ok());

        // Invalid: wrong version
        let invalid_version = json!({
            "jsonrpc": "1.0",
            "id": 1,
            "method": "test"
        });
        assert!(validate_jsonrpc_message(&invalid_version).is_err());
    }
}

// ============================================================================
// Test Module: New Features in 2025-03-26
// ============================================================================

#[cfg(test)]
mod new_features_2025 {
    use super::*;

    #[test]
    fn test_completion_system() {
        let params = CompleteParams {
            reference: CompletionReference::Tool {
                name: "file_tool".to_string(),
            },
            argument: CompletionArgument {
                name: "file_path".to_string(),
                value: "/home/user/".to_string(),
            },
            meta: None,
        };

        assert!(validate_complete_params(&params).is_ok());

        // Test different reference types
        let resource_ref = CompletionReference::Resource {
            uri: "file:///test.txt".to_string(),
        };
        let prompt_ref = CompletionReference::Prompt {
            name: "test_prompt".to_string(),
        };

        assert!(validate_completion_reference(&resource_ref).is_ok());
        assert!(validate_completion_reference(&prompt_ref).is_ok());
    }

    #[test]
    fn test_enhanced_progress_notifications() {
        let progress = ProgressNotificationParams::with_message(
            json!("op-123"),
            0.75,
            "Processing files... 75% complete".to_string(),
        );

        assert_eq!(progress.progress, 0.75);
        assert_eq!(
            progress.message,
            Some("Processing files... 75% complete".to_string())
        );

        // Test validation
        assert!(validate_progress_params(&progress).is_ok());
    }
}

// ============================================================================
// Test Module: Comprehensive Validation
// ============================================================================

#[cfg(test)]
mod validation_comprehensive {
    use super::*;

    #[test]
    fn test_comprehensive_request_validation() {
        // Test initialize
        let init_params = json!({
            "clientInfo": {"name": "test", "version": "1.0"},
            "capabilities": {},
            "protocolVersion": "2025-03-26"
        });
        assert!(validate_mcp_request("initialize", Some(&init_params)).is_ok());

        // Test tools
        let tool_call = json!({
            "name": "test_tool",
            "arguments": {"input": "test"}
        });
        assert!(validate_mcp_request("tools/call", Some(&tool_call)).is_ok());

        // Test resources
        let resource_read = json!({
            "uri": "file:///test.txt"
        });
        assert!(validate_mcp_request("resources/read", Some(&resource_read)).is_ok());
    }

    #[test]
    fn test_uri_validation_comprehensive() {
        let valid_uris = vec![
            "file:///absolute/path",
            "http://example.com/resource",
            "https://secure.example.com/api/data",
            "/absolute/unix/path",
        ];

        for uri in valid_uris {
            assert!(validate_uri(uri).is_ok(), "URI should be valid: {}", uri);
        }

        let invalid_uris = vec![
            "",
            "not-a-uri",
            "scheme:", // Missing ://
        ];

        for uri in invalid_uris {
            assert!(validate_uri(uri).is_err(), "URI should be invalid: {}", uri);
        }
    }

    #[test]
    fn test_content_validation_edge_cases() {
        // Empty text content
        let empty_text = Content::Text {
            text: "".to_string(),
            annotations: None,
        };
        assert!(validate_content(&empty_text).is_err());

        // Invalid image MIME type
        let invalid_image_mime = Content::Image {
            data: "data".to_string(),
            mime_type: "text/plain".to_string(),
            annotations: None,
        };
        assert!(validate_content(&invalid_image_mime).is_err());

        // Invalid audio MIME type
        let invalid_audio_mime = Content::Audio {
            data: "data".to_string(),
            mime_type: "video/mp4".to_string(),
            annotations: None,
        };
        assert!(validate_content(&invalid_audio_mime).is_err());
    }
}
