# Examples

This page showcases real-world examples of using MCP Rust SDK in different scenarios, from basic implementations to complex production-ready applications.

## Quick Start Examples

### Minimal Echo Server

The simplest possible MCP server that echoes back messages.

```rust
use mcp_protocol_sdk::{
    server::McpServer,
    transport::stdio::StdioServerTransport,
    core::tool::ToolHandler,
    protocol::types::{ToolResult, Content},
};
use async_trait::async_trait;
use std::collections::HashMap;
use serde_json::{json, Value};

struct EchoTool;

#[async_trait]
impl ToolHandler for EchoTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_protocol_sdk::core::error::McpError> {
        let message = arguments.get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("No message provided");

        Ok(ToolResult {
            content: vec![Content::text(format!("Echo: {}", message))],
            is_error: None,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("echo-server".to_string(), "1.0.0".to_string());

    server.add_tool(
        "echo".to_string(),
        Some("Echoes back the provided message".to_string()),
        json!({
            "type": "object",
            "properties": {
                "message": {
                    "type": "string",
                    "description": "Message to echo back"
                }
            },
            "required": ["message"]
        }),
        EchoTool,
    ).await?;

    let transport = StdioServerTransport::new();
    server.start(transport).await?;

    Ok(())
}
```

### Basic Client

A simple client that connects to a server and calls tools.

```rust
use mcp_protocol_sdk::{
    client::{McpClient, ClientSession},
    transport::stdio::StdioClientTransport,
};
use std::collections::HashMap;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = McpClient::new("demo-client".to_string(), "1.0.0".to_string());
    let session = ClientSession::new(client);
    
    let transport = StdioClientTransport::new("./echo-server".to_string()).await?;
    let init_result = session.connect(transport).await?;
    
    println!("Connected to: {} v{}", 
        init_result.server_info.name, 
        init_result.server_info.version
    );

    let client = session.client();
    let client_guard = client.lock().await;
    
    let mut args = HashMap::new();
    args.insert("message".to_string(), json!("Hello from client!"));
    
    let result = client_guard.call_tool("echo".to_string(), Some(args)).await?;
    println!("Tool result: {:?}", result);

    Ok(())
}
```

## Transport Examples

### HTTP Server with REST API

An HTTP-based MCP server that can be accessed via REST endpoints.

```rust
use mcp_protocol_sdk::{
    server::McpServer,
    transport::http::HttpServerTransport,
    core::tool::ToolHandler,
    protocol::types::{ToolResult, Content},
};
use async_trait::async_trait;
use std::collections::HashMap;
use serde_json::{json, Value};

struct WeatherTool;

#[async_trait]
impl ToolHandler for WeatherTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_protocol_sdk::core::error::McpError> {
        let city = arguments.get("city")
            .and_then(|v| v.as_str())
            .unwrap_or("Unknown");

        // Simulate weather API call
        let weather_data = json!({
            "city": city,
            "temperature": 22,
            "condition": "Sunny",
            "humidity": 65
        });

        Ok(ToolResult {
            content: vec![Content::text(format!("Weather in {}: {}", city, weather_data))],
            is_error: None,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("weather-server".to_string(), "1.0.0".to_string());

    server.add_tool(
        "get_weather".to_string(),
        Some("Get current weather for a city".to_string()),
        json!({
            "type": "object",
            "properties": {
                "city": {
                    "type": "string",
                    "description": "City name"
                }
            },
            "required": ["city"]
        }),
        WeatherTool,
    ).await?;

    // Start HTTP server
    let transport = HttpServerTransport::new("0.0.0.0:3000");
    server.start(transport).await?;
    
    println!("HTTP server running on http://localhost:3000");
    println!("API endpoint: http://localhost:3000/mcp");
    println!("SSE events: http://localhost:3000/mcp/events");
    
    // Keep running until interrupted
    tokio::signal::ctrl_c().await?;
    server.stop().await?;

    Ok(())
}
```

### WebSocket Real-time Server

A WebSocket server for real-time communication and live data feeds.

