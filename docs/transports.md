# Transport Layers

MCP Rust SDK provides multiple transport options for different use cases. Each transport has its own strengths and is optimized for specific scenarios.

## Transport Overview

| Transport | Use Case | Pros | Cons |
|-----------|----------|------|------|
| **STDIO** | Single client, CLI tools | Low latency, simple setup | One client only |
| **HTTP** | Web applications, REST APIs | Multiple clients, stateless | Higher latency |
| **WebSocket** | Real-time applications | Low latency, bidirectional | More complex setup |

## STDIO Transport

STDIO transport uses standard input/output for communication, typically with a child process.

### Server

```rust
use mcp_protocol_sdk::{
    server::McpServer,
    transport::stdio::StdioServerTransport,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("stdio-server".to_string(), "1.0.0".to_string());
    
    // Add your tools, resources, and prompts
    
    let transport = StdioServerTransport::new();
    server.start(transport).await?;
    
    Ok(())
}
```

### Client

```rust
use mcp_protocol_sdk::{
    client::{McpClient, ClientSession},
    transport::stdio::StdioClientTransport,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = McpClient::new("stdio-client".to_string(), "1.0.0".to_string());
    let session = ClientSession::new(client);
    
    // Connect to server executable
    let transport = StdioClientTransport::new("./my-server".to_string()).await?;
    let init_result = session.connect(transport).await?;
    
    // Use the client...
    
    Ok(())
}
```

### Configuration

```rust
use mcp_protocol_sdk::transport::stdio::StdioConfig;

let config = StdioConfig {
    buffer_size: 8192,
    timeout_ms: Some(30_000),
    working_directory: Some("/path/to/working/dir".to_string()),
    environment: [
        ("VAR1".to_string(), "value1".to_string()),
        ("VAR2".to_string(), "value2".to_string()),
    ].into(),
};

let transport = StdioClientTransport::with_config("./server".to_string(), config).await?;
```

## HTTP Transport

HTTP transport provides RESTful API endpoints with Server-Sent Events for real-time notifications.

### Server

```rust
use mcp_protocol_sdk::{
    server::McpServer,
    transport::http::HttpServerTransport,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("http-server".to_string(), "1.0.0".to_string());
    
    // Add your tools, resources, and prompts
    
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

### Client

```rust
use mcp_protocol_sdk::{
    client::{McpClient, ClientSession},
    transport::http::HttpClientTransport,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = McpClient::new("http-client".to_string(), "1.0.0".to_string());
    let session = ClientSession::new(client);
    
    let transport = HttpClientTransport::new("http://localhost:3000/mcp").await?;
    let init_result = session.connect(transport).await?;
    
    // Use the client...
    
    Ok(())
}
```

### HTTP API Endpoints

#### POST /mcp/request

Send JSON-RPC requests:

```bash
curl -X POST http://localhost:3000/mcp/request \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "id": "1",
    "method": "tools/list",
    "params": {}
  }'
```

#### GET /mcp/events

Subscribe to Server-Sent Events:

```bash
curl -N http://localhost:3000/mcp/events
```

#### Configuration

```rust
use mcp_protocol_sdk::transport::http::{HttpServerConfig, HttpClientConfig};

// Server configuration
let server_config = HttpServerConfig {
    bind_address: "0.0.0.0:3000".to_string(),
    max_connections: 1000,
    request_timeout_ms: 30_000,
    keep_alive_timeout_ms: 60_000,
    max_request_size: 1024 * 1024, // 1MB
    cors_enabled: true,
    cors_origins: vec!["https://myapp.com".to_string()],
    compression: true,
    headers: [
        ("Server".to_string(), "MCP-Rust-SDK/1.0".to_string()),
    ].into(),
};

let transport = HttpServerTransport::with_config(server_config);

// Client configuration
let client_config = HttpClientConfig {
    base_url: "http://localhost:3000/mcp".to_string(),
    timeout_ms: 30_000,
    max_retries: 3,
    retry_delay_ms: 1000,
    headers: [
        ("User-Agent".to_string(), "MyApp/1.0".to_string()),
        ("Authorization".to_string(), "Bearer token123".to_string()),
    ].into(),
    compression: true,
    follow_redirects: true,
};

let transport = HttpClientTransport::with_config(client_config).await?;
```

## WebSocket Transport

WebSocket transport provides full-duplex, real-time communication.

### Server

```rust
use mcp_protocol_sdk::{
    server::McpServer,
    transport::websocket::WebSocketServerTransport,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("ws-server".to_string(), "1.0.0".to_string());
    
    // Add your tools, resources, and prompts
    
    let transport = WebSocketServerTransport::new("0.0.0.0:8080");
    server.start(transport).await?;
    
    println!("WebSocket server running on ws://localhost:8080");
    
    // Keep running until interrupted
    tokio::signal::ctrl_c().await?;
    server.stop().await?;
    
    Ok(())
}
```

### Client

```rust
use mcp_protocol_sdk::{
    client::{McpClient, ClientSession},
    transport::websocket::WebSocketClientTransport,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = McpClient::new("ws-client".to_string(), "1.0.0".to_string());
    let session = ClientSession::new(client);
    
    let transport = WebSocketClientTransport::new("ws://localhost:8080").await?;
    let init_result = session.connect(transport).await?;
    
    // Use the client...
    
    Ok(())
}
```

### Configuration

```rust
use mcp_protocol_sdk::transport::websocket::{WebSocketServerConfig, WebSocketClientConfig};

