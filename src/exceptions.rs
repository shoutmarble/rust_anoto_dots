use std::fmt;

/// A base error type for codec-related issues.
#[derive(Debug, Clone, PartialEq)]
pub enum CodecError {
    Decoding(DecodingError),
}

impl fmt::Display for CodecError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CodecError::Decoding(e) => write!(f, "Decoding error: {}", e),
        }
    }
}

impl std::error::Error for CodecError {}

/// Errors related to decoding operations.
#[derive(Debug, Clone, PartialEq)]
pub struct DecodingError {
    pub message: String,
}

impl DecodingError {
    pub fn new(message: impl Into<String>) -> Self {
        DecodingError {
            message: message.into(),
        }
    }
}

impl fmt::Display for DecodingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.message)
    }
}

impl std::error::Error for DecodingError {}

impl From<DecodingError> for CodecError {
    fn from(e: DecodingError) -> Self {
        CodecError::Decoding(e)
    }
}
