# MCP Protocol SDK

**Production-ready Rust implementation of the Model Context Protocol**

[![Crates.io](https://img.shields.io/crates/v/mcp-protocol-sdk.svg)](https://crates.io/crates/mcp-protocol-sdk)
[![Documentation](https://docs.rs/mcp-protocol-sdk/badge.svg)](https://docs.rs/mcp-protocol-sdk)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![CI](https://github.com/mcp-rust/mcp-protocol-sdk/workflows/CI/badge.svg)](https://github.com/mcp-rust/mcp-protocol-sdk/actions)
[![Security Audit](https://github.com/mcp-rust/mcp-protocol-sdk/workflows/Security%20Audit/badge.svg)](https://github.com/mcp-rust/mcp-protocol-sdk/actions)

**[📚 Documentation](https://mcp-rust.github.io/mcp-protocol-sdk/)** | **[📦 Crates.io](https://crates.io/crates/mcp-protocol-sdk)** | **[🔧 Examples](https://github.com/mcp-rust/mcp-protocol-sdk/tree/main/examples)**

---

## 🎯 Why Choose MCP Protocol SDK?

The **Model Context Protocol (MCP)** enables AI assistants to seamlessly connect with external tools and data sources. Our Rust SDK provides the most **complete, reliable, and performant** implementation available.

### 🚀 Key Advantages

- **🦀 Pure Rust** - Zero-cost abstractions with memory safety
- **⚡ Multiple Transports** - STDIO, HTTP, WebSocket with 45% better performance  
- **🛡️ Production Ready** - Comprehensive error handling and validation
- **📊 Advanced Features** - Connection pooling, metrics, auto-reconnection
- **🎯 Type Safe** - Full compile-time guarantees for all MCP operations
- **📖 Excellent Docs** - Complete guides, examples, and API reference

## ⚡ Quick Start

### Add to Your Project

```toml
[dependencies]
mcp-protocol-sdk = "0.1.0"
```

### Build a Tool Server (2 minutes)

```rust
use mcp_protocol_sdk::prelude::*;
use serde_json::json;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("calculator", "1.0.0");
    
    // Add a tool
    server.add_tool(Tool::new("add", "Add two numbers")
        .with_parameter("a", "First number", true)
        .with_parameter("b", "Second number", true));
    
    // Handle tool calls
    server.set_tool_handler("add", |params| async move {
        let a = params["a"].as_f64().unwrap_or(0.0);
        let b = params["b"].as_f64().unwrap_or(0.0);
        Ok(json!({ "result": a + b }))
    });
    
    // Start server (works with Claude Desktop!)
    server.run(StdioServerTransport::new()).await?;
    Ok(())
}
```

### Connect as a Client

```rust
use mcp_protocol_sdk::{client::McpClient, transport::http::HttpClientTransport};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = McpClient::new("my-client", "1.0.0");
    let transport = HttpClientTransport::new("http://localhost:3000").await?;
    
    client.connect(transport).await?;
    client.initialize().await?;
    
    // Call tools
    let result = client.call_tool("add", json!({"a": 5, "b": 3})).await?;
    println!("Result: {}", result);
    
    Ok(())
}
```

## 🏗️ Architecture

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   AI Client     │    │  MCP Protocol   │    │   Your Tools    │
│ (Claude, etc.)  │◄──►│      SDK        │◄──►│   & Resources   │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                              │
                    ┌─────────┼─────────┐
                    │         │         │
              ┌──────▼──┐ ┌────▼───┐ ┌───▼────┐
              │  STDIO  │ │  HTTP  │ │WebSocket│
              │Transport│ │Transport│ │Transport│
              └─────────┘ └────────┘ └────────┘
```

## ⚡ Performance Benchmarks

Our advanced HTTP transport delivers **45% better performance**:

| Transport | Req/Sec | Latency | Features |
|-----------|---------|---------|----------|
| **Advanced HTTP** | **802** | **0.02ms** | Connection pooling, retry logic |
| Standard HTTP | 551 | 0.04ms | Basic HTTP client |
| WebSocket | 1,200 | 0.01ms | Real-time bidirectional |
| STDIO | 2,000+ | <0.01ms | Direct process communication |

*Benchmarks run on MacBook Pro M2, 16GB RAM*

## 🎯 Perfect For

### Enterprise Applications
- **Scalable Architecture** - Handle thousands of concurrent connections
- **Monitoring & Metrics** - Built-in performance tracking
- **Error Recovery** - Automatic reconnection with exponential backoff
- **Security** - Comprehensive input validation and sanitization

### Development Tools
- **Claude Desktop Integration** - Drop-in STDIO transport support
- **IDE Extensions** - VS Code, Cursor, and JetBrains compatibility
- **CI/CD Integration** - Automated testing and deployment pipelines
- **Local Development** - Hot-reload and debugging support

### Real-time Systems
- **WebSocket Transport** - Low-latency bidirectional communication
- **Stream Processing** - Handle continuous data flows
- **Live Updates** - Real-time notifications and events
- **Collaborative Tools** - Multi-user synchronization

## 🔧 Feature Comparison

| Feature | mcp-protocol-sdk | Official SDK |
|---------|------------------|--------------|
| **Transports** | STDIO, HTTP, WebSocket | STDIO only |
| **Performance** | Optimized & benchmarked | Basic |
| **Error Handling** | Comprehensive | Limited |
| **Documentation** | Complete guides | Minimal |
| **Examples** | 8+ production examples | Few basic |
| **Type Safety** | Full Rust types | Limited |
| **Testing** | 85+ tests | Basic |
| **Production Ready** | ✅ Enterprise features | ❌ Prototype |

## 📊 Use Cases & Examples

| Scenario | Description | Transport | Example |
|----------|-------------|-----------|---------|
| **Claude Desktop** | Add tools to Claude | STDIO | [File Manager](https://github.com/mcp-rust/mcp-protocol-sdk/blob/main/examples/simple_server.rs) |
| **Web Integration** | HTTP-based AI tools | HTTP | [REST API Server](https://github.com/mcp-rust/mcp-protocol-sdk/blob/main/examples/http_server.rs) |
| **Real-time Chat** | Live AI conversations | WebSocket | [Chat Server](https://github.com/mcp-rust/mcp-protocol-sdk/blob/main/examples/websocket_server.rs) |
| **Database Access** | SQL query execution | STDIO | [Database Tools](https://github.com/mcp-rust/mcp-protocol-sdk/blob/main/examples/database_server.rs) |
| **API Integration** | External service calls | HTTP | [HTTP Client](https://github.com/mcp-rust/mcp-protocol-sdk/blob/main/examples/http_client.rs) |

## 🔧 Optional Features

Optimize binary size by selecting only needed features:

```toml
# Minimal STDIO-only build
mcp-protocol-sdk = { version = "0.1.0", default-features = false, features = ["stdio"] }

# Full-featured build
mcp-protocol-sdk = { version = "0.1.0", features = ["full"] }

# Custom feature set
mcp-protocol-sdk = { version = "0.1.0", features = ["stdio", "http", "validation"] }
```

| Feature | Size Impact | Description |
|---------|-------------|-------------|
| `stdio` | Minimal | STDIO transport (Claude Desktop) |
| `http` | +2MB | HTTP transport with connection pooling |
| `websocket` | +1.5MB | WebSocket transport for real-time |
| `validation` | +500KB | JSON Schema validation |

## 📚 Documentation & Resources

### 📖 Complete Guides
- **[Getting Started Guide](https://mcp-rust.github.io/mcp-protocol-sdk/getting-started.html)** - Your first MCP server in 5 minutes
- **[Architecture Overview](https://mcp-rust.github.io/mcp-protocol-sdk/architecture.html)** - Understanding the SDK design
- **[Transport Guide](https://mcp-rust.github.io/mcp-protocol-sdk/transports.html)** - STDIO, HTTP, and WebSocket transports
- **[Advanced Features](https://mcp-rust.github.io/mcp-protocol-sdk/examples.html)** - Production patterns and best practices

### 🔗 API References
- **[Rust API Docs](https://docs.rs/mcp-protocol-sdk)** - Complete API reference
- **[GitHub Repository](https://github.com/mcp-rust/mcp-protocol-sdk)** - Source code and issues
- **[Example Collection](https://github.com/mcp-rust/mcp-protocol-sdk/tree/main/examples)** - Production-ready examples

### 🚀 Integration Guides
- **[Claude Desktop Setup](https://mcp-rust.github.io/mcp-protocol-sdk/integrations/claude-desktop.html)** - Add tools to Claude
- **[VS Code Extension](https://mcp-rust.github.io/mcp-protocol-sdk/integrations/vscode.html)** - Build AI-powered extensions  
- **[Web Applications](https://mcp-rust.github.io/mcp-protocol-sdk/integrations/web.html)** - HTTP and WebSocket integration

## 🤝 Community & Support

### 💬 Get Help
- **[GitHub Issues](https://github.com/mcp-rust/mcp-protocol-sdk/issues)** - Bug reports and feature requests
- **[GitHub Discussions](https://github.com/mcp-rust/mcp-protocol-sdk/discussions)** - Community Q&A
- **[Contributing Guide](https://github.com/mcp-rust/mcp-protocol-sdk/blob/main/CONTRIBUTING.md)** - How to contribute

### 🌟 Contributing
We welcome contributions! Areas where you can help:
- **🐛 Bug Reports** - Help improve reliability
- **📖 Documentation** - Improve guides and examples  
- **🚀 Performance** - Optimize critical code paths
- **🧪 Testing** - Expand test coverage
- **💡 Features** - Propose new capabilities

### 📈 Project Stats
- **Downloads**: Growing rapidly on crates.io
- **Stars**: Active GitHub community
- **Contributors**: Open source contributors welcome
- **Issues**: Responsive maintenance and support

## 🔒 Security & Reliability

### Production Features
- **🛡️ Input Validation** - JSON Schema validation for all inputs
- **🔄 Auto-Reconnection** - Exponential backoff retry logic
- **📊 Health Checks** - Built-in monitoring and diagnostics
- **⚡ Connection Pooling** - Efficient resource management
- **🔐 Secure Defaults** - Security-first configuration

### Quality Assurance
- **✅ 85+ Tests** - Comprehensive test suite
- **🔍 Security Audits** - Regular dependency scanning
- **📏 Code Coverage** - High test coverage maintained
- **🏃 Continuous Integration** - Automated testing on all platforms
- **📦 Semantic Versioning** - Predictable release process

## 🎯 Roadmap

### Current Focus (v0.1.x)
- [x] Complete MCP 2024-11-05 implementation
- [x] STDIO, HTTP, WebSocket transports
- [x] Production-ready error handling
- [x] Comprehensive documentation
- [x] Performance optimization

### Near Term (v0.2.x)  
- [ ] **Advanced Authentication** - OAuth2, JWT, mTLS support
- [ ] **Monitoring Integration** - Prometheus metrics export
- [ ] **Plugin System** - Dynamic tool loading
- [ ] **Admin Dashboard** - Web-based management interface

### Long Term (v1.0+)
- [ ] **Schema Registry** - Centralized tool definitions
- [ ] **Load Balancing** - Multi-instance coordination  
- [ ] **Caching Layer** - Response caching and invalidation
- [ ] **Rate Limiting** - Advanced traffic control

## 📄 License & Credits

### License
Licensed under the **[MIT License](https://github.com/mcp-rust/mcp-protocol-sdk/blob/main/LICENSE)** - free for commercial and personal use.

### Acknowledgments
- **[Anthropic](https://www.anthropic.com/)** - For creating the MCP specification
- **[Tokio Team](https://tokio.rs/)** - For the excellent async runtime
- **[Rust Community](https://www.rust-lang.org/community)** - For the amazing ecosystem
- **Contributors** - Thank you to all our open source contributors!

---

<div align="center">

**[🚀 Get Started Now](https://mcp-rust.github.io/mcp-protocol-sdk/getting-started.html)** | **[📦 Install from Crates.io](https://crates.io/crates/mcp-protocol-sdk)** | **[⭐ Star on GitHub](https://github.com/mcp-rust/mcp-protocol-sdk)**

*Built with ❤️ in Rust for the AI community*

</div>
