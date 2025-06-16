//! Working integration tests for MCP Protocol SDK
//!
//! These tests demonstrate full end-to-end functionality using the current API.

use std::collections::HashMap;
use serde_json::{json, Value};

use mcp_protocol_sdk::{
    core::{
        error::{McpError, McpResult},
        tool::{ToolHandler, EchoTool, AdditionTool},
        resource::ResourceHandler,
        prompt::PromptHandler,
    },
    server::mcp_server::{McpServer, ServerConfig},
    protocol::{
        types::*,
        messages::*,
    },
};

// ========================================================================
// Test Tool Handlers
// ========================================================================

/// Calculator tool for basic arithmetic operations
struct CalculatorTool;

#[async_trait::async_trait]
impl ToolHandler for CalculatorTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        let operation = arguments
            .get("operation")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::validation("Missing 'operation' parameter"))?;

        let a = arguments
            .get("a")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| McpError::validation("Missing or invalid 'a' parameter"))?;

        let b = arguments
            .get("b")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| McpError::validation("Missing or invalid 'b' parameter"))?;

        let result = match operation {
            "add" => a + b,
            "subtract" => a - b,
            "multiply" => a * b,
            "divide" => {
                if b == 0.0 {
                    return Err(McpError::validation("Division by zero"));
                }
                a / b
            }
            _ => return Err(McpError::validation("Invalid operation")),
        };

        Ok(ToolResult {
            content: vec![Content::Text {
                text: json!({"result": result}).to_string(),
                annotations: None,
            }],
            is_error: Some(false),
            meta: None,
        })
    }
}

/// Text processing tool
struct TextProcessorTool;

#[async_trait::async_trait]
impl ToolHandler for TextProcessorTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        let text = arguments
            .get("text")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let operation = arguments
            .get("operation")
            .and_then(|v| v.as_str())
            .unwrap_or("count");

        let result = match operation {
            "count" => json!({"character_count": text.len()}),
            "words" => json!({"word_count": text.split_whitespace().count()}),
            "lines" => json!({"line_count": text.lines().count()}),
            "uppercase" => json!({"result": text.to_uppercase()}),
            "lowercase" => json!({"result": text.to_lowercase()}),
            "reverse" => json!({"result": text.chars().rev().collect::<String>()}),
            _ => return Err(McpError::validation("Invalid text operation")),
        };

        Ok(ToolResult {
            content: vec![Content::Text {
                text: result.to_string(),
                annotations: None,
            }],
            is_error: Some(false),
            meta: Some({
                let mut meta = HashMap::new();
                meta.insert("operation".to_string(), json!(operation));
                meta.insert("input_length".to_string(), json!(text.len()));
                meta
            }),
        })
    }
}

// ========================================================================
// Test Resource Handlers
// ========================================================================

/// Sample data resource handler
struct SampleDataResource;

#[async_trait::async_trait]
impl ResourceHandler for SampleDataResource {
    async fn read(&self, uri: &str, _params: &HashMap<String, String>) -> McpResult<Vec<ResourceContents>> {
        Ok(vec![ResourceContents::Text {
            uri: uri.to_string(),
            mime_type: Some("application/json".to_string()),
            text: json!({
                "name": "Sample Dataset",
                "records": [
                    {"id": 1, "name": "Alice", "age": 30},
                    {"id": 2, "name": "Bob", "age": 25},
                    {"id": 3, "name": "Charlie", "age": 35}
                ],
                "metadata": {
                    "created": "2024-01-01",
                    "updated": "2024-06-16"
                }
            }).to_string(),
        }])
    }

    async fn list(&self) -> McpResult<Vec<ResourceInfo>> {
        Ok(vec![ResourceInfo {
            uri: "memory://sample-data".to_string(),
            name: Some("Sample Data".to_string()),
            description: Some("Sample dataset for testing".to_string()),
            mime_type: Some("application/json".to_string()),
            annotations: None,
            size: Some(1024),
        }])
    }
}

