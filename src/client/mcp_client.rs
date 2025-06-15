//! MCP client implementation
//!
//! This module provides the main MCP client that can connect to MCP servers,
//! initialize connections, and perform operations like calling tools, reading resources,
//! and executing prompts according to the Model Context Protocol specification.

use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::{Mutex, RwLock};

use crate::core::error::{McpError, McpResult};
use crate::protocol::{messages::*, types::*, validation::*};
use crate::transport::traits::Transport;

/// Configuration for the MCP client
#[derive(Debug, Clone)]
pub struct ClientConfig {
    /// Request timeout in milliseconds
    pub request_timeout_ms: u64,
    /// Maximum number of retry attempts
    pub max_retries: u32,
    /// Retry delay in milliseconds
    pub retry_delay_ms: u64,
    /// Whether to validate all outgoing requests
    pub validate_requests: bool,
    /// Whether to validate all incoming responses
    pub validate_responses: bool,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            request_timeout_ms: 30000,
            max_retries: 3,
            retry_delay_ms: 1000,
            validate_requests: true,
            validate_responses: true,
        }
    }
}

/// Main MCP client implementation
pub struct McpClient {
    /// Client information
    info: ClientInfo,
    /// Client capabilities
    capabilities: ClientCapabilities,
    /// Client configuration
    config: ClientConfig,
    /// Active transport
    transport: Arc<Mutex<Option<Box<dyn Transport>>>>,
    /// Server capabilities (available after initialization)
    server_capabilities: Arc<RwLock<Option<ServerCapabilities>>>,
    /// Server information (available after initialization)
    server_info: Arc<RwLock<Option<ServerInfo>>>,
    /// Request ID counter
    request_counter: Arc<Mutex<u64>>,
    /// Connection state
    connected: Arc<RwLock<bool>>,
}

impl McpClient {
    /// Create a new MCP client with the given name and version
    pub fn new(name: String, version: String) -> Self {
        Self {
            info: ClientInfo { name, version },
            capabilities: ClientCapabilities::default(),
            config: ClientConfig::default(),
            transport: Arc::new(Mutex::new(None)),
            server_capabilities: Arc::new(RwLock::new(None)),
            server_info: Arc::new(RwLock::new(None)),
            request_counter: Arc::new(Mutex::new(0)),
            connected: Arc::new(RwLock::new(false)),
        }
    }

    /// Create a new MCP client with custom configuration
    pub fn with_config(name: String, version: String, config: ClientConfig) -> Self {
        let mut client = Self::new(name, version);
        client.config = config;
        client
    }

    /// Set client capabilities
    pub fn set_capabilities(&mut self, capabilities: ClientCapabilities) {
        self.capabilities = capabilities;
    }

    /// Get client information
    pub fn info(&self) -> &ClientInfo {
        &self.info
    }

    /// Get client capabilities
    pub fn capabilities(&self) -> &ClientCapabilities {
        &self.capabilities
    }

    /// Get client configuration
    pub fn config(&self) -> &ClientConfig {
        &self.config
    }

    /// Get server capabilities (if connected)
    pub async fn server_capabilities(&self) -> Option<ServerCapabilities> {
        let capabilities = self.server_capabilities.read().await;
        capabilities.clone()
    }

    /// Get server information (if connected)
    pub async fn server_info(&self) -> Option<ServerInfo> {
        let info = self.server_info.read().await;
        info.clone()
    }

    /// Check if the client is connected
    pub async fn is_connected(&self) -> bool {
        let connected = self.connected.read().await;
        *connected
    }

    // ========================================================================
    // Connection Management
    // ========================================================================

    /// Connect to an MCP server using the provided transport
    pub async fn connect<T>(&mut self, transport: T) -> McpResult<InitializeResult>
    where
        T: Transport + 'static,
    {
        // Set the transport
        {
            let mut transport_guard = self.transport.lock().await;
            *transport_guard = Some(Box::new(transport));
        }

        // Initialize the connection
        let init_result = self.initialize().await?;

        // Mark as connected
        {
            let mut connected = self.connected.write().await;
            *connected = true;
        }

        Ok(init_result)
    }

    /// Disconnect from the server
    pub async fn disconnect(&self) -> McpResult<()> {
        // Close the transport
        {
            let mut transport_guard = self.transport.lock().await;
            if let Some(transport) = transport_guard.as_mut() {
                transport.close().await?;
            }
            *transport_guard = None;
        }

        // Clear server information
        {
            let mut server_capabilities = self.server_capabilities.write().await;
            *server_capabilities = None;
        }
        {
            let mut server_info = self.server_info.write().await;
            *server_info = None;
        }

        // Mark as disconnected
        {
            let mut connected = self.connected.write().await;
            *connected = false;
        }

        Ok(())
    }

