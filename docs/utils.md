# Utils Documentation

The MCP Rust SDK provides a comprehensive set of utility functions to help you work with URIs, handle common operations, and integrate the SDK into your applications. This documentation covers all available utilities with examples and best practices.

## Overview

The utils module contains:
- **URI Handling**: Parsing, validation, and manipulation of URIs
- **Parameter Processing**: Query string parsing and URL parameter handling
- **Content Type Detection**: MIME type guessing and file extension handling
- **Encoding/Decoding**: Percent encoding and decoding for URI components

## URI Utilities

### `parse_uri_with_params`

Parse a URI and extract query parameters into a separate map.

```rust
use mcp_protocol_sdk::utils::parse_uri_with_params;

let (base_uri, params) = parse_uri_with_params("https://api.example.com/data?format=json&limit=10")?;
assert_eq!(base_uri, "https://api.example.com/data");
assert_eq!(params.get("format"), Some(&"json".to_string()));
assert_eq!(params.get("limit"), Some(&"10".to_string()));
```

**Supported URI formats:**
- Full URIs: `https://example.com/path?param=value`
- Absolute paths: `/api/data?format=json`
- Relative paths: `data/file.json?version=1`

**Returns:**
- `(String, HashMap<String, String>)` - Base URI and parameters map
- `McpError::InvalidUri` - If the URI is malformed

### `parse_query_string`

Parse a query string into a parameters map.

```rust
use mcp_protocol_sdk::utils::parse_query_string;

let params = parse_query_string("name=John&age=30&city=New+York")?;
assert_eq!(params.get("name"), Some(&"John".to_string()));
assert_eq!(params.get("age"), Some(&"30".to_string()));
assert_eq!(params.get("city"), Some(&"New York".to_string()));
```

**Features:**
- Handles URL encoding (`+` to space, `%20` to space)
- Supports empty values: `key=` results in `key: ""`
- Supports keys without values: `flag` results in `flag: ""`
- Percent decoding for special characters

### `validate_uri`

Validate that a string is a properly formatted URI.

```rust
use mcp_protocol_sdk::utils::validate_uri;

// Valid URIs
validate_uri("https://example.com")?;                // Full URI
validate_uri("/absolute/path")?;                     // Absolute path
validate_uri("relative/path/file.txt")?;             // Relative path
validate_uri("file:///local/file.txt")?;             // File URI

// Invalid URIs (will return errors)
validate_uri("")?;                                   // Empty string
validate_uri("invalid\0uri")?;                       // Contains null byte
validate_uri("bad\nline\rbreaks")?;                  // Contains line breaks
```

**Validation rules:**
- Cannot be empty
- Cannot contain null bytes (`\0`)
- Cannot contain line breaks (`\n`, `\r`)
- Full URIs must be parseable by the `url` crate
- Relative paths allow most characters

### `normalize_uri`

Normalize a URI to a standard form by removing redundant elements.

```rust
use mcp_protocol_sdk::utils::normalize_uri;

// Remove duplicate slashes
let normalized = normalize_uri("https://example.com//api//data/")?;
assert_eq!(normalized, "https://example.com/api/data");

// Path normalization
let normalized = normalize_uri("/path//to//file/")?;
assert_eq!(normalized, "/path/to/file");

// Root paths remain unchanged
let normalized = normalize_uri("/")?;
assert_eq!(normalized, "/");
```

**Normalization features:**
- Removes duplicate slashes (`//` → `/`)
- Removes trailing slashes (except for root paths)
- Preserves URI scheme and authority
- Handles both full URIs and paths

### `join_uri`

Join a base URI with a relative path to create a complete URI.

```rust
use mcp_protocol_sdk::utils::join_uri;

// Join with full URI base
let joined = join_uri("https://api.example.com/v1", "users/123")?;
assert_eq!(joined, "https://api.example.com/v1/users/123");

// Join with path base
let joined = join_uri("/api/v1", "data/records")?;
assert_eq!(joined, "/api/v1/data/records");

// Absolute relative paths override base
let joined = join_uri("https://example.com/old", "/new/path")?;
assert_eq!(joined, "/new/path");

// Full URI relatives override base completely
let joined = join_uri("https://old.com", "https://new.com/path")?;
assert_eq!(joined, "https://new.com/path");
```

**Join behavior:**
- If relative path is absolute (`/path`), it replaces the base path
- If relative path is a full URI, it completely replaces the base
- Otherwise, the relative path is appended to the base
- Automatic slash handling between components