```rust
use mcp_protocol_sdk::{
    server::McpServer,
    transport::websocket::WebSocketServerTransport,
    core::tool::ToolHandler,
    protocol::types::{ToolResult, Content},
};
use async_trait::async_trait;
use std::collections::HashMap;
use serde_json::{json, Value};
use tokio::time::{interval, Duration};

struct RealTimeDataTool;

#[async_trait]
impl ToolHandler for RealTimeDataTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_protocol_sdk::core::error::McpError> {
        let data_type = arguments.get("type")
            .and_then(|v| v.as_str())
            .unwrap_or("default");

        let current_time = chrono::Utc::now();
        let data = match data_type {
            "stock" => json!({
                "symbol": "AAPL",
                "price": 150.25,
                "change": "+2.5%",
                "timestamp": current_time
            }),
            "crypto" => json!({
                "symbol": "BTC",
                "price": 42000.50,
                "change": "-1.2%",
                "timestamp": current_time
            }),
            _ => json!({
                "message": "Unknown data type",
                "timestamp": current_time
            })
        };

        Ok(ToolResult {
            content: vec![Content::text(format!("Real-time data: {}", data))],
            is_error: None,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("realtime-server".to_string(), "1.0.0".to_string());

    server.add_tool(
        "get_realtime_data".to_string(),
        Some("Get real-time market data".to_string()),
        json!({
            "type": "object",
            "properties": {
                "type": {
                    "type": "string",
                    "enum": ["stock", "crypto"],
                    "description": "Type of data to retrieve"
                }
            },
            "required": ["type"]
        }),
        RealTimeDataTool,
    ).await?;

    // Start WebSocket server
    let transport = WebSocketServerTransport::new("0.0.0.0:8080");
    server.start(transport).await?;
    
    println!("WebSocket server running on ws://localhost:8080");
    
    // Keep running until interrupted
    tokio::signal::ctrl_c().await?;
    server.stop().await?;

    Ok(())
}
```

## Advanced Examples

### Database Integration Server

A server that integrates with databases and provides data access tools.

```rust
use mcp_protocol_sdk::{
    server::McpServer,
    transport::stdio::StdioServerTransport,
    core::{tool::ToolHandler, resource::ResourceHandler},
    protocol::types::{ToolResult, Content, ResourceInfo, ResourceContent},
};
use async_trait::async_trait;
use std::collections::HashMap;
use serde_json::{json, Value};

// Mock database connection
struct Database {
    data: HashMap<String, String>,
}

impl Database {
    fn new() -> Self {
        let mut data = HashMap::new();
        data.insert("user:1".to_string(), json!({"id": 1, "name": "Alice", "email": "alice@example.com"}).to_string());
        data.insert("user:2".to_string(), json!({"id": 2, "name": "Bob", "email": "bob@example.com"}).to_string());
        
        Self { data }
    }

    async fn query(&self, sql: &str) -> Result<String, String> {
        // Simple mock SQL processing
        if sql.contains("SELECT * FROM users") {
            let users: Vec<&String> = self.data.iter()
                .filter(|(k, _)| k.starts_with("user:"))
                .map(|(_, v)| v)
                .collect();
            Ok(format!("[{}]", users.join(",")))
        } else {
            Err("Unsupported query".to_string())
        }
    }

    async fn get(&self, key: &str) -> Option<String> {
        self.data.get(key).cloned()
    }
}

struct DatabaseTool {
    db: Database,
}

#[async_trait]
impl ToolHandler for DatabaseTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_protocol_sdk::core::error::McpError> {
        let query = arguments.get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_protocol_sdk::core::error::McpError::ValidationError("Missing query parameter".to_string()))?;

        match self.db.query(query).await {
            Ok(result) => Ok(ToolResult {
                content: vec![Content::text(result)],
                is_error: None,
            }),
            Err(e) => Ok(ToolResult {
                content: vec![Content::text(format!("Error: {}", e))],
                is_error: Some(true),
            }),
        }
    }
}

struct DatabaseResource {
    db: Database,
}

#[async_trait]
impl ResourceHandler for DatabaseResource {
    async fn read(&self, uri: &str, _params: &HashMap<String, String>) -> Result<Vec<ResourceContent>, mcp_protocol_sdk::core::error::McpError> {
        if uri.starts_with("db://") {
            let key = &uri[5..];
            match self.db.get(key).await {
                Some(data) => Ok(vec![ResourceContent {
                    uri: uri.to_string(),
                    mime_type: Some("application/json".to_string()),
                    text: Some(data),
                    blob: None,
                }]),
                None => Err(mcp_protocol_sdk::core::error::McpError::ResourceNotFound(uri.to_string())),
            }
        } else {
            Err(mcp_protocol_sdk::core::error::McpError::ResourceNotFound(uri.to_string()))
        }
    }

    async fn list(&self) -> Result<Vec<ResourceInfo>, mcp_protocol_sdk::core::error::McpError> {
        Ok(vec![
            ResourceInfo {
                uri: "db://user:1".to_string(),
                name: "User 1".to_string(),
                description: Some("User record #1".to_string()),
                mime_type: Some("application/json".to_string()),
            },
            ResourceInfo {
                uri: "db://user:2".to_string(),
                name: "User 2".to_string(),
                description: Some("User record #2".to_string()),
                mime_type: Some("application/json".to_string()),
            },
        ])
    }

    async fn subscribe(&self, _uri: &str) -> Result<(), mcp_protocol_sdk::core::error::McpError> {
        Ok(())
    }

    async fn unsubscribe(&self, _uri: &str) -> Result<(), mcp_protocol_sdk::core::error::McpError> {
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("database-server".to_string(), "1.0.0".to_string());

    let db = Database::new();

    // Add database query tool
    server.add_tool(
        "query_database".to_string(),
        Some("Execute SQL queries on the database".to_string()),
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
        DatabaseTool { db: Database::new() },
    ).await?;

    // Add database resource access
    server.add_resource_detailed(
        ResourceInfo {
            uri: "db://".to_string(),
            name: "Database".to_string(),
            description: Some("Database records access".to_string()),
            mime_type: Some("application/json".to_string()),
        },
        DatabaseResource { db: Database::new() },
    ).await?;

    let transport = StdioServerTransport::new();
    server.start(transport).await?;

    Ok(())
}
```