/// User profile resource handler
struct UserProfileResource {
    user_id: String,
}

impl UserProfileResource {
    fn new(user_id: String) -> Self {
        Self { user_id }
    }
}

#[async_trait::async_trait]
impl ResourceHandler for UserProfileResource {
    async fn read(&self, uri: &str, _params: &HashMap<String, String>) -> McpResult<Vec<ResourceContents>> {
        // Extract user ID from URI (simple parsing for demo)
        let user_id = uri.split('/').last().unwrap_or(&self.user_id);
        
        Ok(vec![ResourceContents::Text {
            uri: uri.to_string(),
            mime_type: Some("application/json".to_string()),
            text: json!({
                "user_id": user_id,
                "name": format!("User {}", user_id),
                "email": format!("user{}@example.com", user_id),
                "created_at": "2024-01-01T00:00:00Z",
                "last_active": "2024-06-16T12:00:00Z",
                "preferences": {
                    "theme": "dark",
                    "notifications": true
                }
            }).to_string(),
        }])
    }

    async fn list(&self) -> McpResult<Vec<ResourceInfo>> {
        Ok(vec![ResourceInfo {
            uri: format!("memory://users/{}", self.user_id),
            name: Some(format!("User {}", self.user_id)),
            description: Some("User profile data".to_string()),
            mime_type: Some("application/json".to_string()),
            annotations: None,
            size: Some(512),
        }])
    }
}

// ========================================================================
// Test Prompt Handlers
// ========================================================================

/// Code review prompt handler
struct CodeReviewPrompt;

#[async_trait::async_trait]
impl PromptHandler for CodeReviewPrompt {
    async fn get(&self, arguments: HashMap<String, Value>) -> McpResult<PromptResult> {
        let language = arguments
            .get("language")
            .and_then(|v| v.as_str())
            .unwrap_or("generic");

        let focus = arguments
            .get("focus")
            .and_then(|v| v.as_str())
            .unwrap_or("general");

        let messages = vec![
            PromptMessage {
                role: Role::User,
                content: Content::Text {
                    text: format!(
                        "Please review this {} code with a focus on {}.",
                        language, focus
                    ),
                    annotations: None,
                },
            },
            PromptMessage {
                role: Role::Assistant,
                content: Content::Text {
                    text: format!(
                        "I'll review your {} code focusing on {}. Please provide the code you'd like me to examine.",
                        language, focus
                    ),
                    annotations: None,
                },
            },
        ];

        Ok(PromptResult {
            description: Some(format!("Code review assistant for {} with {} focus", language, focus)),
            messages,
            meta: Some({
                let mut meta = HashMap::new();
                meta.insert("language".to_string(), json!(language));
                meta.insert("focus".to_string(), json!(focus));
                meta.insert("template_version".to_string(), json!("1.0"));
                meta
            }),
        })
    }
}

/// Documentation prompt handler
struct DocumentationPrompt;

#[async_trait::async_trait]
impl PromptHandler for DocumentationPrompt {
    async fn get(&self, arguments: HashMap<String, Value>) -> McpResult<PromptResult> {
        let doc_type = arguments
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("api");

        let audience = arguments
            .get("audience")
            .and_then(|v| v.as_str())
            .unwrap_or("developers");

        let messages = vec![
            PromptMessage {
                role: Role::User,
                content: Content::Text {
                    text: format!(
                        "Help me create {} documentation suitable for {}.",
                        doc_type, audience
                    ),
                    annotations: None,
                },
            },
            PromptMessage {
                role: Role::Assistant,
                content: Content::Text {
                    text: format!(
                        "I'll help you create clear, comprehensive {} documentation for {}. What specific aspect would you like to document?",
                        doc_type, audience
                    ),
                    annotations: None,
                },
            },
        ];

        Ok(PromptResult {
            description: Some(format!("Documentation assistant for {} targeting {}", doc_type, audience)),
            messages,
            meta: Some({
                let mut meta = HashMap::new();
                meta.insert("type".to_string(), json!(doc_type));
                meta.insert("audience".to_string(), json!(audience));
                meta.insert("sections".to_string(), json!(["overview", "examples", "reference"]));
                meta
            }),
        })
    }
}

