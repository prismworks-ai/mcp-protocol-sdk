# 📖 API Reference

Complete API documentation for the MCP Protocol SDK v0.5.0.

## Core Types

### McpServer

The main server implementation for handling MCP requests.

```rust
pub struct McpServer {
    // Internal fields...
}

impl McpServer {
    /// Create a new MCP server
    pub fn new(name: String, version: String) -> Self
    
    /// Create a server with custom configuration  
    pub fn with_config(name: String, version: String, config: ServerConfig) -> Self
    
    /// Set server capabilities
    pub fn set_capabilities(&mut self, capabilities: ServerCapabilities)
    
    /// Get server information
    pub fn info(&self) -> &ServerInfo
    
    /// Get server capabilities
    pub fn capabilities(&self) -> &ServerCapabilities
    
    /// Add a tool to the server
    pub async fn add_tool<H>(
        &self,
        name: String,
        description: Option<String>,
        schema: Value,
        handler: H,
    ) -> McpResult<()>
    where
        H: ToolHandler + 'static
    
    /// Add a tool with detailed information
    pub async fn add_tool_detailed<H>(
        &self, 
        info: ToolInfo, 
        handler: H
    ) -> McpResult<()>
    where
        H: ToolHandler + 'static
    
    /// Remove a tool from the server
    pub async fn remove_tool(&self, name: &str) -> McpResult<bool>
    
    /// List all registered tools
    pub async fn list_tools(&self) -> McpResult<Vec<ToolInfo>>
    
    /// Call a tool
    pub async fn call_tool(
        &self,
        name: &str,
        arguments: Option<HashMap<String, Value>>,
    ) -> McpResult<ToolResult>
    
    /// Add a resource to the server
    pub async fn add_resource<H>(
        &self, 
        name: String, 
        uri: String, 
        handler: H
    ) -> McpResult<()>
    where
        H: ResourceHandler + 'static
    
    /// Add a resource with detailed information
    pub async fn add_resource_detailed<H>(
        &self, 
        info: ResourceInfo, 
        handler: H
    ) -> McpResult<()>
    where
        H: ResourceHandler + 'static
    
    /// Remove a resource from the server
    pub async fn remove_resource(&self, uri: &str) -> McpResult<bool>
    
    /// List all registered resources
    pub async fn list_resources(&self) -> McpResult<Vec<ResourceInfo>>
    
    /// Read a resource
    pub async fn read_resource(&self, uri: &str) -> McpResult<Vec<ResourceContents>>
    
    /// Add a prompt to the server
    pub async fn add_prompt<H>(
        &self, 
        info: PromptInfo, 
        handler: H
    ) -> McpResult<()>
    where
        H: PromptHandler + 'static
    
    /// Remove a prompt from the server
    pub async fn remove_prompt(&self, name: &str) -> McpResult<bool>
    
    /// List all registered prompts
    pub async fn list_prompts(&self) -> McpResult<Vec<PromptInfo>>
    
    /// Get a prompt
    pub async fn get_prompt(
        &self,
        name: &str,
        arguments: Option<HashMap<String, Value>>,
    ) -> McpResult<PromptResult>
    
    /// Start the server with the given transport
    pub async fn start<T>(&mut self, transport: T) -> McpResult<()>
    where
        T: ServerTransport + 'static
    
    /// Stop the server
    pub async fn stop(&self) -> McpResult<()>
    
    /// Check if the server is running
    pub async fn is_running(&self) -> bool
    
    /// Get the current server state
    pub async fn state(&self) -> ServerState
}
```

### McpClient

Client for connecting to MCP servers.

