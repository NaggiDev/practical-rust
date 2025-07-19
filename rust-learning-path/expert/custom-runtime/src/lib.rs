//! A simple async runtime implementation for learning purposes.
//! 
//! This crate demonstrates how async runtimes work by implementing
//! a basic executor, task system, and waker mechanism from scratch.

pub mod executor;
pub mod task;
pub mod waker;
pub mod timer;

pub use executor::Executor;
pub use task::Task;
pub use timer::{Timer, YieldTimer};

/// Re-export commonly used types for convenience
pub mod prelude {
    pub use crate::{Executor, Task, Timer, YieldTimer};
    pub use std::future::Future;
    pub use std::pin::Pin;
    pub use std::task::{Context, Poll};
}