    /// Initialize the connection with the server
    async fn initialize(&self) -> McpResult<InitializeResult> {
        let params = InitializeParams::new(
            self.info.clone(),
            self.capabilities.clone(),
        );

        let request = JsonRpcRequest::new(
            Value::from(self.next_request_id().await),
            methods::INITIALIZE.to_string(),
            Some(params),
        )?;

        let response = self.send_request(request).await?;

        // The send_request method will return an error if there was a JSON-RPC error
        // so we can safely extract the result here

        let result: InitializeResult = serde_json::from_value(
            response
                .result
                .ok_or_else(|| McpError::Protocol("Missing initialize result".to_string()))?,
        )?;

        // Store server information
        {
            let mut server_capabilities = self.server_capabilities.write().await;
            *server_capabilities = Some(result.capabilities.clone());
        }
        {
            let mut server_info = self.server_info.write().await;
            *server_info = Some(result.server_info.clone());
        }

        Ok(result)
    }

    // ========================================================================
    // Tool Operations
    // ========================================================================

    /// List available tools from the server
    pub async fn list_tools(&self, cursor: Option<String>) -> McpResult<ListToolsResult> {
        self.ensure_connected().await?;

        let params = ListToolsParams { cursor, meta: None };
        let request = JsonRpcRequest::new(
            Value::from(self.next_request_id().await),
            methods::TOOLS_LIST.to_string(),
            Some(params),
        )?;

        let response = self.send_request(request).await?;
        self.handle_response(response)
    }

    /// Call a tool on the server
    pub async fn call_tool(
        &self,
        name: String,
        arguments: Option<HashMap<String, Value>>,
    ) -> McpResult<CallToolResult> {
        self.ensure_connected().await?;

        let params = CallToolParams::new(name, arguments);

        if self.config.validate_requests {
            validate_call_tool_params(&params)?;
        }

        let request = JsonRpcRequest::new(
            Value::from(self.next_request_id().await),
            methods::TOOLS_CALL.to_string(),
            Some(params),
        )?;

        let response = self.send_request(request).await?;
        self.handle_response(response)
    }

    // ========================================================================
    // Resource Operations
    // ========================================================================

    /// List available resources from the server
    pub async fn list_resources(&self, cursor: Option<String>) -> McpResult<ListResourcesResult> {
        self.ensure_connected().await?;

        let params = ListResourcesParams { cursor, meta: None };
        let request = JsonRpcRequest::new(
            Value::from(self.next_request_id().await),
            methods::RESOURCES_LIST.to_string(),
            Some(params),
        )?;

        let response = self.send_request(request).await?;
        self.handle_response(response)
    }

    /// Read a resource from the server
    pub async fn read_resource(&self, uri: String) -> McpResult<ReadResourceResult> {
        self.ensure_connected().await?;

        let params = ReadResourceParams::new(uri);

        if self.config.validate_requests {
            validate_read_resource_params(&params)?;
        }

        let request = JsonRpcRequest::new(
            Value::from(self.next_request_id().await),
            methods::RESOURCES_READ.to_string(),
            Some(params),
        )?;

        let response = self.send_request(request).await?;
        self.handle_response(response)
    }

    /// Subscribe to resource updates
    pub async fn subscribe_resource(&self, uri: String) -> McpResult<SubscribeResourceResult> {
        self.ensure_connected().await?;

        let params = SubscribeResourceParams { uri, meta: None };
        let request = JsonRpcRequest::new(
            Value::from(self.next_request_id().await),
            methods::RESOURCES_SUBSCRIBE.to_string(),
            Some(params),
        )?;

        let response = self.send_request(request).await?;
        self.handle_response(response)
    }

    /// Unsubscribe from resource updates
    pub async fn unsubscribe_resource(&self, uri: String) -> McpResult<UnsubscribeResourceResult> {
        self.ensure_connected().await?;

        let params = UnsubscribeResourceParams { uri, meta: None };
        let request = JsonRpcRequest::new(
            Value::from(self.next_request_id().await),
            methods::RESOURCES_UNSUBSCRIBE.to_string(),
            Some(params),
        )?;

        let response = self.send_request(request).await?;
        self.handle_response(response)
    }

    // ========================================================================
    // Prompt Operations
    // ========================================================================

    /// List available prompts from the server
    pub async fn list_prompts(&self, cursor: Option<String>) -> McpResult<ListPromptsResult> {
        self.ensure_connected().await?;

        let params = ListPromptsParams { cursor, meta: None };
        let request = JsonRpcRequest::new(
            Value::from(self.next_request_id().await),
            methods::PROMPTS_LIST.to_string(),
            Some(params),
        )?;

        let response = self.send_request(request).await?;
        self.handle_response(response)
    }