```rust
pub struct McpClient {
    // Internal fields...
}

impl McpClient {
    /// Create a new MCP client
    pub fn new(name: String, version: String) -> Self
    
    /// Create a client with custom configuration
    pub fn with_config(name: String, version: String, config: ClientConfig) -> Self
    
    /// Set client capabilities
    pub fn set_capabilities(&mut self, capabilities: ClientCapabilities)
    
    /// Get client information
    pub fn info(&self) -> &ClientInfo
    
    /// Get client capabilities
    pub fn capabilities(&self) -> &ClientCapabilities
    
    /// Get server capabilities (if connected)
    pub async fn server_capabilities(&self) -> Option<ServerCapabilities>
    
    /// Get server information (if connected)
    pub async fn server_info(&self) -> Option<ServerInfo>
    
    /// Check if the client is connected
    pub async fn is_connected(&self) -> bool
    
    /// Connect to an MCP server using the provided transport
    pub async fn connect<T>(&mut self, transport: T) -> McpResult<InitializeResult>
    where
        T: Transport + 'static
    
    /// Disconnect from the server
    pub async fn disconnect(&self) -> McpResult<()>
}
```

## Tool System

### ToolHandler Trait

**Required trait for all tool implementations:**

```rust
#[async_trait]
pub trait ToolHandler: Send + Sync {
    /// Execute the tool with the given arguments
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult>;
}
```

### Tool Struct

Represents a registered tool with its handler and metadata:

```rust
pub struct Tool {
    pub info: ToolInfo,
    pub handler: Box<dyn ToolHandler>,
    pub enabled: bool,
    pub validator: Option<ParameterValidator>,
    pub enhanced_metadata: EnhancedToolMetadata,
}

impl Tool {
    /// Create a new tool with the given information and handler
    pub fn new<H>(
        name: String,
        description: Option<String>,
        input_schema: Value,
        handler: H,
    ) -> Self
    where
        H: ToolHandler + 'static
    
    /// Create a tool with custom validation configuration
    pub fn with_validation<H>(
        name: String,
        description: Option<String>,
        input_schema: Value,
        handler: H,
        validation_config: ValidationConfig,
    ) -> Self
    where
        H: ToolHandler + 'static
    
    /// Enable the tool
    pub fn enable(&mut self)
    
    /// Disable the tool
    pub fn disable(&mut self)
    
    /// Check if the tool is enabled
    pub fn is_enabled(&self) -> bool
    
    /// Execute the tool if it's enabled
    pub async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult>
    
    /// Execute without validation (for advanced use cases)
    pub async fn call_unchecked(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult>
    
    /// Validate parameters without executing
    pub fn validate_parameters(&self, arguments: &mut HashMap<String, Value>) -> McpResult<()>
}
```

### ToolBuilder

Fluent API for creating tools with advanced features:

```rust
pub struct ToolBuilder {
    // Internal fields...
}

impl ToolBuilder {
    /// Create a new tool builder
    pub fn new<S: Into<String>>(name: S) -> Self
    
    /// Set the tool description
    pub fn description<S: Into<String>>(self, description: S) -> Self
    
    /// Set the tool title (for UI display)
    pub fn title<S: Into<String>>(self, title: S) -> Self
    
    /// Set the input schema
    pub fn schema(self, schema: Value) -> Self
    
    /// Set custom validation configuration
    pub fn validation_config(self, config: ValidationConfig) -> Self
    
    /// Enable strict validation
    pub fn strict_validation(self) -> Self
    
    /// Enable permissive validation
    pub fn permissive_validation(self) -> Self
    
    /// Set behavior hints
    pub fn behavior_hints(self, hints: ToolBehaviorHints) -> Self
    
    /// Mark tool as read-only
    pub fn read_only(self) -> Self
    
    /// Mark tool as destructive
    pub fn destructive(self) -> Self
    
    /// Mark tool as idempotent
    pub fn idempotent(self) -> Self
    
    /// Mark tool as requiring authentication
    pub fn requires_auth(self) -> Self
    
    /// Mark tool as potentially long-running
    pub fn long_running(self) -> Self
    
    /// Mark tool as resource-intensive
    pub fn resource_intensive(self) -> Self
    
    /// Mark tool results as cacheable
    pub fn cacheable(self) -> Self
    
    /// Set tool category
    pub fn category(self, category: ToolCategory) -> Self
    
    /// Set tool version
    pub fn version<S: Into<String>>(self, version: S) -> Self
    
    /// Set tool author
    pub fn author<S: Into<String>>(self, author: S) -> Self
    
    /// Mark tool as deprecated
    pub fn deprecated(self, deprecation: ToolDeprecation) -> Self
    
    /// Add custom metadata field
    pub fn custom_metadata<S: Into<String>>(self, key: S, value: serde_json::Value) -> Self
    
    /// Build the tool with the given handler
    pub fn build<H>(self, handler: H) -> McpResult<Tool>
    where
        H: ToolHandler + 'static
}
```

