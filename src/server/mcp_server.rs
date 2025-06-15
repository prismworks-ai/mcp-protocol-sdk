//! MCP server implementation
//!
//! This module provides the main MCP server implementation that handles client connections,
//! manages resources, tools, and prompts, and processes JSON-RPC requests according to
//! the Model Context Protocol specification.

use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use crate::core::{
    error::{McpError, McpResult},
    prompt::{Prompt, PromptHandler},
    resource::{Resource, ResourceHandler},
    tool::{Tool, ToolHandler},
    PromptInfo, ResourceInfo, ToolInfo,
};
use crate::protocol::{messages::*, types::*, validation::*, error_codes::*};
use crate::transport::traits::ServerTransport;

/// Configuration for the MCP server
#[derive(Debug, Clone)]
pub struct ServerConfig {
    /// Maximum number of concurrent requests
    pub max_concurrent_requests: usize,
    /// Request timeout in milliseconds
    pub request_timeout_ms: u64,
    /// Whether to validate all incoming requests
    pub validate_requests: bool,
    /// Whether to enable detailed logging
    pub enable_logging: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            max_concurrent_requests: 100,
            request_timeout_ms: 30000,
            validate_requests: true,
            enable_logging: true,
        }
    }
}

/// Main MCP server implementation
pub struct McpServer {
    /// Server information
    info: ServerInfo,
    /// Server capabilities
    capabilities: ServerCapabilities,
    /// Server configuration
    config: ServerConfig,
    /// Registered resources
    resources: Arc<RwLock<HashMap<String, Resource>>>,
    /// Registered tools
    tools: Arc<RwLock<HashMap<String, Tool>>>,
    /// Registered prompts
    prompts: Arc<RwLock<HashMap<String, Prompt>>>,
    /// Active transport
    transport: Arc<Mutex<Option<Box<dyn ServerTransport>>>>,
    /// Server state
    state: Arc<RwLock<ServerState>>,
    /// Request ID counter
    #[allow(dead_code)]
    request_counter: Arc<Mutex<u64>>,
}

/// Internal server state
#[derive(Debug, Clone, PartialEq)]
pub enum ServerState {
    /// Server is not yet initialized
    Uninitialized,
    /// Server is initializing
    Initializing,
    /// Server is running and ready to accept requests
    Running,
    /// Server is shutting down
    Stopping,
    /// Server has stopped
    Stopped,
}

impl McpServer {
    /// Create a new MCP server with the given name and version
    pub fn new(name: String, version: String) -> Self {
        Self {
            info: ServerInfo { name, version },
            capabilities: ServerCapabilities {
                prompts: Some(PromptsCapability {
                    list_changed: Some(true),
                }),
                resources: Some(ResourcesCapability {
                    subscribe: Some(true),
                    list_changed: Some(true),
                }),
                tools: Some(ToolsCapability {
                    list_changed: Some(true),
                }),
                sampling: None,
                logging: None,
                experimental: None,
                completions: None,
            },
            config: ServerConfig::default(),
            resources: Arc::new(RwLock::new(HashMap::new())),
            tools: Arc::new(RwLock::new(HashMap::new())),
            prompts: Arc::new(RwLock::new(HashMap::new())),
            transport: Arc::new(Mutex::new(None)),
            state: Arc::new(RwLock::new(ServerState::Uninitialized)),
            request_counter: Arc::new(Mutex::new(0)),
        }
    }

    /// Create a new MCP server with custom configuration
    pub fn with_config(name: String, version: String, config: ServerConfig) -> Self {
        let mut server = Self::new(name, version);
        server.config = config;
        server
    }

    /// Set server capabilities
    pub fn set_capabilities(&mut self, capabilities: ServerCapabilities) {
        self.capabilities = capabilities;
    }

    /// Get server information
    pub fn info(&self) -> &ServerInfo {
        &self.info
    }

    /// Get server capabilities
    pub fn capabilities(&self) -> &ServerCapabilities {
        &self.capabilities
    }

