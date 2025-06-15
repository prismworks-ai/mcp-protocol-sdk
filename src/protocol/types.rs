//! Complete MCP Protocol Types for 2025-03-26 Specification
//!
//! This module contains all the core types defined by the Model Context Protocol
//! specification version 2025-03-26, including all new features and capabilities.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Core Protocol Constants
// ============================================================================

/// MCP Protocol version (2025-03-26)
pub const LATEST_PROTOCOL_VERSION: &str = "2025-03-26";
pub const JSONRPC_VERSION: &str = "2.0";

// ============================================================================
// Type Aliases
// ============================================================================

/// Progress token for associating notifications with requests
pub type ProgressToken = serde_json::Value; // string | number

/// Cursor for pagination
pub type Cursor = String;

/// Request ID for JSON-RPC correlation
pub type RequestId = serde_json::Value; // string | number | null

// ============================================================================
// Core Implementation Info
// ============================================================================

/// Information about an MCP implementation
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Implementation {
    /// Name of the implementation
    pub name: String,
    /// Version of the implementation
    pub version: String,
}

// Type aliases for compatibility
pub type ServerInfo = Implementation;
pub type ClientInfo = Implementation;

// ============================================================================
// Capabilities (2025-03-26)
// ============================================================================

/// Server capabilities for 2025-03-26
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ServerCapabilities {
    /// Prompt-related capabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompts: Option<PromptsCapability>,
    /// Resource-related capabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub resources: Option<ResourcesCapability>,
    /// Tool-related capabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<ToolsCapability>,
    /// Sampling-related capabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<SamplingCapability>,
    /// Logging capabilities (2025-03-26)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logging: Option<LoggingCapability>,
    /// Autocompletion capabilities (2025-03-26)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub completions: Option<CompletionsCapability>,
    /// Experimental capabilities (2025-03-26)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<HashMap<String, serde_json::Value>>,
}

/// Client capabilities for 2025-03-26
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ClientCapabilities {
    /// Sampling-related capabilities
    #[serde(skip_serializing_if = "Option::is_none")]
    pub sampling: Option<SamplingCapability>,
    /// Roots listing capabilities (2025-03-26)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub roots: Option<RootsCapability>,
    /// Experimental capabilities (2025-03-26)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub experimental: Option<HashMap<String, serde_json::Value>>,
}

/// Prompt-related server capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct PromptsCapability {
    /// Whether the server supports prompt list change notifications
    #[serde(rename = "listChanged", skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

/// Resource-related server capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ResourcesCapability {
    /// Whether the server supports resource subscriptions
    #[serde(skip_serializing_if = "Option::is_none")]
    pub subscribe: Option<bool>,
    /// Whether the server supports resource list change notifications
    #[serde(rename = "listChanged", skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

/// Tool-related server capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ToolsCapability {
    /// Whether the server supports tool list change notifications
    #[serde(rename = "listChanged", skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

/// Sampling-related capabilities
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct SamplingCapability {
    /// Additional properties
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

/// Logging capabilities (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct LoggingCapability {
    /// Additional properties
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

/// Autocompletion capabilities (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct CompletionsCapability {
    /// Additional properties
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

/// Roots capability for clients (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct RootsCapability {
    /// Whether the client supports notifications for changes to the roots list
    #[serde(rename = "listChanged", skip_serializing_if = "Option::is_none")]
    pub list_changed: Option<bool>,
}

// ============================================================================
// Annotations (2025-03-26 NEW)
// ============================================================================

/// Tool behavior annotations (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct Annotations {
    /// Target audience for this tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub audience: Option<Vec<AnnotationAudience>>,
    /// Danger level of this tool
    #[serde(skip_serializing_if = "Option::is_none")]
    pub danger: Option<DangerLevel>,
    /// Whether this tool performs destructive operations
    #[serde(skip_serializing_if = "Option::is_none")]
    pub destructive: Option<bool>,
    /// Whether this tool is read-only (no side effects)
    #[serde(rename = "readOnly", skip_serializing_if = "Option::is_none")]
    pub read_only: Option<bool>,
}