## Resource System

### ResourceHandler Trait

**Required trait for resource implementations:**

```rust
#[async_trait]
pub trait ResourceHandler: Send + Sync {
    /// Read resource content
    async fn read(&self, uri: &str, params: &HashMap<String, String>) -> McpResult<Vec<ResourceContents>>;
    
    /// List available resources
    async fn list(&self) -> McpResult<Vec<ResourceInfo>>;
}
```

### Resource Struct

```rust
pub struct Resource {
    pub info: ResourceInfo,
    pub handler: Box<dyn ResourceHandler>,
}

impl Resource {
    /// Create a new resource
    pub fn new<H>(info: ResourceInfo, handler: H) -> Self
    where
        H: ResourceHandler + 'static
}
```

## Prompt System

### PromptHandler Trait

**Required trait for prompt implementations:**

```rust
#[async_trait]
pub trait PromptHandler: Send + Sync {
    /// Generate prompt content
    async fn get(&self, arguments: HashMap<String, Value>) -> McpResult<PromptResult>;
}
```

### Prompt Struct

```rust
pub struct Prompt {
    pub info: PromptInfo,
    pub handler: Box<dyn PromptHandler>,
}

impl Prompt {
    /// Create a new prompt
    pub fn new<H>(info: PromptInfo, handler: H) -> Self
    where
        H: PromptHandler + 'static
}
```

## Transport Layer

### Transport Trait (Client)

```rust
#[async_trait]
pub trait Transport: Send + Sync {
    /// Send a JSON-RPC request and wait for a response
    async fn send_request(&mut self, request: JsonRpcRequest) -> McpResult<JsonRpcResponse>;
    
    /// Send a JSON-RPC notification
    async fn send_notification(&mut self, notification: JsonRpcNotification) -> McpResult<()>;
    
    /// Receive a notification from the server
    async fn receive_notification(&mut self) -> McpResult<Option<JsonRpcNotification>>;
    
    /// Close the transport connection
    async fn close(&mut self) -> McpResult<()>;
    
    /// Check if the transport is connected
    fn is_connected(&self) -> bool;
    
    /// Get connection information
    fn connection_info(&self) -> String;
}
```

### ServerTransport Trait

```rust
#[async_trait]
pub trait ServerTransport: Send + Sync {
    /// Start the server transport
    async fn start(&mut self) -> McpResult<()>;
    
    /// Set the request handler
    fn set_request_handler(&mut self, handler: ServerRequestHandler);
    
    /// Send a notification to the client
    async fn send_notification(&mut self, notification: JsonRpcNotification) -> McpResult<()>;
    
    /// Stop the server transport
    async fn stop(&mut self) -> McpResult<()>;
}
```

### TransportConfig

Configuration options for transport implementations:

```rust
pub struct TransportConfig {
    /// Connection timeout in milliseconds
    pub connect_timeout_ms: Option<u64>,
    /// Read timeout in milliseconds
    pub read_timeout_ms: Option<u64>,
    /// Write timeout in milliseconds
    pub write_timeout_ms: Option<u64>,
    /// Maximum message size in bytes
    pub max_message_size: Option<usize>,
    /// Keep-alive interval in milliseconds
    pub keep_alive_ms: Option<u64>,
    /// Whether to enable compression
    pub compression: bool,
    /// Custom headers for HTTP-based transports
    pub headers: std::collections::HashMap<String, String>,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            connect_timeout_ms: Some(30_000),         // 30 seconds
            read_timeout_ms: Some(60_000),            // 60 seconds
            write_timeout_ms: Some(30_000),           // 30 seconds
            max_message_size: Some(16 * 1024 * 1024), // 16 MB
            keep_alive_ms: Some(30_000),              // 30 seconds
            compression: false,
            headers: std::collections::HashMap::new(),
        }
    }
}
```

## Transport Implementations

### STDIO Transport

```rust
// Server-side
pub struct StdioServerTransport {
    // Internal fields...
}

impl StdioServerTransport {
    /// Create a new STDIO server transport
    pub fn new() -> Self
}

// Client-side
pub struct StdioClientTransport {
    // Internal fields...
}

impl StdioClientTransport {
    /// Create a new STDIO client transport
    pub async fn new(command: String) -> McpResult<Self>
    
    /// Create with custom arguments
    pub async fn with_args(command: String, args: Vec<String>) -> McpResult<Self>
}
```

### HTTP Transport

```rust
// Client-side
pub struct HttpClientTransport {
    // Internal fields...
}

impl HttpClientTransport {
    /// Create a new HTTP client transport
    pub async fn new<S: AsRef<str>>(base_url: S, sse_url: Option<S>) -> McpResult<Self>
    
    /// Create with custom configuration
    pub async fn with_config<S: AsRef<str>>(
        base_url: S,
        sse_url: Option<S>,
        config: TransportConfig,
    ) -> McpResult<Self>
}

// Server-side
pub struct HttpServerTransport {
    // Internal fields...
}

impl HttpServerTransport {
    /// Create a new HTTP server transport
    pub fn new<S: Into<String>>(bind_addr: S) -> Self
    
    /// Create with custom configuration
    pub fn with_config<S: Into<String>>(bind_addr: S, config: TransportConfig) -> Self
}
```

### WebSocket Transport

```rust
// Client-side
pub struct WebSocketClientTransport {
    // Internal fields...
}

impl WebSocketClientTransport {
    /// Create a new WebSocket client transport
    pub async fn new<S: AsRef<str>>(url: S) -> McpResult<Self>
    
    /// Create with custom configuration
    pub async fn with_config<S: AsRef<str>>(url: S, config: TransportConfig) -> McpResult<Self>
}

// Server-side
pub struct WebSocketServerTransport {
    // Internal fields...
}

impl WebSocketServerTransport {
    /// Create a new WebSocket server transport
    pub fn new<S: Into<String>>(bind_addr: S) -> Self
    
    /// Create with custom configuration
    pub fn with_config<S: Into<String>>(bind_addr: S, config: TransportConfig) -> Self
}
```

## Protocol Types

### Core MCP Types

