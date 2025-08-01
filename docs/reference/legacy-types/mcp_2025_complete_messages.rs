//! Complete MCP Protocol Messages for 2025-03-26 Specification
//!
//! This module contains all MCP-specific message types with full 2025-03-26 compliance,
//! including new features like metadata support, audio content, and enhanced capabilities.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::protocol::types::*;

// ============================================================================
// Initialization Messages (2025-03-26)
// ============================================================================

/// Parameters for the initialize request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitializeParams {
    /// Information about the client
    #[serde(rename = "clientInfo")]
    pub client_info: Implementation,
    /// Capabilities advertised by the client
    pub capabilities: ClientCapabilities,
    /// Protocol version being used (must be "2025-03-26")
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
}

/// Result of the initialize request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitializeResult {
    /// Information about the server
    #[serde(rename = "serverInfo")]
    pub server_info: Implementation,
    /// Capabilities advertised by the server
    pub capabilities: ServerCapabilities,
    /// Protocol version being used (must be "2025-03-26")
    #[serde(rename = "protocolVersion")]
    pub protocol_version: String,
    /// Instructions for using the server (2025-03-26)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    /// Result metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Initialized notification (sent after initialization completes)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct InitializedNotificationParams {
    /// Notification metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

// ============================================================================
// Tool Messages (2025-03-26 with Annotations)
// ============================================================================

/// Parameters for the tools/list request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListToolsParams {
    /// Optional cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Cursor>,
    /// Request metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<RequestMeta>,
}

/// Result of the tools/list request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListToolsResult {
    /// Available tools (with annotations support)
    pub tools: Vec<Tool>,
    /// Cursor for pagination (if more tools are available)
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<Cursor>,
    /// Result metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for the tools/call request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallToolParams {
    /// Name of the tool to call
    pub name: String,
    /// Arguments to pass to the tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<HashMap<String, serde_json::Value>>,
    /// Request metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<RequestMeta>,
}

/// Tools list changed notification parameters (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolsListChangedNotificationParams {
    /// Notification metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

// ============================================================================
// Resource Messages (2025-03-26)
// ============================================================================

/// Parameters for the resources/list request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResourcesParams {
    /// Optional cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Cursor>,
    /// Request metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<RequestMeta>,
}

/// Result of the resources/list request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResourcesResult {
    /// Available resources
    pub resources: Vec<Resource>,
    /// Cursor for pagination (if more resources are available)
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<Cursor>,
    /// Result metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for the resources/templates/list request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListResourceTemplatesParams {
    /// Optional cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Cursor>,
    /// Request metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<RequestMeta>,
}

/// Result of the resources/templates/list request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListResourceTemplatesResult {
    /// Available resource templates
    #[serde(rename = "resourceTemplates")]
    pub resource_templates: Vec<ResourceTemplate>,
    /// Cursor for pagination
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<Cursor>,
    /// Result metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for the resources/read request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReadResourceParams {
    /// URI of the resource to read
    pub uri: String,
    /// Request metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<RequestMeta>,
}

/// Result of the resources/read request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ReadResourceResult {
    /// Contents of the resource
    pub contents: Vec<ResourceContents>,
    /// Result metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for the resources/subscribe request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubscribeParams {
    /// URI of the resource to subscribe to
    pub uri: String,
    /// Request metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<RequestMeta>,
}

/// Result of the resources/subscribe request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SubscribeResult {
    /// Result metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for the resources/unsubscribe request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnsubscribeParams {
    /// URI of the resource to unsubscribe from
    pub uri: String,
    /// Request metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<RequestMeta>,
}

/// Result of the resources/unsubscribe request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct UnsubscribeResult {
    /// Result metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for the resources/updated notification (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceUpdatedNotificationParams {
    /// URI of the resource that was updated
    pub uri: String,
    /// Notification metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for the resources/list_changed notification (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourcesListChangedNotificationParams {
    /// Notification metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

// ============================================================================
// Prompt Messages (2025-03-26)
// ============================================================================

/// Parameters for the prompts/list request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ListPromptsParams {
    /// Optional cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Cursor>,
    /// Request metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<RequestMeta>,
}

/// Result of the prompts/list request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListPromptsResult {
    /// Available prompts
    pub prompts: Vec<Prompt>,
    /// Cursor for pagination (if more prompts are available)
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<Cursor>,
    /// Result metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for the prompts/get request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetPromptParams {
    /// Name of the prompt to get
    pub name: String,
    /// Arguments to pass to the prompt
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<HashMap<String, serde_json::Value>>,
    /// Request metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<RequestMeta>,
}