// Server configuration
let server_config = WebSocketServerConfig {
    bind_address: "0.0.0.0:8080".to_string(),
    max_connections: 10000,
    max_frame_size: 64 * 1024 * 1024, // 64MB
    max_message_size: 16 * 1024 * 1024, // 16MB
    compression: true,
    ping_interval_ms: Some(30_000),
    pong_timeout_ms: 10_000,
    headers: [
        ("Server".to_string(), "MCP-Rust-SDK/1.0".to_string()),
    ].into(),
};

let transport = WebSocketServerTransport::with_config(server_config);

// Client configuration
let client_config = WebSocketClientConfig {
    url: "ws://localhost:8080".to_string(),
    timeout_ms: 30_000,
    max_retries: 5,
    retry_delay_ms: 2000,
    max_frame_size: 64 * 1024 * 1024, // 64MB
    max_message_size: 16 * 1024 * 1024, // 16MB
    compression: true,
    ping_interval_ms: Some(30_000),
    headers: [
        ("User-Agent".to_string(), "MyApp/1.0".to_string()),
    ].into(),
    subprotocols: vec!["mcp".to_string()],
};

let transport = WebSocketClientTransport::with_config(client_config).await?;
```

## Transport Abstractions

### Custom Transport Implementation

You can implement custom transports by implementing the transport traits:

```rust
use mcp_protocol_sdk::transport::traits::{Transport, ServerTransport, ClientTransport};
use async_trait::async_trait;
use tokio::sync::mpsc;

struct CustomTransport {
    sender: mpsc::UnboundedSender<String>,
    receiver: mpsc::UnboundedReceiver<String>,
}

#[async_trait]
impl Transport for CustomTransport {
    async fn send(&mut self, message: String) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.sender.send(message)?;
        Ok(())
    }

    async fn receive(&mut self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        match self.receiver.recv().await {
            Some(message) => Ok(message),
            None => Err("Channel closed".into()),
        }
    }

    async fn close(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Cleanup logic
        Ok(())
    }
}

#[async_trait]
impl ServerTransport for CustomTransport {
    async fn accept_connection(&mut self) -> Result<Box<dyn Transport + Send>, Box<dyn std::error::Error + Send + Sync>> {
        // Create new connection instance
        let (tx, rx) = mpsc::unbounded_channel();
        Ok(Box::new(CustomTransport {
            sender: tx,
            receiver: rx,
        }))
    }
}

#[async_trait]
impl ClientTransport for CustomTransport {
    async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Connection logic
        Ok(())
    }
}
```

## Transport Selection Guide

### Choose STDIO When:
- Building CLI tools or command-line integrations
- Single client per server instance
- Need lowest possible latency
- Working with local processes
- Simple deployment requirements

### Choose HTTP When:
- Building web applications
- Need to support multiple concurrent clients
- Want RESTful API compatibility
- Need to work through firewalls/proxies
- Building stateless services
- Need to integrate with existing HTTP infrastructure

### Choose WebSocket When:
- Building real-time applications
- Need bidirectional communication
- Want low latency with multiple clients
- Building interactive applications (games, chat, live dashboards)
- Need to push notifications to clients
- Building collaborative applications

## Performance Comparison

### Latency (Local Network)

| Transport | Avg Latency | 95th Percentile |
|-----------|-------------|-----------------|
| STDIO | 0.1ms | 0.3ms |
| WebSocket | 0.5ms | 1.2ms |
| HTTP | 2.1ms | 5.8ms |

### Throughput (Messages/Second)

| Transport | Single Client | 10 Clients | 100 Clients |
|-----------|---------------|------------|-------------|
| STDIO | 50,000 | N/A | N/A |
| WebSocket | 25,000 | 15,000 | 8,000 |
| HTTP | 5,000 | 4,000 | 2,500 |

### Memory Usage (Per Connection)

| Transport | Memory Overhead |
|-----------|-----------------|
| STDIO | 32KB |
| WebSocket | 64KB |
| HTTP | 128KB |

## Best Practices

### Connection Management

```rust
use mcp_protocol_sdk::client::session::{SessionConfig, SessionEventHandler};
use async_trait::async_trait;

struct MyEventHandler;

#[async_trait]
impl SessionEventHandler for MyEventHandler {
    async fn on_connected(&self) {
        println!("Connected to server");
    }

    async fn on_disconnected(&self, reason: &str) {
        println!("Disconnected: {}", reason);
    }

