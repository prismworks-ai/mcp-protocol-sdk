# Architecture Overview

MCP Rust SDK is designed with a modular, extensible architecture that separates concerns and provides clear interfaces for different components.

## High-Level Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                    Application Layer                        │
├─────────────────────────────────────────────────────────────┤
│                  MCP Server / Client                       │
├─────────────────────────────────────────────────────────────┤
│              Protocol Layer (JSON-RPC 2.0)                │
├─────────────────────────────────────────────────────────────┤
│                   Transport Layer                          │
│             (STDIO / HTTP / WebSocket)                     │
└─────────────────────────────────────────────────────────────┘
```

## Core Components

### 1. Protocol Layer

The protocol layer implements the MCP specification using JSON-RPC 2.0.

```rust
// Core protocol types
pub struct JsonRpcRequest {
    pub jsonrpc: String,
    pub id: RequestId,
    pub method: String,
    pub params: Option<serde_json::Value>,
}

pub struct JsonRpcResponse {
    pub jsonrpc: String,
    pub id: RequestId,
    pub result: Option<serde_json::Value>,
    pub error: Option<JsonRpcError>,
}

// MCP-specific message types
pub enum McpRequest {
    Initialize(InitializeRequest),
    ListTools(ListToolsRequest),
    CallTool(CallToolRequest),
    ListResources(ListResourcesRequest),
    ReadResource(ReadResourceRequest),
    ListPrompts(ListPromptsRequest),
    GetPrompt(GetPromptRequest),
}
```

### 2. Transport Layer

The transport layer provides pluggable communication mechanisms.

```rust
#[async_trait]
pub trait Transport: Send + Sync {
    async fn send(&mut self, message: String) -> Result<(), TransportError>;
    async fn receive(&mut self) -> Result<String, TransportError>;
    async fn close(&mut self) -> Result<(), TransportError>;
}

#[async_trait]
pub trait ServerTransport: Transport {
    async fn accept_connection(&mut self) -> Result<Box<dyn Transport + Send>, TransportError>;
}

#[async_trait]
pub trait ClientTransport: Transport {
    async fn connect(&mut self) -> Result<(), TransportError>;
}
```

### 3. Server Architecture

```rust
pub struct McpServer {
    info: ServerInfo,
    tools: HashMap<String, Box<dyn ToolHandler + Send + Sync>>,
    resources: HashMap<String, Box<dyn ResourceHandler + Send + Sync>>,
    prompts: HashMap<String, Box<dyn PromptHandler + Send + Sync>>,
    transport: Option<Box<dyn ServerTransport + Send>>,
    connection_manager: ConnectionManager,
    request_handler: RequestHandler,
}
```

#### Server Component Diagram

```
┌─────────────────────────────────────────────────┐
│                  McpServer                      │
├─────────────────────────────────────────────────┤
│  ┌─────────────┐  ┌─────────────┐  ┌──────────┐ │
│  │ Tool        │  │ Resource    │  │ Prompt   │ │
│  │ Registry    │  │ Registry    │  │ Registry │ │
│  └─────────────┘  └─────────────┘  └──────────┘ │
├─────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────┐ │
│  │         Request Handler                     │ │
│  └─────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────┐ │
│  │       Connection Manager                    │ │
│  └─────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────┐ │
│  │         Transport Layer                     │ │
│  └─────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────┘
```

### 4. Client Architecture

```rust
pub struct McpClient {
    info: ClientInfo,
    transport: Option<Box<dyn ClientTransport + Send>>,
    request_manager: RequestManager,
    notification_handler: NotificationHandler,
}

pub struct ClientSession {
    client: Arc<Mutex<McpClient>>,
    config: SessionConfig,
    state: Arc<Mutex<SessionState>>,
    reconnect_handler: ReconnectHandler,
}
```

#### Client Component Diagram

```
┌─────────────────────────────────────────────────┐
│                ClientSession                    │
├─────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────┐ │
│  │          McpClient                          │ │
│  │  ┌─────────────┐  ┌─────────────────────┐   │ │
│  │  │ Request     │  │ Notification        │   │ │
│  │  │ Manager     │  │ Handler             │   │ │
│  │  └─────────────┘  └─────────────────────┘   │ │
│  └─────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────┐ │
│  │       Reconnection Handler                  │ │
│  └─────────────────────────────────────────────┘ │
├─────────────────────────────────────────────────┤
│  ┌─────────────────────────────────────────────┐ │
│  │         Transport Layer                     │ │
│  └─────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────┘
```

## Data Flow

### Server Request Processing

```
1. Transport receives raw message
   ↓