/// Parameters for the prompts/list_changed notification (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptsListChangedNotificationParams {
    /// Notification metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

// ============================================================================
// Sampling Messages (2025-03-26 with Audio Support)
// ============================================================================

/// Parameters for the sampling/createMessage request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateMessageParams {
    /// Messages to include in the conversation
    pub messages: Vec<SamplingMessage>,
    /// Model preferences
    #[serde(rename = "modelPreferences", skip_serializing_if = "Option::is_none")]
    pub model_preferences: Option<ModelPreferences>,
    /// System prompt
    #[serde(rename = "systemPrompt", skip_serializing_if = "Option::is_none")]
    pub system_prompt: Option<String>,
    /// Whether to include context from resources
    #[serde(rename = "includeContext", skip_serializing_if = "Option::is_none")]
    pub include_context: Option<String>,
    /// Maximum number of tokens to generate
    #[serde(rename = "maxTokens")]
    pub max_tokens: u32,
    /// Stop sequences
    #[serde(rename = "stopSequences", skip_serializing_if = "Option::is_none")]
    pub stop_sequences: Option<Vec<String>>,
    /// Optional metadata for the LLM provider
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<serde_json::Value>,
    /// Request metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<RequestMeta>,
}

// ============================================================================
// Roots Messages (2025-03-26 NEW)
// ============================================================================

/// Parameters for the roots/list request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRootsParams {
    /// Request metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<RequestMeta>,
}

/// Root definition (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Root {
    /// URI of the root
    pub uri: String,
    /// Human-readable name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
}

/// Result of the roots/list request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ListRootsResult {
    /// Available roots
    pub roots: Vec<Root>,
    /// Result metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for the roots/list_changed notification (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RootsListChangedNotificationParams {
    /// Notification metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

// ============================================================================
// Completion Messages (2025-03-26 NEW)
// ============================================================================

/// Parameters for the completion/complete request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompleteParams {
    /// Reference to the request for completion
    #[serde(rename = "ref")]
    pub reference: CompletionReference,
    /// Argument being completed
    pub argument: CompletionArgument,
    /// Request metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<RequestMeta>,
}

/// Reference to the request being completed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum CompletionReference {
    #[serde(rename = "ref/tool")]
    Tool { name: String },
    #[serde(rename = "ref/resource")]
    Resource { uri: String },
    #[serde(rename = "ref/prompt")]
    Prompt { name: String },
}

/// Argument being completed
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompletionArgument {
    /// Name of the argument
    pub name: String,
    /// Current value (partial input)
    pub value: String,
}

/// Result of the completion/complete request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CompleteResult {
    /// Completion values
    pub completion: Completion,
    /// Result metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Completion values
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Completion {
    /// Possible completion values
    pub values: Vec<String>,
    /// Total number of completion options
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u32>,
    /// Whether there are more completions available
    #[serde(rename = "hasMore", skip_serializing_if = "Option::is_none")]
    pub has_more: Option<bool>,
}

// ============================================================================
// Logging Messages (2025-03-26)
// ============================================================================

/// Parameters for the logging/setLevel request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetLoggingLevelParams {
    /// The logging level to set
    pub level: LoggingLevel,
    /// Request metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<RequestMeta>,
}

/// Result of the logging/setLevel request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SetLoggingLevelResult {
    /// Result metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Parameters for the logging/message notification (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct LoggingMessageNotificationParams {
    /// The logging level
    pub level: LoggingLevel,
    /// The logger name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logger: Option<String>,
    /// The log message data
    pub data: serde_json::Value,
    /// Notification metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

// ============================================================================
// Progress Messages (2025-03-26 Enhanced)
// ============================================================================

/// Parameters for the progress notification (2025-03-26 enhanced)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ProgressNotificationParams {
    /// Unique identifier for the progress operation
    #[serde(rename = "progressToken")]
    pub progress_token: ProgressToken,
    /// Current progress (0.0 to 1.0)
    pub progress: f64,
    /// Optional total count
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total: Option<u32>,
    /// Descriptive status message (2025-03-26 NEW)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    /// Notification metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

// ============================================================================
// Ping Messages (2025-03-26)
// ============================================================================

/// Parameters for the ping request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PingParams {
    /// Request metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<RequestMeta>,
}

