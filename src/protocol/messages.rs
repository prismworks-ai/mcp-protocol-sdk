//! MCP Protocol Messages
//!
//! This module defines all protocol message types used in MCP communication,
//! aligned with the 2025-03-26 specification.

use crate::protocol::types::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Request Parameter Types
// ============================================================================

/// Parameters for initialize request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitializeParams {
    /// Protocol version client supports
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    /// Client capabilities
    pub capabilities: ClientCapabilities,
    /// Client implementation info
    #[serde(rename = "clientInfo")]
    pub client_info: Implementation,
    /// Request metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for tool call request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallToolParams {
    /// Name of the tool to call
    pub name: String,
    /// Arguments to pass to the tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<HashMap<String, serde_json::Value>>,
    /// Request metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for resource read request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReadResourceParams {
    /// URI of the resource to read
    pub uri: String,
    /// Request metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for resource subscription request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubscribeResourceParams {
    /// URI of the resource to subscribe to
    pub uri: String,
    /// Request metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for resource unsubscription request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnsubscribeResourceParams {
    /// URI of the resource to unsubscribe from
    pub uri: String,
    /// Request metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for prompt get request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetPromptParams {
    /// Name of the prompt
    pub name: String,
    /// Arguments for prompt templating
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<HashMap<String, String>>,
    /// Request metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for list requests (with pagination)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListParams {
    /// Pagination cursor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Request metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for list tools request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListToolsParams {
    /// Pagination cursor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Request metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for list resources request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResourcesParams {
    /// Pagination cursor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Request metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for list prompts request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListPromptsParams {
    /// Pagination cursor
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<String>,
    /// Request metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for ping request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PingParams {
    /// Request metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for completion request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompleteParams {
    /// Reference to the item being completed
    #[serde(rename = "ref")]
    pub reference: CompletionReference,
    /// Argument being completed
    pub argument: CompletionArgument,
    /// Request metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Reference for completion
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum CompletionReference {
    #[serde(rename = "ref/prompt")]
    Prompt { name: String },
    #[serde(rename = "ref/resource")]
    Resource { uri: String },
    #[serde(rename = "ref/tool")]
    Tool { name: String },
}

/// Argument for completion
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompletionArgument {
    /// Name of the argument
    pub name: String,
    /// Current value for completion
    pub value: String,
}

/// Parameters for sampling/createMessage request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateMessageParams {
    /// Messages in the conversation
    pub messages: Vec<SamplingMessage>,
    /// Maximum tokens to generate
    #[serde(rename = "maxTokens")]
    pub max_tokens: u32,
    /// Optional system prompt
    #[serde(rename = "systemPrompt", skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    /// Include context from servers
    #[serde(rename = "includeContext", skip_serializing_if = "Option::is_none")]
    pub include_context: Option<String>,
    /// Temperature for sampling
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f32>,
    /// Stop sequences
    #[serde(rename = "stopSequences", skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    /// Model preferences
    #[serde(rename = "modelPreferences", skip_serializing_if = "Option::is_none")]
    pub model_preferences: Option<ModelPreferences>,
    /// Provider-specific metadata
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, serde_json::Value>>,
    /// Request metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for logging level set request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetLoggingLevelParams {
    /// Logging level to set
    pub level: LoggingLevel,
    /// Request metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

// ============================================================================
// Response Result Types
// ============================================================================

