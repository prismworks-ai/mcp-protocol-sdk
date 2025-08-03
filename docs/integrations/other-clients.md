# 🔌 Other MCP Clients Integration

Integrate your MCP servers with various AI clients and development tools beyond Claude Desktop and Cursor IDE.

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

        const result = await response.json();
        if (result.error) {
            throw new Error(`Initialization failed: ${result.error.message}`);
        }
        
        return result.result;
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
            throw new Error(`Tool call failed: ${result.error.message}`);
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
        if (result.error) {
            throw new Error(`Tools list failed: ${result.error.message}`);
        }
        
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
        if (result.error) {
            throw new Error(`Resource read failed: ${result.error.message}`);
        }
        
        return result.result;
    }

    async listResources() {
        const response = await fetch(`${this.serverUrl}/mcp`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                jsonrpc: '2.0',
                id: this.requestId++,
                method: 'resources/list'
            })
        });

        const result = await response.json();
        if (result.error) {
            throw new Error(`Resources list failed: ${result.error.message}`);
        }
        
        return result.result.resources || [];
    }
}

// Usage in a web application
async function initializeAI() {
    try {
        const mcpClient = new McpHttpClient('http://localhost:3000');
        const initResult = await mcpClient.initialize();
        
        console.log('MCP client initialized:', initResult);
        
        // Get available tools
        const tools = await mcpClient.listTools();
        console.log('Available tools:', tools);
        
        // Register tools in AI workflow
        for (const tool of tools) {
            registerAITool(tool.name, async (params) => {
                return await mcpClient.callTool(tool.name, params);
            });
        }
        
        // Get available resources
        const resources = await mcpClient.listResources();
        console.log('Available resources:', resources);
        
    } catch (error) {
        console.error('Failed to initialize MCP client:', error);
    }
}