/// Result of the ping request (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PingResult {
    /// Result metadata (2025-03-26)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

// ============================================================================
// Method Constants (2025-03-26)
// ============================================================================

/// JSON-RPC method names for MCP messages (2025-03-26)
pub mod methods {
    /// Initialize the connection
    pub const INITIALIZE: &str = "initialize";
    /// Initialization complete notification
    pub const INITIALIZED: &str = "notifications/initialized";

    /// Ping to check connection
    pub const PING: &str = "ping";

    /// List available tools
    pub const TOOLS_LIST: &str = "tools/list";
    /// Call a tool
    pub const TOOLS_CALL: &str = "tools/call";
    /// Notification when tool list changes
    pub const TOOLS_LIST_CHANGED: &str = "notifications/tools/list_changed";

    /// List available resources
    pub const RESOURCES_LIST: &str = "resources/list";
    /// List resource templates (2025-03-26)
    pub const RESOURCES_TEMPLATES_LIST: &str = "resources/templates/list";
    /// Read a resource
    pub const RESOURCES_READ: &str = "resources/read";
    /// Subscribe to resource updates
    pub const RESOURCES_SUBSCRIBE: &str = "resources/subscribe";
    /// Unsubscribe from resource updates
    pub const RESOURCES_UNSUBSCRIBE: &str = "resources/unsubscribe";
    /// Notification when a resource is updated
    pub const RESOURCES_UPDATED: &str = "notifications/resources/updated";
    /// Notification when resource list changes
    pub const RESOURCES_LIST_CHANGED: &str = "notifications/resources/list_changed";

    /// List available prompts
    pub const PROMPTS_LIST: &str = "prompts/list";
    /// Get a prompt
    pub const PROMPTS_GET: &str = "prompts/get";
    /// Notification when prompt list changes
    pub const PROMPTS_LIST_CHANGED: &str = "notifications/prompts/list_changed";

    /// Create a message using sampling
    pub const SAMPLING_CREATE_MESSAGE: &str = "sampling/createMessage";

    /// List roots (2025-03-26 NEW)
    pub const ROOTS_LIST: &str = "roots/list";
    /// Notification when roots list changes (2025-03-26 NEW)
    pub const ROOTS_LIST_CHANGED: &str = "notifications/roots/list_changed";

    /// Complete argument (2025-03-26 NEW)
    pub const COMPLETION_COMPLETE: &str = "completion/complete";

    /// Set logging level
    pub const LOGGING_SET_LEVEL: &str = "logging/setLevel";
    /// Log message notification
    pub const LOGGING_MESSAGE: &str = "notifications/logging/message";

    /// Progress notification (enhanced in 2025-03-26)
    pub const PROGRESS: &str = "notifications/progress";
}

// ============================================================================
// Helper Constructors
// ============================================================================

impl InitializeParams {
    /// Create new initialize parameters (2025-03-26)
    pub fn new(
        client_info: Implementation,
        capabilities: ClientCapabilities,
    ) -> Self {
        Self {
            client_info,
            capabilities,
            protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
        }
    }
}

impl InitializeResult {
    /// Create new initialize result (2025-03-26)
    pub fn new(
        server_info: Implementation,
        capabilities: ServerCapabilities,
        instructions: Option<String>,
    ) -> Self {
        Self {
            server_info,
            capabilities,
            protocol_version: LATEST_PROTOCOL_VERSION.to_string(),
            instructions,
            meta: None,
        }
    }
}

impl CallToolParams {
    /// Create new call tool parameters (2025-03-26)
    pub fn new(name: String, arguments: Option<HashMap<String, serde_json::Value>>) -> Self {
        Self { 
            name, 
            arguments,
            meta: None,
        }
    }

    /// Add metadata to the request
    pub fn with_metadata(mut self, meta: RequestMeta) -> Self {
        self.meta = Some(meta);
        self
    }

    /// Add progress token for tracking
    pub fn with_progress_token(mut self, token: ProgressToken) -> Self {
        let meta = RequestMeta {
            progress_token: Some(token),
        };
        self.meta = Some(meta);
        self
    }
}

impl ReadResourceParams {
    /// Create new read resource parameters (2025-03-26)
    pub fn new(uri: String) -> Self {
        Self { 
            uri,
            meta: None,
        }
    }
}

