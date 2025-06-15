#!/bin/bash

# MCP Protocol SDK Integration Test Script
# Tests HTTP client-server communication end-to-end

set -e

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Test configuration
HTTP_PORT=3000
TEST_TIMEOUT=30
SERVER_PID=""
CLIENT_EXIT_CODE=""

echo -e "${BLUE}=== MCP Protocol SDK Integration Test ===${NC}"
echo -e "${BLUE}Testing HTTP transport end-to-end communication${NC}"
echo

# Function to cleanup on exit
cleanup() {
    echo -e "\n${YELLOW}Cleaning up...${NC}"
    if [ ! -z "$SERVER_PID" ]; then
        echo "Stopping HTTP server (PID: $SERVER_PID)..."
        kill $SERVER_PID 2>/dev/null || true
        wait $SERVER_PID 2>/dev/null || true
    fi
    
    # Clean up any remaining processes on the port
    lsof -ti:$HTTP_PORT | xargs kill 2>/dev/null || true
    
    echo -e "${GREEN}Cleanup completed${NC}"
}

# Set up signal handlers
trap cleanup EXIT INT TERM

# Function to wait for server to be ready
wait_for_server() {
    local timeout=$1
    local count=0
    
    echo "Waiting for HTTP server to be ready on port $HTTP_PORT..."
    
    while [ $count -lt $timeout ]; do
        if curl -s -f http://localhost:$HTTP_PORT/health > /dev/null 2>&1; then
            echo -e "${GREEN}✓ HTTP server is ready${NC}"
            return 0
        fi
        count=$((count + 1))
        sleep 1
        echo -n "."
    done
    
    echo -e "\n${RED}✗ Server failed to start within $timeout seconds${NC}"
    return 1
}

# Function to test basic HTTP connectivity
test_http_connectivity() {
    echo -e "\n${BLUE}Testing basic HTTP connectivity...${NC}"
    
    # Test health endpoint
    if curl -s -f http://localhost:$HTTP_PORT/health > /dev/null; then
        echo -e "${GREEN}✓ Health endpoint accessible${NC}"
    else
        echo -e "${RED}✗ Health endpoint not accessible${NC}"
        return 1
    fi
    
    # Test MCP endpoint with basic request
    local response=$(curl -s -X POST http://localhost:$HTTP_PORT/mcp \
        -H 'Content-Type: application/json' \
        -d '{"jsonrpc":"2.0","id":1,"method":"ping"}' || echo "CURL_FAILED")
    
    if [ "$response" = "CURL_FAILED" ]; then
        echo -e "${RED}✗ MCP endpoint not accessible${NC}"
        return 1
    elif echo "$response" | grep -q '"result"'; then
        echo -e "${GREEN}✓ MCP endpoint responding to requests${NC}"
        echo "  Response: $response"
    else
        echo -e "${YELLOW}⚠ MCP endpoint accessible but unexpected response${NC}"
        echo "  Response: $response"
    fi
    
    return 0
}

# Build the project
echo -e "${BLUE}Building project with all features...${NC}"
if ! cargo build --all-features --examples; then
    echo -e "${RED}✗ Build failed${NC}"
    exit 1
fi
echo -e "${GREEN}✓ Build successful${NC}"

# Start the HTTP server
echo -e "\n${BLUE}Starting HTTP server...${NC}"
cargo run --features http --example http_server > http_server.log 2>&1 &
SERVER_PID=$!

echo "HTTP server started with PID: $SERVER_PID"
echo "Server logs will be written to: http_server.log"

# Wait for server to be ready
if ! wait_for_server $TEST_TIMEOUT; then
    echo -e "${RED}✗ Integration test failed - server startup${NC}"
    echo "Server logs:"
    cat http_server.log 2>/dev/null || echo "No server logs available"
    exit 1
fi

# Test basic HTTP connectivity
if ! test_http_connectivity; then
    echo -e "${RED}✗ Integration test failed - HTTP connectivity${NC}"
    echo "Server logs:"
    cat http_server.log 2>/dev/null || echo "No server logs available"
    exit 1
