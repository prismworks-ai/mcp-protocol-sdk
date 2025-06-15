# Contributing to MCP Protocol SDK

Thank you for your interest in contributing to the MCP Protocol SDK! This document provides guidelines and information for contributors.

## 🎯 Ways to Contribute

### 1. Bug Reports
- Use GitHub Issues to report bugs
- Include minimal reproduction steps
- Provide system information (OS, Rust version)
- Include relevant error messages and logs

### 2. Feature Requests
- Describe the use case and motivation
- Propose API design if applicable
- Consider backward compatibility

### 3. Code Contributions
- Bug fixes
- New features
- Performance improvements
- Documentation improvements
- Test coverage expansion

### 4. Documentation
- API documentation improvements
- Tutorial and guide enhancements
- Example applications
- Integration guides

## 🛠️ Development Setup

### Prerequisites
- Rust 1.75 or later
- Git
- A GitHub account

### Local Development
```bash
# Clone the repository
git clone https://github.com/your-username/mcp-protocol-sdk.git
cd mcp-protocol-sdk

# Install dependencies and build
cargo build --all-features

# Run tests
cargo test --all-features

# Run examples
cargo run --example echo_server --features stdio,tracing-subscriber
```

## 📝 Pull Request Process

### 1. Before You Start
- Check existing issues and PRs to avoid duplication
- For large changes, create an issue to discuss the approach
- Fork the repository and create a feature branch

### 2. Making Changes
```bash
# Create a feature branch
git checkout -b feature/your-feature-name

# Make your changes
# ... edit files ...

# Test your changes
cargo test --all-features
cargo clippy --all-features
cargo fmt

# Commit with clear messages
git commit -m "feat: add support for custom authentication"
```

### 3. Submitting a PR
- Create a pull request with a clear title and description
- Reference any related issues
- Include tests for new functionality
- Update documentation as needed
- Ensure CI passes

## 🧪 Testing Guidelines

### Test Categories
1. **Unit Tests** - Test individual functions and modules
2. **Integration Tests** - Test component interactions
3. **Example Tests** - Ensure examples compile and run
4. **Feature Tests** - Test feature flag combinations

### Running Tests
```bash
# Run all tests
cargo test --all-features

# Test specific features
cargo test --no-default-features --features stdio
cargo test --no-default-features --features http
cargo test --no-default-features --features websocket

# Test minimal build
cargo check --no-default-features --lib

# Run clippy for linting
cargo clippy --all-features -- -D warnings

# Format code
cargo fmt --all
```

### Writing Tests
```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mcp_protocol_sdk::testing::*;

    #[tokio::test]
    async fn test_tool_execution() {
        let mut server = MockServer::new();
        server.expect_tool_call("test_tool")
            .with_params(json!({"param": "value"}))
            .return_result(json!({"result": "success"}));

        // Test your code here
    }
}
```

## 📋 Code Style

### Rust Guidelines
- Follow official Rust style guidelines
- Use `cargo fmt` for consistent formatting
- Address all `cargo clippy` warnings
- Use meaningful variable and function names
- Add comprehensive documentation

### Documentation Standards
```rust
/// Brief description of the function
/// 
/// # Arguments
/// 
/// * `param1` - Description of parameter
/// * `param2` - Description of parameter
/// 
/// # Returns
/// 
/// Description of return value
/// 
/// # Errors
/// 
/// Description of when this function returns an error
/// 
/// # Examples
/// 
/// ```rust
/// use mcp_protocol_sdk::prelude::*;
/// 
/// let result = my_function("example").await?;
/// assert_eq!(result, expected_value);
/// ```
pub async fn my_function(param1: &str) -> Result<String, McpError> {
    // Implementation
}
```

## 🏗️ Architecture Guidelines

### Module Organization
```
src/
├── lib.rs              # Main library entry point
├── core/               # Core types and traits
│   ├── tool.rs         # Tool system
│   ├── resource.rs     # Resource system
│   └── prompt.rs       # Prompt system
├── transport/          # Transport implementations
│   ├── mod.rs          # Transport trait
│   ├── stdio.rs        # STDIO transport
│   ├── http.rs         # HTTP transport (feature-gated)
│   └── websocket.rs    # WebSocket transport (feature-gated)
├── protocol/           # Protocol implementation
│   ├── types.rs        # Protocol types
│   └── validation.rs   # Validation logic
├── client/             # Client implementation
├── server/             # Server implementation
└── errors.rs           # Error types
```

### Feature Gates
- Use feature gates for optional functionality
- Document feature requirements clearly
- Test all feature combinations in CI

```rust
#[cfg(feature = "http")]
pub mod http {
    // HTTP-specific code
}
```

## 🔄 Release Process

### Version Numbering
We follow [Semantic Versioning](https://semver.org/):
- MAJOR: Breaking changes
- MINOR: New features (backward compatible)
- PATCH: Bug fixes

### Changelog
- Update CHANGELOG.md for all changes
- Use conventional commit format
- Group changes by category (Added, Changed, Fixed, etc.)

## 🤝 Community Guidelines

### Code of Conduct
- Be respectful and inclusive
- Help newcomers and answer questions
- Provide constructive feedback
- Follow our Code of Conduct

### Communication
- Use GitHub Issues for bugs and features
- Join discussions on GitHub Discussions
- Be patient and helpful with questions

## 🎯 Good First Issues

Looking to contribute? Check for issues labeled:
- `good-first-issue` - Perfect for newcomers
- `help-wanted` - We'd love assistance
- `documentation` - Improve our docs
- `example` - Add new examples

## 📚 Resources

- [MCP Specification](https://spec.modelcontextprotocol.io/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Tokio Documentation](https://tokio.rs/)
- [Serde Documentation](https://serde.rs/)

## 🔒 License

By contributing to this project, you agree that your contributions will be licensed under the **MIT License**.

### License Requirements

- All new files must include the MIT license header:
  ```rust
  // Copyright (c) 2025 MCP Rust Contributors
  // SPDX-License-Identifier: MIT
  ```
- Contributions must be compatible with the MIT License
- External dependencies must use compatible licenses (MIT, Apache 2.0, BSD)

### Intellectual Property

- You must have the right to contribute the code
- Original work only - no copied code without proper attribution
- By submitting a pull request, you grant the project maintainers the right to use your contribution under the MIT License

## 🙏 Recognition

Contributors will be:
- Added to the contributors list
- Mentioned in release notes
- Recognized in the community

Thank you for helping make the MCP Protocol SDK better! 🚀