impl GetPromptParams {
    /// Create new get prompt parameters (2025-03-26)
    pub fn new(name: String, arguments: Option<HashMap<String, serde_json::Value>>) -> Self {
        Self { 
            name, 
            arguments,
            meta: None,
        }
    }
}

impl CreateMessageParams {
    /// Create new sampling parameters (2025-03-26)
    pub fn new(messages: Vec<SamplingMessage>, max_tokens: u32) -> Self {
        Self {
            messages,
            model_preferences: None,
            system_prompt: None,
            include_context: None,
            max_tokens,
            stop_sequences: None,
            metadata: None,
            meta: None,
        }
    }

    /// Add model preferences
    pub fn with_model_preferences(mut self, preferences: ModelPreferences) -> Self {
        self.model_preferences = Some(preferences);
        self
    }

    /// Add system prompt
    pub fn with_system_prompt<S: Into<String>>(mut self, prompt: S) -> Self {
        self.system_prompt = Some(prompt.into());
        self
    }
}

impl SamplingMessage {
    /// Create a user message (2025-03-26 with audio support)
    pub fn user_text<S: Into<String>>(content: S) -> Self {
        Self {
            role: Role::User,
            content: Content::text(content),
        }
    }

    /// Create an assistant message (2025-03-26 with audio support)
    pub fn assistant_text<S: Into<String>>(content: S) -> Self {
        Self {
            role: Role::Assistant,
            content: Content::text(content),
        }
    }

    /// Create a user message with image (2025-03-26)
    pub fn user_image<S: Into<String>>(data: S, mime_type: S) -> Self {
        Self {
            role: Role::User,
            content: Content::image(data, mime_type),
        }
    }

    /// Create a user message with audio (2025-03-26 NEW)
    pub fn user_audio<S: Into<String>>(data: S, mime_type: S) -> Self {
        Self {
            role: Role::User,
            content: Content::audio(data, mime_type),
        }
    }

    /// Create a message with embedded resource (2025-03-26 NEW)
    pub fn user_resource<S: Into<String>>(uri: S) -> Self {
        Self {
            role: Role::User,
            content: Content::resource(uri),
        }
    }
}

impl ProgressNotificationParams {
    /// Create progress notification with message (2025-03-26)
    pub fn with_message(
        progress_token: ProgressToken,
        progress: f64,
        message: String,
    ) -> Self {
        Self {
            progress_token,
            progress,
            total: None,
            message: Some(message),
            meta: None,
        }
    }

    /// Create progress notification with total and message (2025-03-26)
    pub fn with_total_and_message(
        progress_token: ProgressToken,
        progress: f64,
        total: u32,
        message: String,
    ) -> Self {
        Self {
            progress_token,
            progress,
            total: Some(total),
            message: Some(message),
            meta: None,
        }
    }
}

impl CallToolResult {
    /// Create successful tool result (2025-03-26)
    pub fn success(content: Vec<Content>) -> Self {
        Self {
            content,
            is_error: Some(false),
            meta: None,
        }
    }

    /// Create error tool result (2025-03-26)
    pub fn error(content: Vec<Content>) -> Self {
        Self {
            content,
            is_error: Some(true),
            meta: None,
        }
    }

    /// Add metadata to result
    pub fn with_metadata(mut self, meta: HashMap<String, serde_json::Value>) -> Self {
        self.meta = Some(meta);
        self
    }
}

impl GetPromptResult {
    /// Create prompt result (2025-03-26)
    pub fn new(messages: Vec<PromptMessage>) -> Self {
        Self {
            description: None,
            messages,
            meta: None,
        }
    }

    /// Add description
    pub fn with_description<S: Into<String>>(mut self, description: S) -> Self {
        self.description = Some(description.into());
        self
    }
}

impl CreateMessageResult {
    /// Create message result (2025-03-26)
    pub fn new(role: Role, content: Content, model: String) -> Self {
        Self {
            role,
            content,
            model,
            stop_reason: None,
            meta: None,
        }
    }

    /// Add stop reason
    pub fn with_stop_reason(mut self, reason: StopReason) -> Self {
        self.stop_reason = Some(reason);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_initialize_params_2025() {
        let client_info = Implementation {
            name: "test-client".to_string(),
            version: "1.0.0".to_string(),
        };
        let capabilities = ClientCapabilities::default();
        
        let params = InitializeParams::new(client_info, capabilities);
        
        assert_eq!(params.protocol_version, LATEST_PROTOCOL_VERSION);
        assert_eq!(params.client_info.name, "test-client");
        
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["protocolVersion"], "2025-03-26");
        assert_eq!(json["clientInfo"]["name"], "test-client");
    }