    /// Get a prompt from the server
    pub async fn get_prompt(
        &self,
        name: String,
        arguments: Option<HashMap<String, Value>>,
    ) -> McpResult<GetPromptResult> {
        self.ensure_connected().await?;

        let params = GetPromptParams::new(name, arguments);

        if self.config.validate_requests {
            validate_get_prompt_params(&params)?;
        }

        let request = JsonRpcRequest::new(
            Value::from(self.next_request_id().await),
            methods::PROMPTS_GET.to_string(),
            Some(params),
        )?;

        let response = self.send_request(request).await?;
        self.handle_response(response)
    }

    // ========================================================================
    // Sampling Operations (if supported by server)
    // ========================================================================

    /// Create a message using server-side sampling
    pub async fn create_message(
        &self,
        params: CreateMessageParams,
    ) -> McpResult<CreateMessageResult> {
        self.ensure_connected().await?;

        // Check if server supports sampling
        {
            let server_capabilities = self.server_capabilities.read().await;
            if let Some(capabilities) = server_capabilities.as_ref() {
                if capabilities.sampling.is_none() {
                    return Err(McpError::Protocol(
                        "Server does not support sampling".to_string(),
                    ));
                }
            } else {
                return Err(McpError::Protocol("Not connected to server".to_string()));
            }
        }

        if self.config.validate_requests {
            validate_create_message_params(&params)?;
        }

        let request = JsonRpcRequest::new(
            Value::from(self.next_request_id().await),
            methods::SAMPLING_CREATE_MESSAGE.to_string(),
            Some(params),
        )?;

        let response = self.send_request(request).await?;
        self.handle_response(response)
    }

    // ========================================================================
    // Utility Operations
    // ========================================================================

    /// Send a ping to the server
    pub async fn ping(&self) -> McpResult<PingResult> {
        self.ensure_connected().await?;

        let request = JsonRpcRequest::new(
            Value::from(self.next_request_id().await),
            methods::PING.to_string(),
            Some(PingParams { meta: None }),
        )?;

        let response = self.send_request(request).await?;
        self.handle_response(response)
    }

    /// Set the logging level on the server
    pub async fn set_logging_level(&self, level: LoggingLevel) -> McpResult<SetLoggingLevelResult> {
        self.ensure_connected().await?;

        let params = SetLoggingLevelParams { level, meta: None };
        let request = JsonRpcRequest::new(
            Value::from(self.next_request_id().await),
            methods::LOGGING_SET_LEVEL.to_string(),
            Some(params),
        )?;

        let response = self.send_request(request).await?;
        self.handle_response(response)
    }

    // ========================================================================
    // Notification Handling
    // ========================================================================

    /// Receive notifications from the server
    pub async fn receive_notification(&self) -> McpResult<Option<JsonRpcNotification>> {
        let mut transport_guard = self.transport.lock().await;
        if let Some(transport) = transport_guard.as_mut() {
            transport.receive_notification().await
        } else {
            Err(McpError::Transport("Not connected".to_string()))
        }
    }

    // ========================================================================
    // Helper Methods
    // ========================================================================

    /// Send a request and get a response
    async fn send_request(&self, request: JsonRpcRequest) -> McpResult<JsonRpcResponse> {
        if self.config.validate_requests {
            validate_jsonrpc_request(&request)?;
            validate_mcp_request(&request.method, request.params.as_ref())?;
        }

        let mut transport_guard = self.transport.lock().await;
        if let Some(transport) = transport_guard.as_mut() {
            let response = transport.send_request(request).await?;

            if self.config.validate_responses {
                validate_jsonrpc_response(&response)?;
            }

            Ok(response)
        } else {
            Err(McpError::Transport("Not connected".to_string()))
        }
    }

    /// Handle a JSON-RPC response and extract the result
    fn handle_response<T>(&self, response: JsonRpcResponse) -> McpResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        // JsonRpcResponse only contains successful responses
        // Errors are handled separately by the transport layer
        let result = response
            .result
            .ok_or_else(|| McpError::Protocol("Missing result in response".to_string()))?;

        serde_json::from_value(result).map_err(|e| McpError::Serialization(e.to_string()))
    }

    /// Ensure the client is connected
    async fn ensure_connected(&self) -> McpResult<()> {
        if !self.is_connected().await {
            return Err(McpError::Connection("Not connected to server".to_string()));
        }
        Ok(())
    }

    /// Get the next request ID
    async fn next_request_id(&self) -> u64 {
        let mut counter = self.request_counter.lock().await;
        *counter += 1;
        *counter
    }
}