### Multi-Client Chat Server

A WebSocket server that supports multiple clients with shared state.

```rust
use mcp_protocol_sdk::{
    server::McpServer,
    transport::websocket::WebSocketServerTransport,
    core::tool::ToolHandler,
    protocol::types::{ToolResult, Content},
};
use async_trait::async_trait;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde_json::{json, Value};

// Shared state for multi-client communication
#[derive(Default)]
struct ChatState {
    messages: Vec<String>,
    users: Vec<String>,
}

struct BroadcastTool {
    state: Arc<RwLock<ChatState>>,
}

#[async_trait]
impl ToolHandler for BroadcastTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_protocol_sdk::core::error::McpError> {
        let user = arguments.get("user")
            .and_then(|v| v.as_str())
            .unwrap_or("Anonymous");
        let message = arguments.get("message")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let formatted_message = format!("{}: {}", user, message);
        
        // Add to shared state
        {
            let mut state = self.state.write().await;
            state.messages.push(formatted_message.clone());
            if !state.users.contains(&user.to_string()) {
                state.users.push(user.to_string());
            }
        }

        Ok(ToolResult {
            content: vec![Content::text(format!("Broadcasted: {}", formatted_message))],
            is_error: None,
        })
    }
}

struct GetMessagesTool {
    state: Arc<RwLock<ChatState>>,
}

#[async_trait]
impl ToolHandler for GetMessagesTool {
    async fn call(&self, _arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_protocol_sdk::core::error::McpError> {
        let state = self.state.read().await;
        let messages = state.messages.join("\n");
        
        Ok(ToolResult {
            content: vec![Content::text(messages)],
            is_error: None,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("chat-server".to_string(), "1.0.0".to_string());
    let state = Arc::new(RwLock::new(ChatState::default()));

    // Add broadcast tool
    server.add_tool(
        "broadcast".to_string(),
        Some("Broadcast a message to all clients".to_string()),
        json!({
            "type": "object",
            "properties": {
                "user": {"type": "string", "description": "Username"},
                "message": {"type": "string", "description": "Message to broadcast"}
            },
            "required": ["user", "message"]
        }),
        BroadcastTool { state: state.clone() },
    ).await?;

    // Add get messages tool
    server.add_tool(
        "get_messages".to_string(),
        Some("Get all chat messages".to_string()),
        json!({"type": "object", "properties": {}}),
        GetMessagesTool { state: state.clone() },
    ).await?;

    let transport = WebSocketServerTransport::new("0.0.0.0:8080");
    server.start(transport).await?;
    
    println!("Multi-client chat server running on ws://localhost:8080");
    
    tokio::signal::ctrl_c().await?;
    server.stop().await?;

    Ok(())
}
```

### Resilient Client with Auto-Reconnection

A production-ready client with session management and fault tolerance.

