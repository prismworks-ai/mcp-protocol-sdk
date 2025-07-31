# 🚀 Quick Start Guide

Get up and running with the **unified MCP Protocol SDK** in just a few minutes!

## Prerequisites

- Rust 1.75+ installed
- Basic familiarity with async Rust

## Installation

Add the unified SDK to your `Cargo.toml`:

```toml
[dependencies]
mcp-protocol-sdk = "0.3.0"

# Choose only the features you need:
# mcp-protocol-sdk = { version = "0.3.0", features = ["stdio", "http", "websocket"] }
```

### 🎉 Migration from Separate Crates

If you were using the old separate crates, migration is simple:

**Before (v0.2.x):**
```toml
[dependencies]
mcp-protocol-client = "0.1.0"
mcp-protocol-server = "0.1.0" 
mcp-protocol-types = "0.1.0"
```

**After (v0.3.0):**
```toml
[dependencies] 
mcp-protocol-sdk = "0.3.0"  # Everything unified!
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
use serde_json::{json, Value};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create a new MCP server
    let mut server = McpServer::new("calculator-server", "1.0.0");
    
    // Add a calculator tool
    let calc_tool = Tool::new(
        "calculate",
        "Perform basic arithmetic calculations",
    )
    .with_parameter("expression", "Mathematical expression to evaluate", true)
    .with_parameter("precision", "Number of decimal places", false);
    
    server.add_tool(calc_tool);
    
    // Handle tool calls
    server.set_tool_handler("calculate", |params: HashMap<String, Value>| async move {
        let expression = params.get("expression")
            .and_then(|v| v.as_str())
            .ok_or("Missing expression parameter")?;
            
        // Simple calculator logic (use a real math parser in production)
        let result = match expression {
            expr if expr.contains('+') => {
                let parts: Vec<&str> = expr.split('+').collect();
                if parts.len() == 2 {
                    let a: f64 = parts[0].trim().parse()?;
                    let b: f64 = parts[1].trim().parse()?;
                    a + b
                } else {
                    return Err("Invalid expression".into());
                }
            }
            // Add more operations as needed
            _ => return Err("Unsupported operation".into()),
        };
        
        Ok(json!({
            "result": result,
            "expression": expression
        }))
    });
    
    // Start the server on STDIO (for Claude Desktop)
    let transport = StdioServerTransport::new();
    server.run(transport).await?;
    
    Ok(())
}
```

## 5-Minute Client Example

Create an MCP client to connect to servers:

```rust
use mcp_protocol_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Connect to an MCP server
    let client = McpClient::new()
        .with_name("my-client")
        .with_version("1.0.0")
        .build();
    
    // Connect via STDIO to a server process
    let transport = StdioClientTransport::new();
    client.connect(transport).await?;
    
    // Initialize the connection
    client.initialize().await?;
    
    // List available tools
    let tools = client.list_tools().await?;
    println!("Available tools: {:#?}", tools);
    
    // Call a tool
    if let Some(tool) = tools.first() {
        let result = client.call_tool(
            &tool.name,
            serde_json::json!({
                "expression": "10 + 5"
            })
        ).await?;
        
        println!("Tool result: {:#?}", result);
    }
    
    Ok(())
}
```

## Next Steps

1. **For Servers**: Read the [Server Development Guide](./servers/README.md)
2. **For Clients**: Read the [Client Development Guide](./clients/README.md)
3. **Integration**: Add your server to [Claude Desktop](./integrations/claude-desktop.md)

## 🎉 Unified Architecture Benefits

With v0.3.0, you get all MCP functionality in one clean package:

### **Before: Multiple Dependencies**
```toml
[dependencies]
mcp-protocol-client = "0.1.0"    # 298 downloads
mcp-protocol-server = "0.1.0"    # 296 downloads  
mcp-protocol-types = "0.1.0"     # Unknown downloads
```

### **After: Unified SDK**
```toml
[dependencies]
mcp-protocol-sdk = "0.3.0"       # 🎯 Everything included!
```

**✅ Benefits:**
- **Faster Builds**: Fewer dependencies to compile
- **Simpler Imports**: One consistent API surface
- **Better Maintenance**: Single version to track
- **Zero Overhead**: Same performance, cleaner code

## Transport Options

The unified SDK supports multiple transport layers:

- **STDIO**: Perfect for Claude Desktop integration
- **HTTP**: RESTful API integration  
- **WebSocket**: Real-time bidirectional communication

Choose the right transport for your use case in our [transport guide](./servers/transports.md).

## Feature Flags

Minimize your binary size by choosing only the features you need:

```toml
[dependencies]
mcp-protocol-sdk = { 
    version = "0.3.0", 
    default-features = false,
    features = ["stdio"]  # Only STDIO transport
}
```

Available features:
- `stdio` - STDIO transport support
- `http` - HTTP transport support
- `websocket` - WebSocket transport support
- `validation` - Enhanced validation (recommended)
- `tracing-subscriber` - Built-in logging

## Examples

Check out more examples in the [examples directory](../examples/) for:
- Database servers
- File system servers  
- AI tool servers
- HTTP API servers
- WebSocket chat servers

Happy coding with the unified SDK! 🦀✨
