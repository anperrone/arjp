/// Custom error type for JSON parsing failures.
#[derive(Debug)]
pub struct ParseError {
    message: String,
}

impl ParseError {
    /// Creates a new ParseError with the given message.
    pub fn new(message: &str) -> Self {
        ParseError {
            message: message.to_string(),
        }
    }
}

impl std::fmt::Display for ParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for ParseError {}

/// Result type alias for parsing operations.
pub type Result<T> = std::result::Result<T, ParseError>;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let error = ParseError::new("test error");
        assert_eq!(error.message, "test error");
    }

    #[test]
    fn test_error_display() {
        let error = ParseError::new("test error");
        assert_eq!(format!("{}", error), "test error");
    }

    #[test]
    fn test_error_as_std_error() {
        let error = ParseError::new("test error");
        let std_error: &dyn std::error::Error = &error;
        assert_eq!(std_error.to_string(), "test error");
    }
}