```rust
use mcp_protocol_sdk::{
    client::{McpClient, ClientSession},
    client::session::SessionConfig,
    transport::websocket::WebSocketClientTransport,
};
use std::time::Duration;
use tokio::time::sleep;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = McpClient::new("resilient-client".to_string(), "1.0.0".to_string());
    
    // Configure session with auto-reconnection
    let session_config = SessionConfig {
        auto_reconnect: true,
        max_reconnect_attempts: 10,
        reconnect_delay_ms: 2000,
        connection_timeout_ms: 15000,
        heartbeat_interval_ms: 30000,
        max_concurrent_requests: 10,
        request_timeout_ms: 30000,
        ..Default::default()
    };
    
    let session = ClientSession::with_config(client, session_config);
    
    // Connect with WebSocket transport
    let transport = WebSocketClientTransport::new("ws://localhost:8080").await?;
    
    match session.connect(transport).await {
        Ok(init_result) => {
            println!("Connected to: {} v{}", 
                init_result.server_info.name, 
                init_result.server_info.version
            );
            
            // Simulate long-running client with periodic requests
            loop {
                let client = session.client();
                let client_guard = client.lock().await;
                
                match client_guard.list_tools().await {
                    Ok(tools) => {
                        println!("Available tools: {:?}", tools.tools.len());
                    }
                    Err(e) => {
                        eprintln!("Error listing tools: {}", e);
                        // Session will auto-reconnect if needed
                    }
                }
                
                drop(client_guard);
                sleep(Duration::from_secs(5)).await;
            }
        }
        Err(e) => {
            eprintln!("Failed to connect: {}", e);
        }
    }
    
    Ok(())
}
```

## Production Examples

### File Management Server

A comprehensive file system server with security and error handling.

