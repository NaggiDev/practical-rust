//! Custom error types for the database tool
//! 
//! This module demonstrates how to create comprehensive error types
//! that provide clear information about what went wrong and why.

use std::fmt;
use thiserror::Error;

/// Custom result type for database operations
pub type Result<T> = std::result::Result<T, DatabaseError>;

/// Comprehensive error type for all database operations
/// 
/// This enum covers all possible error conditions that can occur
/// in our database tool, providing specific error variants for
/// different failure modes.
#[derive(Error, Debug)]
pub enum DatabaseError {
    /// File I/O related errors
    #[error("File operation failed: {message}")]
    FileError { message: String },

    /// JSON serialization/deserialization errors
    #[error("JSON processing failed: {0}")]
    JsonError(#[from] serde_json::Error),

    /// Data validation errors
    #[error("Validation failed: {field} - {message}")]
    ValidationError { field: String, message: String },

    /// Record not found errors
    #[error("Record not found: {id}")]
    RecordNotFound { id: String },

    /// Database corruption or inconsistency errors
    #[error("Database integrity error: {message}")]
    IntegrityError { message: String },

    /// Generic database operation errors
    #[error("Database operation failed: {message}")]
    OperationError { message: String },
}

/// Convert std::io::Error to DatabaseError
/// 
/// This implementation allows us to use the `?` operator with
/// file operations, automatically converting IO errors to our
/// custom error type.
impl From<std::io::Error> for DatabaseError {
    fn from(error: std::io::Error) -> Self {
        DatabaseError::FileError {
            message: error.to_string(),
        }
    }
}

impl DatabaseError {
    /// Create a new validation error
    pub fn validation(field: &str, message: &str) -> Self {
        DatabaseError::ValidationError {
            field: field.to_string(),
            message: message.to_string(),
        }
    }

    /// Create a new record not found error
    pub fn not_found(id: &str) -> Self {
        DatabaseError::RecordNotFound {
            id: id.to_string(),
        }
    }

    /// Create a new integrity error
    pub fn integrity(message: &str) -> Self {
        DatabaseError::IntegrityError {
            message: message.to_string(),
        }
    }

    /// Create a new operation error
    pub fn operation(message: &str) -> Self {
        DatabaseError::OperationError {
            message: message.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_creation() {
        let validation_err = DatabaseError::validation("email", "Invalid format");
        assert!(matches!(validation_err, DatabaseError::ValidationError { .. }));

        let not_found_err = DatabaseError::not_found("123");
        assert!(matches!(not_found_err, DatabaseError::RecordNotFound { .. }));
    }

    #[test]
    fn test_error_display() {
        let err = DatabaseError::validation("email", "Invalid format");
        let error_string = format!("{}", err);
        assert!(error_string.contains("Validation failed"));
        assert!(error_string.contains("email"));
        assert!(error_string.contains("Invalid format"));
    }

    #[test]
    fn test_io_error_conversion() {
        let io_err = std::io::Error::new(std::io::ErrorKind::NotFound, "File not found");
        let db_err: DatabaseError = io_err.into();
        assert!(matches!(db_err, DatabaseError::FileError { .. }));
    }
}