2. Protocol layer deserializes JSON-RPC
   ↓
3. Request router determines handler type
   ↓
4. Handler executes business logic
   ↓
5. Response serialized to JSON-RPC
   ↓
6. Transport sends response
```

```rust
async fn handle_request(&self, raw_message: String) -> Result<String, McpError> {
    // 1. Parse JSON-RPC message
    let request: JsonRpcRequest = serde_json::from_str(&raw_message)?;
    
    // 2. Route to appropriate handler
    let response = match request.method.as_str() {
        "tools/list" => self.handle_list_tools(request).await?,
        "tools/call" => self.handle_call_tool(request).await?,
        "resources/list" => self.handle_list_resources(request).await?,
        "resources/read" => self.handle_read_resource(request).await?,
        "prompts/list" => self.handle_list_prompts(request).await?,
        "prompts/get" => self.handle_get_prompt(request).await?,
        _ => return Err(McpError::MethodNotFound(request.method)),
    };
    
    // 3. Serialize response
    Ok(serde_json::to_string(&response)?)
}
```

### Client Request Flow

```
1. Application calls client method
   ↓
2. Request manager generates JSON-RPC request
   ↓
3. Transport sends request
   ↓
4. Response received and parsed
   ↓
5. Result returned to application
```

```rust
async fn call_tool(&self, name: String, args: Option<HashMap<String, Value>>) -> Result<ToolResult, McpError> {
    // 1. Generate request ID
    let request_id = self.request_manager.generate_id();
    
    // 2. Create JSON-RPC request
    let request = JsonRpcRequest {
        jsonrpc: "2.0".to_string(),
        id: request_id.clone(),
        method: "tools/call".to_string(),
        params: Some(json!({
            "name": name,
            "arguments": args
        })),
    };
    
    // 3. Send request and wait for response
    let response = self.request_manager.send_request(request).await?;
    
    // 4. Parse and return result
    let tool_result: ToolResult = serde_json::from_value(response.result.unwrap())?;
    Ok(tool_result)
}
```

## Error Handling Strategy

### Error Type Hierarchy

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

### Error Recovery

```rust
impl McpServer {
    async fn handle_request_with_recovery(&self, raw_message: String) -> String {
        match self.handle_request(raw_message).await {
            Ok(response) => response,
            Err(McpError::ValidationError(msg)) => {
                self.create_error_response("INVALID_PARAMS", &msg)
            }
            Err(McpError::ToolNotFound(tool)) => {
                self.create_error_response("METHOD_NOT_FOUND", &format!("Tool not found: {}", tool))
            }
            Err(McpError::Timeout) => {
                self.create_error_response("TIMEOUT", "Request timed out")
            }
            Err(e) => {
                self.create_error_response("INTERNAL_ERROR", &e.to_string())
            }
        }
    }
}
```

## Concurrency Model

### Server Concurrency

```rust
impl McpServer {
    pub async fn start<T: ServerTransport + Send + 'static>(&mut self, transport: T) -> Result<(), McpError> {
        let transport = Arc::new(Mutex::new(transport));
        
        loop {
            // Accept new connections
            let connection = transport.lock().await.accept_connection().await?;
            let server_handle = Arc::clone(&self.handler);
            
            // Spawn task for each connection
            tokio::spawn(async move {
                Self::handle_connection(connection, server_handle).await;
            });
        }
    }
    
    async fn handle_connection(
        mut connection: Box<dyn Transport + Send>,
        handler: Arc<dyn RequestHandler + Send + Sync>
    ) {
        loop {
            match connection.receive().await {
                Ok(message) => {
                    let handler = Arc::clone(&handler);
                    
                    // Process each request concurrently
                    tokio::spawn(async move {
                        let response = handler.handle_request(message).await;
                        // Send response...
                    });
                }
                Err(_) => break, // Connection closed
            }
        }
    }
}
```

### Client Concurrency

```rust
impl ClientSession {
    async fn start_background_tasks(&self) {
        let client = Arc::clone(&self.client);
        let config = self.config.clone();
        
        // Heartbeat task
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(
                Duration::from_millis(config.heartbeat_interval_ms)
            );
            
            loop {
                interval.tick().await;
                if let Ok(client) = client.try_lock() {
                    let _ = client.send_ping().await;
                }
            }
        });
        
        // Reconnection task
        let client = Arc::clone(&self.client);
        tokio::spawn(async move {
            // Reconnection logic...
        });
    }
}
```

## Memory Management

### Resource Cleanup

```rust
impl Drop for McpServer {
    fn drop(&mut self) {
        // Gracefully close all connections
        if let Some(transport) = &mut self.transport {
            let _ = futures::executor::block_on(transport.close());
        }
        
        // Clean up handlers
        self.tools.clear();
        self.resources.clear();
        self.prompts.clear();
    }
}
```

### Connection Pooling

```rust
pub struct ConnectionPool {
    connections: Vec<Box<dyn Transport + Send>>,
    max_size: usize,
    current_size: AtomicUsize,
}

