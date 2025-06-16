# 🚀 Local CI Testing Setup - MCP Protocol SDK

This guide explains how to run the same tests locally that GitHub Actions runs in CI, preventing failures before you push.

## 🎯 Overview

Your MCP Protocol SDK now has a complete local CI testing environment that mirrors your GitHub Actions pipeline. This prevents CI failures by catching issues locally before pushing.

## 📋 Quick Start

### Essential Commands

```bash
# Quick validation before committing
make quick

# Full CI check before pushing  
make check

# Mirror exact GitHub Actions pipeline
make ci-local

# Fix common issues automatically
make fmt-fix
make clippy-fix
```

## 🔧 Available Tools

### Make Commands

| Command | Description | Use Case |
|---------|-------------|----------|
| `make quick` | Format + Clippy + Basic tests | Before commit |
| `make check` | Standard CI pipeline | Before push |
| `make ci-local` | Mirror GitHub Actions | Exact CI simulation |
| `make ci-full` | Full pipeline + MSRV | Release preparation |
| `make fmt-fix` | Fix code formatting | Fix style issues |
| `make clippy-fix` | Auto-fix Clippy suggestions | Fix linting |
| `make coverage` | Generate coverage report | Code quality check |
| `make security` | Security audit | Vulnerability check |

### Direct Script Access

```bash
# Local CI script with options
./scripts/local-ci.sh                # Standard checks
./scripts/local-ci.sh --quick         # Quick validation
./scripts/local-ci.sh --full          # Full pipeline
./scripts/local-ci.sh --coverage      # With coverage
./scripts/local-ci.sh --help          # All options
```

## 🪝 Git Hooks (Optional)

Install pre-push hooks to automatically run CI checks:

```bash
# Install hooks (runs CI on every push to main/develop)
make setup-hooks

# Remove hooks
make remove-hooks
```

**Warning:** Pre-push hooks will slow down your pushes but prevent CI failures.

## 📊 What Gets Tested

### 1. Code Quality
- **Formatting**: `cargo fmt --check`
- **Linting**: `cargo clippy` with strict rules
- **Compilation**: All feature combinations

### 2. Test Suite
- **Default features**: `cargo test`
- **All features**: `cargo test --all-features`
- **No default features**: `cargo test --no-default-features`
- **Feature-specific**: Each feature tested individually

### 3. Examples & Documentation
- **Examples compilation**: All examples checked
- **Documentation**: `cargo doc` generation
- **Doc tests**: Embedded code examples

### 4. Security & Dependencies
- **Security audit**: `cargo audit`
- **Dependency analysis**: Duplicate detection
- **License compliance**: License validation

### 5. Advanced Checks (Full CI)
- **MSRV testing**: Multiple Rust versions
- **Code coverage**: `cargo tarpaulin`
- **Benchmarks**: Performance tests

## 🔄 Development Workflow

### Daily Development
```bash
# 1. Make changes
# 2. Quick check
make quick

# 3. Fix any issues
make fmt-fix
make clippy-fix

# 4. Commit
git add .
git commit -m "feat: add new feature"
```

### Before Pushing
```bash
# 1. Full validation
make check

# 2. If everything passes
git push origin feature-branch
```

### Release Preparation
```bash
# Comprehensive validation
make release-check

# This runs:
# - make clean
# - make ci-full  
# - make security
# - make coverage
```

## ⚙️ Configuration

### Tool Installation
Install all required development tools:
```bash
make install-tools
```

This installs:
- `cargo-audit` - Security auditing
- `cargo-tarpaulin` - Code coverage
- `cargo-tree` - Dependency analysis
- `cargo-license` - License checking
- `cargo-deny` - Dependency policies

### Complete Setup
```bash
# Full development environment setup
make dev-setup

# This runs:
# - make install-tools
# - make setup-hooks
```

## 🎯 CI Pipeline Comparison

| GitHub Actions | Local Equivalent | Notes |
|----------------|------------------|-------|
| Format Check | `make fmt` | Exact same command |
| Clippy | `make clippy` | Same flags and settings |
| Test Matrix | `make test-all` | All feature combinations |
| Examples | `make examples` | All examples checked |
| Documentation | `make docs` | Same doc generation |
| Security | `make security` | Same audit tools |
| MSRV | `make ci-full` | Multiple Rust versions |

## 📈 Performance Tips

### Speed Up Local CI

1. **Use Quick Check for Frequent Testing**
   ```bash
   make quick  # ~30 seconds vs 5+ minutes
   ```

2. **Skip Optional Checks During Development**
   ```bash
   ./scripts/local-ci.sh --skip-security --skip-examples
   ```

3. **Use Cargo Cache**
   - Builds are cached automatically
   - First run is slow, subsequent runs are fast

4. **Parallel Testing**
   ```bash
   cargo test --all-features -j 8  # Use 8 cores
   ```

## 🔍 Troubleshooting

### Common Issues

**Formatting Failures**
```bash
# Fix automatically
make fmt-fix
```

**Clippy Warnings**
```bash
# Fix automatically where possible
make clippy-fix

# Or fix manually and re-run
make clippy
```

**Test Failures**
```bash
# Run specific tests
cargo test test_name

# Run with output
cargo test -- --nocapture

# Run single threaded
cargo test -- --test-threads=1
```

**Tool Installation Issues**
```bash
# Reinstall all tools
make install-tools

# Check tool versions
cargo audit --version
cargo tarpaulin --version
```

### Performance Issues

**Slow First Run**
- First run downloads dependencies (~5-10 minutes)
- Subsequent runs use cache (~30 seconds - 2 minutes)

**High Memory Usage**
- Code coverage uses significant memory
- Skip coverage for quick testing: `./scripts/local-ci.sh --skip-coverage`

## 📋 CI Checklist

Before pushing to main/develop:

- [ ] `make quick` passes (format, clippy, basic tests)
- [ ] `make check` passes (full CI pipeline)
- [ ] All new code is tested
- [ ] Documentation is updated
- [ ] No TODO/FIXME comments in critical paths

Before releasing:

- [ ] `make release-check` passes
- [ ] Security audit clean
- [ ] Code coverage maintained/improved
- [ ] All examples work
- [ ] CHANGELOG updated

## 🎉 Benefits

### For You
- **Catch issues early** - Fix problems before CI fails
- **Faster feedback** - Local testing is faster than waiting for CI
- **Work offline** - No internet required for testing
- **Learn faster** - See exactly what CI tests

### For Your Team
- **Clean CI** - Fewer failed builds in shared CI
- **Faster reviews** - PRs are ready when submitted
- **Consistent quality** - Same standards enforced locally
- **Better velocity** - Less time fixing CI failures

## 🔗 Integration with GitHub Actions

Your local CI perfectly mirrors the remote CI:

1. **Same commands** - Identical cargo commands and flags
2. **Same tools** - Exact same versions where possible
3. **Same matrix** - All feature combinations tested
4. **Same security** - Same audit tools and policies

This means:
- ✅ If local CI passes, GitHub CI should pass
- ⚠️ If local CI fails, fix before pushing
- 🚀 Confident pushes knowing CI will succeed

## 📚 Additional Resources

- [GitHub Actions Workflow](/.github/workflows/ci.yml)
- [Security Policy](/.github/workflows/security.yml)
- [Quality Checks](/.github/workflows/quality.yml)
- [Make Help](Makefile) - Run `make help` for all commands
- [Script Help](./scripts/local-ci.sh) - Run `./scripts/local-ci.sh --help`

---

**Happy coding! 🦀✨**

Your MCP Protocol SDK now has a robust local CI environment that ensures code quality and prevents failures before they reach GitHub Actions.
