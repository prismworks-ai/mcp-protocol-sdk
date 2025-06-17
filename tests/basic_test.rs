use mcp_protocol_sdk::protocol::types::*;
use serde_json::json;

#[test]
fn test_basic_protocol_types() {
    // Test that we can create and serialize basic types

    // Test Content
    let text_content = Content::text("Hello, world!");
    let serialized = serde_json::to_string(&text_content).unwrap();
    assert!(serialized.contains("Hello, world!"));

    // Test Tool
    let tool = Tool::new("test_tool", "A test tool");
    assert_eq!(tool.name, "test_tool");

    // Test JSON-RPC Request
    let request = JsonRpcRequest::new(
        json!(1),
        "test_method".to_string(),
        Some(json!({"test": "value"})),
    )
    .unwrap();
    assert_eq!(request.method, "test_method");
    assert_eq!(request.id, json!(1));

    // Test JSON-RPC Response
    let response = JsonRpcResponse::success(json!(1), json!({"result": "success"})).unwrap();
    assert_eq!(response.id, json!(1));

    // Test that serialization works
    let request_json = serde_json::to_string(&request).unwrap();
    assert!(request_json.contains("test_method"));

    let response_json = serde_json::to_string(&response).unwrap();
    assert!(response_json.contains("success"));

    println!("✅ All basic protocol types work correctly!");
}

#[test]
fn test_2025_features() {
    // Test new 2025-03-26 features

    // Test Audio content
    let audio_content = Content::audio("base64audiodata", "audio/wav");
    let serialized = serde_json::to_value(&audio_content).unwrap();
    assert_eq!(serialized["type"], "audio");
    assert_eq!(serialized["data"], "base64audiodata");
    assert_eq!(serialized["mimeType"], "audio/wav");

    // Test Resource content
    let resource_content = Content::resource("file:///test.txt");
    let serialized = serde_json::to_value(&resource_content).unwrap();
    assert_eq!(serialized["type"], "resource");
    assert_eq!(serialized["resource"]["uri"], "file:///test.txt");

    // Test Annotations
    let annotations = Annotations::new()
        .read_only()
        .with_danger_level(DangerLevel::Safe);

    assert_eq!(annotations.read_only, Some(true));
    assert_eq!(annotations.danger, Some(DangerLevel::Safe));

    // Test Tool with annotations
    let tool = Tool::new("safe_tool", "A safe tool").with_annotations(annotations);

    assert!(tool.annotations.is_some());

    println!("✅ All 2025-03-26 features work correctly!");
}

#[test]
fn test_server_capabilities() {
    // Test that we can create and work with server capabilities
    let capabilities = ServerCapabilities {
        tools: Some(ToolsCapability {
            list_changed: Some(true),
        }),
        resources: Some(ResourcesCapability {
            subscribe: Some(true),
            list_changed: Some(true),
        }),
        completions: Some(CompletionsCapability::default()),
        ..Default::default()
    };

    let serialized = serde_json::to_value(&capabilities).unwrap();
    assert_eq!(serialized["tools"]["listChanged"], true);
    assert_eq!(serialized["resources"]["subscribe"], true);

    println!("✅ Server capabilities work correctly!");
}

#[test]
fn test_constants() {
    // Test that protocol constants are correct
    assert_eq!(LATEST_PROTOCOL_VERSION, "2025-03-26");
    assert_eq!(JSONRPC_VERSION, "2.0");
    assert_eq!(PROTOCOL_VERSION, LATEST_PROTOCOL_VERSION);

    println!("✅ Protocol constants are correct!");
}
