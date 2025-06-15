//! Tool system for MCP servers
//!
//! This module provides the abstraction for implementing and managing tools in MCP servers.
//! Tools are functions that can be called by clients to perform specific operations.

use async_trait::async_trait;
use serde_json::Value;
use std::collections::HashMap;

use crate::core::error::{McpError, McpResult};
use crate::protocol::types::{Content, ToolInfo, ToolInputSchema, ToolResult};

/// Trait for implementing tool handlers
#[async_trait]
pub trait ToolHandler: Send + Sync {
    /// Execute the tool with the given arguments
    ///
    /// # Arguments
    /// * `arguments` - Tool arguments as key-value pairs
    ///
    /// # Returns
    /// Result containing the tool execution result or an error
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult>;
}

/// A registered tool with its handler
pub struct Tool {
    /// Information about the tool
    pub info: ToolInfo,
    /// Handler that implements the tool's functionality
    pub handler: Box<dyn ToolHandler>,
    /// Whether the tool is currently enabled
    pub enabled: bool,
}

impl Tool {
    /// Create a new tool with the given information and handler
    ///
    /// # Arguments
    /// * `name` - Name of the tool
    /// * `description` - Optional description of the tool
    /// * `input_schema` - JSON schema describing the input parameters
    /// * `handler` - Implementation of the tool's functionality
    pub fn new<H>(
        name: String,
        description: Option<String>,
        input_schema: Value,
        handler: H,
    ) -> Self
    where
        H: ToolHandler + 'static,
    {
        Self {
            info: ToolInfo {
                name,
                description,
                input_schema: ToolInputSchema {
                    schema_type: "object".to_string(),
                    properties: input_schema
                        .get("properties")
                        .and_then(|p| p.as_object())
                        .map(|obj| obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect()),
                    required: input_schema
                        .get("required")
                        .and_then(|r| r.as_array())
                        .map(|arr| {
                            arr.iter()
                                .filter_map(|v| v.as_str().map(String::from))
                                .collect()
                        }),
                    additional_properties: input_schema
                        .as_object()
                        .unwrap_or(&serde_json::Map::new())
                        .iter()
                        .filter(|(k, _)| !["type", "properties", "required"].contains(&k.as_str()))
                        .map(|(k, v)| (k.clone(), v.clone()))
                        .collect(),
                },
                annotations: None,
            },
            handler: Box::new(handler),
            enabled: true,
        }
    }

    /// Enable the tool
    pub fn enable(&mut self) {
        self.enabled = true;
    }

    /// Disable the tool
    pub fn disable(&mut self) {
        self.enabled = false;
    }

    /// Check if the tool is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Execute the tool if it's enabled
    ///
    /// # Arguments
    /// * `arguments` - Tool arguments as key-value pairs
    ///
    /// # Returns
    /// Result containing the tool execution result or an error
    pub async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        if !self.enabled {
            return Err(McpError::validation(format!(
                "Tool '{}' is disabled",
                self.info.name
            )));
        }

        self.handler.call(arguments).await
    }
}

impl std::fmt::Debug for Tool {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Tool")
            .field("info", &self.info)
            .field("enabled", &self.enabled)
            .finish()
    }
}

/// Helper macro for creating tools with schema validation
///
/// # Examples
/// ```rust
/// use mcp_protocol_sdk::{tool, core::tool::ToolHandler};
/// use serde_json::json;
///
/// struct MyHandler;
/// #[async_trait::async_trait]
/// impl ToolHandler for MyHandler {
///     async fn call(&self, _args: std::collections::HashMap<String, serde_json::Value>) -> mcp_protocol_sdk::McpResult<mcp_protocol_sdk::protocol::types::ToolResult> {
///         // Implementation here
///         todo!()
///     }
/// }
///
/// let tool = tool!(
///     "my_tool",
///     "A sample tool",
///     json!({
///         "type": "object",
///         "properties": {
///             "input": { "type": "string" }
///         }
///     }),
///     MyHandler
/// );
/// ```
#[macro_export]
macro_rules! tool {
    ($name:expr, $schema:expr, $handler:expr) => {
        $crate::core::tool::Tool::new($name.to_string(), None, $schema, $handler)
    };
    ($name:expr, $description:expr, $schema:expr, $handler:expr) => {
        $crate::core::tool::Tool::new(
            $name.to_string(),
            Some($description.to_string()),
            $schema,
            $handler,
        )
    };
}

// Common tool implementations

/// Simple echo tool for testing
pub struct EchoTool;