// ========================================================================
// Integration Tests
// ========================================================================

#[tokio::test]
async fn test_full_server_integration() {
    // Create server with custom config
    let config = ServerConfig {
        max_concurrent_requests: 50,
        request_timeout_ms: 15000,
        validate_requests: true,
        enable_logging: true,
    };

    let server = McpServer::with_config(
        "test-integration-server".to_string(),
        "0.1.0".to_string(),
        config,
    );

    // Verify server info
    assert_eq!(server.info().name, "test-integration-server");
    assert_eq!(server.info().version, "0.1.0");
    assert!(!server.is_running().await);

    // Test tools
    let calculator_schema = json!({
        "type": "object",
        "properties": {
            "operation": {
                "type": "string",
                "enum": ["add", "subtract", "multiply", "divide"]
            },
            "a": {"type": "number"},
            "b": {"type": "number"}
        },
        "required": ["operation", "a", "b"]
    });

    server
        .add_tool(
            "calculator".to_string(),
            Some("Basic calculator operations".to_string()),
            calculator_schema,
            CalculatorTool,
        )
        .await
        .unwrap();

    let text_schema = json!({
        "type": "object",
        "properties": {
            "text": {"type": "string"},
            "operation": {
                "type": "string",
                "enum": ["count", "words", "lines", "uppercase", "lowercase", "reverse"]
            }
        },
        "required": ["text"]
    });

    server
        .add_tool(
            "text_processor".to_string(),
            Some("Text processing operations".to_string()),
            text_schema,
            TextProcessorTool,
        )
        .await
        .unwrap();

    // Test resources
    let sample_data_info = ResourceInfo {
        uri: "memory://sample-data".to_string(),
        name: Some("Sample Data".to_string()),
        description: Some("Sample dataset for testing".to_string()),
        mime_type: Some("application/json".to_string()),
        annotations: None,
        size: Some(1024),
    };

    server
        .add_resource_detailed(sample_data_info, SampleDataResource)
        .await
        .unwrap();

    server
        .add_resource(
            "user-profile-123".to_string(),
            "memory://users/123".to_string(),
            UserProfileResource::new("123".to_string()),
        )
        .await
        .unwrap();

    // Test prompts - Note: PromptInfo does not have annotations field
    let code_review_info = PromptInfo {
        name: "code_review".to_string(),
        description: Some("Code review assistant".to_string()),
        arguments: Some(vec![
            PromptArgument {
                name: "language".to_string(),
                description: Some("Programming language".to_string()),
                required: Some(false),
            },
            PromptArgument {
                name: "focus".to_string(),
                description: Some("Review focus area".to_string()),
                required: Some(false),
            },
        ]),
    };

    server
        .add_prompt(code_review_info, CodeReviewPrompt)
        .await
        .unwrap();

    let doc_info = PromptInfo {
        name: "documentation".to_string(),
        description: Some("Documentation assistant".to_string()),
        arguments: Some(vec![
            PromptArgument {
                name: "type".to_string(),
                description: Some("Documentation type".to_string()),
                required: Some(false),
            },
            PromptArgument {
                name: "audience".to_string(),
                description: Some("Target audience".to_string()),
                required: Some(false),
            },
        ]),
    };

    server
        .add_prompt(doc_info, DocumentationPrompt)
        .await
        .unwrap();

    // Test listing capabilities
    let tools = server.list_tools().await.unwrap();
    assert_eq!(tools.len(), 2);
    assert!(tools.iter().any(|t| t.name == "calculator"));
    assert!(tools.iter().any(|t| t.name == "text_processor"));

    let resources = server.list_resources().await.unwrap();
    assert_eq!(resources.len(), 2);
    assert!(resources.iter().any(|r| r.uri == "memory://sample-data"));
    assert!(resources.iter().any(|r| r.uri == "memory://users/123"));

    let prompts = server.list_prompts().await.unwrap();
    assert_eq!(prompts.len(), 2);
    assert!(prompts.iter().any(|p| p.name == "code_review"));
    assert!(prompts.iter().any(|p| p.name == "documentation"));
}

