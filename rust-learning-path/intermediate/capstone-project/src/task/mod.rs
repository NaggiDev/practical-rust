//! Task system module
//! 
//! This module contains the core task system components including:
//! - Task trait definition and implementations
//! - Task queue for managing pending and completed tasks
//! - Task registry for persistent storage
//! - Task result handling and status tracking

pub mod queue;
pub mod registry;
pub mod traits;

pub use queue::TaskQueue;
pub use registry::TaskRegistry;
pub use traits::{Task, TaskResult, TaskStatus, TaskId};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use uuid::Uuid;
use serde::{Serialize, Deserialize};

/// Unique identifier for tasks
pub type TaskId = Uuid;

/// Status of a task in the system
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TaskStatus {
    /// Task is waiting to be executed
    Pending,
    /// Task is currently being executed
    Running,
    /// Task completed successfully
    Completed,
    /// Task failed with an error
    Failed,
    /// Task was cancelled before completion
    Cancelled,
    /// Task timed out during execution
    TimedOut,
}

/// Result of task execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResult {
    /// Unique identifier for the task
    pub task_id: TaskId,
    /// Current status of the task
    pub status: TaskStatus,
    /// Serialized output data (if successful)
    pub output: Option<String>,
    /// Error message (if failed)
    pub error: Option<String>,
    /// When the task was submitted
    pub submitted_at: chrono::DateTime<chrono::Utc>,
    /// When the task started executing
    pub started_at: Option<chrono::DateTime<chrono::Utc>>,
    /// When the task completed (successfully or with error)
    pub completed_at: Option<chrono::DateTime<chrono::Utc>>,
    /// How long the task took to execute
    pub duration: Option<Duration>,
}

impl TaskResult {
    /// Create a new task result for a pending task
    pub fn new(task_id: TaskId) -> Self {
        Self {
            task_id,
            status: TaskStatus::Pending,
            output: None,
            error: None,
            submitted_at: chrono::Utc::now(),
            started_at: None,
            completed_at: None,
            duration: None,
        }
    }
    
    /// Mark the task as started
    pub fn mark_started(&mut self) {
        self.status = TaskStatus::Running;
        self.started_at = Some(chrono::Utc::now());
    }
    
    /// Mark the task as completed successfully
    pub fn mark_completed(&mut self, output: String) {
        self.status = TaskStatus::Completed;
        self.output = Some(output);
        self.completed_at = Some(chrono::Utc::now());
        self.calculate_duration();
    }
    
    /// Mark the task as failed
    pub fn mark_failed(&mut self, error: String) {
        self.status = TaskStatus::Failed;
        self.error = Some(error);
        self.completed_at = Some(chrono::Utc::now());
        self.calculate_duration();
    }
    
    /// Mark the task as timed out
    pub fn mark_timed_out(&mut self) {
        self.status = TaskStatus::TimedOut;
        self.error = Some("Task execution timed out".to_string());
        self.completed_at = Some(chrono::Utc::now());
        self.calculate_duration();
    }
    
    /// Mark the task as cancelled
    pub fn mark_cancelled(&mut self) {
        self.status = TaskStatus::Cancelled;
        self.error = Some("Task was cancelled".to_string());
        self.completed_at = Some(chrono::Utc::now());
        self.calculate_duration();
    }
    
    /// Calculate the duration if both start and end times are available
    fn calculate_duration(&mut self) {
        if let (Some(started), Some(completed)) = (self.started_at, self.completed_at) {
            self.duration = Some(Duration::from_millis(
                (completed - started).num_milliseconds() as u64
            ));
        }
    }
    
    /// Check if the task is in a terminal state
    pub fn is_terminal(&self) -> bool {
        matches!(
            self.status,
            TaskStatus::Completed | TaskStatus::Failed | TaskStatus::Cancelled | TaskStatus::TimedOut
        )
    }
    
    /// Check if the task completed successfully
    pub fn is_successful(&self) -> bool {
        self.status == TaskStatus::Completed
    }
}

/// Metadata associated with a task
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskMetadata {
    /// Task name for identification
    pub name: String,
    /// Task priority (higher numbers = higher priority)
    pub priority: i32,
    /// Maximum execution time allowed
    pub timeout: Option<Duration>,
    /// Number of retry attempts allowed
    pub max_retries: u32,
    /// Current retry attempt
    pub retry_count: u32,
    /// Tags for categorization
    pub tags: Vec<String>,
    /// Custom metadata
    pub custom_data: HashMap<String, String>,
}

impl Default for TaskMetadata {
    fn default() -> Self {
        Self {
            name: "unnamed_task".to_string(),
            priority: 0,
            timeout: None,
            max_retries: 0,
            retry_count: 0,
            tags: Vec::new(),
            custom_data: HashMap::new(),
        }
    }
}

impl TaskMetadata {
    /// Create new metadata with a name
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            ..Default::default()
        }
    }
    
    /// Set the priority
    pub fn with_priority(mut self, priority: i32) -> Self {
        self.priority = priority;
        self
    }
    
    /// Set the timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
    
    /// Set the maximum retries
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.max_retries = max_retries;
        self
    }
    
    /// Add a tag
    pub fn with_tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }
    
    /// Add custom data
    pub fn with_custom_data(mut self, key: &str, value: &str) -> Self {
        self.custom_data.insert(key.to_string(), value.to_string());
        self
    }
    
    /// Check if a retry is allowed
    pub fn can_retry(&self) -> bool {
        self.retry_count < self.max_retries
    }
    
    /// Increment the retry count
    pub fn increment_retry(&mut self) {
        self.retry_count += 1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_task_result_lifecycle() {
        let task_id = TaskId::new_v4();
        let mut result = TaskResult::new(task_id);
        
        assert_eq!(result.status, TaskStatus::Pending);
        assert!(result.started_at.is_none());
        
        result.mark_started();
        assert_eq!(result.status, TaskStatus::Running);
        assert!(result.started_at.is_some());
        
        result.mark_completed("success".to_string());
        assert_eq!(result.status, TaskStatus::Completed);
        assert!(result.is_successful());
        assert!(result.is_terminal());
        assert!(result.duration.is_some());
    }
    
    #[test]
    fn test_task_metadata() {
        let metadata = TaskMetadata::new("test_task")
            .with_priority(5)
            .with_timeout(Duration::from_secs(30))
            .with_max_retries(3)
            .with_tag("important")
            .with_custom_data("user_id", "123");
        
        assert_eq!(metadata.name, "test_task");
        assert_eq!(metadata.priority, 5);
        assert_eq!(metadata.timeout, Some(Duration::from_secs(30)));
        assert_eq!(metadata.max_retries, 3);
        assert!(metadata.tags.contains(&"important".to_string()));
        assert_eq!(metadata.custom_data.get("user_id"), Some(&"123".to_string()));
    }
    
    #[test]
    fn test_retry_logic() {
        let mut metadata = TaskMetadata::new("test").with_max_retries(2);
        
        assert!(metadata.can_retry());
        metadata.increment_retry();
        assert!(metadata.can_retry());
        metadata.increment_retry();
        assert!(!metadata.can_retry());
    }
}