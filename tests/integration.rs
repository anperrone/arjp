use arjp::{parse_json, JsonParser, JsonValue};
use std::collections::HashMap;

#[test]
fn test_parse_null() {
    let mut parser = JsonParser::new("null");
    assert_eq!(parser.parse().unwrap(), JsonValue::Null);
    assert_eq!(parse_json("null").unwrap(), JsonValue::Null);
}

#[test]
fn test_parse_boolean() {
    let mut parser_true = JsonParser::new("true");
    assert_eq!(parser_true.parse().unwrap(), JsonValue::Boolean(true));
    assert_eq!(parse_json("true").unwrap(), JsonValue::Boolean(true));

    let mut parser_false = JsonParser::new("false");
    assert_eq!(parser_false.parse().unwrap(), JsonValue::Boolean(false));
    assert_eq!(parse_json("false").unwrap(), JsonValue::Boolean(false));
}

#[test]
fn test_parse_string() {
    let mut parser = JsonParser::new("\"hello\"");
    assert_eq!(
        parser.parse().unwrap(),
        JsonValue::String("hello".to_string())
    );
    assert_eq!(
        parse_json("\"hello\"").unwrap(),
        JsonValue::String("hello".to_string())
    );

    let mut parser_escaped = JsonParser::new("\"hello \\\"world\\\"\"");
    assert_eq!(
        parser_escaped.parse().unwrap(),
        JsonValue::String("hello \"world\"".to_string())
    );
    assert_eq!(
        parse_json("\"hello \\\"world\\\"\"").unwrap(),
        JsonValue::String("hello \"world\"".to_string())
    );

    let mut parser_unicode = JsonParser::new("\"unicode \\u263A\"");
    assert_eq!(
        parser_unicode.parse().unwrap(),
        JsonValue::String("unicode ☺".to_string())
    );
    assert_eq!(
        parse_json("\"unicode \\u263A\"").unwrap(),
        JsonValue::String("unicode ☺".to_string())
    );
}

#[test]
fn test_parse_number() {
    let mut parser_int = JsonParser::new("123");
    assert_eq!(parser_int.parse().unwrap(), JsonValue::Number(123.0));
    assert_eq!(parse_json("123").unwrap(), JsonValue::Number(123.0));

    let mut parser_float = JsonParser::new("-456.789");
    assert_eq!(parser_float.parse().unwrap(), JsonValue::Number(-456.789));
    assert_eq!(parse_json("-456.789").unwrap(), JsonValue::Number(-456.789));

    let mut parser_exp = JsonParser::new("1.23e-4");
    assert_eq!(parser_exp.parse().unwrap(), JsonValue::Number(1.23e-4));
    assert_eq!(parse_json("1.23e-4").unwrap(), JsonValue::Number(1.23e-4));
}

#[test]
fn test_parse_array() {
    let mut parser_empty = JsonParser::new("[]");
    assert_eq!(parser_empty.parse().unwrap(), JsonValue::Array(vec![]));
    assert_eq!(parse_json("[]").unwrap(), JsonValue::Array(vec![]));

    let mut parser = JsonParser::new("[1, \"test\", true]");
    assert_eq!(
        parser.parse().unwrap(),
        JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::String("test".to_string()),
            JsonValue::Boolean(true)
        ])
    );
    assert_eq!(
        parse_json("[1, \"test\", true]").unwrap(),
        JsonValue::Array(vec![
            JsonValue::Number(1.0),
            JsonValue::String("test".to_string()),
            JsonValue::Boolean(true)
        ])
    );
}

#[test]
fn test_parse_object() {
    let mut parser_empty = JsonParser::new("{}");
    assert_eq!(
        parser_empty.parse().unwrap(),
        JsonValue::Object(HashMap::new())
    );
    assert_eq!(parse_json("{}").unwrap(), JsonValue::Object(HashMap::new()));

    let mut parser = JsonParser::new("{\"name\": \"John\", \"age\": 30}");
    let mut expected = HashMap::new();
    expected.insert("name".to_string(), JsonValue::String("John".to_string()));
    expected.insert("age".to_string(), JsonValue::Number(30.0));
    assert_eq!(parser.parse().unwrap(), JsonValue::Object(expected.clone()));
    assert_eq!(
        parse_json("{\"name\": \"John\", \"age\": 30}").unwrap(),
        JsonValue::Object(expected)
    );
}

#[test]
fn test_parse_errors() {
    let mut parser_unterminated = JsonParser::new("\"unterminated");
    assert!(parser_unterminated.parse().is_err());
    assert!(parse_json("\"unterminated").is_err());

    let mut parser_invalid = JsonParser::new("invalid");
    assert!(parser_invalid.parse().is_err());
    assert!(parse_json("invalid").is_err());

    let mut parser_unterminated_array = JsonParser::new("[1,");
    assert!(parser_unterminated_array.parse().is_err());
    assert!(parse_json("[1,").is_err());
}
