# ⚡ Cursor IDE Integration

Integrate your MCP server with Cursor IDE to enhance AI-powered development workflows.

## Overview

Cursor IDE can connect to MCP servers to provide AI assistants with custom development tools and capabilities. This enables:

- Custom code analysis tools
- Project-specific workflows
- Database schema access
- API documentation integration
- Development environment automation

## Integration Methods

### 1. VS Code Extension (Recommended)

Create a VS Code extension that runs your MCP server and integrates with Cursor:

#### Extension Setup

Create `package.json`:

```json
{
  "name": "my-mcp-extension",
  "displayName": "My MCP Tools",
  "description": "Custom MCP tools for development",
  "version": "1.0.0",
  "engines": {
    "vscode": "^1.60.0"
  },
  "categories": ["Other"],
  "activationEvents": ["*"],
  "main": "./out/extension.js",
  "contributes": {
    "commands": [
      {
        "command": "myMcp.start",
        "title": "Start MCP Server"
      },
      {
        "command": "myMcp.stop", 
        "title": "Stop MCP Server"
      }
    ],
    "configuration": {
      "type": "object",
      "title": "My MCP Configuration",
      "properties": {
        "myMcp.serverPath": {
          "type": "string",
          "default": "",
          "description": "Path to MCP server binary"
        }
      }
    }
  },
  "scripts": {
    "compile": "tsc -p ./",
    "watch": "tsc -watch -p ./"
  },
  "devDependencies": {
    "@types/vscode": "^1.60.0",
    "typescript": "^4.0.0"
  }
}
```

Create `src/extension.ts`:

```typescript
import * as vscode from 'vscode';
import { spawn, ChildProcess } from 'child_process';

let mcpServer: ChildProcess | null = null;

export function activate(context: vscode.ExtensionContext) {
    // Register commands
    const startCommand = vscode.commands.registerCommand('myMcp.start', startMcpServer);
    const stopCommand = vscode.commands.registerCommand('myMcp.stop', stopMcpServer);
    
    context.subscriptions.push(startCommand, stopCommand);
    
    // Auto-start server when extension activates
    startMcpServer();
}

async function startMcpServer() {
    const config = vscode.workspace.getConfiguration('myMcp');
    const serverPath = config.get<string>('serverPath');
    
    if (!serverPath) {
        vscode.window.showErrorMessage('MCP server path not configured');
        return;
    }
    
    try {
        mcpServer = spawn(serverPath, [], {
            stdio: ['pipe', 'pipe', 'pipe']
        });
        
        mcpServer.on('error', (error) => {
            vscode.window.showErrorMessage(`MCP server error: ${error.message}`);
        });
        
        mcpServer.on('exit', (code) => {
            if (code !== 0) {
                vscode.window.showWarningMessage(`MCP server exited with code ${code}`);
            }
        });
        
        vscode.window.showInformationMessage('MCP server started');
        
        // Register MCP capabilities with Cursor
        await registerMcpCapabilities();
        
    } catch (error) {
        vscode.window.showErrorMessage(`Failed to start MCP server: ${error}`);
    }
}

function stopMcpServer() {
    if (mcpServer) {
        mcpServer.kill();
        mcpServer = null;
        vscode.window.showInformationMessage('MCP server stopped');
    }
}

async function registerMcpCapabilities() {
    // Communicate with Cursor's AI system to register MCP tools
    // This part depends on Cursor's specific integration APIs
}

export function deactivate() {
    stopMcpServer();
}
```

### 2. Development Environment Server

Create a server specifically for development workflows:

```rust
use mcp_protocol_sdk::prelude::*;
use serde_json::json;
use std::collections::HashMap;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("cursor-dev-server", "1.0.0")
        .with_description("Development tools for Cursor IDE");
    
    setup_code_analysis_tools(&mut server).await?;
    setup_project_tools(&mut server).await?;
    setup_database_tools(&mut server).await?;
    
    let transport = StdioServerTransport::new();
    server.run(transport).await?;
    
    Ok(())
}

async fn setup_code_analysis_tools(server: &mut McpServer) -> Result<(), Box<dyn std::error::Error>> {
    // Code complexity analysis
    let complexity_tool = Tool::new(
        "analyze_complexity",
        "Analyze code complexity metrics"
    )
    .with_parameter("file_path", "Path to source file", true)
    .with_parameter("language", "Programming language", false);
    
    server.add_tool(complexity_tool);
    
    server.set_tool_handler("analyze_complexity", |params: HashMap<String, Value>| async move {
        let file_path = params.get("file_path")
            .and_then(|v| v.as_str())
            .ok_or("Missing file_path parameter")?;
        
        let language = params.get("language")
            .and_then(|v| v.as_str())
            .unwrap_or("unknown");
        
        let content = tokio::fs::read_to_string(file_path).await
            .map_err(|e| format!("Failed to read file: {}", e))?;
        
        let metrics = analyze_code_complexity(&content, language)?;
        
        Ok(json!({
            "file": file_path,
            "language": language,
            "lines_of_code": metrics.lines_of_code,
            "cyclomatic_complexity": metrics.cyclomatic_complexity,
            "cognitive_complexity": metrics.cognitive_complexity,
            "maintainability_index": metrics.maintainability_index,
            "suggestions": metrics.suggestions
        }))
    });
    
    // Find TODO/FIXME comments
    let todo_tool = Tool::new(
        "find_todos",
        "Find TODO, FIXME, and other comment markers in codebase"
    )
    .with_parameter("directory", "Directory to search", true)
    .with_parameter("extensions", "File extensions to include", false);
    
    server.add_tool(todo_tool);
    
    server.set_tool_handler("find_todos", |params: HashMap<String, Value>| async move {
        let directory = params.get("directory")
            .and_then(|v| v.as_str())
            .ok_or("Missing directory parameter")?;
        
        let extensions = params.get("extensions")
            .and_then(|v| v.as_array())
            .map(|arr| arr.iter().filter_map(|v| v.as_str()).collect())
            .unwrap_or_else(|| vec!["rs", "js", "ts", "py", "java", "cpp", "c", "h"]);
        
        let todos = find_todo_comments(directory, &extensions).await?;
        
        Ok(json!({
            "directory": directory,
            "total_items": todos.len(),
            "items": todos
        }))
    });
    
    Ok(())
}

async fn setup_project_tools(server: &mut McpServer) -> Result<(), Box<dyn std::error::Error>> {
    // Project structure analysis
    let structure_tool = Tool::new(
        "analyze_project_structure",
        "Analyze project structure and architecture"
    )
    .with_parameter("root_path", "Project root directory", true);
    
    server.add_tool(structure_tool);
    
    server.set_tool_handler("analyze_project_structure", |params: HashMap<String, Value>| async move {
        let root_path = params.get("root_path")
            .and_then(|v| v.as_str())
            .ok_or("Missing root_path parameter")?;
        
        let analysis = analyze_project_structure(root_path).await?;
        
        Ok(json!({
            "project_type": analysis.project_type,
            "build_system": analysis.build_system,
            "dependencies": analysis.dependencies,
            "source_directories": analysis.source_directories,
            "test_directories": analysis.test_directories,
            "documentation": analysis.documentation,
            "ci_cd": analysis.ci_cd,
            "recommendations": analysis.recommendations
        }))
    });
    
    Ok(())
}

async fn setup_database_tools(server: &mut McpServer) -> Result<(), Box<dyn std::error::Error>> {
    // Database schema inspection
    let schema_tool = Tool::new(
        "inspect_database_schema",
        "Inspect database schema and relationships"
    )
    .with_parameter("connection_string", "Database connection string", true)
    .with_parameter("schema_name", "Schema name (optional)", false);
    
    server.add_tool(schema_tool);
    
    server.set_tool_handler("inspect_database_schema", |params: HashMap<String, Value>| async move {
        let connection_string = params.get("connection_string")
            .and_then(|v| v.as_str())
            .ok_or("Missing connection_string parameter")?;
        
        let schema_name = params.get("schema_name")
            .and_then(|v| v.as_str());
        
        let schema_info = inspect_database_schema(connection_string, schema_name).await?;
        
        Ok(json!({
            "tables": schema_info.tables,
            "relationships": schema_info.relationships,
            "indexes": schema_info.indexes,
            "constraints": schema_info.constraints
        }))
    });
    
    Ok(())
}

// Helper functions (implement based on your needs)
struct ComplexityMetrics {
    lines_of_code: usize,
    cyclomatic_complexity: i32,
    cognitive_complexity: i32,
    maintainability_index: f64,
    suggestions: Vec<String>,
}

fn analyze_code_complexity(content: &str, language: &str) -> Result<ComplexityMetrics, String> {
    // Implement code complexity analysis
    Ok(ComplexityMetrics {
        lines_of_code: content.lines().count(),
        cyclomatic_complexity: 1, // Placeholder
        cognitive_complexity: 1,  // Placeholder
        maintainability_index: 85.0, // Placeholder
        suggestions: vec!["Code looks good!".to_string()],
    })
}

async fn find_todo_comments(directory: &str, extensions: &[&str]) -> Result<Vec<serde_json::Value>, String> {
    // Implement TODO finder
    Ok(vec![])
}

struct ProjectAnalysis {
    project_type: String,
    build_system: String,
    dependencies: Vec<String>,
    source_directories: Vec<String>,
    test_directories: Vec<String>,
    documentation: Vec<String>,
    ci_cd: Vec<String>,
    recommendations: Vec<String>,
}

async fn analyze_project_structure(root_path: &str) -> Result<ProjectAnalysis, String> {
    // Implement project analysis
    Ok(ProjectAnalysis {
        project_type: "Rust".to_string(),
        build_system: "Cargo".to_string(),
        dependencies: vec!["tokio".to_string()],
        source_directories: vec!["src/".to_string()],
        test_directories: vec!["tests/".to_string()],
        documentation: vec!["README.md".to_string()],
        ci_cd: vec![".github/workflows/".to_string()],
        recommendations: vec!["Add more tests".to_string()],
    })
}

struct DatabaseSchema {
    tables: Vec<serde_json::Value>,
    relationships: Vec<serde_json::Value>,
    indexes: Vec<serde_json::Value>,
    constraints: Vec<serde_json::Value>,
}

async fn inspect_database_schema(connection_string: &str, schema_name: Option<&str>) -> Result<DatabaseSchema, String> {
    // Implement database schema inspection
    Ok(DatabaseSchema {
        tables: vec![],
        relationships: vec![],
        indexes: vec![],
        constraints: vec![],
    })
}
```

