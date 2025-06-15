//! HTTP transport implementation for MCP
//!
//! This module provides HTTP-based transport for MCP communication,
//! including Server-Sent Events (SSE) for real-time communication.

use async_trait::async_trait;
use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::{sse::Event, Sse},
    routing::{get, post},
    Json, Router,
};
use reqwest::Client;
use serde_json::Value;
use std::{collections::HashMap, convert::Infallible, sync::Arc, time::Duration};
use tokio::sync::{broadcast, mpsc, Mutex, RwLock};

#[cfg(all(feature = "futures", feature = "tokio-stream"))]
use futures::stream::Stream;

#[cfg(feature = "tokio-stream")]
use tokio_stream::{wrappers::BroadcastStream, StreamExt};

use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};

use crate::core::error::{McpError, McpResult};
use crate::protocol::types::{JsonRpcNotification, JsonRpcRequest, JsonRpcResponse, JsonRpcError, error_codes, JsonRpcMessage};
use crate::transport::traits::{ConnectionState, ServerTransport, Transport, TransportConfig};

// ============================================================================
// HTTP Client Transport
// ============================================================================

/// HTTP transport for MCP clients
///
/// This transport communicates with an MCP server via HTTP requests and
/// optionally uses Server-Sent Events for real-time notifications.
pub struct HttpClientTransport {
    client: Client,
    base_url: String,
    sse_url: Option<String>,
    headers: HeaderMap,
    /// For tracking active requests (currently used for metrics/debugging)
    pending_requests: Arc<Mutex<HashMap<Value, tokio::sync::oneshot::Sender<JsonRpcResponse>>>>,
    notification_receiver: Option<mpsc::UnboundedReceiver<JsonRpcNotification>>,
    config: TransportConfig,
    state: ConnectionState,
    request_id_counter: Arc<Mutex<u64>>,
}

impl HttpClientTransport {
    /// Create a new HTTP client transport
    ///
    /// # Arguments
    /// * `base_url` - Base URL for the MCP server
    /// * `sse_url` - Optional URL for Server-Sent Events (for notifications)
    ///
    /// # Returns
    /// Result containing the transport or an error
    pub async fn new<S: AsRef<str>>(base_url: S, sse_url: Option<S>) -> McpResult<Self> {
        Self::with_config(base_url, sse_url, TransportConfig::default()).await
    }

    /// Create a new HTTP client transport with custom configuration
    ///
    /// # Arguments
    /// * `base_url` - Base URL for the MCP server
    /// * `sse_url` - Optional URL for Server-Sent Events
    /// * `config` - Transport configuration
    ///
    /// # Returns
    /// Result containing the transport or an error
    pub async fn with_config<S: AsRef<str>>(
        base_url: S,
        sse_url: Option<S>,
        config: TransportConfig,
    ) -> McpResult<Self> {
        let client_builder = Client::builder()
            .timeout(Duration::from_millis(
                config.read_timeout_ms.unwrap_or(60_000),
            ))
            .connect_timeout(Duration::from_millis(
                config.connect_timeout_ms.unwrap_or(30_000),
            ));

        // Note: reqwest doesn't have a gzip() method, it's enabled by default with features

        let client = client_builder
            .build()
            .map_err(|e| McpError::Http(format!("Failed to create HTTP client: {}", e)))?;

        let mut headers = HeaderMap::new();
        headers.insert("Content-Type", "application/json".parse().unwrap());
        headers.insert("Accept", "application/json".parse().unwrap());

        // Add custom headers from config
        for (key, value) in &config.headers {
            if let (Ok(header_name), Ok(header_value)) = (
                key.parse::<axum::http::HeaderName>(),
                value.parse::<axum::http::HeaderValue>(),
            ) {
                headers.insert(header_name, header_value);
            }
        }

        let (notification_sender, notification_receiver) = mpsc::unbounded_channel();

        // Set up SSE connection for notifications if URL provided
        if let Some(sse_url) = &sse_url {
            let sse_url = sse_url.as_ref().to_string();
            let client_clone = client.clone();
            let headers_clone = headers.clone();

            tokio::spawn(async move {
                if let Err(e) = Self::handle_sse_stream(
                    client_clone,
                    sse_url,
                    headers_clone,
                    notification_sender,
                )
                .await
                {
                    tracing::error!("SSE stream error: {}", e);
                }
            });
        }

        Ok(Self {
            client,
            base_url: base_url.as_ref().to_string(),
            sse_url: sse_url.map(|s| s.as_ref().to_string()),
            headers,
            pending_requests: Arc::new(Mutex::new(HashMap::new())),
            notification_receiver: Some(notification_receiver),
            config,
            state: ConnectionState::Connected,
            request_id_counter: Arc::new(Mutex::new(0)),
        })
    }

