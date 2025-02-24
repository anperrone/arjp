use crate::error::{ParseError, Result};
use crate::value::JsonValue;
use std::collections::HashMap;

/// JSON parser implementation with performance optimizations.
pub struct JsonParser<'a> {
    input: &'a str,
    position: usize,
    remaining: &'a str,
}

impl<'a> JsonParser<'a> {
    /// Creates a new parser instance for the given JSON string.
    pub fn new(input: &'a str) -> Self {
        JsonParser {
            input,
            position: 0,
            remaining: input,
        }
    }

    /// Advances the parser to the next character and returns it.
    #[inline]
    fn next_char(&mut self) -> Option<char> {
        let c = self.remaining.chars().next();
        if let Some(ch) = c {
            self.position += ch.len_utf8();
            self.remaining = &self.input[self.position..];
        }
        c
    }

    /// Skips whitespace characters efficiently.
    #[inline]
    fn skip_whitespace(&mut self) {
        while let Some(c) = self.remaining.chars().next() {
            if !c.is_whitespace() {
                break;
            }
            self.next_char();
        }
    }

    /// Parses the input JSON string into a JsonValue.
    pub fn parse(&mut self) -> Result<JsonValue> {
        self.skip_whitespace();
        let result = self.parse_value()?;
        self.skip_whitespace();
        if !self.remaining.is_empty() {
            return Err(ParseError::new("Extra characters after JSON value"));
        }
        Ok(result)
    }

    /// Parses any JSON value (null, boolean, string, number, array, or object).
    fn parse_value(&mut self) -> Result<JsonValue> {
        self.skip_whitespace();
        match self.remaining.chars().next() {
            Some('n') => self.parse_null(),
            Some('t') | Some('f') => self.parse_boolean(),
            Some('"') => self.parse_string(),
            Some('[') => self.parse_array(),
            Some('{') => self.parse_object(),
            Some('0'..='9') | Some('-') => self.parse_number(),
            Some(_) => Err(ParseError::new("Unexpected character")),
            None => Err(ParseError::new("Unexpected end of input")),
        }
    }

    /// Parses the JSON null value.
    fn parse_null(&mut self) -> Result<JsonValue> {
        if self.remaining.starts_with("null") {
            self.position += 4;
            self.remaining = &self.input[self.position..];
            Ok(JsonValue::Null)
        } else {
            Err(ParseError::new("Invalid null value"))
        }
    }

    /// Parses a JSON boolean value (true or false).
    fn parse_boolean(&mut self) -> Result<JsonValue> {
        match self.remaining {
            s if s.starts_with("true") => {
                self.position += 4;
                self.remaining = &self.input[self.position..];
                Ok(JsonValue::Boolean(true))
            }
            s if s.starts_with("false") => {
                self.position += 5;
                self.remaining = &self.input[self.position..];
                Ok(JsonValue::Boolean(false))
            }
            _ => Err(ParseError::new("Invalid boolean value")),
        }
    }

    /// Parses a JSON string with optimized character handling.
    fn parse_string(&mut self) -> Result<JsonValue> {
        self.next_char(); // Skip opening quote
        let mut result = String::with_capacity(16);

        while let Some(c) = self.remaining.chars().next() {
            match c {
                '"' => {
                    self.next_char();
                    return Ok(JsonValue::String(result));
                }
                '\\' => {
                    self.next_char();
                    match self.next_char() {
                        Some('"') => result.push('"'),
                        Some('\\') => result.push('\\'),
                        Some('/') => result.push('/'),
                        Some('b') => result.push('\u{0008}'),
                        Some('f') => result.push('\u{000C}'),
                        Some('n') => result.push('\n'),
                        Some('r') => result.push('\r'),
                        Some('t') => result.push('\t'),
                        Some('u') => {
                            let mut code = 0u16;
                            for i in (0..4).rev() {
                                let c = self
                                    .next_char()
                                    .ok_or(ParseError::new("Incomplete unicode escape sequence"))?;
                                if let Some(digit) = c.to_digit(16) {
                                    code |= (digit as u16) << (i * 4);
                                } else {
                                    return Err(ParseError::new("Invalid unicode escape sequence"));
                                }
                            }
                            result.push(char::from_u32(code as u32).ok_or_else(|| {
                                ParseError::new(&format!("Invalid unicode code point: {}", code))
                            })?);
                        }
                        Some(c) => {
                            return Err(ParseError::new(&format!(
                                "Invalid escape sequence: \\{}",
                                c
                            )))
                        }
                        None => return Err(ParseError::new("Unterminated string after escape")),
                    }
                }
                _ => {
                    result.push(c);
                    self.next_char();
                }
            }
        }
        Err(ParseError::new("Unterminated string"))
    }

    /// Parses a JSON number with optimized string construction.
    fn parse_number(&mut self) -> Result<JsonValue> {
        let mut num_str = String::with_capacity(16);
        let mut has_digits = false;

        if self.remaining.starts_with('-') {
            num_str.push('-');
            self.next_char();
        }
        while let Some(c) = self.remaining.chars().next() {
            if c.is_ascii_digit() {
                num_str.push(c);
                has_digits = true;
                self.next_char();
            } else {
                break;
            }
        }
        if !has_digits {
            return Err(ParseError::new("Number must contain at least one digit"));
        }
        if self.remaining.starts_with('.') {
            num_str.push('.');
            self.next_char();
            has_digits = false;
            while let Some(c) = self.remaining.chars().next() {
                if c.is_ascii_digit() {
                    num_str.push(c);
                    has_digits = true;
                    self.next_char();
                } else {
                    break;
                }
            }
            if !has_digits {
                return Err(ParseError::new(
                    "Decimal point must be followed by at least one digit",
                ));
            }
        }
        if self.remaining.starts_with('e') || self.remaining.starts_with('E') {
            num_str.push(self.remaining.chars().next().unwrap());
            self.next_char();
            if self.remaining.starts_with('+') || self.remaining.starts_with('-') {
                num_str.push(self.remaining.chars().next().unwrap());
                self.next_char();
            }
            has_digits = false;
            while let Some(c) = self.remaining.chars().next() {
                if c.is_ascii_digit() {
                    num_str.push(c);
                    has_digits = true;
                    self.next_char();
                } else {
                    break;
                }
            }
            if !has_digits {
                return Err(ParseError::new(
                    "Exponent must be followed by at least one digit",
                ));
            }
        }
        num_str
            .parse::<f64>()
            .map(JsonValue::Number)
            .map_err(|e| ParseError::new(&format!("Invalid number: {}", e)))
    }

