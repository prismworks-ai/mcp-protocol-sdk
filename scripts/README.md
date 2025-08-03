# Scripts Directory

Utility scripts for MCP Protocol SDK maintenance and badge management.

## 🏷️ Badge Manager

### `badge-manager.sh`

Comprehensive badge management script that provides:

- **Badge Status Validation**: Check accessibility of all badges
- **Workflow Triggers**: Automatically trigger GitHub workflows
- **Intelligent Updates**: Update specific badges or all at once
- **Status Monitoring**: View recent workflow execution status
- **Troubleshooting**: Built-in help for common badge issues

#### Usage

```bash
# Check all badge status
./scripts/badge-manager.sh check

# Update all badges
./scripts/badge-manager.sh update

# Update specific badge
./scripts/badge-manager.sh update codecov
./scripts/badge-manager.sh update ci
./scripts/badge-manager.sh update security

# Show recent workflow status
./scripts/badge-manager.sh status

# List all available badges
./scripts/badge-manager.sh list

# Get help
./scripts/badge-manager.sh help
```

#### Prerequisites

1. **GitHub CLI** (recommended for full functionality):
   ```bash
   # Install GitHub CLI
   curl -fsSL https://cli.github.com/packages/githubcli-archive-keyring.gpg | sudo dd of=/usr/share/keyrings/githubcli-archive-keyring.gpg
   echo "deb [arch=$(dpkg --print-architecture) signed-by=/usr/share/keyrings/githubcli-archive-keyring.gpg] https://cli.github.com/packages stable main" | sudo tee /etc/apt/sources.list.d/github-cli.list > /dev/null
   sudo apt update
   sudo apt install gh
   
   # Authenticate
   gh auth login
   ```

2. **curl** (for badge validation):
   ```bash
   sudo apt install curl  # Ubuntu/Debian
   brew install curl      # macOS
   ```

#### Features

- ✅ **Badge Validation**: Tests all badge URLs for accessibility
- 🚀 **Workflow Triggers**: Automatically trigger GitHub Actions workflows
- 📊 **Status Monitoring**: View recent workflow execution status
- 🎯 **Targeted Updates**: Update specific badges (ci, security, deps, docs, benchmarks, codecov)
- 🔧 **Troubleshooting**: Built-in help for common issues
- 🎨 **Colored Output**: Enhanced readability with color-coded messages

#### Badge Types

| Type | Description | Workflow |
|------|-------------|----------|
| `ci` | Continuous Integration | `ci.yml` |
| `security` | Security Audit | `security.yml` |
| `deps` | Dependencies | `dependencies.yml` |
| `docs` | Documentation | `docs.yml` |
| `benchmarks` | Performance Tests | `benchmarks.yml` |
| `codecov` | Code Coverage | `codecov-refresh.yml` |

#### Example Output

```bash
$ ./scripts/badge-manager.sh check
=================================
🏷️ Badge Manager for MCP Protocol SDK
=================================

📊 Checking Prerequisites
==================================================
✅ GitHub CLI detected
✅ GitHub CLI authenticated
✅ curl available for badge status checks

📊 Badge Status Validation
==================================================
✅ ci
✅ security
✅ dependencies
✅ documentation
✅ benchmarks
✅ release
❌ codecov - Not accessible

⚠️ Failed badges: codecov
```

## 🛠️ Legacy Scripts

### `fix-badges.sh` (Deprecated)

The original badge fix script has been replaced by `badge-manager.sh` for enhanced functionality. The old script is kept for reference but `badge-manager.sh` is recommended for all badge management tasks.

## 🔗 Related Files

- [Workflow Status Documentation](../.github/WORKFLOW_STATUS.md)
- [Badge Update Workflow](../.github/workflows/badge-update.yml)
- [Codecov Refresh Workflow](../.github/workflows/codecov-refresh.yml)
- [Codecov Configuration](../codecov.yml)

## 📝 Notes

- All scripts should be run from the project root directory
- GitHub CLI authentication is required for workflow triggers
- Badge updates may take 5-10 minutes to appear due to GitHub caching
- The codecov badge requires `CODECOV_TOKEN` to be configured in repository secrets

---

**Need Help?** Run `./scripts/badge-manager.sh help` for detailed usage information.
