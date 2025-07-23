# Quick Start Guide

## Prerequisites

1. Install Rust: https://rustup.rs/
2. Install cargo-make: `cargo install cargo-make`

## Quick Start with cargo-generate (Recommended)

```bash
# Install cargo-generate
cargo install cargo-generate

# Create a new project from this template
cargo generate --git https://github.com/yourusername/mcp-server-rust-template.git

# Follow the prompts to configure your project
# Then navigate to your new project directory
cd your-project-name

# Initialize and start developing
cargo make init
cargo make dev
```

## Manual Setup

1. Clone this template:
   ```bash
   git clone https://github.com/yourusername/mcp-server-rust-template.git my-mcp-server
   cd my-mcp-server
   ```

2. Install dependencies:
   ```bash
   # Run the installation script
   ./install.sh
   
   # Or install manually:
   cargo install cargo-make
   cargo install worker-build
   cargo install wrangler
   ```

## Development

1. Start the development server:
   ```bash
   cargo make dev
   ```

2. Test the MCP endpoints:

   **Initialize session:**
   ```bash
   curl -X POST http://localhost:8787/mcp \
     -H "Content-Type: application/json" \
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
   ```

   **List available tools:**
   ```bash
   curl -X POST http://localhost:8787/mcp \
     -H "Content-Type: application/json" \
     -d '{
       "jsonrpc": "2.0",
       "method": "tools/list",
       "id": 2
     }'
   ```

   **Call a tool (example with add):**
   ```bash
   curl -X POST http://localhost:8787/mcp \
     -H "Content-Type: application/json" \
     -d '{
       "jsonrpc": "2.0",
       "method": "tools/call",
       "params": {
         "name": "add",
         "arguments": {"a": 5, "b": 3}
       },
       "id": 3
     }'
   ```

## Available Commands

```bash
# Development
cargo make dev              # Start dev server
cargo make watch            # Watch mode with auto-rebuild

# Testing
cargo make test             # Run all tests
cargo make test-integration # Run integration tests
cargo make coverage         # Generate coverage report

# Code Quality
cargo make format           # Format code
cargo make lint            # Run clippy
cargo make check           # Type check

# Building & Deployment
cargo make build           # Build for production
cargo make deploy          # Deploy to Cloudflare

# Utilities
cargo make clean           # Clean build artifacts
cargo make docs            # Generate documentation
cargo make --list-all-steps # Show all available tasks
```

## Project Structure

```
mcp-server-rust-template/
├── src/
│   ├── lib.rs              # Main server implementation
│   └── tools/              # Tool implementations
│       ├── mod.rs          # Tool registry
│       └── example_calculator.rs # Example tools
├── tests/                  # Unit and integration tests
├── examples/               # Example scripts
│   └── test_mcp_client.sh  # Integration test script
├── Cargo.toml              # Rust dependencies
├── Makefile.toml           # cargo-make tasks
├── cargo-generate.toml     # Template configuration
├── wrangler.toml           # Cloudflare config
└── README.md               # Full documentation
```

## Common Issues

1. **Build fails with worker-build error**
   - Run `cargo make install-deps` to install all dependencies

2. **Port already in use**
   - Kill any existing wrangler process or use: `wrangler dev --port 8788`

3. **Rust compilation errors**
   - Ensure you're using a recent Rust version: `rustup update`

4. **cargo-make not found**
   - Install it with: `cargo install cargo-make`

## Next Steps

1. Customize your server:
   - Update server constants in `src/lib.rs`
   - Add your own tools in `src/tools/`
   - See [TEMPLATE_CONFIG.md](TEMPLATE_CONFIG.md) for detailed customization guide

2. Test your implementation:
   - Run `cargo make test` for unit tests
   - Run `cargo make test-integration` for end-to-end tests

3. Deploy to production:
   - Configure your Cloudflare account
   - Run `cargo make deploy`

For more detailed documentation, see [README.md](README.md)