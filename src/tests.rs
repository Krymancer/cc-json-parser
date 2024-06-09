#[cfg(test)]
mod tests {
    use crate::parse_json;

    #[test]
    fn test_invalid_path() {
        let path = String::from("invalid/path");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true)
    }

    #[test]
    fn test_step1_valid() {
        let path = String::from("./tests/step1/valid.json");
        let result = parse_json(path);
        assert_eq!(result.is_ok(), true)
    }

    #[test]
    fn test_step1_invalid() {
        let path = String::from("./tests/step1/invalid.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true)
    }

    #[test]
    fn test_step2_valid() {
        let path = String::from("./tests/step2/valid.json");
        let result = parse_json(path);
        assert_eq!(result.is_ok(), true)
    }

    #[test]
    fn test_step2_invalid() {
        let path = String::from("./tests/step2/invalid.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true)
    }

    #[test]
    fn test_step2_valid2() {
        let path = String::from("./tests/step2/valid2.json");
        let result = parse_json(path);
        assert_eq!(result.is_ok(), true)
    }

    #[test]
    fn test_step2_invalid2() {
        let path = String::from("./tests/step2/invalid2.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true)
    }

    #[test]
    fn test_step3_valid() {
        let path = String::from("./tests/step3/valid.json");
        let result = parse_json(path);
        assert_eq!(result.is_ok(), true)
    }

    #[test]
    fn test_step3_invalid() {
        let path = String::from("./tests/step3/invalid.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true)
    }

    #[test]
    fn test_step4_valid() {
        let path = String::from("./tests/step4/valid.json");
        let result = parse_json(path);
        assert_eq!(result.is_ok(), true)
    }

    #[test]
    fn test_step4_invalid() {
        let path = String::from("./tests/step4/invalid.json");
        let result = parse_json(path);
        assert_eq!(result.is_err(), true)
    }

    #[test]
    fn test_step4_valid2() {
        let path = String::from("./tests/step4/valid2.json");
        let result = parse_json(path);
        assert_eq!(result.is_ok(), true)
    }
}
