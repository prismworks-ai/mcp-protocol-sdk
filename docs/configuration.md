# 🔧 Configuration Guide

Configure the MCP Protocol SDK for optimal performance and functionality in your environment.

## Feature Flags

The SDK uses Cargo features to enable optional functionality. Choose only what you need:

### Available Features

| Feature | Description | Default |
|---------|-------------|---------|
| `stdio` | STDIO transport support | ✅ Yes |
| `http` | HTTP transport support | ✅ Yes |
| `websocket` | WebSocket transport support | ✅ Yes |
| `validation` | Enhanced validation | ✅ Yes |
| `tracing-subscriber` | Built-in logging | ❌ No |

### Minimal Configuration

For the smallest binary size, disable default features:

```toml
[dependencies]
mcp-protocol-sdk = { 
    version = "0.1.0", 
    default-features = false,
    features = ["stdio"]  # Only what you need
}
```

### Server-Only Configuration

```toml
[dependencies]
mcp-protocol-sdk = { 
    version = "0.1.0", 
    features = ["stdio", "validation", "tracing-subscriber"]
}
```

### Client-Only Configuration

```toml
[dependencies]
mcp-protocol-sdk = { 
    version = "0.1.0", 
    features = ["http", "websocket"]
}
```

### Full-Featured Configuration

```toml
[dependencies]
mcp-protocol-sdk = "0.1.0"  # All features enabled
```

## Environment Configuration

### Logging

Configure logging using the `RUST_LOG` environment variable:

```bash
# Basic info logging
export RUST_LOG=info

# Debug level for development
export RUST_LOG=debug

# Specific module logging
export RUST_LOG=mcp_protocol_sdk=debug

# Multiple modules
export RUST_LOG=mcp_protocol_sdk=debug,tokio=info
```

In your code:

```rust
use tracing::{info, debug, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging (with tracing-subscriber feature)
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();
    
    info!("Starting MCP server");
    
    // Your server code...
    Ok(())
}
```

### Transport Configuration

#### STDIO Transport

```rust
use mcp_protocol_sdk::transport::StdioServerTransport;

let transport = StdioServerTransport::new()
    .with_buffer_size(8192)
    .with_timeout(Duration::from_secs(30));
```

#### HTTP Transport

```rust
use mcp_protocol_sdk::transport::HttpServerTransport;

let transport = HttpServerTransport::new("0.0.0.0:3000")
    .with_cors_enabled(true)
    .with_timeout(Duration::from_secs(60))
    .with_max_connections(100)
    .with_keep_alive(true);
```

#### WebSocket Transport

```rust
use mcp_protocol_sdk::transport::WebSocketServerTransport;

let transport = WebSocketServerTransport::new("0.0.0.0:8080")
    .with_heartbeat_interval(Duration::from_secs(30))
    .with_max_connections(50)
    .with_compression_enabled(true);
```

## Server Configuration

### Basic Server Setup

```rust
use mcp_protocol_sdk::prelude::*;

let server = McpServer::new("my-server", "1.0.0")
    .with_description("My awesome MCP server")
    .with_timeout(Duration::from_secs(30))
    .with_max_concurrent_requests(10)
    .with_request_size_limit(1024 * 1024); // 1MB
```

### Advanced Server Configuration

```rust
let server = McpServer::builder()
    .name("advanced-server")
    .version("2.0.0")
    .description("Advanced MCP server with custom configuration")
    .timeout(Duration::from_secs(60))
    .max_concurrent_requests(100)
    .request_size_limit(10 * 1024 * 1024) // 10MB
    .enable_compression(true)
    .enable_heartbeat(Duration::from_secs(30))
    .build();
```

## Client Configuration

### Basic Client Setup

```rust
use mcp_protocol_sdk::prelude::*;

let client = McpClient::new()
    .with_name("my-client")
    .with_version("1.0.0")
    .with_timeout(Duration::from_secs(30))
    .build();
```

