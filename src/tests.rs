#[cfg(test)]
mod tests {
    use crate::parser::{parse_json, JsonValue};

    #[test]
    fn test_invalid_path() {
        let path = String::from("invalid/path");
        let result = parse_json(path);
        assert!(result.is_err());

        if let Err(e) = result {
            assert_eq!(e.to_string(), "Falied to read File");
        }
    }

    #[test]
    fn test_step1_valid() {
        let path = String::from("./tests/step1/valid.json");
        let result = parse_json(path).expect("Error parsing JSON");
        assert_eq!(result, JsonValue::Object(vec![]));
    }

    #[test]
    fn test_step1_invalid() {
        let path = String::from("./tests/step1/invalid.json");
        let result = parse_json(path);
        assert!(result.is_err());

        if let Err(e) = result {
            assert_eq!(e.to_string(), "Empty file");
        }
    }

    #[test]
    fn test_step2_valid() {
        let path = String::from("./tests/step2/valid.json");
        let result = parse_json(path).expect("Falied to parse JSON");
        assert_eq!(
            result,
            JsonValue::Object(vec![(
                "key".to_string(),
                JsonValue::String("value".to_string())
            )])
        );
    }

    #[test]
    fn test_step2_invalid() {
        let path = String::from("./tests/step2/invalid.json");
        let result = parse_json(path);
        assert!(result.is_err());

        if let Err(e) = result {
            assert_eq!(e.to_string(), "Trailing comma in object");
        }
    }

    #[test]
    fn test_step2_valid2() {
        let path = String::from("./tests/step2/valid2.json");
        let result = parse_json(path).expect("Falied to parse JSON");
        assert_eq!(
            result,
            JsonValue::Object(vec![
                ("key".to_string(), JsonValue::String("value".to_string())),
                ("key2".to_string(), JsonValue::String("value".to_string()))
            ])
        );
    }

    #[test]
    fn test_step2_invalid2() {
        let path = String::from("./tests/step2/invalid2.json");
        let result = parse_json(path);
        assert!(result.is_err());

        if let Err(e) = result {
            assert_eq!(e.to_string(), "Unexpected character: k");
        }
    }

    #[test]
    fn test_step3_valid() {
        let path = String::from("./tests/step3/valid.json");
        let result = parse_json(path).expect("Falied to parse JSON");
        assert_eq!(
            result,
            JsonValue::Object(vec![
                ("key1".to_string(), JsonValue::Bool(true)),
                ("key2".to_string(), JsonValue::Bool(false)),
                ("key3".to_string(), JsonValue::Null),
                ("key4".to_string(), JsonValue::String("value".to_string())),
                ("key5".to_string(), JsonValue::Number(101.0))
            ])
        );
    }

    #[test]
    fn test_step3_invalid() {
        let path = String::from("./tests/step3/invalid.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);

        if let Err(e) = result {
            assert_eq!(e.to_string(), "Unexpected character: F");
        }
    }

    #[test]
    fn test_step4_valid() {
        let path = String::from("./tests/step4/valid.json");
        let result = parse_json(path).expect("Falied to parse JSON");
        assert_eq!(
            result,
            JsonValue::Object(vec![
                ("key".to_string(), JsonValue::String("value".to_string())),
                ("key-n".to_string(), JsonValue::Number(101.0)),
                ("key-o".to_string(), JsonValue::Object(vec![])),
                ("key-l".to_string(), JsonValue::Array(vec![]))
            ])
        );
    }

    #[test]
    fn test_step4_invalid() {
        let path = String::from("./tests/step4/invalid.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);

        if let Err(e) = result {
            assert_eq!(e.to_string(), "Unexpected character: '");
        }
    }

    #[test]
    fn test_step4_valid2() {
        let path = String::from("./tests/step4/valid2.json");
        let result = parse_json(path).expect("Falied to parse JSON");
        assert_eq!(
            result,
            JsonValue::Object(vec![
                ("key".to_string(), JsonValue::String("value".to_string())),
                ("key-n".to_string(), JsonValue::Number(101.0)),
                (
                    "key-o".to_string(),
                    JsonValue::Object(vec![(
                        "inner key".to_string(),
                        JsonValue::String("inner value".to_string())
                    )])
                ),
                (
                    "key-l".to_string(),
                    JsonValue::Array(vec![JsonValue::String("list value".to_string())])
                )
            ])
        );
    }

    // Custom test that I made to guarantee basic checks like nulls, nesting
    // Negative and floating point numbers
    #[test]
    fn test_custom_valid() {
        let path = String::from("./tests/custom/valid.json");
        let result = parse_json(path).expect("Falied to parse JSON");
        assert_eq!(
            result,
            JsonValue::Object(vec![
                ("string".to_string(), JsonValue::String("value".to_string())),
                ("positive_number".to_string(), JsonValue::Number(123.456)),
                ("negative_number".to_string(), JsonValue::Number(-789.123)),
                ("boolean_true".to_string(), JsonValue::Bool(true)),
                ("boolean_false".to_string(), JsonValue::Bool(false)),
                ("null_value".to_string(), JsonValue::Null),
                (
                    "object".to_string(),
                    JsonValue::Object(vec![
                        (
                            "nested_string".to_string(),
                            JsonValue::String("nested_value".to_string())
                        ),
                        ("nested_number".to_string(), JsonValue::Number(789.0)),
                        (
                            "nested_negative_number".to_string(),
                            JsonValue::Number(-456.0)
                        ),
                        ("nested_float".to_string(), JsonValue::Number(0.987)),
                        (
                            "nested_object".to_string(),
                            JsonValue::Object(vec![(
                                "inner_key".to_string(),
                                JsonValue::String("inner_value".to_string())
                            )])
                        ),
                        (
                            "nested_array".to_string(),
                            JsonValue::Array(vec![
                                JsonValue::Number(1.0),
                                JsonValue::Number(-2.0),
                                JsonValue::Number(3.14),
                                JsonValue::String("four".to_string()),
                                JsonValue::Bool(true),
                                JsonValue::Null
                            ])
                        )
                    ])
                ),
                (
                    "array".to_string(),
                    JsonValue::Array(vec![
                        JsonValue::String("string_in_array".to_string()),
                        JsonValue::Number(42.0),
                        JsonValue::Number(-99.0),
                        JsonValue::Number(3.1415),
                        JsonValue::Bool(false),
                        JsonValue::Null,
                        JsonValue::Object(vec![(
                            "array_object_key".to_string(),
                            JsonValue::String("array_object_value".to_string())
                        )]),
                        JsonValue::Array(vec![
                            JsonValue::String("nested_array_in_array".to_string()),
                            JsonValue::Number(-45.67)
                        ])
                    ])
                ),
                ("empty_object".to_string(), JsonValue::Object(vec![])),
                ("empty_array".to_string(), JsonValue::Array(vec![]))
            ])
        );
    }
}
