//! Client session management
//!
//! This module provides session management for MCP clients, including connection
//! state tracking, notification handling, and automatic reconnection capabilities.

use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{broadcast, mpsc, watch, Mutex, RwLock};
use tokio::time::{sleep, timeout};

use crate::client::mcp_client::McpClient;
use crate::core::error::{McpError, McpResult};
use crate::protocol::{messages::*, types::*};
use crate::transport::traits::Transport;

/// Session state
#[derive(Debug, Clone, PartialEq)]
pub enum SessionState {
    /// Session is disconnected
    Disconnected,
    /// Session is connecting
    Connecting,
    /// Session is connected and active
    Connected,
    /// Session is reconnecting after a failure
    Reconnecting,
    /// Session has failed and cannot reconnect
    Failed(String),
}

/// Notification handler trait
pub trait NotificationHandler: Send + Sync {
    /// Handle a notification from the server
    fn handle_notification(&self, notification: JsonRpcNotification);
}

/// Session configuration
#[derive(Debug, Clone)]
pub struct SessionConfig {
    /// Whether to enable automatic reconnection
    pub auto_reconnect: bool,
    /// Maximum number of reconnection attempts
    pub max_reconnect_attempts: u32,
    /// Initial reconnection delay in milliseconds
    pub reconnect_delay_ms: u64,
    /// Maximum reconnection delay in milliseconds
    pub max_reconnect_delay_ms: u64,
    /// Reconnection backoff multiplier
    pub reconnect_backoff: f64,
    /// Connection timeout in milliseconds
    pub connection_timeout_ms: u64,
    /// Heartbeat interval in milliseconds (0 to disable)
    pub heartbeat_interval_ms: u64,
    /// Heartbeat timeout in milliseconds
    pub heartbeat_timeout_ms: u64,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            auto_reconnect: true,
            max_reconnect_attempts: 5,
            reconnect_delay_ms: 1000,
            max_reconnect_delay_ms: 30000,
            reconnect_backoff: 2.0,
            connection_timeout_ms: 10000,
            heartbeat_interval_ms: 30000,
            heartbeat_timeout_ms: 5000,
        }
    }
}

/// Client session that manages connection lifecycle and notifications
pub struct ClientSession {
    /// The underlying MCP client
    client: Arc<Mutex<McpClient>>,
    /// Session configuration
    config: SessionConfig,
    /// Current session state
    state: Arc<RwLock<SessionState>>,
    /// State change broadcaster
    state_tx: watch::Sender<SessionState>,
    /// State change receiver
    state_rx: watch::Receiver<SessionState>,
    /// Notification handlers
    notification_handlers: Arc<RwLock<Vec<Box<dyn NotificationHandler>>>>,
    /// Connection timestamp
    connected_at: Arc<RwLock<Option<Instant>>>,
    /// Reconnection attempts counter
    reconnect_attempts: Arc<Mutex<u32>>,
    /// Shutdown signal
    shutdown_tx: Arc<Mutex<Option<mpsc::Sender<()>>>>,
}

impl ClientSession {
    /// Create a new client session
    pub fn new(client: McpClient) -> Self {
        let (state_tx, state_rx) = watch::channel(SessionState::Disconnected);

        Self {
            client: Arc::new(Mutex::new(client)),
            config: SessionConfig::default(),
            state: Arc::new(RwLock::new(SessionState::Disconnected)),
            state_tx,
            state_rx,
            notification_handlers: Arc::new(RwLock::new(Vec::new())),
            connected_at: Arc::new(RwLock::new(None)),
            reconnect_attempts: Arc::new(Mutex::new(0)),
            shutdown_tx: Arc::new(Mutex::new(None)),
        }
    }

    /// Create a new client session with custom configuration
    pub fn with_config(client: McpClient, config: SessionConfig) -> Self {
        let mut session = Self::new(client);
        session.config = config;
        session
    }

    /// Get the current session state
    pub async fn state(&self) -> SessionState {
        let state = self.state.read().await;
        state.clone()
    }

    /// Subscribe to state changes
    pub fn subscribe_state_changes(&self) -> watch::Receiver<SessionState> {
        self.state_rx.clone()
    }

    /// Check if the session is connected
    pub async fn is_connected(&self) -> bool {
        let state = self.state.read().await;
        matches!(*state, SessionState::Connected)
    }

    /// Get connection uptime
    pub async fn uptime(&self) -> Option<Duration> {
        let connected_at = self.connected_at.read().await;
        connected_at.map(|time| time.elapsed())
    }

    /// Add a notification handler
    pub async fn add_notification_handler<H>(&self, handler: H)
    where
        H: NotificationHandler + 'static,
    {
        let mut handlers = self.notification_handlers.write().await;
        handlers.push(Box::new(handler));
    }

