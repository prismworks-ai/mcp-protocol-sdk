# 🔌 Other MCP Clients Integration

Integrate your MCP servers with various AI clients and development tools beyond Claude Desktop, Cursor, and VS Code.

## Overview

The MCP protocol is designed to be client-agnostic, allowing integration with any AI system or development tool. This guide covers integration patterns for:

- Web-based AI interfaces
- Custom AI applications
- IDE plugins
- Command-line tools
- Mobile applications
- Browser extensions

## Generic Integration Patterns

### 1. HTTP Client Integration

For web applications and services that prefer HTTP communication:

```javascript
// JavaScript/TypeScript HTTP client
class McpHttpClient {
    constructor(serverUrl) {
        this.serverUrl = serverUrl;
        this.requestId = 1;
    }

    async initialize() {
        const response = await fetch(`${this.serverUrl}/mcp`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                jsonrpc: '2.0',
                id: this.requestId++,
                method: 'initialize',
                params: {
                    protocolVersion: '2024-11-05',
                    capabilities: {
                        tools: {},
                        resources: {},
                        prompts: {}
                    },
                    clientInfo: {
                        name: 'custom-client',
                        version: '1.0.0'
                    }
                }
            })
        });

        return await response.json();
    }

    async callTool(toolName, params) {
        const response = await fetch(`${this.serverUrl}/mcp`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                jsonrpc: '2.0',
                id: this.requestId++,
                method: 'tools/call',
                params: {
                    name: toolName,
                    arguments: params
                }
            })
        });

        const result = await response.json();
        if (result.error) {
            throw new Error(result.error.message);
        }
        
        return result.result;
    }

    async listTools() {
        const response = await fetch(`${this.serverUrl}/mcp`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                jsonrpc: '2.0',
                id: this.requestId++,
                method: 'tools/list'
            })
        });

        const result = await response.json();
        return result.result.tools || [];
    }

    async readResource(uri) {
        const response = await fetch(`${this.serverUrl}/mcp`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                jsonrpc: '2.0',
                id: this.requestId++,
                method: 'resources/read',
                params: { uri }
            })
        });

        const result = await response.json();
        return result.result;
    }
}

// Usage in a web application
async function initializeAI() {
    const mcpClient = new McpHttpClient('http://localhost:3000');
    await mcpClient.initialize();
    
    // Get available tools
    const tools = await mcpClient.listTools();
    console.log('Available tools:', tools);
    
    // Use tools in AI workflow
    for (const tool of tools) {
        registerAITool(tool.name, async (params) => {
            return await mcpClient.callTool(tool.name, params);
        });
    }
}
```

### 2. WebSocket Client Integration

For real-time applications requiring bidirectional communication:

```javascript
class McpWebSocketClient {
    constructor(serverUrl) {
        this.serverUrl = serverUrl;
        this.requestId = 1;
        this.pendingRequests = new Map();
        this.ws = null;
    }

    async connect() {
        return new Promise((resolve, reject) => {
            this.ws = new WebSocket(this.serverUrl);
            
            this.ws.onopen = async () => {
                await this.initialize();
                resolve();
            };
            
            this.ws.onmessage = (event) => {
                const message = JSON.parse(event.data);
                this.handleMessage(message);
            };
            
            this.ws.onerror = (error) => {
                reject(error);
            };
            
            this.ws.onclose = () => {
                console.log('WebSocket connection closed');
            };
        });
    }

    handleMessage(message) {
        if (message.id && this.pendingRequests.has(message.id)) {
            const { resolve, reject } = this.pendingRequests.get(message.id);
            this.pendingRequests.delete(message.id);
            
            if (message.error) {
                reject(new Error(message.error.message));
            } else {
                resolve(message.result);
            }
        } else if (message.method) {
            // Handle notifications from server
            this.handleNotification(message);
        }
    }

    async sendRequest(method, params = {}) {
        return new Promise((resolve, reject) => {
            const id = this.requestId++;
            this.pendingRequests.set(id, { resolve, reject });
            
            this.ws.send(JSON.stringify({
                jsonrpc: '2.0',
                id,
                method,
                params
            }));
            
            // Set timeout
            setTimeout(() => {
                if (this.pendingRequests.has(id)) {
                    this.pendingRequests.delete(id);
                    reject(new Error('Request timeout'));
                }
            }, 10000);
        });
    }

    async initialize() {
        return await this.sendRequest('initialize', {
            protocolVersion: '2024-11-05',
            capabilities: {
                tools: {},
                resources: {},
                prompts: {}
            },
            clientInfo: {
                name: 'websocket-client',
                version: '1.0.0'
            }
        });
    }

    async callTool(toolName, params) {
        return await this.sendRequest('tools/call', {
            name: toolName,
            arguments: params
        });
    }

    handleNotification(message) {
        switch (message.method) {
            case 'notifications/tools/list_changed':
                this.onToolsChanged();
                break;
            case 'notifications/resources/list_changed':
                this.onResourcesChanged();
                break;
            default:
                console.log('Unknown notification:', message);
        }
    }

    onToolsChanged() {
        // Refresh available tools
        this.listTools().then(tools => {
            this.updateAICapabilities(tools);
        });
    }

    onResourcesChanged() {
        // Refresh available resources
        console.log('Resources changed, refreshing...');
    }
}
```

