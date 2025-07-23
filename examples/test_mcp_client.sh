#!/bin/bash

# Example script to test the MCP server with different transport types

SERVER_URL="http://localhost:8787"

echo "=== Testing Streamable HTTP Transport ==="
echo ""

echo "1. Testing initialize method (JSON response):"
curl -X POST "$SERVER_URL/mcp" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "initialize",
    "params": {
      "protocol_version": "2024-11-05",
      "capabilities": {},
      "client_info": {
        "name": "test-client",
        "version": "1.0.0"
      }
    },
    "id": 1
  }'
echo -e "\n"

echo "2. Testing tools/list method:"
curl -X POST "$SERVER_URL/mcp" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "tools/list",
    "id": 2
  }'
echo -e "\n"

echo "3. Testing tools/call with add:"
curl -X POST "$SERVER_URL/mcp" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "tools/call",
    "params": {
      "name": "add",
      "arguments": {
        "a": 5,
        "b": 3
      }
    },
    "id": 3
  }'
echo -e "\n"

echo "4. Testing tools/call with calculate:"
curl -X POST "$SERVER_URL/mcp" \
  -H "Content-Type: application/json" \
  -H "Accept: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "tools/call",
    "params": {
      "name": "calculate",
      "arguments": {
        "operation": "multiply",
        "a": 7,
        "b": 8
      }
    },
    "id": 4
  }'
echo -e "\n"

echo "5. Testing SSE stream response:"
curl -X POST "$SERVER_URL/mcp" \
  -H "Content-Type: application/json" \
  -H "Accept: text/event-stream" \
  -d '{
    "jsonrpc": "2.0",
    "method": "initialize",
    "params": {
      "protocol_version": "2024-11-05",
      "capabilities": {},
      "client_info": {
        "name": "sse-client",
        "version": "1.0.0"
      }
    },
    "id": 5
  }'
echo -e "\n"

echo ""
echo "=== Testing Legacy SSE Transport ==="
echo ""

echo "1. Getting SSE endpoint:"
curl -X GET "$SERVER_URL/sse" \
  -H "Accept: text/event-stream"
echo -e "\n"

echo "2. Sending request to messages endpoint:"
curl -X POST "$SERVER_URL/messages" \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "tools/list",
    "id": 6
  }'
echo -e "\n"

echo ""
echo "=== Testing Legacy HTTP Endpoints ==="
echo ""

echo "1. Testing /mcp/add endpoint:"
curl -X POST "$SERVER_URL/mcp/add" \
  -H "Content-Type: application/json" \
  -d '{"a": 10, "b": 20}'
echo -e "\n"

echo "2. Testing /mcp/calculate endpoint:"
curl -X POST "$SERVER_URL/mcp/calculate" \
  -H "Content-Type: application/json" \
  -d '{"operation": "divide", "a": 100, "b": 25}'
echo -e "\n"

echo "3. Testing GET /mcp endpoint:"
curl -X GET "$SERVER_URL/mcp"
echo -e "\n"