## Encoding and Decoding

### `percent_encode`

Encode a string for safe use in URI components.

```rust
use mcp_protocol_sdk::utils::percent_encode;

let encoded = percent_encode("hello world!@#$%");
assert_eq!(encoded, "hello+world%21%40%23%24%25");

let encoded = percent_encode("user@example.com");
assert_eq!(encoded, "user%40example.com");
```

**Encoding rules:**
- Alphanumeric characters (`A-Z`, `a-z`, `0-9`) are preserved
- Unreserved characters (`-`, `_`, `.`, `~`) are preserved
- Spaces are converted to `+`
- All other characters are percent-encoded (`%XX`)

### `percent_decode`

Decode a percent-encoded string.

```rust
use mcp_protocol_sdk::utils::percent_decode;

let decoded = percent_decode("hello+world%21%40%23%24%25")?;
assert_eq!(decoded, "hello world!@#$%");

let decoded = percent_decode("user%40example.com")?;
assert_eq!(decoded, "user@example.com");
```

**Decoding rules:**
- `+` characters are converted to spaces
- `%XX` sequences are converted to their byte values
- Invalid hex sequences return `McpError::InvalidUri`
- Incomplete sequences (missing hex digits) return errors

## File and Content Type Utilities

### `get_uri_extension`

Extract the file extension from a URI.

```rust
use mcp_protocol_sdk::utils::get_uri_extension;

assert_eq!(get_uri_extension("file.txt"), Some("txt".to_string()));
assert_eq!(get_uri_extension("https://example.com/data.JSON"), Some("json".to_string()));
assert_eq!(get_uri_extension("/path/to/archive.tar.gz"), Some("gz".to_string()));
assert_eq!(get_uri_extension("no-extension"), None);
assert_eq!(get_uri_extension("/path/to/dir/"), None);
```

**Features:**
- Works with full URIs and paths
- Returns lowercase extensions
- Returns the last extension for compound extensions (`.tar.gz` → `gz`)
- Returns `None` if no extension is found
- Ignores extensions in directory names

### `guess_mime_type`

Guess the MIME type based on the file extension in a URI.

```rust
use mcp_protocol_sdk::utils::guess_mime_type;

assert_eq!(guess_mime_type("document.pdf"), Some("application/pdf".to_string()));
assert_eq!(guess_mime_type("image.PNG"), Some("image/png".to_string()));
assert_eq!(guess_mime_type("data.json"), Some("application/json".to_string()));
assert_eq!(guess_mime_type("style.css"), Some("text/css".to_string()));
assert_eq!(guess_mime_type("unknown.xyz"), None);
```

**Supported file types:**

| Extension | MIME Type |
|-----------|-----------|
| txt | text/plain |
| html, htm | text/html |
| css | text/css |
| js | application/javascript |
| json | application/json |
| xml | application/xml |
| pdf | application/pdf |
| zip | application/zip |
| png | image/png |
| jpg, jpeg | image/jpeg |
| gif | image/gif |
| webp | image/webp |
| svg | image/svg+xml |
| mp3 | audio/mpeg |
| wav | audio/wav |
| mp4 | video/mp4 |
| webm | video/webm |
| csv | text/csv |
| md | text/markdown |
| yaml, yml | application/x-yaml |
| toml | application/toml |

## Practical Examples

### Resource URI Processing

```rust
use mcp_protocol_sdk::utils::{parse_uri_with_params, normalize_uri, guess_mime_type};

// Process a resource URI from an MCP request
async fn process_resource_uri(uri: &str) -> Result<ResourceInfo, McpError> {
    // Parse and validate the URI
    let (base_uri, params) = parse_uri_with_params(uri)?;
    let normalized_uri = normalize_uri(&base_uri)?;
    
    // Extract metadata
    let mime_type = guess_mime_type(&normalized_uri);
    let format = params.get("format").cloned();
    let version = params.get("version").cloned();
    
    Ok(ResourceInfo {
        uri: normalized_uri,
        mime_type,
        format,
        version,
        params,
    })
}

struct ResourceInfo {
    uri: String,
    mime_type: Option<String>,
    format: Option<String>,
    version: Option<String>,
    params: std::collections::HashMap<String, String>,
}
```

### URI Builder