    /// Connect to the server with the provided transport
    pub async fn connect<T>(&self, transport: T) -> McpResult<InitializeResult>
    where
        T: Transport + 'static,
    {
        self.transition_state(SessionState::Connecting).await?;

        let connect_future = async {
            let mut client = self.client.lock().await;
            client.connect(transport).await
        };

        let result = timeout(
            Duration::from_millis(self.config.connection_timeout_ms),
            connect_future,
        )
        .await;

        match result {
            Ok(Ok(init_result)) => {
                self.transition_state(SessionState::Connected).await?;

                // Record connection time
                {
                    let mut connected_at = self.connected_at.write().await;
                    *connected_at = Some(Instant::now());
                }

                // Reset reconnection attempts
                {
                    let mut attempts = self.reconnect_attempts.lock().await;
                    *attempts = 0;
                }

                // Start background tasks
                self.start_background_tasks().await?;

                Ok(init_result)
            }
            Ok(Err(error)) => {
                self.transition_state(SessionState::Failed(error.to_string()))
                    .await?;
                Err(error)
            }
            Err(_) => {
                let error = McpError::Connection("Connection timeout".to_string());
                self.transition_state(SessionState::Failed(error.to_string()))
                    .await?;
                Err(error)
            }
        }
    }

    /// Disconnect from the server
    pub async fn disconnect(&self) -> McpResult<()> {
        // Stop background tasks
        self.stop_background_tasks().await;

        // Disconnect the client
        {
            let client = self.client.lock().await;
            client.disconnect().await?;
        }

        // Update state
        self.transition_state(SessionState::Disconnected).await?;

        // Clear connection time
        {
            let mut connected_at = self.connected_at.write().await;
            *connected_at = None;
        }

        Ok(())
    }

    /// Reconnect to the server
    pub async fn reconnect<T>(
        &self,
        transport_factory: impl Fn() -> T,
    ) -> McpResult<InitializeResult>
    where
        T: Transport + 'static,
    {
        if !self.config.auto_reconnect {
            return Err(McpError::Connection(
                "Auto-reconnect is disabled".to_string(),
            ));
        }

        let mut attempts = self.reconnect_attempts.lock().await;
        if *attempts >= self.config.max_reconnect_attempts {
            let error = McpError::Connection("Max reconnection attempts exceeded".to_string());
            self.transition_state(SessionState::Failed(error.to_string()))
                .await?;
            return Err(error);
        }

        *attempts += 1;
        let current_attempts = *attempts;
        drop(attempts);

        self.transition_state(SessionState::Reconnecting).await?;

        // Calculate reconnection delay with exponential backoff
        let delay = std::cmp::min(
            (self.config.reconnect_delay_ms as f64
                * self
                    .config
                    .reconnect_backoff
                    .powi(current_attempts as i32 - 1)) as u64,
            self.config.max_reconnect_delay_ms,
        );

        sleep(Duration::from_millis(delay)).await;

        // Attempt to reconnect
        self.connect(transport_factory()).await
    }

    /// Get the underlying client (for direct operations)
    pub fn client(&self) -> Arc<Mutex<McpClient>> {
        self.client.clone()
    }

    /// Get session configuration
    pub fn config(&self) -> &SessionConfig {
        &self.config
    }

    // ========================================================================
    // Background Tasks
    // ========================================================================

    /// Start background tasks (notification handling, heartbeat)
    async fn start_background_tasks(&self) -> McpResult<()> {
        let (_shutdown_tx, shutdown_rx): (broadcast::Sender<()>, broadcast::Receiver<()>) =
            broadcast::channel(16);
        {
            let mut shutdown_guard = self.shutdown_tx.lock().await;
            *shutdown_guard = Some(mpsc::channel(1).0); // Store a dummy for interface compatibility
        }

        // Start notification handler task
        {
            let client = self.client.clone();
            let handlers = self.notification_handlers.clone();
            let mut shutdown_rx_clone = shutdown_rx.resubscribe();

            tokio::spawn(async move {
                loop {
                    tokio::select! {
                        _ = shutdown_rx_clone.recv() => break,
                        notification_result = async {
                            let client_guard = client.lock().await;
                            client_guard.receive_notification().await
                        } => {
                            match notification_result {
                                Ok(Some(notification)) => {
                                    let handlers_guard = handlers.read().await;
                                    for handler in handlers_guard.iter() {
                                        handler.handle_notification(notification.clone());
                                    }
                                }
                                Ok(None) => {
                                    // No notification available, continue
                                }
                                Err(_) => {
                                    // Error receiving notification, might be disconnected
                                    break;
                                }
                            }
                        }
                    }
                }
            });
        }

        // Start heartbeat task if enabled
        if self.config.heartbeat_interval_ms > 0 {
            let client = self.client.clone();
            let heartbeat_interval = Duration::from_millis(self.config.heartbeat_interval_ms);
            let heartbeat_timeout = Duration::from_millis(self.config.heartbeat_timeout_ms);
            let state = self.state.clone();
            let state_tx = self.state_tx.clone();
            let mut shutdown_rx_clone = shutdown_rx.resubscribe();

            tokio::spawn(async move {
                let mut interval = tokio::time::interval(heartbeat_interval);

                loop {
                    tokio::select! {
                        _ = shutdown_rx_clone.recv() => break,
                        _ = interval.tick() => {
                            // Check if we're still connected
                            {
                                let current_state = state.read().await;
                                if !matches!(*current_state, SessionState::Connected) {
                                    break;
                                }
                            }

                            // Send ping
                            let ping_result = timeout(heartbeat_timeout, async {
                                let client_guard = client.lock().await;
                                client_guard.ping().await
                            }).await;

                            if ping_result.is_err() {
                                // Heartbeat failed, mark as disconnected
                                let _ = state_tx.send(SessionState::Disconnected);
                                break;
                            }
                        }
                    }
                }
            });
        }

        Ok(())
    }

