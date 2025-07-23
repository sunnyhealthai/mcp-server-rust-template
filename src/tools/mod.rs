use serde::{Deserialize, Serialize};
use serde_json::json;
use crate::{ContentItem, Tool, ToolsCallResult};

pub mod example_calculator;

#[derive(Debug, Serialize, Deserialize)]
pub struct ToolRegistry {
    tools: Vec<Tool>,
}

impl ToolRegistry {
    pub fn new() -> Self {
        Self {
            tools: Vec::new(),
        }
    }

    pub fn register_tool(&mut self, tool: Tool) {
        self.tools.push(tool);
    }

    pub fn get_tools(&self) -> &Vec<Tool> {
        &self.tools
    }

    pub fn call_tool(&self, name: &str, arguments: Option<serde_json::Value>) -> Result<ToolsCallResult, String> {
        match name {
            "add" => example_calculator::handle_add(arguments),
            "calculate" => example_calculator::handle_calculate(arguments),
            _ => Err(format!("Unknown tool: {}", name)),
        }
    }
}

pub fn register_default_tools(registry: &mut ToolRegistry) {
    // Register example calculator tools
    registry.register_tool(Tool {
        name: "add".to_string(),
        description: Some("Add two numbers".to_string()),
        input_schema: json!({
            "type": "object",
            "properties": {
                "a": { "type": "number" },
                "b": { "type": "number" }
            },
            "required": ["a", "b"]
        }),
    });

    registry.register_tool(Tool {
        name: "calculate".to_string(),
        description: Some("Perform arithmetic operations".to_string()),
        input_schema: json!({
            "type": "object",
            "properties": {
                "operation": { 
                    "type": "string", 
                    "enum": ["add", "subtract", "multiply", "divide"]
                },
                "a": { "type": "number" },
                "b": { "type": "number" }
            },
            "required": ["operation", "a", "b"]
        }),
    });
}