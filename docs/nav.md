---
layout: page
title: Navigation
permalink: /nav/
---

# MCP Rust SDK Documentation

## Quick Navigation

### 🚀 Getting Started
- [**Home**](/) - Project overview and quick start
- [**Getting Started Guide**](getting-started) - Step-by-step tutorial
- [**Installation**](getting-started#installation) - Add to your project

### 📚 Learning Resources  
- [**Examples Collection**](examples) - Real-world usage examples
- [**Transport Guide**](transports) - Deep dive into transport options
- [**Architecture Overview**](architecture) - System design and patterns

### 📖 Reference
- [**API Reference**](https://docs.rs/mcp-protocol-sdk) - Complete API documentation
- [**Project Overview**](https://github.com/rishirandhawa/mcp-protocol-sdk/blob/main/PROJECT_OVERVIEW.md) - Why this SDK was created

### 🔗 External Links
- [**GitHub Repository**](https://github.com/rishirandhawa/mcp-protocol-sdk) - Source code & issues
- [**Crates.io Package**](https://crates.io/crates/mcp-protocol-sdk) - Package registry
- [**Contributing Guide**](https://github.com/rishirandhawa/mcp-protocol-sdk/blob/main/CONTRIBUTING.md) - How to contribute

---

## Popular Examples

### Basic Server & Client
```rust
// Simple server
let mut server = McpServer::new("my-server".to_string(), "1.0.0".to_string());
server.add_tool("echo".to_string(), None, json!({}), EchoTool).await?;
server.start(StdioServerTransport::new()).await?;

// Client connection
let session = ClientSession::new(McpClient::new("client".to_string(), "1.0.0".to_string()));
let transport = WebSocketClientTransport::new("ws://localhost:8080").await?;
session.connect(transport).await?;
```

### Multiple Transports
```rust
// HTTP with SSE
let transport = HttpServerTransport::new("0.0.0.0:3000");
// API: http://localhost:3000/mcp
// Events: http://localhost:3000/mcp/events

// WebSocket  
let transport = WebSocketServerTransport::new("0.0.0.0:8080");
// Connect: ws://localhost:8080

// STDIO (default)
let transport = StdioServerTransport::new();
// Process communication
```

## Feature Comparison

| Feature | mcp-protocol-sdk | Official rmcp |
|---------|--------------|---------------|
| **Transports** | STDIO, HTTP, WebSocket | STDIO, SSE only |
| **Session Mgmt** | ✅ Auto-reconnect | ❌ Basic |
| **Error Handling** | ✅ Comprehensive | ❌ Limited |
| **Production Ready** | ✅ Monitoring, validation | ❌ Basic |
| **Documentation** | ✅ Extensive | ❌ Minimal |
| **Examples** | ✅ 8+ complete | ❌ Few basic |

---

*Need help? Check our [GitHub Issues](https://github.com/rishirandhawa/mcp-protocol-sdk/issues) or [Contributing Guide](https://github.com/rishirandhawa/mcp-protocol-sdk/blob/main/CONTRIBUTING.md)*
