//! Task trait definitions and implementations
//! 
//! This module demonstrates advanced trait usage including:
//! - Generic trait definitions with associated types
//! - Trait objects for dynamic dispatch
//! - Object safety considerations
//! - Trait bounds and where clauses

use std::any::Any;
use std::fmt::Debug;
use std::time::Duration;
use serde::{Serialize, Deserialize};

pub use super::{TaskId, TaskResult, TaskStatus, TaskMetadata};

/// Core trait that all tasks must implement
/// 
/// This trait demonstrates several advanced Rust concepts:
/// - Associated types for flexible return types
/// - Trait bounds ensuring thread safety
/// - Object safety for dynamic dispatch
pub trait Task: Send + Sync + Debug {
    /// The type of output this task produces
    type Output: Send + Sync + Debug + 'static;
    
    /// The type of error this task can produce
    type Error: std::error::Error + Send + Sync + 'static;
    
    /// Execute the task and return the result
    /// 
    /// This is the core method that defines what the task does.
    /// Implementations should be idempotent when possible.
    fn execute(&self) -> Result<Self::Output, Self::Error>;
    
    /// Get the name of this task for identification
    fn name(&self) -> &str;
    
    /// Get the timeout for this task (if any)
    /// 
    /// Tasks can specify their own timeout, or use the system default
    fn timeout(&self) -> Option<Duration> {
        None
    }
    
    /// Get the priority of this task
    /// 
    /// Higher numbers indicate higher priority
    fn priority(&self) -> i32 {
        0
    }
    
    /// Get tags associated with this task
    fn tags(&self) -> Vec<String> {
        Vec::new()
    }
    
    /// Check if this task can be retried on failure
    fn can_retry(&self) -> bool {
        false
    }
    
    /// Get the maximum number of retry attempts
    fn max_retries(&self) -> u32 {
        0
    }
    
    /// Validate that the task is ready to execute
    /// 
    /// This can be used to check preconditions before execution
    fn validate(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Ok(())
    }
    
    /// Called before task execution starts
    /// 
    /// This can be used for setup, logging, etc.
    fn before_execute(&self) {}
    
    /// Called after task execution completes (success or failure)
    /// 
    /// This can be used for cleanup, logging, etc.
    fn after_execute(&self) {}
}

/// Trait for tasks that can be serialized and deserialized
/// 
/// This enables task persistence and distribution across processes
pub trait SerializableTask: Task + Serialize + for<'de> Deserialize<'de> {
    /// Serialize the task to JSON
    fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(self)
    }
    
    /// Deserialize the task from JSON
    fn from_json(json: &str) -> Result<Self, serde_json::Error>
    where
        Self: Sized,
    {
        serde_json::from_str(json)
    }
}

/// Trait for tasks that can be cloned
/// 
/// This is useful for retry logic and task distribution
pub trait ClonableTask: Task + Clone {
    /// Create a copy of this task for retry or distribution
    fn clone_task(&self) -> Box<dyn ClonableTask> {
        Box::new(self.clone())
    }
}

/// Trait object type for dynamic task dispatch
/// 
/// This allows storing different task types in the same collection
/// while maintaining type safety through the trait system
pub trait DynTask: Send + Sync + Debug {
    /// Execute the task and return a serialized result
    fn execute_dyn(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>>;
    
    /// Get the task name
    fn name(&self) -> &str;
    
    /// Get the task timeout
    fn timeout(&self) -> Option<Duration>;
    
    /// Get the task priority
    fn priority(&self) -> i32;
    
    /// Get the task tags
    fn tags(&self) -> Vec<String>;
    
    /// Check if the task can be retried
    fn can_retry(&self) -> bool;
    
    /// Get maximum retries
    fn max_retries(&self) -> u32;
    
    /// Validate the task
    fn validate(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    
    /// Before execute hook
    fn before_execute(&self);
    
    /// After execute hook
    fn after_execute(&self);
    
    /// Get the task as Any for downcasting
    fn as_any(&self) -> &dyn Any;
}

/// Blanket implementation of DynTask for all Task implementors
impl<T> DynTask for T
where
    T: Task + 'static,
    T::Output: Serialize,
{
    fn execute_dyn(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        let result = self.execute()?;
        let serialized = serde_json::to_string(&result)
            .map_err(|e| Box::new(e) as Box<dyn std::error::Error + Send + Sync>)?;
        Ok(serialized)
    }
    
    fn name(&self) -> &str {
        Task::name(self)
    }
    
    fn timeout(&self) -> Option<Duration> {
        Task::timeout(self)
    }
    
    fn priority(&self) -> i32 {
        Task::priority(self)
    }
    
    fn tags(&self) -> Vec<String> {
        Task::tags(self)
    }
    
    fn can_retry(&self) -> bool {
        Task::can_retry(self)
    }
    
    fn max_retries(&self) -> u32 {
        Task::max_retries(self)
    }
    
    fn validate(&self) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        Task::validate(self)
    }
    
    fn before_execute(&self) {
        Task::before_execute(self)
    }
    
    fn after_execute(&self) {
        Task::after_execute(self)
    }
    
    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// A wrapper that makes any Task into a trait object
pub struct TaskBox {
    inner: Box<dyn DynTask>,
    metadata: TaskMetadata,
}

impl TaskBox {
    /// Create a new TaskBox from any Task
    pub fn new<T>(task: T) -> Self
    where
        T: Task + 'static,
        T::Output: Serialize,
    {
        let metadata = TaskMetadata::new(task.name())
            .with_priority(task.priority())
            .with_max_retries(task.max_retries());
        
        Self {
            inner: Box::new(task),
            metadata,
        }
    }
    
    /// Execute the wrapped task
    pub fn execute(&self) -> Result<String, Box<dyn std::error::Error + Send + Sync>> {
        self.inner.validate()?;
        self.inner.before_execute();
        
        let result = self.inner.execute_dyn();
        
        self.inner.after_execute();
        result
    }
    
    /// Get the task metadata
    pub fn metadata(&self) -> &TaskMetadata {
        &self.metadata
    }
    
    /// Get mutable access to the task metadata
    pub fn metadata_mut(&mut self) -> &mut TaskMetadata {
        &mut self.metadata
    }
    
    /// Downcast to a specific task type
    pub fn downcast_ref<T: Task + 'static>(&self) -> Option<&T> {
        self.inner.as_any().downcast_ref::<T>()
    }
}

impl Debug for TaskBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TaskBox")
            .field("name", &self.inner.name())
            .field("priority", &self.inner.priority())
            .field("metadata", &self.metadata)
            .finish()
    }
}

