# MCP Protocol SDK

**A production-ready, feature-complete Rust implementation of the Model Context Protocol**

[![Crates.io](https://img.shields.io/crates/v/mcp-protocol-sdk.svg)](https://crates.io/crates/mcp-protocol-sdk)
[![Documentation](https://docs.rs/mcp-protocol-sdk/badge.svg)](https://docs.rs/mcp-protocol-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/rishirandhawa/mcp-protocol-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/rishirandhawa/mcp-protocol-sdk/actions/workflows/ci.yml)

---

## Why MCP Protocol SDK?

The **Model Context Protocol (MCP)** is revolutionizing how AI assistants interact with external systems. While the official `rmcp` SDK provides basic functionality, **mcp-protocol-sdk** was created to fill the gap for production applications that need:

- **Enterprise-grade reliability** with comprehensive error handling
- **Multiple transport options** beyond just STDIO
- **Advanced session management** with auto-reconnection
- **Production-ready features** like monitoring, validation, and performance optimization

## 🚀 Quick Start

### Server in 30 seconds

```rust
use mcp_protocol_sdk::{
    server::McpServer,
    transport::stdio::StdioServerTransport,
    core::tool::ToolHandler,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("my-server".to_string(), "1.0.0".to_string());
    
    // Add your tools, resources, and prompts
    server.add_tool("echo".to_string(), None, json!({}), EchoTool).await?;
    
    // Start with any transport
    let transport = StdioServerTransport::new();
    server.start(transport).await?;
    
    Ok(())
}
```

### Client Connection

```rust
use mcp_protocol_sdk::{
    client::{McpClient, ClientSession},
    transport::websocket::WebSocketClientTransport,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = McpClient::new("my-client".to_string(), "1.0.0".to_string());
    let session = ClientSession::new(client);
    
    // Connect via WebSocket, HTTP, or STDIO
    let transport = WebSocketClientTransport::new("ws://localhost:8080").await?;
    let init_result = session.connect(transport).await?;
    
    // Use the connected client
    let tool_result = session.client().lock().await
        .call_tool("echo".to_string(), None).await?;
    
    Ok(())
}
```

## 🔥 Key Features

### Multi-Transport Architecture
- **STDIO Transport**: Efficient process-based communication
- **HTTP Transport**: RESTful API with Server-Sent Events
- **WebSocket Transport**: Real-time bidirectional communication

### Production-Ready Components
- **Session Management**: Auto-reconnection with exponential backoff
- **Error Recovery**: Comprehensive error handling and graceful degradation
- **Validation**: JSON Schema validation for tool parameters
- **Monitoring**: Built-in metrics and performance tracking

### Developer Experience
- **Type Safety**: Full Rust type system for all MCP constructs
- **Async/Await**: Built on Tokio for high-performance operations
- **Rich APIs**: Intuitive, well-documented interfaces
- **Extensive Examples**: 8+ complete examples covering all use cases

## 📊 Performance

- **High Throughput**: >10,000 requests/second
- **Low Latency**: <1ms for simple operations
- **Memory Efficient**: Minimal allocation overhead
- **Scalable**: Supports thousands of concurrent connections

## 🎯 Use Cases

### Perfect For

- **Enterprise Applications** requiring reliability and monitoring
- **Multi-Client Systems** with WebSocket or HTTP transports
- **Complex Integrations** needing advanced session management
- **Production Deployments** requiring comprehensive error handling
- **Real-time Applications** with live data streaming
- **Scalable Services** handling multiple concurrent connections

### Examples

- **AI-Powered IDEs** with real-time code assistance
- **Customer Support Platforms** with live chat integration
- **Data Analytics Dashboards** with AI insights
- **IoT Management Systems** with AI-driven automation
- **Content Management Systems** with AI content generation

## 📚 Documentation

- [**Getting Started Guide**](getting-started.md) - Step-by-step tutorial
- [**API Reference**](https://docs.rs/mcp-protocol-sdk) - Complete API documentation
- [**Rust Documentation**](rust-docs/) - Generated code documentation
- [**Examples Collection**](examples.md) - Real-world usage examples
- [**Transport Guide**](transports.md) - Transport layer documentation
- [**Architecture Overview**](architecture.md) - System design and patterns

## 🔗 Quick Links

- [**GitHub Repository**](https://github.com/rishirandhawa/mcp-protocol-sdk)
- [**Crates.io Package**](https://crates.io/crates/mcp-protocol-sdk)
- [**Issue Tracker**](https://github.com/rishirandhawa/mcp-protocol-sdk/issues)
- [**Contributing Guide**](https://github.com/rishirandhawa/mcp-protocol-sdk/blob/main/CONTRIBUTING.md)

---

## Comparison with Official SDK

| Feature | mcp-protocol-sdk | Official rmcp |
|---------|--------------|---------------|
| **Transport Options** | STDIO, HTTP, WebSocket | STDIO, SSE only |
| **Session Management** | ✅ Auto-reconnection | ❌ Basic |
| **Error Handling** | ✅ Comprehensive | ❌ Limited |
| **Production Ready** | ✅ Enterprise features | ❌ Basic functionality |
| **Documentation** | ✅ Extensive | ❌ Minimal |
| **Examples** | ✅ 8+ complete examples | ❌ Few basic examples |
| **Test Coverage** | ✅ 85+ tests | ❌ Limited testing |
| **Performance** | ✅ Optimized & benchmarked | ❌ Not optimized |

---

*Built with ❤️ for the Rust and AI communities*
