# 📖 API Reference

Complete API documentation for the MCP Protocol SDK.

## Core Types

### McpServer

The main server implementation for handling MCP requests.

```rust
pub struct McpServer {
    // Internal fields...
}

impl McpServer {
    /// Create a new MCP server
    pub fn new(name: &str, version: &str) -> Self
    
    /// Create a server with description
    pub fn with_description(mut self, description: &str) -> Self
    
    /// Add a tool to the server
    pub fn add_tool(&mut self, tool: Tool) -> &mut Self
    
    /// Add a resource to the server
    pub fn add_resource(&mut self, resource: Resource) -> &mut Self
    
    /// Add a prompt to the server
    pub fn add_prompt(&mut self, prompt: Prompt) -> &mut Self
    
    /// Set a tool handler
    pub fn set_tool_handler<F, Fut>(&mut self, name: &str, handler: F)
    where
        F: Fn(HashMap<String, Value>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Value, String>> + Send + 'static
    
    /// Set a resource handler
    pub fn set_resource_handler<F, Fut>(&mut self, uri_pattern: &str, handler: F)
    where
        F: Fn(&str) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<ResourceContent, String>> + Send + 'static
    
    /// Set a prompt handler
    pub fn set_prompt_handler<F, Fut>(&mut self, name: &str, handler: F)
    where
        F: Fn(HashMap<String, Value>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<PromptMessage, String>> + Send + 'static
    
    /// Run the server with a transport
    pub async fn run<T: Transport>(self, transport: T) -> Result<(), McpError>
}
```

### McpClient

Client for connecting to MCP servers.

```rust
pub struct McpClient {
    // Internal fields...
}

impl McpClient {
    /// Create a new client
    pub fn new() -> ClientBuilder
    
    /// Connect to a server
    pub async fn connect<T: Transport>(&self, transport: T) -> Result<(), McpError>
    
    /// Initialize the connection
    pub async fn initialize(&self) -> Result<InitializeResult, McpError>
    
    /// List available tools
    pub async fn list_tools(&self) -> Result<Vec<ToolInfo>, McpError>
    
    /// Call a tool
    pub async fn call_tool(&self, name: &str, params: Value) -> Result<Value, McpError>
    
    /// List available resources
    pub async fn list_resources(&self) -> Result<Vec<ResourceInfo>, McpError>
    
    /// Read a resource
    pub async fn read_resource(&self, uri: &str) -> Result<ResourceContent, McpError>
    
    /// List available prompts
    pub async fn list_prompts(&self) -> Result<Vec<PromptInfo>, McpError>
    
    /// Get a prompt
    pub async fn get_prompt(&self, name: &str, params: Value) -> Result<PromptMessage, McpError>
}
```

## Tool System

### Tool

Define tools that AI models can call.

```rust
pub struct Tool {
    pub name: String,
    pub description: String,
    pub parameters: Vec<ToolParameter>,
}

impl Tool {
    /// Create a new tool
    pub fn new(name: &str, description: &str) -> Self
    
    /// Add a parameter to the tool
    pub fn with_parameter(mut self, name: &str, description: &str, required: bool) -> Self
    
    /// Add a typed parameter
    pub fn with_typed_parameter(mut self, param: ToolParameter) -> Self
}

pub struct ToolParameter {
    pub name: String,
    pub description: String,
    pub parameter_type: ParameterType,
    pub required: bool,
    pub default_value: Option<Value>,
}

pub enum ParameterType {
    String,
    Number,
    Boolean,
    Array,
    Object,
}
```

### Tool Handler

```rust
pub type ToolHandler = Box<dyn Fn(HashMap<String, Value>) -> Pin<Box<dyn Future<Output = Result<Value, String>> + Send>> + Send + Sync>;
```

### Tool Call Result

```rust
pub struct ToolCallResult {
    pub content: Vec<Content>,
    pub is_error: bool,
}

pub enum Content {
    Text(TextContent),
    Image(ImageContent),
    Resource(ResourceContent),
}

pub struct TextContent {
    pub text: String,
    pub mime_type: Option<String>,
}
```

## Resource System

### Resource

Define resources that can be accessed.

```rust
pub struct Resource {
    pub uri: String,
    pub name: Option<String>,
    pub description: Option<String>,
    pub mime_type: Option<String>,
}

impl Resource {
    /// Create a new resource
    pub fn new(uri: &str, name: &str, mime_type: &str) -> Self
    
    /// Set resource description
    pub fn with_description(mut self, description: &str) -> Self
    
    /// Set MIME type
    pub fn with_mime_type(mut self, mime_type: &str) -> Self
}
```