    #[test]
    fn test_call_tool_with_metadata() {
        let mut args = HashMap::new();
        args.insert("param1".to_string(), json!("value1"));
        
        let params = CallToolParams::new("test_tool".to_string(), Some(args))
            .with_progress_token(json!("progress-123"));
        
        assert_eq!(params.name, "test_tool");
        assert!(params.meta.is_some());
        assert_eq!(params.meta.unwrap().progress_token, Some(json!("progress-123")));
    }

    #[test]
    fn test_sampling_message_with_audio() {
        // Test text message
        let text_msg = SamplingMessage::user_text("Hello, world!");
        assert_eq!(text_msg.role, Role::User);
        
        // Test audio message (new in 2025-03-26)
        let audio_msg = SamplingMessage::user_audio("base64data", "audio/wav");
        assert_eq!(audio_msg.role, Role::User);
        
        let json = serde_json::to_value(&audio_msg).unwrap();
        assert_eq!(json["content"]["type"], "audio");
        assert_eq!(json["content"]["mimeType"], "audio/wav");
    }

    #[test]
    fn test_progress_with_message() {
        let progress = ProgressNotificationParams::with_message(
            json!("token-123"),
            0.5,
            "Processing files...".to_string(),
        );
        
        assert_eq!(progress.progress, 0.5);
        assert_eq!(progress.message, Some("Processing files...".to_string()));
        
        let json = serde_json::to_value(&progress).unwrap();
        assert_eq!(json["progressToken"], "token-123");
        assert_eq!(json["progress"], 0.5);
        assert_eq!(json["message"], "Processing files...");
    }

    #[test]
    fn test_tool_result_with_metadata() {
        let content = vec![Content::text("Operation successful")];
        let mut meta = HashMap::new();
        meta.insert("execution_time".to_string(), json!(1.5));
        
        let result = CallToolResult::success(content)
            .with_metadata(meta);
        
        assert_eq!(result.is_error, Some(false));
        assert!(result.meta.is_some());
        assert_eq!(
            result.meta.unwrap().get("execution_time"),
            Some(&json!(1.5))
        );
    }

    #[test]
    fn test_complete_params() {
        let params = CompleteParams {
            reference: CompletionReference::Tool {
                name: "test_tool".to_string(),
            },
            argument: CompletionArgument {
                name: "file_path".to_string(),
                value: "/home/user/".to_string(),
            },
            meta: None,
        };
        
        let json = serde_json::to_value(&params).unwrap();
        assert_eq!(json["ref"]["type"], "ref/tool");
        assert_eq!(json["ref"]["name"], "test_tool");
        assert_eq!(json["argument"]["name"], "file_path");
        assert_eq!(json["argument"]["value"], "/home/user/");
    }

    #[test]
    fn test_all_method_constants() {
        // Verify all method constants are correct for 2025-03-26
        assert_eq!(methods::INITIALIZE, "initialize");
        assert_eq!(methods::TOOLS_CALL, "tools/call");
        assert_eq!(methods::RESOURCES_READ, "resources/read");
        assert_eq!(methods::PROMPTS_GET, "prompts/get");
        assert_eq!(methods::SAMPLING_CREATE_MESSAGE, "sampling/createMessage");
        
        // New in 2025-03-26
        assert_eq!(methods::ROOTS_LIST, "roots/list");
        assert_eq!(methods::COMPLETION_COMPLETE, "completion/complete");
        assert_eq!(methods::RESOURCES_TEMPLATES_LIST, "resources/templates/list");
    }

    #[test]
    fn test_server_capabilities_completions() {
        let caps = ServerCapabilities {
            completions: Some(CompletionsCapability::default()),
            logging: Some(LoggingCapability::default()),
            experimental: Some(HashMap::new()),
            ..Default::default()
        };
        
        let json = serde_json::to_value(&caps).unwrap();
        assert!(json.get("completions").is_some());
        assert!(json.get("logging").is_some());
        assert!(json.get("experimental").is_some());
    }

    #[test]
    fn test_embedded_resource_content() {
        let resource_content = Content::resource("file:///test.txt");
        let json = serde_json::to_value(&resource_content).unwrap();
        
        assert_eq!(json["type"], "resource");
        assert_eq!(json["resource"]["uri"], "file:///test.txt");
    }
}