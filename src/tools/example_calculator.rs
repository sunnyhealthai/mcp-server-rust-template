use serde::{Deserialize, Serialize};
use crate::{ContentItem, ToolsCallResult};

#[derive(Debug, Deserialize)]
pub struct CalculateRequest {
    operation: Operation,
    a: f64,
    b: f64,
}

#[derive(Debug, Deserialize)]
pub struct AddRequest {
    a: f64,
    b: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}

pub fn handle_add(arguments: Option<serde_json::Value>) -> Result<ToolsCallResult, String> {
    match arguments {
        Some(args) => {
            match serde_json::from_value::<AddRequest>(args) {
                Ok(add_req) => {
                    let sum = add_req.a + add_req.b;
                    Ok(ToolsCallResult {
                        content: vec![ContentItem {
                            content_type: "text".to_string(),
                            text: sum.to_string(),
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

pub fn handle_calculate(arguments: Option<serde_json::Value>) -> Result<ToolsCallResult, String> {
    match arguments {
        Some(args) => {
            match serde_json::from_value::<CalculateRequest>(args) {
                Ok(calc_req) => {
                    let result = match calc_req.operation {
                        Operation::Add => calc_req.a + calc_req.b,
                        Operation::Subtract => calc_req.a - calc_req.b,
                        Operation::Multiply => calc_req.a * calc_req.b,
                        Operation::Divide => {
                            if calc_req.b == 0.0 {
                                return Ok(ToolsCallResult {
                                    content: vec![ContentItem {
                                        content_type: "text".to_string(),
                                        text: "Cannot divide by zero".to_string(),
                                    }],
                                    is_error: Some(true),
                                });
                            }
                            calc_req.a / calc_req.b
                        }
                    };
                    
                    Ok(ToolsCallResult {
                        content: vec![ContentItem {
                            content_type: "text".to_string(),
                            text: result.to_string(),
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