```rust
// Server information
pub struct ServerInfo {
    pub name: String,
    pub version: String,
    pub title: Option<String>,
}

// Client information
pub struct ClientInfo {
    pub name: String,
    pub version: String,
    pub title: Option<String>,
}

// Tool information
pub struct ToolInfo {
    pub name: String,
    pub description: Option<String>,
    pub input_schema: ToolInputSchema,
    pub annotations: Option<Annotations>,
    pub title: Option<String>,
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
}

// Tool execution result
pub struct ToolResult {
    pub content: Vec<Content>,
    pub is_error: Option<bool>,
    pub structured_content: Option<serde_json::Value>,
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
}

// Resource information
pub struct ResourceInfo {
    pub uri: String,
    pub name: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
    pub annotations: Option<Annotations>,
    pub size: Option<u64>,
    pub title: Option<String>,
    pub meta: Option<serde_json::Map<String, serde_json::Value>>,
}

// Resource contents
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum ResourceContents {
    Text {
        uri: String,
        mime_type: Option<String>,
        text: String,
        meta: Option<serde_json::Map<String, serde_json::Value>>,
    },
    Blob {
        uri: String,
        mime_type: Option<String>,
        blob: String, // Base64 encoded
        meta: Option<serde_json::Map<String, serde_json::Value>>,
    },
}

// Content blocks
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "lowercase")]
pub enum Content {
    Text {
        text: String,
        annotations: Option<Annotations>,
        meta: Option<serde_json::Map<String, serde_json::Value>>,
    },
    Image {
        data: String, // Base64 encoded
        mime_type: String,
        annotations: Option<Annotations>,
        meta: Option<serde_json::Map<String, serde_json::Value>>,
    },
    Audio {
        data: String, // Base64 encoded
        mime_type: String,
        annotations: Option<Annotations>,
        meta: Option<serde_json::Map<String, serde_json::Value>>,
    },
}

impl Content {
    /// Create text content
    pub fn text<S: Into<String>>(text: S) -> Self {
        Self::Text {
            text: text.into(),
            annotations: None,
            meta: None,
        }
    }
    
    /// Create image content
    pub fn image<S: Into<String>>(data: S, mime_type: S) -> Self {
        Self::Image {
            data: data.into(),
            mime_type: mime_type.into(),
            annotations: None,
            meta: None,
        }
    }
    
    /// Create audio content
    pub fn audio<S: Into<String>>(data: S, mime_type: S) -> Self {
        Self::Audio {
            data: data.into(),
            mime_type: mime_type.into(),
            annotations: None,
            meta: None,
        }
    }
}
```

## Error Handling

### McpError

```rust
#[derive(Debug, thiserror::Error)]
pub enum McpError {
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    #[error("Transport error: {0}")]
    Transport(String),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Tool not found: {0}")]
    ToolNotFound(String),
    
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
    
    #[error("Prompt not found: {0}")]
    PromptNotFound(String),
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    
    #[error("HTTP error: {0}")]
    Http(String),
    
    #[error("WebSocket error: {0}")]
    WebSocket(String),
    
    #[error("Timeout error: {0}")]
    Timeout(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

impl McpError {
    /// Create a validation error
    pub fn validation<S: Into<String>>(msg: S) -> Self {
        Self::Validation(msg.into())
    }
    
    /// Create an internal error
    pub fn internal<S: Into<String>>(msg: S) -> Self {
        Self::Internal(msg.into())
    }
}

/// Result type for MCP operations
pub type McpResult<T> = Result<T, McpError>;
```

## Helper Utilities

### Parameter Extraction

```rust
/// Extension trait for easier parameter extraction
pub trait ParameterExt {
    /// Extract a required string parameter
    fn get_string(&self, key: &str) -> McpResult<&str>;
    
    /// Extract an optional string parameter
    fn get_optional_string(&self, key: &str) -> Option<&str>;
    
    /// Extract a required number parameter
    fn get_number(&self, key: &str) -> McpResult<f64>;
    
    /// Extract an optional number parameter
    fn get_optional_number(&self, key: &str) -> Option<f64>;
    
    /// Extract a required integer parameter
    fn get_integer(&self, key: &str) -> McpResult<i64>;
    
    /// Extract an optional integer parameter
    fn get_optional_integer(&self, key: &str) -> Option<i64>;
    
    /// Extract a required boolean parameter
    fn get_boolean(&self, key: &str) -> McpResult<bool>;
    
    /// Extract an optional boolean parameter
    fn get_optional_boolean(&self, key: &str) -> Option<bool>;
}

impl ParameterExt for HashMap<String, Value> {
    // Implementation details...
}
```

## Usage Examples