```rust
use mcp_protocol_sdk::{
    server::McpServer,
    transport::http::HttpServerTransport,
    core::{tool::ToolHandler, resource::ResourceHandler},
    protocol::types::{ToolResult, Content, ResourceInfo, ResourceContent},
};
use async_trait::async_trait;
use std::collections::HashMap;
use serde_json::{json, Value};
use tokio::fs;
use std::path::Path;

struct ReadFileTool {
    allowed_paths: Vec<String>,
}

impl ReadFileTool {
    fn new(allowed_paths: Vec<String>) -> Self {
        Self { allowed_paths }
    }

    fn is_path_allowed(&self, path: &str) -> bool {
        self.allowed_paths.iter().any(|allowed| path.starts_with(allowed))
    }
}

#[async_trait]
impl ToolHandler for ReadFileTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_protocol_sdk::core::error::McpError> {
        let path = arguments.get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_protocol_sdk::core::error::McpError::ValidationError("Missing path parameter".to_string()))?;

        // Security check
        if !self.is_path_allowed(path) {
            return Ok(ToolResult {
                content: vec![Content::text("Access denied: Path not allowed".to_string())],
                is_error: Some(true),
            });
        }

        // Validate path exists and is a file
        if !Path::new(path).exists() {
            return Ok(ToolResult {
                content: vec![Content::text(format!("File not found: {}", path))],
                is_error: Some(true),
            });
        }

        if !Path::new(path).is_file() {
            return Ok(ToolResult {
                content: vec![Content::text(format!("Path is not a file: {}", path))],
                is_error: Some(true),
            });
        }

        match fs::read_to_string(path).await {
            Ok(content) => {
                // Limit content size for safety
                if content.len() > 1_000_000 {
                    Ok(ToolResult {
                        content: vec![Content::text("File too large (>1MB)".to_string())],
                        is_error: Some(true),
                    })
                } else {
                    Ok(ToolResult {
                        content: vec![Content::text(content)],
                        is_error: None,
                    })
                }
            }
            Err(e) => Ok(ToolResult {
                content: vec![Content::text(format!("Error reading file: {}", e))],
                is_error: Some(true),
            }),
        }
    }
}

struct WriteFileTool {
    allowed_paths: Vec<String>,
}

impl WriteFileTool {
    fn new(allowed_paths: Vec<String>) -> Self {
        Self { allowed_paths }
    }

    fn is_path_allowed(&self, path: &str) -> bool {
        self.allowed_paths.iter().any(|allowed| path.starts_with(allowed))
    }
}

#[async_trait]
impl ToolHandler for WriteFileTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_protocol_sdk::core::error::McpError> {
        let path = arguments.get("path")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_protocol_sdk::core::error::McpError::ValidationError("Missing path parameter".to_string()))?;
        let content = arguments.get("content")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_protocol_sdk::core::error::McpError::ValidationError("Missing content parameter".to_string()))?;

        // Security check
        if !self.is_path_allowed(path) {
            return Ok(ToolResult {
                content: vec![Content::text("Access denied: Path not allowed".to_string())],
                is_error: Some(true),
            });
        }

        // Limit content size
        if content.len() > 1_000_000 {
            return Ok(ToolResult {
                content: vec![Content::text("Content too large (>1MB)".to_string())],
                is_error: Some(true),
            });
        }

        // Create parent directories if they don't exist
        if let Some(parent) = Path::new(path).parent() {
            if let Err(e) = fs::create_dir_all(parent).await {
                return Ok(ToolResult {
                    content: vec![Content::text(format!("Error creating directories: {}", e))],
                    is_error: Some(true),
                });
            }
        }

        match fs::write(path, content).await {
            Ok(_) => Ok(ToolResult {
                content: vec![Content::text(format!("File written successfully: {}", path))],
                is_error: None,
            }),
            Err(e) => Ok(ToolResult {
                content: vec![Content::text(format!("Error writing file: {}", e))],
                is_error: Some(true),
            }),
        }
    }
}

struct ListDirectoryTool {
    allowed_paths: Vec<String>,
}

impl ListDirectoryTool {
    fn new(allowed_paths: Vec<String>) -> Self {
        Self { allowed_paths }
    }

    fn is_path_allowed(&self, path: &str) -> bool {
        self.allowed_paths.iter().any(|allowed| path.starts_with(allowed))
    }
}

#[async_trait]
impl ToolHandler for ListDirectoryTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_protocol_sdk::core::error::McpError> {
        let path = arguments.get("path")
            .and_then(|v| v.as_str())
            .unwrap_or(".");

        // Security check
        if !self.is_path_allowed(path) {
            return Ok(ToolResult {
                content: vec![Content::text("Access denied: Path not allowed".to_string())],
                is_error: Some(true),
            });
        }

        match fs::read_dir(path).await {
            Ok(mut entries) => {
                let mut files = Vec::new();
                while let Some(entry) = entries.next_entry().await.transpose() {
                    match entry {
                        Ok(entry) => {
                            let name = entry.file_name().to_string_lossy().to_string();
                            let metadata = entry.metadata().await;
                            let file_type = match metadata {
                                Ok(meta) if meta.is_dir() => "directory",
                                Ok(meta) if meta.is_file() => "file",
                                _ => "unknown",
                            };
                            let size = metadata.map(|m| m.len()).unwrap_or(0);
                            
                            files.push(json!({
                                "name": name,
                                "type": file_type,
                                "size": size
                            }));
                        }
                        Err(e) => {
                            eprintln!("Error reading directory entry: {}", e);
                        }
                    }
                }

                Ok(ToolResult {
                    content: vec![Content::text(json!(files).to_string())],
                    is_error: None,
                })
            }
            Err(e) => Ok(ToolResult {
                content: vec![Content::text(format!("Error listing directory: {}", e))],
                is_error: Some(true),
            }),
        }
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("file-server".to_string(), "1.0.0".to_string());

    // Define allowed paths for security
    let allowed_paths = vec![
        "/tmp/".to_string(),
        "/home/user/documents/".to_string(),
        "/var/uploads/".to_string(),
    ];

    // Add file operations with security constraints
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
        ReadFileTool::new(allowed_paths.clone()),
    ).await?;

    server.add_tool(
        "write_file".to_string(),
        Some("Write content to a file".to_string()),
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "File path to write"},
                "content": {"type": "string", "description": "Content to write"}
            },
            "required": ["path", "content"]
        }),
        WriteFileTool::new(allowed_paths.clone()),
    ).await?;

    server.add_tool(
        "list_directory".to_string(),
        Some("List contents of a directory".to_string()),
        json!({
            "type": "object",
            "properties": {
                "path": {"type": "string", "description": "Directory path to list"}
            }
        }),
        ListDirectoryTool::new(allowed_paths),
    ).await?;

    let transport = HttpServerTransport::new("0.0.0.0:3000");
    server.start(transport).await?;
    
    println!("Secure file management server running on http://localhost:3000");
    println!("Allowed paths: /tmp/, /home/user/documents/, /var/uploads/");
    
    tokio::signal::ctrl_c().await?;
    server.stop().await?;

    Ok(())
}
```

### Microservice Integration Server

An enterprise-ready server that integrates with external APIs and services.