### Advanced Client Configuration

```rust
let client = McpClient::builder()
    .name("advanced-client")
    .version("2.0.0")
    .timeout(Duration::from_secs(60))
    .retry_config(RetryConfig {
        max_retries: 3,
        initial_delay: Duration::from_millis(100),
        max_delay: Duration::from_secs(5),
        backoff_multiplier: 2.0,
    })
    .connection_pool_size(5)
    .enable_compression(true)
    .build();
```

## Performance Tuning

### Memory Configuration

```rust
// Configure memory limits
std::env::set_var("TOKIO_WORKER_THREADS", "4");

let server = McpServer::new("server", "1.0.0")
    .with_memory_limit(256 * 1024 * 1024) // 256MB
    .with_request_buffer_size(64 * 1024); // 64KB
```

### Concurrency Configuration

```rust
// Limit concurrent operations
let server = McpServer::new("server", "1.0.0")
    .with_max_concurrent_requests(50)
    .with_max_concurrent_tools(10)
    .with_tool_execution_timeout(Duration::from_secs(30));
```

### Connection Pool Configuration

```rust
// For HTTP clients
let client = McpClient::new()
    .with_connection_pool_config(ConnectionPoolConfig {
        max_connections: 10,
        idle_timeout: Duration::from_secs(30),
        connection_timeout: Duration::from_secs(10),
        keep_alive: true,
    })
    .build();
```

## Security Configuration

### TLS Configuration

```rust
use mcp_protocol_sdk::transport::HttpServerTransport;

let transport = HttpServerTransport::new("0.0.0.0:443")
    .with_tls_config(TlsConfig {
        cert_path: "/path/to/cert.pem",
        key_path: "/path/to/private.key",
        ca_cert_path: Some("/path/to/ca.pem"), // For client cert validation
    });
```

### Authentication Configuration

```rust
let server = McpServer::new("secure-server", "1.0.0")
    .with_auth_config(AuthConfig {
        required: true,
        method: AuthMethod::ApiKey,
        api_keys: vec!["your-api-key".to_string()],
        jwt_secret: Some("your-jwt-secret".to_string()),
    });
```

### Rate Limiting

```rust
let server = McpServer::new("rate-limited-server", "1.0.0")
    .with_rate_limit_config(RateLimitConfig {
        requests_per_minute: 60,
        burst_size: 10,
        per_client: true,
    });
```

## Configuration Files

### TOML Configuration

Create `mcp-config.toml`:

```toml
[server]
name = "my-server"
version = "1.0.0"
description = "My MCP server"
timeout = "30s"
max_concurrent_requests = 50

[transport]
type = "http"
host = "0.0.0.0"
port = 3000
cors_enabled = true

[logging]
level = "info"
format = "json"
file = "/var/log/mcp-server.log"

[security]
auth_required = true
api_keys = ["key1", "key2"]
tls_enabled = false

[performance]
memory_limit = "256MB"
connection_pool_size = 10
```

Load configuration:

```rust
use serde::Deserialize;

#[derive(Deserialize)]
struct Config {
    server: ServerConfig,
    transport: TransportConfig,
    logging: LoggingConfig,
    security: SecurityConfig,
    performance: PerformanceConfig,
}

fn load_config() -> Result<Config, Box<dyn std::error::Error>> {
    let config_str = std::fs::read_to_string("mcp-config.toml")?;
    let config: Config = toml::from_str(&config_str)?;
    Ok(config)
}
```

### JSON Configuration

```json
{
  "server": {
    "name": "my-server",
    "version": "1.0.0",
    "description": "My MCP server",
    "timeout": "30s",
    "maxConcurrentRequests": 50
  },
  "transport": {
    "type": "websocket",
    "host": "0.0.0.0",
    "port": 8080,
    "heartbeatInterval": "30s"
  },
  "logging": {
    "level": "info",
    "format": "json"
  }
}
```

