# 🖥️ Claude Desktop Integration

Integrate your MCP server with Claude Desktop to provide AI with powerful custom capabilities.

## Overview

Claude Desktop can connect to MCP servers via STDIO transport, allowing Claude to:
- Use your custom tools during conversations
- Access your resources and data
- Utilize your prompt templates

## Quick Setup

### 1. Build Your Server

First, ensure your server uses STDIO transport:

```rust
use mcp_protocol_sdk::prelude::*;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::collections::HashMap;

// Tool handler example
struct MyToolHandler;

#[async_trait]
impl ToolHandler for MyToolHandler {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        // Your tool implementation
        Ok(ToolResult {
            content: vec![Content::text("Tool executed successfully".to_string())],
            is_error: None,
            structured_content: None,
            meta: None,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create server (note: requires String parameters)
    let mut server = McpServer::new("my-server".to_string(), "1.0.0".to_string());
    
    // Add your tools using the actual API
    setup_capabilities(&mut server).await?;
    
    // Use STDIO transport for Claude Desktop
    use mcp_protocol_sdk::transport::stdio::StdioServerTransport;
    let transport = StdioServerTransport::new();
    server.start(transport).await?;
    
    Ok(())
}

async fn setup_capabilities(server: &mut McpServer) -> McpResult<()> {
    // Add tools using the correct API
    server.add_tool(
        "example_tool".to_string(),
        Some("Example tool description".to_string()),
        json!({
            "type": "object",
            "properties": {
                "input": {"type": "string", "description": "Tool input"}
            },
            "required": ["input"]
        }),
        MyToolHandler,
    ).await?;
    
    Ok(())
}
```

### 2. Build and Install Your Server

```bash
# Build your server binary
cargo build --release --features stdio

# Install to a permanent location
sudo cp target/release/my-server /usr/local/bin/
# Or on macOS with Homebrew:
# cp target/release/my-server /opt/homebrew/bin/
```

### 3. Configure Claude Desktop

Add your server to Claude Desktop's configuration file:

**macOS**: `~/Library/Application Support/Claude/claude_desktop_config.json`
**Windows**: `%APPDATA%\Claude\claude_desktop_config.json`
**Linux**: `~/.config/Claude/claude_desktop_config.json`

```json
{
  "mcpServers": {
    "my-server": {
      "command": "/usr/local/bin/my-server",
      "args": []
    }
  }
}
```

### 4. Restart Claude Desktop

Restart Claude Desktop to load your server. You should see your tools available in conversations!

## Server Examples for Claude Desktop

### File System Server

Perfect for file operations:

```rust
use mcp_protocol_sdk::prelude::*;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::collections::HashMap;

// File read handler
struct FileReadHandler;

#[async_trait]
impl ToolHandler for FileReadHandler {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        let path = arguments
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::Validation("Missing path parameter".to_string()))?;
        
        match tokio::fs::read_to_string(path).await {
            Ok(content) => Ok(ToolResult {
                content: vec![Content::text(content.clone())],
                is_error: None,
                structured_content: Some(json!({
                    "content": content,
                    "path": path,
                    "size": content.len()
                })),
                meta: None,
            }),
            Err(e) => Ok(ToolResult {
                content: vec![Content::text(format!("Failed to read file: {}", e))],
                is_error: Some(true),
                structured_content: None,
                meta: None,
            }),
        }
    }
}

// File write handler
struct FileWriteHandler;

#[async_trait]
impl ToolHandler for FileWriteHandler {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        let path = arguments
            .get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::Validation("Missing path parameter".to_string()))?;
            
        let content = arguments
            .get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::Validation("Missing content parameter".to_string()))?;
        
        match tokio::fs::write(path, content).await {
            Ok(_) => Ok(ToolResult {
                content: vec![Content::text(format!("Successfully wrote {} bytes to {}", content.len(), path))],
                is_error: None,
                structured_content: Some(json!({
                    "success": true,
                    "path": path,
                    "bytes_written": content.len()
                })),
                meta: None,
            }),
            Err(e) => Ok(ToolResult {
                content: vec![Content::text(format!("Failed to write file: {}", e))],
                is_error: Some(true),
                structured_content: None,
                meta: None,
            }),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("filesystem-server".to_string(), "1.0.0".to_string());
    
    // Add read file tool
    server.add_tool(
        "read_file".to_string(),
        Some("Read a text file".to_string()),
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "File path to read"
                }
            },
            "required": ["path"]
        }),
        FileReadHandler,
    ).await?;
    
    // Add write file tool
    server.add_tool(
        "write_file".to_string(),
        Some("Write content to a file".to_string()),
        json!({
            "type": "object",
            "properties": {
                "path": {
                    "type": "string",
                    "description": "File path to write"
                },
                "content": {
                    "type": "string",
                    "description": "Content to write"
                }
            },
            "required": ["path", "content"]
        }),
        FileWriteHandler,
    ).await?;
    
    use mcp_protocol_sdk::transport::stdio::StdioServerTransport;
    let transport = StdioServerTransport::new();
    server.start(transport).await?;
    
    Ok(())
}
```