```rust
use mcp_protocol_sdk::{
    server::McpServer,
    server::mcp_server::ServerConfig,
    transport::http::HttpServerTransport,
    core::tool::ToolHandler,
    protocol::types::{ToolResult, Content},
};
use async_trait::async_trait;
use std::collections::HashMap;
use serde_json::{json, Value};
use reqwest::Client;
use tokio::time::{timeout, Duration};

struct HttpRequestTool {
    client: Client,
    allowed_hosts: Vec<String>,
}

impl HttpRequestTool {
    fn new(allowed_hosts: Vec<String>) -> Self {
        let client = Client::builder()
            .timeout(Duration::from_secs(30))
            .user_agent("MCP-SDK/1.0")
            .build()
            .expect("Failed to create HTTP client");

        Self { client, allowed_hosts }
    }

    fn is_host_allowed(&self, url: &str) -> bool {
        self.allowed_hosts.iter().any(|host| url.contains(host))
    }
}

#[async_trait]
impl ToolHandler for HttpRequestTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_protocol_sdk::core::error::McpError> {
        let url = arguments.get("url")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_protocol_sdk::core::error::McpError::ValidationError("Missing url parameter".to_string()))?;

        let method = arguments.get("method")
            .and_then(|v| v.as_str())
            .unwrap_or("GET");

        let headers = arguments.get("headers")
            .and_then(|v| v.as_object())
            .cloned()
            .unwrap_or_default();

        let body = arguments.get("body")
            .and_then(|v| v.as_str());

        // Security check
        if !self.is_host_allowed(url) {
            return Ok(ToolResult {
                content: vec![Content::text("Access denied: Host not allowed".to_string())],
                is_error: Some(true),
            });
        }

        let mut request = match method.to_uppercase().as_str() {
            "GET" => self.client.get(url),
            "POST" => self.client.post(url),
            "PUT" => self.client.put(url),
            "DELETE" => self.client.delete(url),
            _ => {
                return Ok(ToolResult {
                    content: vec![Content::text(format!("Unsupported HTTP method: {}", method))],
                    is_error: Some(true),
                });
            }
        };

        // Add headers
        for (key, value) in headers {
            if let Some(value_str) = value.as_str() {
                request = request.header(key, value_str);
            }
        }

        // Add body if present
        if let Some(body_content) = body {
            request = request.body(body_content.to_string());
        }

        // Execute request with timeout
        match timeout(Duration::from_secs(30), request.send()).await {
            Ok(Ok(response)) => {
                let status = response.status();
                let headers = response.headers().clone();
                
                match response.text().await {
                    Ok(body) => {
                        let result = json!({
                            "status": status.as_u16(),
                            "headers": headers.iter().map(|(k, v)| (k.to_string(), v.to_str().unwrap_or("").to_string())).collect::<HashMap<String, String>>(),
                            "body": body
                        });

                        Ok(ToolResult {
                            content: vec![Content::text(result.to_string())],
                            is_error: None,
                        })
                    }
                    Err(e) => Ok(ToolResult {
                        content: vec![Content::text(format!("Error reading response body: {}", e))],
                        is_error: Some(true),
                    }),
                }
            }
            Ok(Err(e)) => Ok(ToolResult {
                content: vec![Content::text(format!("HTTP request failed: {}", e))],
                is_error: Some(true),
            }),
            Err(_) => Ok(ToolResult {
                content: vec![Content::text("Request timeout".to_string())],
                is_error: Some(true),
            }),
        }
    }
}

struct DatabaseQueryTool {
    connection_string: String,
}

impl DatabaseQueryTool {
    fn new(connection_string: String) -> Self {
        Self { connection_string }
    }
}

#[async_trait]
impl ToolHandler for DatabaseQueryTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, mcp_protocol_sdk::core::error::McpError> {
        let query = arguments.get("query")
            .and_then(|v| v.as_str())
            .ok_or_else(|| mcp_protocol_sdk::core::error::McpError::ValidationError("Missing query parameter".to_string()))?;

        // In a real implementation, you would connect to the actual database
        // For this example, we'll simulate database queries
        let result = match query.to_lowercase().as_str() {
            q if q.contains("select") => {
                json!([
                    {"id": 1, "name": "Alice", "status": "active"},
                    {"id": 2, "name": "Bob", "status": "inactive"}
                ])
            }
            q if q.contains("insert") => {
                json!({"affected_rows": 1, "last_insert_id": 3})
            }
            q if q.contains("update") => {
                json!({"affected_rows": 1})
            }
            q if q.contains("delete") => {
                json!({"affected_rows": 1})
            }
            _ => {
                return Ok(ToolResult {
                    content: vec![Content::text("Unsupported query type".to_string())],
                    is_error: Some(true),
                });
            }
        };

        Ok(ToolResult {
            content: vec![Content::text(result.to_string())],
            is_error: None,
        })
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Configure server for production use
    let config = ServerConfig {
        max_concurrent_requests: 100,
        request_timeout_ms: 60000,
        validate_requests: true,
        enable_logging: true,
    };

    let mut server = McpServer::with_config(
        "microservice-server".to_string(),
        "1.0.0".to_string(),
        config,
    );

    // Define allowed external hosts
    let allowed_hosts = vec![
        "api.github.com".to_string(),
        "httpbin.org".to_string(),
        "jsonplaceholder.typicode.com".to_string(),
    ];

    // Add HTTP request tool
    server.add_tool(
        "http_request".to_string(),
        Some("Make HTTP requests to external APIs".to_string()),
        json!({
            "type": "object",
            "properties": {
                "url": {"type": "string", "description": "Request URL"},
                "method": {"type": "string", "enum": ["GET", "POST", "PUT", "DELETE"], "default": "GET"},
                "headers": {"type": "object", "description": "Request headers"},
                "body": {"type": "string", "description": "Request body"}
            },
            "required": ["url"]
        }),
        HttpRequestTool::new(allowed_hosts),
    ).await?;

    // Add database query tool
    server.add_tool(
        "database_query".to_string(),
        Some("Execute database queries".to_string()),
        json!({
            "type": "object",
            "properties": {
                "query": {"type": "string", "description": "SQL query to execute"}
            },
            "required": ["query"]
        }),
        DatabaseQueryTool::new("postgresql://localhost/mydb".to_string()),
    ).await?;

    let transport = HttpServerTransport::new("0.0.0.0:8080");
    server.start(transport).await?;
    
    println!("Microservice integration server running on http://localhost:8080");
    println!("Endpoints:");
    println!("  - POST /mcp (MCP JSON-RPC)");
    println!("  - GET /mcp/events (Server-Sent Events)");
    println!("  - GET /health (Health check)");
    
    tokio::signal::ctrl_c().await?;
    server.stop().await?;

    Ok(())
}
```