    /// Get server configuration
    pub fn config(&self) -> &ServerConfig {
        &self.config
    }

    // ========================================================================
    // Resource Management
    // ========================================================================

    /// Add a resource to the server
    pub async fn add_resource<H>(&self, name: String, uri: String, handler: H) -> McpResult<()>
    where
        H: ResourceHandler + 'static,
    {
        let resource_info = ResourceInfo {
            uri: uri.clone(),
            name: Some(name.clone()),
            description: None,
            mime_type: None,
            annotations: None,
            size: None,
        };

        validate_resource_info(&resource_info)?;

        let resource = Resource::new(resource_info, handler);

        {
            let mut resources = self.resources.write().await;
            resources.insert(uri, resource);
        }

        // Emit list changed notification if we have an active transport
        self.emit_resources_list_changed().await?;

        Ok(())
    }

    /// Add a resource with detailed information
    pub async fn add_resource_detailed<H>(&self, info: ResourceInfo, handler: H) -> McpResult<()>
    where
        H: ResourceHandler + 'static,
    {
        validate_resource_info(&info)?;

        let uri = info.uri.clone();
        let resource = Resource::new(info, handler);

        {
            let mut resources = self.resources.write().await;
            resources.insert(uri, resource);
        }

        self.emit_resources_list_changed().await?;

        Ok(())
    }

    /// Remove a resource from the server
    pub async fn remove_resource(&self, uri: &str) -> McpResult<bool> {
        let removed = {
            let mut resources = self.resources.write().await;
            resources.remove(uri).is_some()
        };

        if removed {
            self.emit_resources_list_changed().await?;
        }

        Ok(removed)
    }

    /// List all registered resources
    pub async fn list_resources(&self) -> McpResult<Vec<ResourceInfo>> {
        let resources = self.resources.read().await;
        Ok(resources.values().map(|r| r.info.clone()).collect())
    }

    /// Read a resource
    pub async fn read_resource(&self, uri: &str) -> McpResult<Vec<ResourceContents>> {
        let resources = self.resources.read().await;

        match resources.get(uri) {
            Some(resource) => {
                let params = HashMap::new(); // URL parameter extraction will be implemented in future versions
                resource.handler.read(uri, &params).await
            }
            None => Err(McpError::ResourceNotFound(uri.to_string())),
        }
    }

    // ========================================================================
    // Tool Management
    // ========================================================================