    /// Stop background tasks
    async fn stop_background_tasks(&self) {
        let shutdown_tx = {
            let mut shutdown_guard = self.shutdown_tx.lock().await;
            shutdown_guard.take()
        };

        if let Some(tx) = shutdown_tx {
            let _ = tx.send(()).await; // Ignore error if receiver is dropped
        }
    }

    /// Transition to a new state
    async fn transition_state(&self, new_state: SessionState) -> McpResult<()> {
        {
            let mut state = self.state.write().await;
            *state = new_state.clone();
        }

        // Broadcast the state change
        if self.state_tx.send(new_state).is_err() {
            // Receiver may have been dropped, which is okay
        }

        Ok(())
    }
}

/// Default notification handler that logs notifications
pub struct LoggingNotificationHandler;

impl NotificationHandler for LoggingNotificationHandler {
    fn handle_notification(&self, notification: JsonRpcNotification) {
        tracing::info!(
            "Received notification: {} {:?}",
            notification.method,
            notification.params
        );
    }
}

/// Resource update notification handler
pub struct ResourceUpdateHandler {
    callback: Box<dyn Fn(String) + Send + Sync>,
}

impl ResourceUpdateHandler {
    /// Create a new resource update handler
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(String) + Send + Sync + 'static,
    {
        Self {
            callback: Box::new(callback),
        }
    }
}

impl NotificationHandler for ResourceUpdateHandler {
    fn handle_notification(&self, notification: JsonRpcNotification) {
        if notification.method == methods::RESOURCES_UPDATED {
            if let Some(params) = notification.params {
                if let Ok(update_params) = serde_json::from_value::<ResourceUpdatedParams>(params) {
                    (self.callback)(update_params.uri);
                }
            }
        }
    }
}

/// Tool list changed notification handler
pub struct ToolListChangedHandler {
    callback: Box<dyn Fn() + Send + Sync>,
}

impl ToolListChangedHandler {
    /// Create a new tool list changed handler
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        Self {
            callback: Box::new(callback),
        }
    }
}

impl NotificationHandler for ToolListChangedHandler {
    fn handle_notification(&self, notification: JsonRpcNotification) {
        if notification.method == methods::TOOLS_LIST_CHANGED {
            (self.callback)();
        }
    }
}

/// Progress notification handler
pub struct ProgressHandler {
    callback: Box<dyn Fn(String, f32, Option<u32>) + Send + Sync>,
}

impl ProgressHandler {
    /// Create a new progress handler
    pub fn new<F>(callback: F) -> Self
    where
        F: Fn(String, f32, Option<u32>) + Send + Sync + 'static,
    {
        Self {
            callback: Box::new(callback),
        }
    }
}

impl NotificationHandler for ProgressHandler {
    fn handle_notification(&self, notification: JsonRpcNotification) {
        if notification.method == methods::PROGRESS {
            if let Some(params) = notification.params {
                if let Ok(progress_params) = serde_json::from_value::<ProgressParams>(params) {
                    (self.callback)(
                        progress_params.progress_token.to_string(),
                        progress_params.progress as f32,
                        progress_params.total,
                    );
                }
            }
        }
    }
}

/// Session statistics
#[derive(Debug, Clone)]
pub struct SessionStats {
    /// Current session state
    pub state: SessionState,
    /// Connection uptime
    pub uptime: Option<Duration>,
    /// Number of reconnection attempts
    pub reconnect_attempts: u32,
    /// Connection timestamp
    pub connected_at: Option<Instant>,
}