#[async_trait]
impl ToolHandler for EchoTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        let message = arguments
            .get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("Hello, World!");

        Ok(ToolResult {
            content: vec![Content::Text {
                text: message.to_string(),
                annotations: None,
            }],
            is_error: None,
            meta: None,
        })
    }
}

/// Tool for adding two numbers
pub struct AdditionTool;

#[async_trait]
impl ToolHandler for AdditionTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        let a = arguments
            .get("a")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| McpError::validation("Missing or invalid 'a' parameter"))?;

        let b = arguments
            .get("b")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| McpError::validation("Missing or invalid 'b' parameter"))?;

        let result = a + b;

        Ok(ToolResult {
            content: vec![Content::Text {
                text: result.to_string(),
                annotations: None,
            }],
            is_error: None,
            meta: None,
        })
    }
}

/// Tool for getting current timestamp
pub struct TimestampTool;

#[async_trait]
impl ToolHandler for TimestampTool {
    async fn call(&self, _arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        use std::time::{SystemTime, UNIX_EPOCH};

        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|e| McpError::internal(e.to_string()))?
            .as_secs();

        Ok(ToolResult {
            content: vec![Content::Text {
                text: timestamp.to_string(),
                annotations: None,
            }],
            is_error: None,
            meta: None,
        })
    }
}

/// Builder for creating tools with fluent API
pub struct ToolBuilder {
    name: String,
    description: Option<String>,
    input_schema: Option<Value>,
}

impl ToolBuilder {
    /// Create a new tool builder with the given name
    pub fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            description: None,
            input_schema: None,
        }
    }

    /// Set the tool description
    pub fn description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }

    /// Set the input schema
    pub fn schema(mut self, schema: Value) -> Self {
        self.input_schema = Some(schema);
        self
    }

    /// Build the tool with the given handler
    pub fn build<H>(self, handler: H) -> McpResult<Tool>
    where
        H: ToolHandler + 'static,
    {
        let schema = self.input_schema.unwrap_or_else(|| {
            serde_json::json!({
                "type": "object",
                "properties": {},
                "additionalProperties": true
            })
        });

        Ok(Tool::new(self.name, self.description, schema, handler))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_echo_tool() {
        let tool = EchoTool;
        let mut args = HashMap::new();
        args.insert("message".to_string(), json!("test message"));

        let result = tool.call(args).await.unwrap();
        match &result.content[0] {
            Content::Text { text, .. } => assert_eq!(text, "test message"),
            _ => panic!("Expected text content"),
        }
    }

    #[tokio::test]
    async fn test_addition_tool() {
        let tool = AdditionTool;
        let mut args = HashMap::new();
        args.insert("a".to_string(), json!(5.0));
        args.insert("b".to_string(), json!(3.0));

        let result = tool.call(args).await.unwrap();
        match &result.content[0] {
            Content::Text { text, .. } => assert_eq!(text, "8"),
            _ => panic!("Expected text content"),
        }
    }

    #[test]
    fn test_tool_creation() {
        let tool = Tool::new(
            "test_tool".to_string(),
            Some("Test tool".to_string()),
            json!({"type": "object"}),
            EchoTool,
        );

        assert_eq!(tool.info.name, "test_tool");
        assert_eq!(tool.info.description, Some("Test tool".to_string()));
        assert!(tool.is_enabled());
    }

    #[test]
    fn test_tool_enable_disable() {
        let mut tool = Tool::new(
            "test_tool".to_string(),
            None,
            json!({"type": "object"}),
            EchoTool,
        );

        assert!(tool.is_enabled());

        tool.disable();
        assert!(!tool.is_enabled());

        tool.enable();
        assert!(tool.is_enabled());
    }

    #[tokio::test]
    async fn test_disabled_tool() {
        let mut tool = Tool::new(
            "test_tool".to_string(),
            None,
            json!({"type": "object"}),
            EchoTool,
        );

        tool.disable();

        let result = tool.call(HashMap::new()).await;
        assert!(result.is_err());
        match result.unwrap_err() {
            McpError::Validation(msg) => assert!(msg.contains("disabled")),
            _ => panic!("Expected validation error"),
        }
    }

    #[test]
    fn test_tool_builder() {
        let tool = ToolBuilder::new("test")
            .description("A test tool")
            .schema(json!({"type": "object", "properties": {"x": {"type": "number"}}}))
            .build(EchoTool)
            .unwrap();

        assert_eq!(tool.info.name, "test");
        assert_eq!(tool.info.description, Some("A test tool".to_string()));
    }
}
