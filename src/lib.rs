use worker::*;
use serde::{Deserialize, Serialize};
use serde_json::json;

mod tools;
use tools::{ToolRegistry, register_default_tools};

// Core MCP types
#[derive(Debug, Serialize)]
struct McpResponse {
    content: Vec<ContentItem>,
}

#[derive(Debug, Serialize)]
struct ContentItem {
    #[serde(rename = "type")]
    content_type: String,
    text: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct JsonRpcRequest {
    jsonrpc: String,
    method: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    params: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct JsonRpcResponse {
    jsonrpc: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    result: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    error: Option<JsonRpcError>,
    #[serde(skip_serializing_if = "Option::is_none")]
    id: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct JsonRpcError {
    code: i32,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    data: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
struct InitializeResult {
    protocol_version: String,
    capabilities: ServerCapabilities,
    server_info: ServerInfo,
}

#[derive(Debug, Serialize)]
struct ServerCapabilities {
    experimental: Option<serde_json::Value>,
    logging: Option<serde_json::Value>,
    prompts: Option<PromptsCapability>,
    resources: Option<ResourcesCapability>,
    tools: Option<ToolsCapability>,
}

#[derive(Debug, Serialize)]
struct PromptsCapability {
    list_changed: Option<bool>,
}

#[derive(Debug, Serialize)]
struct ResourcesCapability {
    subscribe: Option<bool>,
    list_changed: Option<bool>,
}

#[derive(Debug, Serialize)]
struct ToolsCapability {
    list_changed: Option<bool>,
}

#[derive(Debug, Serialize)]
struct ServerInfo {
    name: String,
    version: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct InitializeParams {
    protocol_version: String,
    capabilities: ClientCapabilities,
    client_info: ClientInfo,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClientCapabilities {
    experimental: Option<serde_json::Value>,
    sampling: Option<serde_json::Value>,
    roots: Option<RootsCapability>,
}

#[derive(Debug, Serialize, Deserialize)]
struct RootsCapability {
    list_changed: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ClientInfo {
    name: String,
    version: String,
}

#[derive(Debug, Serialize)]
struct ToolsList {
    tools: Vec<Tool>,
}

#[derive(Debug, Serialize, Clone)]
pub struct Tool {
    name: String,
    description: Option<String>,
    input_schema: serde_json::Value,
}

#[derive(Debug, Deserialize)]
struct ToolsCallParams {
    name: String,
    arguments: Option<serde_json::Value>,
}

#[derive(Debug, Serialize)]
pub struct ToolsCallResult {
    content: Vec<ContentItem>,
    is_error: Option<bool>,
}

// Configuration constants - Update these for your MCP server
const SERVER_NAME: &str = "{{server_name}}";
const SERVER_VERSION: &str = "1.0.0";
const PROTOCOL_VERSION: &str = "2024-11-05";

fn handle_json_rpc_request(request: JsonRpcRequest, tool_registry: &ToolRegistry) -> JsonRpcResponse {
    match request.method.as_str() {
        "initialize" => {
            let result = InitializeResult {
                protocol_version: PROTOCOL_VERSION.to_string(),
                capabilities: ServerCapabilities {
                    experimental: None,
                    logging: None,
                    prompts: None,
                    resources: None,
                    tools: Some(ToolsCapability {
                        list_changed: Some(false),
                    }),
                },
                server_info: ServerInfo {
                    name: SERVER_NAME.to_string(),
                    version: SERVER_VERSION.to_string(),
                },
            };
            
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(serde_json::to_value(result).unwrap()),
                error: None,
                id: request.id,
            }
        },
        "tools/list" => {
            let tools = ToolsList {
                tools: tool_registry.get_tools().clone(),
            };
            
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: Some(serde_json::to_value(tools).unwrap()),
                error: None,
                id: request.id,
            }
        },
        "tools/call" => {
            if let Some(params) = request.params {
                if let Ok(call_params) = serde_json::from_value::<ToolsCallParams>(params) {
                    match tool_registry.call_tool(&call_params.name, call_params.arguments) {
                        Ok(result) => {
                            JsonRpcResponse {
                                jsonrpc: "2.0".to_string(),
                                result: Some(serde_json::to_value(result).unwrap()),
                                error: None,
                                id: request.id,
                            }
                        },
                        Err(e) => {
                            JsonRpcResponse {
                                jsonrpc: "2.0".to_string(),
                                result: None,
                                error: Some(JsonRpcError {
                                    code: -32603,
                                    message: format!("Internal error: {}", e),
                                    data: None,
                                }),
                                id: request.id,
                            }
                        }
                    }
                } else {
                    JsonRpcResponse {
                        jsonrpc: "2.0".to_string(),
                        result: None,
                        error: Some(JsonRpcError {
                            code: -32602,
                            message: "Invalid params".to_string(),
                            data: None,
                        }),
                        id: request.id,
                    }
                }
            } else {
                JsonRpcResponse {
                    jsonrpc: "2.0".to_string(),
                    result: None,
                    error: Some(JsonRpcError {
                        code: -32602,
                        message: "Missing params".to_string(),
                        data: None,
                    }),
                    id: request.id,
                }
            }
        },
        _ => {
            JsonRpcResponse {
                jsonrpc: "2.0".to_string(),
                result: None,
                error: Some(JsonRpcError {
                    code: -32601,
                    message: "Method not found".to_string(),
                    data: None,
                }),
                id: request.id,
            }
        }
    }
}

fn cors_headers() -> Headers {
    let mut headers = Headers::new();
    headers.append("Access-Control-Allow-Origin", "*").unwrap();
    headers.append("Access-Control-Allow-Methods", "GET, POST, OPTIONS").unwrap();
    headers.append("Access-Control-Allow-Headers", "Content-Type, Mcp-Session-Id").unwrap();
    headers.append("Access-Control-Max-Age", "86400").unwrap();
    headers
}

async fn handle_mcp_request(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    // Initialize tool registry
    let mut tool_registry = ToolRegistry::new();
    register_default_tools(&mut tool_registry);

    // Handle OPTIONS preflight
    if req.method() == Method::Options {
        return Response::empty()
            .map(|resp| resp.with_headers(cors_headers()));
    }

    // Parse JSON-RPC request
    let body = req.text().await?;
    let json_rpc_request: JsonRpcRequest = serde_json::from_str(&body)
        .map_err(|e| worker::Error::RustError(format!("Invalid JSON-RPC request: {}", e)))?;

    // Handle as notification if no ID
    if json_rpc_request.id.is_none() {
        return Response::empty()
            .map(|resp| resp.with_headers(cors_headers()));
    }

    // Process request
    let json_rpc_response = handle_json_rpc_request(json_rpc_request, &tool_registry);

    // Check Accept header for response format
    let accept_header = req.headers().get("Accept").unwrap_or(None).unwrap_or_default();
    
    if accept_header.contains("text/event-stream") {
        // Return SSE stream
        let sse_data = format!("data: {}\n\n", serde_json::to_string(&json_rpc_response)?);
        Response::ok(sse_data)
            .map(|resp| {
                let mut headers = cors_headers();
                headers.append("Content-Type", "text/event-stream").unwrap();
                headers.append("Cache-Control", "no-cache").unwrap();
                resp.with_headers(headers)
            })
    } else {
        // Return JSON
        Response::ok(serde_json::to_string(&json_rpc_response)?)
            .map(|resp| {
                let mut headers = cors_headers();
                headers.append("Content-Type", "application/json").unwrap();
                resp.with_headers(headers)
            })
    }
}

async fn handle_sse_get(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let sse_response = r#"event: endpoint
data: {"endpoint": "/messages"}

"#;
    
    Response::ok(sse_response)
        .map(|resp| {
            let mut headers = cors_headers();
            headers.append("Content-Type", "text/event-stream").unwrap();
            headers.append("Cache-Control", "no-cache").unwrap();
            resp.with_headers(headers)
        })
}

async fn handle_messages(req: Request, ctx: RouteContext<()>) -> Result<Response> {
    handle_mcp_request(req, ctx).await
}

// Legacy endpoints - kept for backward compatibility
async fn handle_legacy_add(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let body = req.text().await?;
    let add_request: serde_json::Value = serde_json::from_str(&body)?;
    
    let mut tool_registry = ToolRegistry::new();
    register_default_tools(&mut tool_registry);
    
    match tool_registry.call_tool("add", Some(add_request)) {
        Ok(result) => {
            let response = McpResponse {
                content: result.content,
            };
            Response::ok(serde_json::to_string(&response)?)
                .map(|resp| {
                    let mut headers = cors_headers();
                    headers.append("Content-Type", "application/json").unwrap();
                    resp.with_headers(headers)
                })
        },
        Err(e) => {
            Response::error(e, 400)
        }
    }
}

async fn handle_legacy_calculate(mut req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let body = req.text().await?;
    let calc_request: serde_json::Value = serde_json::from_str(&body)?;
    
    let mut tool_registry = ToolRegistry::new();
    register_default_tools(&mut tool_registry);
    
    match tool_registry.call_tool("calculate", Some(calc_request)) {
        Ok(result) => {
            let response = McpResponse {
                content: result.content,
            };
            Response::ok(serde_json::to_string(&response)?)
                .map(|resp| {
                    let mut headers = cors_headers();
                    headers.append("Content-Type", "application/json").unwrap();
                    resp.with_headers(headers)
                })
        },
        Err(e) => {
            Response::error(e, 400)
        }
    }
}

async fn handle_info(_req: Request, _ctx: RouteContext<()>) -> Result<Response> {
    let info = json!({
        "server": SERVER_NAME,
        "version": SERVER_VERSION,
        "protocol_version": PROTOCOL_VERSION,
        "transport": ["streamable_http", "legacy_sse"],
        "endpoints": {
            "mcp": "/mcp",
            "sse": "/sse",
            "messages": "/messages",
            "legacy": {
                "add": "/mcp/add",
                "calculate": "/mcp/calculate"
            }
        }
    });
    
    Response::ok(serde_json::to_string(&info)?)
        .map(|resp| {
            let mut headers = cors_headers();
            headers.append("Content-Type", "application/json").unwrap();
            resp.with_headers(headers)
        })
}

#[event(fetch)]
pub async fn main(req: Request, env: Env, ctx: Context) -> Result<Response> {
    utils::set_panic_hook();

    let router = Router::new();
    
    router
        // Main MCP endpoints
        .post_async("/mcp", handle_mcp_request)
        .options("/mcp", |_, _| Response::empty().map(|resp| resp.with_headers(cors_headers())))
        
        // Legacy SSE transport
        .get_async("/sse", handle_sse_get)
        .post_async("/messages", handle_messages)
        .options("/messages", |_, _| Response::empty().map(|resp| resp.with_headers(cors_headers())))
        
        // Legacy HTTP endpoints (kept for compatibility)
        .post_async("/mcp/add", handle_legacy_add)
        .post_async("/mcp/calculate", handle_legacy_calculate)
        .options("/mcp/add", |_, _| Response::empty().map(|resp| resp.with_headers(cors_headers())))
        .options("/mcp/calculate", |_, _| Response::empty().map(|resp| resp.with_headers(cors_headers())))
        
        // Info endpoint
        .get_async("/mcp", handle_info)
        .get_async("/", handle_info)
        
        .run(req, env)
        .await
}

mod utils {
    use console_error_panic_hook;

    pub fn set_panic_hook() {
        console_error_panic_hook::set_once();
    }
}