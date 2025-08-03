//! MCP Protocol Method Constants
//!
//! This module contains all the method name constants used in the MCP protocol
//! as defined in the 2025-03-26 specification.

// Core protocol methods
pub const INITIALIZE: &str = "initialize";
pub const INITIALIZED: &str = "notifications/initialized";
pub const PING: &str = "ping";

// Tool-related methods
pub const TOOLS_LIST: &str = "tools/list";
pub const TOOLS_CALL: &str = "tools/call";
pub const TOOLS_LIST_CHANGED: &str = "notifications/tools/list_changed";

// Resource-related methods
pub const RESOURCES_LIST: &str = "resources/list";
pub const RESOURCES_TEMPLATES_LIST: &str = "resources/templates/list"; // New in 2025-06-18
pub const RESOURCES_READ: &str = "resources/read";
pub const RESOURCES_SUBSCRIBE: &str = "resources/subscribe";
pub const RESOURCES_UNSUBSCRIBE: &str = "resources/unsubscribe";
pub const RESOURCES_UPDATED: &str = "notifications/resources/updated";
pub const RESOURCES_LIST_CHANGED: &str = "notifications/resources/list_changed";

// Prompt-related methods
pub const PROMPTS_LIST: &str = "prompts/list";
pub const PROMPTS_GET: &str = "prompts/get";
pub const PROMPTS_LIST_CHANGED: &str = "notifications/prompts/list_changed";

// Sampling methods
pub const SAMPLING_CREATE_MESSAGE: &str = "sampling/createMessage";

// Root-related methods (New in 2025-06-18)
pub const ROOTS_LIST: &str = "roots/list";
pub const ROOTS_LIST_CHANGED: &str = "notifications/roots/list_changed";

// Completion methods (New in 2025-06-18)
pub const COMPLETION_COMPLETE: &str = "completion/complete";

// Logging methods
pub const LOGGING_SET_LEVEL: &str = "logging/setLevel";
pub const LOGGING_MESSAGE: &str = "notifications/message";

// Progress and notification methods
pub const PROGRESS: &str = "notifications/progress";
pub const CANCELLED: &str = "notifications/cancelled"; // New in 2025-06-18
