//! Simple MCP Server Example
//!
//! This example demonstrates how to create a basic MCP server with a few tools,
//! resources, and prompts. It uses STDIO transport for communication.

use async_trait::async_trait;
use serde_json::{json, Value};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

use mcp_protocol_sdk::{
    core::{
        error::{McpError, McpResult},
        prompt::PromptHandler,
        resource::ResourceHandler,
        tool::ToolHandler,
    },
    protocol::types::{
        Content, PromptArgument, Prompt as PromptInfo, PromptMessage, GetPromptResult as PromptResult,
        ResourceContents, Resource as ResourceInfo, ToolResult, Role,
    },
    server::mcp_server::ServerConfig,
    server::McpServer,
    transport::stdio::StdioServerTransport,
};

// ============================================================================
// Tool Handlers
// ============================================================================

/// A simple calculator tool that adds two numbers
struct CalculatorHandler;

#[async_trait]
impl ToolHandler for CalculatorHandler {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        let a = arguments
            .get("a")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| McpError::Validation("Missing or invalid 'a' parameter".to_string()))?;

        let b = arguments
            .get("b")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| McpError::Validation("Missing or invalid 'b' parameter".to_string()))?;

        let operation = arguments
            .get("operation")
            .and_then(|v| v.as_str())
            .unwrap_or("add");

        let result = match operation {
            "add" => a + b,
            "subtract" => a - b,
            "multiply" => a * b,
            "divide" => {
                if b == 0.0 {
                    return Ok(ToolResult {
                        content: vec![Content::text("Error: Division by zero")],
                        is_error: Some(true),
                        meta: None,
                    });
                }
                a / b
            }
            _ => {
                return Ok(ToolResult {
                    content: vec![Content::text(format!(
                        "Error: Unknown operation '{}'",
                        operation
                    ))],
                    is_error: Some(true),
                    meta: None,
                });
            }
        };

        Ok(ToolResult {
            content: vec![Content::text(format!(
                "{} {} {} = {}",
                a, operation, b, result
            ))],
            is_error: None,
            meta: None,
        })
    }
}

/// A tool that echoes back the input with some formatting
struct EchoHandler;

#[async_trait]
impl ToolHandler for EchoHandler {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        let message = arguments
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("Hello, World!");

        let uppercase = arguments
            .get("uppercase")
            .and_then(|v| v.as_bool())
            .unwrap_or(false);

        let prefix = arguments
            .get("prefix")
            .and_then(|v| v.as_str())
            .unwrap_or("Echo");

        let formatted_message = if uppercase {
            format!("{}: {}", prefix, message.to_uppercase())
        } else {
            format!("{}: {}", prefix, message)
        };

        Ok(ToolResult {
            content: vec![Content::text(formatted_message)],
            is_error: None,
            meta: None,
        })
    }
}

// ============================================================================
// Resource Handlers
// ============================================================================

/// A simple file system resource handler (simplified for demo)
struct FileSystemHandler {
    files: Arc<RwLock<HashMap<String, String>>>,
}

impl FileSystemHandler {
    fn new() -> Self {
        let mut files = HashMap::new();
        files.insert(
            "file:///demo.txt".to_string(),
            "This is a demo file!".to_string(),
        );
        files.insert(
            "file:///config.json".to_string(),
            r#"{"name": "demo", "version": "1.0"}"#.to_string(),
        );
        files.insert(
            "file:///data.csv".to_string(),
            "name,age\nAlice,30\nBob,25\n".to_string(),
        );

        Self {
            files: Arc::new(RwLock::new(files)),
        }
    }
}

#[async_trait]
impl ResourceHandler for FileSystemHandler {
    async fn read(
        &self,
        uri: &str,
        _params: &HashMap<String, String>,
    ) -> McpResult<Vec<ResourceContents>> {
        let files = self.files.read().await;

        if let Some(content) = files.get(uri) {
            let mime_type = if uri.ends_with(".json") {
                Some("application/json".to_string())
            } else if uri.ends_with(".csv") {
                Some("text/csv".to_string())
            } else {
                Some("text/plain".to_string())
            };

            Ok(vec![ResourceContents::Text {
                uri: uri.to_string(),
                mime_type,
                text: content.clone(),
            }])
        } else {
            Err(McpError::ResourceNotFound(uri.to_string()))
        }
    }

