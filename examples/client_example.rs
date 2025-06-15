//! Simple MCP Client Example
//!
//! This example demonstrates how to create a basic MCP client that connects to
//! an MCP server and performs various operations like calling tools, reading
//! resources, and executing prompts.

use serde_json::json;
use std::collections::HashMap;

use mcp_protocol_sdk::{
    client::session::SessionConfig,
    client::{ClientSession, McpClient},
    core::error::McpResult,
    transport::stdio::StdioClientTransport,
    Content,
};

#[tokio::main]
async fn main() -> McpResult<()> {
    // Initialize logging
    #[cfg(feature = "tracing-subscriber")]
    tracing_subscriber::fmt::init();

    tracing::info!("Starting MCP client example...");

    // Create client with configuration
    let client = McpClient::new("simple-demo-client".to_string(), "1.0.0".to_string());

    // Create a session for connection management
    let session_config = SessionConfig {
        auto_reconnect: true,
        max_reconnect_attempts: 3,
        reconnect_delay_ms: 1000,
        connection_timeout_ms: 5000,
        heartbeat_interval_ms: 30000,
        ..Default::default()
    };

    let session = ClientSession::with_config(client, session_config);

    // Connect to the server (assumes a server is running on STDIO)
    tracing::info!("Connecting to server...");

    // In a real scenario, you would specify the server command/process
    // For this example, we'll assume a server is already available
    let transport =
        StdioClientTransport::new("echo".to_string(), vec!["hello".to_string()]).await?;

    match session.connect(transport).await {
        Ok(init_result) => {
            tracing::info!(
                "Connected to server: {} v{}",
                init_result.server_info.name,
                init_result.server_info.version
            );
            tracing::info!("Server capabilities: {:?}", init_result.capabilities);
        }
        Err(e) => {
            tracing::error!("Failed to connect to server: {}", e);
            return Err(e);
        }
    }

    // Get the client for operations
    let client = session.client();

    // Demonstrate various client operations
    match demonstrate_operations(&client).await {
        Ok(_) => tracing::info!("All operations completed successfully"),
        Err(e) => tracing::error!("Operation failed: {}", e),
    }

    // Disconnect from the server
    tracing::info!("Disconnecting from server...");
    session.disconnect().await?;

    tracing::info!("Client example completed");
    Ok(())
}

async fn demonstrate_operations(
    client: &std::sync::Arc<tokio::sync::Mutex<McpClient>>,
) -> McpResult<()> {
    // 1. List available tools
    tracing::info!("=== Listing Tools ===");
    {
        let client_guard = client.lock().await;
        let tools_result = client_guard.list_tools(None).await?;

        tracing::info!("Available tools:");
        for tool in &tools_result.tools {
            tracing::info!(
                "  - {}: {}",
                tool.name,
                tool.description.as_deref().unwrap_or("No description")
            );
        }
    }

    // 2. Call the calculator tool
    tracing::info!("=== Calling Calculator Tool ===");
    {
        let client_guard = client.lock().await;
        let mut args = HashMap::new();
        args.insert("a".to_string(), json!(15.5));
        args.insert("b".to_string(), json!(4.5));
        args.insert("operation".to_string(), json!("multiply"));

        match client_guard
            .call_tool("calculator".to_string(), Some(args))
            .await
        {
            Ok(result) => {
                tracing::info!("Calculator result:");
                for content in &result.content {
                    match content {
                        Content::Text { text, .. } => {
                            tracing::info!("  {}", text);
                        }
                        _ => tracing::info!("  (non-text content)"),
                    }
                }
            }
            Err(e) => tracing::error!("Calculator tool failed: {}", e),
        }
    }

    // 3. Call the echo tool
    tracing::info!("=== Calling Echo Tool ===");
    {
        let client_guard = client.lock().await;
        let mut args = HashMap::new();
        args.insert("message".to_string(), json!("Hello from MCP client!"));
        args.insert("uppercase".to_string(), json!(true));
        args.insert("prefix".to_string(), json!("CLIENT"));

        match client_guard.call_tool("echo".to_string(), Some(args)).await {
            Ok(result) => {
                tracing::info!("Echo result:");
                for content in &result.content {
                    match content {
                        Content::Text { text, .. } => {
                            tracing::info!("  {}", text);
                        }
                        _ => tracing::info!("  (non-text content)"),
                    }
                }
            }
            Err(e) => tracing::error!("Echo tool failed: {}", e),
        }
    }

    // 4. List available resources
    tracing::info!("=== Listing Resources ===");
    {
        let client_guard = client.lock().await;
        let resources_result = client_guard.list_resources(None).await?;

        tracing::info!("Available resources:");
        for resource in &resources_result.resources {
            tracing::info!(
                "  - {}: {} ({})",
                resource.name.as_deref().unwrap_or("Unknown"),
                resource.uri,
                resource.mime_type.as_deref().unwrap_or("unknown type")
            );
        }
    }

    // 5. Read a specific resource
    tracing::info!("=== Reading Resource ===");
    {
        let client_guard = client.lock().await;
        match client_guard
            .read_resource("file:///demo.txt".to_string())
            .await
        {
            Ok(result) => {
                tracing::info!("Resource content:");
                for content in &result.contents {
                    match content {
                        mcp_protocol_sdk::ResourceContents::Text { text, .. } => {
                            tracing::info!("  {}", text);
                        }
                        mcp_protocol_sdk::ResourceContents::Blob { .. } => {
                            tracing::info!("  (binary content)");
                        }
                    }
                }
            }
            Err(e) => tracing::error!("Failed to read resource: {}", e),
        }
    }

    // 6. List available prompts
    tracing::info!("=== Listing Prompts ===");
    {
        let client_guard = client.lock().await;
        let prompts_result = client_guard.list_prompts(None).await?;

        tracing::info!("Available prompts:");
        for prompt in &prompts_result.prompts {
            tracing::info!(
                "  - {}: {}",
                prompt.name,
                prompt.description.as_deref().unwrap_or("No description")
            );
            if let Some(args) = &prompt.arguments {
                for arg in args {
                    tracing::info!(
                        "    - {}: {} (required: {})",
                        arg.name,
                        arg.description.as_deref().unwrap_or("No description"),
                        arg.required.unwrap_or(false)
                    );
                }
            }
        }
    }

    // 7. Get a prompt
    tracing::info!("=== Getting Code Review Prompt ===");
    {
        let client_guard = client.lock().await;
        let mut args = HashMap::new();
        args.insert("language".to_string(), json!("Rust"));
        args.insert("focus".to_string(), json!("security"));

        match client_guard
            .get_prompt("code-review".to_string(), Some(args))
            .await
        {
            Ok(result) => {
                tracing::info!("Code review prompt:");
                if let Some(description) = &result.description {
                    tracing::info!("  Description: {}", description);
                }
                for (i, message) in result.messages.iter().enumerate() {
                    tracing::info!("  Message {}: [{:?}]", i + 1, message.role);
                    match &message.content {
                        Content::Text { text, .. } => {
                            tracing::info!("    {}", text);
                        }
                        _ => tracing::info!("    (non-text content)"),
                    }
                }
            }
            Err(e) => tracing::error!("Failed to get prompt: {}", e),
        }
    }

    // 8. Test ping
    tracing::info!("=== Testing Ping ===");
    {
        let client_guard = client.lock().await;
        match client_guard.ping().await {
            Ok(_) => tracing::info!("Ping successful"),
            Err(e) => tracing::error!("Ping failed: {}", e),
        }
    }

    Ok(())
}