### Complete Server Example

```rust
use mcp_protocol_sdk::prelude::*;
use async_trait::async_trait;
use std::collections::HashMap;
use serde_json::{json, Value};

// Tool handler implementation
struct CalculatorHandler;

#[async_trait]
impl ToolHandler for CalculatorHandler {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        let a = arguments.get_number("a")?;
        let b = arguments.get_number("b")?;
        let operation = arguments.get_string("operation")?;
        
        let result = match operation {
            "add" => a + b,
            "subtract" => a - b,
            "multiply" => a * b,
            "divide" => {
                if b == 0.0 {
                    return Ok(ToolResult {
                        content: vec![Content::text("Error: Division by zero")],
                        is_error: Some(true),
                        structured_content: None,
                        meta: None,
                    });
                }
                a / b
            }
            _ => return Err(McpError::validation("Invalid operation")),
        };
        
        Ok(ToolResult {
            content: vec![Content::text(result.to_string())],
            is_error: None,
            structured_content: Some(json!({
                "operation": operation,
                "operands": [a, b],
                "result": result
            })),
            meta: None,
        })
    }
}

#[tokio::main]
async fn main() -> McpResult<()> {
    // Create server
    let mut server = McpServer::new("calculator".to_string(), "1.0.0".to_string());
    
    // Add calculator tool
    server.add_tool(
        "calculate".to_string(),
        Some("Perform arithmetic calculations".to_string()),
        json!({
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
        }),
        CalculatorHandler,
    ).await?;
    
    // Start with STDIO transport
    use mcp_protocol_sdk::transport::stdio::StdioServerTransport;
    let transport = StdioServerTransport::new();
    server.start(transport).await?;
    
    Ok(())
}
```

### Complete Client Example

```rust
use mcp_protocol_sdk::prelude::*;
use mcp_protocol_sdk::client::McpClient;
use serde_json::json;

#[tokio::main]
async fn main() -> McpResult<()> {
    // Create client
    let mut client = McpClient::new("my-client".to_string(), "1.0.0".to_string());
    
    // Connect with STDIO transport
    use mcp_protocol_sdk::transport::stdio::StdioClientTransport;
    let transport = StdioClientTransport::new("./calculator-server".to_string()).await?;
    
    // Connect and initialize
    let init_result = client.connect(transport).await?;
    
    println!("Connected to: {} v{}", 
        init_result.server_info.name,
        init_result.server_info.version
    );
    
    // Check capabilities
    if let Some(capabilities) = client.server_capabilities().await {
        if capabilities.tools.is_some() {
            println!("✅ Server supports tools");
        }
    }
    
    Ok(())
}
```

## Type Aliases and Convenience Types

```rust
/// JSON-RPC request ID type
pub type RequestId = serde_json::Value;

/// Server request handler function type
pub type ServerRequestHandler = std::sync::Arc<
    dyn Fn(JsonRpcRequest) -> std::pin::Pin<Box<dyn std::future::Future<Output = McpResult<JsonRpcResponse>> + Send + 'static>>
        + Send
        + Sync,
>;

/// Tool parameter validation function
pub type ParameterValidationFn = Box<dyn Fn(&mut HashMap<String, Value>) -> McpResult<()> + Send + Sync>;
```

## Important Notes

### API Requirements

1. **String Parameters**: All names and identifiers must be `String`, not `&str`
2. **Async Traits**: Use `#[async_trait]` for all handler implementations
3. **Error Handling**: Use `McpResult<T>` for all fallible operations
4. **JSON Schemas**: Required for tool parameter validation

### Thread Safety

All public types implement `Send + Sync` where appropriate for safe concurrent usage.

### Memory Management

The SDK uses `Arc` and `Box` internally for efficient memory management while maintaining thread safety.

---

For complete examples and usage patterns, see the [Examples Directory](../../examples/) and [Getting Started Guide](../getting-started.md).