```rust
use mcp_protocol_sdk::utils::{join_uri, percent_encode};

struct UriBuilder {
    base: String,
    params: Vec<(String, String)>,
}

impl UriBuilder {
    fn new(base: &str) -> Self {
        Self {
            base: base.to_string(),
            params: Vec::new(),
        }
    }
    
    fn path(mut self, path: &str) -> Result<Self, McpError> {
        self.base = join_uri(&self.base, path)?;
        Ok(self)
    }
    
    fn param(mut self, key: &str, value: &str) -> Self {
        self.params.push((key.to_string(), value.to_string()));
        self
    }
    
    fn build(self) -> String {
        if self.params.is_empty() {
            return self.base;
        }
        
        let query_string: Vec<String> = self.params
            .into_iter()
            .map(|(k, v)| format!("{}={}", percent_encode(&k), percent_encode(&v)))
            .collect();
            
        format!("{}?{}", self.base, query_string.join("&"))
    }
}

// Usage
let uri = UriBuilder::new("https://api.example.com")
    .path("v1/users")?
    .param("limit", "10")
    .param("format", "json")
    .param("query", "active users")
    .build();
    
assert_eq!(uri, "https://api.example.com/v1/users?limit=10&format=json&query=active+users");
```

### HTTP Client with URI Utilities

```rust
use mcp_protocol_sdk::utils::{validate_uri, normalize_uri, parse_uri_with_params};
use reqwest::Client;

struct HttpResourceHandler {
    client: Client,
    base_url: String,
}

impl HttpResourceHandler {
    fn new(base_url: &str) -> Result<Self, McpError> {
        validate_uri(base_url)?;
        let normalized_base = normalize_uri(base_url)?;
        
        Ok(Self {
            client: Client::new(),
            base_url: normalized_base,
        })
    }
    
    async fn fetch_resource(&self, uri: &str) -> Result<String, McpError> {
        // Parse the URI and extract parameters
        let (resource_path, params) = parse_uri_with_params(uri)?;
        
        // Build the full URL
        let full_url = join_uri(&self.base_url, &resource_path)?;
        let mut url = reqwest::Url::parse(&full_url)
            .map_err(|e| McpError::InvalidUri(e.to_string()))?;
            
        // Add query parameters
        for (key, value) in params {
            url.query_pairs_mut().append_pair(&key, &value);
        }
        
        // Make the request
        let response = self.client
            .get(url)
            .send()
            .await
            .map_err(|e| McpError::Transport(e.to_string()))?;
            
        let content = response
            .text()
            .await
            .map_err(|e| McpError::Transport(e.to_string()))?;
            
        Ok(content)
    }
}
```

### File System Resource Handler

