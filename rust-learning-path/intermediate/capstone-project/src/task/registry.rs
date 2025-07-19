//! Task registry for managing task metadata and persistence
//! 
//! This module demonstrates advanced ownership patterns and data management

use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::path::Path;

use super::{TaskId, TaskResult, TaskMetadata, TaskBox};
use crate::error::{TaskError, Result};

/// Registry for managing task metadata and persistence
/// 
/// This demonstrates several advanced concepts:
/// - Read-write locks for concurrent access
/// - Shared ownership with Arc
/// - Error handling with context
/// - File I/O with proper error propagation
pub struct TaskRegistry {
    /// Task metadata storage
    metadata: Arc<RwLock<HashMap<TaskId, TaskMetadata>>>,
    
    /// Task results storage
    results: Arc<RwLock<HashMap<TaskId, TaskResult>>>,
    
    /// Storage backend
    storage_path: Option<std::path::PathBuf>,
}

impl TaskRegistry {
    /// Create a new task registry
    pub fn new() -> Self {
        Self {
            metadata: Arc::new(RwLock::new(HashMap::new())),
            results: Arc::new(RwLock::new(HashMap::new())),
            storage_path: None,
        }
    }
    
    /// Create a new task registry with file-based persistence
    pub fn with_storage<P: AsRef<Path>>(path: P) -> Result<Self> {
        let storage_path = path.as_ref().to_path_buf();
        
        // Ensure the directory exists
        if let Some(parent) = storage_path.parent() {
            std::fs::create_dir_all(parent)
                .map_err(|e| TaskError::io_error("create storage directory", Some(parent), e))?;
        }
        
        let mut registry = Self {
            metadata: Arc::new(RwLock::new(HashMap::new())),
            results: Arc::new(RwLock::new(HashMap::new())),
            storage_path: Some(storage_path),
        };
        
        // Load existing data
        registry.load_from_storage()?;
        
        Ok(registry)
    }
    
    /// Register a new task
    pub fn register_task(&self, task_id: TaskId, task: &TaskBox) -> Result<()> {
        let metadata = task.metadata().clone();
        
        // Store metadata
        {
            let mut metadata_map = self.metadata.write()
                .map_err(|_| TaskError::LockError("Failed to acquire metadata write lock".to_string()))?;
            metadata_map.insert(task_id, metadata);
        }
        
        // Create initial result
        let result = TaskResult::new(task_id);
        {
            let mut results_map = self.results.write()
                .map_err(|_| TaskError::LockError("Failed to acquire results write lock".to_string()))?;
            results_map.insert(task_id, result);
        }
        
        // Persist to storage if configured
        if self.storage_path.is_some() {
            self.save_to_storage()?;
        }
        
        Ok(())
    }
    
    /// Get task metadata
    pub fn get_metadata(&self, task_id: TaskId) -> Result<Option<TaskMetadata>> {
        let metadata_map = self.metadata.read()
            .map_err(|_| TaskError::LockError("Failed to acquire metadata read lock".to_string()))?;
        Ok(metadata_map.get(&task_id).cloned())
    }
    
    /// Update task result
    pub fn update_result(&self, task_id: TaskId, result: TaskResult) -> Result<()> {
        {
            let mut results_map = self.results.write()
                .map_err(|_| TaskError::LockError("Failed to acquire results write lock".to_string()))?;
            results_map.insert(task_id, result);
        }
        
        // Persist to storage if configured
        if self.storage_path.is_some() {
            self.save_to_storage()?;
        }
        
        Ok(())
    }
    
    /// Get task result
    pub fn get_result(&self, task_id: TaskId) -> Result<Option<TaskResult>> {
        let results_map = self.results.read()
            .map_err(|_| TaskError::LockError("Failed to acquire results read lock".to_string()))?;
        Ok(results_map.get(&task_id).cloned())
    }
    
    /// Get all task IDs
    pub fn get_all_task_ids(&self) -> Result<Vec<TaskId>> {
        let metadata_map = self.metadata.read()
            .map_err(|_| TaskError::LockError("Failed to acquire metadata read lock".to_string()))?;
        Ok(metadata_map.keys().cloned().collect())
    }
    
