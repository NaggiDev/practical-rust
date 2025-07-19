//! # Distributed Task Queue System
//! 
//! This library provides a comprehensive task queue system that demonstrates
//! all the concepts learned in the Intermediate Level of the Rust Learning Path.
//! 
//! ## Key Features
//! 
//! - **Generic Task System**: Support for any task type implementing the `Task` trait
//! - **Concurrent Execution**: Multi-threaded task processing with configurable worker pools
//! - **Persistent Storage**: Task and result persistence with JSON serialization
//! - **Real-time Monitoring**: Live status tracking and progress reporting
//! - **Robust Error Handling**: Comprehensive error types with detailed context
//! 
//! ## Quick Start
//! 
//! ```rust
//! use capstone_project::{TaskQueue, WorkerPool, Task};
//! 
//! // Define a custom task
//! #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
//! struct MathTask {
//!     operation: String,
//!     operands: Vec<i32>,
//! }
//! 
//! impl Task for MathTask {
//!     type Output = i32;
//!     type Error = String;
//!     
//!     fn execute(&self) -> Result<Self::Output, Self::Error> {
//!         match self.operation.as_str() {
//!             "sum" => Ok(self.operands.iter().sum()),
//!             "product" => Ok(self.operands.iter().product()),
//!             _ => Err(format!("Unknown operation: {}", self.operation)),
//!         }
//!     }
//!     
//!     fn name(&self) -> &str {
//!         &self.operation
//!     }
//! }
//! ```

pub mod task;
pub mod worker;
pub mod monitor;
pub mod storage;
pub mod error;
pub mod config;

// Re-export main types for convenience
pub use task::{Task, TaskId, TaskQueue, TaskResult, TaskStatus};
pub use worker::{WorkerPool, WorkerConfig};
pub use monitor::{Monitor, TaskMonitor, ProgressReport};
pub use storage::{Storage, JsonStorage, MemoryStorage};
pub use error::{TaskError, Result};
pub use config::Config;

/// The main task queue system that integrates all components
pub struct TaskQueueSystem {
    queue: std::sync::Arc<std::sync::Mutex<TaskQueue>>,
    worker_pool: WorkerPool,
    monitor: TaskMonitor,
    storage: Box<dyn Storage>,
}

impl TaskQueueSystem {
    /// Create a new task queue system with the given configuration
    pub fn new(config: Config) -> Result<Self> {
        let storage: Box<dyn Storage> = match config.storage_type.as_str() {
            "json" => Box::new(JsonStorage::new(&config.storage_path)?),
            "memory" => Box::new(MemoryStorage::new()),
            _ => return Err(TaskError::ConfigurationError(
                format!("Unknown storage type: {}", config.storage_type)
            )),
        };
        
        let queue = std::sync::Arc::new(std::sync::Mutex::new(TaskQueue::new()));
        let monitor = TaskMonitor::new();
        let worker_pool = WorkerPool::new(
            config.worker_count,
            std::sync::Arc::clone(&queue),
            monitor.clone(),
        )?;
        
        Ok(Self {
            queue,
            worker_pool,
            monitor,
            storage,
        })
    }
    
    /// Submit a task for execution
    pub fn submit<T: Task + 'static>(&self, task: T) -> Result<TaskId> {
        let task_id = {
            let mut queue = self.queue.lock()
                .map_err(|_| TaskError::LockError("Failed to acquire queue lock".to_string()))?;
            queue.submit(Box::new(task))?
        };
        
        // Persist the task
        self.storage.save_task(task_id, &*self.queue.lock().unwrap())?;
        
        // Notify monitor
        self.monitor.task_submitted(task_id);
        
        Ok(task_id)
    }
    
    /// Get the result of a completed task
    pub fn get_result(&self, task_id: TaskId) -> Result<Option<TaskResult>> {
        let queue = self.queue.lock()
            .map_err(|_| TaskError::LockError("Failed to acquire queue lock".to_string()))?;
        Ok(queue.get_result(task_id))
    }
    
    /// Wait for a task to complete and return its result
    pub fn wait_for_result(&self, task_id: TaskId) -> Result<TaskResult> {
        self.monitor.wait_for_completion(task_id)
    }
    
    /// Get current system status
    pub fn status(&self) -> ProgressReport {
        self.monitor.get_progress_report()
    }
    
    /// Start the worker pool
    pub fn start(&self) -> Result<()> {
        self.worker_pool.start()
    }
    
    /// Stop the worker pool gracefully
    pub fn stop(&self) -> Result<()> {
        self.worker_pool.stop()
    }
}

impl Drop for TaskQueueSystem {
    fn drop(&mut self) {
        let _ = self.worker_pool.stop();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
    struct TestTask {
        value: i32,
    }
    
    impl Task for TestTask {
        type Output = i32;
        type Error = String;
        
        fn execute(&self) -> std::result::Result<Self::Output, Self::Error> {
            Ok(self.value * 2)
        }
        
        fn name(&self) -> &str {
            "test_task"
        }
    }
    
    #[test]
    fn test_system_creation() {
        let config = Config::default();
        let system = TaskQueueSystem::new(config);
        assert!(system.is_ok());
    }
    
    #[test]
    fn test_task_submission() {
        let config = Config::default();
        let system = TaskQueueSystem::new(config).unwrap();
        
        let task = TestTask { value: 5 };
        let task_id = system.submit(task);
        assert!(task_id.is_ok());
    }
}