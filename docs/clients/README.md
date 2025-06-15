# 💻 MCP Client Development Guide

Build robust MCP clients to connect and interact with MCP servers, enabling AI models to access external capabilities.

## What is an MCP Client?

An MCP client connects to MCP servers to:
- Discover available tools, resources, and prompts
- Execute tool calls on behalf of AI models
- Retrieve resources and content
- Manage server connections and sessions

## Core Client Patterns

### 1. Basic Client Setup

```rust
use mcp_protocol_sdk::prelude::*;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Create client
    let client = McpClient::new()
        .with_name("my-ai-client")
        .with_version("1.0.0")
        .with_timeout(Duration::from_secs(30))
        .build();
    
    // Connect to server
    let transport = StdioClientTransport::new();
    client.connect(transport).await?;
    
    // Initialize session
    client.initialize().await?;
    
    // Use the client...
    interact_with_server(&client).await?;
    
    Ok(())
}
```

### 2. Server Discovery & Capabilities

```rust
async fn discover_capabilities(client: &McpClient) -> Result<(), Box<dyn std::error::Error>> {
    // Get server info
    let server_info = client.get_server_info().await?;
    println!("Connected to: {} v{}", server_info.name, server_info.version);
    
    // List available tools
    let tools = client.list_tools().await?;
    println!("Available tools:");
    for tool in &tools {
        println!("  - {}: {}", tool.name, tool.description);
        for param in &tool.parameters {
            println!("    {} ({}): {}", 
                param.name, 
                if param.required { "required" } else { "optional" },
                param.description
            );
        }
    }
    
    // List available resources
    let resources = client.list_resources().await?;
    println!("Available resources:");
    for resource in &resources {
        println!("  - {}: {}", resource.uri, resource.description);
    }
    
    // List available prompts
    let prompts = client.list_prompts().await?;
    println!("Available prompts:");
    for prompt in &prompts {
        println!("  - {}: {}", prompt.name, prompt.description);
    }
    
    Ok(())
}
```

### 3. Tool Execution

```rust
async fn execute_tools(client: &McpClient) -> Result<(), Box<dyn std::error::Error>> {
    // Simple tool call
    let result = client.call_tool(
        "calculate",
        json!({
            "expression": "10 + 5 * 2",
            "precision": 2
        })
    ).await?;
    
    println!("Calculation result: {}", result);
    
    // Batch tool calls
    let batch_results = client.call_tools_batch(vec![
        ToolCall::new("read_file", json!({"path": "config.json"})),
        ToolCall::new("read_file", json!({"path": "data.csv"})),
        ToolCall::new("calculate", json!({"expression": "100 / 4"})),
    ]).await?;
    
    for (i, result) in batch_results.iter().enumerate() {
        println!("Batch result {}: {:?}", i, result);
    }
    
    Ok(())
}
```

### 4. Resource Access

```rust
async fn access_resources(client: &McpClient) -> Result<(), Box<dyn std::error::Error>> {
    // Read a specific resource
    let content = client.read_resource("file://config.json").await?;
    match content {
        ResourceContent::Text(text) => {
            println!("Config content: {}", text);
        }
        ResourceContent::Binary(data) => {
            println!("Binary data, {} bytes", data.len());
        }
    }
    
    // Read multiple resources
    let resources = vec![
        "file://config.json",
        "database://users/1",
        "api://weather/current"
    ];
    
    for uri in resources {
        match client.read_resource(uri).await {
            Ok(content) => println!("Successfully read: {}", uri),
            Err(e) => println!("Failed to read {}: {}", uri, e),
        }
    }
    
    Ok(())
}
```

### 5. Prompt Management

```rust
async fn work_with_prompts(client: &McpClient) -> Result<(), Box<dyn std::error::Error>> {
    // Get a prompt with parameters
    let prompt = client.get_prompt(
        "code_review",
        json!({
            "code": "fn main() { println!(\"Hello\"); }",
            "language": "rust",
            "style": "thorough"
        })
    ).await?;
    
    println!("Generated prompt: {}", prompt.content);
    
    // Use prompt in AI conversation
    let ai_response = send_to_ai_model(&prompt).await?;
    println!("AI response: {}", ai_response);
    
    Ok(())
}
```

## Connection Management

### Multiple Server Connections

