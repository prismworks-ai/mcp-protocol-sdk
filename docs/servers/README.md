# 📡 MCP Server Development Guide

Build powerful MCP servers that extend AI capabilities with custom tools and resources.

## What is an MCP Server?

An MCP server provides tools, resources, and prompts that AI models can use to enhance their capabilities. Your server acts as a bridge between the AI and external systems like databases, APIs, file systems, or any custom functionality.

## Core Concepts

### Tools
Functions that AI models can call to perform actions:
- Execute code
- Query databases  
- Call APIs
- Manipulate files
- Perform calculations

### Resources
Static or dynamic content that AI models can access:
- Files and documents
- Database records
- API responses
- Generated content

### Prompts
Reusable prompt templates with parameters:
- Instruction templates
- Few-shot examples
- Domain-specific prompts

## Quick Server Setup

### 1. Basic Server Structure

```rust
use mcp_protocol_sdk::prelude::*;
use serde_json::{json, Value};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize server
    let mut server = McpServer::new("my-server", "1.0.0")
        .with_description("My awesome MCP server");
    
    // Add capabilities
    setup_tools(&mut server).await?;
    setup_resources(&mut server).await?;
    setup_prompts(&mut server).await?;
    
    // Choose transport (STDIO for Claude Desktop)
    let transport = StdioServerTransport::new();
    
    // Start serving
    server.run(transport).await?;
    Ok(())
}
```

### 2. Adding Tools

```rust
async fn setup_tools(server: &mut McpServer) -> Result<(), Box<dyn std::error::Error>> {
    // Define a tool
    let file_reader = Tool::new(
        "read_file",
        "Read contents of a file"
    )
    .with_parameter("path", "File path to read", true)
    .with_parameter("encoding", "File encoding (default: utf-8)", false);
    
    server.add_tool(file_reader);
    
    // Handle tool calls
    server.set_tool_handler("read_file", |params: HashMap<String, Value>| async move {
        let path = params.get("path")
            .and_then(|v| v.as_str())
            .ok_or("Missing path parameter")?;
            
        let encoding = params.get("encoding")
            .and_then(|v| v.as_str())
            .unwrap_or("utf-8");
        
        // Read file (add proper error handling)
        let content = tokio::fs::read_to_string(path).await
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        Ok(json!({
            "content": content,
            "path": path,
            "encoding": encoding,
            "size": content.len()
        }))
    });
    
    Ok(())
}
```

### 3. Adding Resources

```rust
async fn setup_resources(server: &mut McpServer) -> Result<(), Box<dyn std::error::Error>> {
    // Static resource
    let config_resource = Resource::new(
        "file://config.json",
        "Application configuration",
        "application/json"
    );
    server.add_resource(config_resource);
    
    // Dynamic resource handler
    server.set_resource_handler("file://", |uri: &str| async move {
        let path = uri.strip_prefix("file://").unwrap_or(uri);
        
        let content = tokio::fs::read_to_string(path).await
            .map_err(|e| format!("Failed to read resource: {}", e))?;
        
        Ok(ResourceContent::Text(content))
    });
    
    Ok(())
}
```

### 4. Adding Prompts

```rust
async fn setup_prompts(server: &mut McpServer) -> Result<(), Box<dyn std::error::Error>> {
    let code_review = Prompt::new(
        "code_review",
        "Code review prompt template"
    )
    .with_parameter("code", "Code to review", true)
    .with_parameter("language", "Programming language", true)
    .with_parameter("style", "Review style (thorough|quick)", false);
    
    server.add_prompt(code_review);
    
    server.set_prompt_handler("code_review", |params: HashMap<String, Value>| async move {
        let code = params.get("code").and_then(|v| v.as_str()).unwrap_or("");
        let language = params.get("language").and_then(|v| v.as_str()).unwrap_or("unknown");
        let style = params.get("style").and_then(|v| v.as_str()).unwrap_or("thorough");
        
        let prompt = format!(
            "Please review this {} code with a {} approach:\n\n```{}\n{}\n```\n\nFocus on:",
            language, style, language, code
        );
        
        Ok(PromptMessage::new("user", prompt))
    });
    
    Ok(())
}
```

## Transport Selection

### STDIO Transport (Recommended for Claude Desktop)

```rust
use mcp_protocol_sdk::transport::StdioServerTransport;

let transport = StdioServerTransport::new();
server.run(transport).await?;
```

**Best for:**
- Claude Desktop integration
- Command-line tools
- Process-based isolation

### HTTP Transport

```rust
use mcp_protocol_sdk::transport::HttpServerTransport;

let transport = HttpServerTransport::new("127.0.0.1:3000")
    .with_cors_enabled(true)
    .with_timeout(Duration::from_secs(30));
    
server.run(transport).await?;
```

**Best for:**
- Web applications
- REST API integration
- Service-oriented architectures

### WebSocket Transport

```rust
use mcp_protocol_sdk::transport::WebSocketServerTransport;

let transport = WebSocketServerTransport::new("127.0.0.1:8080")
    .with_heartbeat_interval(Duration::from_secs(30));
    
server.run(transport).await?;
```

**Best for:**
- Real-time applications
- Bidirectional communication
- High-frequency interactions

## Advanced Features

### Error Handling

```rust
server.set_tool_handler("risky_operation", |params| async move {
    match perform_operation(&params).await {
        Ok(result) => Ok(json!({"status": "success", "data": result})),
        Err(e) => {
            // Log error internally
            tracing::error!("Operation failed: {}", e);
            
            // Return user-friendly error
            Err(format!("Operation failed: {}", e.user_message()))
        }
    }
});
```

### Logging & Monitoring

```rust
use tracing::{info, warn, error};

server.set_logging_handler(|level, message, data| async move {
    match level {
        LoggingLevel::Error => error!("{}: {:?}", message, data),
        LoggingLevel::Warning => warn!("{}: {:?}", message, data),
        LoggingLevel::Info => info!("{}: {:?}", message, data),
        _ => {}
    }
});
```

### Authentication & Authorization

```rust
server.set_auth_handler(|request| async move {
    // Validate API key, JWT, etc.
    let auth_header = request.headers().get("Authorization")
        .ok_or("Missing authorization")?;
    
    validate_token(auth_header).await?;
    Ok(())
});
```

## Production Considerations

### Performance
- Use connection pooling for databases
- Implement caching for expensive operations
- Use streaming for large responses
- Profile your tool handlers

### Security
- Validate all inputs thoroughly
- Implement rate limiting
- Use least-privilege access
- Audit tool capabilities

### Reliability
- Handle network failures gracefully
- Implement retry mechanisms
- Use circuit breakers for external services
- Monitor and alert on errors

## Real-World Examples

Check out these complete server examples:

1. **[Database Server](../examples/database_server.rs)** - SQL query execution
2. **[File System Server](../examples/simple_server.rs)** - File operations
3. **[HTTP API Server](../examples/http_server.rs)** - RESTful interface
4. **[WebSocket Chat Server](../examples/websocket_server.rs)** - Real-time communication

## Next Steps

1. **Deploy your server**: See [deployment guide](./deployment.md)
2. **Add to Claude Desktop**: Follow [integration guide](../integrations/claude-desktop.md)
3. **Testing**: Read [testing strategies](../advanced/testing.md)
4. **Monitoring**: Set up [observability](../advanced/monitoring.md)

Happy server building! 🚀