    async fn handle_sse_stream(
        client: Client,
        sse_url: String,
        headers: HeaderMap,
        notification_sender: mpsc::UnboundedSender<JsonRpcNotification>,
    ) -> McpResult<()> {
        let mut request = client.get(&sse_url);
        for (name, value) in headers.iter() {
            // Convert axum headers to reqwest headers
            let name_str = name.as_str();
            let value_bytes = value.as_bytes();
            request = request.header(name_str, value_bytes);
        }

        let response = request
            .send()
            .await
            .map_err(|e| McpError::Http(format!("SSE connection failed: {}", e)))?;

        let mut stream = response.bytes_stream();

        #[cfg(feature = "tokio-stream")]
        {
            while let Some(chunk) = stream.next().await {
                match chunk {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        for line in text.lines() {
                            if line.starts_with("data: ") {
                                let data = &line[6..]; // Remove "data: " prefix
                                if let Ok(notification) =
                                    serde_json::from_str::<JsonRpcNotification>(data)
                                {
                                    if notification_sender.send(notification).is_err() {
                                        tracing::debug!("Notification receiver dropped");
                                        return Ok(());
                                    }
                                }
                            }
                        }
                    }
                    Err(e) => {
                        tracing::error!("SSE stream error: {}", e);
                        break;
                    }
                }
            }
        }

        #[cfg(not(feature = "tokio-stream"))]
        {
            tracing::warn!("SSE streaming requires tokio-stream feature");
        }

        Ok(())
    }

    async fn next_request_id(&self) -> u64 {
        let mut counter = self.request_id_counter.lock().await;
        *counter += 1;
        *counter
    }

    /// Track request for metrics/debugging purposes
    async fn track_request(&self, request_id: &Value) {
        // For HTTP transport, we mainly use this for debugging and metrics
        // Since HTTP is synchronous request/response, we don't need the async
        // tracking that WebSocket uses, but we keep the interface for consistency
        let mut pending = self.pending_requests.lock().await;
        let (sender, _receiver) = tokio::sync::oneshot::channel();
        pending.insert(request_id.clone(), sender);
    }

    /// Remove tracked request
    async fn untrack_request(&self, request_id: &Value) {
        let mut pending = self.pending_requests.lock().await;
        pending.remove(request_id);
    }

    /// Get count of active requests (for debugging/metrics)
    pub async fn active_request_count(&self) -> usize {
        let pending = self.pending_requests.lock().await;
        pending.len()
    }
}

