# 🚀 PUBLISHING TO CRATES.IO

**Step-by-step guide to publish the MCP Protocol SDK**

## ⚡ Quick Publish (After Setup)

```bash
# 1. Verify everything is ready
cargo test --all-features
cargo clippy --all-features -- -D warnings
cargo doc --all-features --no-deps

# 2. Update version if needed
# Edit Cargo.toml version field

# 3. Publish to crates.io
cargo publish --all-features

# 4. Create GitHub release
git tag v0.1.0
git push origin v0.1.0
gh release create v0.1.0 --generate-notes
```

## 📋 Complete Pre-Publication Checklist

### ✅ 1. Code Quality
```bash
# Run full test suite
cargo test --all-features

# Check for warnings
cargo clippy --all-features -- -D warnings

# Format code
cargo fmt --check

# Security audit
cargo audit

# Check minimal features
cargo check --no-default-features
cargo check --no-default-features --features stdio
```

### ✅ 2. Documentation
```bash
# Generate docs (should work without warnings)
cargo doc --all-features --no-deps

# Test examples
cargo run --example echo_server --features stdio,tracing-subscriber
cargo run --example http_server --features http
cargo run --example websocket_server --features websocket

# Verify README compiles
cargo test --doc
```

### ✅ 3. Metadata Verification
- [ ] **Version**: Set to appropriate version (0.1.0 for initial)
- [ ] **Description**: Clear, under 200 characters
- [ ] **Keywords**: 5 relevant keywords maximum
- [ ] **Categories**: Appropriate crates.io categories
- [ ] **License**: MIT license file present
- [ ] **Repository**: Correct GitHub URL
- [ ] **Homepage**: GitHub Pages URL
- [ ] **Authors**: Correct author information

### ✅ 4. GitHub Repository
- [ ] **Repository**: Transferred to `mcp-rust` organization
- [ ] **GitHub Pages**: Live at https://mcp-rust.github.io/mcp-protocol-sdk/
- [ ] **Workflows**: All GitHub Actions passing
- [ ] **README**: Links to GitHub Pages and docs.rs
- [ ] **License**: MIT license in repository root

## 🔧 Crates.io Setup (One-time)

### 1. Account Setup
```bash
# Create account at https://crates.io/
# Get API token from https://crates.io/me

# Login (will prompt for token)
cargo login
```

### 2. Verify Cargo.toml
```toml
[package]
name = "mcp-protocol-sdk"
version = "0.1.0"
edition = "2021"
rust-version = "1.75"
authors = ["MCP Rust Contributors <mcp-rust@users.noreply.github.com>"]
description = "Production-ready Rust SDK for the Model Context Protocol (MCP) with multiple transport support"
license = "MIT"
repository = "https://github.com/mcp-rust/mcp-protocol-sdk"
keywords = ["mcp", "ai", "protocol", "jsonrpc", "llm"]
categories = ["api-bindings", "network-programming", "development-tools", "asynchronous"]
readme = "README.md"
homepage = "https://mcp-rust.github.io/mcp-protocol-sdk/"
documentation = "https://docs.rs/mcp-protocol-sdk"
```

## 📦 Publishing Process

### Step 1: Pre-publish Verification
```bash
# Dry run to check everything
cargo publish --dry-run --all-features

# This will:
# - Verify Cargo.toml metadata
# - Check that all files are included
# - Validate dependencies
# - Ensure package builds correctly
```

### Step 2: Publish to Crates.io
```bash
# Publish the real package
cargo publish --all-features

# Expected output:
# Updating crates.io index
# Packaging mcp-protocol-sdk v0.1.0
# Verifying mcp-protocol-sdk v0.1.0
# Compiling mcp-protocol-sdk v0.1.0
# Uploading mcp-protocol-sdk v0.1.0
```

