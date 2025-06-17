//! Client builder for MCP clients
//!
//! Provides a builder pattern for creating and configuring MCP clients.

use crate::client::McpClient;
use crate::core::error::McpResult;
use crate::protocol::types::ClientCapabilities;
use std::time::Duration;

/// Configuration for retry behavior
#[derive(Debug, Clone)]
pub struct RetryConfig {
    /// Maximum number of retry attempts
    pub max_attempts: Option<u32>,
    /// Initial delay between retry attempts (ms)
    pub initial_delay_ms: u64,
    /// Maximum delay between retry attempts (ms)
    pub max_delay_ms: u64,
    /// Backoff multiplier for retry delays
    pub backoff_multiplier: f64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_attempts: Some(3),
            initial_delay_ms: 1000,
            max_delay_ms: 30000,
            backoff_multiplier: 2.0,
        }
    }
}

/// Configuration for connections
#[derive(Debug, Clone)]
pub struct ConnectionConfig {
    /// Connection timeout in milliseconds
    pub timeout_ms: u64,
    /// Whether to enable keep-alive
    pub keep_alive: bool,
    /// Whether to enable compression
    pub compression: bool,
}

impl Default for ConnectionConfig {
    fn default() -> Self {
        Self {
            timeout_ms: 30000,
            keep_alive: true,
            compression: false,
        }
    }
}

/// Builder for creating MCP clients with configuration
pub struct McpClientBuilder {
    name: Option<String>,
    version: Option<String>,
    capabilities: Option<ClientCapabilities>,
    timeout: Option<Duration>,
    retry_config: Option<RetryConfig>,
    connection_config: Option<ConnectionConfig>,
}

impl McpClientBuilder {
    /// Create a new client builder
    pub fn new() -> Self {
        Self {
            name: None,
            version: None,
            capabilities: None,
            timeout: None,
            retry_config: None,
            connection_config: None,
        }
    }

    /// Set client name
    pub fn with_name<S: Into<String>>(mut self, name: S) -> Self {
        self.name = Some(name.into());
        self
    }

    /// Set client version
    pub fn with_version<S: Into<String>>(mut self, version: S) -> Self {
        self.version = Some(version.into());
        self
    }

    /// Set client capabilities
    pub fn with_capabilities(mut self, capabilities: ClientCapabilities) -> Self {
        self.capabilities = Some(capabilities);
        self
    }

    /// Set request timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }

    /// Set retry configuration
    pub fn with_retry_config(mut self, retry_config: RetryConfig) -> Self {
        self.retry_config = Some(retry_config);
        self
    }

    /// Set connection configuration
    pub fn with_connection_config(mut self, connection_config: ConnectionConfig) -> Self {
        self.connection_config = Some(connection_config);
        self
    }

    /// Build the client
    pub fn build(self) -> McpResult<McpClient> {
        let mut client = McpClient::new(
            self.name.unwrap_or_else(|| "mcp-client".to_string()),
            self.version.unwrap_or_else(|| "1.0.0".to_string()),
        );

        client.set_capabilities(self.capabilities.unwrap_or_default());

        Ok(client)
    }
}

impl Default for McpClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

// Legacy alias for compatibility - single definition only
pub type ClientBuilder = McpClientBuilder;
