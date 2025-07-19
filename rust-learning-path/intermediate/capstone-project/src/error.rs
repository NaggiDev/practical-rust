//! Error handling for the task queue system
//! 
//! This module demonstrates comprehensive error handling patterns including:
//! - Custom error types with detailed context
//! - Error conversion chains using the `From` trait
//! - Error source tracking for debugging
//! - Thread-safe error handling for concurrent contexts

use std::fmt;
use std::error::Error;
use std::sync::PoisonError;

/// Result type alias for convenience
pub type Result<T> = std::result::Result<T, TaskError>;

/// Comprehensive error type covering all possible failure modes
#[derive(Debug)]
pub enum TaskError {
    /// Task execution failed with a specific error message
    ExecutionFailed {
        task_name: String,
        error: String,
        context: Option<String>,
    },
    
    /// Task execution timed out
    Timeout {
        task_name: String,
        duration: std::time::Duration,
    },
    
    /// Required resource is unavailable (e.g., worker threads, storage)
    ResourceUnavailable(String),
    
    /// Configuration error
    ConfigurationError(String),
    
    /// Serialization/deserialization error
    SerializationError {
        context: String,
        source: serde_json::Error,
    },
    
    /// File I/O error
    IoError {
        operation: String,
        path: Option<std::path::PathBuf>,
        source: std::io::Error,
    },
    
    /// Thread synchronization error (mutex poisoning, etc.)
    LockError(String),
    
    /// Channel communication error
    ChannelError(String),
    
    /// Task not found
    TaskNotFound(crate::task::TaskId),
    
    /// Invalid task state transition
    InvalidState {
        task_id: crate::task::TaskId,
        current_state: String,
        attempted_transition: String,
    },
    
    /// Worker pool error
    WorkerPoolError(String),
    
    /// Storage backend error
    StorageError {
        operation: String,
        details: String,
    },
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TaskError::ExecutionFailed { task_name, error, context } => {
                write!(f, "Task '{}' execution failed: {}", task_name, error)?;
                if let Some(ctx) = context {
                    write!(f, " (Context: {})", ctx)?;
                }
                Ok(())
            }
            
            TaskError::Timeout { task_name, duration } => {
                write!(f, "Task '{}' timed out after {:?}", task_name, duration)
            }
            
            TaskError::ResourceUnavailable(resource) => {
                write!(f, "Resource unavailable: {}", resource)
            }
            
            TaskError::ConfigurationError(msg) => {
                write!(f, "Configuration error: {}", msg)
            }
            
            TaskError::SerializationError { context, source } => {
                write!(f, "Serialization error in {}: {}", context, source)
            }
            
            TaskError::IoError { operation, path, source } => {
                write!(f, "I/O error during {}", operation)?;
                if let Some(p) = path {
                    write!(f, " (path: {})", p.display())?;
                }
                write!(f, ": {}", source)
            }
            
            TaskError::LockError(msg) => {
                write!(f, "Lock error: {}", msg)
            }
            
            TaskError::ChannelError(msg) => {
                write!(f, "Channel communication error: {}", msg)
            }
            
            TaskError::TaskNotFound(id) => {
                write!(f, "Task not found: {}", id)
            }
            
            TaskError::InvalidState { task_id, current_state, attempted_transition } => {
                write!(f, "Invalid state transition for task {}: cannot {} from state '{}'", 
                       task_id, attempted_transition, current_state)
            }
            
            TaskError::WorkerPoolError(msg) => {
                write!(f, "Worker pool error: {}", msg)
            }
            
            TaskError::StorageError { operation, details } => {
                write!(f, "Storage error during {}: {}", operation, details)
            }
        }
    }
}

impl Error for TaskError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            TaskError::SerializationError { source, .. } => Some(source),
            TaskError::IoError { source, .. } => Some(source),
            _ => None,
        }
    }
}

// Error conversion implementations for automatic error propagation

impl From<std::io::Error> for TaskError {
    fn from(error: std::io::Error) -> Self {
        TaskError::IoError {
            operation: "unknown".to_string(),
            path: None,
            source: error,
        }
    }
}

impl From<serde_json::Error> for TaskError {
    fn from(error: serde_json::Error) -> Self {
        TaskError::SerializationError {
            context: "unknown".to_string(),
            source: error,
        }
    }
}

