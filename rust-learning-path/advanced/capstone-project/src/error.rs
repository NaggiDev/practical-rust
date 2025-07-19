//! Comprehensive error handling for the task execution engine.
//!
//! This module defines all error types used throughout the system and provides
//! conversion mechanisms between different error types.

use std::fmt;
use thiserror::Error;

/// Main result type used throughout the engine
pub type Result<T> = std::result::Result<T, EngineError>;

/// Comprehensive error type for the task execution engine
#[derive(Error, Debug)]
pub enum EngineError {
    /// Thread pool related errors
    #[error("Thread pool error: {message}")]
    ThreadPool { message: String },

    /// Memory allocation errors
    #[error("Memory allocation error: {message}")]
    Memory { message: String },

    /// FFI related errors
    #[error("FFI error: {message}")]
    Ffi { message: String },

    /// Task execution errors
    #[error("Task execution error: {message}")]
    TaskExecution { message: String },

    /// Configuration errors
    #[error("Configuration error: {message}")]
    Configuration { message: String },

    /// I/O errors
    #[error("I/O error: {source}")]
    Io {
        #[from]
        source: std::io::Error,
    },

    /// Serialization errors
    #[error("Serialization error: {source}")]
    Serialization {
        #[from]
        source: serde_json::Error,
    },

    /// Generic errors with context
    #[error("Error: {message}")]
    Generic { message: String },
}

impl EngineError {
    /// Create a new thread pool error
    pub fn thread_pool<S: Into<String>>(message: S) -> Self {
        Self::ThreadPool {
            message: message.into(),
        }
    }

    /// Create a new memory error
    pub fn memory<S: Into<String>>(message: S) -> Self {
        Self::Memory {
            message: message.into(),
        }
    }

    /// Create a new FFI error
    pub fn ffi<S: Into<String>>(message: S) -> Self {
        Self::Ffi {
            message: message.into(),
        }
    }

    /// Create a new task execution error
    pub fn task_execution<S: Into<String>>(message: S) -> Self {
        Self::TaskExecution {
            message: message.into(),
        }
    }

    /// Create a new configuration error
    pub fn configuration<S: Into<String>>(message: S) -> Self {
        Self::Configuration {
            message: message.into(),
        }
    }

    /// Create a generic error
    pub fn generic<S: Into<String>>(message: S) -> Self {
        Self::Generic {
            message: message.into(),
        }
    }
}

/// Error type for memory allocation operations
#[derive(Debug, Clone)]
pub struct AllocationError {
    pub size: usize,
    pub align: usize,
    pub message: String,
}

impl fmt::Display for AllocationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Failed to allocate {} bytes with alignment {}: {}",
            self.size, self.align, self.message
        )
    }
}

impl std::error::Error for AllocationError {}

impl From<AllocationError> for EngineError {
    fn from(err: AllocationError) -> Self {
        EngineError::memory(err.to_string())
    }
}

/// Error type for FFI operations
#[derive(Debug, Clone)]
pub struct FfiError {
    pub operation: String,
    pub code: i32,
    pub message: String,
}

impl fmt::Display for FfiError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "FFI operation '{}' failed with code {}: {}",
            self.operation, self.code, self.message
        )
    }
}

impl std::error::Error for FfiError {}

impl From<FfiError> for EngineError {
    fn from(err: FfiError) -> Self {
        EngineError::ffi(err.to_string())
    }
}

/// Error type for task execution
#[derive(Debug, Clone)]
pub struct TaskError {
    pub task_id: u64,
    pub task_type: String,
    pub message: String,
}

impl fmt::Display for TaskError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Task {} of type '{}' failed: {}",
            self.task_id, self.task_type, self.message
        )
    }
}

impl std::error::Error for TaskError {}

impl From<TaskError> for EngineError {
    fn from(err: TaskError) -> Self {
        EngineError::task_execution(err.to_string())
    }
}