**Configuration:**
```json
{
  "mcpServers": {
    "filesystem": {
      "command": "/usr/local/bin/filesystem-server",
      "args": []
    }
  }
}
```

### Database Query Server

Access your databases from Claude:

```rust
use mcp_protocol_sdk::prelude::*;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::collections::HashMap;

// SQL query handler (simplified - use proper database driver in production)
struct SqlQueryHandler;

#[async_trait]
impl ToolHandler for SqlQueryHandler {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        let query = arguments
            .get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::Validation("Missing query parameter".to_string()))?;
        
        // Simulate database query (implement actual database logic)
        let simulated_result = if query.trim().to_lowercase().starts_with("select") {
            json!({
                "rows": [
                    {"id": 1, "name": "Example", "value": 42},
                    {"id": 2, "name": "Another", "value": 84}
                ],
                "row_count": 2
            })
        } else {
            json!({
                "affected_rows": 1,
                "message": "Query executed successfully"
            })
        };
        
        Ok(ToolResult {
            content: vec![Content::text(format!("Query executed: {}", query))],
            is_error: None,
            structured_content: Some(simulated_result),
            meta: None,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("database-server".to_string(), "1.0.0".to_string());
    
    // Add SQL query tool
    server.add_tool(
        "execute_sql".to_string(),
        Some("Execute SQL query".to_string()),
        json!({
            "type": "object",
            "properties": {
                "query": {
                    "type": "string",
                    "description": "SQL query to execute"
                }
            },
            "required": ["query"]
        }),
        SqlQueryHandler,
    ).await?;
    
    use mcp_protocol_sdk::transport::stdio::StdioServerTransport;
    let transport = StdioServerTransport::new();
    server.start(transport).await?;
    
    Ok(())
}
```

**Configuration:**
```json
{
  "mcpServers": {
    "database": {
      "command": "/usr/local/bin/database-server",
      "args": []
    }
  }
}
```

### API Integration Server

Connect Claude to external APIs:

```rust
use mcp_protocol_sdk::prelude::*;
use async_trait::async_trait;
use serde_json::{json, Value};
use std::collections::HashMap;

// Weather API handler (simplified example)
struct WeatherHandler;

#[async_trait]
impl ToolHandler for WeatherHandler {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        let location = arguments
            .get("location")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::Validation("Missing location parameter".to_string()))?;
        
        // Simulate weather API call (implement actual API integration)
        let weather_data = json!({
            "location": location,
            "temperature": 22,
            "condition": "Sunny",
            "humidity": 65,
            "wind_speed": 10
        });
        
        Ok(ToolResult {
            content: vec![Content::text(
                format!("Weather in {}: 22°C, Sunny, 65% humidity, 10 km/h wind", location)
            )],
            is_error: None,
            structured_content: Some(weather_data),
            meta: None,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("weather-server".to_string(), "1.0.0".to_string());
    
    // Add weather tool
    server.add_tool(
        "get_weather".to_string(),
        Some("Get current weather for a location".to_string()),
        json!({
            "type": "object",
            "properties": {
                "location": {
                    "type": "string",
                    "description": "City or location name"
                }
            },
            "required": ["location"]
        }),
        WeatherHandler,
    ).await?;
    
    use mcp_protocol_sdk::transport::stdio::StdioServerTransport;
    let transport = StdioServerTransport::new();
    server.start(transport).await?;
    
    Ok(())
}
```

**Configuration:**
```json
{
  "mcpServers": {
    "weather": {
      "command": "/usr/local/bin/weather-server",
      "args": []
    }
  }
}
```

## Advanced Configuration

### Multiple Servers

