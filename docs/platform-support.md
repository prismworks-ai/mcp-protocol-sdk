# Multi-Platform Support - MCP Protocol SDK

The MCP Protocol SDK provides comprehensive multi-platform support across all major operating systems and architectures, ensuring your MCP servers and clients work seamlessly regardless of the deployment environment.

## 🌍 Platform Support Overview

### Primary Platforms (Tier 1)

| Platform | Architecture | Support Level | Testing | CI Status |
|----------|-------------|---------------|---------|-----------||
| **Linux** | x86_64 | ✅ Full | Automated | ✅ Ubuntu Latest |
| **macOS** | x86_64 (Intel) | ✅ Full | Automated | ✅ macOS Latest |
| **macOS** | aarch64 (Apple Silicon) | ✅ Full | Cross-compile | ✅ Automated |
| **Windows** | x86_64 | ✅ Full | Automated | ✅ Windows Latest |

### Additional Targets (Tier 2)

| Platform | Architecture | Support Level | Testing | Availability |
|----------|-------------|---------------|---------|--------------||
| **Linux** | aarch64 (ARM64) | ✅ Full | Cross-compile | ✅ Production Ready |
| **Linux** | x86_64 (musl) | ✅ Full | Cross-compile | ✅ Static Linking |
| **Windows** | x86_64 (GNU) | ✅ Full | Cross-compile | ✅ MinGW Compatible |

## 🛠️ Build & Test Matrix

Our comprehensive CI/CD pipeline ensures reliability across all supported platforms:

### GitHub Actions Test Matrix

```yaml
# Primary platform testing
matrix:
  os: [ubuntu-latest, windows-latest, macos-latest]
  rust: [stable, beta, nightly, "1.85"]
  
# Cross-compilation targets
targets:
  - x86_64-pc-windows-gnu      # Windows (GNU)
  - aarch64-apple-darwin       # macOS (Apple Silicon)
  - x86_64-unknown-linux-musl  # Linux (Static)
  - aarch64-unknown-linux-gnu  # Linux (ARM64)
```

### Rust Version Support

| Version | Support Level | Purpose |
|---------|--------------|---------||
| **1.85+** | ✅ Required | Minimum Supported Rust Version (MSRV) |
| **Stable** | ✅ Primary | Recommended for production |
| **Beta** | ✅ Testing | Early compatibility testing |
| **Nightly** | ✅ Advanced | Latest features and optimizations |

## 📦 Installation by Platform

### Linux

#### Ubuntu/Debian
```bash
# Install Rust and dependencies
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt-get update
sudo apt-get install -y pkg-config libssl-dev

# Add to Cargo.toml
[dependencies]
mcp-protocol-sdk = "0.5.0"
```

#### CentOS/RHEL/Fedora
```bash
# Install Rust and dependencies
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo yum install -y pkgconf-devel openssl-devel  # CentOS/RHEL
# OR
sudo dnf install -y pkgconf-devel openssl-devel  # Fedora

# Add to Cargo.toml
[dependencies]
mcp-protocol-sdk = "0.5.0"
```

#### Alpine Linux (musl)
```bash
# Install Rust and dependencies
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
apk add --no-cache pkgconf openssl-dev musl-dev

# Use musl target for static linking
rustup target add x86_64-unknown-linux-musl
cargo build --target x86_64-unknown-linux-musl
```

### macOS

#### Intel Macs (x86_64)
```bash
# Install Rust via rustup
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Or via Homebrew
brew install rust

# Add to Cargo.toml
[dependencies]
mcp-protocol-sdk = "0.5.0"
```

#### Apple Silicon Macs (aarch64)
```bash
# Install Rust (automatically detects Apple Silicon)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Verify native compilation
rustc --print target-list | grep aarch64-apple-darwin

# Add to Cargo.toml
[dependencies]
mcp-protocol-sdk = "0.5.0"
```

### Windows

