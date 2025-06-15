//! Transport layer implementations
//!
//! This module provides concrete implementations of the transport traits
//! for different communication protocols including STDIO, HTTP, and WebSocket.

pub mod traits;

#[cfg(feature = "stdio")]
pub mod stdio;

#[cfg(feature = "http")]
pub mod http;

#[cfg(feature = "websocket")]
pub mod websocket;

// Re-export commonly used types
pub use traits::{
    ConnectionState, EventEmittingTransport, FilterableTransport, ReconnectConfig,
    ReconnectableTransport, ServerTransport, Transport, TransportConfig, TransportEvent,
    TransportStats,
};

// Re-export transport implementations when features are enabled
#[cfg(feature = "stdio")]
pub use stdio::{StdioClientTransport, StdioServerTransport};

#[cfg(feature = "http")]
pub use http::{HttpClientTransport, HttpServerTransport};

#[cfg(feature = "websocket")]
pub use websocket::{WebSocketClientTransport, WebSocketServerTransport};