impl ClientSession {
    /// Get session statistics
    pub async fn stats(&self) -> SessionStats {
        let state = self.state().await;
        let uptime = self.uptime().await;
        let reconnect_attempts = {
            let attempts = self.reconnect_attempts.lock().await;
            *attempts
        };
        let connected_at = {
            let connected_at = self.connected_at.read().await;
            *connected_at
        };

        SessionStats {
            state,
            uptime,
            reconnect_attempts,
            connected_at,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::mcp_client::McpClient;
    use async_trait::async_trait;

    // Mock transport for testing
    struct MockTransport;

    #[async_trait]
    impl Transport for MockTransport {
        async fn send_request(&mut self, _request: JsonRpcRequest) -> McpResult<JsonRpcResponse> {
            // Return a successful initialize response
            let init_result = InitializeResult::new(
                ServerInfo {
                    name: "test-server".to_string(),
                    version: "1.0.0".to_string(),
                },
                ServerCapabilities::default(),
                Some("MCP client session for 2025-03-26".to_string()),
            );
            JsonRpcResponse::success(serde_json::Value::from(1), init_result)
                .map_err(|e| McpError::Serialization(e.to_string()))
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
    async fn test_session_creation() {
        let client = McpClient::new("test-client".to_string(), "1.0.0".to_string());
        let session = ClientSession::new(client);

        assert_eq!(session.state().await, SessionState::Disconnected);
        assert!(!session.is_connected().await);
        assert!(session.uptime().await.is_none());
    }

    #[tokio::test]
    async fn test_session_connection() {
        let client = McpClient::new("test-client".to_string(), "1.0.0".to_string());
        let session = ClientSession::new(client);

        let transport = MockTransport;
        let result = session.connect(transport).await;

        assert!(result.is_ok());
        assert_eq!(session.state().await, SessionState::Connected);
        assert!(session.is_connected().await);
        assert!(session.uptime().await.is_some());
    }

    #[tokio::test]
    async fn test_session_disconnect() {
        let client = McpClient::new("test-client".to_string(), "1.0.0".to_string());
        let session = ClientSession::new(client);

        // Connect first
        let transport = MockTransport;
        session.connect(transport).await.unwrap();
        assert!(session.is_connected().await);

        // Then disconnect
        session.disconnect().await.unwrap();
        assert_eq!(session.state().await, SessionState::Disconnected);
        assert!(!session.is_connected().await);
        assert!(session.uptime().await.is_none());
    }

    #[tokio::test]
    async fn test_notification_handlers() {
        let client = McpClient::new("test-client".to_string(), "1.0.0".to_string());
        let session = ClientSession::new(client);

        // Add a logging notification handler
        session
            .add_notification_handler(LoggingNotificationHandler)
            .await;

        // Add a resource update handler
        session
            .add_notification_handler(ResourceUpdateHandler::new(|uri| {
                println!("Resource updated: {}", uri);
            }))
            .await;

        // Add a tool list changed handler
        session
            .add_notification_handler(ToolListChangedHandler::new(|| {
                println!("Tool list changed");
            }))
            .await;

        // Add a progress handler
        session
            .add_notification_handler(ProgressHandler::new(|token, progress, total| {
                println!("Progress {}: {} / {:?}", token, progress, total);
            }))
            .await;

        let handlers = session.notification_handlers.read().await;
        assert_eq!(handlers.len(), 4);
    }

    #[tokio::test]
    async fn test_session_stats() {
        let client = McpClient::new("test-client".to_string(), "1.0.0".to_string());
        let session = ClientSession::new(client);

        let stats = session.stats().await;
        assert_eq!(stats.state, SessionState::Disconnected);
        assert!(stats.uptime.is_none());
        assert_eq!(stats.reconnect_attempts, 0);
        assert!(stats.connected_at.is_none());
    }

    #[tokio::test]
    async fn test_session_config() {
        let client = McpClient::new("test-client".to_string(), "1.0.0".to_string());
        let config = SessionConfig {
            auto_reconnect: false,
            max_reconnect_attempts: 10,
            reconnect_delay_ms: 2000,
            ..Default::default()
        };
        let session = ClientSession::with_config(client, config.clone());

        assert!(!session.config().auto_reconnect);
        assert_eq!(session.config().max_reconnect_attempts, 10);
        assert_eq!(session.config().reconnect_delay_ms, 2000);
    }

    #[tokio::test]
    async fn test_state_subscription() {
        let client = McpClient::new("test-client".to_string(), "1.0.0".to_string());
        let session = ClientSession::new(client);

        let mut state_rx = session.subscribe_state_changes();

        // Initial state
        assert_eq!(*state_rx.borrow(), SessionState::Disconnected);

        // Change state
        session
            .transition_state(SessionState::Connecting)
            .await
            .unwrap();

        // Wait for change
        state_rx.changed().await.unwrap();
        assert_eq!(*state_rx.borrow(), SessionState::Connecting);
    }
}