### 3. Python Client Integration

For Python-based AI applications:

```python
import asyncio
import json
import websockets
from typing import Dict, Any, Optional, List

class McpPythonClient:
    def __init__(self, server_url: str):
        self.server_url = server_url
        self.request_id = 1
        self.pending_requests = {}
        self.websocket = None

    async def connect(self):
        self.websocket = await websockets.connect(self.server_url)
        
        # Start message handler
        asyncio.create_task(self.message_handler())
        
        # Initialize connection
        await self.initialize()

    async def message_handler(self):
        async for message in self.websocket:
            data = json.loads(message)
            await self.handle_message(data)

    async def handle_message(self, message: Dict[str, Any]):
        if 'id' in message and message['id'] in self.pending_requests:
            future = self.pending_requests.pop(message['id'])
            if 'error' in message:
                future.set_exception(Exception(message['error']['message']))
            else:
                future.set_result(message.get('result'))
        elif 'method' in message:
            await self.handle_notification(message)

    async def send_request(self, method: str, params: Optional[Dict[str, Any]] = None) -> Any:
        request_id = self.request_id
        self.request_id += 1
        
        future = asyncio.Future()
        self.pending_requests[request_id] = future
        
        request = {
            'jsonrpc': '2.0',
            'id': request_id,
            'method': method
        }
        
        if params:
            request['params'] = params
        
        await self.websocket.send(json.dumps(request))
        
        try:
            return await asyncio.wait_for(future, timeout=10.0)
        except asyncio.TimeoutError:
            self.pending_requests.pop(request_id, None)
            raise Exception('Request timeout')

    async def initialize(self):
        return await self.send_request('initialize', {
            'protocolVersion': '2024-11-05',
            'capabilities': {
                'tools': {},
                'resources': {},
                'prompts': {}
            },
            'clientInfo': {
                'name': 'python-client',
                'version': '1.0.0'
            }
        })

    async def call_tool(self, tool_name: str, params: Dict[str, Any]) -> Any:
        return await self.send_request('tools/call', {
            'name': tool_name,
            'arguments': params
        })

    async def list_tools(self) -> List[Dict[str, Any]]:
        result = await self.send_request('tools/list')
        return result.get('tools', [])

    async def read_resource(self, uri: str) -> Any:
        return await self.send_request('resources/read', {'uri': uri})

    async def handle_notification(self, message: Dict[str, Any]):
        method = message.get('method')
        if method == 'notifications/tools/list_changed':
            await self.on_tools_changed()
        elif method == 'notifications/resources/list_changed':
            await self.on_resources_changed()

    async def on_tools_changed(self):
        tools = await self.list_tools()
        print(f"Tools updated: {len(tools)} available")

    async def on_resources_changed(self):
        print("Resources changed")

# Usage in AI application
async def main():
    client = McpPythonClient('ws://localhost:8080/mcp')
    await client.connect()
    
    # Get available tools
    tools = await client.list_tools()
    print(f"Available tools: {[tool['name'] for tool in tools]}")
    
    # Use tools in AI workflow
    for tool in tools:
        result = await client.call_tool(tool['name'], {
            'example_param': 'value'
        })
        print(f"Tool {tool['name']} result: {result}")

if __name__ == '__main__':
    asyncio.run(main())
```