```rust
use mcp_protocol_sdk::utils::{validate_uri, normalize_uri, guess_mime_type, get_uri_extension};
use std::path::{Path, PathBuf};
use tokio::fs;

struct FileSystemHandler {
    root_path: PathBuf,
}

impl FileSystemHandler {
    fn new(root_path: &str) -> Result<Self, McpError> {
        let path = PathBuf::from(root_path);
        if !path.exists() || !path.is_dir() {
            return Err(McpError::InvalidUri("Root path must be an existing directory".to_string()));
        }
        
        Ok(Self { root_path: path })
    }
    
    async fn read_file(&self, uri: &str) -> Result<ResourceContent, McpError> {
        // Validate and normalize the URI
        validate_uri(uri)?;
        let normalized_uri = normalize_uri(uri)?;
        
        // Parse parameters if any
        let (file_path, params) = parse_uri_with_params(&normalized_uri)?;
        
        // Convert URI to file system path
        let relative_path = if file_path.starts_with("file://") {
            &file_path[7..]
        } else if file_path.starts_with('/') {
            &file_path[1..]
        } else {
            &file_path
        };
        
        let full_path = self.root_path.join(relative_path);
        
        // Security check - ensure path is within root
        if !full_path.starts_with(&self.root_path) {
            return Err(McpError::InvalidUri("Path traversal not allowed".to_string()));
        }
        
        // Read the file
        let content = fs::read_to_string(&full_path)
            .await
            .map_err(|e| McpError::ResourceNotFound(format!("Cannot read file: {}", e)))?;
            
        // Determine MIME type
        let mime_type = guess_mime_type(&normalized_uri);
        
        // Check for encoding parameter
        let encoding = params.get("encoding").cloned().unwrap_or_else(|| "utf-8".to_string());
        
        Ok(ResourceContent {
            uri: normalized_uri,
            mime_type,
            text: Some(content),
            blob: None,
        })
    }
    
    async fn list_directory(&self, uri: &str) -> Result<Vec<ResourceInfo>, McpError> {
        validate_uri(uri)?;
        let normalized_uri = normalize_uri(uri)?;
        
        // Convert URI to directory path
        let dir_path = if normalized_uri.starts_with("file://") {
            self.root_path.join(&normalized_uri[7..])
        } else if normalized_uri.starts_with('/') {
            self.root_path.join(&normalized_uri[1..])
        } else {
            self.root_path.join(&normalized_uri)
        };
        
        // Security check
        if !dir_path.starts_with(&self.root_path) {
            return Err(McpError::InvalidUri("Path traversal not allowed".to_string()));
        }
        
        let mut entries = fs::read_dir(&dir_path)
            .await
            .map_err(|e| McpError::ResourceNotFound(format!("Cannot read directory: {}", e)))?;
            
        let mut resources = Vec::new();
        
        while let Some(entry) = entries.next_entry()
            .await
            .map_err(|e| McpError::Internal(e.to_string()))? {
            
            let file_name = entry.file_name().to_string_lossy().to_string();
            let file_path = entry.path();
            
            // Create URI for this entry
            let relative_path = file_path.strip_prefix(&self.root_path)
                .map_err(|_| McpError::Internal("Path calculation error".to_string()))?;
                
            let entry_uri = format!("file:///{}", relative_path.to_string_lossy().replace('\\', "/"));
            
            // Get metadata
            let metadata = entry.metadata()
                .await
                .map_err(|e| McpError::Internal(e.to_string()))?;
                
            let mime_type = if metadata.is_dir() {
                Some("inode/directory".to_string())
            } else {
                guess_mime_type(&entry_uri)
            };
            
            resources.push(ResourceInfo {
                uri: entry_uri,
                name: file_name,
                description: if metadata.is_dir() {
                    Some("Directory".to_string())
                } else {
                    Some(format!("File ({} bytes)", metadata.len()))
                },
                mime_type,
            });
        }
        
        resources.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(resources)
    }
}

struct ResourceContent {
    uri: String,
    mime_type: Option<String>,
    text: Option<String>,
    blob: Option<Vec<u8>>,
}

struct ResourceInfo {
    uri: String,
    name: String,
    description: Option<String>,
    mime_type: Option<String>,
}
```

### Database Resource Handler with URI Parameters

```rust
use mcp_protocol_sdk::utils::{parse_uri_with_params, percent_decode};
use sqlx::{PgPool, Row};

struct DatabaseHandler {
    pool: PgPool,
}

impl DatabaseHandler {
    async fn query_resource(&self, uri: &str) -> Result<String, McpError> {
        // Parse database URI: db://table?filter=value&limit=10&offset=0
        let (table_path, params) = parse_uri_with_params(uri)?;
        
        if !table_path.starts_with("db://") {
            return Err(McpError::InvalidUri("Database URI must start with db://".to_string()));
        }
        
        let table_name = &table_path[5..]; // Remove "db://" prefix
        
        // Validate table name (prevent SQL injection)
        if !table_name.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(McpError::InvalidUri("Invalid table name".to_string()));
        }
        
        // Build SQL query from parameters
        let mut query = format!("SELECT * FROM {}", table_name);
        let mut conditions = Vec::new();
        let mut limit_clause = String::new();
        let mut offset_clause = String::new();
        
        for (key, value) in params {
            match key.as_str() {
                "limit" => {
                    let limit: u32 = value.parse()
                        .map_err(|_| McpError::ValidationError("Invalid limit value".to_string()))?;
                    limit_clause = format!(" LIMIT {}", limit);
                }
                "offset" => {
                    let offset: u32 = value.parse()
                        .map_err(|_| McpError::ValidationError("Invalid offset value".to_string()))?;
                    offset_clause = format!(" OFFSET {}", offset);
                }
                "order_by" => {
                    // Validate column name
                    if value.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        query.push_str(&format!(" ORDER BY {}", value));
                    }
                }
                _ => {
                    // Treat as filter condition
                    if key.chars().all(|c| c.is_alphanumeric() || c == '_') {
                        conditions.push(format!("{} = $1", key));
                    }
                }
            }
        }
        
        // Add WHERE clause if there are conditions
        if !conditions.is_empty() {
            query.push_str(&format!(" WHERE {}", conditions.join(" AND ")));
        }
        
        // Add LIMIT and OFFSET
        query.push_str(&limit_clause);
        query.push_str(&offset_clause);
        
        // Execute query (simplified - in practice, use proper parameter binding)
        let rows = sqlx::query(&query)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| McpError::Internal(format!("Database query failed: {}", e)))?;
            
        // Convert rows to JSON
        let mut results = Vec::new();
        for row in rows {
            let mut record = serde_json::Map::new();
            
            for (i, column) in row.columns().iter().enumerate() {
                let column_name = column.name();
                let value: Option<String> = row.try_get(i).ok();
                record.insert(column_name.to_string(), serde_json::Value::String(value.unwrap_or_default()));
            }
            
            results.push(serde_json::Value::Object(record));
        }
        
        Ok(serde_json::to_string_pretty(&results)
            .map_err(|e| McpError::Internal(format!("JSON serialization failed: {}", e)))?)
    }
}
```