#### Windows 10/11 (x86_64)
```powershell
# Install Rust via rustup
# Download and run: https://rustup.rs/

# Or via Chocolatey
choco install rust

# Or via Scoop
scoop install rust

# Add to Cargo.toml
[dependencies]
mcp-protocol-sdk = "0.5.0"
```

#### Windows with WSL2
```bash
# Install Rust in WSL2 environment
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
sudo apt-get update
sudo apt-get install -y pkg-config libssl-dev

# Cross-compile for Windows from WSL2
rustup target add x86_64-pc-windows-gnu
cargo build --target x86_64-pc-windows-gnu
```

## 🔧 Cross-Compilation Guide

### Compiling for Different Targets

```bash
# Add target architectures
rustup target add aarch64-apple-darwin          # macOS Apple Silicon
rustup target add x86_64-pc-windows-gnu         # Windows GNU
rustup target add x86_64-unknown-linux-musl     # Linux static
rustup target add aarch64-unknown-linux-gnu     # Linux ARM64

# Cross-compile examples
cargo build --target aarch64-apple-darwin       # For Apple Silicon
cargo build --target x86_64-pc-windows-gnu      # For Windows
cargo build --target x86_64-unknown-linux-musl  # For static Linux
cargo build --target aarch64-unknown-linux-gnu  # For ARM64 Linux
```

### Docker Multi-Architecture

```dockerfile
# Multi-stage build for multiple architectures
FROM --platform=$BUILDPLATFORM rust:1.85 AS builder

ARG TARGETPLATFORM
ARG BUILDPLATFORM

WORKDIR /app
COPY . .

# Install cross-compilation tools
RUN case "$TARGETPLATFORM" in \
    "linux/amd64") TARGET=x86_64-unknown-linux-musl ;; \
    "linux/arm64") TARGET=aarch64-unknown-linux-gnu ;; \
    *) echo "Unsupported platform: $TARGETPLATFORM" && exit 1 ;; \
    esac && \
    rustup target add $TARGET && \
    cargo build --release --target $TARGET

# Runtime stage
FROM alpine:latest
RUN apk --no-cache add ca-certificates
COPY --from=builder /app/target/*/release/mcp-server /usr/local/bin/
CMD ["mcp-server"]
```

## 🚀 Platform-Specific Features

### Feature Matrix by Platform

| Feature | Linux | macOS | Windows | Notes |
|---------|-------|-------|---------|-------|
| **STDIO Transport** | ✅ | ✅ | ✅ | Core functionality |
| **HTTP Transport** | ✅ | ✅ | ✅ | All async features |
| **WebSocket Transport** | ✅ | ✅ | ✅ | Real-time communication |
| **Process Management** | ✅ | ✅ | ✅ | Via tokio::process |
| **File System Access** | ✅ | ✅ | ✅ | Platform-specific paths |
| **TLS/SSL** | ✅ | ✅ | ✅ | OpenSSL/native TLS |
| **Unix Sockets** | ✅ | ✅ | ❌ | Unix-like systems only |
| **Named Pipes** | ❌ | ❌ | ✅ | Windows-specific |

### Platform-Specific Configuration

#### Linux-Specific
```rust
// Use systemd integration
[target.'cfg(target_os = "linux")'.dependencies]
systemd = "0.10"

// Example: Linux-specific file paths
#[cfg(target_os = "linux")]
fn get_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("/etc"))
        .join("mcp-server")
}
```

#### macOS-Specific
```rust
// Use macOS frameworks
[target.'cfg(target_os = "macos")'.dependencies]
core-foundation = "0.9"

// Example: macOS-specific paths
#[cfg(target_os = "macos")]
fn get_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("/Library/Application Support"))
        .join("MCP Server")
}
```

#### Windows-Specific
```rust
// Use Windows APIs
[target.'cfg(target_os = "windows")'.dependencies]
windows = { version = "0.52", features = ["Win32_Foundation"] }

// Example: Windows-specific paths
#[cfg(target_os = "windows")]
fn get_config_dir() -> PathBuf {
    dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from(r"C:\ProgramData"))
        .join("MCP Server")
}
```

