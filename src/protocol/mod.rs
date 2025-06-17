//! MCP protocol implementation (2025-03-26)
//!
//! This module contains the core protocol types and message handling for the
//! Model Context Protocol version 2025-03-26, including JSON-RPC message
//! serialization, validation, and new features like audio content, annotations,
//! and enhanced capabilities.

pub mod messages;
pub mod methods;
pub mod missing_types;
pub mod types;
pub mod types_2025;
pub mod validation;

// Re-export commonly used types and constants
pub use messages::*;
pub use missing_types::*;
pub use types::*;
pub use validation::*;

// Re-export method constants for convenience
pub use methods::{
    CANCELLED, COMPLETION_COMPLETE, INITIALIZE, INITIALIZED, LOGGING_MESSAGE, LOGGING_SET_LEVEL,
    PING, PROGRESS, PROMPTS_GET, PROMPTS_LIST, PROMPTS_LIST_CHANGED, RESOURCES_LIST,
    RESOURCES_LIST_CHANGED, RESOURCES_READ, RESOURCES_SUBSCRIBE, RESOURCES_TEMPLATES_LIST,
    RESOURCES_UNSUBSCRIBE, RESOURCES_UPDATED, ROOTS_LIST, ROOTS_LIST_CHANGED,
    SAMPLING_CREATE_MESSAGE, TOOLS_CALL, TOOLS_LIST, TOOLS_LIST_CHANGED,
};

// Re-export 2025-03-26 specific constants and error codes
pub use types::error_codes;
pub use types::{JSONRPC_VERSION, LATEST_PROTOCOL_VERSION};

// Legacy constant for compatibility
pub const MCP_PROTOCOL_VERSION: &str = LATEST_PROTOCOL_VERSION;

// Export types_2025 for comprehensive tests
pub use types_2025 as types_2025_comprehensive;