## Testing Examples

### Unit Testing Tools and Resources

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mcp_protocol_sdk::{
        core::{tool::ToolHandler, resource::ResourceHandler},
        protocol::types::{ToolResult, Content},
    };
    use std::collections::HashMap;
    use serde_json::json;

    #[tokio::test]
    async fn test_echo_tool() {
        let tool = EchoTool;
        let mut args = HashMap::new();
        args.insert("message".to_string(), json!("Hello World"));

        let result = tool.call(args).await.unwrap();
        
        assert!(!result.content.is_empty());
        if let Content::Text { text } = &result.content[0] {
            assert_eq!(text, "Echo: Hello World");
        } else {
            panic!("Expected text content");
        }
        assert!(result.is_error.is_none());
    }

    #[tokio::test]
    async fn test_calculator_tool() {
        let tool = CalculatorHandler;
        let mut args = HashMap::new();
        args.insert("a".to_string(), json!(5.0));
        args.insert("b".to_string(), json!(3.0));
        args.insert("operation".to_string(), json!("add"));

        let result = tool.call(args).await.unwrap();
        
        assert!(!result.content.is_empty());
        if let Content::Text { text } = &result.content[0] {
            assert!(text.contains("5 add 3 = 8"));
        } else {
            panic!("Expected text content");
        }
    }

    #[tokio::test]
    async fn test_calculator_division_by_zero() {
        let tool = CalculatorHandler;
        let mut args = HashMap::new();
        args.insert("a".to_string(), json!(5.0));
        args.insert("b".to_string(), json!(0.0));
        args.insert("operation".to_string(), json!("divide"));

        let result = tool.call(args).await.unwrap();
        
        assert!(result.is_error.unwrap_or(false));
        if let Content::Text { text } = &result.content[0] {
            assert!(text.contains("Division by zero"));
        }
    }

    #[tokio::test]
    async fn test_file_system_resource() {
        let resource = FileSystemHandler::new();
        
        // Test listing resources
        let resources = resource.list().await.unwrap();
        assert!(!resources.is_empty());
        
        // Test reading a specific resource
        let content = resource.read("file:///demo.txt", &HashMap::new()).await.unwrap();
        assert!(!content.is_empty());
        assert_eq!(content[0].text, Some("This is a demo file!".to_string()));
    }

    #[tokio::test]
    async fn test_resource_not_found() {
        let resource = FileSystemHandler::new();
        
        let result = resource.read("file:///nonexistent.txt", &HashMap::new()).await;
        assert!(result.is_err());
    }
}
```

### Integration Testing

```rust
#[cfg(test)]
mod integration_tests {
    use super::*;
    use mcp_protocol_sdk::{
        server::McpServer,
        client::{McpClient, ClientSession},
        transport::stdio::{StdioServerTransport, StdioClientTransport},
    };
    use tokio::process::{Command, Child};
    use std::time::Duration;
    use tokio::time::sleep;

