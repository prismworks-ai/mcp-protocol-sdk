//! MCP protocol implementation (2025-03-26)
//!
//! This module contains the core protocol types and message handling for the
//! Model Context Protocol version 2025-03-26, including JSON-RPC message
//! serialization, validation, and new features like audio content, annotations,
//! and enhanced capabilities.

pub mod messages;
pub mod types;
pub mod validation;

// Re-export commonly used types and constants
pub use messages::methods;
pub use types::*;
pub use validation::*;

// Re-export 2025-03-26 specific constants and error codes
pub use types::error_codes;
pub use types::{JSONRPC_VERSION, LATEST_PROTOCOL_VERSION};

// Legacy constant for compatibility
pub const MCP_PROTOCOL_VERSION: &str = LATEST_PROTOCOL_VERSION;