You can run multiple MCP servers simultaneously:

```json
{
  "mcpServers": {
    "filesystem": {
      "command": "/usr/local/bin/filesystem-server",
      "args": []
    },
    "database": {
      "command": "/usr/local/bin/database-server",
      "args": ["--database", "production"]
    },
    "weather": {
      "command": "/usr/local/bin/weather-server",
      "args": ["--api-key", "your-api-key"]
    }
  }
}
```

### Server with Arguments

Pass configuration arguments to your server:

```rust
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    // Parse command line arguments
    let api_key = args.iter()
        .position(|x| x == "--api-key")
        .and_then(|i| args.get(i + 1))
        .map(|s| s.as_str())
        .unwrap_or("default-key");
    
    let mut server = McpServer::new("configurable-server".to_string(), "1.0.0".to_string());
    
    // Use configuration in your server setup
    println!("Starting server with API key: {}", api_key);
    
    // ... rest of server setup
    
    Ok(())
}
```

### Environment Variables

Use environment variables for sensitive configuration:

```rust
use std::env;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "sqlite:///tmp/default.db".to_string());
    
    let api_key = env::var("API_KEY")
        .unwrap_or_else(|_| "default-key".to_string());
    
    let mut server = McpServer::new("env-server".to_string(), "1.0.0".to_string());
    
    // Use environment configuration
    println!("Connecting to database: {}", database_url);
    
    // ... server setup with environment config
    
    Ok(())
}
```

## Best Practices

### 1. Error Handling

Always provide helpful error messages:

```rust
#[async_trait]
impl ToolHandler for SafeToolHandler {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        match self.process_arguments(&arguments) {
            Ok(result) => Ok(ToolResult {
                content: vec![Content::text(result)],
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
```

### 2. Input Validation

Validate all inputs thoroughly:

```rust
fn validate_file_path(path: &str) -> Result<(), String> {
    if path.is_empty() {
        return Err("Path cannot be empty".to_string());
    }
    
    if path.contains("..") {
        return Err("Path traversal not allowed".to_string());
    }
    
    Ok(())
}
```

### 3. Resource Cleanup

Ensure proper resource cleanup:

```rust
#[async_trait]
impl ToolHandler for ResourceHandler {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        let _guard = self.acquire_resource().await?;
        
        // Resource is automatically cleaned up when _guard is dropped
        let result = self.perform_operation(&arguments).await?;
        
        Ok(result)
    }
}
```

### 4. Logging and Debugging

Add proper logging for troubleshooting:

```rust
use tracing::{info, error, debug};

#[async_trait]
impl ToolHandler for LoggingHandler {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        debug!("Tool called with arguments: {:?}", arguments);
        
        match self.execute(&arguments).await {
            Ok(result) => {
                info!("Tool executed successfully");
                Ok(result)
            }
            Err(e) => {
                error!("Tool execution failed: {}", e);
                Err(e)
            }
        }
    }
}
```

## Troubleshooting

### Common Issues

1. **Server not appearing in Claude Desktop**:
   - Check the configuration file path
   - Verify the binary path is correct
   - Restart Claude Desktop after configuration changes

2. **Tools not working**:
   - Check server logs for errors
   - Verify JSON schema validation
   - Ensure proper error handling in tool implementations

3. **Performance issues**:
   - Use async I/O for all blocking operations
   - Implement proper timeouts
   - Consider connection pooling for database access

### Debugging Configuration

Enable detailed logging in your server:

```toml
[dependencies]
mcp-protocol-sdk = { version = "0.5.0", features = ["stdio", "tracing-subscriber"] }
tracing = "0.1"
tracing-subscriber = "0.3"
```

```rust
use tracing_subscriber;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    // ... rest of server setup
    
    Ok(())
}
```

## Next Steps

- [🚀 Getting Started](../getting-started.md) - Basic server development
- [📖 Implementation Guide](../implementation-guide.md) - Complete development guide
- [🔧 Examples](../../examples/) - Working example projects
- [⚡ Cursor Integration](./cursor.md) - IDE integration guide

## Resources

- [Claude Desktop Documentation](https://docs.anthropic.com/claude/docs/desktop-app)
- [MCP Protocol Specification](https://modelcontextprotocol.io/)
- [Example Servers Repository](../../examples/)

Your MCP server will seamlessly integrate with Claude Desktop, giving Claude powerful new capabilities! 🎉