    struct TestServer {
        child: Child,
    }

    impl TestServer {
        async fn start() -> Result<Self, Box<dyn std::error::Error>> {
            let child = Command::new("cargo")
                .args(["run", "--example", "simple_server"])
                .stdin(std::process::Stdio::piped())
                .stdout(std::process::Stdio::piped())
                .stderr(std::process::Stdio::piped())
                .spawn()?;

            // Wait for server to start
            sleep(Duration::from_millis(500)).await;

            Ok(TestServer { child })
        }
    }

    impl Drop for TestServer {
        fn drop(&mut self) {
            let _ = self.child.kill();
        }
    }

    #[tokio::test]
    async fn test_client_server_integration() {
        let _server = TestServer::start().await.expect("Failed to start test server");
        
        let client = McpClient::new("test-client".to_string(), "1.0.0".to_string());
        let session = ClientSession::new(client);
        
        // Give server time to fully start
        sleep(Duration::from_millis(1000)).await;
        
        let transport = StdioClientTransport::new("cargo run --example simple_server".to_string())
            .await
            .expect("Failed to create transport");
            
        let init_result = session.connect(transport).await.expect("Failed to connect");
        
        assert_eq!(init_result.server_info.name, "simple-demo-server");
        assert_eq!(init_result.server_info.version, "1.0.0");
        
        let client = session.client();
        let client_guard = client.lock().await;
        
        // Test listing tools
        let tools = client_guard.list_tools().await.expect("Failed to list tools");
        assert!(!tools.tools.is_empty());
        
        // Test calling a tool
        let mut args = HashMap::new();
        args.insert("message".to_string(), json!("Integration test"));
        
        let result = client_guard.call_tool("echo".to_string(), Some(args)).await;
        assert!(result.is_ok());
    }
}
```

## Running the Examples

All examples are available in the repository and can be run with Cargo:

### Basic Examples
```bash
# Clone the repository
git clone https://github.com/rishirandhawa/mcp-protocol-sdk.git
cd mcp-protocol-sdk

# Run basic examples
cargo run --example simple_server
cargo run --example echo_server
cargo run --example client_example
```

### Transport Examples
```bash
# HTTP examples (requires http feature)
cargo run --example http_server --features http
cargo run --example http_client --features http

# WebSocket examples (requires websocket feature)
cargo run --example websocket_server --features websocket
cargo run --example websocket_client --features websocket
```

### Advanced Examples
```bash
# Database integration
cargo run --example database_server

# Multi-client chat
cargo run --example websocket_server --features websocket

# File management
cargo run --example http_server --features http
```

### Production Examples
```bash
# Run with specific configurations
RUST_LOG=info cargo run --example microservice_server --features "http,websocket" --release

# Run tests
cargo test
cargo test --example simple_server
```

## Example Configuration

### Server Configuration
```rust
use mcp_protocol_sdk::server::mcp_server::ServerConfig;

let config = ServerConfig {
    max_concurrent_requests: 100,
    request_timeout_ms: 30000,
    validate_requests: true,
    enable_logging: true,
};

let server = McpServer::with_config("my-server".to_string(), "1.0.0".to_string(), config);
```

### Client Configuration
```rust
use mcp_protocol_sdk::client::session::SessionConfig;

let session_config = SessionConfig {
    auto_reconnect: true,
    max_reconnect_attempts: 5,
    reconnect_delay_ms: 1000,
    connection_timeout_ms: 10000,
    heartbeat_interval_ms: 30000,
    max_concurrent_requests: 10,
    request_timeout_ms: 30000,
    ..Default::default()
};

let session = ClientSession::with_config(client, session_config);
```

## Next Steps

- [Architecture Guide](architecture.md) - Understanding the system design
- [Transport Guide](transports.md) - Deep dive into transport options
- [API Reference](api-reference.md) - Complete API documentation
- [Integration Guide](integrations/) - Platform-specific integrations

## Contributing Examples

We welcome contributions of new examples! Please:

1. Follow the existing code style and structure
2. Include comprehensive error handling
3. Add appropriate tests
4. Document the example purpose and usage
5. Submit a pull request with your example

For more details, see our [Contributing Guide](../CONTRIBUTING.md).