    async fn list(&self) -> McpResult<Vec<ResourceInfo>> {
        let files = self.files.read().await;

        Ok(files
            .keys()
            .map(|uri| {
                let name = uri.split('/').last().unwrap_or(uri);
                let mime_type = if uri.ends_with(".json") {
                    Some("application/json".to_string())
                } else if uri.ends_with(".csv") {
                    Some("text/csv".to_string())
                } else {
                    Some("text/plain".to_string())
                };

                ResourceInfo {
                    uri: uri.clone(),
                    name: Some(name.to_string()),
                    description: Some(format!("Demo file: {}", name)),
                    mime_type,
                    annotations: None,
                    size: None,
                }
            })
            .collect())
    }

    async fn subscribe(&self, _uri: &str) -> McpResult<()> {
        // In a real implementation, this would set up file watching
        Ok(())
    }

    async fn unsubscribe(&self, _uri: &str) -> McpResult<()> {
        // In a real implementation, this would remove file watching
        Ok(())
    }
}

// ============================================================================
// Prompt Handlers
// ============================================================================

/// A prompt handler that generates code review prompts
struct CodeReviewPromptHandler;

#[async_trait]
impl PromptHandler for CodeReviewPromptHandler {
    async fn get(&self, arguments: HashMap<String, Value>) -> McpResult<PromptResult> {
        let language = arguments
            .get("language")
            .and_then(|v| v.as_str())
            .unwrap_or("any");

        let focus = arguments
            .get("focus")
            .and_then(|v| v.as_str())
            .unwrap_or("general");

        let system_prompt = format!(
            "You are an expert code reviewer specializing in {} code. Focus on {} aspects of code quality.",
            language, focus
        );

        let user_prompt = match focus {
            "security" => "Please review this code for security vulnerabilities, potential exploits, and security best practices.",
            "performance" => "Please review this code for performance issues, optimization opportunities, and efficiency improvements.",
            "style" => "Please review this code for style consistency, readability, and adherence to coding standards.",
            _ => "Please provide a comprehensive code review covering functionality, readability, maintainability, and best practices.",
        };

        Ok(PromptResult {
            description: Some(format!(
                "Code review prompt for {} focusing on {}",
                language, focus
            )),
            messages: vec![
                PromptMessage {
                    role: Role::User,
                    content: Content::text(system_prompt),
                },
                PromptMessage {
                    role: Role::User,
                    content: Content::text(user_prompt),
                },
            ],
            meta: None,
        })
    }
}

/// A prompt handler that generates documentation prompts
struct DocumentationPromptHandler;

#[async_trait]
impl PromptHandler for DocumentationPromptHandler {
    async fn get(&self, arguments: HashMap<String, Value>) -> McpResult<PromptResult> {
        let doc_type = arguments
            .get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("function");

        let language = arguments
            .get("language")
            .and_then(|v| v.as_str())
            .unwrap_or("any");

        let (system_prompt, user_prompt) = match doc_type {
            "api" => (
                format!("You are a technical writer specializing in API documentation for {} applications.", language),
                "Please generate comprehensive API documentation for the provided code, including endpoints, parameters, responses, and usage examples."
            ),
            "class" => (
                format!("You are a technical writer specializing in {} class documentation.", language),
                "Please generate detailed class documentation including purpose, methods, properties, usage examples, and relationships with other classes."
            ),
            "function" => (
                format!("You are a technical writer specializing in {} function documentation.", language),
                "Please generate comprehensive function documentation including purpose, parameters, return values, exceptions, and usage examples."
            ),
            _ => (
                "You are a technical writer specializing in software documentation.".to_string(),
                "Please generate appropriate documentation for the provided code."
            ),
        };

        Ok(PromptResult {
            description: Some(format!(
                "Documentation prompt for {} {}",
                language, doc_type
            )),
            messages: vec![
                PromptMessage {
                    role: Role::User,
                    content: Content::text(system_prompt),
                },
                PromptMessage {
                    role: Role::User,
                    content: Content::text(user_prompt),
                },
            ],
            meta: None,
        })
    }
}