### 3. HTTP Server for Cursor Integration

For more flexible integration, create an HTTP server:

```rust
use mcp_protocol_sdk::prelude::*;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone)]
struct AppState {
    mcp_server: Arc<McpServer>,
}

#[derive(Deserialize)]
struct ToolRequest {
    tool: String,
    params: serde_json::Value,
}

#[derive(Serialize)]
struct ToolResponse {
    success: bool,
    result: Option<serde_json::Value>,
    error: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut server = McpServer::new("cursor-http-server", "1.0.0");
    setup_development_tools(&mut server).await?;
    
    let state = AppState {
        mcp_server: Arc::new(server),
    };
    
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/tools", get(list_tools))
        .route("/tools/execute", post(execute_tool))
        .route("/analyze/project", post(analyze_project))
        .route("/analyze/file", post(analyze_file))
        .with_state(state);
    
    println!("Starting HTTP server on http://localhost:3000");
    
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await?;
    axum::serve(listener, app).await?;
    
    Ok(())
}

async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "server": "cursor-dev-tools",
        "version": "1.0.0"
    }))
}

async fn list_tools(State(state): State<AppState>) -> Json<Vec<serde_json::Value>> {
    // Return available tools
    Json(vec![
        serde_json::json!({
            "name": "analyze_code",
            "description": "Analyze code quality and complexity"
        }),
        serde_json::json!({
            "name": "find_todos",
            "description": "Find TODO comments in codebase"
        })
    ])
}

async fn execute_tool(
    State(state): State<AppState>,
    Json(request): Json<ToolRequest>
) -> Result<Json<ToolResponse>, StatusCode> {
    // Execute MCP tool and return result
    match state.mcp_server.call_tool(&request.tool, request.params).await {
        Ok(result) => Ok(Json(ToolResponse {
            success: true,
            result: Some(result),
            error: None,
        })),
        Err(e) => Ok(Json(ToolResponse {
            success: false,
            result: None,
            error: Some(e.to_string()),
        })),
    }
}

async fn analyze_project(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Analyze entire project
    let result = state.mcp_server
        .call_tool("analyze_project_structure", params)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(result))
}

async fn analyze_file(
    State(state): State<AppState>,
    Json(params): Json<serde_json::Value>
) -> Result<Json<serde_json::Value>, StatusCode> {
    // Analyze specific file
    let result = state.mcp_server
        .call_tool("analyze_complexity", params)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(result))
}

async fn setup_development_tools(server: &mut McpServer) -> Result<(), Box<dyn std::error::Error>> {
    // Add all your development tools here
    Ok(())
}
```