#[tokio::test]
async fn test_tool_operations() {
    let server = McpServer::new("tool-test-server".to_string(), "1.0.0".to_string());

    let schema = json!({
        "type": "object",
        "properties": {
            "operation": {"type": "string"},
            "a": {"type": "number"},
            "b": {"type": "number"}
        },
        "required": ["operation", "a", "b"]
    });

    server
        .add_tool(
            "calculator".to_string(),
            Some("Calculator tool".to_string()),
            schema,
            CalculatorTool,
        )
        .await
        .unwrap();

    // Test successful operations
    let mut args = HashMap::new();
    args.insert("operation".to_string(), json!("add"));
    args.insert("a".to_string(), json!(15.0));
    args.insert("b".to_string(), json!(25.0));

    let result = server.call_tool("calculator", Some(args)).await.unwrap();
    assert_eq!(result.is_error, Some(false));
    assert_eq!(result.content.len(), 1);

    // Test division
    let mut div_args = HashMap::new();
    div_args.insert("operation".to_string(), json!("divide"));
    div_args.insert("a".to_string(), json!(100.0));
    div_args.insert("b".to_string(), json!(4.0));

    let div_result = server.call_tool("calculator", Some(div_args)).await.unwrap();
    assert_eq!(div_result.is_error, Some(false));

    // Test division by zero
    let mut zero_args = HashMap::new();
    zero_args.insert("operation".to_string(), json!("divide"));
    zero_args.insert("a".to_string(), json!(100.0));
    zero_args.insert("b".to_string(), json!(0.0));

    let zero_result = server.call_tool("calculator", Some(zero_args)).await;
    assert!(zero_result.is_err());

    // Test invalid operation
    let mut invalid_args = HashMap::new();
    invalid_args.insert("operation".to_string(), json!("power"));
    invalid_args.insert("a".to_string(), json!(2.0));
    invalid_args.insert("b".to_string(), json!(3.0));

    let invalid_result = server.call_tool("calculator", Some(invalid_args)).await;
    assert!(invalid_result.is_err());

    // Test tool not found
    let not_found_result = server.call_tool("nonexistent", None).await;
    assert!(not_found_result.is_err());
}

#[tokio::test]
async fn test_text_processing_tool() {
    let server = McpServer::new("text-server".to_string(), "1.0.0".to_string());

    let schema = json!({
        "type": "object",
        "properties": {
            "text": {"type": "string"},
            "operation": {"type": "string"}
        },
        "required": ["text"]
    });

    server
        .add_tool(
            "text_processor".to_string(),
            Some("Text processing tool".to_string()),
            schema,
            TextProcessorTool,
        )
        .await
        .unwrap();

    let test_text = "Hello World!\nThis is a test.\nWith multiple lines.";

    // Test character count
    let mut count_args = HashMap::new();
    count_args.insert("text".to_string(), json!(test_text));
    count_args.insert("operation".to_string(), json!("count"));

    let count_result = server.call_tool("text_processor", Some(count_args)).await.unwrap();
    assert!(count_result.meta.is_some());

    // Test word count
    let mut word_args = HashMap::new();
    word_args.insert("text".to_string(), json!(test_text));
    word_args.insert("operation".to_string(), json!("words"));

    let word_result = server.call_tool("text_processor", Some(word_args)).await.unwrap();
    assert_eq!(word_result.is_error, Some(false));

    // Test case conversion
    let mut upper_args = HashMap::new();
    upper_args.insert("text".to_string(), json!("hello world"));
    upper_args.insert("operation".to_string(), json!("uppercase"));

    let upper_result = server.call_tool("text_processor", Some(upper_args)).await.unwrap();
    assert_eq!(upper_result.is_error, Some(false));

    // Test reverse
    let mut reverse_args = HashMap::new();
    reverse_args.insert("text".to_string(), json!("hello"));
    reverse_args.insert("operation".to_string(), json!("reverse"));

    let reverse_result = server.call_tool("text_processor", Some(reverse_args)).await.unwrap();
    assert_eq!(reverse_result.is_error, Some(false));
}

