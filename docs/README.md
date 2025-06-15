# MCP Protocol SDK Documentation

Welcome to the Model Context Protocol (MCP) SDK documentation! This Rust SDK provides a complete implementation of the MCP specification, enabling you to build both servers and clients for AI model context sharing.

## 📚 Documentation Structure

### Getting Started
- [🚀 Quick Start Guide](./getting-started.md) - Get up and running in 5 minutes
- [🏗️ Installation & Setup](./installation.md) - Detailed installation instructions
- [🔧 Configuration](./configuration.md) - Configuration options and best practices

### Building Servers
- [📡 Server Development Guide](./servers/README.md) - Complete guide to building MCP servers
- [🔌 Transport Options](./servers/transports.md) - Choose your transport layer
- [🛠️ Tools & Resources](./servers/tools-resources.md) - Implementing tools and resources
- [📋 Examples](./servers/examples.md) - Real-world server examples

### Building Clients
- [💻 Client Development Guide](./clients/README.md) - Complete guide to building MCP clients
- [🔗 Connection Management](./clients/connections.md) - Managing server connections
- [📞 Making Requests](./clients/requests.md) - Calling server methods
- [📋 Examples](./clients/examples.md) - Real-world client examples

### Integration Guides
- [🖥️ Claude Desktop Integration](./integrations/claude-desktop.md) - Add your server to Claude Desktop
- [⚡ Cursor Integration](./integrations/cursor.md) - Integrate with Cursor IDE
- [📝 VS Code Integration](./integrations/vscode.md) - VS Code extension development
- [🔌 Other Clients](./integrations/other-clients.md) - Integration with other MCP clients

### Advanced Topics
- [🎯 Performance Optimization](./advanced/performance.md) - Optimize your implementations
- [🔒 Security Best Practices](./advanced/security.md) - Keep your implementations secure
- [🧪 Testing](./advanced/testing.md) - Testing strategies and tools
- [🐛 Debugging](./advanced/debugging.md) - Debug common issues

### API Reference
- [📖 API Documentation](./api/README.md) - Complete API reference
- [📊 Protocol Specification](./api/protocol.md) - MCP protocol details
- [🔧 Types & Interfaces](./api/types.md) - Data types and interfaces

## 🆘 Support

- [❓ FAQ](./faq.md) - Frequently asked questions
- [🐛 Troubleshooting](./troubleshooting.md) - Common issues and solutions
- [💬 Community](./community.md) - Join the community

## 🚀 Quick Links

| What do you want to do? | Go here |
|-------------------------|---------|
| Build an MCP server | [Server Guide](./servers/README.md) |
| Build an MCP client | [Client Guide](./clients/README.md) |
| Add to Claude Desktop | [Claude Integration](./integrations/claude-desktop.md) |
| Add to VS Code | [VS Code Integration](./integrations/vscode.md) |
| See examples | [Examples](./examples/README.md) |
| API reference | [API Docs](./api/README.md) |

## 📈 Feature Support

This SDK supports all MCP protocol features:

- ✅ **Core Protocol** - Full JSON-RPC implementation
- ✅ **Tools** - Custom tool registration and execution
- ✅ **Resources** - Resource discovery and access
- ✅ **Prompts** - Prompt templates and management
- ✅ **Logging** - Structured logging support
- ✅ **Sampling** - LLM sampling integration
- ✅ **Transport Layers** - STDIO, HTTP, WebSocket support
- ✅ **Authentication** - Security and auth mechanisms
- ✅ **Error Handling** - Robust error management

Choose only the features you need with our [optional feature flags](./configuration.md#feature-flags).
