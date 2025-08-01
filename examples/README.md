# MCP Protocol SDK Examples

This directory contains examples demonstrating various aspects of the MCP Protocol SDK.

## Organization

### 📁 [Client Examples](client/)
Examples showing how to build MCP clients:
- Basic client with stdio transport
- HTTP and WebSocket clients
- Advanced client features with retry logic and health checks

### 📁 [Server Examples](server/)
Examples showing how to build MCP servers:
- Simple and echo servers
- Database server with persistence
- HTTP and WebSocket servers
- Resource and tool handlers

### 📁 [Utilities](utilities/)
Utility tools and benchmarks:
- Transport performance benchmarking
- Development and testing tools
- Performance monitoring utilities

## Quick Start

### Simple Echo Server & Client

**1. Start the echo server:**
```bash
cargo run --example echo_server --features "stdio,tracing-subscriber"
```

**2. In another terminal, run the client:**
```bash
cargo run --example basic_client --features stdio
```

### HTTP Server & Client

**1. Start the HTTP server:**
```bash
cargo run --example http_server --features http
```

**2. In another terminal, run the HTTP client:**
```bash
cargo run --example http_client --features http
```

### WebSocket Real-time Communication

**1. Start the WebSocket server:**
```bash
cargo run --example websocket_server --features websocket
```

**2. Connect with WebSocket client:**
```bash
cargo run --example websocket_client --features websocket
```

## Core Patterns

### Basic Tool Implementation (2025-06-18 Schema)
```rust
use mcp_protocol_sdk::{
    core::tool::ToolHandler,
    protocol::types::{ToolResult, Content},
    core::error::McpError,
};
use async_trait::async_trait;
use std::collections::HashMap;
use serde_json::Value;

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
            structured_content: None,  // New in 2025-06-18
            meta: None,                // New in 2025-06-18
        })
    }
}
```

### Resource Handler Implementation
```rust
use mcp_protocol_sdk::{
    core::resource::ResourceHandler,
    protocol::types::{ResourceInfo, ResourceContents},
};

struct FileResourceHandler;

#[async_trait]
impl ResourceHandler for FileResourceHandler {
    async fn read(&self, uri: &str, _params: &HashMap<String, String>) -> McpResult<Vec<ResourceContents>> {
        if uri.starts_with("file://") {
            let path = &uri[7..];
            match std::fs::read_to_string(path) {
                Ok(content) => Ok(vec![ResourceContents::Text {
                    uri: uri.to_string(),
                    mime_type: Some("text/plain".to_string()),
                    text: content,
                    meta: None,  // New in 2025-06-18
                }]),
                Err(_) => Err(McpError::ResourceNotFound(uri.to_string())),
            }
        } else {
            Err(McpError::ResourceNotFound(uri.to_string()))
        }
    }

    async fn list(&self) -> McpResult<Vec<ResourceInfo>> {
        Ok(vec![ResourceInfo {
            uri: "file://example.txt".to_string(),
            name: "Example File".to_string(),  // Direct String in 2025-06-18
            description: Some("Example text file".to_string()),
            mime_type: Some("text/plain".to_string()),
            annotations: None,
            size: None,
            title: None,  // New in 2025-06-18
            meta: None,   // New in 2025-06-18
        }])
    }
}
```

### Client Connection Pattern
```rust
use mcp_protocol_sdk::{
    client::{McpClient, ClientSession},
    transport::stdio::StdioClientTransport,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = McpClient::new("example-client".to_string(), "1.0.0".to_string());
    let session = ClientSession::new(client);
    
    let transport = StdioClientTransport::new("./server-command".to_string()).await?;
    let init_result = session.connect(transport).await?;
    
    println!("Connected to: {} v{}", 
        init_result.server_info.name, 
        init_result.server_info.version
    );
    
    let client = session.client();
    let client_guard = client.lock().await;
    
    // List available tools
    let tools = client_guard.list_tools().await?;
    println!("Available tools: {}", tools.tools.len());
    
    // Call a tool
    let mut args = HashMap::new();
    args.insert("message".to_string(), json!("Hello World"));
    
    let result = client_guard.call_tool("echo".to_string(), Some(args)).await?;
    println!("Result: {:?}", result);
    
    Ok(())
}
```

## Features

Most examples require specific features to be enabled:

- `stdio` - Standard I/O transport (default)
- `http` - HTTP transport support
- `websocket` - WebSocket transport support
- `tracing-subscriber` - Logging support
- `chrono` - Date/time utilities
- `fastrand` - Random number generation
- `validation` - JSON schema validation

## Transport Comparison

| Transport | Use Case | Latency | Throughput | Real-time |
|-----------|----------|---------|------------|----------|
| **STDIO** | CLI tools, local processes | Low | High | No |
| **HTTP** | Web APIs, REST services | Medium | Medium | SSE only |
| **WebSocket** | Real-time apps, live data | Low | High | Yes |

See [transport_benchmark.rs](utilities/transport_benchmark.rs) for performance comparisons.

## Documentation

For detailed documentation:
- [Getting Started Guide](../docs/getting-started.md)
- [Implementation Guide](../docs/implementation-guide.md) - Complete client and server development
- [Getting Started Guide](../docs/getting-started.md) - Quick start examples
- [Examples Documentation](../docs/examples.md)
- [Architecture Guide](../docs/architecture.md)
- [Transport Guide](../docs/transports.md)

## Testing Examples

To verify all examples work correctly:

```bash
# Test all client examples
for example in basic_client http_client websocket_client advanced_http_client conservative_http_demo; do
    echo "Testing $example..."
    cargo check --example $example --features "stdio,http,websocket,tracing-subscriber,chrono,fastrand" --quiet
done

# Test all server examples  
for example in simple_server echo_server database_server http_server websocket_server; do
    echo "Testing $example..."
    cargo check --example $example --features "stdio,http,websocket,tracing-subscriber" --quiet
done

# Test utilities
cargo check --example transport_benchmark --features "http,tracing-subscriber" --quiet

echo "All examples verified!"
```

## Contributing

When adding new examples:
1. Place them in the appropriate subdirectory (client/, server/, utilities/)
2. Add the example entry to `Cargo.toml`
3. Update the relevant README files with code snippets
4. Include proper documentation and comments
5. Ensure compatibility with the current MCP 2025-06-18 specification

See [Contributing Guidelines](../docs/CONTRIBUTING.md) for more details.