```rust
use std::collections::HashMap;

struct MultiServerClient {
    clients: HashMap<String, McpClient>,
}

impl MultiServerClient {
    pub async fn new() -> Self {
        Self {
            clients: HashMap::new(),
        }
    }
    
    pub async fn connect_server(
        &mut self, 
        name: &str, 
        transport: impl Transport
    ) -> Result<(), Box<dyn std::error::Error>> {
        let client = McpClient::new()
            .with_name("multi-client")
            .build();
            
        client.connect(transport).await?;
        client.initialize().await?;
        
        self.clients.insert(name.to_string(), client);
        Ok(())
    }
    
    pub async fn call_tool_on_server(
        &self,
        server_name: &str,
        tool_name: &str,
        params: Value
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let client = self.clients.get(server_name)
            .ok_or(format!("Server {} not connected", server_name))?;
            
        client.call_tool(tool_name, params).await
    }
}
```

### Connection Resilience

```rust
async fn resilient_client_connection() -> Result<(), Box<dyn std::error::Error>> {
    let client = McpClient::new()
        .with_retry_config(RetryConfig {
            max_retries: 3,
            initial_delay: Duration::from_millis(100),
            max_delay: Duration::from_secs(5),
            backoff_multiplier: 2.0,
        })
        .with_heartbeat_interval(Duration::from_secs(30))
        .build();
    
    // Auto-reconnection on failure
    client.set_connection_error_handler(|error| async move {
        eprintln!("Connection lost: {}. Attempting reconnection...", error);
        // Custom reconnection logic
    });
    
    // Monitor connection health
    client.set_health_check_handler(|| async move {
        // Custom health check logic
        Ok(true)
    });
    
    Ok(())
}
```

## Transport Selection

### STDIO Client (Connect to STDIO servers)

```rust
use mcp_protocol_sdk::transport::StdioClientTransport;

// Connect to external process
let transport = StdioClientTransport::new_process("./my-server", &["--config", "prod.json"])?;

// Or use existing stdio streams
let transport = StdioClientTransport::new();
```

### HTTP Client

```rust
use mcp_protocol_sdk::transport::HttpClientTransport;

let transport = HttpClientTransport::new("http://localhost:3000/mcp")
    .with_timeout(Duration::from_secs(30))
    .with_auth_token("your-api-key")
    .with_user_agent("MyClient/1.0");
```

### WebSocket Client

```rust
use mcp_protocol_sdk::transport::WebSocketClientTransport;

let transport = WebSocketClientTransport::new("ws://localhost:8080/mcp")
    .with_heartbeat_interval(Duration::from_secs(30))
    .with_max_reconnect_attempts(5);
```

## Advanced Client Patterns

### Request Batching

```rust
struct BatchingClient {
    client: McpClient,
    batch_size: usize,
    pending_requests: Vec<PendingRequest>,
}

impl BatchingClient {
    pub async fn queue_tool_call(&mut self, tool: &str, params: Value) -> RequestId {
        let request = PendingRequest::new(tool, params);
        let id = request.id();
        
        self.pending_requests.push(request);
        
        if self.pending_requests.len() >= self.batch_size {
            self.flush_batch().await.unwrap();
        }
        
        id
    }
    
    async fn flush_batch(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if self.pending_requests.is_empty() {
            return Ok(());
        }
        
        let batch: Vec<ToolCall> = self.pending_requests
            .drain(..)
            .map(|req| ToolCall::new(&req.tool, req.params))
            .collect();
            
        let results = self.client.call_tools_batch(batch).await?;
        
        // Process results...
        
        Ok(())
    }
}
```

### Caching Layer

```rust
use std::time::{Duration, Instant};
use std::collections::HashMap;

struct CachingClient {
    client: McpClient,
    cache: HashMap<String, CacheEntry>,
    default_ttl: Duration,
}

struct CacheEntry {
    value: Value,
    expires_at: Instant,
}

impl CachingClient {
    pub async fn call_tool_cached(
        &mut self,
        tool: &str,
        params: Value,
        ttl: Option<Duration>
    ) -> Result<Value, Box<dyn std::error::Error>> {
        let cache_key = format!("{}:{}", tool, serde_json::to_string(&params)?);
        
        // Check cache
        if let Some(entry) = self.cache.get(&cache_key) {
            if entry.expires_at > Instant::now() {
                return Ok(entry.value.clone());
            }
        }
        
        // Call server
        let result = self.client.call_tool(tool, params).await?;
        
        // Cache result
        self.cache.insert(cache_key, CacheEntry {
            value: result.clone(),
            expires_at: Instant::now() + ttl.unwrap_or(self.default_ttl),
        });
        
        Ok(result)
    }
}
```

### Metrics & Monitoring