    /// Add a tool to the server
    pub async fn add_tool<H>(
        &self,
        name: String,
        description: Option<String>,
        schema: Value,
        handler: H,
    ) -> McpResult<()>
    where
        H: ToolHandler + 'static,
    {
        let tool_schema = ToolInputSchema {
            schema_type: "object".to_string(),
            properties: schema.get("properties").and_then(|p| p.as_object()).map(|obj| {
                obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
            }),
            required: schema.get("required").and_then(|r| {
                r.as_array().map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str().map(|s| s.to_string()))
                        .collect()
                })
            }),
            additional_properties: schema.as_object().unwrap_or(&serde_json::Map::new()).iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
        };

        let tool_info = ToolInfo {
            name: name.clone(),
            description,
            input_schema: tool_schema,
            annotations: None,
        };

        validate_tool_info(&tool_info)?;

        let tool = Tool::new(
            name.clone(),
            tool_info.description.clone(),
            serde_json::to_value(&tool_info.input_schema)?,
            handler,
        );

        {
            let mut tools = self.tools.write().await;
            tools.insert(name, tool);
        }

        self.emit_tools_list_changed().await?;

        Ok(())
    }

    /// Add a tool with detailed information
    pub async fn add_tool_detailed<H>(&self, info: ToolInfo, handler: H) -> McpResult<()>
    where
        H: ToolHandler + 'static,
    {
        validate_tool_info(&info)?;

        let name = info.name.clone();
        let tool = Tool::new(
            name.clone(),
            info.description.clone(),
            serde_json::to_value(&info.input_schema)?,
            handler,
        );

        {
            let mut tools = self.tools.write().await;
            tools.insert(name, tool);
        }

        self.emit_tools_list_changed().await?;

        Ok(())
    }

    /// Remove a tool from the server
    pub async fn remove_tool(&self, name: &str) -> McpResult<bool> {
        let removed = {
            let mut tools = self.tools.write().await;
            tools.remove(name).is_some()
        };

        if removed {
            self.emit_tools_list_changed().await?;
        }

        Ok(removed)
    }

    /// List all registered tools
    pub async fn list_tools(&self) -> McpResult<Vec<ToolInfo>> {
        let tools = self.tools.read().await;
        Ok(tools.values().map(|t| t.info.clone()).collect())
    }

    /// Call a tool
    pub async fn call_tool(
        &self,
        name: &str,
        arguments: Option<HashMap<String, Value>>,
    ) -> McpResult<ToolResult> {
        let tools = self.tools.read().await;

        match tools.get(name) {
            Some(tool) => {
                if !tool.enabled {
                    return Err(McpError::ToolNotFound(format!(
                        "Tool '{}' is disabled",
                        name
                    )));
                }

                let args = arguments.unwrap_or_default();
                tool.handler.call(args).await
            }
            None => Err(McpError::ToolNotFound(name.to_string())),
        }
    }

    // ========================================================================
    // Prompt Management
    // ========================================================================

    /// Add a prompt to the server
    pub async fn add_prompt<H>(&self, info: PromptInfo, handler: H) -> McpResult<()>
    where
        H: PromptHandler + 'static,
    {
        validate_prompt_info(&info)?;

        let name = info.name.clone();
        let prompt = Prompt::new(info, handler);

        {
            let mut prompts = self.prompts.write().await;
            prompts.insert(name, prompt);
        }

        self.emit_prompts_list_changed().await?;

        Ok(())
    }

    /// Remove a prompt from the server
    pub async fn remove_prompt(&self, name: &str) -> McpResult<bool> {
        let removed = {
            let mut prompts = self.prompts.write().await;
            prompts.remove(name).is_some()
        };

        if removed {
            self.emit_prompts_list_changed().await?;
        }

        Ok(removed)
    }

    /// List all registered prompts
    pub async fn list_prompts(&self) -> McpResult<Vec<PromptInfo>> {
        let prompts = self.prompts.read().await;
        Ok(prompts.values().map(|p| p.info.clone()).collect())
    }

    /// Get a prompt
    pub async fn get_prompt(
        &self,
        name: &str,
        arguments: Option<HashMap<String, Value>>,
    ) -> McpResult<PromptResult> {
        let prompts = self.prompts.read().await;

        match prompts.get(name) {
            Some(prompt) => {
                let args = arguments.unwrap_or_default();
                prompt.handler.get(args).await
            }
            None => Err(McpError::PromptNotFound(name.to_string())),
        }
    }

    // ========================================================================
    // Server Lifecycle
    // ========================================================================

    /// Start the server with the given transport
    pub async fn start<T>(&mut self, transport: T) -> McpResult<()>
    where
        T: ServerTransport + 'static,
    {
        let mut state = self.state.write().await;

        match *state {
            ServerState::Uninitialized => {
                *state = ServerState::Initializing;
            }
            _ => return Err(McpError::Protocol("Server is already started".to_string())),
        }

        drop(state);

        // Set up the transport
        {
            let mut transport_guard = self.transport.lock().await;
            *transport_guard = Some(Box::new(transport));
        }

        // Start the transport
        {
            let mut transport_guard = self.transport.lock().await;
            if let Some(transport) = transport_guard.as_mut() {
                transport.start().await?;
            }
        }

        // Update state to running
        {
            let mut state = self.state.write().await;
            *state = ServerState::Running;
        }

        Ok(())
    }

    /// Stop the server
    pub async fn stop(&self) -> McpResult<()> {
        let mut state = self.state.write().await;

        match *state {
            ServerState::Running => {
                *state = ServerState::Stopping;
            }
            ServerState::Stopped => return Ok(()),
            _ => return Err(McpError::Protocol("Server is not running".to_string())),
        }

        drop(state);

        // Stop the transport
        {
            let mut transport_guard = self.transport.lock().await;
            if let Some(transport) = transport_guard.as_mut() {
                transport.stop().await?;
            }
        }

        // Update state to stopped
        {
            let mut state = self.state.write().await;
            *state = ServerState::Stopped;
        }

        Ok(())
    }

    /// Check if the server is running
    pub async fn is_running(&self) -> bool {
        let state = self.state.read().await;
        matches!(*state, ServerState::Running)
    }

    /// Get the current server state
    pub async fn state(&self) -> ServerState {
        let state = self.state.read().await;
        state.clone()
    }

    // ========================================================================
    // Request Handling
    // ========================================================================

    /// Handle an incoming JSON-RPC request
    pub async fn handle_request(&self, request: JsonRpcRequest) -> McpResult<JsonRpcResponse> {
        // Validate the request if configured to do so
        if self.config.validate_requests {
            validate_jsonrpc_request(&request)?;
            validate_mcp_request(&request.method, request.params.as_ref())?;
        }

        // Route the request to the appropriate handler
        let result = match request.method.as_str() {
            methods::INITIALIZE => self.handle_initialize(request.params).await,
            methods::PING => self.handle_ping().await,
            methods::TOOLS_LIST => self.handle_tools_list(request.params).await,
            methods::TOOLS_CALL => self.handle_tools_call(request.params).await,
            methods::RESOURCES_LIST => self.handle_resources_list(request.params).await,
            methods::RESOURCES_READ => self.handle_resources_read(request.params).await,
            methods::RESOURCES_SUBSCRIBE => self.handle_resources_subscribe(request.params).await,
            methods::RESOURCES_UNSUBSCRIBE => {
                self.handle_resources_unsubscribe(request.params).await
            }
            methods::PROMPTS_LIST => self.handle_prompts_list(request.params).await,
            methods::PROMPTS_GET => self.handle_prompts_get(request.params).await,
            methods::LOGGING_SET_LEVEL => self.handle_logging_set_level(request.params).await,
            _ => Err(McpError::Protocol(format!(
                "Unknown method: {}",
                request.method
            ))),
        };

        // Convert the result to a JSON-RPC response
        match result {
            Ok(result_value) => Ok(JsonRpcResponse::success(request.id, result_value)?),
            Err(error) => {
                let (code, message) = match error {
                    McpError::ToolNotFound(_) => (TOOL_NOT_FOUND, error.to_string()),
                    McpError::ResourceNotFound(_) => (RESOURCE_NOT_FOUND, error.to_string()),
                    McpError::PromptNotFound(_) => (PROMPT_NOT_FOUND, error.to_string()),
                    McpError::Validation(_) => (INVALID_PARAMS, error.to_string()),
                    _ => (INTERNAL_ERROR, error.to_string()),
                };
                // For now, return errors as part of the result
                // TODO: Implement proper JSON-RPC error handling for 2025-03-26
                Ok(JsonRpcResponse::success(request.id, serde_json::json!({
                    "error": {
                        "code": code,
                        "message": message,
                    }
                }))?)
            }
        }
    }

    // ========================================================================
    // Individual Request Handlers
    // ========================================================================

    async fn handle_initialize(&self, params: Option<Value>) -> McpResult<Value> {
        let params: InitializeParams = match params {
            Some(p) => serde_json::from_value(p)?,
            None => {
                return Err(McpError::Validation(
                    "Missing initialize parameters".to_string(),
                ))
            }
        };

        validate_initialize_params(&params)?;

        let result = InitializeResult::new(
            self.info.clone(),
            self.capabilities.clone(),
            None, // instructions
        );

        Ok(serde_json::to_value(result)?)
    }

    async fn handle_ping(&self) -> McpResult<Value> {
        Ok(serde_json::to_value(PingResult { meta: None })?)
    }

    async fn handle_tools_list(&self, params: Option<Value>) -> McpResult<Value> {
        let _params: ListToolsParams = match params {
            Some(p) => serde_json::from_value(p)?,
            None => ListToolsParams::default(),
        };

        let tools = self.list_tools().await?;
        let result = ListToolsResult {
            tools,
            next_cursor: None, // Pagination support will be added in future versions
            meta: None,
        };

        Ok(serde_json::to_value(result)?)
    }

    async fn handle_tools_call(&self, params: Option<Value>) -> McpResult<Value> {
        let params: CallToolParams = match params {
            Some(p) => serde_json::from_value(p)?,
            None => {
                return Err(McpError::Validation(
                    "Missing tool call parameters".to_string(),
                ))
            }
        };

        validate_call_tool_params(&params)?;

        let result = self.call_tool(&params.name, params.arguments).await?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_resources_list(&self, params: Option<Value>) -> McpResult<Value> {
        let _params: ListResourcesParams = match params {
            Some(p) => serde_json::from_value(p)?,
            None => ListResourcesParams::default(),
        };

        let resources = self.list_resources().await?;
        let result = ListResourcesResult {
            resources,
            next_cursor: None, // Pagination support will be added in future versions
            meta: None,
        };

        Ok(serde_json::to_value(result)?)
    }

    async fn handle_resources_read(&self, params: Option<Value>) -> McpResult<Value> {
        let params: ReadResourceParams = match params {
            Some(p) => serde_json::from_value(p)?,
            None => {
                return Err(McpError::Validation(
                    "Missing resource read parameters".to_string(),
                ))
            }
        };

        validate_read_resource_params(&params)?;

        let contents = self.read_resource(&params.uri).await?;
        let result = ReadResourceResult { contents, meta: None };

        Ok(serde_json::to_value(result)?)
    }

    async fn handle_resources_subscribe(&self, params: Option<Value>) -> McpResult<Value> {
        let params: SubscribeResourceParams = match params {
            Some(p) => serde_json::from_value(p)?,
            None => {
                return Err(McpError::Validation(
                    "Missing resource subscribe parameters".to_string(),
                ))
            }
        };

        // Resource subscriptions functionality planned for future implementation
        let _uri = params.uri;
        let result = SubscribeResourceResult { meta: None };

        Ok(serde_json::to_value(result)?)
    }

    async fn handle_resources_unsubscribe(&self, params: Option<Value>) -> McpResult<Value> {
        let params: UnsubscribeResourceParams = match params {
            Some(p) => serde_json::from_value(p)?,
            None => {
                return Err(McpError::Validation(
                    "Missing resource unsubscribe parameters".to_string(),
                ))
            }
        };

        // Resource subscriptions functionality planned for future implementation
        let _uri = params.uri;
        let result = UnsubscribeResourceResult { meta: None };

        Ok(serde_json::to_value(result)?)
    }

    async fn handle_prompts_list(&self, params: Option<Value>) -> McpResult<Value> {
        let _params: ListPromptsParams = match params {
            Some(p) => serde_json::from_value(p)?,
            None => ListPromptsParams::default(),
        };

        let prompts = self.list_prompts().await?;
        let result = ListPromptsResult {
            prompts,
            next_cursor: None, // Pagination support will be added in future versions
            meta: None,
        };

        Ok(serde_json::to_value(result)?)
    }

    async fn handle_prompts_get(&self, params: Option<Value>) -> McpResult<Value> {
        let params: GetPromptParams = match params {
            Some(p) => serde_json::from_value(p)?,
            None => {
                return Err(McpError::Validation(
                    "Missing prompt get parameters".to_string(),
                ))
            }
        };

        validate_get_prompt_params(&params)?;

        let result = self.get_prompt(&params.name, params.arguments).await?;
        Ok(serde_json::to_value(result)?)
    }

    async fn handle_logging_set_level(&self, params: Option<Value>) -> McpResult<Value> {
        let _params: SetLoggingLevelParams = match params {
            Some(p) => serde_json::from_value(p)?,
            None => {
                return Err(McpError::Validation(
                    "Missing logging level parameters".to_string(),
                ))
            }
        };

        // Logging level management feature planned for future implementation
        let result = SetLoggingLevelResult { meta: None };
        Ok(serde_json::to_value(result)?)
    }

    // ========================================================================
    // Notification Helpers
    // ========================================================================

    async fn emit_resources_list_changed(&self) -> McpResult<()> {
        let notification = JsonRpcNotification::new(
            methods::RESOURCES_LIST_CHANGED.to_string(),
            Some(ResourceListChangedParams { meta: None }),
        )?;

        self.send_notification(notification).await
    }

    async fn emit_tools_list_changed(&self) -> McpResult<()> {
        let notification = JsonRpcNotification::new(
            methods::TOOLS_LIST_CHANGED.to_string(),
            Some(ToolListChangedParams { meta: None }),
        )?;

        self.send_notification(notification).await
    }

    async fn emit_prompts_list_changed(&self) -> McpResult<()> {
        let notification = JsonRpcNotification::new(
            methods::PROMPTS_LIST_CHANGED.to_string(),
            Some(PromptListChangedParams { meta: None }),
        )?;

        self.send_notification(notification).await
    }

    /// Send a notification through the transport
    async fn send_notification(&self, notification: JsonRpcNotification) -> McpResult<()> {
        let mut transport_guard = self.transport.lock().await;
        if let Some(transport) = transport_guard.as_mut() {
            transport.send_notification(notification).await?;
        }
        Ok(())
    }

    // ========================================================================
    // Utility Methods
    // ========================================================================

    #[allow(dead_code)]
    async fn next_request_id(&self) -> u64 {
        let mut counter = self.request_counter.lock().await;
        *counter += 1;
        *counter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[tokio::test]
    async fn test_server_creation() {
        let server = McpServer::new("test-server".to_string(), "1.0.0".to_string());
        assert_eq!(server.info().name, "test-server");
        assert_eq!(server.info().version, "1.0.0");
        assert!(!server.is_running().await);
    }

    #[tokio::test]
    async fn test_tool_management() {
        let server = McpServer::new("test-server".to_string(), "1.0.0".to_string());

        // Add a tool
        let schema = json!({
            "type": "object",
            "properties": {
                "name": {"type": "string"}
            }
        });

        struct TestToolHandler;

        #[async_trait::async_trait]
        impl ToolHandler for TestToolHandler {
            async fn call(&self, _arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
                Ok(ToolResult {
                    content: vec![Content::text("Hello from tool")],
                    is_error: None,
                    meta: None,
                })
            }
        }

        server
            .add_tool(
                "test_tool".to_string(),
                Some("A test tool".to_string()),
                schema,
                TestToolHandler,
            )
            .await
            .unwrap();

        // List tools
        let tools = server.list_tools().await.unwrap();
        assert_eq!(tools.len(), 1);
        assert_eq!(tools[0].name, "test_tool");

        // Call tool
        let result = server.call_tool("test_tool", None).await.unwrap();
        assert_eq!(result.content.len(), 1);
    }

    #[tokio::test]
    async fn test_initialize_request() {
        let server = McpServer::new("test-server".to_string(), "1.0.0".to_string());

        let init_params = InitializeParams::new(
            ClientInfo {
                name: "test-client".to_string(),
                version: "1.0.0".to_string(),
            },
            ClientCapabilities::default(),
        );

        let request =
            JsonRpcRequest::new(json!(1), methods::INITIALIZE.to_string(), Some(init_params))
                .unwrap();

        let response = server.handle_request(request).await.unwrap();
        assert!(response.result.is_some());
    }
}