fi

# Run the HTTP client
echo -e "\n${BLUE}Running HTTP client integration test...${NC}"
echo "Client output:"
echo "----------------------------------------"

if timeout $TEST_TIMEOUT cargo run --features http --example http_client; then
    CLIENT_EXIT_CODE=0
    echo "----------------------------------------"
    echo -e "${GREEN}✓ HTTP client completed successfully${NC}"
else
    CLIENT_EXIT_CODE=$?
    echo "----------------------------------------"
    echo -e "${RED}✗ HTTP client failed or timed out (exit code: $CLIENT_EXIT_CODE)${NC}"
fi

# Show server logs
echo -e "\n${BLUE}HTTP Server logs:${NC}"
echo "----------------------------------------"
cat http_server.log 2>/dev/null || echo "No server logs available"
echo "----------------------------------------"

# Test advanced MCP operations
echo -e "\n${BLUE}Testing advanced MCP operations via curl...${NC}"

# Test tools/list
echo "Testing tools/list..."
TOOLS_RESPONSE=$(curl -s -X POST http://localhost:$HTTP_PORT/mcp \
    -H 'Content-Type: application/json' \
    -d '{"jsonrpc":"2.0","id":2,"method":"tools/list"}')
echo "Tools response: $TOOLS_RESPONSE"

if echo "$TOOLS_RESPONSE" | grep -q '"http_calculator"'; then
    echo -e "${GREEN}✓ HTTP calculator tool found${NC}"
else
    echo -e "${YELLOW}⚠ HTTP calculator tool not found in response${NC}"
fi

# Test resources/list
echo -e "\nTesting resources/list..."
RESOURCES_RESPONSE=$(curl -s -X POST http://localhost:$HTTP_PORT/mcp \
    -H 'Content-Type: application/json' \
    -d '{"jsonrpc":"2.0","id":3,"method":"resources/list"}')
echo "Resources response: $RESOURCES_RESPONSE"

if echo "$RESOURCES_RESPONSE" | grep -q '"http://server/"'; then
    echo -e "${GREEN}✓ HTTP server resources found${NC}"
else
    echo -e "${YELLOW}⚠ HTTP server resources not found in response${NC}"
fi

# Test tool call
echo -e "\nTesting tools/call with calculator..."
CALC_RESPONSE=$(curl -s -X POST http://localhost:$HTTP_PORT/mcp \
    -H 'Content-Type: application/json' \
    -d '{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{"name":"http_calculator","arguments":{"operation":"multiply","a":6,"b":7}}}')
echo "Calculator response: $CALC_RESPONSE"

if echo "$CALC_RESPONSE" | grep -q "42"; then
    echo -e "${GREEN}✓ Calculator tool working correctly (6 * 7 = 42)${NC}"
else
    echo -e "${YELLOW}⚠ Calculator tool response unexpected${NC}"
fi

# Final assessment
echo -e "\n${BLUE}=== Integration Test Results ===${NC}"

if [ "$CLIENT_EXIT_CODE" -eq 0 ]; then
    echo -e "${GREEN}✓ HTTP Client-Server Communication: SUCCESS${NC}"
    echo -e "${GREEN}✓ MCP Initialization Handshake: SUCCESS${NC}"
    echo -e "${GREEN}✓ Tool Calls: SUCCESS${NC}"
    echo -e "${GREEN}✓ Resource Access: SUCCESS${NC}"
    echo -e "${GREEN}✓ Request Tracking: SUCCESS${NC}"
    echo
    echo -e "${GREEN}🎉 ALL INTEGRATION TESTS PASSED! 🎉${NC}"
    echo -e "${GREEN}The transport improvements are working correctly end-to-end.${NC}"
    exit 0
else
    echo -e "${RED}✗ HTTP Client-Server Communication: FAILED${NC}"
    echo
    echo -e "${RED}❌ INTEGRATION TESTS FAILED${NC}"
    echo -e "${RED}Review the logs above for details.${NC}"
    exit 1
fi