## 📋 Testing & Validation

### Local Testing Commands

```bash
# Test on current platform
cargo test --all-features

# Test specific features
cargo test --features stdio
cargo test --features http
cargo test --features websocket

# Test cross-compilation (no execution)
cargo check --target aarch64-apple-darwin
cargo check --target x86_64-pc-windows-gnu
cargo check --target x86_64-unknown-linux-musl

# Test examples
cargo run --example echo_server --features stdio,tracing-subscriber
cargo run --example http_server --features http
cargo run --example websocket_server --features websocket
```

### CI/CD Platform Validation

Our GitHub Actions workflow automatically tests:

1. **Native Compilation**: Direct compilation on Ubuntu, macOS, and Windows
2. **Cross-Compilation**: Building for all supported target architectures
3. **Feature Testing**: Each transport and feature combination
4. **Example Validation**: All examples compile and basic functionality works
5. **Documentation**: Platform-specific docs build correctly

## 🐛 Platform-Specific Troubleshooting

### Linux Issues

**OpenSSL linking errors:**
```bash
# Ubuntu/Debian
sudo apt-get install pkg-config libssl-dev

# CentOS/RHEL
sudo yum install pkgconf-devel openssl-devel

# Alpine
apk add pkgconf openssl-dev
```

**Permission issues with STDIO:**
```bash
# Ensure proper permissions for Claude Desktop
chmod +x your-mcp-server
```

### macOS Issues

**Apple Silicon compatibility:**
```bash
# Verify native ARM64 compilation
file target/release/your-mcp-server
# Should show: Mach-O 64-bit executable arm64

# Force Intel compatibility if needed
arch -x86_64 cargo build
```

**Security/Gatekeeper issues:**
```bash
# Sign your binary for distribution
codesign -s "Developer ID Application" target/release/your-mcp-server

# Or allow unsigned binaries locally
spctl --add target/release/your-mcp-server
```

### Windows Issues

**MSVC vs GNU toolchain:**
```powershell
# Install MSVC toolchain (recommended)
rustup toolchain install stable-x86_64-pc-windows-msvc
rustup default stable-x86_64-pc-windows-msvc

# Or use GNU toolchain
rustup toolchain install stable-x86_64-pc-windows-gnu
```

**PowerShell execution policy:**
```powershell
# Allow script execution
Set-ExecutionPolicy -ExecutionPolicy RemoteSigned -Scope CurrentUser
```

## 📊 Performance Characteristics

### Platform Performance Comparison

| Platform | Compilation Speed | Runtime Performance | Binary Size | Memory Usage |
|----------|------------------|-------------------|-------------|--------------||
| **Linux** | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ |
| **macOS** | ⭐⭐⭐⭐ | ⭐⭐⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐⭐ |
| **Windows** | ⭐⭐⭐ | ⭐⭐⭐⭐ | ⭐⭐⭐ | ⭐⭐⭐⭐ |

### Optimization Tips by Platform

#### Linux Optimizations
```toml
# Use musl for smaller, static binaries
[profile.release]
strip = true
lto = true

# For minimal deployments
[dependencies]
mcp-protocol-sdk = { version = "0.5.0", default-features = false, features = ["stdio"] }
```

#### macOS Optimizations
```bash
# Universal binary for both Intel and Apple Silicon
cargo install cargo-lipo
rustup target add x86_64-apple-darwin
rustup target add aarch64-apple-darwin
cargo lipo --release
```

#### Windows Optimizations
```toml
# Reduce Windows binary size
[profile.release]
strip = true
panic = "abort"
codegen-units = 1
```

## 🌐 Deployment Scenarios

### Container Deployment

