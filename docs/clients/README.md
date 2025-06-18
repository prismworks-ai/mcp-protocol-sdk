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
    let mut client = McpClient::new("my-ai-client".to_string(), "1.0.0".to_string());
    
    // Connect to server with transport
    let transport = StdioClientTransport::new("./my-server", vec!["--config", "prod.json"]).await?;
    let init_result = client.connect(transport).await?;
    
    // Client is now connected and initialized
    println!("Connected to: {} v{}", 
        init_result.server_info.name, 
        init_result.server_info.version);
    
    // Use the client...
    interact_with_server(&client).await?;
    
    Ok(())
}
```

### 2. Server Discovery & Capabilities

```rust
async fn discover_capabilities(client: &McpClient) -> Result<(), Box<dyn std::error::Error>> {
    // Get server info (available after connection)
    if let Some(server_info) = client.server_info().await {
        println!("Connected to: {} v{}", server_info.name, server_info.version);
    }
    
    // List available tools
    let tools_result = client.list_tools(None).await?;
    println!("Available tools:");
    for tool in &tools_result.tools {
        println!("  - {}: {}", tool.name, tool.description);
        if let Some(input_schema) = &tool.input_schema {
            if let Some(properties) = input_schema.get("properties") {
                println!("    Parameters: {}", serde_json::to_string_pretty(properties)?);
            }
        }
    }
    
    // List available resources
    let resources_result = client.list_resources(None).await?;
    println!("Available resources:");
    for resource in &resources_result.resources {
        println!("  - {}: {}", resource.uri, resource.name);
        if let Some(description) = &resource.description {
            println!("    Description: {}", description);
        }
    }
    
    // List available prompts
    let prompts_result = client.list_prompts(None).await?;
    println!("Available prompts:");
    for prompt in &prompts_result.prompts {
        println!("  - {}: {}", prompt.name, prompt.description.as_ref().unwrap_or(&"No description".to_string()));
    }
    
    Ok(())
}
```

### 3. Tool Execution

```rust
async fn execute_tools(client: &McpClient) -> Result<(), Box<dyn std::error::Error>> {
    // Simple tool call
    let mut arguments = std::collections::HashMap::new();
    arguments.insert("expression".to_string(), serde_json::json!("10 + 5 * 2"));
    arguments.insert("precision".to_string(), serde_json::json!(2));
    
    let result = client.call_tool("calculate".to_string(), Some(arguments)).await?;
    
    println!("Calculation result: {:?}", result.content);
    
    // Multiple individual tool calls
    let mut args1 = std::collections::HashMap::new();
    args1.insert("path".to_string(), serde_json::json!("config.json"));
    
    let mut args2 = std::collections::HashMap::new();
    args2.insert("path".to_string(), serde_json::json!("data.csv"));
    
    let mut args3 = std::collections::HashMap::new();
    args3.insert("expression".to_string(), serde_json::json!("100 / 4"));
    
    let result1 = client.call_tool("read_file".to_string(), Some(args1)).await?;
    let result2 = client.call_tool("read_file".to_string(), Some(args2)).await?;
    let result3 = client.call_tool("calculate".to_string(), Some(args3)).await?;
    
    println!("File 1 result: {:?}", result1.content);
    println!("File 2 result: {:?}", result2.content);
    println!("Calculation result: {:?}", result3.content);
    
    Ok(())
}
```

### 4. Resource Access

```rust
async fn access_resources(client: &McpClient) -> Result<(), Box<dyn std::error::Error>> {
    // Read a specific resource
    let result = client.read_resource("file://config.json".to_string()).await?;
    
    for content_item in &result.contents {
        match &content_item.text {
            Some(text) => {
                println!("Config content: {}", text);
            }
            None => {
                if let Some(blob) = &content_item.blob {
                    println!("Binary data, {} bytes", blob.len());
                }
            }
        }
    }
    
    // Read multiple resources
    let resources = vec![
        "file://config.json",
        "database://users/1",
        "api://weather/current"
    ];
    
    for uri in resources {
        match client.read_resource(uri.to_string()).await {
            Ok(result) => {
                println!("Successfully read: {} ({} items)", uri, result.contents.len());
            }
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
    let mut arguments = std::collections::HashMap::new();
    arguments.insert("code".to_string(), "fn main() { println!(\"Hello\"); }".to_string());
    arguments.insert("language".to_string(), "rust".to_string());
    arguments.insert("style".to_string(), "thorough".to_string());
    
    let prompt_result = client.get_prompt(
        "code_review".to_string(),
        Some(arguments)
    ).await?;
    
    // Display the prompt messages
    for message in &prompt_result.messages {
        match &message.content {
            Some(content) => {
                if let Some(text) = &content.text {
                    println!("Generated prompt content: {}", text);
                }
            }
            None => {
                println!("Empty message content");
            }
        }
    }
    
    // Use prompt in AI conversation
    // (This would be implementation-specific for your AI model)
    // let ai_response = send_to_ai_model(&prompt_result).await?;
    // println!("AI response: {}", ai_response);
    
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
        transport: impl Transport + 'static
    ) -> Result<(), Box<dyn std::error::Error>> {
        let mut client = McpClient::new("multi-client".to_string(), "1.0.0".to_string());
            
        client.connect(transport).await?;
        
        self.clients.insert(name.to_string(), client);
        Ok(())
    }
    
    pub async fn call_tool_on_server(
        &self,
        server_name: &str,
        tool_name: &str,
        params: Option<std::collections::HashMap<String, serde_json::Value>>
    ) -> Result<crate::protocol::messages::CallToolResult, Box<dyn std::error::Error>> {
        let client = self.clients.get(server_name)
            .ok_or(format!("Server {} not connected", server_name))?;
            
        client.call_tool(tool_name.to_string(), params).await.map_err(|e| e.into())
    }
}
```

### Connection Resilience

```rust
async fn resilient_client_connection() -> Result<(), Box<dyn std::error::Error>> {
    let config = ClientConfig {
        request_timeout_ms: 30000,
        max_retries: 3,
        retry_delay_ms: 1000,
        validate_requests: true,
        validate_responses: true,
    };
    
    let client = McpClient::with_config("resilient-client".to_string(), "1.0.0".to_string(), config);
    
    // Connect with custom transport
    let transport = HttpClientTransport::new("http://localhost:3000/mcp", None).await?;
    client.connect(transport).await?;
    
    // Connection is now active with timeout and retry configuration
    
    Ok(())
}
```

## Transport Selection

### STDIO Client (Connect to STDIO servers)

```rust
use mcp_protocol_sdk::transport::StdioClientTransport;

// Connect to external process
let transport = StdioClientTransport::new("./my-server", vec!["--config", "prod.json"]).await?;

// Or connect to a simple executable
let transport = StdioClientTransport::new("./my-server", vec![]).await?;
```

### HTTP Client

```rust
use mcp_protocol_sdk::transport::HttpClientTransport;

let transport = HttpClientTransport::new("http://localhost:3000/mcp", None).await?;
```

### WebSocket Client

```rust
use mcp_protocol_sdk::transport::WebSocketClientTransport;

let transport = WebSocketClientTransport::new("ws://localhost:8080/mcp").await?;
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
    use async_trait::async_trait;
    
    // Mock transport for testing
    struct MockTransport {
        responses: Vec<JsonRpcResponse>,
        current: usize,
    }
    
    impl MockTransport {
        fn new(responses: Vec<JsonRpcResponse>) -> Self {
            Self { responses, current: 0 }
        }
    }
    
    #[async_trait]
    impl Transport for MockTransport {
        async fn send_request(&mut self, _request: JsonRpcRequest) -> McpResult<JsonRpcResponse> {
            if self.current < self.responses.len() {
                let response = self.responses[self.current].clone();
                self.current += 1;
                Ok(response)
            } else {
                Err(McpError::Transport("No more responses".to_string()))
            }
        }
        
        async fn send_notification(&mut self, _notification: JsonRpcNotification) -> McpResult<()> {
            Ok(())
        }
        
        async fn receive_notification(&mut self) -> McpResult<Option<JsonRpcNotification>> {
            Ok(None)
        }
        
        async fn close(&mut self) -> McpResult<()> {
            Ok(())
        }
    }
    
    #[tokio::test]
    async fn test_tool_calling() {
        let tool_result = CallToolResult {
            content: vec![ToolContent::Text {
                text: "4".to_string(),
            }],
        };
        
        let response = JsonRpcResponse::success(
            serde_json::Value::from(1), 
            tool_result
        ).unwrap();
        
        let transport = MockTransport::new(vec![response]);
        
        let mut client = McpClient::new("test-client".to_string(), "1.0.0".to_string());
        client.connect(transport).await.unwrap();
        
        // Test tool call
        let mut args = std::collections::HashMap::new();
        args.insert("expression".to_string(), serde_json::json!("2+2"));
        
        let result = client.call_tool("calculate".to_string(), Some(args)).await.unwrap();
        assert_eq!(result.content.len(), 1);
    }
}
```

## Error Handling Best Practices

```rust
use mcp_protocol_sdk::core::error::McpError;

async fn robust_tool_calling(client: &McpClient) -> Result<CallToolResult, McpError> {
    let mut args = std::collections::HashMap::new();
    args.insert("param".to_string(), serde_json::json!("value"));
    
    match client.call_tool("my_tool".to_string(), Some(args.clone())).await {
        Ok(result) => Ok(result),
        Err(McpError::Transport(_)) => {
            // Handle connection issues
            log::error!("Transport error, connection may be lost");
            // Attempt reconnection would require reconnection logic
            Err(McpError::Transport("Connection lost".to_string()))
        }
        Err(McpError::Protocol(msg)) => {
            // Handle protocol errors
            log::error!("Protocol error: {}", msg);
            Err(McpError::Protocol(msg))
        }
        Err(McpError::Serialization(msg)) => {
            // Handle serialization errors
            log::warn!("Serialization error: {}", msg);
            Err(McpError::Serialization(msg))
        }
        Err(e) => {
            log::error!("Unexpected error: {}", e);
            Err(e)
        }
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