### Resource Content

```rust
pub enum ResourceContent {
    Text(String),
    Binary(Vec<u8>),
}

impl ResourceContent {
    /// Get content as text
    pub fn as_text(&self) -> Option<&str>
    
    /// Get content as bytes
    pub fn as_bytes(&self) -> &[u8]
    
    /// Convert to string if possible
    pub fn to_string(&self) -> Result<String, std::str::Utf8Error>
}
```

## Prompt System

### Prompt

Define reusable prompt templates.

```rust
pub struct Prompt {
    pub name: String,
    pub description: Option<String>,
    pub parameters: Vec<PromptParameter>,
}

impl Prompt {
    /// Create a new prompt
    pub fn new(name: &str, description: &str) -> Self
    
    /// Add a parameter
    pub fn with_parameter(mut self, name: &str, description: &str, required: bool) -> Self
}

pub struct PromptParameter {
    pub name: String,
    pub description: String,
    pub required: bool,
}
```

### Prompt Message

```rust
pub struct PromptMessage {
    pub role: MessageRole,
    pub content: MessageContent,
}

pub enum MessageRole {
    User,
    Assistant,
    System,
}

pub enum MessageContent {
    Text(String),
    Mixed(Vec<ContentPart>),
}

pub enum ContentPart {
    Text(String),
    Image(ImageContent),
}
```

## Transport Layer

### Transport Trait

```rust
pub trait Transport: Send + Sync {
    type Error: std::error::Error + Send + Sync + 'static;
    
    /// Send a message
    async fn send(&mut self, message: JsonRpcMessage) -> Result<(), Self::Error>;
    
    /// Receive a message
    async fn receive(&mut self) -> Result<JsonRpcMessage, Self::Error>;
    
    /// Close the transport
    async fn close(&mut self) -> Result<(), Self::Error>;
}
```

### STDIO Transport

```rust
pub struct StdioServerTransport {
    // Internal fields...
}

impl StdioServerTransport {
    /// Create a new STDIO transport
    pub fn new() -> Self
    
    /// Set buffer size
    pub fn with_buffer_size(mut self, size: usize) -> Self
    
    /// Set timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self
}

pub struct StdioClientTransport {
    // Internal fields...
}

impl StdioClientTransport {
    /// Create a new STDIO client transport
    pub fn new() -> Self
    
    /// Create transport for external process
    pub fn new_process(command: &str, args: &[&str]) -> Result<Self, std::io::Error>
}
```

### HTTP Transport

```rust
pub struct HttpServerTransport {
    // Internal fields...
}

impl HttpServerTransport {
    /// Create a new HTTP server transport
    pub fn new(bind_addr: &str) -> Self
    
    /// Enable CORS
    pub fn with_cors_enabled(mut self, enabled: bool) -> Self
    
    /// Set timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self
    
    /// Set maximum connections
    pub fn with_max_connections(mut self, max: usize) -> Self
}

pub struct HttpClientTransport {
    // Internal fields...
}

impl HttpClientTransport {
    /// Create a new HTTP client transport
    pub fn new(server_url: &str) -> Self
    
    /// Set timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self
    
    /// Set authentication token
    pub fn with_auth_token(mut self, token: &str) -> Self
}
```

### WebSocket Transport

```rust
pub struct WebSocketServerTransport {
    // Internal fields...
}

impl WebSocketServerTransport {
    /// Create a new WebSocket server transport
    pub fn new(bind_addr: &str) -> Self
    
    /// Set heartbeat interval
    pub fn with_heartbeat_interval(mut self, interval: Duration) -> Self
    
    /// Set maximum connections
    pub fn with_max_connections(mut self, max: usize) -> Self
}

pub struct WebSocketClientTransport {
    // Internal fields...
}

impl WebSocketClientTransport {
    /// Create a new WebSocket client transport
    pub fn new(server_url: &str) -> Self
    
    /// Set heartbeat interval
    pub fn with_heartbeat_interval(mut self, interval: Duration) -> Self
}
```

## Protocol Types

### JSON-RPC Messages

```rust
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: Value,
    pub method: String,
    pub params: Option<Value>,
}

pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: Value,
    pub result: Option<Value>,
    pub error: Option<JsonRpcError>,
}

pub struct JsonRpcNotification {
    pub jsonrpc: String,
    pub method: String,
    pub params: Option<Value>,
}

pub struct JsonRpcError {
    pub code: i32,
    pub message: String,
    pub data: Option<Value>,
}
```

### MCP Protocol Messages

