//! Domain-Specific Language for task definition and configuration.
//!
//! This module provides macros that allow users to define tasks and configure
//! the execution engine using a more readable and type-safe syntax.

pub mod macros;
pub mod config;

pub use macros::*;
pub use config::*;

/// Re-export commonly used types for the DSL
pub use crate::ffi::{MathOperation, StringOperation, ArrayOperation};
pub use crate::traits::{Task, ExecutionOptions, ResourceRequirements};
pub use std::time::Duration;