impl<T> From<PoisonError<T>> for TaskError {
    fn from(error: PoisonError<T>) -> Self {
        TaskError::LockError(format!("Mutex poisoned: {}", error))
    }
}

impl<T> From<std::sync::mpsc::SendError<T>> for TaskError {
    fn from(error: std::sync::mpsc::SendError<T>) -> Self {
        TaskError::ChannelError(format!("Failed to send message: {}", error))
    }
}

impl From<std::sync::mpsc::RecvError> for TaskError {
    fn from(error: std::sync::mpsc::RecvError) -> Self {
        TaskError::ChannelError(format!("Failed to receive message: {}", error))
    }
}

// Helper functions for creating errors with context

impl TaskError {
    /// Create an execution error with context
    pub fn execution_failed(task_name: &str, error: &str, context: Option<&str>) -> Self {
        TaskError::ExecutionFailed {
            task_name: task_name.to_string(),
            error: error.to_string(),
            context: context.map(|s| s.to_string()),
        }
    }
    
    /// Create a timeout error
    pub fn timeout(task_name: &str, duration: std::time::Duration) -> Self {
        TaskError::Timeout {
            task_name: task_name.to_string(),
            duration,
        }
    }
    
    /// Create an I/O error with context
    pub fn io_error(operation: &str, path: Option<&std::path::Path>, source: std::io::Error) -> Self {
        TaskError::IoError {
            operation: operation.to_string(),
            path: path.map(|p| p.to_path_buf()),
            source,
        }
    }
    
    /// Create a serialization error with context
    pub fn serialization_error(context: &str, source: serde_json::Error) -> Self {
        TaskError::SerializationError {
            context: context.to_string(),
            source,
        }
    }
    
    /// Create an invalid state error
    pub fn invalid_state(
        task_id: crate::task::TaskId,
        current_state: &str,
        attempted_transition: &str,
    ) -> Self {
        TaskError::InvalidState {
            task_id,
            current_state: current_state.to_string(),
            attempted_transition: attempted_transition.to_string(),
        }
    }
    
    /// Create a storage error
    pub fn storage_error(operation: &str, details: &str) -> Self {
        TaskError::StorageError {
            operation: operation.to_string(),
            details: details.to_string(),
        }
    }
}

/// Extension trait for adding context to Results
pub trait ResultExt<T> {
    /// Add context to an error
    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String;
    
    /// Add context to an error with a static string
    fn context(self, msg: &'static str) -> Result<T>;
}

impl<T, E> ResultExt<T> for std::result::Result<T, E>
where
    E: Into<TaskError>,
{
    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| {
            let base_error = e.into();
            match base_error {
                TaskError::ExecutionFailed { task_name, error, context: None } => {
                    TaskError::ExecutionFailed {
                        task_name,
                        error,
                        context: Some(f()),
                    }
                }
                other => other,
            }
        })
    }
    
    fn context(self, msg: &'static str) -> Result<T> {
        self.with_context(|| msg.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    
    #[test]
    fn test_error_display() {
        let error = TaskError::ExecutionFailed {
            task_name: "test_task".to_string(),
            error: "division by zero".to_string(),
            context: Some("in calculation step".to_string()),
        };
        
        let display = format!("{}", error);
        assert!(display.contains("test_task"));
        assert!(display.contains("division by zero"));
        assert!(display.contains("in calculation step"));
    }
    
    #[test]
    fn test_error_conversion() {
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let task_error: TaskError = io_error.into();
        
        match task_error {
            TaskError::IoError { .. } => (),
            _ => panic!("Expected IoError"),
        }
    }
    
    #[test]
    fn test_result_extension() {
        let result: std::result::Result<i32, std::io::Error> = 
            Err(std::io::Error::new(std::io::ErrorKind::NotFound, "test"));
        
        let with_context = result.context("during test operation");
        assert!(with_context.is_err());
    }
    
    #[test]
    fn test_error_source_chain() {
        let json_error = serde_json::from_str::<i32>("invalid json").unwrap_err();
        let task_error = TaskError::serialization_error("parsing config", json_error);
        
        assert!(task_error.source().is_some());
    }
}