```rust
pub struct InitializeParams {
    pub protocol_version: String,
    pub capabilities: ClientCapabilities,
    pub client_info: ClientInfo,
}

pub struct ClientCapabilities {
    pub tools: Option<ToolCapabilities>,
    pub resources: Option<ResourceCapabilities>,
    pub prompts: Option<PromptCapabilities>,
}

pub struct ClientInfo {
    pub name: String,
    pub version: String,
}
```

## Error Handling

### McpError

```rust
pub enum McpError {
    /// Transport-related errors
    Transport(Box<dyn std::error::Error + Send + Sync>),
    
    /// Protocol-related errors
    Protocol(String),
    
    /// JSON serialization/deserialization errors
    Json(serde_json::Error),
    
    /// Tool not found
    ToolNotFound(String),
    
    /// Resource not found
    ResourceNotFound(String),
    
    /// Prompt not found
    PromptNotFound(String),
    
    /// Invalid parameters
    InvalidParameters(String),
    
    /// Timeout error
    Timeout,
    
    /// Connection error
    Connection(String),
}

impl std::error::Error for McpError {}

impl std::fmt::Display for McpError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            McpError::Transport(e) => write!(f, "Transport error: {}", e),
            McpError::Protocol(msg) => write!(f, "Protocol error: {}", msg),
            McpError::Json(e) => write!(f, "JSON error: {}", e),
            McpError::ToolNotFound(name) => write!(f, "Tool not found: {}", name),
            McpError::ResourceNotFound(uri) => write!(f, "Resource not found: {}", uri),
            McpError::PromptNotFound(name) => write!(f, "Prompt not found: {}", name),
            McpError::InvalidParameters(msg) => write!(f, "Invalid parameters: {}", msg),
            McpError::Timeout => write!(f, "Request timeout"),
            McpError::Connection(msg) => write!(f, "Connection error: {}", msg),
        }
    }
}
```

## Validation

### ValidationError

```rust
pub enum ValidationError {
    /// Missing required field
    MissingField(String),
    
    /// Invalid field type
    InvalidType { field: String, expected: String, found: String },
    
    /// Invalid field value
    InvalidValue { field: String, value: String, reason: String },
    
    /// Schema validation error
    SchemaError(String),
}

/// Validation result type
pub type ValidationResult<T> = Result<T, ValidationError>;
```

### Validators

```rust
pub trait Validator<T> {
    fn validate(&self, value: &T) -> ValidationResult<()>;
}

pub struct ToolValidator;
pub struct ResourceValidator;
pub struct PromptValidator;
pub struct MessageValidator;
```

## Utilities

### URI Utilities

```rust
pub mod uri {
    /// Parse a URI with parameters
    pub fn parse_uri_with_params(uri: &str) -> Result<(String, HashMap<String, String>), String>
    
    /// Join URI components
    pub fn join_uri(base: &str, path: &str) -> String
    
    /// Normalize a URI
    pub fn normalize_uri(uri: &str) -> String
    
    /// Validate URI format
    pub fn validate_uri(uri: &str) -> Result<(), String>
    
    /// Get file extension from URI
    pub fn get_uri_extension(uri: &str) -> Option<String>
    
    /// Guess MIME type from URI
    pub fn guess_mime_type(uri: &str) -> Option<String>
    
    /// Percent encode/decode
    pub fn percent_encode(input: &str) -> String
    pub fn percent_decode(input: &str) -> Result<String, String>
}
```

## Builder Patterns

### ServerBuilder

```rust
pub struct ServerBuilder {
    // Internal fields...
}

impl ServerBuilder {
    /// Set server name
    pub fn name(mut self, name: &str) -> Self
    
    /// Set server version
    pub fn version(mut self, version: &str) -> Self
    
    /// Set server description
    pub fn description(mut self, description: &str) -> Self
    
    /// Set request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self
    
    /// Set maximum concurrent requests
    pub fn max_concurrent_requests(mut self, max: usize) -> Self
    
    /// Set request size limit
    pub fn request_size_limit(mut self, limit: usize) -> Self
    
    /// Build the server
    pub fn build(self) -> McpServer
}
```

### ClientBuilder

```rust
pub struct ClientBuilder {
    // Internal fields...
}

impl ClientBuilder {
    /// Set client name
    pub fn name(mut self, name: &str) -> Self
    
    /// Set client version
    pub fn version(mut self, version: &str) -> Self
    
    /// Set request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self
    
    /// Set retry configuration
    pub fn retry_config(mut self, config: RetryConfig) -> Self
    
    /// Set connection pool size
    pub fn connection_pool_size(mut self, size: usize) -> Self
    
    /// Build the client
    pub fn build(self) -> McpClient
}
```

