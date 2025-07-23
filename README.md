# MCP Server Rust Template

A production-ready template for building MCP (Model Context Protocol) servers in Rust, deployable to Cloudflare Workers.

## 🚀 Quick Start

### Using cargo-generate (Recommended)

```bash
# Install cargo-generate if you haven't already
cargo install cargo-generate

# Generate a new project from this template
cargo generate --git git@github.com:sunnyhealthai/mcp-server-rust-template.git --name my-mcp-server

# Navigate to your new project
cd my-mcp-server

# Install dependencies and start developing
cargo make init
cargo make dev
```

### Manual Setup

1. **Clone this template**
   ```bash
   git clone git@github.com:sunnyhealthai/mcp-server-rust-template.git my-mcp-server
   cd my-mcp-server
   ```

2. **Install cargo-make**
   ```bash
   cargo install cargo-make
   ```

3. **Install dependencies**
   ```bash
   cargo make install-deps
   ```

4. **Run locally**
   ```bash
   cargo make dev
   ```

5. **Test your server**
   ```bash
   cargo make test
   cargo make test-integration
   ```

## 🛠️ Customization Guide

### 1. Update Server Information

Edit `src/lib.rs` and update these constants:
```rust
const SERVER_NAME: &str = "Your Server Name";
const SERVER_VERSION: &str = "1.0.0";
const PROTOCOL_VERSION: &str = "2024-11-05";
```

### 2. Update Package Information

Edit `Cargo.toml`:
```toml
[package]
name = "your-mcp-server"
authors = ["Your Name <your.email@example.com>"]
repository = "https://github.com/yourusername/your-mcp-server"
```

Edit `package.json`:
```json
{
  "name": "your-mcp-server",
  "description": "Your MCP server description"
}
```

### 3. Implement Your Tools

The template includes example calculator tools. To add your own:

1. Create a new tool module in `src/tools/`:
   ```rust
   // src/tools/your_tool.rs
   use crate::{ContentItem, ToolsCallResult};
   
   pub fn handle_your_tool(arguments: Option<serde_json::Value>) -> Result<ToolsCallResult, String> {
       // Your implementation
   }
   ```

2. Register your tool in `src/tools/mod.rs`:
   ```rust
   pub fn register_default_tools(registry: &mut ToolRegistry) {
       registry.register_tool(Tool {
           name: "your_tool".to_string(),
           description: Some("Description of your tool".to_string()),
           input_schema: json!({
               "type": "object",
               "properties": {
                   // Define your parameters
               },
               "required": []
           }),
       });
   }
   ```

3. Add the tool handler in `src/tools/mod.rs`:
   ```rust
   pub fn call_tool(&self, name: &str, arguments: Option<serde_json::Value>) -> Result<ToolsCallResult, String> {
       match name {
           "your_tool" => your_tool::handle_your_tool(arguments),
           // ... other tools
       }
   }
   ```

### 4. Remove Example Tools (Optional)

To remove the example calculator tools:
1. Delete `src/tools/example_calculator.rs`
2. Remove the calculator tool registrations from `src/tools/mod.rs`
3. Remove the calculator handlers from the `call_tool` method

## 📁 Project Structure

```
├── src/
│   ├── lib.rs              # Main server implementation
│   ├── tools/              # Tool implementations
│   │   ├── mod.rs          # Tool registry and management
│   │   └── example_calculator.rs  # Example calculator tools
│   └── utils.rs            # Utility functions
├── tests/                  # Unit and integration tests
├── examples/               # Example scripts and usage
├── Cargo.toml              # Rust dependencies
├── package.json            # NPM scripts
├── wrangler.toml           # Cloudflare Workers config
└── README.md               # This file
```

## 🔧 Development

### Available Commands

- `cargo make dev` - Start development server
- `cargo make build` - Build for production
- `cargo make deploy` - Deploy to Cloudflare Workers
- `cargo make test` - Run Rust tests
- `cargo make test-integration` - Run integration tests
- `cargo make lint` - Run clippy linter
- `cargo make format` - Format code with rustfmt
- `cargo make ci` - Run full CI pipeline
- `cargo make clean` - Clean build artifacts
- `cargo make --list-all-steps` - Show all available tasks

### Task Aliases

- `cargo make t` - Alias for test
- `cargo make b` - Alias for build
- `cargo make d` - Alias for deploy
- `cargo make c` - Alias for check

### Testing

The template includes comprehensive tests:
- Unit tests for tools in `tests/integration_test.rs`
- Transport tests in `tests/transport_test.rs`
- Integration test script in `examples/test_mcp_client.sh`

### Adding Tests

Add tests for your tools in `tests/integration_test.rs`:
```rust
#[test]
fn test_your_tool() {
    // Your test implementation
}
```

## 🌐 API Reference

### MCP Protocol Endpoints

#### POST `/mcp` - Streamable HTTP Transport
The main MCP endpoint supporting JSON-RPC 2.0 requests.

**Supported Methods:**
- `initialize` - Initialize MCP session
- `tools/list` - List available tools
- `tools/call` - Execute a tool

**Headers:**
- `Content-Type: application/json`
- `Accept: application/json` or `Accept: text/event-stream`

#### GET `/sse` - Legacy SSE Transport
Returns SSE stream with endpoint information (for backward compatibility).

#### POST `/messages` - Legacy SSE Messages
Handles JSON-RPC messages for legacy SSE transport.

### Example Usage

```bash
# Initialize session
curl -X POST http://localhost:8787/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "initialize",
    "params": {
      "protocol_version": "2024-11-05",
      "capabilities": {},
      "client_info": {
        "name": "example-client",
        "version": "1.0.0"
      }
    },
    "id": 1
  }'

# List available tools
curl -X POST http://localhost:8787/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "tools/list",
    "id": 2
  }'

# Call a tool
curl -X POST http://localhost:8787/mcp \
  -H "Content-Type: application/json" \
  -d '{
    "jsonrpc": "2.0",
    "method": "tools/call",
    "params": {
      "name": "your_tool",
      "arguments": {
        // Your tool arguments
      }
    },
    "id": 3
  }'
```

## 🚀 Deployment

### Deploy to Cloudflare Workers

1. **Login to Cloudflare**
   ```bash
   wrangler login
   ```

2. **Configure your worker**
   Edit `wrangler.toml`:
   ```toml
   name = "your-mcp-server"
   ```

3. **Deploy**
   ```bash
   cargo make deploy
   ```

### Environment Variables

Add any required environment variables to your Cloudflare Worker:
```bash
wrangler secret put YOUR_SECRET_NAME
```

Access in your code:
```rust
let secret = ctx.env.secret("YOUR_SECRET_NAME")?;
```

## 📚 Resources

- [MCP Protocol Documentation](https://github.com/modelcontextprotocol/specification)
- [Cloudflare Workers Documentation](https://developers.cloudflare.com/workers/)
- [Rust WASM Documentation](https://rustwasm.github.io/book/)

## 🤝 Contributing

1. Fork the repository
2. Create your feature branch (`git checkout -b feature/amazing-feature`)
3. Commit your changes (`git commit -m 'Add some amazing feature'`)
4. Push to the branch (`git push origin feature/amazing-feature`)
5. Open a Pull Request

## 📄 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🙏 Acknowledgments

- Based on the MCP (Model Context Protocol) specification
- Built for deployment on Cloudflare Workers
- Inspired by the TypeScript MCP implementation