/// Client builder for easier construction
pub struct McpClientBuilder {
    name: String,
    version: String,
    capabilities: ClientCapabilities,
    config: ClientConfig,
}

impl McpClientBuilder {
    /// Create a new client builder
    pub fn new(name: String, version: String) -> Self {
        Self {
            name,
            version,
            capabilities: ClientCapabilities::default(),
            config: ClientConfig::default(),
        }
    }

    /// Set client capabilities
    pub fn capabilities(mut self, capabilities: ClientCapabilities) -> Self {
        self.capabilities = capabilities;
        self
    }

    /// Set client configuration
    pub fn config(mut self, config: ClientConfig) -> Self {
        self.config = config;
        self
    }

    /// Set request timeout
    pub fn request_timeout(mut self, timeout_ms: u64) -> Self {
        self.config.request_timeout_ms = timeout_ms;
        self
    }

    /// Set maximum retries
    pub fn max_retries(mut self, retries: u32) -> Self {
        self.config.max_retries = retries;
        self
    }

    /// Enable or disable request validation
    pub fn validate_requests(mut self, validate: bool) -> Self {
        self.config.validate_requests = validate;
        self
    }

    /// Enable or disable response validation
    pub fn validate_responses(mut self, validate: bool) -> Self {
        self.config.validate_responses = validate;
        self
    }

    /// Build the client
    pub fn build(self) -> McpClient {
        let mut client = McpClient::new(self.name, self.version);
        client.set_capabilities(self.capabilities);
        client.config = self.config;
        client
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;

    // Mock transport for testing
    struct MockTransport {
        responses: Vec<JsonRpcResponse>,
        current: usize,
    }

    impl MockTransport {
        fn new(responses: Vec<JsonRpcResponse>) -> Self {
            Self {
                responses,
                current: 0,
            }
        }
    }

    #[async_trait]
    impl Transport for MockTransport {
        async fn send_request(&mut self, _request: JsonRpcRequest) -> McpResult<JsonRpcResponse> {
            if self.current < self.responses.len() {
                let response = self.responses[self.current].clone();
                self.current += 1;
                Ok(response)
            } else {
                Err(McpError::Transport("No more responses".to_string()))
            }
        }

        async fn send_notification(&mut self, _notification: JsonRpcNotification) -> McpResult<()> {
            Ok(())
        }

        async fn receive_notification(&mut self) -> McpResult<Option<JsonRpcNotification>> {
            Ok(None)
        }

        async fn close(&mut self) -> McpResult<()> {
            Ok(())
        }
    }

    #[tokio::test]
    async fn test_client_creation() {
        let client = McpClient::new("test-client".to_string(), "1.0.0".to_string());
        assert_eq!(client.info().name, "test-client");
        assert_eq!(client.info().version, "1.0.0");
        assert!(!client.is_connected().await);
    }

    #[tokio::test]
    async fn test_client_builder() {
        let client = McpClientBuilder::new("test-client".to_string(), "1.0.0".to_string())
            .request_timeout(5000)
            .max_retries(5)
            .validate_requests(false)
            .build();

        assert_eq!(client.config().request_timeout_ms, 5000);
        assert_eq!(client.config().max_retries, 5);
        assert!(!client.config().validate_requests);
    }

    #[tokio::test]
    async fn test_mock_connection() {
        let init_result = InitializeResult::new(
            ServerInfo {
                name: "test-server".to_string(),
                version: "1.0.0".to_string(),
            },
            ServerCapabilities::default(),
            Some("Test client for MCP 2025-03-26".to_string()),
        );

        let init_response = JsonRpcResponse::success(Value::from(1), init_result.clone()).unwrap();

        let transport = MockTransport::new(vec![init_response]);

        let mut client = McpClient::new("test-client".to_string(), "1.0.0".to_string());
        let result = client.connect(transport).await.unwrap();

        assert_eq!(result.server_info.name, "test-server");
        assert!(client.is_connected().await);
    }

    #[tokio::test]
    async fn test_disconnect() {
        let init_result = InitializeResult::new(
            ServerInfo {
                name: "test-server".to_string(),
                version: "1.0.0".to_string(),
            },
            ServerCapabilities::default(),
            Some("Test client for MCP 2025-03-26".to_string()),
        );

        let init_response = JsonRpcResponse::success(Value::from(1), init_result).unwrap();

        let transport = MockTransport::new(vec![init_response]);

        let mut client = McpClient::new("test-client".to_string(), "1.0.0".to_string());
        client.connect(transport).await.unwrap();

        assert!(client.is_connected().await);

        client.disconnect().await.unwrap();
        assert!(!client.is_connected().await);
        assert!(client.server_info().await.is_none());
        assert!(client.server_capabilities().await.is_none());
    }
}