#[async_trait]
impl Transport for HttpClientTransport {
    async fn send_request(&mut self, request: JsonRpcRequest) -> McpResult<JsonRpcResponse> {
        // Generate request ID if not present or ensure we have a valid ID
        let request_with_id = if request.id == Value::Null {
            let request_id = self.next_request_id().await;
            JsonRpcRequest {
                id: Value::from(request_id),
                ..request
            }
        } else {
            request
        };

        // Track the request for debugging/metrics
        self.track_request(&request_with_id.id).await;

        let url = format!("{}/mcp", self.base_url);

        let mut http_request = self.client.post(&url);
        
        // Apply headers from config and defaults
        for (name, value) in self.headers.iter() {
            let name_str = name.as_str();
            let value_bytes = value.as_bytes();
            http_request = http_request.header(name_str, value_bytes);
        }

        // Apply timeout from config if specified
        if let Some(timeout_ms) = self.config.read_timeout_ms {
            http_request = http_request.timeout(Duration::from_millis(timeout_ms));
        }

        let response = http_request
            .json(&request_with_id)
            .send()
            .await
            .map_err(|e| {
                // Untrack request on error
                let request_id = request_with_id.id.clone();
                let pending_requests = self.pending_requests.clone();
                tokio::spawn(async move {
                    let mut pending = pending_requests.lock().await;
                    pending.remove(&request_id);
                });
                McpError::Http(format!("HTTP request failed: {}", e))
            })?;

        if !response.status().is_success() {
            // Untrack request on HTTP error
            self.untrack_request(&request_with_id.id).await;
            return Err(McpError::Http(format!(
                "HTTP error: {} {}",
                response.status().as_u16(),
                response.status().canonical_reason().unwrap_or("Unknown")
            )));
        }

        let json_response: JsonRpcResponse = response
            .json()
            .await
            .map_err(|e| {
                // Untrack request on parse error
                let request_id = request_with_id.id.clone();
                let pending_requests = self.pending_requests.clone();
                tokio::spawn(async move {
                    let mut pending = pending_requests.lock().await;
                    pending.remove(&request_id);
                });
                McpError::Http(format!("Failed to parse response: {}", e))
            })?;

        // Validate response ID matches request ID
        if json_response.id != request_with_id.id {
            self.untrack_request(&request_with_id.id).await;
            return Err(McpError::Http(format!(
                "Response ID {:?} does not match request ID {:?}",
                json_response.id, request_with_id.id
            )));
        }

        // Untrack successful request
        self.untrack_request(&request_with_id.id).await;

        Ok(json_response)
    }

    async fn send_notification(&mut self, notification: JsonRpcNotification) -> McpResult<()> {
        let url = format!("{}/mcp/notify", self.base_url);

        let mut http_request = self.client.post(&url);
        
        // Apply headers from config and defaults
        for (name, value) in self.headers.iter() {
            let name_str = name.as_str();
            let value_bytes = value.as_bytes();
            http_request = http_request.header(name_str, value_bytes);
        }

        // Apply write timeout from config if specified
        if let Some(timeout_ms) = self.config.write_timeout_ms {
            http_request = http_request.timeout(Duration::from_millis(timeout_ms));
        }

        let response = http_request
            .json(&notification)
            .send()
            .await
            .map_err(|e| McpError::Http(format!("HTTP notification failed: {}", e)))?;

        if !response.status().is_success() {
            return Err(McpError::Http(format!(
                "HTTP notification error: {} {}",
                response.status().as_u16(),
                response.status().canonical_reason().unwrap_or("Unknown")
            )));
        }

        Ok(())
    }

    async fn receive_notification(&mut self) -> McpResult<Option<JsonRpcNotification>> {
        if let Some(ref mut receiver) = self.notification_receiver {
            match receiver.try_recv() {
                Ok(notification) => Ok(Some(notification)),
                Err(mpsc::error::TryRecvError::Empty) => Ok(None),
                Err(mpsc::error::TryRecvError::Disconnected) => Err(McpError::Http(
                    "Notification channel disconnected".to_string(),
                )),
            }
        } else {
            Ok(None)
        }
    }

    async fn close(&mut self) -> McpResult<()> {
        self.state = ConnectionState::Disconnected;
        self.notification_receiver = None;
        Ok(())
    }

    fn is_connected(&self) -> bool {
        matches!(self.state, ConnectionState::Connected)
    }

    fn connection_info(&self) -> String {
        format!(
            "HTTP transport (base: {}, sse: {:?}, state: {:?})",
            self.base_url, self.sse_url, self.state
        )
    }
}

// ============================================================================
// HTTP Server Transport
// ============================================================================

/// Shared state for HTTP server transport
#[derive(Clone)]
struct HttpServerState {
    notification_sender: broadcast::Sender<JsonRpcNotification>,
    request_handler: Option<
        Arc<
            dyn Fn(JsonRpcRequest) -> tokio::sync::oneshot::Receiver<JsonRpcResponse> + Send + Sync,
        >,
    >,
}

/// HTTP transport for MCP servers
///
/// This transport serves MCP requests over HTTP and provides Server-Sent Events
/// for real-time notifications to clients.
pub struct HttpServerTransport {
    bind_addr: String,
    config: TransportConfig,
    state: Arc<RwLock<HttpServerState>>,
    server_handle: Option<tokio::task::JoinHandle<()>>,
    running: Arc<RwLock<bool>>,
}

