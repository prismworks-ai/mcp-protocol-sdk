# API Reference

This page provides a comprehensive API reference for MCP Rust SDK. For complete documentation with examples, visit [docs.rs/mcp-protocol-sdk](https://docs.rs/mcp-protocol-sdk).

## Core Types

### Server

#### `McpServer`

The main server struct that handles MCP requests and manages tools, resources, and prompts.

```rust
impl McpServer {
    pub fn new(name: String, version: String) -> Self;
    
    pub async fn add_tool<T>(&mut self, 
        name: String, 
        description: Option<String>, 
        schema: serde_json::Value, 
        handler: T
    ) -> Result<(), McpError>
    where T: ToolHandler + Send + Sync + 'static;
    
    pub async fn add_resource_detailed<R>(&mut self, 
        info: ResourceInfo, 
        handler: R
    ) -> Result<(), McpError>
    where R: ResourceHandler + Send + Sync + 'static;
    
    pub async fn add_prompt_detailed<P>(&mut self, 
        info: PromptInfo, 
        handler: P
    ) -> Result<(), McpError>
    where P: PromptHandler + Send + Sync + 'static;
    
    pub async fn start<T: ServerTransport + Send + 'static>(&mut self, transport: T) -> Result<(), McpError>;
    pub async fn stop(&mut self) -> Result<(), McpError>;
}
```

### Client

#### `McpClient`

The main client struct for connecting to MCP servers.

```rust
impl McpClient {
    pub fn new(name: String, version: String) -> Self;
    
    pub async fn list_tools(&self) -> Result<ListToolsResult, McpError>;
    pub async fn call_tool(&self, name: String, arguments: Option<HashMap<String, serde_json::Value>>) -> Result<ToolResult, McpError>;
    
    pub async fn list_resources(&self) -> Result<ListResourcesResult, McpError>;
    pub async fn read_resource(&self, uri: String) -> Result<ReadResourceResult, McpError>;
    
    pub async fn list_prompts(&self) -> Result<ListPromptsResult, McpError>;
    pub async fn get_prompt(&self, name: String, arguments: Option<HashMap<String, serde_json::Value>>) -> Result<GetPromptResult, McpError>;
}
```

#### `ClientSession`

Session management wrapper around `McpClient` with auto-reconnection.

```rust
impl ClientSession {
    pub fn new(client: McpClient) -> Self;
    pub fn with_config(client: McpClient, config: SessionConfig) -> Self;
    
    pub async fn connect<T: ClientTransport + Send + 'static>(&self, transport: T) -> Result<InitializeResult, McpError>;
    pub async fn disconnect(&self) -> Result<(), McpError>;
    
    pub fn client(&self) -> Arc<Mutex<McpClient>>;
    pub fn is_connected(&self) -> bool;
}
```

## Transport Types

### STDIO Transport

#### `StdioServerTransport`

```rust
impl StdioServerTransport {
    pub fn new() -> Self;
    pub fn with_config(config: StdioServerConfig) -> Self;
}
```

#### `StdioClientTransport`

```rust
impl StdioClientTransport {
    pub async fn new(command: String) -> Result<Self, McpError>;
    pub async fn with_config(command: String, config: StdioClientConfig) -> Result<Self, McpError>;
}
```

### HTTP Transport

#### `HttpServerTransport`

```rust
impl HttpServerTransport {
    pub fn new(bind_address: &str) -> Self;
    pub fn with_config(config: HttpServerConfig) -> Self;
}
```

#### `HttpClientTransport`

```rust
impl HttpClientTransport {
    pub async fn new(base_url: &str) -> Result<Self, McpError>;
    pub async fn with_config(config: HttpClientConfig) -> Result<Self, McpError>;
}
```

### WebSocket Transport

#### `WebSocketServerTransport`

```rust
impl WebSocketServerTransport {
    pub fn new(bind_address: &str) -> Self;
    pub fn with_config(config: WebSocketServerConfig) -> Self;
}
```

#### `WebSocketClientTransport`

```rust
impl WebSocketClientTransport {
    pub async fn new(url: &str) -> Result<Self, McpError>;
    pub async fn with_config(config: WebSocketClientConfig) -> Result<Self, McpError>;
}
```

## Handler Traits

### `ToolHandler`

Implement this trait to create custom tools.

```rust
#[async_trait]
pub trait ToolHandler: Send + Sync {
    async fn call(&self, arguments: HashMap<String, serde_json::Value>) -> Result<ToolResult, McpError>;
}
```

### `ResourceHandler`

Implement this trait to serve resources.

```rust
#[async_trait]
pub trait ResourceHandler: Send + Sync {
    async fn read(&self, uri: &str, params: &HashMap<String, String>) -> Result<Vec<ResourceContent>, McpError>;
    async fn list(&self) -> Result<Vec<ResourceInfo>, McpError>;
}
```

### `PromptHandler`

Implement this trait to create prompt templates.

```rust
#[async_trait]
pub trait PromptHandler: Send + Sync {
    async fn get_prompt(&self, arguments: HashMap<String, serde_json::Value>) -> Result<Vec<PromptMessage>, McpError>;
}
```

## Protocol Types

### `ToolResult`

```rust
pub struct ToolResult {
    pub content: Vec<Content>,
    pub is_error: Option<bool>,
}
```

### `Content`

```rust
pub enum Content {
    Text { text: String },
    Image { data: String, mime_type: String },
    Resource { uri: String },
}

impl Content {
    pub fn text(text: impl Into<String>) -> Self;
    pub fn image(data: impl Into<String>, mime_type: impl Into<String>) -> Self;
    pub fn resource(uri: impl Into<String>) -> Self;
}
```

### `ResourceInfo`

```rust
pub struct ResourceInfo {
    pub uri: String,
    pub name: String,
    pub description: Option<String>,
    pub mime_type: Option<String>,
}
```

### `ResourceContent`

```rust
pub struct ResourceContent {
    pub uri: String,
    pub mime_type: Option<String>,
    pub text: Option<String>,
    pub blob: Option<String>,
}
```

### `PromptInfo`

```rust
pub struct PromptInfo {
    pub name: String,
    pub description: Option<String>,
    pub arguments: Option<Vec<serde_json::Value>>,
}
```

### `PromptMessage`

```rust
pub struct PromptMessage {
    pub role: String,
    pub content: Vec<Content>,
}
```

## Configuration Types

### `SessionConfig`

```rust
pub struct SessionConfig {
    pub auto_reconnect: bool,
    pub max_reconnect_attempts: u32,
    pub reconnect_delay_ms: u64,
    pub connection_timeout_ms: u64,
    pub heartbeat_interval_ms: u64,
    pub max_concurrent_requests: usize,
    pub request_timeout_ms: u64,
}

impl Default for SessionConfig;
```

### Transport Configurations

#### `HttpServerConfig`

```rust
pub struct HttpServerConfig {
    pub bind_address: String,
    pub max_connections: usize,
    pub request_timeout_ms: u64,
    pub keep_alive_timeout_ms: u64,
    pub max_request_size: usize,
    pub cors_enabled: bool,
    pub cors_origins: Vec<String>,
    pub compression: bool,
    pub headers: HashMap<String, String>,
}
```

#### `WebSocketServerConfig`

```rust
pub struct WebSocketServerConfig {
    pub bind_address: String,
    pub max_connections: usize,
    pub max_frame_size: usize,
    pub max_message_size: usize,
    pub compression: bool,
    pub ping_interval_ms: Option<u64>,
    pub pong_timeout_ms: u64,
    pub headers: HashMap<String, String>,
}
```

## Error Types

### `McpError`

```rust
#[derive(Debug, thiserror::Error)]
pub enum McpError {
    #[error("Transport error: {0}")]
    Transport(String),
    
    #[error("Protocol error: {0}")]
    Protocol(String),
    
    #[error("Validation error: {0}")]
    ValidationError(String),
    
    #[error("Tool not found: {0}")]
    ToolNotFound(String),
    
    #[error("Resource not found: {0}")]
    ResourceNotFound(String),
    
    #[error("Tool execution error: {0}")]
    ToolExecutionError(String),
    
    #[error("Timeout")]
    Timeout,
    
    #[error("Serialization error: {0}")]
    Serialization(#[from] serde_json::Error),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
```

## Utility Functions

### URI Utilities

```rust
pub mod utils {
    pub fn parse_uri(uri: &str) -> Result<(String, String, HashMap<String, String>), McpError>;
    pub fn build_uri(scheme: &str, path: &str, params: &HashMap<String, String>) -> String;
    pub fn normalize_uri(uri: &str) -> Result<String, McpError>;
}
```

## Feature Flags

The following features are available:

- `stdio` (default) - STDIO transport support
- `http` - HTTP transport with Axum and Reqwest
- `websocket` - WebSocket transport with Tokio-Tungstenite  
- `validation` - JSON Schema validation
- `full` - All features enabled

## Examples

For complete examples, see:

- [Getting Started Guide](getting-started.md)
- [Examples Collection](examples.md)
- [Transport Guide](transports.md)
- [GitHub Repository Examples](https://github.com/rishirandhawa/mcp-protocol-sdk/tree/main/examples)

## Complete API Documentation

For the complete API documentation with all methods, types, and examples, visit:

**[docs.rs/mcp-protocol-sdk](https://docs.rs/mcp-protocol-sdk)**

This includes:
- Full API documentation with examples
- Source code links
- Dependency information
- Feature flag documentation
- Module-level documentation