## Cursor Configuration

### 1. Extension Configuration

Create `.cursor/extensions.json`:

```json
{
  "recommendations": [
    "your-publisher.my-mcp-extension"
  ],
  "unwantedRecommendations": []
}
```

### 2. Workspace Settings

Create `.vscode/settings.json`:

```json
{
  "myMcp.serverPath": "/usr/local/bin/cursor-dev-server",
  "myMcp.autoStart": true,
  "myMcp.logLevel": "info"
}
```

### 3. Tasks Configuration

Create `.vscode/tasks.json`:

```json
{
    "version": "2.0.0",
    "tasks": [
        {
            "label": "Start MCP Server",
            "type": "shell",
            "command": "/usr/local/bin/cursor-dev-server",
            "args": ["--workspace", "${workspaceFolder}"],
            "group": "build",
            "presentation": {
                "echo": true,
                "reveal": "silent",
                "focus": false,
                "panel": "shared"
            },
            "runOptions": {
                "runOn": "folderOpen"
            }
        }
    ]
}
```

## Integration Patterns

### 1. Code Analysis Integration

```javascript
// In your VS Code extension
async function analyzeCurrentFile() {
    const editor = vscode.window.activeTextEditor;
    if (!editor) return;
    
    const filePath = editor.document.fileName;
    const language = editor.document.languageId;
    
    // Call MCP server
    const result = await callMcpTool('analyze_complexity', {
        file_path: filePath,
        language: language
    });
    
    // Show results in Cursor
    showAnalysisResults(result);
}

function showAnalysisResults(analysis) {
    const panel = vscode.window.createWebviewPanel(
        'codeAnalysis',
        'Code Analysis Results',
        vscode.ViewColumn.Beside,
        { enableScripts: true }
    );
    
    panel.webview.html = generateAnalysisHtml(analysis);
}
```

### 2. Project Intelligence

```javascript
async function enhanceProjectIntelligence() {
    const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
    if (!workspaceFolder) return;
    
    // Analyze project structure
    const structure = await callMcpTool('analyze_project_structure', {
        root_path: workspaceFolder.uri.fsPath
    });
    
    // Update Cursor's understanding
    await vscode.commands.executeCommand('cursor.updateProjectContext', {
        structure: structure,
        capabilities: await getMcpCapabilities()
    });
}
```

### 3. Database Schema Integration

```javascript
async function integrateDatabase() {
    const config = vscode.workspace.getConfiguration('database');
    const connectionString = config.get('connectionString');
    
    if (connectionString) {
        const schema = await callMcpTool('inspect_database_schema', {
            connection_string: connectionString
        });
        
        // Provide schema info to Cursor's AI
        await registerDatabaseContext(schema);
    }
}
```

## Advanced Features

### 1. Real-time Code Feedback

```rust
// In your MCP server
server.set_tool_handler("real_time_analysis", |params: HashMap<String, Value>| async move {
    let code = params.get("code")
        .and_then(|v| v.as_str())
        .ok_or("Missing code parameter")?;
    
    let language = params.get("language")
        .and_then(|v| v.as_str())
        .unwrap_or("unknown");
    
    // Perform real-time analysis
    let analysis = RealTimeAnalyzer::new()
        .analyze_code(code, language)
        .await?;
    
    Ok(json!({
        "syntax_errors": analysis.syntax_errors,
        "style_issues": analysis.style_issues,
        "performance_hints": analysis.performance_hints,
        "security_warnings": analysis.security_warnings
    }))
});
```

### 2. Context-Aware Suggestions

```rust
server.set_tool_handler("context_suggestions", |params: HashMap<String, Value>| async move {
    let cursor_position = params.get("cursor_position").unwrap();
    let surrounding_code = params.get("surrounding_code")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    
    let project_context = params.get("project_context").unwrap();
    
    let suggestions = ContextEngine::new()
        .with_project_context(project_context)
        .with_cursor_position(cursor_position)
        .generate_suggestions(surrounding_code)
        .await?;
    
    Ok(json!({
        "suggestions": suggestions,
        "confidence": suggestions.confidence_score(),
        "reasoning": suggestions.reasoning()
    }))
});
```

### 3. Intelligent Refactoring