```yaml
# Docker Compose multi-architecture
version: '3.8'
services:
  mcp-server:
    platform: linux/amd64  # or linux/arm64
    image: your-registry/mcp-server:latest
    ports:
      - "3000:3000"
```

### Cloud Platform Support

| Platform | Architecture | Support | Notes |
|----------|-------------|---------|-------|
| **AWS Lambda** | x86_64, ARM64 | ✅ | Use lambda-runtime |
| **Google Cloud Run** | x86_64, ARM64 | ✅ | Container deployment |
| **Azure Container Instances** | x86_64 | ✅ | Standard deployment |
| **Kubernetes** | Multi-arch | ✅ | Platform-specific node selection |

### Edge Computing

```rust
// Example: Edge-optimized build
[profile.edge]
inherits = "release"
strip = true
panic = "abort"
codegen-units = 1
opt-level = "z"  // Optimize for size
```

## 📚 Integration Examples

### Claude Desktop Integration

Platform-specific configuration paths:

```json
// macOS: ~/Library/Application Support/Claude/claude_desktop_config.json
// Windows: %APPDATA%\Claude\claude_desktop_config.json  
// Linux: ~/.config/Claude/claude_desktop_config.json

{
  "mcpServers": {
    "your-server": {
      "command": "/path/to/your-mcp-server",
      "args": ["--config", "/path/to/config.json"],
      "env": {
        "RUST_LOG": "info"
      }
    }
  }
}
```

### Cross-Platform Server Example

```rust
use mcp_protocol_sdk::prelude::*;
use std::path::PathBuf;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Platform-specific configuration
    let config_dir = get_platform_config_dir();
    let config_path = config_dir.join("server.json");
    
    // Create server with platform detection
    let mut server = McpServer::new("multi-platform-server", "1.0.0");
    
    // Add platform-specific tools
    add_platform_tools(&mut server).await?;
    
    // Start with appropriate transport
    let transport = StdioServerTransport::new();
    server.run(transport).await?;
    
    Ok(())
}

fn get_platform_config_dir() -> PathBuf {
    #[cfg(target_os = "windows")]
    return dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from(r"C:\ProgramData"))
        .join("MCP Server");
    
    #[cfg(target_os = "macos")]
    return dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("/Library/Application Support"))
        .join("MCP Server");
    
    #[cfg(target_os = "linux")]
    return dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("/etc"))
        .join("mcp-server");
}

async fn add_platform_tools(server: &mut McpServer) -> Result<(), Box<dyn std::error::Error>> {
    // Cross-platform file operations
    server.add_tool(
        Tool::new("get_platform_info", "Get current platform information")
    );
    
    // Platform-specific tools
    #[cfg(target_os = "linux")]
    server.add_tool(
        Tool::new("systemd_status", "Check systemd service status")
    );
    
    #[cfg(target_os = "macos")]
    server.add_tool(
        Tool::new("launchctl_status", "Check launchd service status")
    );
    
    #[cfg(target_os = "windows")]
    server.add_tool(
        Tool::new("service_status", "Check Windows service status")
    );
    
    Ok(())
}
```

## 🔗 Related Documentation

- [Getting Started Guide](getting-started.md) - Platform-specific installation
- [Configuration Guide](configuration.md) - Platform-specific settings  
- [Examples Collection](examples.md) - Cross-platform examples
- [Integration Guides](integrations/) - Platform-specific integrations
- [GitHub Actions CI](../.github/workflows/ci.yml) - Full testing matrix

## 🤝 Contributing

When contributing platform-specific improvements:

1. **Test on Target Platform**: Ensure changes work on the intended platform
2. **Update CI Matrix**: Add new platforms to GitHub Actions if needed
3. **Document Platform Differences**: Update this guide with any platform-specific requirements
4. **Cross-Platform Testing**: Verify changes don't break other platforms

---

**Need Help?** Platform-specific issues can be reported in our [GitHub Issues](https://github.com/mcp-rust/mcp-protocol-sdk/issues) with the `platform:<os>` label.