## Specific Integration Examples

### 1. Jupyter Notebook Integration

Create a Jupyter extension for MCP integration:

```python
# mcp_jupyter_extension.py
from IPython.core.magic import Magics, magics_class, line_magic, cell_magic
from IPython.core.magic_arguments import argument, magic_arguments, parse_argstring
import asyncio
import json

@magics_class
class McpMagics(Magics):
    def __init__(self, shell, mcp_client):
        super().__init__(shell)
        self.mcp_client = mcp_client

    @line_magic
    @magic_arguments()
    @argument('tool_name', help='Name of the MCP tool to call')
    @argument('--params', help='JSON parameters for the tool')
    def mcp_call(self, line):
        """Call an MCP tool from Jupyter"""
        args = parse_argstring(self.mcp_call, line)
        
        params = {}
        if args.params:
            params = json.loads(args.params)
        
        loop = asyncio.get_event_loop()
        result = loop.run_until_complete(
            self.mcp_client.call_tool(args.tool_name, params)
        )
        
        return result

    @line_magic
    def mcp_tools(self, line):
        """List available MCP tools"""
        loop = asyncio.get_event_loop()
        tools = loop.run_until_complete(self.mcp_client.list_tools())
        
        for tool in tools:
            print(f"- {tool['name']}: {tool.get('description', 'No description')}")

# Load extension
def load_ipython_extension(ipython):
    # Initialize MCP client
    client = McpPythonClient('ws://localhost:8080/mcp')
    loop = asyncio.get_event_loop()
    loop.run_until_complete(client.connect())
    
    # Register magics
    ipython.register_magic_function(McpMagics(ipython, client))
```

Usage in Jupyter:
```python
# Load the extension
%load_ext mcp_jupyter_extension

# List available tools
%mcp_tools

# Call a tool
%mcp_call analyze_data --params '{"dataset": "sales.csv", "analysis_type": "summary"}'
```

### 2. Streamlit Integration

Create a Streamlit app with MCP integration:

```python
import streamlit as st
import asyncio
import json
from mcp_client import McpPythonClient

@st.cache_resource
def get_mcp_client():
    client = McpPythonClient('ws://localhost:8080/mcp')
    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    loop.run_until_complete(client.connect())
    return client, loop

def main():
    st.title("AI Assistant with MCP Tools")
    
    # Initialize MCP client
    client, loop = get_mcp_client()
    
    # Sidebar for tool selection
    st.sidebar.header("Available Tools")
    tools = loop.run_until_complete(client.list_tools())
    
    selected_tool = st.sidebar.selectbox(
        "Select a tool:",
        options=[tool['name'] for tool in tools],
        format_func=lambda x: f"{x} - {next(t['description'] for t in tools if t['name'] == x)}"
    )
    
    if selected_tool:
        tool_info = next(t for t in tools if t['name'] == selected_tool)
        
        st.header(f"Tool: {selected_tool}")
        st.write(tool_info.get('description', 'No description available'))
        
        # Dynamic parameter input
        params = {}
        if 'parameters' in tool_info:
            st.subheader("Parameters")
            for param in tool_info['parameters']:
                if param['type'] == 'string':
                    params[param['name']] = st.text_input(
                        param['name'],
                        help=param.get('description', '')
                    )
                elif param['type'] == 'number':
                    params[param['name']] = st.number_input(
                        param['name'],
                        help=param.get('description', '')
                    )
                elif param['type'] == 'boolean':
                    params[param['name']] = st.checkbox(
                        param['name'],
                        help=param.get('description', '')
                    )
        
        # Execute tool
        if st.button("Execute Tool"):
            with st.spinner("Executing..."):
                try:
                    result = loop.run_until_complete(
                        client.call_tool(selected_tool, params)
                    )
                    
                    st.success("Tool executed successfully!")
                    st.json(result)
                    
                except Exception as e:
                    st.error(f"Error executing tool: {str(e)}")

    # Chat interface
    st.header("AI Chat")
    
    if "messages" not in st.session_state:
        st.session_state.messages = []
    
    for message in st.session_state.messages:
        with st.chat_message(message["role"]):
            st.markdown(message["content"])
    
    if prompt := st.chat_input("Ask the AI assistant..."):
        st.session_state.messages.append({"role": "user", "content": prompt})
        
        with st.chat_message("user"):
            st.markdown(prompt)
        
        with st.chat_message("assistant"):
            # Here you would integrate with your AI model
            # and use MCP tools as needed
            response = process_ai_request(prompt, client, loop)
            st.markdown(response)
            st.session_state.messages.append({"role": "assistant", "content": response})

def process_ai_request(prompt, client, loop):
    # This is where you'd integrate with your AI model
    # and determine which MCP tools to use
    
    # For demonstration, we'll just echo back with available tools
    tools = loop.run_until_complete(client.list_tools())
    tool_names = [tool['name'] for tool in tools]
    
    return f"I understand you want: '{prompt}'. I have these tools available: {', '.join(tool_names)}"

if __name__ == "__main__":
    main()
```