### Environment-Based Configuration

```rust
use std::env;

fn get_config_from_env() -> ServerConfig {
    ServerConfig {
        name: env::var("MCP_SERVER_NAME").unwrap_or_else(|_| "default-server".to_string()),
        version: env::var("MCP_SERVER_VERSION").unwrap_or_else(|_| "1.0.0".to_string()),
        host: env::var("MCP_HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
        port: env::var("MCP_PORT")
            .unwrap_or_else(|_| "3000".to_string())
            .parse()
            .unwrap_or(3000),
        log_level: env::var("RUST_LOG").unwrap_or_else(|_| "info".to_string()),
    }
}
```

## Development vs Production

### Development Configuration

```rust
#[cfg(debug_assertions)]
fn create_dev_server() -> McpServer {
    McpServer::new("dev-server", "dev")
        .with_timeout(Duration::from_secs(300)) // Longer timeout for debugging
        .with_detailed_errors(true)
        .with_cors_permissive(true)
        .with_request_logging(true)
}
```

### Production Configuration

```rust
#[cfg(not(debug_assertions))]
fn create_prod_server() -> McpServer {
    McpServer::new("prod-server", "1.0.0")
        .with_timeout(Duration::from_secs(30))
        .with_detailed_errors(false) // Don't leak internal details
        .with_security_headers(true)
        .with_rate_limiting(true)
        .with_monitoring_enabled(true)
}
```

## Platform-Specific Configuration

### Linux/Unix Configuration

```rust
#[cfg(unix)]
fn unix_config() -> TransportConfig {
    TransportConfig::Unix {
        socket_path: "/tmp/mcp-server.sock",
        permissions: 0o600,
    }
}
```

### Windows Configuration

```rust
#[cfg(windows)]
fn windows_config() -> TransportConfig {
    TransportConfig::NamedPipe {
        pipe_name: r"\\.\pipe\mcp-server",
    }
}
```

### macOS Configuration

```rust
#[cfg(target_os = "macos")]
fn macos_config() -> ServerConfig {
    ServerConfig {
        // macOS-specific optimizations
        use_kqueue: true,
        launchd_integration: true,
    }
}
```

## Monitoring Configuration

### Metrics Configuration

```rust
let server = McpServer::new("monitored-server", "1.0.0")
    .with_metrics_config(MetricsConfig {
        enabled: true,
        endpoint: "/metrics",
        format: MetricsFormat::Prometheus,
        include_system_metrics: true,
    });
```

### Health Check Configuration

```rust
let server = McpServer::new("server", "1.0.0")
    .with_health_check_config(HealthCheckConfig {
        endpoint: "/health",
        check_database: true,
        check_external_services: true,
        timeout: Duration::from_secs(5),
    });
```

## Troubleshooting Configuration

### Debug Configuration

```rust
let server = McpServer::new("debug-server", "1.0.0")
    .with_debug_config(DebugConfig {
        log_requests: true,
        log_responses: true,
        include_stack_traces: true,
        dump_protocol_messages: true,
    });
```

### Profiling Configuration

```rust
#[cfg(feature = "profiling")]
let server = McpServer::new("profiled-server", "1.0.0")
    .with_profiling_config(ProfilingConfig {
        cpu_profiling: true,
        memory_profiling: true,
        output_dir: "/tmp/mcp-profiles",
    });
```

## Best Practices

1. **Use feature flags** to minimize binary size
2. **Configure timeouts** appropriately for your use case
3. **Enable logging** in development, optimize in production
4. **Use environment variables** for deployment-specific config
5. **Implement health checks** for production deployments
6. **Configure rate limiting** to prevent abuse
7. **Enable TLS** for production HTTP/WebSocket transports
8. **Monitor performance** with metrics collection
9. **Test configurations** in staging environments
10. **Document configuration** options for your team

Your MCP server is now properly configured for any environment! 🚀
