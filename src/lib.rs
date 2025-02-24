//! ARJP - Another Rust JSON Parser
//!
//! A simple and efficient JSON parsing library with performance optimizations.
//!
//! This library provides a straightforward way to parse JSON strings into a structured
//! representation. It supports all JSON data types with minimal dependencies and includes
//! optimizations such as pre-allocated data structures and efficient character handling.
//!
//! # Example
//!
//! ```rust
//! use arjp::{JsonParser, JsonValue, parse_json};
//!
//! let json_str = r#"{"name": "John", "age": 30}"#;
//! let mut parser = JsonParser::new(json_str);
//! let value = parser.parse().unwrap();
//! println!("Parsed: {:?}", value);
//!
//! // Or use the convenience function:
//! let value = parse_json(json_str).unwrap();
//! println!("Parsed: {:?}", value);
//! ```

mod error;
mod parser;
mod value;

pub use error::{ParseError, Result};
pub use parser::JsonParser;
pub use value::JsonValue;

/// Convenience function to parse a JSON string in one step.
///
/// # Arguments
///
/// * `input` - The JSON string to parse.
///
/// # Returns
///
/// A `Result` containing the parsed `JsonValue` or a `ParseError`.
pub fn parse_json(input: &str) -> Result<JsonValue> {
    let mut parser = JsonParser::new(input);
    parser.parse()
}