### 3. Browser Extension Integration

Create a browser extension that uses MCP tools:

**manifest.json:**
```json
{
  "manifest_version": 3,
  "name": "MCP Assistant",
  "version": "1.0.0",
  "description": "AI assistant with MCP tool integration",
  "permissions": [
    "activeTab",
    "storage"
  ],
  "background": {
    "service_worker": "background.js"
  },
  "content_scripts": [
    {
      "matches": ["<all_urls>"],
      "js": ["content.js"]
    }
  ],
  "action": {
    "default_popup": "popup.html",
    "default_title": "MCP Assistant"
  }
}
```

**background.js:**
```javascript
class McpExtensionClient {
    constructor() {
        this.serverUrl = 'ws://localhost:8080/mcp';
        this.ws = null;
        this.requestId = 1;
        this.pendingRequests = new Map();
    }

    async connect() {
        return new Promise((resolve, reject) => {
            this.ws = new WebSocket(this.serverUrl);
            
            this.ws.onopen = async () => {
                await this.initialize();
                resolve();
            };
            
            this.ws.onmessage = (event) => {
                const message = JSON.parse(event.data);
                this.handleMessage(message);
            };
            
            this.ws.onerror = reject;
        });
    }

    async initialize() {
        return await this.sendRequest('initialize', {
            protocolVersion: '2024-11-05',
            capabilities: { tools: {}, resources: {}, prompts: {} },
            clientInfo: { name: 'browser-extension', version: '1.0.0' }
        });
    }

    async sendRequest(method, params = {}) {
        return new Promise((resolve, reject) => {
            const id = this.requestId++;
            this.pendingRequests.set(id, { resolve, reject });
            
            this.ws.send(JSON.stringify({
                jsonrpc: '2.0',
                id,
                method,
                params
            }));
        });
    }

    handleMessage(message) {
        if (message.id && this.pendingRequests.has(message.id)) {
            const { resolve, reject } = this.pendingRequests.get(message.id);
            this.pendingRequests.delete(message.id);
            
            if (message.error) {
                reject(new Error(message.error.message));
            } else {
                resolve(message.result);
            }
        }
    }

    async callTool(toolName, params) {
        return await this.sendRequest('tools/call', {
            name: toolName,
            arguments: params
        });
    }

    async listTools() {
        const result = await this.sendRequest('tools/list');
        return result.tools || [];
    }
}

// Initialize client
let mcpClient = new McpExtensionClient();

// Handle messages from popup/content scripts
chrome.runtime.onMessage.addListener((request, sender, sendResponse) => {
    if (request.action === 'callTool') {
        mcpClient.callTool(request.toolName, request.params)
            .then(sendResponse)
            .catch(error => sendResponse({ error: error.message }));
        return true; // Async response
    } else if (request.action === 'listTools') {
        mcpClient.listTools()
            .then(sendResponse)
            .catch(error => sendResponse({ error: error.message }));
        return true;
    }
});

// Connect on startup
mcpClient.connect().catch(console.error);
```

**popup.html:**
```html
<!DOCTYPE html>
<html>
<head>
    <style>
        body { width: 300px; padding: 10px; }
        .tool { margin: 5px 0; padding: 5px; border: 1px solid #ccc; }
        .tool button { margin-left: 10px; }
    </style>
</head>
<body>
    <h3>MCP Assistant</h3>
    <div id="tools"></div>
    <script src="popup.js"></script>
</body>
</html>
```