    async fn on_reconnecting(&self, attempt: u32) {
        println!("Reconnecting... (attempt {})", attempt);
    }

    async fn on_error(&self, error: &str) {
        eprintln!("Session error: {}", error);
    }
}

let session_config = SessionConfig {
    auto_reconnect: true,
    max_reconnect_attempts: 10,
    reconnect_delay_ms: 2000,
    connection_timeout_ms: 15000,
    heartbeat_interval_ms: 30000,
    event_handler: Some(Box::new(MyEventHandler)),
    ..Default::default()
};
```

### Error Handling

```rust
use mcp_protocol_sdk::core::error::McpError;

async fn handle_transport_errors(session: &ClientSession) {
    loop {
        let client = session.client();
        let client_guard = client.lock().await;
        
        match client_guard.list_tools().await {
            Ok(tools) => {
                // Process tools
                println!("Tools available: {}", tools.tools.len());
            }
            Err(McpError::Transport(msg)) => {
                eprintln!("Transport error: {}", msg);
                // Session will auto-reconnect if configured
                tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
            }
            Err(McpError::Timeout) => {
                eprintln!("Request timed out");
                // Retry or handle gracefully
            }
            Err(e) => {
                eprintln!("Other error: {}", e);
                break;
            }
        }
        
        drop(client_guard);
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}
```

### Security Considerations

#### HTTP Transport Security

```rust
use mcp_protocol_sdk::transport::http::HttpServerConfig;

let secure_config = HttpServerConfig {
    bind_address: "0.0.0.0:443".to_string(),
    tls_cert_path: Some("/path/to/cert.pem".to_string()),
    tls_key_path: Some("/path/to/key.pem".to_string()),
    cors_enabled: true,
    cors_origins: vec!["https://trusted-domain.com".to_string()],
    max_request_size: 1024 * 1024, // 1MB limit
    rate_limit_requests_per_minute: Some(100),
    auth_required: true,
    allowed_headers: vec![
        "Authorization".to_string(),
        "Content-Type".to_string(),
    ],
    ..Default::default()
};
```

#### WebSocket Security

```rust
use mcp_protocol_sdk::transport::websocket::WebSocketServerConfig;

let secure_config = WebSocketServerConfig {
    bind_address: "0.0.0.0:8443".to_string(),
    tls_cert_path: Some("/path/to/cert.pem".to_string()),
    tls_key_path: Some("/path/to/key.pem".to_string()),
    max_connections: 1000,
    max_frame_size: 1024 * 1024, // 1MB
    max_message_size: 1024 * 1024, // 1MB
    origin_check: Some(vec!["https://trusted-domain.com".to_string()]),
    subprotocol_check: Some(vec!["mcp".to_string()]),
    auth_required: true,
    ..Default::default()
};
```

## Monitoring and Observability

### Transport Metrics

```rust
use mcp_protocol_sdk::transport::metrics::{TransportMetrics, MetricsCollector};

struct MyMetricsCollector;

impl MetricsCollector for MyMetricsCollector {
    fn record_connection(&self, transport_type: &str) {
        println!("New {} connection", transport_type);
    }

    fn record_disconnection(&self, transport_type: &str, duration_ms: u64) {
        println!("{} connection closed after {}ms", transport_type, duration_ms);
    }

    fn record_message_sent(&self, transport_type: &str, size_bytes: usize) {
        println!("Sent {} bytes via {}", size_bytes, transport_type);
    }

    fn record_message_received(&self, transport_type: &str, size_bytes: usize) {
        println!("Received {} bytes via {}", size_bytes, transport_type);
    }

    fn record_error(&self, transport_type: &str, error: &str) {
        eprintln!("{} error: {}", transport_type, error);
    }
}

// Enable metrics collection
let transport = HttpServerTransport::with_metrics(
    config,
    Box::new(MyMetricsCollector),
);
```

### Health Checks

```rust
use mcp_protocol_sdk::server::health::{HealthCheck, HealthStatus};

async fn check_transport_health(server: &McpServer) -> HealthStatus {
    match server.get_connection_count().await {
        Ok(count) if count > 0 => HealthStatus::Healthy,
        Ok(_) => HealthStatus::Warning("No active connections".to_string()),
        Err(e) => HealthStatus::Unhealthy(format!("Transport error: {}", e)),
    }
}
```

## Migration Between Transports

### STDIO to HTTP

```rust
// Before (STDIO)
let transport = StdioServerTransport::new();

// After (HTTP)
let transport = HttpServerTransport::new("0.0.0.0:3000");
```

### HTTP to WebSocket

```rust
// Before (HTTP)
let transport = HttpClientTransport::new("http://localhost:3000/mcp").await?;

// After (WebSocket)
let transport = WebSocketClientTransport::new("ws://localhost:8080").await?;
```

## Next Steps

- [Architecture Guide](architecture.md) - Understanding the system design
- [Examples](examples.md) - Real-world usage examples
- [API Reference](https://docs.rs/mcp-protocol-sdk) - Complete API documentation