impl ConnectionPool {
    pub async fn get_connection(&self) -> Result<Box<dyn Transport + Send>, McpError> {
        // Try to reuse existing connection
        if let Some(conn) = self.try_get_pooled_connection() {
            return Ok(conn);
        }
        
        // Create new connection if under limit
        if self.current_size.load(Ordering::Relaxed) < self.max_size {
            let conn = self.create_new_connection().await?;
            self.current_size.fetch_add(1, Ordering::Relaxed);
            return Ok(conn);
        }
        
        Err(McpError::Transport("Connection pool exhausted".to_string()))
    }
}
```

## Security Considerations

### Input Validation

```rust
impl ToolHandler for SecureCalculatorTool {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, McpError> {
        // Validate input parameters
        let a = arguments.get("a")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| McpError::ValidationError("Invalid parameter 'a'".to_string()))?;
        
        let b = arguments.get("b")
            .and_then(|v| v.as_f64())
            .ok_or_else(|| McpError::ValidationError("Invalid parameter 'b'".to_string()))?;
        
        // Validate ranges
        if a.abs() > 1e10 || b.abs() > 1e10 {
            return Err(McpError::ValidationError("Numbers too large".to_string()));
        }
        
        // Safe calculation
        let result = a + b;
        
        Ok(ToolResult {
            content: vec![Content::text(result.to_string())],
            is_error: None,
        })
    }
}
```

### Rate Limiting

```rust
use std::time::{Duration, Instant};
use std::collections::HashMap;

pub struct RateLimiter {
    requests: HashMap<String, Vec<Instant>>,
    max_requests: usize,
    window_duration: Duration,
}

impl RateLimiter {
    pub fn check_rate_limit(&mut self, client_id: &str) -> bool {
        let now = Instant::now();
        let requests = self.requests.entry(client_id.to_string()).or_insert_with(Vec::new);
        
        // Remove old requests outside the window
        requests.retain(|&time| now.duration_since(time) < self.window_duration);
        
        // Check if under limit
        if requests.len() < self.max_requests {
            requests.push(now);
            true
        } else {
            false
        }
    }
}
```

## Performance Optimizations

### Message Batching

```rust
pub struct BatchProcessor {
    pending_requests: Vec<JsonRpcRequest>,
    batch_size: usize,
    flush_interval: Duration,
}

impl BatchProcessor {
    pub async fn add_request(&mut self, request: JsonRpcRequest) {
        self.pending_requests.push(request);
        
        if self.pending_requests.len() >= self.batch_size {
            self.flush_batch().await;
        }
    }
    
    async fn flush_batch(&mut self) {
        if !self.pending_requests.is_empty() {
            let batch = std::mem::take(&mut self.pending_requests);
            self.process_batch(batch).await;
        }
    }
}
```

### Connection Multiplexing

```rust
pub struct MultiplexedTransport {
    connection: Arc<Mutex<Box<dyn Transport + Send>>>,
    channels: HashMap<u32, mpsc::UnboundedSender<String>>,
    next_channel_id: AtomicU32,
}

impl MultiplexedTransport {
    pub async fn create_channel(&self) -> u32 {
        let channel_id = self.next_channel_id.fetch_add(1, Ordering::Relaxed);
        let (tx, rx) = mpsc::unbounded_channel();
        
        self.channels.lock().await.insert(channel_id, tx);
        
        // Spawn channel handler
        tokio::spawn(async move {
            // Handle channel messages...
        });
        
        channel_id
    }
}
```

## Testing Strategy

### Unit Testing

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio_test;

    #[tokio::test]
    async fn test_tool_handler() {
        let tool = EchoTool;
        let mut args = HashMap::new();
        args.insert("message".to_string(), json!("test"));
        
        let result = tool.call(args).await.unwrap();
        assert_eq!(result.content[0].text, Some("Echo: test".to_string()));
    }
    
    #[tokio::test]
    async fn test_error_handling() {
        let tool = CalculatorTool;
        let mut args = HashMap::new();
        args.insert("a".to_string(), json!(1.0));
        args.insert("b".to_string(), json!(0.0));
        args.insert("operation".to_string(), json!("divide"));
        
        let result = tool.call(args).await;
        assert!(matches!(result, Err(McpError::ToolExecutionError(_))));
    }
}
```

