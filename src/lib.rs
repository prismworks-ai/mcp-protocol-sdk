// Copyright (c) 2025 MCP Rust Contributors
// SPDX-License-Identifier: MIT

//! # MCP Rust SDK (2025-03-26)
//!
//! A comprehensive Rust SDK for the [Model Context Protocol (MCP)](https://modelcontextprotocol.io/)
//! version 2025-03-26, providing both server and client implementations with full MCP specification 
//! compliance including audio content, annotations, and enhanced capabilities.
//!
//! ## Features
//!
//! - 🚀 **High Performance**: Built with Rust's zero-cost abstractions and async/await
//! - 🛡️ **Type Safety**: Leverages Rust's type system to prevent runtime errors
//! - 🔌 **Multiple Transports**: Support for STDIO, HTTP/SSE, and WebSocket transports
//! - 🎯 **Full MCP 2025-03-26 Compliance**: Complete implementation of the latest MCP specification
//! - 📚 **Rich Ecosystem**: Tools, resources, prompts, and sampling support
//! - 🎵 **Audio Support**: NEW in 2025-03-26 - Audio content support for multimodal interactions
//! - 🏷️ **Annotations**: NEW in 2025-03-26 - Tool and content annotations for enhanced metadata
//! - 🔧 **Autocompletion**: NEW in 2025-03-26 - Argument autocompletion capabilities
//! - 📁 **Roots Support**: NEW in 2025-03-26 - File system roots for enhanced resource access
//!
//! ## Quick Start
//!
//! ### Server Example
//!
//! ```rust,no_run
//! use mcp_protocol_sdk::{
//!     server::McpServer,
//!     core::{tool::ToolHandler, error::McpResult},
//!     protocol::types::{Content, CallToolResult},
//! };
//! use async_trait::async_trait;
//! use std::collections::HashMap;
//! use serde_json::Value;
//!
//! struct EchoHandler;
//!
//! #[async_trait]
//! impl ToolHandler for EchoHandler {
//!     async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<CallToolResult> {
//!         let message = arguments.get("message")
//!             .and_then(|v| v.as_str())
//!             .unwrap_or("Hello, World!");
//!         
//!         Ok(CallToolResult {
//!             content: vec![Content::text(message)],
//!             is_error: Some(false),
//!             meta: None,
//!         })
//!     }
//! }
//!
//! #[tokio::main]
//! async fn main() -> McpResult<()> {
//!     let mut server = McpServer::new("echo-server".to_string(), "1.0.0".to_string());
//!     
//!     server.add_tool(
//!         "echo".to_string(),
//!         Some("Echo a message".to_string()),
//!         serde_json::json!({
//!             "type": "object",
//!             "properties": {
//!                 "message": { "type": "string" }
//!             }
//!         }),
//!         EchoHandler,
//!     ).await?;
//!     
//!     Ok(())
//! }
//! ```
//!
//! ## Module Organization
//!
//! - [`core`]: Core abstractions for resources, tools, prompts, and errors
//! - [`protocol`]: MCP protocol types and message definitions (2025-03-26)
//! - [`transport`]: Transport layer implementations (STDIO, HTTP, WebSocket)
//! - [`server`]: MCP server implementation and lifecycle management
//! - [`client`]: MCP client implementation and session management
//! - [`utils`]: Utility functions and helpers

pub mod client;
pub mod core;
pub mod protocol;
pub mod server;
pub mod transport;
pub mod utils;

// Re-export commonly used types for convenience
pub use core::error::{McpError, McpResult};
pub use protocol::types::*;

/// Prelude module for convenient imports (2025-03-26)
pub mod prelude {
    pub use crate::client::McpClient;
    pub use crate::core::{
        error::{McpError, McpResult},
        prompt::{Prompt, PromptHandler},
        resource::{Resource, ResourceHandler},
        tool::{Tool, ToolHandler},
    };
    pub use crate::protocol::types::*;
    pub use crate::server::McpServer;
    pub use async_trait::async_trait;
    pub use serde_json::{json, Value};
    pub use std::collections::HashMap;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_exports() {
        // Basic smoke test to ensure all modules are accessible
        let _error = McpError::Protocol("test".to_string());
    }
}