impl HttpServerTransport {
    /// Create a new HTTP server transport
    ///
    /// # Arguments
    /// * `bind_addr` - Address to bind the HTTP server to (e.g., "0.0.0.0:3000")
    ///
    /// # Returns
    /// New HTTP server transport instance
    pub fn new<S: Into<String>>(bind_addr: S) -> Self {
        Self::with_config(bind_addr, TransportConfig::default())
    }

    /// Create a new HTTP server transport with custom configuration
    ///
    /// # Arguments
    /// * `bind_addr` - Address to bind the HTTP server to
    /// * `config` - Transport configuration
    ///
    /// # Returns
    /// New HTTP server transport instance
    pub fn with_config<S: Into<String>>(bind_addr: S, config: TransportConfig) -> Self {
        let (notification_sender, _) = broadcast::channel(1000);

        Self {
            bind_addr: bind_addr.into(),
            config,
            state: Arc::new(RwLock::new(HttpServerState {
                notification_sender,
                request_handler: None,
            })),
            server_handle: None,
            running: Arc::new(RwLock::new(false)),
        }
    }

    /// Set the request handler function
    ///
    /// # Arguments
    /// * `handler` - Function that processes incoming requests
    pub async fn set_request_handler<F>(&mut self, handler: F)
    where
        F: Fn(JsonRpcRequest) -> tokio::sync::oneshot::Receiver<JsonRpcResponse>
            + Send
            + Sync
            + 'static,
    {
        let mut state = self.state.write().await;
        state.request_handler = Some(Arc::new(handler));
    }
}

#[async_trait]
impl ServerTransport for HttpServerTransport {
    async fn start(&mut self) -> McpResult<()> {
        tracing::info!("Starting HTTP server on {}", self.bind_addr);

        let state = self.state.clone();
        let bind_addr = self.bind_addr.clone();
        let running = self.running.clone();
        let _config = self.config.clone(); // TODO: Use config for timeouts/limits

        // Create the Axum app with configuration-based settings
        let mut app = Router::new()
            .route("/mcp", post(handle_mcp_request))
            .route("/mcp/notify", post(handle_mcp_notification))
            .route("/mcp/events", get(handle_sse_events))
            .route("/health", get(handle_health_check))
            .with_state(state);

        // Apply CORS configuration
        let cors_layer = CorsLayer::new()
            .allow_origin(Any)
            .allow_methods(Any)
            .allow_headers(Any);

        app = app.layer(ServiceBuilder::new().layer(cors_layer).into_inner());

        // Start the server
        let listener = tokio::net::TcpListener::bind(&bind_addr)
            .await
            .map_err(|e| McpError::Http(format!("Failed to bind to {}: {}", bind_addr, e)))?;

        *running.write().await = true;

        let server_handle = tokio::spawn(async move {
            if let Err(e) = axum::serve(listener, app).await {
                tracing::error!("HTTP server error: {}", e);
            }
        });

        self.server_handle = Some(server_handle);

        tracing::info!("HTTP server started successfully on {}", self.bind_addr);
        Ok(())
    }

    async fn handle_request(&mut self, request: JsonRpcRequest) -> McpResult<JsonRpcResponse> {
        // This is now handled by the HTTP server itself and should not be called directly
        // The HTTP transport handles requests through the HTTP server routes
        tracing::warn!("handle_request called directly on HTTP transport - this may indicate a configuration issue");
        
        let state = self.state.read().await;

        if let Some(ref handler) = state.request_handler {
            let response_rx = handler(request);
            drop(state); // Release the lock

            match response_rx.await {
                Ok(response) => Ok(response),
                Err(_) => Err(McpError::Http("Request handler channel closed".to_string())),
            }
        } else {
            // Return an error indicating no handler is configured
            Err(McpError::Http("No request handler configured for HTTP transport".to_string()))
        }
    }

    async fn send_notification(&mut self, notification: JsonRpcNotification) -> McpResult<()> {
        let state = self.state.read().await;

        if let Err(_) = state.notification_sender.send(notification) {
            tracing::warn!("No SSE clients connected to receive notification");
        }

        Ok(())
    }

    async fn stop(&mut self) -> McpResult<()> {
        tracing::info!("Stopping HTTP server");

        *self.running.write().await = false;

        if let Some(handle) = self.server_handle.take() {
            handle.abort();
        }

        Ok(())
    }