#[tokio::test]
async fn test_resource_operations() {
    let server = McpServer::new("resource-server".to_string(), "1.0.0".to_string());

    // Add sample data resource
    let sample_info = ResourceInfo {
        uri: "memory://test-data".to_string(),
        name: Some("Test Dataset".to_string()),
        description: Some("Test data for integration testing".to_string()),
        mime_type: Some("application/json".to_string()),
        annotations: None,
        size: Some(512),
    };

    server
        .add_resource_detailed(sample_info, SampleDataResource)
        .await
        .unwrap();

    // Add user profile resource
    server
        .add_resource(
            "user-456".to_string(),
            "memory://users/456".to_string(),
            UserProfileResource::new("456".to_string()),
        )
        .await
        .unwrap();

    // Test listing resources
    let resources = server.list_resources().await.unwrap();
    assert_eq!(resources.len(), 2);

    let test_data_resource = resources.iter().find(|r| r.uri == "memory://test-data");
    assert!(test_data_resource.is_some());
    assert_eq!(test_data_resource.unwrap().name, Some("Test Dataset".to_string()));

    // Test reading resources
    let sample_contents = server.read_resource("memory://test-data").await.unwrap();
    assert_eq!(sample_contents.len(), 1);
    match &sample_contents[0] {
        ResourceContents::Text { text, .. } => {
            let data: Value = serde_json::from_str(text).unwrap();
            assert!(data.get("name").is_some());
            assert!(data.get("records").is_some());
        }
        _ => panic!("Expected text content"),
    }

    let profile_contents = server.read_resource("memory://users/456").await.unwrap();
    assert_eq!(profile_contents.len(), 1);
    match &profile_contents[0] {
        ResourceContents::Text { text, .. } => {
            let profile: Value = serde_json::from_str(text).unwrap();
            assert_eq!(profile["user_id"], "456");
        }
        _ => panic!("Expected text content"),
    }

    // Test reading non-existent resource
    let not_found = server.read_resource("memory://nonexistent").await;
    assert!(not_found.is_err());

    // Test removing resources
    let removed = server.remove_resource("memory://test-data").await.unwrap();
    assert!(removed);

    let resources_after = server.list_resources().await.unwrap();
    assert_eq!(resources_after.len(), 1);

    let not_removed = server.remove_resource("memory://nonexistent").await.unwrap();
    assert!(!not_removed);
}

