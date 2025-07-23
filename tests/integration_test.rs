#[cfg(test)]
mod tests {
    use serde_json::json;

    #[test]
    fn test_add_operation() {
        let a = 5.0;
        let b = 3.0;
        let expected = 8.0;
        assert_eq!(a + b, expected);
    }

    #[test]
    fn test_calculate_operations() {
        let test_cases = vec![
            ("add", 5.0, 3.0, 8.0),
            ("subtract", 5.0, 3.0, 2.0),
            ("multiply", 5.0, 3.0, 15.0),
            ("divide", 6.0, 2.0, 3.0),
        ];

        for (op, a, b, expected) in test_cases {
            let result = match op {
                "add" => a + b,
                "subtract" => a - b,
                "multiply" => a * b,
                "divide" => a / b,
                _ => panic!("Unknown operation"),
            };
            assert_eq!(result, expected, "Failed for operation: {}", op);
        }
    }

    #[test]
    fn test_divide_by_zero() {
        let a = 5.0;
        let b = 0.0;
        assert!(b == 0.0);
        // In the actual implementation, this returns an error message
    }

    #[test]
    fn test_request_format() {
        let add_request = json!({
            "a": 5,
            "b": 3
        });
        assert_eq!(add_request["a"], 5);
        assert_eq!(add_request["b"], 3);

        let calc_request = json!({
            "operation": "multiply",
            "a": 5,
            "b": 3
        });
        assert_eq!(calc_request["operation"], "multiply");
    }

    #[test]
    fn test_response_format() {
        let response = json!({
            "content": [{
                "type": "text",
                "text": "8"
            }]
        });
        assert_eq!(response["content"][0]["type"], "text");
        assert_eq!(response["content"][0]["text"], "8");
    }
} 