/// Target audience for tool usage
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum AnnotationAudience {
    /// For general users
    User,
    /// For developers and technical users
    Developer,
    /// For administrative operations
    Admin,
}

/// Danger level classification for tools
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum DangerLevel {
    /// Safe operations with no risk
    Safe,
    /// Low risk operations
    Low,
    /// Medium risk operations
    Medium,
    /// High risk operations
    High,
    /// Critical operations
    Critical,
}

// ============================================================================
// Content Types (2025-03-26 with Audio Support)
// ============================================================================

/// Text content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TextContent {
    /// Content type identifier
    #[serde(rename = "type")]
    pub content_type: String, // "text"
    /// The text content
    pub text: String,
    /// Content annotations (2025-03-26)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
}

/// Image content
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ImageContent {
    /// Content type identifier
    #[serde(rename = "type")]
    pub content_type: String, // "image"
    /// Base64-encoded image data
    pub data: String,
    /// MIME type of the image
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    /// Content annotations (2025-03-26)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
}

/// Audio content (2025-03-26 NEW)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct AudioContent {
    /// Content type identifier
    #[serde(rename = "type")]
    pub content_type: String, // "audio"
    /// Base64-encoded audio data
    pub data: String,
    /// MIME type of the audio
    #[serde(rename = "mimeType")]
    pub mime_type: String,
    /// Content annotations (2025-03-26)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
}

/// Embedded resource content (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct EmbeddedResource {
    /// Content type identifier
    #[serde(rename = "type")]
    pub content_type: String, // "resource"
    /// Resource reference
    pub resource: ResourceReference,
    /// Content annotations (2025-03-26)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
}

/// Resource reference for embedded resources
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceReference {
    /// URI of the resource
    pub uri: String,
}

/// Unified content type (2025-03-26 complete)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "type")]
pub enum Content {
    /// Text content
    #[serde(rename = "text")]
    Text {
        /// The text content
        text: String,
        /// Content annotations (2025-03-26)
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Annotations>,
    },
    /// Image content
    #[serde(rename = "image")]
    Image {
        /// Base64-encoded image data
        data: String,
        /// MIME type of the image
        #[serde(rename = "mimeType")]
        mime_type: String,
        /// Content annotations (2025-03-26)
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Annotations>,
    },
    /// Audio content (2025-03-26 NEW)
    #[serde(rename = "audio")]
    Audio {
        /// Base64-encoded audio data
        data: String,
        /// MIME type of the audio
        #[serde(rename = "mimeType")]
        mime_type: String,
        /// Content annotations (2025-03-26)
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Annotations>,
    },
    /// Embedded resource content (2025-03-26 NEW)
    #[serde(rename = "resource")]
    Resource {
        /// Resource reference
        resource: ResourceReference,
        /// Content annotations (2025-03-26)
        #[serde(skip_serializing_if = "Option::is_none")]
        annotations: Option<Annotations>,
    },
}

// ============================================================================
// Tool Types (2025-03-26 with Annotations)
// ============================================================================

/// Tool definition with annotations (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Tool {
    /// Name of the tool
    pub name: String,
    /// Description of what the tool does
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// JSON Schema describing the tool's input parameters
    #[serde(rename = "inputSchema")]
    pub input_schema: ToolInputSchema,
    /// Tool behavior annotations (2025-03-26 NEW)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
}

/// Tool input schema
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ToolInputSchema {
    /// Schema type (always "object")
    #[serde(rename = "type")]
    pub schema_type: String,
    /// Schema properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub properties: Option<HashMap<String, serde_json::Value>>,
    /// Required properties
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<Vec<String>>,
    /// Additional schema properties
    #[serde(flatten)]
    pub additional_properties: HashMap<String, serde_json::Value>,
}