/// Result for initialize request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitializeResult {
    /// Protocol version server supports
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    /// Server capabilities
    pub capabilities: ServerCapabilities,
    /// Server implementation info
    #[serde(rename = "serverInfo")]
    pub server_info: Implementation,
    /// Optional instructions for the client
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Response metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Result for list tools request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListToolsResult {
    /// Available tools
    pub tools: Vec<Tool>,
    /// Next cursor for pagination
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Response metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Result for list resources request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResourcesResult {
    /// Available resources
    pub resources: Vec<Resource>,
    /// Next cursor for pagination
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Response metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Result for list resource templates request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResourceTemplatesResult {
    /// Available resource templates
    #[serde(rename = "resourceTemplates")]
    pub resource_templates: Vec<ResourceTemplate>,
    /// Next cursor for pagination
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Response metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Result for read resource request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReadResourceResult {
    /// Resource contents
    pub contents: Vec<ResourceContents>,
    /// Response metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Result for list prompts request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListPromptsResult {
    /// Available prompts
    pub prompts: Vec<Prompt>,
    /// Next cursor for pagination
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<String>,
    /// Response metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Result for completion request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompleteResult {
    /// Completion information
    pub completion: CompletionData,
    /// Response metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Completion data
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompletionData {
    /// Completion values
    pub values: Vec<String>,
    /// Total number of completions available
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u32>,
    /// Whether there are more completions available
    #[serde(rename = "hasMore", skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

/// Result for list roots request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRootsResult {
    /// Available roots
    pub roots: Vec<Root>,
    /// Response metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Result for ping request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PingResult {
    /// Response metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Result for set logging level request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetLoggingLevelResult {
    /// Response metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Result for subscribe resource request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubscribeResourceResult {
    /// Response metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Result for unsubscribe resource request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnsubscribeResourceResult {
    /// Response metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Root definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Root {
    /// URI of the root
    pub uri: String,
    /// Optional name for the root
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

// ============================================================================
// Notification Parameter Types
// ============================================================================

/// Parameters for progress notification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProgressParams {
    /// Progress token from original request
    #[serde(rename = "progressToken")]
    pub progress_token: ProgressToken,
    /// Current progress value
    pub progress: f32,
    /// Total progress expected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f32>,
    /// Optional progress message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Parameters for resource updated notification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceUpdatedParams {
    /// URI of the updated resource
    pub uri: String,
}

/// Parameters for cancelled notification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CancelledParams {
    /// ID of the request being cancelled
    #[serde(rename = "requestId")]
    pub request_id: RequestId,
    /// Optional reason for cancellation
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
}

/// Parameters for initialized notification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitializedParams {
    /// Notification metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for logging message notification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoggingMessageParams {
    /// Logging level
    pub level: LoggingLevel,
    /// Logger name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logger: Option<String>,
    /// Log data
    pub data: serde_json::Value,
}

/// Parameters for tool list changed notification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolListChangedParams {
    /// Response metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for resource list changed notification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceListChangedParams {
    /// Response metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for prompt list changed notification
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptListChangedParams {
    /// Response metadata
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for progress notification (alias for better naming)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProgressNotificationParams {
    /// Progress token from original request
    #[serde(rename = "progressToken")]
    pub progress_token: ProgressToken,
    /// Current progress value
    pub progress: f32,
    /// Total progress expected
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<f32>,
    /// Optional progress message
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
}

/// Parameters for logging message notification (alias for better naming)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoggingMessageNotificationParams {
    /// Logging level
    pub level: LoggingLevel,
    /// Logger name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logger: Option<String>,
    /// Log data
    pub data: serde_json::Value,
}

// ============================================================================
// Helper Constructors
// ============================================================================

impl CallToolParams {
    pub fn new(name: String) -> Self {
        Self {
            name,
            arguments: None,
            meta: None,
        }
    }

    pub fn new_with_arguments(name: String, arguments: HashMap<String, serde_json::Value>) -> Self {
        Self {
            name,
            arguments: Some(arguments),
            meta: None,
        }
    }

    pub fn with_arguments(mut self, arguments: HashMap<String, serde_json::Value>) -> Self {
        self.arguments = Some(arguments);
        self
    }
}

impl ReadResourceParams {
    pub fn new(uri: String) -> Self {
        Self { uri, meta: None }
    }
}

impl GetPromptParams {
    pub fn new(name: String) -> Self {
        Self {
            name,
            arguments: None,
            meta: None,
        }
    }

    pub fn new_with_arguments(name: String, arguments: HashMap<String, String>) -> Self {
        Self {
            name,
            arguments: Some(arguments),
            meta: None,
        }
    }

    pub fn with_arguments(mut self, arguments: HashMap<String, String>) -> Self {
        self.arguments = Some(arguments);
        self
    }
}

impl InitializeParams {
    pub fn new(
        protocol_version: String,
        capabilities: ClientCapabilities,
        client_info: Implementation,
    ) -> Self {
        Self {
            protocol_version,
            capabilities,
            client_info,
            meta: None,
        }
    }
}

impl InitializeResult {
    pub fn new(
        protocol_version: String,
        capabilities: ServerCapabilities,
        server_info: Implementation,
    ) -> Self {
        Self {
            protocol_version,
            capabilities,
            server_info,
            instructions: None,
            meta: None,
        }
    }
}

impl Root {
    pub fn new(uri: String) -> Self {
        Self { uri, name: None }
    }

    pub fn with_name(mut self, name: String) -> Self {
        self.name = Some(name);
        self
    }
}

// ============================================================================
// Default Implementations
// ============================================================================

impl Default for ListToolsParams {
    fn default() -> Self {
        Self {
            cursor: None,
            meta: None,
        }
    }
}

impl Default for ListResourcesParams {
    fn default() -> Self {
        Self {
            cursor: None,
            meta: None,
        }
    }
}

impl Default for ListPromptsParams {
    fn default() -> Self {
        Self {
            cursor: None,
            meta: None,
        }
    }
}

impl Default for PingParams {
    fn default() -> Self {
        Self { meta: None }
    }
}