### Integration Testing

```rust
#[tokio::test]
async fn test_client_server_integration() {
    // Start server
    let mut server = McpServer::new("test-server".to_string(), "1.0.0".to_string());
    server.add_tool("echo".to_string(), None, json!({}), EchoTool).await.unwrap();
    
    let (server_tx, server_rx) = mpsc::unbounded_channel();
    let (client_tx, client_rx) = mpsc::unbounded_channel();
    
    let server_transport = MockTransport::new(server_rx, client_tx);
    let client_transport = MockTransport::new(client_rx, server_tx);
    
    // Start server in background
    tokio::spawn(async move {
        server.start(server_transport).await.unwrap();
    });
    
    // Test client
    let client = McpClient::new("test-client".to_string(), "1.0.0".to_string());
    let session = ClientSession::new(client);
    
    session.connect(client_transport).await.unwrap();
    
    let result = session.client().lock().await
        .call_tool("echo".to_string(), Some([("message".to_string(), json!("test"))].into()))
        .await.unwrap();
    
    assert!(result.content[0].text.as_ref().unwrap().contains("test"));
}
```

## Monitoring and Observability

### Metrics Collection

```rust
use prometheus::{Counter, Histogram, Gauge};

pub struct McpMetrics {
    requests_total: Counter,
    request_duration: Histogram,
    active_connections: Gauge,
    errors_total: Counter,
}

impl McpMetrics {
    pub fn record_request(&self, method: &str, duration: Duration) {
        self.requests_total.with_label_values(&[method]).inc();
        self.request_duration.observe(duration.as_secs_f64());
    }
    
    pub fn record_connection(&self, delta: i64) {
        if delta > 0 {
            self.active_connections.inc();
        } else {
            self.active_connections.dec();
        }
    }
    
    pub fn record_error(&self, error_type: &str) {
        self.errors_total.with_label_values(&[error_type]).inc();
    }
}
```

### Distributed Tracing

```rust
use tracing::{info, error, instrument};

impl McpServer {
    #[instrument(skip(self, request))]
    async fn handle_request(&self, request: JsonRpcRequest) -> Result<JsonRpcResponse, McpError> {
        info!("Processing request: method={}", request.method);
        
        let start = Instant::now();
        let result = self.process_request(request).await;
        let duration = start.elapsed();
        
        match &result {
            Ok(_) => info!("Request completed successfully in {:?}", duration),
            Err(e) => error!("Request failed: {} (took {:?})", e, duration),
        }
        
        result
    }
}
```

## Extension Points

### Custom Tool Types

```rust
pub trait ToolHandler: Send + Sync {
    async fn call(&self, arguments: HashMap<String, Value>) -> Result<ToolResult, McpError>;
    
    // Optional methods for advanced features
    async fn validate_arguments(&self, arguments: &HashMap<String, Value>) -> Result<(), McpError> {
        Ok(()) // Default: no validation
    }
    
    async fn get_schema(&self) -> Result<Value, McpError> {
        Ok(json!({})) // Default: empty schema
    }
    
    async fn get_examples(&self) -> Result<Vec<ToolExample>, McpError> {
        Ok(vec![]) // Default: no examples
    }
}
```

### Plugin System

```rust
pub trait Plugin: Send + Sync {
    fn name(&self) -> &str;
    fn version(&self) -> &str;
    
    async fn initialize(&self, server: &mut McpServer) -> Result<(), McpError>;
    async fn shutdown(&self) -> Result<(), McpError>;
}

impl McpServer {
    pub async fn load_plugin(&mut self, plugin: Box<dyn Plugin>) -> Result<(), McpError> {
        plugin.initialize(self).await?;
        self.plugins.insert(plugin.name().to_string(), plugin);
        Ok(())
    }
}
```

## Next Steps

- [Getting Started](getting-started.md) - Build your first MCP application
- [Examples](examples.md) - Real-world usage examples  
- [Transport Guide](transports.md) - Deep dive into transport options
- [API Reference](https://docs.rs/mcp-protocol-sdk) - Complete API documentation
