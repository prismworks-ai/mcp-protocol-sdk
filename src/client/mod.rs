//! MCP client implementation
//!
//! This module provides the main client implementation for the Model Context Protocol.

pub mod mcp_client;
pub mod session;

// Re-export the main client type
pub use mcp_client::McpClient;
pub use session::ClientSession;
