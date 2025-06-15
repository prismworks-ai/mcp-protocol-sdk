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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("my-server", "1.0.0");
    
    // Add your tools, resources, prompts...
    setup_capabilities(&mut server).await?;
    
    // Use STDIO transport for Claude Desktop
    let transport = StdioServerTransport::new();
    server.run(transport).await?;
    
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
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("filesystem-server", "1.0.0")
        .with_description("File system operations for Claude");
    
    // Read file tool
    let read_tool = Tool::new("read_file", "Read a text file")
        .with_parameter("path", "File path to read", true);
    server.add_tool(read_tool);
    
    server.set_tool_handler("read_file", |params: HashMap<String, Value>| async move {
        let path = params.get("path")
            .and_then(|v| v.as_str())
            .ok_or("Missing path parameter")?;
        
        let content = tokio::fs::read_to_string(path).await
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        Ok(json!({
            "content": content,
            "path": path,
            "size": content.len()
        }))
    });
    
    // Write file tool
    let write_tool = Tool::new("write_file", "Write content to a file")
        .with_parameter("path", "File path to write", true)
        .with_parameter("content", "Content to write", true);
    server.add_tool(write_tool);
    
    server.set_tool_handler("write_file", |params: HashMap<String, Value>| async move {
        let path = params.get("path")
            .and_then(|v| v.as_str())
            .ok_or("Missing path parameter")?;
            
        let content = params.get("content")
            .and_then(|v| v.as_str())
            .ok_or("Missing content parameter")?;
        
        tokio::fs::write(path, content).await
            .map_err(|e| format!("Failed to write file: {}", e))?;
        
        Ok(json!({
            "success": true,
            "path": path,
            "bytes_written": content.len()
        }))
    });
    
    let transport = StdioServerTransport::new();
    server.run(transport).await?;
    
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

For database interactions:

```rust
use mcp_protocol_sdk::prelude::*;
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("database-server", "1.0.0")
        .with_description("Database query interface for Claude");
    
    // SQL query tool
    let query_tool = Tool::new("execute_sql", "Execute a SQL query")
        .with_parameter("query", "SQL query to execute", true)
        .with_parameter("database", "Database name", false);
    server.add_tool(query_tool);
    
    server.set_tool_handler("execute_sql", |params: HashMap<String, Value>| async move {
        let query = params.get("query")
            .and_then(|v| v.as_str())
            .ok_or("Missing query parameter")?;
            
        let database = params.get("database")
            .and_then(|v| v.as_str())
            .unwrap_or("default");
        
        // Execute query (implement your database logic)
        let results = execute_database_query(database, query).await?;
        
        Ok(json!({
            "results": results,
            "query": query,
            "database": database,
            "row_count": results.len()
        }))
    });
    
    let transport = StdioServerTransport::new();
    server.run(transport).await?;
    
    Ok(())
}

async fn execute_database_query(database: &str, query: &str) -> Result<Vec<serde_json::Value>, String> {
    // Your database implementation here
    Ok(vec![])
}
```

**Configuration with environment:**
```json
{
  "mcpServers": {
    "database": {
      "command": "/usr/local/bin/database-server",
      "args": ["--db-url", "postgresql://user:pass@localhost/mydb"],
      "env": {
        "DATABASE_URL": "postgresql://user:pass@localhost/mydb"
      }
    }
  }
}
```

### Web API Server

For external API integration:

```rust
use mcp_protocol_sdk::prelude::*;
use serde_json::json;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("web-api-server", "1.0.0")
        .with_description("Web API integration for Claude");
    
    // Weather API tool
    let weather_tool = Tool::new("get_weather", "Get current weather for a location")
        .with_parameter("location", "City name or coordinates", true)
        .with_parameter("units", "Temperature units (celsius/fahrenheit)", false);
    server.add_tool(weather_tool);
    
    server.set_tool_handler("get_weather", |params: HashMap<String, Value>| async move {
        let location = params.get("location")
            .and_then(|v| v.as_str())
            .ok_or("Missing location parameter")?;
            
        let units = params.get("units")
            .and_then(|v| v.as_str())
            .unwrap_or("celsius");
        
        // Call weather API
        let weather_data = fetch_weather(location, units).await?;
        
        Ok(json!({
            "location": location,
            "temperature": weather_data.temperature,
            "condition": weather_data.condition,
            "humidity": weather_data.humidity,
            "units": units
        }))
    });
    
    let transport = StdioServerTransport::new();
    server.run(transport).await?;
    
    Ok(())
}
```

## Advanced Configuration

### Server with Arguments

```json
{
  "mcpServers": {
    "my-server": {
      "command": "/usr/local/bin/my-server",
      "args": [
        "--config", "/path/to/config.json",
        "--log-level", "info",
        "--feature", "advanced"
      ]
    }
  }
}
```

### Server with Environment Variables

```json
{
  "mcpServers": {
    "my-server": {
      "command": "/usr/local/bin/my-server",
      "args": [],
      "env": {
        "DATABASE_URL": "postgresql://localhost/mydb",
        "API_KEY": "your-api-key-here",
        "LOG_LEVEL": "debug"
      }
    }
  }
}
```

### Multiple Servers