#[tokio::test]
async fn test_prompt_operations() {
    let server = McpServer::new("prompt-server".to_string(), "1.0.0".to_string());

    // Add code review prompt
    let code_review_info = PromptInfo {
        name: "code_review".to_string(),
        description: Some("Code review prompt".to_string()),
        arguments: Some(vec![
            PromptArgument {
                name: "language".to_string(),
                description: Some("Programming language".to_string()),
                required: Some(false),
            },
            PromptArgument {
                name: "focus".to_string(),
                description: Some("Review focus".to_string()),
                required: Some(false),
            },
        ]),
    };

    server
        .add_prompt(code_review_info, CodeReviewPrompt)
        .await
        .unwrap();

    // Add documentation prompt
    let doc_info = PromptInfo {
        name: "documentation".to_string(),
        description: Some("Documentation prompt".to_string()),
        arguments: Some(vec![
            PromptArgument {
                name: "type".to_string(),
                description: Some("Doc type".to_string()),
                required: Some(false),
            },
        ]),
    };

    server
        .add_prompt(doc_info, DocumentationPrompt)
        .await
        .unwrap();

    // Test listing prompts
    let prompts = server.list_prompts().await.unwrap();
    assert_eq!(prompts.len(), 2);

    let code_prompt = prompts.iter().find(|p| p.name == "code_review");
    assert!(code_prompt.is_some());
    assert!(code_prompt.unwrap().arguments.is_some());

    // Test getting prompts with arguments
    let mut args = HashMap::new();
    args.insert("language".to_string(), json!("rust"));
    args.insert("focus".to_string(), json!("security"));

    let result = server.get_prompt("code_review", Some(args)).await.unwrap();
    assert!(result.description.is_some());
    assert!(result.description.unwrap().contains("rust"));
    assert_eq!(result.messages.len(), 2);
    assert!(result.meta.is_some());

    let meta = result.meta.unwrap();
    assert_eq!(meta.get("language").unwrap(), &json!("rust"));
    assert_eq!(meta.get("focus").unwrap(), &json!("security"));

    // Test with different arguments
    let mut doc_args = HashMap::new();
    doc_args.insert("type".to_string(), json!("api"));
    doc_args.insert("audience".to_string(), json!("beginners"));

    let doc_result = server.get_prompt("documentation", Some(doc_args)).await.unwrap();
    assert!(doc_result.description.is_some());
    assert!(doc_result.description.unwrap().contains("api"));
    assert_eq!(doc_result.messages.len(), 2);

    // Test prompt not found
    let not_found = server.get_prompt("nonexistent", None).await;
    assert!(not_found.is_err());

    // Test removing prompts
    let removed = server.remove_prompt("code_review").await.unwrap();
    assert!(removed);

    let prompts_after = server.list_prompts().await.unwrap();
    assert_eq!(prompts_after.len(), 1);
}

#[tokio::test]
async fn test_json_rpc_request_handling() {
    let server = McpServer::new("rpc-server".to_string(), "1.0.0".to_string());

    // Test initialize request
    let init_params = InitializeParams::new(
        ClientInfo {
            name: "test-client".to_string(),
            version: "1.0.0".to_string(),
        },
        ClientCapabilities::default(),
    );

    let init_request = JsonRpcRequest::new(
        json!(1),
        "initialize".to_string(),
        Some(init_params),
    ).unwrap();

    let init_response = server.handle_request(init_request).await.unwrap();
    assert!(init_response.result.is_some());

    // Test ping request
    let ping_request = JsonRpcRequest::new(
        json!(2),
        "ping".to_string(),
        None::<Value>,
    ).unwrap();

    let ping_response = server.handle_request(ping_request).await.unwrap();
    assert!(ping_response.result.is_some());

    // Add a tool for testing
    let schema = json!({
        "type": "object",
        "properties": {
            "message": {"type": "string"}
        }
    });

    server
        .add_tool(
            "echo".to_string(),
            Some("Echo tool".to_string()),
            schema,
            EchoTool,
        )
        .await
        .unwrap();

    // Test tools/list request
    let tools_list_params = ListToolsParams::default();
    let tools_request = JsonRpcRequest::new(
        json!(3),
        "tools/list".to_string(),
        Some(tools_list_params),
    ).unwrap();

    let tools_response = server.handle_request(tools_request).await.unwrap();
    assert!(tools_response.result.is_some());

    // Test tools/call request
    let mut call_args = HashMap::new();
    call_args.insert("message".to_string(), json!("Hello, World!"));

    let call_params = CallToolParams {
        name: "echo".to_string(),
        arguments: Some(call_args),
        meta: None,
    };

    let call_request = JsonRpcRequest::new(
        json!(4),
        "tools/call".to_string(),
        Some(call_params),
    ).unwrap();

    let call_response = server.handle_request(call_request).await.unwrap();
    assert!(call_response.result.is_some());

    // Test invalid method
    let invalid_request = JsonRpcRequest::new(
        json!(5),
        "invalid/method".to_string(),
        None::<Value>,
    ).unwrap();

    let invalid_response = server.handle_request(invalid_request).await;
    // Should still return a response with error information
    assert!(invalid_response.is_ok());
}

