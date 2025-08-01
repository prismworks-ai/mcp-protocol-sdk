# Server Examples

This directory contains examples demonstrating how to build MCP servers using the SDK.

## Examples

### Simple Server
- **File**: `simple_server.rs`
- **Features**: `stdio`, `tracing-subscriber`
- **Description**: Basic MCP server with stdio transport and file operations

**Key Code Pattern:**
```rust
use mcp_protocol_sdk::{
    server::McpServer,
    transport::stdio::StdioServerTransport,
    core::tool::ToolHandler,
    protocol::types::{ToolResult, Content},
};
use async_trait::async_trait;

struct FileReadTool;

#[async_trait]
impl ToolHandler for FileReadTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, McpError> {
        let path = arguments.get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::ValidationError("Missing path parameter".to_string()))?;
            
        match std::fs::read_to_string(path) {
            Ok(content) => Ok(ToolResult {
                content: vec![Content::text(content)],
                is_error: None,
                structured_content: None,
                meta: None,
            }),
            Err(e) => Ok(ToolResult {
                content: vec![Content::text(format!("Error: {}", e))],
                is_error: Some(true),
                structured_content: None,
                meta: None,
            }),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("file-server".to_string(), "1.0.0".to_string());
    
    server.add_tool(
        "read_file".to_string(),
        Some("Read contents of a file".to_string()),
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "File path to read"}
            },
            "required": ["path"]
        }),
        FileReadTool,
    ).await?;
    
    let transport = StdioServerTransport::new();
    server.start(transport).await?;
    
    Ok(())
}
```

### Echo Server
- **File**: `echo_server.rs`
- **Features**: `stdio`, `tracing-subscriber`
- **Description**: Echo server that mirrors client requests

**Key Code Pattern:**
```rust
struct EchoTool;

#[async_trait]
impl ToolHandler for EchoTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, McpError> {
        let message = arguments.get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("No message provided");

        Ok(ToolResult {
            content: vec![Content::text(format!("Echo: {}", message))],
            is_error: None,
            structured_content: None,
            meta: None,
        })
    }
}
```

### Database Server
- **File**: `database_server.rs`
- **Features**: `stdio`, `tracing-subscriber`
- **Description**: Database server example with data persistence and SQL-like operations

### HTTP Server
- **File**: `http_server.rs`
- **Features**: `http`
- **Description**: MCP server using HTTP transport with REST endpoints

**Key Code Pattern:**
```rust
use mcp_protocol_sdk::{
    server::McpServer,
    transport::http::HttpServerTransport,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("http-server".to_string(), "1.0.0".to_string());
    
    // Add tools...
    
    let transport = HttpServerTransport::new("0.0.0.0:3000");
    server.start(transport).await?;
    
    println!("HTTP server running on http://localhost:3000");
    println!("API endpoint: http://localhost:3000/mcp");
    println!("SSE events: http://localhost:3000/mcp/events");
    
    tokio::signal::ctrl_c().await?;
    server.stop().await?;
    
    Ok(())
}
```

### WebSocket Server
- **File**: `websocket_server.rs`
- **Features**: `websocket`
- **Description**: MCP server using WebSocket transport for real-time communication

**Key Code Pattern:**
```rust
use mcp_protocol_sdk::{
    server::McpServer,
    transport::websocket::WebSocketServerTransport,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("websocket-server".to_string(), "1.0.0".to_string());
    
    // Add real-time tools...
    
    let transport = WebSocketServerTransport::new("0.0.0.0:8080");
    server.start(transport).await?;
    
    println!("WebSocket server running on ws://localhost:8080");
    
    tokio::signal::ctrl_c().await?;
    server.stop().await?;
    
    Ok(())
}
```

## Running Examples

```bash
# Run simple server example
cargo run --example simple_server --features "stdio,tracing-subscriber"

# Run echo server example
cargo run --example echo_server --features "stdio,tracing-subscriber"

# Run database server example
cargo run --example database_server --features "stdio,tracing-subscriber"

# Run HTTP server example
cargo run --example http_server --features http

# Run WebSocket server example
cargo run --example websocket_server --features websocket
```

## Common Server Patterns

### Adding Tools with Schema Validation
```rust
server.add_tool(
    "calculate".to_string(),
    Some("Perform mathematical calculations".to_string()),
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
    CalculatorTool,
).await?;
```

### Adding Resources
```rust
use mcp_protocol_sdk::{
    core::resource::ResourceHandler,
    protocol::types::{ResourceInfo, ResourceContents},
};

struct FileSystemHandler;

#[async_trait]
impl ResourceHandler for FileSystemHandler {
    async fn read(&self, uri: &str, _params: &HashMap<String, String>) -> McpResult<Vec<ResourceContents>> {
        if uri.starts_with("file://") {
            let path = &uri[7..];
            match std::fs::read_to_string(path) {
                Ok(content) => Ok(vec![ResourceContents::Text {
                    uri: uri.to_string(),
                    mime_type: Some("text/plain".to_string()),
                    text: content,
                    meta: None,
                }]),
                Err(e) => Err(McpError::ResourceNotFound(format!("File not found: {}", e))),
            }
        } else {
            Err(McpError::ResourceNotFound(uri.to_string()))
        }
    }
    
    async fn list(&self) -> McpResult<Vec<ResourceInfo>> {
        Ok(vec![ResourceInfo {
            uri: "file://example.txt".to_string(),
            name: "Example File".to_string(),
            description: Some("Example text file".to_string()),
            mime_type: Some("text/plain".to_string()),
            annotations: None,
            size: None,
            title: None,
            meta: None,
        }])
    }
}

// Add to server
server.add_resource_handler(FileSystemHandler).await?;
```

### Error Handling
```rust
// Always use proper error handling in tools
let result = match some_operation() {
    Ok(value) => ToolResult {
        content: vec![Content::text(format!("Success: {}", value))],
        is_error: None,
        structured_content: None,
        meta: None,
    },
    Err(e) => ToolResult {
        content: vec![Content::text(format!("Error: {}", e))],
        is_error: Some(true),
        structured_content: None,
        meta: None,
    },
};

Ok(result)
```

For more details on building MCP servers, see the [Implementation Guide](../../docs/implementation-guide.md) and [Getting Started Guide](../../docs/getting-started.md).
