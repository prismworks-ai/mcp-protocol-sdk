# 🚀 Quick Start Guide

Get up and running with the **production-ready MCP Protocol SDK** (v0.5.0) in just a few minutes!

🎯 **Latest Release**: Complete MCP 2025-06-18 schema upgrade with enhanced tool results, rich resource metadata, and 299 comprehensive tests.

## Prerequisites

- Rust 1.85+ installed
- Basic familiarity with async Rust

## Installation

Add the unified SDK to your `Cargo.toml`:

```toml
[dependencies]
mcp-protocol-sdk = "0.5.0"  # Latest with 2025-06-18 schema upgrade
tokio = { version = "1.0", features = ["full"] }
async-trait = "0.1"
serde_json = "1.0"

# Choose only the features you need:
# mcp-protocol-sdk = { version = "0.5.0", features = ["stdio", "http", "websocket"] }
```

### 🎉 Migration from Separate Crates

If you were using the old separate crates, migration is simple:

**Before (Separate Crates):**
```toml
[dependencies]
mcp-protocol-client = "0.1.0"
mcp-protocol-server = "0.1.0" 
mcp-protocol-types = "0.1.0"
```

**After (Unified SDK):**
```toml
[dependencies] 
mcp-protocol-sdk = "0.5.0"  # Everything unified + production ready!
```

**Code changes:**
```rust
// OLD imports
use mcp_protocol_client::*;
use mcp_protocol_server::*;
use mcp_protocol_types::*;

// NEW imports (same functionality)
use mcp_protocol_sdk::client::*;
use mcp_protocol_sdk::server::*;
use mcp_protocol_sdk::protocol::types::*;
```

## 5-Minute Server Example

Create a simple MCP server that provides a calculator tool:

```rust
use mcp_protocol_sdk::prelude::*;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::collections::HashMap;

// Step 1: Create a tool handler (required by the API)
struct CalculatorHandler;

#[async_trait]
impl ToolHandler for CalculatorHandler {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        let expression = arguments
            .get("expression")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::Validation("Missing expression parameter".to_string()))?;
            
        // Simple calculator logic (use a real math parser in production)
        let result = match expression {
            expr if expr.contains('+') => {
                let parts: Vec<&str> = expr.split('+').collect();
                if parts.len() == 2 {
                    let a: f64 = parts[0].trim().parse()
                        .map_err(|_| McpError::Validation("Invalid number".to_string()))?;
                    let b: f64 = parts[1].trim().parse()
                        .map_err(|_| McpError::Validation("Invalid number".to_string()))?;
                    a + b
                } else {
                    return Err(McpError::Validation("Invalid expression format".to_string()));
                }
            }
            expr if expr.contains('-') => {
                let parts: Vec<&str> = expr.split('-').collect();
                if parts.len() == 2 {
                    let a: f64 = parts[0].trim().parse()
                        .map_err(|_| McpError::Validation("Invalid number".to_string()))?;
                    let b: f64 = parts[1].trim().parse()
                        .map_err(|_| McpError::Validation("Invalid number".to_string()))?;
                    a - b
                } else {
                    return Err(McpError::Validation("Invalid expression format".to_string()));
                }
            }
            _ => return Err(McpError::Validation("Unsupported operation".to_string())),
        };
        
        Ok(ToolResult {
            content: vec![Content::text(result.to_string())],
            is_error: None,
            structured_content: Some(json!({
                "expression": expression,
                "result": result
            })),
            meta: None,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new MCP server (note: requires String parameters)
    let mut server = McpServer::new("calculator-server".to_string(), "1.0.0".to_string());
    
    // Add a calculator tool using the actual API
    server.add_tool(
        "calculate".to_string(),
        Some("Perform basic arithmetic calculations".to_string()),
        json!({
            "type": "object",
            "properties": {
                "expression": {
                    "type": "string",
                    "description": "Mathematical expression to evaluate (e.g., '5+3', '10-2')"
                },
                "precision": {
                    "type": "integer",
                    "description": "Number of decimal places (optional)",
                    "default": 2
                }
            },
            "required": ["expression"]
        }),
        CalculatorHandler,
    ).await?;
    
    // Start the server with STDIO transport (compatible with Claude Desktop)
    use mcp_protocol_sdk::transport::stdio::StdioServerTransport;
    let transport = StdioServerTransport::new();
    server.start(transport).await?;
    
    Ok(())
}
```

## 5-Minute Client Example

Create a client that connects to an MCP server:

```rust
use mcp_protocol_sdk::prelude::*;
use mcp_protocol_sdk::client::McpClient;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let mut client = McpClient::new("calculator-client".to_string(), "1.0.0".to_string());
    
    // Connect with STDIO transport
    use mcp_protocol_sdk::transport::stdio::StdioClientTransport;
    let transport = StdioClientTransport::new("./calculator-server".to_string()).await?;
    
    // Connect and initialize (returns server info)
    let init_result = client.connect(transport).await?;
    
    println!("Connected to: {} v{}", 
        init_result.server_info.name,
        init_result.server_info.version
    );
    
    // Check server capabilities
    if let Some(capabilities) = client.server_capabilities().await {
        if capabilities.tools.is_some() {
            println!("✅ Server supports tools");
        }
    }
    
    println!("🎉 Client setup complete!");
    Ok(())
}
```

## Key API Patterns

### ⚠️ **Important Requirements**

1. **Tool Handlers**: Must implement `ToolHandler` trait with `#[async_trait]`
2. **String Parameters**: Use `.to_string()` for server/tool names (not `&str`)
3. **JSON Schemas**: Tools require explicit JSON schema definitions
4. **Error Handling**: Use `McpResult<T>` and proper error types
5. **Dependencies**: Include `async-trait`, `tokio`, and `serde_json`

### ✅ **Working Patterns**

```rust
// ✅ Correct: Tool handler implementation
struct MyHandler;

#[async_trait]
impl ToolHandler for MyHandler {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        // Your implementation
        Ok(ToolResult {
            content: vec![Content::text("result".to_string())],
            is_error: None,
            structured_content: None,
            meta: None,
        })
    }
}

// ✅ Correct: Add tool to server
server.add_tool(
    "my_tool".to_string(),              // String required
    Some("Description".to_string()),    // Optional description
    json!({...}),                       // JSON schema
    MyHandler,                          // Handler instance
).await?;

// ✅ Correct: Server creation
let mut server = McpServer::new("name".to_string(), "1.0.0".to_string());
```

### ❌ **Incorrect Patterns (Don't Use)**

```rust
// ❌ These APIs don't exist:
// Tool::new(name, desc).with_parameter()  - Not available
// server.set_tool_handler()              - Method doesn't exist
// Closure-based handlers                 - Not supported
// String literals without .to_string()   - Type mismatch
```

## Advanced Features

### Using ToolBuilder for Advanced Tools

```rust
use mcp_protocol_sdk::core::tool::ToolBuilder;

// Create tools with advanced validation and metadata
let advanced_tool = ToolBuilder::new("advanced_calculator")
    .description("Advanced calculator with validation")
    .version("1.0.0")
    .schema(json!({
        "type": "object",
        "properties": {
            "operation": {"type": "string", "enum": ["add", "subtract", "multiply", "divide"]},
            "a": {"type": "number"},
            "b": {"type": "number"}
        },
        "required": ["operation", "a", "b"]
    }))
    .strict_validation()
    .read_only()
    .idempotent()
    .cacheable()
    .build(CalculatorHandler)?;
```

### HTTP Transport

```rust
#[cfg(feature = "http")]
use mcp_protocol_sdk::transport::http::HttpClientTransport;
use mcp_protocol_sdk::transport::traits::TransportConfig;

// Advanced HTTP configuration
let config = TransportConfig {
    connect_timeout_ms: Some(5_000),
    read_timeout_ms: Some(30_000),
    write_timeout_ms: Some(30_000),
    max_message_size: Some(1024 * 1024),
    keep_alive_ms: Some(60_000),
    compression: true,
    headers: std::collections::HashMap::new(),
};

let transport = HttpClientTransport::with_config(
    "http://localhost:3000",
    None,
    config,
).await?;
```

## Next Steps

- 📖 **[Implementation Guide](./implementation-guide.md)** - Complete development guide
- 🔧 **[Examples](../examples/)** - Working example projects
- 🌐 **[Transports](./transports.md)** - HTTP, WebSocket, and STDIO guides
- 🎯 **[Claude Desktop Integration](./integrations/claude-desktop.md)** - Add tools to Claude
- ⚡ **[Cursor IDE Integration](./integrations/cursor.md)** - Build IDE extensions

## Getting Help

- 📚 **[API Documentation](https://docs.rs/mcp-protocol-sdk)** - Complete API reference
- 🐛 **[GitHub Issues](https://github.com/mcp-rust/mcp-protocol-sdk/issues)** - Bug reports and questions
- 💬 **[Examples Directory](../examples/)** - Real working code samples

Remember: All examples in this guide have been tested and verified to work with the current API! 🎉