**popup.js:**
```javascript
document.addEventListener('DOMContentLoaded', async () => {
    const toolsDiv = document.getElementById('tools');
    
    try {
        // Get available tools
        const tools = await new Promise((resolve) => {
            chrome.runtime.sendMessage({ action: 'listTools' }, resolve);
        });
        
        // Display tools
        tools.forEach(tool => {
            const toolDiv = document.createElement('div');
            toolDiv.className = 'tool';
            toolDiv.innerHTML = `
                <strong>${tool.name}</strong>
                <p>${tool.description}</p>
                <button onclick="executeTool('${tool.name}')">Execute</button>
            `;
            toolsDiv.appendChild(toolDiv);
        });
        
    } catch (error) {
        toolsDiv.innerHTML = `<p>Error: ${error.message}</p>`;
    }
});

async function executeTool(toolName) {
    try {
        const result = await new Promise((resolve) => {
            chrome.runtime.sendMessage({
                action: 'callTool',
                toolName: toolName,
                params: {} // You could add parameter input here
            }, resolve);
        });
        
        alert(`Tool result: ${JSON.stringify(result, null, 2)}`);
    } catch (error) {
        alert(`Error: ${error.message}`);
    }
}
```

### 4. Mobile App Integration (React Native)

Create a React Native app with MCP integration:

```javascript
// McpClient.js
import { NativeModules, NativeEventEmitter } from 'react-native';

class McpReactNativeClient {
    constructor(serverUrl) {
        this.serverUrl = serverUrl;
        this.requestId = 1;
        this.pendingRequests = new Map();
        this.ws = null;
    }

    connect() {
        return new Promise((resolve, reject) => {
            this.ws = new WebSocket(this.serverUrl);
            
            this.ws.onopen = async () => {
                await this.initialize();
                resolve();
            };
            
            this.ws.onmessage = (event) => {
                const message = JSON.parse(event.data);
                this.handleMessage(message);
            };
            
            this.ws.onerror = reject;
        });
    }

    async initialize() {
        return await this.sendRequest('initialize', {
            protocolVersion: '2024-11-05',
            capabilities: { tools: {}, resources: {}, prompts: {} },
            clientInfo: { name: 'react-native-app', version: '1.0.0' }
        });
    }

    sendRequest(method, params = {}) {
        return new Promise((resolve, reject) => {
            const id = this.requestId++;
            this.pendingRequests.set(id, { resolve, reject });
            
            this.ws.send(JSON.stringify({
                jsonrpc: '2.0',
                id,
                method,
                params
            }));
            
            // Timeout
            setTimeout(() => {
                if (this.pendingRequests.has(id)) {
                    this.pendingRequests.delete(id);
                    reject(new Error('Request timeout'));
                }
            }, 10000);
        });
    }

    handleMessage(message) {
        if (message.id && this.pendingRequests.has(message.id)) {
            const { resolve, reject } = this.pendingRequests.get(message.id);
            this.pendingRequests.delete(message.id);
            
            if (message.error) {
                reject(new Error(message.error.message));
            } else {
                resolve(message.result);
            }
        }
    }

    async callTool(toolName, params) {
        return await this.sendRequest('tools/call', {
            name: toolName,
            arguments: params
        });
    }

    async listTools() {
        const result = await this.sendRequest('tools/list');
        return result.tools || [];
    }
}

export default McpReactNativeClient;
```