```rust
server.set_tool_handler("suggest_refactoring", |params: HashMap<String, Value>| async move {
    let code_selection = params.get("code_selection")
        .and_then(|v| v.as_str())
        .ok_or("Missing code_selection parameter")?;
    
    let refactoring_type = params.get("refactoring_type")
        .and_then(|v| v.as_str())
        .unwrap_or("auto");
    
    let refactorings = RefactoringEngine::new()
        .analyze_code(code_selection)
        .suggest_refactorings(refactoring_type)
        .await?;
    
    Ok(json!({
        "available_refactorings": refactorings.options,
        "recommended": refactorings.recommended,
        "impact_analysis": refactorings.impact
    }))
});
```

## Testing Your Integration

### 1. Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mcp_protocol_sdk::testing::MockClient;
    
    #[tokio::test]
    async fn test_code_analysis() {
        let server = create_test_server().await;
        let client = MockClient::new();
        
        let result = client.call_tool("analyze_complexity", json!({
            "file_path": "test.rs",
            "language": "rust"
        })).await.unwrap();
        
        assert!(result["lines_of_code"].as_u64().unwrap() > 0);
    }
}
```

### 2. Integration Tests

```typescript
// In your VS Code extension tests
describe('Cursor MCP Integration', () => {
    test('should start MCP server on activation', async () => {
        await vscode.extensions.getExtension('your-publisher.my-mcp-extension')!.activate();
        
        // Wait for server to start
        await new Promise(resolve => setTimeout(resolve, 1000));
        
        // Test MCP capabilities
        const tools = await listMcpTools();
        expect(tools.length).toBeGreaterThan(0);
    });
    
    test('should analyze code on command', async () => {
        const document = await vscode.workspace.openTextDocument({
            content: 'fn main() { println!("Hello, world!"); }',
            language: 'rust'
        });
        
        await vscode.window.showTextDocument(document);
        await vscode.commands.executeCommand('myMcp.analyzeCurrentFile');
        
        // Check that analysis was performed
        // Add your assertions here
    });
});
```

## Deployment

### 1. Extension Packaging

```bash
# Install vsce
npm install -g vsce

# Package extension
vsce package

# Install locally
code --install-extension my-mcp-extension-1.0.0.vsix
```

### 2. Binary Distribution

```bash
# Build optimized binary
cargo build --release --features stdio,http

# Create distribution package
mkdir -p dist/cursor-dev-tools
cp target/release/cursor-dev-server dist/cursor-dev-tools/
cp README.md dist/cursor-dev-tools/
cp install.sh dist/cursor-dev-tools/

# Create installer
tar -czf cursor-dev-tools-1.0.0.tar.gz -C dist cursor-dev-tools
```

### 3. Installation Script

Create `install.sh`:

```bash
#!/bin/bash
set -e

INSTALL_DIR="/usr/local/bin"
BINARY_NAME="cursor-dev-server"

echo "Installing Cursor Development Tools..."

# Copy binary
sudo cp "$BINARY_NAME" "$INSTALL_DIR/"
sudo chmod +x "$INSTALL_DIR/$BINARY_NAME"

echo "Installation complete!"
echo "Configure in Cursor: Set myMcp.serverPath to $INSTALL_DIR/$BINARY_NAME"
```

## Best Practices

### 1. Performance Optimization

- Cache analysis results
- Use async operations
- Implement request debouncing
- Optimize file I/O operations

### 2. Error Handling

- Provide clear error messages
- Implement graceful fallbacks
- Log errors for debugging
- Handle network timeouts

### 3. Security Considerations

- Validate all file paths
- Sanitize user inputs
- Implement access controls
- Audit tool capabilities

## Troubleshooting

### Common Issues

1. **Server won't start**: Check binary permissions and path
2. **Tools not appearing**: Verify extension activation
3. **Slow performance**: Implement caching and optimize algorithms
4. **Connection errors**: Check STDIO communication

### Debug Mode

Enable debug logging in your server:

```rust
use tracing::{info, debug, error};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::fmt()
        .with_env_filter("debug")
        .init();
    
    debug!("Starting Cursor dev server in debug mode");
    // ... rest of your server code
}
```

## Next Steps

1. **Build your development tools** using the patterns above
2. **Create a VS Code extension** for seamless integration
3. **Test with real projects** to validate functionality
4. **Share with the community** to get feedback

Your custom development tools are now available in Cursor! 🚀