```rust
use std::sync::atomic::{AtomicU64, Ordering};

struct InstrumentedClient {
    client: McpClient,
    tool_calls: AtomicU64,
    resource_reads: AtomicU64,
    errors: AtomicU64,
}

impl InstrumentedClient {
    pub async fn call_tool(&self, tool: &str, params: Value) -> Result<Value, Box<dyn std::error::Error>> {
        let start = Instant::now();
        
        match self.client.call_tool(tool, params).await {
            Ok(result) => {
                self.tool_calls.fetch_add(1, Ordering::Relaxed);
                
                // Record metrics
                record_tool_call_duration(tool, start.elapsed());
                record_tool_call_success(tool);
                
                Ok(result)
            }
            Err(e) => {
                self.errors.fetch_add(1, Ordering::Relaxed);
                record_tool_call_error(tool, &e);
                Err(e)
            }
        }
    }
    
    pub fn get_metrics(&self) -> ClientMetrics {
        ClientMetrics {
            tool_calls: self.tool_calls.load(Ordering::Relaxed),
            resource_reads: self.resource_reads.load(Ordering::Relaxed),
            errors: self.errors.load(Ordering::Relaxed),
        }
    }
}
```

## Integration Examples

### AI Model Integration

```rust
async fn ai_model_integration() -> Result<(), Box<dyn std::error::Error>> {
    let client = setup_mcp_client().await?;
    
    // AI model requests tool use
    let ai_request = "Please read the file config.json and calculate the sum of all numeric values";
    
    // Parse AI request and map to MCP calls
    let tools = client.list_tools().await?;
    let suitable_tools = find_suitable_tools(&tools, &ai_request);
    
    // Execute tools
    for tool in suitable_tools {
        let result = client.call_tool(&tool.name, tool.params).await?;
        
        // Send result back to AI model
        send_result_to_ai(&result).await?;
    }
    
    Ok(())
}
```

### Web Application Integration

```rust
use axum::{extract::State, http::StatusCode, response::Json, routing::post, Router};

#[derive(Clone)]
struct AppState {
    mcp_client: Arc<McpClient>,
}

async fn execute_tool(
    State(state): State<AppState>,
    Json(payload): Json<ToolRequest>
) -> Result<Json<Value>, StatusCode> {
    let result = state.mcp_client
        .call_tool(&payload.tool, payload.params)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(result))
}

pub fn create_app(mcp_client: McpClient) -> Router {
    let state = AppState {
        mcp_client: Arc::new(mcp_client),
    };
    
    Router::new()
        .route("/tools/execute", post(execute_tool))
        .with_state(state)
}
```

## Testing Your Client

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mcp_protocol_sdk::testing::MockServer;
    
    #[tokio::test]
    async fn test_tool_calling() {
        // Create mock server
        let mut mock_server = MockServer::new();
        mock_server.expect_tool_call("calculate")
            .with_params(json!({"expression": "2+2"}))
            .return_result(json!({"result": 4}));
        
        // Connect client to mock
        let client = McpClient::new().build();
        client.connect(mock_server.transport()).await.unwrap();
        
        // Test tool call
        let result = client.call_tool("calculate", json!({"expression": "2+2"})).await.unwrap();
        assert_eq!(result["result"], 4);
    }
}
```

## Error Handling Best Practices

```rust
use mcp_protocol_sdk::errors::{McpError, ErrorCategory};

async fn robust_tool_calling(client: &McpClient) -> Result<Value, McpError> {
    match client.call_tool("my_tool", json!({})).await {
        Ok(result) => Ok(result),
        Err(McpError::Transport(_)) => {
            // Handle connection issues
            client.reconnect().await?;
            client.call_tool("my_tool", json!({})).await
        }
        Err(McpError::Protocol(msg)) => {
            // Handle protocol errors
            log::error!("Protocol error: {}", msg);
            Err(McpError::Protocol(msg))
        }
        Err(McpError::ToolNotFound(tool)) => {
            // Handle missing tools gracefully
            log::warn!("Tool {} not available, using fallback", tool);
            Ok(json!({"error": "Tool not available"}))
        }
        Err(e) => Err(e),
    }
}
```

## Real-World Examples

Check out these complete client examples:

1. **[Basic Client](../examples/client_example.rs)** - Simple tool calling
2. **[HTTP Client](../examples/http_client.rs)** - REST API integration
3. **[WebSocket Client](../examples/websocket_client.rs)** - Real-time communication

## Next Steps

1. **Integrate with AI models**: See [AI integration guide](../integrations/ai-models.md)
2. **Build web interfaces**: Check [web integration patterns](../integrations/web.md)
3. **Production deployment**: Read [production guide](../advanced/production.md)
4. **Monitoring**: Set up [client monitoring](../advanced/monitoring.md)

Happy client building! 🚀
