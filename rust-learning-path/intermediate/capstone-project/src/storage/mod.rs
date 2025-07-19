//! Storage backends for task persistence
//! 
//! This module demonstrates trait-based storage abstraction

pub mod json_store;
pub mod memory_store;

pub use json_store::JsonStorage;
pub use memory_store::MemoryStorage;

use crate::task::{TaskId, TaskQueue};
use crate::error::Result;

/// Trait for storage backends
pub trait Storage: Send + Sync {
    fn save_task(&self, task_id: TaskId, queue: &TaskQueue) -> Result<()>;
    fn load_tasks(&self) -> Result<Vec<TaskId>>;
    fn delete_task(&self, task_id: TaskId) -> Result<()>;
}

pub struct JsonStorage {
    path: std::path::PathBuf,
}

impl JsonStorage {
    pub fn new(path: &std::path::Path) -> Result<Self> {
        Ok(Self {
            path: path.to_path_buf(),
        })
    }
}

impl Storage for JsonStorage {
    fn save_task(&self, _task_id: TaskId, _queue: &TaskQueue) -> Result<()> {
        // Placeholder implementation
        Ok(())
    }
    
    fn load_tasks(&self) -> Result<Vec<TaskId>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
    
    fn delete_task(&self, _task_id: TaskId) -> Result<()> {
        // Placeholder implementation
        Ok(())
    }
}

pub struct MemoryStorage {
    // Placeholder implementation
}

impl MemoryStorage {
    pub fn new() -> Self {
        Self {}
    }
}

impl Storage for MemoryStorage {
    fn save_task(&self, _task_id: TaskId, _queue: &TaskQueue) -> Result<()> {
        // Placeholder implementation
        Ok(())
    }
    
    fn load_tasks(&self) -> Result<Vec<TaskId>> {
        // Placeholder implementation
        Ok(Vec::new())
    }
    
    fn delete_task(&self, _task_id: TaskId) -> Result<()> {
        // Placeholder implementation
        Ok(())
    }
}