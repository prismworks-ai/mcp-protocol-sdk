# Workflow Status & Badge Management

This document provides comprehensive information about workflow badges and their management.

## 🏷️ Current Badge Status

| Badge | Status | Workflow | Purpose |
|-------|--------|----------|----------|
| [![CI](https://github.com/mcp-rust/mcp-protocol-sdk/actions/workflows/ci.yml/badge.svg)](https://github.com/mcp-rust/mcp-protocol-sdk/actions/workflows/ci.yml) | Active | `ci.yml` | Continuous Integration |
| [![Security](https://github.com/mcp-rust/mcp-protocol-sdk/actions/workflows/security.yml/badge.svg)](https://github.com/mcp-rust/mcp-protocol-sdk/actions/workflows/security.yml) | Active | `security.yml` | Security Audit |
| [![Dependencies](https://github.com/mcp-rust/mcp-protocol-sdk/actions/workflows/dependencies.yml/badge.svg)](https://github.com/mcp-rust/mcp-protocol-sdk/actions/workflows/dependencies.yml) | Weekly | `dependencies.yml` | Dependency Updates |
| [![Documentation](https://github.com/mcp-rust/mcp-protocol-sdk/actions/workflows/docs.yml/badge.svg)](https://github.com/mcp-rust/mcp-protocol-sdk/actions/workflows/docs.yml) | Active | `docs.yml` | Documentation Build |
| [![Benchmarks](https://github.com/mcp-rust/mcp-protocol-sdk/actions/workflows/benchmarks.yml/badge.svg)](https://github.com/mcp-rust/mcp-protocol-sdk/actions/workflows/benchmarks.yml) | On-Demand | `benchmarks.yml` | Performance Testing |
| [![Release](https://github.com/mcp-rust/mcp-protocol-sdk/actions/workflows/release.yml/badge.svg)](https://github.com/mcp-rust/mcp-protocol-sdk/actions/workflows/release.yml) | On-Tag | `release.yml` | Release Process |
| [![codecov](https://codecov.io/gh/mcp-rust/mcp-protocol-sdk/branch/main/graph/badge.svg)](https://codecov.io/gh/mcp-rust/mcp-protocol-sdk) | ⚠️ **Needs Attention** | `codecov-refresh.yml` | Code Coverage |

## 🚀 Badge Update Solutions

### Automated Badge Updates

A dedicated **Badge Update Workflow** (`badge-update.yml`) provides:

- **Manual Triggers**: Update specific badges on-demand
- **Scheduled Updates**: Weekly refresh to keep badges current
- **Intelligent Routing**: Trigger only necessary workflows
- **Status Reporting**: Comprehensive update status

### Manual Badge Management

Use the enhanced badge manager script:

```bash
# Check all badge status
./scripts/badge-manager.sh check

# Update all badges
./scripts/badge-manager.sh update

# Update specific badge (e.g., codecov)
./scripts/badge-manager.sh update codecov

# Show workflow status
./scripts/badge-manager.sh status
```

## 🎯 Codecov Badge Fix

### Current Issue
The codecov badge has never been updated due to:
1. Missing or misconfigured `CODECOV_TOKEN`
2. Coverage upload failures
3. Badge caching issues

### Solution Steps

1. **Configure Codecov Token**:
   ```bash
   # Repository Settings → Secrets → Actions → New repository secret
   # Name: CODECOV_TOKEN
   # Value: <your-codecov-token>
   ```

2. **Trigger Codecov Refresh**:
   ```bash
   # Using GitHub CLI
   gh workflow run codecov-refresh.yml
   
   # Or using badge manager
   ./scripts/badge-manager.sh update codecov
   ```

3. **Verify Upload**:
   - Check [Codecov Dashboard](https://codecov.io/gh/mcp-rust/mcp-protocol-sdk)
   - Monitor workflow progress
   - Validate badge URL response

## 🔧 Workflow Trigger Methods

### 1. GitHub CLI (Recommended)
```bash
# Install GitHub CLI
curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg
echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null
sudo apt update
sudo apt install gh

# Authenticate
gh auth login

# Trigger workflows
gh workflow run ci.yml
gh workflow run codecov-refresh.yml
gh workflow run badge-update.yml -f badge_type=codecov
```

### 2. GitHub UI
1. Navigate to [Actions](https://github.com/mcp-rust/mcp-protocol-sdk/actions)
2. Select desired workflow
3. Click "Run workflow"
4. Choose branch and parameters
5. Click "Run workflow"

### 3. Badge Manager Script
```bash
# Make executable
chmod +x scripts/badge-manager.sh

# Update all badges
./scripts/badge-manager.sh update

# Update specific badge
./scripts/badge-manager.sh update codecov
```

## 📊 Workflow Monitoring

### Status Dashboard
- **Main**: [GitHub Actions](https://github.com/mcp-rust/mcp-protocol-sdk/actions)
- **Codecov**: [Coverage Dashboard](https://codecov.io/gh/mcp-rust/mcp-protocol-sdk)
- **Crates.io**: [Package Status](https://crates.io/crates/mcp-protocol-sdk)

### Health Checks
```bash
# Check workflow status
gh run list --limit 10

# Check specific workflow
gh run list --workflow=ci.yml --limit 5

# View workflow details
gh run view [RUN_ID]
```

## 🔄 Badge Update Schedule

| Badge | Update Trigger | Frequency | Notes |
|-------|---------------|-----------|-------|
| CI | Push/PR | On every commit | Auto-updates |
| Security | Weekly + Manual | Sunday 00:00 UTC | Scheduled |
| Dependencies | Weekly + Manual | Monday 02:00 UTC | Scheduled |
| Documentation | Push to main | On doc changes | Auto-updates |
| Benchmarks | Manual | On-demand | Manual trigger |
| Release | Tag push | On releases | Auto-updates |
| Codecov | Push + Manual | On coverage changes | **Manual fix needed** |

## 🛠️ Troubleshooting

### Common Issues

1. **Badge shows "unknown" status**
   - Workflow hasn't run yet
   - Workflow file syntax error
   - Badge URL incorrect

2. **Badge shows old status**
   - GitHub badge caching (5-10 minutes)
   - Workflow completed but badge not refreshed
   - CDN caching issues

3. **Codecov badge not updating**
   - Missing `CODECOV_TOKEN`
   - Coverage upload failed
   - Codecov service issues

### Solutions

1. **Force badge refresh**:
   ```bash
   ./scripts/badge-manager.sh update
   ```

2. **Check workflow logs**:
   ```bash
   gh run list --limit 5
   gh run view [RUN_ID]
   ```

3. **Validate badge URLs**:
   ```bash
   ./scripts/badge-manager.sh check
   ```

## 🔗 Quick Links

- [Actions Dashboard](https://github.com/mcp-rust/mcp-protocol-sdk/actions)
- [Badge Update Workflow](https://github.com/mcp-rust/mcp-protocol-sdk/actions/workflows/badge-update.yml)
- [Codecov Dashboard](https://codecov.io/gh/mcp-rust/mcp-protocol-sdk)
- [Repository Settings](https://github.com/mcp-rust/mcp-protocol-sdk/settings)
- [Workflow Files](.github/workflows/)

---

**Last Updated**: 2025-08-03  
**Maintained By**: Repository maintainers  
**Status**: ✅ Active monitoring and automated updates