    fn is_running(&self) -> bool {
        // Check if we have an active server handle
        self.server_handle.is_some()
    }

    fn server_info(&self) -> String {
        format!("HTTP server transport (bind: {})", self.bind_addr)
    }
}

// ============================================================================
// HTTP Route Handlers
// ============================================================================

/// Handle MCP JSON-RPC requests
async fn handle_mcp_request(
    State(state): State<Arc<RwLock<HttpServerState>>>,
    Json(request): Json<JsonRpcRequest>,
) -> Result<Json<JsonRpcMessage>, StatusCode> {
    let state_guard = state.read().await;

    if let Some(ref handler) = state_guard.request_handler {
        let response_rx = handler(request);
        drop(state_guard); // Release the lock

        match response_rx.await {
            Ok(response) => Ok(Json(JsonRpcMessage::Response(response))),
            Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
        }
    } else {
        let error_response = JsonRpcError::error(
            request.id,
            error_codes::METHOD_NOT_FOUND,
            "No request handler configured".to_string(),
            None,
        );
        Ok(Json(JsonRpcMessage::Error(error_response)))
    }
}

/// Handle MCP notification requests
async fn handle_mcp_notification(Json(_notification): Json<JsonRpcNotification>) -> StatusCode {
    // Notifications don't require a response
    StatusCode::OK
}

/// Handle Server-Sent Events for real-time notifications
#[cfg(all(feature = "tokio-stream", feature = "futures"))]
async fn handle_sse_events(
    State(state): State<Arc<RwLock<HttpServerState>>>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    let state_guard = state.read().await;
    let receiver = state_guard.notification_sender.subscribe();
    drop(state_guard);

    let stream = BroadcastStream::new(receiver).map(|result| {
        match result {
            Ok(notification) => match serde_json::to_string(&notification) {
                Ok(json) => Ok(Event::default().data(json)),
                Err(e) => {
                    tracing::error!("Failed to serialize notification: {}", e);
                    Ok(Event::default().data("{}"))
                }
            },
            Err(_) => Ok(Event::default().data("{}")), // Lagged or closed
        }
    });

    Sse::new(stream).keep_alive(
        axum::response::sse::KeepAlive::new()
            .interval(Duration::from_secs(30))
            .text("keep-alive"),
    )
}

/// Handle Server-Sent Events (fallback when features not available)
#[cfg(not(all(feature = "tokio-stream", feature = "futures")))]
async fn handle_sse_events(_state: State<Arc<RwLock<HttpServerState>>>) -> StatusCode {
    StatusCode::NOT_IMPLEMENTED
}

/// Handle health check requests
async fn handle_health_check() -> Json<Value> {
    #[cfg(feature = "chrono")]
    let timestamp = chrono::Utc::now().to_rfc3339();
    #[cfg(not(feature = "chrono"))]
    let timestamp = "unavailable";

    Json(serde_json::json!({
        "status": "healthy",
        "transport": "http",
        "timestamp": timestamp
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_http_client_creation() {
        let transport = HttpClientTransport::new("http://localhost:3000", None).await;
        assert!(transport.is_ok());

        let transport = transport.unwrap();
        assert!(transport.is_connected());
        assert_eq!(transport.base_url, "http://localhost:3000");
    }

    #[tokio::test]
    async fn test_http_server_creation() {
        let transport = HttpServerTransport::new("127.0.0.1:0");
        assert_eq!(transport.bind_addr, "127.0.0.1:0");
        assert!(!transport.is_running());
    }

    #[test]
    fn test_http_server_with_config() {
        let mut config = TransportConfig::default();
        config.compression = true;

        let transport = HttpServerTransport::with_config("0.0.0.0:8080", config);
        assert_eq!(transport.bind_addr, "0.0.0.0:8080");
        assert!(transport.config.compression);
    }

    #[tokio::test]
    async fn test_http_client_with_sse() {
        let transport = HttpClientTransport::new(
            "http://localhost:3000",
            Some("http://localhost:3000/events"),
        )
        .await;

        assert!(transport.is_ok());
        let transport = transport.unwrap();
        assert!(transport.sse_url.is_some());
        assert_eq!(transport.sse_url.unwrap(), "http://localhost:3000/events");
    }
}