/// Result of a tool execution (2025-03-26 with metadata)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CallToolResult {
    /// Content returned by the tool
    pub content: Vec<Content>,
    /// Whether this result represents an error
    #[serde(rename = "isError", skip_serializing_if = "Option::is_none")]
    pub is_error: Option<bool>,
    /// Result metadata (2025-03-26 NEW)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

// Re-export types with legacy names for compatibility
pub type ToolInfo = Tool;
pub type ToolResult = CallToolResult;

// ============================================================================
// Resource Types (2025-03-26)
// ============================================================================

/// Resource definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Resource {
    /// URI of the resource
    pub uri: String,
    /// Human-readable name of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description of the resource
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// MIME type of the resource
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
    /// Resource annotations (2025-03-26)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Annotations>,
    /// Resource size in bytes (2025-03-26)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}

/// Resource template for URI patterns
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ResourceTemplate {
    /// URI template with variables
    #[serde(rename = "uriTemplate")]
    pub uri_template: String,
    /// Human-readable name
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// Description of the resource template
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// MIME type of resources from this template
    #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
    pub mime_type: Option<String>,
}

/// Content of a resource
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ResourceContents {
    /// Text resource content
    Text {
        /// URI of the resource
        uri: String,
        /// MIME type
        #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
        mime_type: Option<String>,
        /// Text content
        text: String,
    },
    /// Binary resource content
    Blob {
        /// URI of the resource
        uri: String,
        /// MIME type
        #[serde(rename = "mimeType", skip_serializing_if = "Option::is_none")]
        mime_type: Option<String>,
        /// Base64-encoded binary data
        blob: String,
    },
}

// Legacy type aliases for compatibility
pub type ResourceInfo = Resource;

// ============================================================================
// Prompt Types (2025-03-26)
// ============================================================================

/// Prompt definition
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Prompt {
    /// Name of the prompt
    pub name: String,
    /// Description of what the prompt does
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Arguments that the prompt accepts
    #[serde(skip_serializing_if = "Option::is_none")]
    pub arguments: Option<Vec<PromptArgument>>,
}

/// Argument for a prompt
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptArgument {
    /// Name of the argument
    pub name: String,
    /// Description of the argument
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Whether this argument is required
    #[serde(skip_serializing_if = "Option::is_none")]
    pub required: Option<bool>,
}

/// Message role
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Role {
    User,
    Assistant,
}

/// Message in a prompt result (2025-03-26 with audio support)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PromptMessage {
    /// Role of the message
    pub role: Role,
    /// Content of the message (supports all content types)
    pub content: Content,
}

/// Result of prompt execution (2025-03-26 with metadata)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GetPromptResult {
    /// Description of the prompt result
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    /// Messages generated by the prompt
    pub messages: Vec<PromptMessage>,
    /// Result metadata (2025-03-26 NEW)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

// Legacy type aliases for compatibility
pub type PromptInfo = Prompt;
pub type PromptResult = GetPromptResult;

// ============================================================================
// Sampling Types (2025-03-26)
// ============================================================================

/// A message in a sampling conversation (2025-03-26 with audio)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct SamplingMessage {
    /// Role of the message
    pub role: Role,
    /// Content of the message (text, image, or audio)
    pub content: Content,
}

/// Model preferences for sampling
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Default)]
pub struct ModelPreferences {
    /// Hints about cost constraints
    #[serde(rename = "costPriority", skip_serializing_if = "Option::is_none")]
    pub cost_priority: Option<f32>,
    /// Hints about speed constraints
    #[serde(rename = "speedPriority", skip_serializing_if = "Option::is_none")]
    pub speed_priority: Option<f32>,
    /// Hints about quality constraints
    #[serde(rename = "qualityPriority", skip_serializing_if = "Option::is_none")]
    pub quality_priority: Option<f32>,
}

/// Result of sampling/createMessage (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct CreateMessageResult {
    /// Role of the generated message
    pub role: Role,
    /// Content of the generated message
    pub content: Content,
    /// Model used for generation
    pub model: String,
    /// Stop reason
    #[serde(rename = "stopReason", skip_serializing_if = "Option::is_none")]
    pub stop_reason: Option<StopReason>,
    /// Result metadata (2025-03-26 NEW)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
}