    /// Remove a task from the registry
    pub fn remove_task(&self, task_id: TaskId) -> Result<bool> {
        let metadata_removed = {
            let mut metadata_map = self.metadata.write()
                .map_err(|_| TaskError::LockError("Failed to acquire metadata write lock".to_string()))?;
            metadata_map.remove(&task_id).is_some()
        };
        
        let result_removed = {
            let mut results_map = self.results.write()
                .map_err(|_| TaskError::LockError("Failed to acquire results write lock".to_string()))?;
            results_map.remove(&task_id).is_some()
        };
        
        let removed = metadata_removed || result_removed;
        
        // Persist to storage if configured and something was removed
        if removed && self.storage_path.is_some() {
            self.save_to_storage()?;
        }
        
        Ok(removed)
    }
    
    /// Get registry statistics
    pub fn stats(&self) -> Result<RegistryStats> {
        let metadata_count = {
            let metadata_map = self.metadata.read()
                .map_err(|_| TaskError::LockError("Failed to acquire metadata read lock".to_string()))?;
            metadata_map.len()
        };
        
        let results_map = self.results.read()
            .map_err(|_| TaskError::LockError("Failed to acquire results read lock".to_string()))?;
        
        let mut stats = RegistryStats {
            total_tasks: metadata_count,
            pending_tasks: 0,
            running_tasks: 0,
            completed_tasks: 0,
            failed_tasks: 0,
        };
        
        for result in results_map.values() {
            match result.status {
                crate::task::TaskStatus::Pending => stats.pending_tasks += 1,
                crate::task::TaskStatus::Running => stats.running_tasks += 1,
                crate::task::TaskStatus::Completed => stats.completed_tasks += 1,
                crate::task::TaskStatus::Failed => stats.failed_tasks += 1,
                crate::task::TaskStatus::Cancelled => stats.failed_tasks += 1,
                crate::task::TaskStatus::TimedOut => stats.failed_tasks += 1,
            }
        }
        
        Ok(stats)
    }
    
    /// Save registry data to storage
    fn save_to_storage(&self) -> Result<()> {
        let storage_path = match &self.storage_path {
            Some(path) => path,
            None => return Ok(()), // No storage configured
        };
        
        let data = {
            let metadata_map = self.metadata.read()
                .map_err(|_| TaskError::LockError("Failed to acquire metadata read lock".to_string()))?;
            let results_map = self.results.read()
                .map_err(|_| TaskError::LockError("Failed to acquire results read lock".to_string()))?;
            
            RegistryData {
                metadata: metadata_map.clone(),
                results: results_map.clone(),
            }
        };
        
        let json = serde_json::to_string_pretty(&data)
            .map_err(|e| TaskError::serialization_error("registry data", e))?;
        
        std::fs::write(storage_path, json)
            .map_err(|e| TaskError::io_error("save registry", Some(storage_path), e))?;
        
        Ok(())
    }
    
    /// Load registry data from storage
    fn load_from_storage(&mut self) -> Result<()> {
        let storage_path = match &self.storage_path {
            Some(path) => path,
            None => return Ok(()), // No storage configured
        };
        
        if !storage_path.exists() {
            return Ok(()); // No existing data to load
        }
        
        let json = std::fs::read_to_string(storage_path)
            .map_err(|e| TaskError::io_error("load registry", Some(storage_path), e))?;
        
        let data: RegistryData = serde_json::from_str(&json)
            .map_err(|e| TaskError::serialization_error("registry data", e))?;
        
        {
            let mut metadata_map = self.metadata.write()
                .map_err(|_| TaskError::LockError("Failed to acquire metadata write lock".to_string()))?;
            *metadata_map = data.metadata;
        }
        
        {
            let mut results_map = self.results.write()
                .map_err(|_| TaskError::LockError("Failed to acquire results write lock".to_string()))?;
            *results_map = data.results;
        }
        
        Ok(())
    }
}

impl Default for TaskRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Registry statistics
#[derive(Debug, Clone)]
pub struct RegistryStats {
    pub total_tasks: usize,
    pub pending_tasks: usize,
    pub running_tasks: usize,
    pub completed_tasks: usize,
    pub failed_tasks: usize,
}

