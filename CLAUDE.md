# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Template Project Overview

This is a template repository for creating MCP (Model Context Protocol) servers in Rust. When helping users customize this template, focus on:

1. **Tool Implementation**: The main customization point is adding new tools in `src/tools/`
2. **Server Configuration**: Update constants in `src/lib.rs` (SERVER_NAME, SERVER_VERSION)
3. **Package Metadata**: Update Cargo.toml and package.json with project-specific information

## Common Development Commands

This project uses cargo-make for task automation. Install it with:
```bash
cargo install cargo-make
```

### Building and Testing
- `cargo make test` - Run all unit and integration tests
- `cargo make test-integration` - Run integration tests with live server
- `cargo make build` - Build WASM for Cloudflare Workers
- `cargo make build-release` - Build optimized release version

### Development Server
- `cargo make dev` - Start local development server on port 8787
- `cargo make watch` - Watch for changes and rebuild automatically

### Code Quality
- `cargo make format` - Format code with rustfmt
- `cargo make lint` - Run clippy linter
- `cargo make check` - Check for compilation errors
- `cargo make ci` - Run complete CI pipeline

### Deployment
- `cargo make deploy` - Deploy to Cloudflare Workers (runs tests first)

### Other Commands
- `cargo make clean` - Clean build artifacts
- `cargo make docs` - Generate and open documentation
- `cargo make coverage` - Generate test coverage report
- `cargo make --list-all-steps` - Show all available tasks

## Architecture Overview

This template provides a complete MCP server implementation with the following structure:

### Core Components (src/lib.rs)

#### Protocol Implementation
- **JSON-RPC 2.0**: Full implementation with request/response handling
- **MCP Methods**: `initialize`, `tools/list`, `tools/call`
- **Error Handling**: Proper JSON-RPC error codes (-32601, -32602, -32603)

#### Transport Layers
1. **Streamable HTTP** (`/mcp`): Modern transport with content negotiation
   - Supports both JSON responses and SSE streaming
   - Uses Accept header to determine response format
   
2. **Legacy SSE** (`/sse`, `/messages`): Backward compatibility
   - GET `/sse` returns SSE stream with endpoint info
   - POST `/messages` handles JSON-RPC requests

### Tool System (src/tools/)

The tool system is designed for easy extension:

1. **Tool Registry** (`src/tools/mod.rs`):
   - `ToolRegistry` struct manages all tools
   - `register_default_tools()` function for tool registration
   - `call_tool()` method dispatches to specific handlers

2. **Tool Implementation**:
   - Each tool is a separate module in `src/tools/`
   - Tools return `ToolsCallResult` with content items
   - Error handling built into the result structure

3. **Example Tools** (`src/tools/example_calculator.rs`):
   - `add` - Simple addition example
   - `calculate` - Complex operation example with enums

### Customization Points

When customizing this template:

1. **Server Identity**: Update these constants in `src/lib.rs`:
   ```rust
   const SERVER_NAME: &str = "Your Server Name";
   const SERVER_VERSION: &str = "1.0.0";
   const PROTOCOL_VERSION: &str = "2024-11-05";
   ```

2. **Adding New Tools**:
   - Create new module in `src/tools/your_tool.rs`
   - Implement handler function returning `Result<ToolsCallResult, String>`
   - Register in `register_default_tools()` function
   - Add case in `call_tool()` match statement

3. **Removing Example Tools**:
   - Delete `src/tools/example_calculator.rs`
   - Remove registrations from `register_default_tools()`
   - Remove cases from `call_tool()` match

### Key Design Patterns
- Pattern matching for operation dispatch
- Result type for comprehensive error handling  
- CORS support with preflight handling
- Modular tool system for easy extension
- Separation of transport and business logic

### Testing Strategy
- Unit tests in `tests/integration_test.rs` for core operations
- Transport tests in `tests/transport_test.rs` for protocol compliance
- Shell script (`examples/test_mcp_client.sh`) for end-to-end testing
- Each tool should have corresponding unit tests

### Cloudflare Workers Configuration
- WASM target with optimized release profile
- Minimal dependencies for small bundle size
- Observability enabled in wrangler.toml
- Ready for environment variables and secrets

## Important Implementation Details

### Error Handling
- All errors return proper JSON-RPC error responses
- Tool errors are wrapped in ToolsCallResult with is_error flag
- Unknown methods return -32601 (Method not found)
- Invalid parameters return -32602 (Invalid params)
- Internal errors return -32603 (Internal error)

### CORS Configuration
- Supports all origins with `Access-Control-Allow-Origin: *`
- Handles MCP-specific headers like `Mcp-Session-Id`
- Proper OPTIONS preflight responses

### Tool Schemas
Tools are defined with JSON Schema for validation:
- Use standard JSON Schema types and constraints
- Required fields should be specified in the schema
- Descriptions help with tool discovery and usage

### Future Extensibility
- Session management hooks with `Mcp-Session-Id` header support
- Prepared for Durable Objects integration
- Clean separation allows adding resources and prompts capabilities