/// Reasons why sampling stopped
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub enum StopReason {
    EndTurn,
    StopSequence,
    MaxTokens,
    #[serde(untagged)]
    Other(String),
}

// ============================================================================
// Logging Types (2025-03-26)
// ============================================================================

/// Logging level enumeration (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum LoggingLevel {
    Debug,
    Info,
    Notice,
    Warning,
    Error,
    Critical,
    Alert,
    Emergency,
}

// ============================================================================
// JSON-RPC Types (2025-03-26 with Batching)
// ============================================================================

/// JSON-RPC request message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcRequest {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Request ID for correlation
    pub id: RequestId,
    /// Method name being called
    pub method: String,
    /// Method parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

/// JSON-RPC response message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcResponse {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Request ID for correlation
    pub id: RequestId,
    /// Result of the method call
    #[serde(skip_serializing_if = "Option::is_none")]
    pub result: Option<serde_json::Value>,
}

/// JSON-RPC error message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcError {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Request ID for correlation
    pub id: RequestId,
    /// Error information
    pub error: ErrorObject,
}

/// Error object
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct ErrorObject {
    /// Error code
    pub code: i32,
    /// Error message
    pub message: String,
    /// Additional error data
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// JSON-RPC notification message
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct JsonRpcNotification {
    /// JSON-RPC version (always "2.0")
    pub jsonrpc: String,
    /// Method name being called
    pub method: String,
    /// Method parameters
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<serde_json::Value>,
}

/// JSON-RPC batch request (2025-03-26 NEW)
pub type JsonRpcBatchRequest = Vec<JsonRpcRequestOrNotification>;

/// JSON-RPC batch response (2025-03-26 NEW)
pub type JsonRpcBatchResponse = Vec<JsonRpcResponseOrError>;

/// Items in a batch request
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum JsonRpcRequestOrNotification {
    Request(JsonRpcRequest),
    Notification(JsonRpcNotification),
}

/// Items in a batch response
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum JsonRpcResponseOrError {
    Response(JsonRpcResponse),
    Error(JsonRpcError),
}

/// Complete JSON-RPC message types (2025-03-26)
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum JsonRpcMessage {
    Request(JsonRpcRequest),
    Response(JsonRpcResponse),
    Error(JsonRpcError),
    Notification(JsonRpcNotification),
    BatchRequest(JsonRpcBatchRequest),
    BatchResponse(JsonRpcBatchResponse),
}

// ============================================================================
// Request/Response Metadata (2025-03-26 NEW)
// ============================================================================

/// Base request with metadata support
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Request {
    /// Method name
    pub method: String,
    /// Parameters with metadata support
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<RequestParams>,
}

/// Request parameters with metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestParams {
    /// Request metadata (2025-03-26 NEW)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<RequestMeta>,
    /// Additional parameters
    #[serde(flatten)]
    pub params: HashMap<String, serde_json::Value>,
}

/// Request metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RequestMeta {
    /// Progress token for out-of-band progress notifications
    #[serde(rename = "progressToken", skip_serializing_if = "Option::is_none")]
    pub progress_token: Option<ProgressToken>,
}

/// Base notification with metadata support
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Notification {
    /// Method name
    pub method: String,
    /// Parameters with metadata support
    #[serde(skip_serializing_if = "Option::is_none")]
    pub params: Option<NotificationParams>,
}

/// Notification parameters with metadata
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct NotificationParams {
    /// Notification metadata (2025-03-26 NEW)
    #[serde(rename = "_meta", skip_serializing_if = "Option::is_none")]
    pub meta: Option<HashMap<String, serde_json::Value>>,
    /// Additional parameters
    #[serde(flatten)]
    pub params: HashMap<String, serde_json::Value>,
}

// ============================================================================
// Pagination Support
// ============================================================================

/// Base for paginated requests
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaginatedRequest {
    /// Cursor for pagination
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cursor: Option<Cursor>,
}

