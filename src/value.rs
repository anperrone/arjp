use std::collections::HashMap;

/// Represents a JSON value according to the JSON specification.
#[derive(Debug, PartialEq, Clone)]
pub enum JsonValue {
    /// The JSON null value.
    Null,
    /// A JSON boolean value (true or false).
    Boolean(bool),
    /// A JSON number, stored as a 64-bit float.
    Number(f64),
    /// A JSON string.
    String(String),
    /// A JSON array containing a list of values.
    Array(Vec<JsonValue>),
    /// A JSON object containing key-value pairs.
    Object(HashMap<String, JsonValue>),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_null() {
        let value = JsonValue::Null;
        assert_eq!(value, JsonValue::Null);
    }

    #[test]
    fn test_boolean() {
        let true_value = JsonValue::Boolean(true);
        let false_value = JsonValue::Boolean(false);
        assert_eq!(true_value, JsonValue::Boolean(true));
        assert_eq!(false_value, JsonValue::Boolean(false));
        assert_ne!(true_value, false_value);
    }

    #[test]
    fn test_number() {
        let value = JsonValue::Number(123.45);
        assert_eq!(value, JsonValue::Number(123.45));
    }

    #[test]
    fn test_string() {
        let value = JsonValue::String("test".to_string());
        assert_eq!(value, JsonValue::String("test".to_string()));
    }

    #[test]
    fn test_array() {
        let value = JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Boolean(true)]);
        assert_eq!(
            value,
            JsonValue::Array(vec![JsonValue::Number(1.0), JsonValue::Boolean(true)])
        );
    }

    #[test]
    fn test_object() {
        let mut map = HashMap::new();
        map.insert("key".to_string(), JsonValue::String("value".to_string()));
        let value = JsonValue::Object(map.clone());
        assert_eq!(value, JsonValue::Object(map));
    }
}