    /// Parses a JSON array with pre-allocated capacity.
    fn parse_array(&mut self) -> Result<JsonValue> {
        self.next_char();
        self.skip_whitespace();
        let mut values = Vec::with_capacity(4);

        if self.remaining.starts_with(']') {
            self.next_char();
            return Ok(JsonValue::Array(values));
        }

        loop {
            values.push(self.parse_value()?);
            self.skip_whitespace();
            match self.remaining.chars().next() {
                Some(']') => {
                    self.next_char();
                    return Ok(JsonValue::Array(values));
                }
                Some(',') => {
                    self.next_char();
                    self.skip_whitespace();
                }
                Some(c) => {
                    return Err(ParseError::new(&format!(
                        "Expected comma or closing bracket, got '{}'",
                        c
                    )))
                }
                None => return Err(ParseError::new("Unterminated array")),
            }
        }
    }

    /// Parses a JSON object with pre-allocated capacity.
    fn parse_object(&mut self) -> Result<JsonValue> {
        self.next_char();
        self.skip_whitespace();
        let mut map = HashMap::with_capacity(4);

        if self.remaining.starts_with('}') {
            self.next_char();
            return Ok(JsonValue::Object(map));
        }

        loop {
            let key = match self.parse_value()? {
                JsonValue::String(s) => s,
                _ => return Err(ParseError::new("Object keys must be strings")),
            };
            self.skip_whitespace();
            if !self.remaining.starts_with(':') {
                return Err(ParseError::new("Expected colon after key in object"));
            }
            self.next_char();
            map.insert(key, self.parse_value()?);
            self.skip_whitespace();
            match self.remaining.chars().next() {
                Some('}') => {
                    self.next_char();
                    return Ok(JsonValue::Object(map));
                }
                Some(',') => {
                    self.next_char();
                    self.skip_whitespace();
                }
                Some(c) => {
                    return Err(ParseError::new(&format!(
                        "Expected comma or closing brace, got '{}'",
                        c
                    )))
                }
                None => return Err(ParseError::new("Unterminated object")),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_null() {
        let mut parser = JsonParser::new("null");
        assert_eq!(parser.parse_null().unwrap(), JsonValue::Null);
    }

    #[test]
    fn test_parse_boolean() {
        let mut parser_true = JsonParser::new("true");
        assert_eq!(
            parser_true.parse_boolean().unwrap(),
            JsonValue::Boolean(true)
        );

        let mut parser_false = JsonParser::new("false");
        assert_eq!(
            parser_false.parse_boolean().unwrap(),
            JsonValue::Boolean(false)
        );
    }

    #[test]
    fn test_parse_string() {
        let mut parser = JsonParser::new("\"hello\"");
        assert_eq!(
            parser.parse_string().unwrap(),
            JsonValue::String("hello".to_string())
        );

        let mut parser_escaped = JsonParser::new("\"hello \\\"world\\\"\"");
        assert_eq!(
            parser_escaped.parse_string().unwrap(),
            JsonValue::String("hello \"world\"".to_string())
        );

        let mut parser_unicode = JsonParser::new("\"\\u263A\"");
        assert_eq!(
            parser_unicode.parse_string().unwrap(),
            JsonValue::String("☺".to_string())
        );
    }

    #[test]
    fn test_parse_number() {
        let mut parser_int = JsonParser::new("123");
        assert_eq!(parser_int.parse_number().unwrap(), JsonValue::Number(123.0));

        let mut parser_float = JsonParser::new("-456.789");
        assert_eq!(
            parser_float.parse_number().unwrap(),
            JsonValue::Number(-456.789)
        );

        let mut parser_exp = JsonParser::new("1.23e-4");
        assert_eq!(
            parser_exp.parse_number().unwrap(),
            JsonValue::Number(1.23e-4)
        );
    }

    #[test]
    fn test_parse_array() {
        let mut parser_empty = JsonParser::new("[]");
        assert_eq!(
            parser_empty.parse_array().unwrap(),
            JsonValue::Array(vec![])
        );

        let mut parser = JsonParser::new("[1, \"test\"]");
        assert_eq!(
            parser.parse_array().unwrap(),
            JsonValue::Array(vec![
                JsonValue::Number(1.0),
                JsonValue::String("test".to_string())
            ])
        );
    }

    #[test]
    fn test_parse_object() {
        let mut parser_empty = JsonParser::new("{}");
        assert_eq!(
            parser_empty.parse_object().unwrap(),
            JsonValue::Object(HashMap::new())
        );

        let mut parser = JsonParser::new("{\"key\": \"value\"}");
        let mut expected = HashMap::new();
        expected.insert("key".to_string(), JsonValue::String("value".to_string()));
        assert_eq!(parser.parse_object().unwrap(), JsonValue::Object(expected));
    }
}