## Testing Utilities

The utils module includes comprehensive tests. Here are examples of how to test your own URI-handling code:

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use mcp_protocol_sdk::utils::*;

    #[test]
    fn test_uri_roundtrip() {
        let original = "https://example.com/path?name=John Doe&age=30&city=New York";
        let (base_uri, params) = parse_uri_with_params(original).unwrap();
        
        // Reconstruct URI
        let mut reconstructed = base_uri;
        if !params.is_empty() {
            let query_string: Vec<String> = params
                .into_iter()
                .map(|(k, v)| format!("{}={}", percent_encode(&k), percent_encode(&v)))
                .collect();
            reconstructed.push('?');
            reconstructed.push_str(&query_string.join("&"));
        }
        
        // The reconstructed URI should be functionally equivalent
        let (base2, params2) = parse_uri_with_params(&reconstructed).unwrap();
        assert_eq!(base_uri, base2);
        // Note: parameter order might differ, so check individual values
    }

    #[test]
    fn test_path_security() {
        // Test that path traversal attempts are handled properly
        assert!(validate_uri("../../../etc/passwd").is_ok()); // Validation allows it
        // But your handler should reject it during processing
        
        let handler = FileSystemHandler::new("/safe/root").unwrap();
        let result = handler.read_file("../../../etc/passwd").await;
        assert!(result.is_err()); // Should be rejected by security check
    }

    #[test]
    fn test_mime_type_detection() {
        assert_eq!(guess_mime_type("data.json"), Some("application/json".to_string()));
        assert_eq!(guess_mime_type("DATA.JSON"), Some("application/json".to_string()));
        assert_eq!(guess_mime_type("file.unknown"), None);
        
        // Test with URIs
        assert_eq!(
            guess_mime_type("https://api.example.com/data.json?version=1"),
            Some("application/json".to_string())
        );
    }

    #[tokio::test]
    async fn test_resource_handler_integration() {
        let handler = HttpResourceHandler::new("https://httpbin.org").unwrap();
        
        // Test with parameters
        let result = handler.fetch_resource("/get?test=value").await;
        assert!(result.is_ok());
        
        // Test normalization
        let result = handler.fetch_resource("//get//").await;
        assert!(result.is_ok()); // Should normalize to /get
    }
}
```

## Error Handling

All utility functions return `McpResult<T>` which is an alias for `Result<T, McpError>`. Common error types:

- **`McpError::InvalidUri`**: Malformed URIs, invalid characters, or parsing failures
- **`McpError::ValidationError`**: Parameter validation failures
- **`McpError::ResourceNotFound`**: File or resource not found
- **`McpError::Internal`**: Internal processing errors

```rust
use mcp_protocol_sdk::core::error::{McpError, McpResult};

fn handle_uri_error(result: McpResult<String>) {
    match result {
        Ok(uri) => println!("Processed URI: {}", uri),
        Err(McpError::InvalidUri(msg)) => {
            eprintln!("Invalid URI: {}", msg);
            // Handle invalid URI - maybe use a default or ask user to correct
        }
        Err(McpError::ValidationError(msg)) => {
            eprintln!("Validation failed: {}", msg);
            // Handle validation error - invalid parameters
        }
        Err(e) => {
            eprintln!("Unexpected error: {}", e);
            // Handle other errors
        }
    }
}
```

## Best Practices

### URI Validation

Always validate URIs before processing:

```rust
// Good: Validate before use
if let Ok(normalized) = normalize_uri(user_input) {
    process_uri(&normalized);
} else {
    return_error("Invalid URI format");
}

// Better: Use a validation function
fn validate_and_process_uri(uri: &str) -> McpResult<String> {
    validate_uri(uri)?;
    let normalized = normalize_uri(uri)?;
    Ok(process_uri(&normalized))
}
```

### Parameter Handling

Be defensive when handling URI parameters:

```rust
let (base_uri, params) = parse_uri_with_params(uri)?;

