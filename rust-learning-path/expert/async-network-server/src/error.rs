use thiserror::Error;

/// Custom error types for the async network server
/// 
/// This demonstrates error handling patterns in async Rust:
/// - Using thiserror for ergonomic error definitions
/// - Categorizing errors by domain (Network, HTTP, etc.)
/// - Providing context for debugging
/// - Enabling error propagation with the ? operator
#[derive(Error, Debug)]
pub enum ServerError {
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("HTTP parsing error: {0}")]
    HttpParsing(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Connection error: {0}")]
    Connection(String),
    
    #[error("Internal server error: {0}")]
    Internal(String),
    
    #[error("Timeout error: {0}")]
    Timeout(String),
}

/// Result type alias for server operations
/// 
/// This is a common pattern in Rust to reduce boilerplate
/// when working with a specific error type throughout a module
pub type ServerResult<T> = Result<T, ServerError>;

impl ServerError {
    /// Convert a generic I/O error to a ServerError
    pub fn from_io_error(err: std::io::Error, context: &str) -> Self {
        ServerError::Network(format!("{}: {}", context, err))
    }
    
    /// Create a timeout error with context
    pub fn timeout(context: &str) -> Self {
        ServerError::Timeout(context.to_string())
    }
    
    /// Create an internal error with context
    pub fn internal(msg: &str) -> Self {
        ServerError::Internal(msg.to_string())
    }
}

// TODO: Implement conversion from other error types
// This would typically include:
// - From<std::io::Error> for common I/O operations
// - From<serde_json::Error> for JSON parsing
// - From<tokio::time::error::Elapsed> for timeout operations