## Configuration Types

### RetryConfig

```rust
pub struct RetryConfig {
    pub max_retries: usize,
    pub initial_delay: Duration,
    pub max_delay: Duration,
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
        }
    }
}
```

### TransportConfig

```rust
pub struct TransportConfig {
    pub timeout: Duration,
    pub buffer_size: usize,
    pub keep_alive: bool,
}

impl Default for TransportConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            buffer_size: 8192,
            keep_alive: true,
        }
    }
}
```

## Feature Flags

### Conditional Compilation

```rust
// HTTP transport (requires "http" feature)
#[cfg(feature = "http")]
pub mod http {
    pub use crate::transport::http::*;
}

// WebSocket transport (requires "websocket" feature)
#[cfg(feature = "websocket")]
pub mod websocket {
    pub use crate::transport::websocket::*;
}

// Validation utilities (requires "validation" feature)
#[cfg(feature = "validation")]
pub mod validation {
    pub use crate::validation::*;
}
```

## Testing Utilities

### Mock Types

```rust
#[cfg(feature = "testing")]
pub mod testing {
    /// Mock server for testing clients
    pub struct MockServer {
        // Internal fields...
    }
    
    impl MockServer {
        pub fn new() -> Self
        
        pub fn expect_tool_call(&mut self, name: &str) -> &mut ToolCallExpectation
        
        pub fn expect_resource_read(&mut self, uri: &str) -> &mut ResourceReadExpectation
        
        pub fn start(&mut self) -> MockTransport
    }
    
    /// Mock client for testing servers
    pub struct MockClient {
        // Internal fields...
    }
    
    impl MockClient {
        pub fn new() -> Self
        
        pub async fn call_tool(&self, name: &str, params: Value) -> Result<Value, McpError>
        
        pub async fn read_resource(&self, uri: &str) -> Result<ResourceContent, McpError>
    }
    
    /// Mock transport for testing
    pub struct MockTransport {
        // Internal fields...
    }
}
```

## Prelude

Common imports for convenience:

```rust
pub mod prelude {
    // Core types
    pub use crate::{McpServer, McpClient};
    
    // Tool system
    pub use crate::core::tool::{Tool, ToolParameter, ParameterType};
    
    // Resource system
    pub use crate::core::resource::{Resource, ResourceContent};
    
    // Prompt system
    pub use crate::core::prompt::{Prompt, PromptMessage, MessageRole};
    
    // Transport layer
    pub use crate::transport::{Transport, StdioServerTransport, StdioClientTransport};
    
    #[cfg(feature = "http")]
    pub use crate::transport::{HttpServerTransport, HttpClientTransport};
    
    #[cfg(feature = "websocket")]
    pub use crate::transport::{WebSocketServerTransport, WebSocketClientTransport};
    
    // Error types
    pub use crate::errors::McpError;
    
    // Protocol types
    pub use crate::protocol::types::{JsonRpcRequest, JsonRpcResponse, JsonRpcNotification};
    
    // Common external types
    pub use serde_json::{json, Value};
    pub use std::collections::HashMap;
    pub use std::time::Duration;
    
    // Async runtime
    pub use tokio;
}
```

## Examples

### Basic Server Example

```rust
use mcp_protocol_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<(), McpError> {
    let mut server = McpServer::new("example-server", "1.0.0");
    
    // Add a simple echo tool
    let echo_tool = Tool::new("echo", "Echo back the input message")
        .with_parameter("message", "Message to echo", true);
    
    server.add_tool(echo_tool);
    
    server.set_tool_handler("echo", |params| async move {
        let message = params.get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("No message provided");
        
        Ok(json!({ "echo": message }))
    });
    
    let transport = StdioServerTransport::new();
    server.run(transport).await
}
```

### Basic Client Example

```rust
use mcp_protocol_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<(), McpError> {
    let client = McpClient::new()
        .name("example-client")
        .version("1.0.0")
        .build();
    
    let transport = StdioClientTransport::new();
    client.connect(transport).await?;
    
    client.initialize().await?;
    
    let tools = client.list_tools().await?;
    println!("Available tools: {:?}", tools);
    
    if !tools.is_empty() {
        let result = client.call_tool(
            &tools[0].name,
            json!({ "message": "Hello, World!" })
        ).await?;
        
        println!("Tool result: {:?}", result);
    }
    
    Ok(())
}
```

This completes the API reference documentation for the MCP Protocol SDK. All public types, methods, and usage patterns are documented with examples. 📚
