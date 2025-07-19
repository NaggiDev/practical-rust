//! # Capstone Project: High-Performance Task Execution Engine
//!
//! This library demonstrates advanced Rust concepts by implementing a high-performance
//! task execution engine that combines:
//! - Concurrency with custom thread pools and work stealing
//! - Unsafe Rust with custom memory allocators
//! - FFI integration with C libraries
//! - Macros for domain-specific languages
//! - Advanced traits and error handling
//!
//! ## Example Usage
//!
//! ```rust
//! use capstone_project::*;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Create a task engine with 4 worker threads
//! let engine = TaskEngine::builder()
//!     .workers(4)
//!     .queue_size(1000)
//!     .build()?;
//!
//! // Submit a mathematical task
//! let result = engine.submit_math_task(MathOperation::Factorial, vec![5]).await?;
//! println!("5! = {}", result);
//!
//! // Submit a string processing task
//! let result = engine.submit_string_task(StringOperation::Reverse, "hello".to_string()).await?;
//! println!("Reversed: {}", result);
//! # Ok(())
//! # }
//! ```

pub mod engine;
pub mod memory;
pub mod ffi;
pub mod dsl;
pub mod traits;
pub mod error;

// Re-export main types for convenience
pub use engine::{TaskEngine, TaskEngineBuilder};
pub use error::{EngineError, Result};
pub use traits::{Task, TaskExecutor, ResourceManager};
pub use ffi::{MathOperation, StringOperation, ArrayOperation};

// Re-export DSL macros
pub use dsl::*;