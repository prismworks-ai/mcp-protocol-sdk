//! MCP client implementation
//!
//! This module provides the main client implementation for the Model Context Protocol.

pub mod builder;
pub mod mcp_client;
pub mod session;

// Re-export the main client type and builder
pub use builder::{ConnectionConfig, McpClientBuilder, RetryConfig};
pub use mcp_client::McpClient;
pub use session::{ClientSession, SessionConfig, SessionState};

// Legacy alias for test compatibility
pub type ClientBuilder = McpClientBuilder;
