#[cfg(test)]
mod tests {
    use serde_json::json;

    // Helper function to create a valid JSON-RPC request
    fn create_json_rpc_request(method: &str, params: Option<serde_json::Value>, id: Option<serde_json::Value>) -> serde_json::Value {
        let mut request = json!({
            "jsonrpc": "2.0",
            "method": method
        });
        
        if let Some(p) = params {
            request["params"] = p;
        }
        
        if let Some(i) = id {
            request["id"] = i;
        }
        
        request
    }

    #[test]
    fn test_initialize_request() {
        let request = create_json_rpc_request(
            "initialize",
            Some(json!({
                "protocol_version": "2024-11-05",
                "capabilities": {},
                "client_info": {
                    "name": "test-client",
                    "version": "1.0.0"
                }
            })),
            Some(json!(1))
        );
        
        assert_eq!(request["jsonrpc"], "2.0");
        assert_eq!(request["method"], "initialize");
        assert_eq!(request["id"], 1);
    }

    #[test]
    fn test_tools_list_request() {
        let request = create_json_rpc_request(
            "tools/list",
            None,
            Some(json!(2))
        );
        
        assert_eq!(request["jsonrpc"], "2.0");
        assert_eq!(request["method"], "tools/list");
        assert_eq!(request["id"], 2);
    }

    #[test]
    fn test_tools_call_request() {
        let request = create_json_rpc_request(
            "tools/call",
            Some(json!({
                "name": "add",
                "arguments": {
                    "a": 5,
                    "b": 3
                }
            })),
            Some(json!(3))
        );
        
        assert_eq!(request["jsonrpc"], "2.0");
        assert_eq!(request["method"], "tools/call");
        assert_eq!(request["params"]["name"], "add");
        assert_eq!(request["params"]["arguments"]["a"], 5);
        assert_eq!(request["params"]["arguments"]["b"], 3);
    }

    #[test]
    fn test_calculate_tool_call() {
        let request = create_json_rpc_request(
            "tools/call",
            Some(json!({
                "name": "calculate",
                "arguments": {
                    "operation": "multiply",
                    "a": 4,
                    "b": 7
                }
            })),
            Some(json!(4))
        );
        
        assert_eq!(request["params"]["name"], "calculate");
        assert_eq!(request["params"]["arguments"]["operation"], "multiply");
    }

    #[test]
    fn test_notification_without_id() {
        let notification = create_json_rpc_request(
            "notification/test",
            Some(json!({"data": "test"})),
            None
        );
        
        assert_eq!(notification["jsonrpc"], "2.0");
        assert_eq!(notification["method"], "notification/test");
        assert!(!notification.as_object().unwrap().contains_key("id"));
    }
}