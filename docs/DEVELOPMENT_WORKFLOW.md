# Development Workflow Guide

This guide explains how to develop for MCP Rust SDK with local checks that mirror GitHub Actions.

## 🚀 Quick Start

### 1. Set up development environment
```bash
# Install all necessary tools
make setup
# or
./scripts/setup-dev.sh
```

### 2. Run checks before committing
```bash
# Quick essential checks (recommended before each commit)
make quick-check

# Full CI checks (run before pushing)
make check
```

## 🔧 Available Commands

### Essential Development
```bash
make format      # Format code
make lint        # Run clippy
make test        # Run tests
make examples    # Check examples
make docs        # Build docs
```

### Pre-commit Workflow
```bash
make pre-commit  # Run all pre-commit checks
# Equivalent to: format + lint + compile + test + examples
```

### Full CI Validation
```bash
make ci-local    # Run complete CI suite locally
# This mirrors ALL GitHub Actions workflows
```

## 🔄 Recommended Workflow

### Daily Development
1. **Make changes** to code
2. **Format and check** regularly:
   ```bash
   make format lint
   ```
3. **Test your changes**:
   ```bash
   make test
   ```

### Before Committing
1. **Run essential checks**:
   ```bash
   make quick-check
   ```
   This runs automatically via git pre-commit hook.

2. **If checks fail**, fix issues:
   ```bash
   make format      # Fix formatting
   make lint-fix    # Auto-fix clippy issues
   cargo test       # Run tests to see failures
   ```

### Before Pushing
1. **Run full CI suite**:
   ```bash
   make check
   ```
   This ensures your changes will pass GitHub Actions.

2. **Check specific features** you modified:
   ```bash
   make test-http       # If you changed HTTP transport
   make test-websocket  # If you changed WebSocket transport
   make test-stdio      # If you changed STDIO transport
   ```

## 🧪 Testing Strategy

### Test Levels
1. **Unit tests**: `make test`
2. **Feature tests**: `make test-all`
3. **Integration tests**: `make test-all`
4. **Example compilation**: `make examples`

### Feature-Specific Testing
```bash
make test-stdio      # Test STDIO transport only
make test-http       # Test HTTP transport only  
make test-websocket  # Test WebSocket transport only
make test-validation # Test validation features only
```

## 🔍 What Each Check Does

### Pre-commit Hook (Automatic)
When you run `git commit`, automatically runs:
- ✅ Code formatting check (`cargo fmt --check`)
- ✅ Clippy lints (`cargo clippy`)
- ✅ Compilation check (`cargo check`)
- ✅ Quick tests (`cargo test --lib --bins`)
- ✅ Documentation build (`cargo doc`)
- ✅ Basic examples compilation

### Quick Check (`make quick-check`)
Essential checks for fast feedback:
- ✅ All pre-commit checks
- ✅ Feature compilation checks
- ⏭️ Skips cross-platform tests
- ⏭️ Skips benchmarks and coverage

### Full CI Check (`make check`)
Complete validation matching GitHub Actions:
- ✅ All quick checks
- ✅ Cross-platform compilation
- ✅ All feature combinations
- ✅ All examples
- ✅ Security audit
- ✅ Code coverage (if tarpaulin installed)
- ✅ Benchmarks

## 🛠️ Tool Installation

The setup script installs these tools:
- **rustfmt**: Code formatting
- **clippy**: Linting and suggestions
- **cargo-audit**: Security vulnerability scanning
- **cargo-tarpaulin**: Code coverage (Linux/macOS)
- **cargo-deny**: Dependency analysis

### Manual Installation
```bash
# Essential tools (required)
rustup component add rustfmt clippy

# Optional tools (recommended)
cargo install cargo-audit
cargo install cargo-tarpaulin  # Linux/macOS only
cargo install cargo-deny
cargo install cargo-watch      # For development
cargo install cargo-outdated   # Check dependencies
```

## ⚡ Development Tips

### Watch Mode
```bash
make watch       # Auto-run tests on file changes
make watch-check # Auto-run checks on file changes
```

### Fixing Common Issues

#### Formatting Issues
```bash
make format      # Auto-fix formatting
```

#### Clippy Issues  
```bash
make lint-fix    # Auto-fix clippy issues
# Review changes before committing!
```

#### Test Failures
```bash
cargo test --verbose                    # See detailed output
cargo test test_name -- --nocapture    # See println! output
cargo test --features http             # Test specific features
```

#### Documentation Issues
```bash
make docs-open   # Build and open docs in browser
```

## 🚫 Preventing CI Failures

### Common CI Failure Causes
1. **Formatting**: Code not formatted with `rustfmt`
2. **Clippy warnings**: Linting issues
3. **Compilation**: Code doesn't compile with all features
4. **Test failures**: Unit or integration tests fail
5. **Example issues**: Examples don't compile
6. **Documentation**: Doc comments have issues

### Prevention Strategy
1. **Use pre-commit hooks** (automatic)
2. **Run `make quick-check`** before each commit
3. **Run `make check`** before pushing
4. **Test feature combinations** you modify
5. **Keep dependencies updated** with `cargo update`

## 📊 Performance Monitoring

### Benchmarks
```bash
make bench       # Run performance benchmarks
```

### Binary Size Analysis
```bash
make bloat       # Analyze what contributes to binary size
```

### Dependency Analysis
```bash
make deps        # Show dependency tree
make outdated    # Check for outdated dependencies
```

## 🔒 Security

### Security Audit
```bash
make audit       # Check for security vulnerabilities
```

### Dependency Checking
The project uses `cargo-deny` to check:
- License compatibility
- Security advisories
- Duplicate dependencies
- Banned dependencies

## 📝 Documentation

### Building Docs
```bash
make docs        # Build documentation
make docs-open   # Build and open in browser
```

### Documentation Standards
- All public APIs must have doc comments
- Include examples in doc comments
- Link to related functions/types
- Explain safety requirements for unsafe code

## 🎯 CI/CD Integration

### GitHub Actions Workflows
The repository has these workflows that run on every push/PR:
1. **CI** (`ci.yml`): Main compilation and testing
2. **Quality** (`quality.yml`): Code quality checks
3. **Security** (`security.yml`): Security audits
4. **Docs** (`docs.yml`): Documentation checks
5. **Benchmarks** (`benchmarks.yml`): Performance regression detection
6. **Dependencies** (`dependencies.yml`): Dependency checking

### Local Equivalents
```bash
make check       # Runs equivalent of ALL workflows
make quick-check # Runs essential checks only
make audit       # Security workflow
make bench       # Benchmarks workflow
make docs        # Documentation workflow
```

## 🐛 Troubleshooting

### Pre-commit Hook Issues
```bash
# Check if hook is executable
ls -la .git/hooks/pre-commit

# Re-enable if needed
chmod +x .git/hooks/pre-commit

# Bypass hook temporarily (not recommended)
git commit --no-verify
```

### Tool Installation Issues
```bash
# Re-run setup
make setup

# Manual tool installation
cargo install cargo-audit cargo-tarpaulin
```

### Performance Issues
```bash
# Clean build cache
make clean

# Use faster linker (Linux)
sudo apt install mold
export RUSTFLAGS="-C link-arg=-fuse-ld=mold"
```

By following this workflow, you can catch issues locally before they cause CI failures, saving time and maintaining code quality! 🚀