#[tokio::test]
async fn test_concurrent_operations() {
    let server = std::sync::Arc::new(
        McpServer::new("concurrent-server".to_string(), "1.0.0".to_string())
    );

    // Add multiple tools
    for i in 0..5 {
        let tool_name = format!("tool_{}", i);
        let schema = json!({
            "type": "object",
            "properties": {
                "a": {"type": "number"},
                "b": {"type": "number"}
            }
        });

        server
            .add_tool(
                tool_name,
                Some(format!("Tool number {}", i)),
                schema,
                AdditionTool,
            )
            .await
            .unwrap();
    }

    // Test concurrent tool calls
    let mut handles = vec![];

    for i in 0..10 {
        let server_clone = server.clone();
        let handle = tokio::spawn(async move {
            let tool_name = format!("tool_{}", i % 5);
            let mut args = HashMap::new();
            args.insert("a".to_string(), json!(i as f64));
            args.insert("b".to_string(), json!(1.0));

            server_clone.call_tool(&tool_name, Some(args)).await
        });
        handles.push(handle);
    }

    // Wait for all calls to complete
    let results = futures::future::join_all(handles).await;

    // Verify all calls succeeded - AdditionTool returns is_error: None
    for result in results {
        let tool_result = result.unwrap().unwrap();
        assert!(tool_result.is_error.is_none() || tool_result.is_error == Some(false));
    }

    // Test concurrent resource reads
    server
        .add_resource(
            "concurrent-data".to_string(),
            "memory://concurrent".to_string(),
            SampleDataResource,
        )
        .await
        .unwrap();

    let mut read_handles = vec![];

    for _ in 0..5 {
        let server_clone = server.clone();
        let handle = tokio::spawn(async move {
            server_clone.read_resource("memory://concurrent").await
        });
        read_handles.push(handle);
    }

    let read_results = futures::future::join_all(read_handles).await;

    for result in read_results {
        let contents = result.unwrap().unwrap();
        assert_eq!(contents.len(), 1);
    }
}

#[tokio::test]
async fn test_error_handling() {
    let server = McpServer::new("error-server".to_string(), "1.0.0".to_string());

    // Test missing tool
    let missing_tool_result = server.call_tool("nonexistent", None).await;
    assert!(missing_tool_result.is_err());
    match missing_tool_result.unwrap_err() {
        McpError::ToolNotFound(_) => {},
        _ => panic!("Expected ToolNotFound error"),
    }

    // Test missing resource
    let missing_resource_result = server.read_resource("memory://nonexistent").await;
    assert!(missing_resource_result.is_err());
    match missing_resource_result.unwrap_err() {
        McpError::ResourceNotFound(_) => {},
        _ => panic!("Expected ResourceNotFound error"),
    }

    // Test missing prompt
    let missing_prompt_result = server.get_prompt("nonexistent", None).await;
    assert!(missing_prompt_result.is_err());
    match missing_prompt_result.unwrap_err() {
        McpError::PromptNotFound(_) => {},
        _ => panic!("Expected PromptNotFound error"),
    }

    // Add calculator for validation testing
    let schema = json!({
        "type": "object",
        "properties": {
            "operation": {"type": "string"},
            "a": {"type": "number"},
            "b": {"type": "number"}
        },
        "required": ["operation", "a", "b"]
    });

    server
        .add_tool(
            "calculator".to_string(),
            Some("Calculator".to_string()),
            schema,
            CalculatorTool,
        )
        .await
        .unwrap();

    // Test invalid arguments
    let mut invalid_args = HashMap::new();
    invalid_args.insert("operation".to_string(), json!("invalid"));
    invalid_args.insert("a".to_string(), json!("not_a_number"));

    let validation_result = server.call_tool("calculator", Some(invalid_args)).await;
    assert!(validation_result.is_err());
}