/// Base for paginated results
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct PaginatedResult {
    /// Cursor for next page
    #[serde(rename = "nextCursor", skip_serializing_if = "Option::is_none")]
    pub next_cursor: Option<Cursor>,
}

// ============================================================================
// Helper Constructors
// ============================================================================

impl Content {
    /// Create text content
    pub fn text<S: Into<String>>(text: S) -> Self {
        Self::Text {
            text: text.into(),
            annotations: None,
        }
    }

    /// Create image content
    pub fn image<S: Into<String>>(data: S, mime_type: S) -> Self {
        Self::Image {
            data: data.into(),
            mime_type: mime_type.into(),
            annotations: None,
        }
    }

    /// Create audio content (2025-03-26 NEW)
    pub fn audio<S: Into<String>>(data: S, mime_type: S) -> Self {
        Self::Audio {
            data: data.into(),
            mime_type: mime_type.into(),
            annotations: None,
        }
    }

    /// Create embedded resource content (2025-03-26 NEW)
    pub fn resource<S: Into<String>>(uri: S) -> Self {
        Self::Resource {
            resource: ResourceReference { uri: uri.into() },
            annotations: None,
        }
    }
}

impl Annotations {
    /// Create new annotations
    pub fn new() -> Self {
        Self {
            audience: None,
            danger: None,
            destructive: None,
            read_only: None,
        }
    }

    /// Mark as read-only tool
    pub fn read_only(mut self) -> Self {
        self.read_only = Some(true);
        self.destructive = Some(false);
        self
    }

    /// Mark as destructive tool
    pub fn destructive(mut self, danger: DangerLevel) -> Self {
        self.destructive = Some(true);
        self.read_only = Some(false);
        self.danger = Some(danger);
        self
    }

    /// Set audience
    pub fn for_audience(mut self, audience: Vec<AnnotationAudience>) -> Self {
        self.audience = Some(audience);
        self
    }

    /// Set danger level
    pub fn with_danger_level(mut self, danger: DangerLevel) -> Self {
        self.danger = Some(danger);
        self
    }
}

impl Tool {
    /// Create a new tool
    pub fn new<S: Into<String>>(name: S, description: S) -> Self {
        Self {
            name: name.into(),
            description: Some(description.into()),
            input_schema: ToolInputSchema {
                schema_type: "object".to_string(),
                properties: None,
                required: None,
                additional_properties: HashMap::new(),
            },
            annotations: None,
        }
    }

    /// Add annotations to the tool
    pub fn with_annotations(mut self, annotations: Annotations) -> Self {
        self.annotations = Some(annotations);
        self
    }
}

impl JsonRpcRequest {
    /// Create a new JSON-RPC request
    pub fn new<T: Serialize>(
        id: RequestId,
        method: String,
        params: Option<T>,
    ) -> std::result::Result<Self, serde_json::Error> {
        let params = match params {
            Some(p) => Some(serde_json::to_value(p)?),
            None => None,
        };

        Ok(Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id,
            method,
            params,
        })
    }
}

impl JsonRpcResponse {
    /// Create a successful JSON-RPC response
    pub fn success<T: Serialize>(
        id: RequestId,
        result: T,
    ) -> std::result::Result<Self, serde_json::Error> {
        Ok(Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id,
            result: Some(serde_json::to_value(result)?),
        })
    }
}

impl JsonRpcError {
    /// Create an error JSON-RPC response
    pub fn error(
        id: RequestId,
        code: i32,
        message: String,
        data: Option<serde_json::Value>,
    ) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            id,
            error: ErrorObject {
                code,
                message,
                data,
            },
        }
    }
}

impl JsonRpcNotification {
    /// Create a new JSON-RPC notification
    pub fn new<T: Serialize>(
        method: String,
        params: Option<T>,
    ) -> std::result::Result<Self, serde_json::Error> {
        let params = match params {
            Some(p) => Some(serde_json::to_value(p)?),
            None => None,
        };

        Ok(Self {
            jsonrpc: JSONRPC_VERSION.to_string(),
            method,
            params,
        })
    }
}