// ============================================================================
// Main Server Setup
// ============================================================================

#[tokio::main]
async fn main() -> McpResult<()> {
    // Initialize logging
    #[cfg(feature = "tracing-subscriber")]
    tracing_subscriber::fmt::init();

    // Create server with custom configuration
    let config = ServerConfig {
        max_concurrent_requests: 50,
        request_timeout_ms: 30000,
        validate_requests: true,
        enable_logging: true,
    };

    let mut server = McpServer::with_config(
        "simple-demo-server".to_string(),
        "1.0.0".to_string(),
        config,
    );

    // Add tools
    tracing::info!("Adding tools...");

    server
        .add_tool(
            "calculator".to_string(),
            Some("Perform basic arithmetic operations".to_string()),
            json!({
                "type": "object",
                "properties": {
                    "a": {
                        "type": "number",
                        "description": "First number"
                    },
                    "b": {
                        "type": "number",
                        "description": "Second number"
                    },
                    "operation": {
                        "type": "string",
                        "enum": ["add", "subtract", "multiply", "divide"],
                        "description": "Operation to perform",
                        "default": "add"
                    }
                },
                "required": ["a", "b"]
            }),
            CalculatorHandler,
        )
        .await?;

    server
        .add_tool(
            "echo".to_string(),
            Some("Echo back a message with optional formatting".to_string()),
            json!({
                "type": "object",
                "properties": {
                    "message": {
                        "type": "string",
                        "description": "Message to echo back"
                    },
                    "uppercase": {
                        "type": "boolean",
                        "description": "Convert to uppercase",
                        "default": false
                    },
                    "prefix": {
                        "type": "string",
                        "description": "Prefix to add to the message",
                        "default": "Echo"
                    }
                }
            }),
            EchoHandler,
        )
        .await?;

    // Add resources
    tracing::info!("Adding resources...");

    let fs_handler = FileSystemHandler::new();
    server
        .add_resource_detailed(
            ResourceInfo {
                uri: "file:///".to_string(),
                name: Some("Demo File System".to_string()),
                description: Some("Demo file system with sample files".to_string()),
                mime_type: Some("inode/directory".to_string()),
                annotations: None,
                size: None,
            },
            fs_handler,
        )
        .await?;

    // Add prompts
    tracing::info!("Adding prompts...");

    server
        .add_prompt(
            PromptInfo {
                name: "code-review".to_string(),
                description: Some("Generate code review prompts".to_string()),
                arguments: Some(vec![
                    PromptArgument {
                        name: "language".to_string(),
                        description: Some("Programming language".to_string()),
                        required: Some(false),
                    },
                    PromptArgument {
                        name: "focus".to_string(),
                        description: Some(
                            "Review focus (security, performance, style, general)".to_string(),
                        ),
                        required: Some(false),
                    },
                ]),
            },
            CodeReviewPromptHandler,
        )
        .await?;

    server
        .add_prompt(
            PromptInfo {
                name: "documentation".to_string(),
                description: Some("Generate documentation prompts".to_string()),
                arguments: Some(vec![
                    PromptArgument {
                        name: "type".to_string(),
                        description: Some("Documentation type (api, class, function)".to_string()),
                        required: Some(false),
                    },
                    PromptArgument {
                        name: "language".to_string(),
                        description: Some("Programming language".to_string()),
                        required: Some(false),
                    },
                ]),
            },
            DocumentationPromptHandler,
        )
        .await?;

    // Create and start the server with STDIO transport
    tracing::info!("Starting server...");

    let transport = StdioServerTransport::new();
    server.start(transport).await?;

    tracing::info!("Server started successfully! Listening for requests...");

    // Keep the server running until interrupted
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl+c");

    tracing::info!("Shutting down server...");
    server.stop().await?;
    tracing::info!("Server stopped");

    Ok(())
}
