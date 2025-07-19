//! Advanced trait system for the task execution engine.
//!
//! This module defines the core traits that enable flexible and extensible
//! task execution, resource management, and system integration.

pub mod executor;
pub mod resource;

pub use executor::*;
pub use resource::*;

use crate::error::Result;
use std::any::Any;
use std::fmt::Debug;
use std::future::Future;
use std::pin::Pin;

/// Core trait for all tasks in the system
pub trait Task: Send + Sync + Debug {
    /// The type of result this task produces
    type Output: Send + Sync;

    /// Unique identifier for this task
    fn id(&self) -> u64;

    /// Human-readable name for this task type
    fn name(&self) -> &str;

    /// Execute the task and return the result
    fn execute(&self) -> Pin<Box<dyn Future<Output = Result<Self::Output>> + Send + '_>>;

    /// Estimate the computational cost of this task (for scheduling)
    fn estimated_cost(&self) -> u32 {
        1
    }

    /// Get task priority (higher values = higher priority)
    fn priority(&self) -> u8 {
        0
    }

    /// Convert to Any for downcasting
    fn as_any(&self) -> &dyn Any;
}

/// Trait for objects that can execute tasks
pub trait TaskExecutor: Send + Sync {
    /// Execute a task and return its result
    fn execute_task<T: Task>(
        &self,
        task: T,
    ) -> Pin<Box<dyn Future<Output = Result<T::Output>> + Send + '_>>;

    /// Get the number of currently running tasks
    fn active_tasks(&self) -> usize;

    /// Get the number of queued tasks
    fn queued_tasks(&self) -> usize;

    /// Shutdown the executor gracefully
    fn shutdown(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>>;
}

/// Trait for managing system resources
pub trait ResourceManager: Send + Sync {
    /// Allocate resources for a task
    fn allocate_resources(&self, task_id: u64, estimated_cost: u32) -> Result<ResourceHandle>;

    /// Release resources for a completed task
    fn release_resources(&self, handle: ResourceHandle) -> Result<()>;

    /// Get current resource utilization (0.0 to 1.0)
    fn utilization(&self) -> f64;

    /// Get available resource capacity
    fn available_capacity(&self) -> u32;
}

/// Handle for allocated resources
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ResourceHandle {
    pub id: u64,
    pub task_id: u64,
    pub allocated_at: std::time::Instant,
}

impl ResourceHandle {
    pub fn new(id: u64, task_id: u64) -> Self {
        Self {
            id,
            task_id,
            allocated_at: std::time::Instant::now(),
        }
    }

    pub fn age(&self) -> std::time::Duration {
        self.allocated_at.elapsed()
    }
}

/// Trait for objects that can be scheduled
pub trait Schedulable: Send + Sync {
    /// Get the scheduling priority
    fn priority(&self) -> u8;

    /// Get the estimated execution time
    fn estimated_duration(&self) -> std::time::Duration;

    /// Check if this item can be executed now
    fn can_execute(&self) -> bool {
        true
    }

    /// Get dependencies that must complete before this can execute
    fn dependencies(&self) -> Vec<u64> {
        Vec::new()
    }
}

/// Trait for monitoring and metrics collection
pub trait Monitorable {
    /// Get current metrics as key-value pairs
    fn metrics(&self) -> std::collections::HashMap<String, f64>;

    /// Get health status
    fn health(&self) -> HealthStatus;

    /// Get detailed status information
    fn status(&self) -> String;
}

/// Health status for monitorable components
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HealthStatus {
    Healthy,
    Warning { message: String },
    Critical { message: String },
    Unknown,
}

impl HealthStatus {
    pub fn is_healthy(&self) -> bool {
        matches!(self, HealthStatus::Healthy)
    }

    pub fn is_critical(&self) -> bool {
        matches!(self, HealthStatus::Critical { .. })
    }
}