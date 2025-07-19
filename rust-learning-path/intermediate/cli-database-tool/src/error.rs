use std::fmt;
use thiserror::Error;

/// Custom error types for the database tool
/// 
/// This demonstrates advanced error handling patterns in Rust:
/// - Custom error types with thiserror
/// - Error conversion and propagation
/// - Structured error information
#[derive(Error, Debug)]
pub enum DatabaseError {
    /// I/O errors when reading/writing files
    #[error("File operation failed: {0}")]
    Io(#[from] std::io::Error),

    /// JSON serialization/deserialization errors
    #[error("JSON processing failed: {0}")]
    Json(#[from] serde_json::Error),

    /// Database-specific errors
    #[error("Database error: {message}")]
    Database { message: String },

    /// Record validation errors
    #[error("Invalid record: {field} - {reason}")]
    InvalidRecord { field: String, reason: String },

    /// Record not found errors
    #[error("Record with ID '{id}' not found")]
    RecordNotFound { id: String },

    /// Record already exists errors
    #[error("Record with ID '{id}' already exists")]
    RecordExists { id: String },

    /// Database corruption or inconsistency errors
    #[error("Database corruption detected: {details}")]
    Corruption { details: String },
}

impl DatabaseError {
    /// Create a new database error with a custom message
    pub fn database(message: impl Into<String>) -> Self {
        DatabaseError::Database {
            message: message.into(),
        }
    }

    /// Create a new record validation error
    pub fn invalid_record(field: impl Into<String>, reason: impl Into<String>) -> Self {
        DatabaseError::InvalidRecord {
            field: field.into(),
            reason: reason.into(),
        }
    }

    /// Create a new record not found error
    pub fn record_not_found(id: impl Into<String>) -> Self {
        DatabaseError::RecordNotFound { id: id.into() }
    }

    /// Create a new record exists error
    pub fn record_exists(id: impl Into<String>) -> Self {
        DatabaseError::RecordExists { id: id.into() }
    }

    /// Create a new corruption error
    pub fn corruption(details: impl Into<String>) -> Self {
        DatabaseError::Corruption {
            details: details.into(),
        }
    }

    /// Check if this error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self {
            DatabaseError::Io(_) => false,
            DatabaseError::Json(_) => false,
            DatabaseError::Database { .. } => true,
            DatabaseError::InvalidRecord { .. } => true,
            DatabaseError::RecordNotFound { .. } => true,
            DatabaseError::RecordExists { .. } => true,
            DatabaseError::Corruption { .. } => false,
        }
    }

    /// Get error category for logging/metrics
    pub fn category(&self) -> &'static str {
        match self {
            DatabaseError::Io(_) => "io",
            DatabaseError::Json(_) => "serialization",
            DatabaseError::Database { .. } => "database",
            DatabaseError::InvalidRecord { .. } => "validation",
            DatabaseError::RecordNotFound { .. } => "not_found",
            DatabaseError::RecordExists { .. } => "conflict",
            DatabaseError::Corruption { .. } => "corruption",
        }
    }
}

/// Result type alias for database operations
pub type DatabaseResult<T> = Result<T, DatabaseError>;

// TODO: Implement the following as part of the learning exercise:
// 1. Error context chaining for better debugging
// 2. Error recovery strategies
// 3. Logging integration
// 4. Error metrics collection