### Step 3: Verify Publication
```bash
# Check on crates.io (may take a few minutes)
# https://crates.io/crates/mcp-protocol-sdk

# Test installation
cargo install mcp-protocol-sdk --all-features

# Check docs.rs generation (takes 5-10 minutes)
# https://docs.rs/mcp-protocol-sdk
```

### Step 4: Create GitHub Release
```bash
# Create and push tag
git tag v0.1.0
git push origin v0.1.0

# Create GitHub release
gh release create v0.1.0 \
  --title "v0.1.0 - Initial Public Release" \
  --notes "🎉 **Initial Release of MCP Protocol SDK**

## ✨ Features
- Complete MCP 2024-11-05 protocol implementation
- Multiple transport support (STDIO, HTTP, WebSocket)  
- Production-ready error handling and validation
- Comprehensive documentation and examples
- High-performance async/await architecture

## 📦 Installation
\`\`\`toml
[dependencies]
mcp-protocol-sdk = \"0.1.0\"
\`\`\`

## 📚 Documentation
- [Getting Started Guide](https://mcp-rust.github.io/mcp-protocol-sdk/getting-started.html)
- [API Reference](https://docs.rs/mcp-protocol-sdk)
- [Examples](https://mcp-rust.github.io/mcp-protocol-sdk/examples.html)

See the [full changelog](CHANGELOG.md) for complete details."
```

## 🔄 Future Releases

### Version Numbering
- **Patch (0.1.X)**: Bug fixes, documentation updates
- **Minor (0.X.0)**: New features, backward compatible
- **Major (X.0.0)**: Breaking changes

### Release Process
```bash
# 1. Update version in Cargo.toml
# 2. Update CHANGELOG.md
# 3. Commit and tag
git add Cargo.toml CHANGELOG.md
git commit -m "chore: Release v0.1.1"
git tag v0.1.1

# 4. Publish
cargo publish --all-features
git push origin main --tags

# 5. Create GitHub release
gh release create v0.1.1 --generate-notes
```

## ⚠️ Important Notes

### Crates.io Limitations
- **Immutable**: Once published, versions cannot be deleted (only yanked)
- **Size Limit**: 10MB package size maximum
- **Name**: Package name must be unique on crates.io
- **Keywords**: Maximum 5 keywords
- **Categories**: Must be from predefined list

### Best Practices
- **Test Thoroughly**: Use `cargo publish --dry-run` first
- **Semantic Versioning**: Follow semver for all releases
- **Documentation**: Ensure docs.rs builds successfully
- **Examples**: Test all examples before publishing
- **Dependencies**: Use conservative dependency versions

### Troubleshooting

#### Common Issues
```bash
# Issue: "package name already exists"
# Solution: Choose different name or contact crates.io team

# Issue: "documentation build failed"
# Solution: Fix doc comments and run cargo doc locally

# Issue: "package too large"
# Solution: Add files to exclude in Cargo.toml

# Issue: "invalid category"
# Solution: Use categories from https://crates.io/category_slugs
```

#### Recovery Commands
```bash
# Yank version (if broken)
cargo yank --vers 0.1.0 --undo  # to un-yank

# Check publish status
cargo search mcp-protocol-sdk

# Force rebuild docs.rs
# Visit https://docs.rs/crate/mcp-protocol-sdk/0.1.0
# Click "Rebuild"
```

## 🎯 Success Metrics

### Immediate (24 hours)
- [ ] Package appears on crates.io
- [ ] Documentation builds on docs.rs
- [ ] Examples can be run successfully
- [ ] GitHub release created
- [ ] No critical issues reported

### Short-term (1 week)
- [ ] Download count > 10
- [ ] GitHub stars > 5
- [ ] At least one external project tests it
- [ ] Documentation feedback incorporated

### Long-term (1 month)
- [ ] Download count > 100
- [ ] GitHub stars > 25  
- [ ] Active community engagement (issues, PRs)
- [ ] Positive feedback from users

---

**🚀 Ready to publish? Run the Quick Publish commands at the top!**
