//! Memory management module with custom allocators.
//!
//! This module demonstrates unsafe Rust by implementing custom memory
//! allocators optimized for the task execution engine.

pub mod allocator;
pub mod pool;

pub use allocator::*;
pub use pool::*;

// Re-export the resource manager from traits for convenience
pub use crate::traits::resource::BasicResourceManager;