```json
{
  "mcpServers": {
    "filesystem": {
      "command": "/usr/local/bin/filesystem-server",
      "args": []
    },
    "database": {
      "command": "/usr/local/bin/database-server",
      "args": ["--readonly"],
      "env": {
        "DATABASE_URL": "postgresql://localhost/mydb"
      }
    },
    "web-apis": {
      "command": "/usr/local/bin/api-server",
      "args": ["--config", "/etc/api-config.json"]
    }
  }
}
```

## Server Development Best Practices

### Error Handling

```rust
server.set_tool_handler("risky_operation", |params| async move {
    match perform_operation(params).await {
        Ok(result) => Ok(json!({"success": true, "data": result})),
        Err(e) => {
            // Log detailed error internally
            eprintln!("Operation failed: {:?}", e);
            
            // Return user-friendly error to Claude
            Err(format!("Unable to complete operation: {}", e.user_message()))
        }
    }
});
```

### Input Validation

```rust
server.set_tool_handler("validate_input", |params| async move {
    let path = params.get("path")
        .and_then(|v| v.as_str())
        .ok_or("Missing path parameter")?;
    
    // Validate path security
    if path.contains("..") || path.starts_with('/') {
        return Err("Invalid path: security violation".to_string());
    }
    
    // Validate path exists
    if !std::path::Path::new(path).exists() {
        return Err(format!("Path does not exist: {}", path));
    }
    
    // Continue with operation...
    Ok(json!({"validated": true}))
});
```

### Resource Management

```rust
use std::sync::Arc;
use tokio::sync::Semaphore;

struct DatabaseServer {
    connection_pool: Arc<ConnectionPool>,
    concurrency_limit: Arc<Semaphore>,
}

impl DatabaseServer {
    pub async fn handle_query(&self, query: &str) -> Result<Value, String> {
        // Limit concurrent operations
        let _permit = self.concurrency_limit.acquire().await
            .map_err(|_| "Server overloaded")?;
            
        // Use connection pool
        let conn = self.connection_pool.get().await
            .map_err(|e| format!("Database connection failed: {}", e))?;
            
        // Execute query with timeout
        let result = tokio::time::timeout(
            Duration::from_secs(30),
            conn.execute(query)
        ).await
        .map_err(|_| "Query timeout")?
        .map_err(|e| format!("Query failed: {}", e))?;
        
        Ok(json!({"results": result}))
    }
}
```

## Testing Your Integration

### 1. Test Server Standalone

```bash
# Test your server directly
echo '{"jsonrpc": "2.0", "id": 1, "method": "tools/list"}' | ./my-server
```

### 2. Test with Claude Desktop

1. Add server to configuration
2. Restart Claude Desktop
3. Start a conversation and ask Claude to use your tools
4. Check server logs for errors

### 3. Debug Configuration Issues

Check Claude Desktop logs:
- **macOS**: `~/Library/Logs/Claude/`
- **Windows**: `%APPDATA%\Claude\logs\`
- **Linux**: `~/.local/share/Claude/logs/`

## Common Issues & Solutions

### Server Not Loading

**Problem**: Server doesn't appear in Claude Desktop

**Solutions**:
1. Check configuration file syntax with JSON validator
2. Verify server binary path and permissions
3. Test server execution manually
4. Check Claude Desktop logs for errors

### Tool Calls Failing

**Problem**: Tools are visible but calls fail

**Solutions**:
1. Validate tool parameter definitions
2. Add proper error handling in tool handlers
3. Test with minimal examples first
4. Check server output for error messages

### Performance Issues

**Problem**: Server responses are slow

**Solutions**:
1. Add timeout handling
2. Implement connection pooling
3. Cache expensive operations
4. Use async/await properly

### Security Considerations

**Important**: Your server runs with Claude Desktop's permissions!

1. **Validate all inputs** thoroughly
2. **Restrict file system access** to safe directories
3. **Sanitize database queries** to prevent injection
4. **Rate limit** expensive operations
5. **Log security events** for monitoring

## Production Deployment

### Binary Distribution

```bash
# Build optimized binary
cargo build --release --features stdio

# Strip debug symbols
strip target/release/my-server

# Create installer package
# (platform-specific packaging)
```

### Configuration Management

```json
{
  "mcpServers": {
    "my-server": {
      "command": "/opt/my-company/bin/my-server",
      "args": ["--config", "/opt/my-company/etc/config.json"],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

### Monitoring & Logging

```rust
use tracing::{info, warn, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_target(false)
        .with_thread_ids(true)
        .with_file(true)
        .with_line_number(true)
        .init();
    
    info!("Starting MCP server");
    
    let mut server = McpServer::new("my-server", "1.0.0");
    
    // Log tool calls
    server.set_tool_call_middleware(|tool_name, params| async move {
        info!("Tool called: {} with params: {:?}", tool_name, params);
    });
    
    // Log errors
    server.set_error_handler(|error| async move {
        error!("Server error: {:?}", error);
    });
    
    let transport = StdioServerTransport::new();
    server.run(transport).await?;
    
    Ok(())
}
```

## Example Projects

Check out these complete examples:

1. **[Filesystem Server](../../examples/simple_server.rs)** - File operations
2. **[Database Server](../../examples/database_server.rs)** - SQL queries  
3. **[Echo Server](../../examples/echo_server.rs)** - Simple echo tool

## Next Steps

1. **Build your first server** using the examples above
2. **Test integration** with Claude Desktop
3. **Add monitoring** and error handling
4. **Share with others** in the MCP community

Your tools are now available to Claude! Start experimenting with natural language interactions that trigger your custom functionality. 🚀
