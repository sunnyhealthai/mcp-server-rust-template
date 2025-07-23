# Template Configuration Guide

This guide helps you customize the MCP Server Rust Template for your specific use case.

## Quick Customization Checklist

- [ ] Update server name and version in `src/lib.rs`
- [ ] Update package name and metadata in `Cargo.toml`
- [ ] Update worker name in `wrangler.toml`
- [ ] Remove or modify example calculator tools
- [ ] Add your custom tools
- [ ] Update tests for your tools
- [ ] Update README with your project information

## Step-by-Step Customization

### 1. Server Configuration

Edit `src/lib.rs` and update these constants:

```rust
const SERVER_NAME: &str = "My Custom MCP Server";  // Your server name
const SERVER_VERSION: &str = "1.0.0";              // Your version
const PROTOCOL_VERSION: &str = "2024-11-05";       // Keep this unless protocol changes
```

### 2. Package Metadata

Update `Cargo.toml`:
```toml
[package]
name = "my-custom-mcp-server"
version = "1.0.0"
authors = ["Your Name <your.email@example.com>"]
description = "Description of your MCP server"
repository = "https://github.com/yourusername/your-repo"
```

Update `wrangler.toml`:
```toml
name = "my-custom-mcp-server"
```

### 3. Implement Your Tools

#### Option A: Modify Example Tools

1. Edit `src/tools/example_calculator.rs` to implement your logic
2. Update tool names and schemas in `src/tools/mod.rs`

#### Option B: Create New Tools (Recommended)

1. Create a new file `src/tools/my_tool.rs`:

```rust
use serde::{Deserialize, Serialize};
use crate::{ContentItem, ToolsCallResult};

#[derive(Debug, Deserialize)]
pub struct MyToolRequest {
    // Define your input parameters
    input: String,
}

pub fn handle_my_tool(arguments: Option<serde_json::Value>) -> Result<ToolsCallResult, String> {
    match arguments {
        Some(args) => {
            match serde_json::from_value::<MyToolRequest>(args) {
                Ok(request) => {
                    // Your tool logic here
                    let result = format!("Processed: {}", request.input);
                    
                    Ok(ToolsCallResult {
                        content: vec![ContentItem {
                            content_type: "text".to_string(),
                            text: result,
                        }],
                        is_error: Some(false),
                    })
                },
                Err(e) => {
                    Ok(ToolsCallResult {
                        content: vec![ContentItem {
                            content_type: "text".to_string(),
                            text: format!("Invalid arguments: {}", e),
                        }],
                        is_error: Some(true),
                    })
                }
            }
        },
        None => {
            Ok(ToolsCallResult {
                content: vec![ContentItem {
                    content_type: "text".to_string(),
                    text: "Missing arguments".to_string(),
                }],
                is_error: Some(true),
            })
        }
    }
}
```

2. Update `src/tools/mod.rs`:

```rust
pub mod my_tool;  // Add your module

// In register_default_tools function:
registry.register_tool(Tool {
    name: "my_tool".to_string(),
    description: Some("Description of my tool".to_string()),
    input_schema: json!({
        "type": "object",
        "properties": {
            "input": { "type": "string" }
        },
        "required": ["input"]
    }),
});

// In call_tool function:
match name {
    "my_tool" => my_tool::handle_my_tool(arguments),
    // ... other tools
}
```

### 4. Update Tests

1. Add tests for your tools in `tests/integration_test.rs`:

```rust
#[test]
fn test_my_tool() {
    // Your test implementation
}
```

2. Update transport tests if needed in `tests/transport_test.rs`

### 5. Clean Up

Remove example tools if not needed:
1. Delete `src/tools/example_calculator.rs`
2. Remove calculator imports and registrations from `src/tools/mod.rs`
3. Remove calculator tests from `tests/integration_test.rs`

## Tool Development Best Practices

1. **Input Validation**: Always validate input parameters
2. **Error Handling**: Return meaningful error messages
3. **Type Safety**: Use Rust's type system for safety
4. **Documentation**: Add clear descriptions for tools
5. **Testing**: Write comprehensive tests for each tool

## Example Tool Patterns

### Simple Text Processing Tool
```rust
pub fn handle_uppercase(arguments: Option<serde_json::Value>) -> Result<ToolsCallResult, String> {
    // Converts input text to uppercase
}
```

### API Integration Tool
```rust
pub async fn handle_api_call(arguments: Option<serde_json::Value>) -> Result<ToolsCallResult, String> {
    // Makes external API calls
}
```

### Data Transformation Tool
```rust
pub fn handle_json_transform(arguments: Option<serde_json::Value>) -> Result<ToolsCallResult, String> {
    // Transforms JSON data
}
```

## Deployment Configuration

### Environment Variables

Add secrets to your Cloudflare Worker:
```bash
wrangler secret put API_KEY
wrangler secret put DATABASE_URL
```

Access in your code:
```rust
let api_key = ctx.env.secret("API_KEY")?;
```

### Custom Routes

Add new routes in `src/lib.rs`:
```rust
router
    .get_async("/health", handle_health_check)
    .post_async("/webhook", handle_webhook)
```

## Need Help?

- Check the [MCP Protocol Specification](https://github.com/modelcontextprotocol/specification)
- Review the example calculator implementation
- Refer to the Cloudflare Workers documentation