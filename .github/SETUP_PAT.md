# GitHub Personal Access Token Setup for Automated Dependencies

## Overview

The Dependencies workflow requires a Personal Access Token (PAT) to create pull requests automatically. GitHub's default `GITHUB_TOKEN` cannot create pull requests due to security restrictions.

## Required Setup (Repository Admin Only)

### 1. Generate Personal Access Token

1. Go to **GitHub Settings** → **Developer settings** → **Personal access tokens** → **Tokens (classic)**
2. Click **"Generate new token (classic)"**
3. Configure the token:
   - **Note**: `mcp-protocol-sdk-dependencies` (or descriptive name)
   - **Expiration**: Choose appropriate duration (90 days recommended for security)
   - **Scopes**: Select the following required permissions:
     - ✅ `repo` (Full control of private repositories)
     - ✅ `workflow` (Update GitHub Action workflows)
     - ✅ `write:packages` (if using GitHub Packages)

### 2. Add Token to Repository Secrets

1. Go to **Repository Settings** → **Secrets and variables** → **Actions**
2. Click **"New repository secret"**
3. Configure the secret:
   - **Name**: `GH_TOKEN`
   - **Secret**: Paste the generated PAT from step 1
4. Click **"Add secret"**

### 3. Verify Setup

Once the secret is added:

1. Go to **Actions** tab in the repository
2. Find the **"Dependencies"** workflow
3. Click **"Run workflow"** → **"Run workflow"** to test
4. Monitor the workflow execution - it should now successfully create pull requests

## Security Considerations

### Token Permissions
- The PAT has broad repository access - treat it like a password
- Only repository administrators should generate and configure this token
- The token is securely stored in GitHub Secrets and not visible in logs

### Token Rotation
- Set reasonable expiration dates (90 days recommended)
- Set calendar reminders to rotate tokens before expiration
- When rotating, generate new token and update the `GH_TOKEN` secret

### Access Control
- Only users with "Admin" repository permissions can view/modify secrets
- The token inherits the permissions of the user who created it
- Consider using a dedicated service account for automation tokens

## Troubleshooting

### Workflow Still Fails?

1. **Check Secret Name**: Must be exactly `GH_TOKEN` (case-sensitive)
2. **Verify Token Permissions**: Ensure `repo` and `workflow` scopes are enabled
3. **Check Token Expiration**: Generate new token if expired
4. **Repository Permissions**: Token creator must have admin access to the repository

### Common Errors

- `"Resource not accessible by integration"` → Token missing `repo` scope
- `"Bad credentials"` → Token invalid or expired
- `"Not Found"` → Token missing `workflow` scope or repository access

## Workflow Behavior After Setup

### Automated Schedule
- Runs every **Monday at 2 AM UTC**
- Checks for dependency updates across all Rust crates
- Runs full test suite with all features enabled
- Performs security audit with `cargo audit`

### Pull Request Creation
- Creates branch: `update-dependencies`
- Includes detailed update summary with before/after package versions
- Applies labels: `dependencies`, `automated`
- Auto-deletes branch after PR merge/close

### Manual Trigger
- Available via **Actions** → **Dependencies** → **"Run workflow"**
- Useful for testing or immediate dependency updates
- Same behavior as scheduled runs

## Alternative Solutions

### GitHub App (Advanced)
For organizations with multiple repositories, consider creating a GitHub App with appropriate permissions instead of using PATs.

### Dependabot (Built-in)
GitHub's Dependabot provides similar functionality but with different configuration and behavior patterns.

---

**Status**: ⏳ **Setup Required**  
**Action Required**: Repository admin needs to add `GH_TOKEN` secret  
**Estimated Setup Time**: 5 minutes  
**Security Level**: High (requires admin access)