function registerAITool(toolName, handler) {
    // Register the tool with your AI system
    console.log(`Registered tool: ${toolName}`);
    // Implementation depends on your AI framework
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
        this.reconnectAttempts = 0;
        this.maxReconnectAttempts = 5;
        this.reconnectDelay = 2000;
    }

    async connect() {
        return new Promise((resolve, reject) => {
            try {
                this.ws = new WebSocket(this.serverUrl);
                
                this.ws.onopen = async () => {
                    console.log('WebSocket connected');
                    this.reconnectAttempts = 0;
                    
                    try {
                        await this.initialize();
                        resolve();
                    } catch (error) {
                        reject(error);
                    }
                };
                
                this.ws.onmessage = (event) => {
                    try {
                        const message = JSON.parse(event.data);
                        this.handleMessage(message);
                    } catch (error) {
                        console.error('Failed to parse message:', error);
                    }
                };
                
                this.ws.onerror = (error) => {
                    console.error('WebSocket error:', error);
                    reject(error);
                };
                
                this.ws.onclose = (event) => {
                    console.log('WebSocket connection closed:', event.code, event.reason);
                    this.handleDisconnection();
                };
                
            } catch (error) {
                reject(error);
            }
        });
    }

    async handleDisconnection() {
        if (this.reconnectAttempts < this.maxReconnectAttempts) {
            this.reconnectAttempts++;
            console.log(`Attempting to reconnect (${this.reconnectAttempts}/${this.maxReconnectAttempts})...`);
            
            setTimeout(() => {
                this.connect().catch(error => {
                    console.error('Reconnection failed:', error);
                });
            }, this.reconnectDelay * this.reconnectAttempts);
        } else {
            console.error('Max reconnection attempts reached');
        }
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
        if (!this.ws || this.ws.readyState !== WebSocket.OPEN) {
            throw new Error('WebSocket not connected');
        }
        
        return new Promise((resolve, reject) => {
            const id = this.requestId++;
            this.pendingRequests.set(id, { resolve, reject });
            
            const request = {
                jsonrpc: '2.0',
                id,
                method,
                params
            };
            
            this.ws.send(JSON.stringify(request));
            
            // Set timeout
            setTimeout(() => {
                if (this.pendingRequests.has(id)) {
                    this.pendingRequests.delete(id);
                    reject(new Error('Request timeout'));
                }
            }, 30000); // 30 second timeout
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

    async listTools() {
        const result = await this.sendRequest('tools/list');
        return result.tools || [];
    }

    async readResource(uri) {
        return await this.sendRequest('resources/read', { uri });
    }

    async listResources() {
        const result = await this.sendRequest('resources/list');
        return result.resources || [];
    }

    handleNotification(message) {
        switch (message.method) {
            case 'notifications/tools/list_changed':
                this.onToolsChanged();
                break;
            case 'notifications/resources/list_changed':
                this.onResourcesChanged();
                break;
            case 'notifications/prompts/list_changed':
                this.onPromptsChanged();
                break;
            default:
                console.log('Unknown notification:', message);
        }
    }

    async onToolsChanged() {
        try {
            const tools = await this.listTools();
            this.updateAICapabilities(tools);
            console.log('Tools updated:', tools.length, 'available');
        } catch (error) {
            console.error('Failed to refresh tools:', error);
        }
    }

    async onResourcesChanged() {
        try {
            const resources = await this.listResources();
            console.log('Resources updated:', resources.length, 'available');
        } catch (error) {
            console.error('Failed to refresh resources:', error);
        }
    }

    onPromptsChanged() {
        console.log('Prompts changed, refreshing...');
    }

    updateAICapabilities(tools) {
        // Update your AI system with new tool capabilities
        console.log('Updating AI with new tools:', tools.map(t => t.name));
    }

    disconnect() {
        if (this.ws) {
            this.ws.close();
            this.ws = null;
        }
    }
}
```

### 3. Python Client Integration

For Python-based AI applications:

```python
import asyncio
import json
import websockets
import aiohttp
from typing import Dict, Any, Optional, List
import logging

logger = logging.getLogger(__name__)

class McpPythonClient:
    def __init__(self, server_url: str, transport: str = 'websocket'):
        self.server_url = server_url
        self.transport = transport
        self.request_id = 1
        self.pending_requests = {}
        self.websocket = None
        self.session = None
        self.reconnect_attempts = 0
        self.max_reconnect_attempts = 5
        self.reconnect_delay = 2.0

    async def connect(self):
        """Connect to the MCP server"""
        if self.transport == 'websocket':
            await self._connect_websocket()
        elif self.transport == 'http':
            await self._connect_http()
        else:
            raise ValueError(f"Unsupported transport: {self.transport}")
        
        # Initialize the connection
        await self.initialize()

    async def _connect_websocket(self):
        """Connect via WebSocket"""
        try:
            self.websocket = await websockets.connect(self.server_url)
            
            # Start message handler
            asyncio.create_task(self._message_handler())
            
            logger.info(f"Connected to MCP server via WebSocket: {self.server_url}")
            self.reconnect_attempts = 0
            
        except Exception as e:
            logger.error(f"Failed to connect via WebSocket: {e}")
            await self._handle_reconnection()

    async def _connect_http(self):
        """Connect via HTTP"""
        self.session = aiohttp.ClientSession()
        logger.info(f"Connected to MCP server via HTTP: {self.server_url}")

    async def _handle_reconnection(self):
        """Handle reconnection logic"""
        if self.reconnect_attempts < self.max_reconnect_attempts:
            self.reconnect_attempts += 1
            delay = self.reconnect_delay * self.reconnect_attempts
            
            logger.info(f"Reconnecting in {delay}s (attempt {self.reconnect_attempts}/{self.max_reconnect_attempts})")
            await asyncio.sleep(delay)
            
            try:
                await self.connect()
            except Exception as e:
                logger.error(f"Reconnection attempt {self.reconnect_attempts} failed: {e}")
                await self._handle_reconnection()
        else:
            logger.error("Max reconnection attempts reached")
            raise ConnectionError("Failed to reconnect to MCP server")

    async def _message_handler(self):
        """Handle incoming WebSocket messages"""
        try:
            async for message in self.websocket:
                try:
                    data = json.loads(message)
                    await self._handle_message(data)
                except json.JSONDecodeError as e:
                    logger.error(f"Failed to parse message: {e}")
                except Exception as e:
                    logger.error(f"Error handling message: {e}")
        except websockets.exceptions.ConnectionClosed:
            logger.warning("WebSocket connection closed")
            await self._handle_reconnection()
        except Exception as e:
            logger.error(f"Message handler error: {e}")

    async def _handle_message(self, message: Dict[str, Any]):
        """Handle a parsed message"""
        if 'id' in message and message['id'] in self.pending_requests:
            future = self.pending_requests.pop(message['id'])
            if 'error' in message:
                future.set_exception(Exception(message['error']['message']))
            else:
                future.set_result(message.get('result'))
        elif 'method' in message:
            await self._handle_notification(message)

    async def _send_request(self, method: str, params: Optional[Dict[str, Any]] = None) -> Any:
        """Send a request and wait for response"""
        if self.transport == 'websocket':
            return await self._send_websocket_request(method, params)
        elif self.transport == 'http':
            return await self._send_http_request(method, params)
        else:
            raise ValueError(f"Unsupported transport: {self.transport}")

    async def _send_websocket_request(self, method: str, params: Optional[Dict[str, Any]] = None) -> Any:
        """Send request via WebSocket"""
        if not self.websocket or self.websocket.closed:
            raise ConnectionError("WebSocket not connected")
        
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
            return await asyncio.wait_for(future, timeout=30.0)
        except asyncio.TimeoutError:
            self.pending_requests.pop(request_id, None)
            raise Exception('Request timeout')

    async def _send_http_request(self, method: str, params: Optional[Dict[str, Any]] = None) -> Any:
        """Send request via HTTP"""
        if not self.session:
            raise ConnectionError("HTTP session not initialized")
        
        request_id = self.request_id
        self.request_id += 1
        
        request = {
            'jsonrpc': '2.0',
            'id': request_id,
            'method': method
        }
        
        if params:
            request['params'] = params
        
        try:
            async with self.session.post(
                f"{self.server_url}/mcp",
                json=request,
                timeout=aiohttp.ClientTimeout(total=30)
            ) as response:
                result = await response.json()
                
                if 'error' in result:
                    raise Exception(result['error']['message'])
                
                return result.get('result')
                
        except aiohttp.ClientTimeout:
            raise Exception('Request timeout')
        except Exception as e:
            raise Exception(f'HTTP request failed: {e}')

    async def initialize(self):
        """Initialize the MCP connection"""
        return await self._send_request('initialize', {
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
        """Call an MCP tool"""
        return await self._send_request('tools/call', {
            'name': tool_name,
            'arguments': params
        })

    async def list_tools(self) -> List[Dict[str, Any]]:
        """List available tools"""
        result = await self._send_request('tools/list')
        return result.get('tools', [])

    async def read_resource(self, uri: str) -> Any:
        """Read a resource"""
        return await self._send_request('resources/read', {'uri': uri})

    async def list_resources(self) -> List[Dict[str, Any]]:
        """List available resources"""
        result = await self._send_request('resources/list')
        return result.get('resources', [])

    async def get_prompt(self, name: str, arguments: Optional[Dict[str, Any]] = None) -> Any:
        """Get a prompt"""
        params = {'name': name}
        if arguments:
            params['arguments'] = arguments
        return await self._send_request('prompts/get', params)

    async def list_prompts(self) -> List[Dict[str, Any]]:
        """List available prompts"""
        result = await self._send_request('prompts/list')
        return result.get('prompts', [])

    async def _handle_notification(self, message: Dict[str, Any]):
        """Handle notifications from server"""
        method = message.get('method')
        if method == 'notifications/tools/list_changed':
            await self._on_tools_changed()
        elif method == 'notifications/resources/list_changed':
            await self._on_resources_changed()
        elif method == 'notifications/prompts/list_changed':
            await self._on_prompts_changed()
        else:
            logger.info(f"Unknown notification: {method}")

    async def _on_tools_changed(self):
        """Handle tools list changed notification"""
        try:
            tools = await self.list_tools()
            logger.info(f"Tools updated: {len(tools)} available")
        except Exception as e:
            logger.error(f"Failed to refresh tools: {e}")

    async def _on_resources_changed(self):
        """Handle resources list changed notification"""
        try:
            resources = await self.list_resources()
            logger.info(f"Resources updated: {len(resources)} available")
        except Exception as e:
            logger.error(f"Failed to refresh resources: {e}")

    async def _on_prompts_changed(self):
        """Handle prompts list changed notification"""
        try:
            prompts = await self.list_prompts()
            logger.info(f"Prompts updated: {len(prompts)} available")
        except Exception as e:
            logger.error(f"Failed to refresh prompts: {e}")

    async def disconnect(self):
        """Disconnect from the server"""
        if self.websocket:
            await self.websocket.close()
            self.websocket = None
        
        if self.session:
            await self.session.close()
            self.session = None
        
        logger.info("Disconnected from MCP server")

    async def __aenter__(self):
        """Async context manager entry"""
        await self.connect()
        return self

    async def __aexit__(self, exc_type, exc_val, exc_tb):
        """Async context manager exit"""
        await self.disconnect()

# Usage example
async def main():
    # Example usage with async context manager
    async with McpPythonClient('ws://localhost:8080/mcp', 'websocket') as client:
        # Get available tools
        tools = await client.list_tools()
        print(f"Available tools: {[tool['name'] for tool in tools]}")
        
        # Call a tool
        if tools:
            result = await client.call_tool(tools[0]['name'], {
                'example_param': 'value'
            })
            print(f"Tool result: {result}")
        
        # Get available resources
        resources = await client.list_resources()
        print(f"Available resources: {[res['uri'] for res in resources]}")
        
        # Read a resource
        if resources:
            content = await client.read_resource(resources[0]['uri'])
            print(f"Resource content: {content}")

if __name__ == '__main__':
    logging.basicConfig(level=logging.INFO)
    asyncio.run(main())
```

## Best Practices for Integration

### 1. Error Handling and Resilience

```javascript
class RobustMcpClient {
    constructor(serverUrl) {
        this.serverUrl = serverUrl;
        this.retryAttempts = 3;
        this.retryDelay = 1000;
        this.circuitBreaker = {
            failures: 0,
            threshold: 5,
            timeout: 30000,
            lastFailure: null
        };
    }

    async callToolWithRetry(toolName, params, attempt = 1) {
        try {
            // Check circuit breaker
            if (this.isCircuitOpen()) {
                throw new Error('Circuit breaker is open');
            }
            
            const result = await this.callTool(toolName, params);
            
            // Reset circuit breaker on success
            this.circuitBreaker.failures = 0;
            return result;
            
        } catch (error) {
            this.circuitBreaker.failures++;
            this.circuitBreaker.lastFailure = Date.now();
            
            if (attempt < this.retryAttempts) {
                console.log(`Attempt ${attempt} failed, retrying...`);
                await this.delay(this.retryDelay * attempt);
                return await this.callToolWithRetry(toolName, params, attempt + 1);
            }
            
            throw error;
        }
    }

    isCircuitOpen() {
        if (this.circuitBreaker.failures >= this.circuitBreaker.threshold) {
            const timeSinceLastFailure = Date.now() - this.circuitBreaker.lastFailure;
            return timeSinceLastFailure < this.circuitBreaker.timeout;
        }
        return false;
    }

    delay(ms) {
        return new Promise(resolve => setTimeout(resolve, ms));
    }

    async callToolWithFallback(toolName, params, fallbackFn) {
        try {
            return await this.callToolWithRetry(toolName, params);
        } catch (error) {
            console.warn(`Tool ${toolName} failed, using fallback:`, error.message);
            return await fallbackFn(params);
        }
    }
}
```

### 2. Performance Optimization

```javascript
class OptimizedMcpClient {
    constructor(serverUrl) {
        this.serverUrl = serverUrl;
        this.cache = new Map();
        this.batchQueue = [];
        this.batchTimeout = null;
        this.requestPool = new Map();
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
        const promises = batch.map(async (item) => {
            try {
                const result = await this.callTool(item.toolName, item.params);
                item.resolve(result);
            } catch (error) {
                item.reject(error);
            }
        });
        
        await Promise.allSettled(promises);
    }

    // Request pooling for identical requests
    async callToolPooled(toolName, params) {
        const requestKey = `${toolName}:${JSON.stringify(params)}`;
        
        if (this.requestPool.has(requestKey)) {
            return await this.requestPool.get(requestKey);
        }
        
        const promise = this.callTool(toolName, params);
        this.requestPool.set(requestKey, promise);
        
        try {
            const result = await promise;
            this.requestPool.delete(requestKey);
            return result;
        } catch (error) {
            this.requestPool.delete(requestKey);
            throw error;
        }
    }
}
```

### 3. Connection Management

```javascript
class ManagedMcpClient {
    constructor(serverUrl) {
        this.serverUrl = serverUrl;
        this.connection = null;
        this.connectionState = 'disconnected';
        this.heartbeatInterval = null;
        this.reconnectAttempts = 0;
        this.maxReconnectAttempts = 5;
        this.reconnectDelay = 2000;
        this.listeners = new Map();
    }

    async connect() {
        if (this.connectionState === 'connected') {
            return;
        }
        
        this.connectionState = 'connecting';
        
        try {
            this.connection = new WebSocket(this.serverUrl);
            
            this.connection.onopen = () => {
                this.connectionState = 'connected';
                this.reconnectAttempts = 0;
                this.startHeartbeat();
                this.emit('connected');
            };
            
            this.connection.onclose = () => {
                this.connectionState = 'disconnected';
                this.stopHeartbeat();
                this.emit('disconnected');
                this.handleReconnection();
            };
            
            this.connection.onerror = (error) => {
                this.emit('error', error);
            };
            
            this.connection.onmessage = (event) => {
                this.handleMessage(JSON.parse(event.data));
            };
            
        } catch (error) {
            this.connectionState = 'disconnected';
            throw error;
        }
    }

    startHeartbeat() {
        this.heartbeatInterval = setInterval(() => {
            if (this.connection && this.connection.readyState === WebSocket.OPEN) {
                this.ping();
            }
        }, 30000); // 30 second heartbeat
    }

    stopHeartbeat() {
        if (this.heartbeatInterval) {
            clearInterval(this.heartbeatInterval);
            this.heartbeatInterval = null;
        }
    }

    async ping() {
        try {
            await this.sendRequest('ping');
        } catch (error) {
            console.warn('Heartbeat failed:', error);
        }
    }

    async handleReconnection() {
        if (this.reconnectAttempts < this.maxReconnectAttempts) {
            this.reconnectAttempts++;
            const delay = this.reconnectDelay * Math.pow(2, this.reconnectAttempts - 1);
            
            console.log(`Reconnecting in ${delay}ms (attempt ${this.reconnectAttempts})`);
            
            setTimeout(() => {
                this.connect();
            }, delay);
        } else {
            this.emit('maxReconnectAttemptsReached');
        }
    }

    on(event, listener) {
        if (!this.listeners.has(event)) {
            this.listeners.set(event, []);
        }
        this.listeners.get(event).push(listener);
    }

    emit(event, ...args) {
        const eventListeners = this.listeners.get(event);
        if (eventListeners) {
            eventListeners.forEach(listener => {
                try {
                    listener(...args);
                } catch (error) {
                    console.error('Event listener error:', error);
                }
            });
        }
    }

    async waitForConnection() {
        if (this.connectionState === 'connected') {
            return;
        }
        
        return new Promise((resolve, reject) => {
            const timeout = setTimeout(() => {
                reject(new Error('Connection timeout'));
            }, 10000);
            
            const onConnected = () => {
                clearTimeout(timeout);
                resolve();
            };
            
            const onError = (error) => {
                clearTimeout(timeout);
                reject(error);
            };
            
            this.on('connected', onConnected);
            this.on('error', onError);
        });
    }

    async disconnect() {
        this.stopHeartbeat();
        
        if (this.connection) {
            this.connection.close();
            this.connection = null;
        }
        
        this.connectionState = 'disconnected';
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
            onclose: null,
            readyState: WebSocket.OPEN
        };
        
        global.WebSocket = jest.fn(() => mockWebSocket);
        client = new McpWebSocketClient('ws://localhost:8080');
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

    test('should handle errors gracefully', async () => {
        await client.connect();
        
        const toolCallPromise = client.callTool('failing_tool', {});
        
        // Simulate error response
        mockWebSocket.onmessage({
            data: JSON.stringify({
                jsonrpc: '2.0',
                id: 2,
                error: { message: 'Tool execution failed' }
            })
        });
        
        await expect(toolCallPromise).rejects.toThrow('Tool execution failed');
    });

    test('should handle disconnection and reconnection', async () => {
        const reconnectSpy = jest.spyOn(client, 'handleDisconnection');
        
        await client.connect();
        
        // Simulate disconnection
        mockWebSocket.onclose({ code: 1000, reason: 'Normal closure' });
        
        expect(reconnectSpy).toHaveBeenCalled();
    });
});
```

### 2. Integration Tests

```python
# pytest integration test example
import pytest
import asyncio
from mcp_client import McpPythonClient

@pytest.fixture
async def mcp_client():
    client = McpPythonClient('ws://localhost:8080/mcp')
    await client.connect()
    yield client
    await client.disconnect()

@pytest.mark.asyncio
async def test_list_tools(mcp_client):
    tools = await mcp_client.list_tools()
    assert isinstance(tools, list)
    assert len(tools) > 0
    
    # Check tool structure
    tool = tools[0]
    assert 'name' in tool
    assert 'description' in tool
    assert 'inputSchema' in tool

@pytest.mark.asyncio
async def test_call_tool_end_to_end(mcp_client):
    # Get available tools
    tools = await mcp_client.list_tools()
    assert len(tools) > 0
    
    # Call the first tool
    tool_name = tools[0]['name']
    result = await mcp_client.call_tool(tool_name, {
        'test_param': 'test_value'
    })
    
    assert result is not None

@pytest.mark.asyncio
async def test_resource_operations(mcp_client):
    # List resources
    resources = await mcp_client.list_resources()
    
    if resources:
        # Read the first resource
        resource_uri = resources[0]['uri']
        content = await mcp_client.read_resource(resource_uri)
        assert content is not None

@pytest.mark.asyncio
async def test_error_handling(mcp_client):
    # Test calling non-existent tool
    with pytest.raises(Exception) as exc_info:
        await mcp_client.call_tool('non_existent_tool', {})
    
    assert 'not found' in str(exc_info.value).lower()

@pytest.mark.asyncio
async def test_connection_resilience():
    # Test reconnection behavior
    client = McpPythonClient('ws://localhost:8080/mcp')
    
    # Initial connection
    await client.connect()
    tools_before = await client.list_tools()
    
    # Simulate disconnection and reconnection
    await client.disconnect()
    await asyncio.sleep(1)
    await client.connect()
    
    # Verify functionality after reconnection
    tools_after = await client.list_tools()
    assert tools_before == tools_after
    
    await client.disconnect()
```

## Deployment Considerations

### 1. Security

- **Authentication**: Implement proper authentication mechanisms
- **Authorization**: Control access to sensitive tools and resources
- **Input validation**: Validate all parameters before sending to MCP server
- **Network security**: Use secure connections (WSS/HTTPS)
- **Rate limiting**: Implement client-side rate limiting

### 2. Monitoring

```javascript
class MonitoredMcpClient extends McpWebSocketClient {
    constructor(serverUrl, options = {}) {
        super(serverUrl);
        this.metrics = {
            totalRequests: 0,
            successfulRequests: 0,
            failedRequests: 0,
            avgResponseTime: 0,
            connectionUptime: 0
        };
        this.logger = options.logger || console;
    }

    async callTool(toolName, params) {
        const startTime = Date.now();
        this.metrics.totalRequests++;
        
        try {
            const result = await super.callTool(toolName, params);
            
            this.metrics.successfulRequests++;
            this.updateResponseTime(Date.now() - startTime);
            
            this.logger.info(`Tool ${toolName} executed successfully`, {
                duration: Date.now() - startTime,
                params: Object.keys(params)
            });
            
            return result;
            
        } catch (error) {
            this.metrics.failedRequests++;
            
            this.logger.error(`Tool ${toolName} failed`, {
                error: error.message,
                duration: Date.now() - startTime,
                params: Object.keys(params)
            });
            
            throw error;
        }
    }

    updateResponseTime(duration) {
        const totalResponses = this.metrics.successfulRequests;
        this.metrics.avgResponseTime = 
            ((this.metrics.avgResponseTime * (totalResponses - 1)) + duration) / totalResponses;
    }

    getMetrics() {
        return {
            ...this.metrics,
            successRate: this.metrics.totalRequests > 0 
                ? (this.metrics.successfulRequests / this.metrics.totalRequests) * 100 
                : 0
        };
    }
}
```

### 3. Configuration Management

```python
class ConfigurableMcpClient:
    def __init__(self, config_path: str = None):
        self.config = self.load_config(config_path)
        self.client = McpPythonClient(
            self.config['server_url'],
            transport=self.config.get('transport', 'websocket')
        )

    def load_config(self, config_path: str = None) -> dict:
        import os
        import json
        
        # Default configuration
        default_config = {
            'server_url': 'ws://localhost:8080/mcp',
            'transport': 'websocket',
            'timeout': 30,
            'retry_attempts': 3,
            'retry_delay': 1.0,
            'enable_caching': True,
            'cache_ttl': 300,
            'enable_metrics': True
        }
        
        # Load from file if provided
        if config_path and os.path.exists(config_path):
            with open(config_path, 'r') as f:
                file_config = json.load(f)
                default_config.update(file_config)
        
        # Override with environment variables
        env_overrides = {
            'MCP_SERVER_URL': 'server_url',
            'MCP_TRANSPORT': 'transport',
            'MCP_TIMEOUT': 'timeout',
            'MCP_RETRY_ATTEMPTS': 'retry_attempts'
        }
        
        for env_var, config_key in env_overrides.items():
            if os.environ.get(env_var):
                default_config[config_key] = os.environ[env_var]
        
        return default_config

    async def connect(self):
        await self.client.connect()

    async def call_tool(self, tool_name: str, params: dict):
        return await self.client.call_tool(tool_name, params)
```

## Next Steps

1. **Choose the appropriate integration pattern** for your use case
2. **Implement proper error handling and resilience** patterns
3. **Add monitoring and metrics** to track performance
4. **Test thoroughly** with unit and integration tests
5. **Deploy with proper security considerations**
6. **Monitor and optimize** based on real-world usage

## Resources

- [🖥️ Claude Desktop Integration](./claude-desktop.md) - Complete desktop integration
- [⚡ Cursor IDE Integration](./cursor.md) - IDE integration patterns
- [🚀 Getting Started](../getting-started.md) - Basic server development
- [📖 Implementation Guide](../implementation-guide.md) - Complete development guide
- [🔧 Examples](../../examples/) - Working example projects

Your MCP servers can now integrate with any AI client or development tool! 🚀

The key is implementing the MCP protocol correctly in your client and handling the specific requirements of each platform. Whether it's a web app, mobile app, IDE plugin, or command-line tool, the patterns shown here provide a solid foundation for integration.