// ============================================================================
// Error Codes
// ============================================================================

/// Standard JSON-RPC error codes
pub mod error_codes {
    /// Invalid JSON was received
    pub const PARSE_ERROR: i32 = -32700;
    /// The JSON sent is not a valid Request object
    pub const INVALID_REQUEST: i32 = -32600;
    /// The method does not exist / is not available
    pub const METHOD_NOT_FOUND: i32 = -32601;
    /// Invalid method parameter(s)
    pub const INVALID_PARAMS: i32 = -32602;
    /// Internal JSON-RPC error
    pub const INTERNAL_ERROR: i32 = -32603;

    /// MCP-specific error codes
    pub const TOOL_NOT_FOUND: i32 = -32000;
    pub const RESOURCE_NOT_FOUND: i32 = -32001;
    pub const PROMPT_NOT_FOUND: i32 = -32002;
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn test_protocol_version() {
        assert_eq!(LATEST_PROTOCOL_VERSION, "2025-03-26");
        assert_eq!(JSONRPC_VERSION, "2.0");
    }

    #[test]
    fn test_content_types() {
        // Test text content
        let text = Content::text("Hello, world!");
        let json = serde_json::to_value(&text).unwrap();
        assert_eq!(json["type"], "text");
        assert_eq!(json["text"], "Hello, world!");

        // Test audio content (new in 2025-03-26)
        let audio = Content::audio("base64data", "audio/wav");
        let json = serde_json::to_value(&audio).unwrap();
        assert_eq!(json["type"], "audio");
        assert_eq!(json["data"], "base64data");
        assert_eq!(json["mimeType"], "audio/wav");

        // Test resource content (new in 2025-03-26)
        let resource = Content::resource("file:///test.txt");
        let json = serde_json::to_value(&resource).unwrap();
        assert_eq!(json["type"], "resource");
        assert_eq!(json["resource"]["uri"], "file:///test.txt");
    }

    #[test]
    fn test_annotations() {
        let annotations = Annotations::new()
            .read_only()
            .for_audience(vec![AnnotationAudience::Developer])
            .with_danger_level(DangerLevel::Safe);

        assert_eq!(annotations.read_only, Some(true));
        assert_eq!(annotations.destructive, Some(false));
        assert_eq!(annotations.danger, Some(DangerLevel::Safe));
        assert_eq!(
            annotations.audience,
            Some(vec![AnnotationAudience::Developer])
        );
    }

    #[test]
    fn test_tool_with_annotations() {
        let tool = Tool::new("safe_reader", "Read file safely")
            .with_annotations(Annotations::new().read_only());

        assert_eq!(tool.name, "safe_reader");
        assert!(tool.annotations.is_some());
        assert_eq!(tool.annotations.unwrap().read_only, Some(true));
    }

    #[test]
    fn test_jsonrpc_batching() {
        let req1 = JsonRpcRequest::new(json!(1), "method1".to_string(), Some(json!({}))).unwrap();
        let req2 = JsonRpcRequest::new::<serde_json::Value>(json!(2), "method2".to_string(), None)
            .unwrap();

        let batch: JsonRpcBatchRequest = vec![
            JsonRpcRequestOrNotification::Request(req1),
            JsonRpcRequestOrNotification::Request(req2),
        ];

        let json = serde_json::to_value(&batch).unwrap();
        assert!(json.is_array());
        assert_eq!(json.as_array().unwrap().len(), 2);
    }

    #[test]
    fn test_server_capabilities_2025() {
        let caps = ServerCapabilities {
            tools: Some(ToolsCapability {
                list_changed: Some(true),
            }),
            completions: Some(CompletionsCapability::default()),
            logging: Some(LoggingCapability::default()),
            experimental: Some(HashMap::new()),
            ..Default::default()
        };

        let json = serde_json::to_value(&caps).unwrap();
        assert!(json["tools"]["listChanged"].as_bool().unwrap());
        assert!(json["completions"].is_object());
        assert!(json["logging"].is_object());
        assert!(json["experimental"].is_object());
    }
}
