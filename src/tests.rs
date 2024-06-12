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

    #[test]
    fn test_json_org_fail_1() {
        let path = String::from("./tests/json_org_tests/fail1.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_10() {
        let path = String::from("./tests/json_org_tests/fail10.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_11() {
        let path = String::from("./tests/json_org_tests/fail11.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_12() {
        let path = String::from("./tests/json_org_tests/fail12.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_13() {
        let path = String::from("./tests/json_org_tests/fail13.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_14() {
        let path = String::from("./tests/json_org_tests/fail14.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_15() {
        let path = String::from("./tests/json_org_tests/fail15.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_16() {
        let path = String::from("./tests/json_org_tests/fail16.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_17() {
        let path = String::from("./tests/json_org_tests/fail17.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_18() {
        let path = String::from("./tests/json_org_tests/fail18.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_19() {
        let path = String::from("./tests/json_org_tests/fail19.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_2() {
        let path = String::from("./tests/json_org_tests/fail2.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_20() {
        let path = String::from("./tests/json_org_tests/fail20.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_21() {
        let path = String::from("./tests/json_org_tests/fail21.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_22() {
        let path = String::from("./tests/json_org_tests/fail22.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_23() {
        let path = String::from("./tests/json_org_tests/fail23.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_24() {
        let path = String::from("./tests/json_org_tests/fail24.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_25() {
        let path = String::from("./tests/json_org_tests/fail25.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_26() {
        let path = String::from("./tests/json_org_tests/fail26.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_27() {
        let path = String::from("./tests/json_org_tests/fail27.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_28() {
        let path = String::from("./tests/json_org_tests/fail28.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_29() {
        let path = String::from("./tests/json_org_tests/fail29.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_3() {
        let path = String::from("./tests/json_org_tests/fail3.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_30() {
        let path = String::from("./tests/json_org_tests/fail30.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_31() {
        let path = String::from("./tests/json_org_tests/fail31.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_32() {
        let path = String::from("./tests/json_org_tests/fail32.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_33() {
        let path = String::from("./tests/json_org_tests/fail33.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_4() {
        let path = String::from("./tests/json_org_tests/fail4.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_5() {
        let path = String::from("./tests/json_org_tests/fail5.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_6() {
        let path = String::from("./tests/json_org_tests/fail6.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_7() {
        let path = String::from("./tests/json_org_tests/fail7.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_8() {
        let path = String::from("./tests/json_org_tests/fail8.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_fail_9() {
        let path = String::from("./tests/json_org_tests/fail9.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true);
    }

    #[test]
    fn test_json_org_pass_1() {
        let path = String::from("./tests/json_org_tests/pass1.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), false);
    }

    #[test]
    fn test_json_org_pass_2() {
        let path = String::from("./tests/json_org_tests/pass2.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), false);
    }

    #[test]
    fn test_json_org_pass_3() {
        let path = String::from("./tests/json_org_tests/pass3.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), false);
    }
}
