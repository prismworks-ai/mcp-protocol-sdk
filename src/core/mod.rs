//! Core abstractions and types for the MCP SDK
//!
//! This module contains the fundamental building blocks for MCP implementations,
//! including error handling, resource management, tool execution, and prompt handling.

pub mod error;
pub mod prompt;
pub mod resource;
pub mod tool;
pub mod tool_discovery;
pub mod tool_metadata;
pub mod validation;

// Re-export commonly used items
pub use error::{McpError, McpResult};
pub use prompt::{Prompt, PromptHandler};
pub use resource::{Resource, ResourceHandler, ResourceTemplate};
pub use tool::{Tool, ToolHandler, ToolBuilder};
pub use tool_discovery::{ToolRegistry, DiscoveryCriteria, DiscoveryResult, GlobalToolStats, DeprecationCleanupPolicy};
pub use tool_metadata::{
    EnhancedToolMetadata, ToolBehaviorHints, ToolCategory, ToolDeprecation, 
    CategoryFilter, DeprecationSeverity
};
pub use validation::{ParameterValidator, ValidationConfig, ParameterType};

// Re-export protocol types through core for convenience
pub use crate::protocol::types::{
    PromptArgument, PromptInfo, PromptMessage, PromptResult, ResourceInfo, ToolInfo,
};