// Example task implementations for testing and demonstration

/// A simple mathematical task for demonstration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MathTask {
    pub operation: String,
    pub operands: Vec<f64>,
}

impl Task for MathTask {
    type Output = f64;
    type Error = String;
    
    fn execute(&self) -> Result<Self::Output, Self::Error> {
        match self.operation.as_str() {
            "add" => Ok(self.operands.iter().sum()),
            "multiply" => Ok(self.operands.iter().product()),
            "subtract" => {
                if self.operands.len() != 2 {
                    return Err("Subtract requires exactly 2 operands".to_string());
                }
                Ok(self.operands[0] - self.operands[1])
            }
            "divide" => {
                if self.operands.len() != 2 {
                    return Err("Divide requires exactly 2 operands".to_string());
                }
                if self.operands[1] == 0.0 {
                    return Err("Division by zero".to_string());
                }
                Ok(self.operands[0] / self.operands[1])
            }
            _ => Err(format!("Unknown operation: {}", self.operation)),
        }
    }
    
    fn name(&self) -> &str {
        &self.operation
    }
    
    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_secs(5))
    }
    
    fn can_retry(&self) -> bool {
        true
    }
    
    fn max_retries(&self) -> u32 {
        3
    }
}

impl SerializableTask for MathTask {}
impl ClonableTask for MathTask {}

/// A task that simulates work by sleeping
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SleepTask {
    pub duration_ms: u64,
    pub name: String,
}

impl Task for SleepTask {
    type Output = String;
    type Error = String;
    
    fn execute(&self) -> Result<Self::Output, Self::Error> {
        std::thread::sleep(Duration::from_millis(self.duration_ms));
        Ok(format!("Slept for {} ms", self.duration_ms))
    }
    
    fn name(&self) -> &str {
        &self.name
    }
    
    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_millis(self.duration_ms * 2))
    }
}

impl SerializableTask for SleepTask {}
impl ClonableTask for SleepTask {}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_math_task() {
        let task = MathTask {
            operation: "add".to_string(),
            operands: vec![1.0, 2.0, 3.0],
        };
        
        let result = task.execute().unwrap();
        assert_eq!(result, 6.0);
    }
    
    #[test]
    fn test_math_task_error() {
        let task = MathTask {
            operation: "divide".to_string(),
            operands: vec![10.0, 0.0],
        };
        
        let result = task.execute();
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("Division by zero"));
    }
    
    #[test]
    fn test_task_box() {
        let task = MathTask {
            operation: "multiply".to_string(),
            operands: vec![2.0, 3.0, 4.0],
        };
        
        let task_box = TaskBox::new(task);
        assert_eq!(task_box.inner.name(), "multiply");
        
        let result = task_box.execute().unwrap();
        let parsed: f64 = serde_json::from_str(&result).unwrap();
        assert_eq!(parsed, 24.0);
    }
    
    #[test]
    fn test_serializable_task() {
        let task = MathTask {
            operation: "add".to_string(),
            operands: vec![1.0, 2.0],
        };
        
        let json = task.to_json().unwrap();
        let deserialized = MathTask::from_json(&json).unwrap();
        
        assert_eq!(task.operation, deserialized.operation);
        assert_eq!(task.operands, deserialized.operands);
    }
    
    #[test]
    fn test_sleep_task() {
        let task = SleepTask {
            duration_ms: 10,
            name: "test_sleep".to_string(),
        };
        
        let start = std::time::Instant::now();
        let result = task.execute().unwrap();
        let elapsed = start.elapsed();
        
        assert!(elapsed >= Duration::from_millis(10));
        assert!(result.contains("10 ms"));
    }
}