// Good: Provide defaults for optional parameters
let limit: usize = params.get("limit")
    .and_then(|v| v.parse().ok())
    .unwrap_or(10)
    .min(100); // Cap at maximum

let format = params.get("format")
    .map(|s| s.to_lowercase())
    .unwrap_or_else(|| "json".to_string());

// Good: Validate enum values
let sort_order = match params.get("order").map(|s| s.as_str()) {
    Some("asc") | Some("desc") | None => params.get("order").cloned().unwrap_or_else(|| "asc".to_string()),
    Some(invalid) => return Err(McpError::ValidationError(format!("Invalid sort order: {}", invalid))),
};
```

### Security Considerations

When building file or database handlers:

```rust
// Always validate paths to prevent directory traversal
fn is_safe_path(path: &Path, root: &Path) -> bool {
    path.canonicalize()
        .map(|p| p.starts_with(root))
        .unwrap_or(false)
}

// Sanitize SQL identifiers
fn is_valid_identifier(name: &str) -> bool {
    !name.is_empty() 
        && name.len() <= 64 
        && name.chars().all(|c| c.is_alphanumeric() || c == '_')
        && !name.starts_with(char::is_numeric)
}

// Limit resource sizes
const MAX_FILE_SIZE: u64 = 10 * 1024 * 1024; // 10MB
const MAX_QUERY_RESULTS: usize = 1000;
```

### Performance Tips

```rust
// Cache MIME type mappings for better performance
use std::collections::HashMap;
use once_cell::sync::Lazy;

static MIME_TYPES: Lazy<HashMap<&str, &str>> = Lazy::new(|| {
    let mut map = HashMap::new();
    map.insert("json", "application/json");
    map.insert("xml", "application/xml");
    map.insert("html", "text/html");
    // ... more mappings
    map
});

pub fn fast_guess_mime_type(uri: &str) -> Option<String> {
    get_uri_extension(uri)
        .and_then(|ext| MIME_TYPES.get(ext.as_str()))
        .map(|mime| mime.to_string())
}

// Pre-compile regex for validation if needed
use regex::Regex;
use once_cell::sync::Lazy;

static URI_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z][a-zA-Z0-9+.-]*:").unwrap()
});

pub fn has_scheme(uri: &str) -> bool {
    URI_PATTERN.is_match(uri)
}
```

## Integration with MCP Components

The utils module is designed to work seamlessly with other MCP SDK components:

### With Resource Handlers

```rust
use mcp_protocol_sdk::{
    core::resource::ResourceHandler,
    protocol::types::{ResourceInfo, ResourceContent},
    utils::*,
};

#[async_trait]
impl ResourceHandler for MyResourceHandler {
    async fn read(&self, uri: &str, params: &HashMap<String, String>) -> McpResult<Vec<ResourceContent>> {
        // Use utils for URI processing
        let normalized_uri = normalize_uri(uri)?;
        let mime_type = guess_mime_type(&normalized_uri);
        
        // Your resource reading logic here
        let content = self.read_resource_content(&normalized_uri, params).await?;
        
        Ok(vec![ResourceContent {
            uri: normalized_uri,
            mime_type,
            text: Some(content),
            blob: None,
        }])
    }
}
```

### With Tool Handlers

```rust
use mcp_protocol_sdk::{
    core::tool::ToolHandler,
    protocol::types::{ToolResult, Content},
    utils::*,
};

#[async_trait]
impl ToolHandler for MyToolHandler {
    async fn call(&self, arguments: HashMap<String, Value>) -> McpResult<ToolResult> {
        // Extract and validate URI argument
        let uri = arguments.get("uri")
            .and_then(|v| v.as_str())
            .ok_or_else(|| McpError::ValidationError("Missing uri parameter".to_string()))?;
            
        // Use utils for processing
        validate_uri(uri)?;
        let (base_uri, params) = parse_uri_with_params(uri)?;
        
        // Your tool logic here
        let result = self.process_uri(&base_uri, &params).await?;
        
        Ok(ToolResult {
            content: vec![Content::text(result)],
            is_error: None,
        })
    }
}
```

## See Also

- [Examples Documentation](examples.md) - See utils in action
- [API Reference](api-reference.md) - Complete API documentation
- [Architecture Guide](architecture.md) - Understanding the system design
- [MCP Specification](https://modelcontextprotocol.io/) - Official protocol documentation