/// Serializable registry data
#[derive(Debug, serde::Serialize, serde::Deserialize)]
struct RegistryData {
    metadata: HashMap<TaskId, TaskMetadata>,
    results: HashMap<TaskId, TaskResult>,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::traits::MathTask;
    use tempfile::tempdir;
    
    #[test]
    fn test_registry_creation() {
        let registry = TaskRegistry::new();
        let stats = registry.stats().unwrap();
        assert_eq!(stats.total_tasks, 0);
    }
    
    #[test]
    fn test_task_registration() {
        let registry = TaskRegistry::new();
        let task_id = TaskId::new_v4();
        let task = TaskBox::new(MathTask {
            operation: "add".to_string(),
            operands: vec![1.0, 2.0],
        });
        
        registry.register_task(task_id, &task).unwrap();
        
        let metadata = registry.get_metadata(task_id).unwrap();
        assert!(metadata.is_some());
        
        let result = registry.get_result(task_id).unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap().status, crate::task::TaskStatus::Pending);
    }
    
    #[test]
    fn test_result_update() {
        let registry = TaskRegistry::new();
        let task_id = TaskId::new_v4();
        let task = TaskBox::new(MathTask {
            operation: "multiply".to_string(),
            operands: vec![3.0, 4.0],
        });
        
        registry.register_task(task_id, &task).unwrap();
        
        let mut result = TaskResult::new(task_id);
        result.mark_completed("12.0".to_string());
        
        registry.update_result(task_id, result).unwrap();
        
        let updated_result = registry.get_result(task_id).unwrap().unwrap();
        assert_eq!(updated_result.status, crate::task::TaskStatus::Completed);
        assert_eq!(updated_result.output, Some("12.0".to_string()));
    }
    
    #[test]
    fn test_task_removal() {
        let registry = TaskRegistry::new();
        let task_id = TaskId::new_v4();
        let task = TaskBox::new(MathTask {
            operation: "subtract".to_string(),
            operands: vec![10.0, 5.0],
        });
        
        registry.register_task(task_id, &task).unwrap();
        assert!(registry.get_metadata(task_id).unwrap().is_some());
        
        let removed = registry.remove_task(task_id).unwrap();
        assert!(removed);
        assert!(registry.get_metadata(task_id).unwrap().is_none());
    }
    
    #[test]
    fn test_persistence() {
        let dir = tempdir().unwrap();
        let storage_path = dir.path().join("registry.json");
        
        let task_id = TaskId::new_v4();
        let task = TaskBox::new(MathTask {
            operation: "divide".to_string(),
            operands: vec![20.0, 4.0],
        });
        
        // Create registry and register task
        {
            let registry = TaskRegistry::with_storage(&storage_path).unwrap();
            registry.register_task(task_id, &task).unwrap();
        }
        
        // Create new registry and verify data was loaded
        {
            let registry = TaskRegistry::with_storage(&storage_path).unwrap();
            let metadata = registry.get_metadata(task_id).unwrap();
            assert!(metadata.is_some());
            assert_eq!(metadata.unwrap().name, "divide");
        }
    }
    
    #[test]
    fn test_concurrent_access() {
        use std::thread;
        use std::sync::Arc;
        
        let registry = Arc::new(TaskRegistry::new());
        let mut handles = vec![];
        
        // Spawn multiple threads registering tasks
        for i in 0..10 {
            let registry_clone = Arc::clone(&registry);
            let handle = thread::spawn(move || {
                let task_id = TaskId::new_v4();
                let task = TaskBox::new(MathTask {
                    operation: "add".to_string(),
                    operands: vec![i as f64, (i + 1) as f64],
                });
                registry_clone.register_task(task_id, &task).unwrap();
                task_id
            });
            handles.push(handle);
        }
        
        // Wait for all threads and collect task IDs
        let mut task_ids = vec![];
        for handle in handles {
            task_ids.push(handle.join().unwrap());
        }
        
        // Verify all tasks were registered
        let stats = registry.stats().unwrap();
        assert_eq!(stats.total_tasks, 10);
        
        // Verify we can retrieve all tasks
        for task_id in task_ids {
            assert!(registry.get_metadata(task_id).unwrap().is_some());
        }
    }
}