```javascript
// App.js
import React, { useState, useEffect } from 'react';
import {
    View,
    Text,
    FlatList,
    TouchableOpacity,
    StyleSheet,
    Alert,
    TextInput
} from 'react-native';
import McpReactNativeClient from './McpClient';

const App = () => {
    const [client, setClient] = useState(null);
    const [tools, setTools] = useState([]);
    const [messages, setMessages] = useState([]);
    const [inputText, setInputText] = useState('');

    useEffect(() => {
        initializeClient();
    }, []);

    const initializeClient = async () => {
        try {
            const mcpClient = new McpReactNativeClient('ws://your-server.com:8080/mcp');
            await mcpClient.connect();
            
            const availableTools = await mcpClient.listTools();
            setTools(availableTools);
            setClient(mcpClient);
            
            Alert.alert('Success', 'Connected to MCP server');
        } catch (error) {
            Alert.alert('Error', `Failed to connect: ${error.message}`);
        }
    };

    const executeTool = async (toolName) => {
        if (!client) return;
        
        try {
            const result = await client.callTool(toolName, {
                input: inputText
            });
            
            setMessages(prev => [...prev, {
                type: 'tool_result',
                tool: toolName,
                result: JSON.stringify(result, null, 2)
            }]);
            
        } catch (error) {
            Alert.alert('Error', `Tool execution failed: ${error.message}`);
        }
    };

    const renderTool = ({ item }) => (
        <TouchableOpacity
            style={styles.toolItem}
            onPress={() => executeTool(item.name)}
        >
            <Text style={styles.toolName}>{item.name}</Text>
            <Text style={styles.toolDescription}>{item.description}</Text>
        </TouchableOpacity>
    );

    const renderMessage = ({ item }) => (
        <View style={styles.messageItem}>
            <Text style={styles.messageType}>{item.type}: {item.tool}</Text>
            <Text style={styles.messageContent}>{item.result}</Text>
        </View>
    );

    return (
        <View style={styles.container}>
            <Text style={styles.header}>MCP Mobile Assistant</Text>
            
            <TextInput
                style={styles.input}
                placeholder="Enter input for tools..."
                value={inputText}
                onChangeText={setInputText}
            />
            
            <Text style={styles.sectionHeader}>Available Tools:</Text>
            <FlatList
                data={tools}
                renderItem={renderTool}
                keyExtractor={item => item.name}
                style={styles.toolsList}
            />
            
            <Text style={styles.sectionHeader}>Results:</Text>
            <FlatList
                data={messages}
                renderItem={renderMessage}
                keyExtractor={(item, index) => index.toString()}
                style={styles.messagesList}
            />
        </View>
    );
};

const styles = StyleSheet.create({
    container: {
        flex: 1,
        padding: 20,
        backgroundColor: '#f5f5f5'
    },
    header: {
        fontSize: 24,
        fontWeight: 'bold',
        textAlign: 'center',
        marginBottom: 20
    },
    input: {
        borderWidth: 1,
        borderColor: '#ddd',
        padding: 10,
        marginBottom: 20,
        backgroundColor: 'white'
    },
    sectionHeader: {
        fontSize: 18,
        fontWeight: 'bold',
        marginBottom: 10
    },
    toolsList: {
        maxHeight: 200,
        marginBottom: 20
    },
    toolItem: {
        backgroundColor: 'white',
        padding: 15,
        marginBottom: 5,
        borderRadius: 5
    },
    toolName: {
        fontSize: 16,
        fontWeight: 'bold'
    },
    toolDescription: {
        fontSize: 14,
        color: '#666'
    },
    messagesList: {
        flex: 1
    },
    messageItem: {
        backgroundColor: 'white',
        padding: 10,
        marginBottom: 5,
        borderRadius: 5
    },
    messageType: {
        fontSize: 12,
        fontWeight: 'bold',
        color: '#333'
    },
    messageContent: {
        fontSize: 12,
        fontFamily: 'monospace',
        marginTop: 5
    }
});

export default App;
```

## Best Practices for Integration

### 1. Error Handling

Always implement robust error handling:

```javascript
class RobustMcpClient {
    constructor(serverUrl) {
        this.serverUrl = serverUrl;
        this.retryAttempts = 3;
        this.retryDelay = 1000;
    }

    async callToolWithRetry(toolName, params, attempt = 1) {
        try {
            return await this.callTool(toolName, params);
        } catch (error) {
            if (attempt < this.retryAttempts) {
                console.log(`Attempt ${attempt} failed, retrying...`);
                await this.delay(this.retryDelay * attempt);
                return await this.callToolWithRetry(toolName, params, attempt + 1);
            }
            throw error;
        }
    }

    delay(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }
}
```

### 2. Connection Management

Implement proper connection lifecycle management:

```javascript
class ManagedMcpClient {
    constructor(serverUrl) {
        this.serverUrl = serverUrl;
        this.reconnectAttempts = 0;
        this.maxReconnectAttempts = 5;
        this.reconnectDelay = 2000;
    }

    async connect() {
        try {
            await this.establishConnection();
            this.reconnectAttempts = 0;
        } catch (error) {
            await this.handleConnectionError(error);
        }
    }

    async handleConnectionError(error) {
        if (this.reconnectAttempts < this.maxReconnectAttempts) {
            this.reconnectAttempts++;
            console.log(`Connection failed, attempting reconnect ${this.reconnectAttempts}/${this.maxReconnectAttempts}`);
            
            await this.delay(this.reconnectDelay);
            await this.connect();
        } else {
            throw new Error(`Failed to connect after ${this.maxReconnectAttempts} attempts`);
        }
    }
}
```

### 3. Performance Optimization

Implement caching and batching:

```javascript
class OptimizedMcpClient {
    constructor(serverUrl) {
        this.serverUrl = serverUrl;
        this.cache = new Map();
        this.batchQueue = [];
        this.batchTimeout = null;
    }

    async callToolCached(toolName, params, ttl = 60000) {
        const cacheKey = `${toolName}:${JSON.stringify(params)}`;
        const cached = this.cache.get(cacheKey);
        
        if (cached && Date.now() - cached.timestamp < ttl) {
            return cached.result;
        }
        
        const result = await this.callTool(toolName, params);
        this.cache.set(cacheKey, {
            result,
            timestamp: Date.now()
        });
        
        return result;
    }

    queueToolCall(toolName, params) {
        return new Promise((resolve, reject) => {
            this.batchQueue.push({ toolName, params, resolve, reject });
            
            if (this.batchTimeout) {
                clearTimeout(this.batchTimeout);
            }
            
            this.batchTimeout = setTimeout(() => {
                this.processBatch();
            }, 100);
        });
    }

    async processBatch() {
        const batch = this.batchQueue.splice(0);
        this.batchTimeout = null;
        
        // Process batch efficiently
        for (const item of batch) {
            try {
                const result = await this.callTool(item.toolName, item.params);
                item.resolve(result);
            } catch (error) {
                item.reject(error);
            }
        }
    }
}
```

## Testing Integration

### 1. Unit Tests

```javascript
// Jest test example
describe('McpClient', () => {
    let client;
    let mockWebSocket;

    beforeEach(() => {
        mockWebSocket = {
            send: jest.fn(),
            close: jest.fn(),
            onopen: null,
            onmessage: null,
            onerror: null,
            onclose: null
        };
        
        global.WebSocket = jest.fn(() => mockWebSocket);
        client = new McpClient('ws://localhost:8080');
    });

    test('should connect and initialize', async () => {
        const connectPromise = client.connect();
        
        // Simulate connection opening
        mockWebSocket.onopen();
        
        // Simulate initialization response
        mockWebSocket.onmessage({
            data: JSON.stringify({
                jsonrpc: '2.0',
                id: 1,
                result: { protocolVersion: '2024-11-05' }
            })
        });
        
        await connectPromise;
        expect(mockWebSocket.send).toHaveBeenCalled();
    });

    test('should call tools successfully', async () => {
        // Setup connection
        await client.connect();
        
        const toolCallPromise = client.callTool('test_tool', { param: 'value' });
        
        // Simulate tool response
        mockWebSocket.onmessage({
            data: JSON.stringify({
                jsonrpc: '2.0',
                id: 2,
                result: { success: true, data: 'test result' }
            })
        });
        
        const result = await toolCallPromise;
        expect(result).toEqual({ success: true, data: 'test result' });
    });
});
```

### 2. Integration Tests

```javascript
describe('MCP Integration', () => {
    let server;
    let client;

    beforeAll(async () => {
        // Start test MCP server
        server = await startTestMcpServer();
        client = new McpClient(server.url);
        await client.connect();
    });

    afterAll(async () => {
        await client.disconnect();
        await server.stop();
    });

    test('should list available tools', async () => {
        const tools = await client.listTools();
        expect(tools).toBeInstanceOf(Array);
        expect(tools.length).toBeGreaterThan(0);
    });

    test('should execute tools end-to-end', async () => {
        const result = await client.callTool('echo', { message: 'hello' });
        expect(result.message).toBe('hello');
    });
});
```

Your MCP servers can now integrate with any AI client or development tool! 🚀

The key is implementing the MCP protocol correctly in your client and handling the specific requirements of each platform. Whether it's a web app, mobile app, IDE plugin, or command-line tool, the patterns shown here provide a solid foundation for integration.
