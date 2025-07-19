//! Task queue implementation
//! 
//! This module demonstrates advanced ownership and concurrency patterns:
//! - Thread-safe collections with Arc<Mutex<T>>
//! - Priority queues and custom ordering
//! - Shared ownership across multiple threads
//! - Interior mutability patterns

use std::collections::{BinaryHeap, HashMap, VecDeque};
use std::sync::{Arc, Mutex, Condvar};
use std::cmp::Ordering;
use std::time::{Duration, Instant};

use super::{TaskId, TaskResult, TaskStatus, TaskBox};
use crate::error::{TaskError, Result};

/// A thread-safe task queue that manages pending and completed tasks
/// 
/// This implementation demonstrates several key concepts:
/// - Shared ownership with Arc for thread safety
/// - Interior mutability with Mutex
/// - Condition variables for thread coordination
/// - Priority-based task ordering
#[derive(Debug)]
pub struct TaskQueue {
    /// Pending tasks ordered by priority
    pending: Arc<Mutex<BinaryHeap<PriorityTask>>>,
    
    /// Currently running tasks
    running: Arc<Mutex<HashMap<TaskId, TaskBox>>>,
    
    /// Completed task results
    completed: Arc<Mutex<HashMap<TaskId, TaskResult>>>,
    
    /// Condition variable for notifying workers of new tasks
    task_available: Arc<Condvar>,
    
    /// Condition variable for notifying waiters of completed tasks
    task_completed: Arc<Condvar>,
    
    /// Queue statistics
    stats: Arc<Mutex<QueueStats>>,
}

/// Wrapper for tasks with priority ordering
#[derive(Debug)]
struct PriorityTask {
    task: TaskBox,
    task_id: TaskId,
    submitted_at: Instant,
}

impl PartialEq for PriorityTask {
    fn eq(&self, other: &Self) -> bool {
        self.task.metadata().priority == other.task.metadata().priority
    }
}

impl Eq for PriorityTask {}

impl PartialOrd for PriorityTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PriorityTask {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher priority tasks come first (reverse ordering for max-heap)
        other.task.metadata().priority.cmp(&self.task.metadata().priority)
            .then_with(|| self.submitted_at.cmp(&other.submitted_at)) // FIFO for same priority
    }
}

/// Queue statistics for monitoring
#[derive(Debug, Clone, Default)]
pub struct QueueStats {
    pub total_submitted: u64,
    pub total_completed: u64,
    pub total_failed: u64,
    pub total_cancelled: u64,
    pub current_pending: usize,
    pub current_running: usize,
}

impl TaskQueue {
    /// Create a new empty task queue
    pub fn new() -> Self {
        Self {
            pending: Arc::new(Mutex::new(BinaryHeap::new())),
            running: Arc::new(Mutex::new(HashMap::new())),
            completed: Arc::new(Mutex::new(HashMap::new())),
            task_available: Arc::new(Condvar::new()),
            task_completed: Arc::new(Condvar::new()),
            stats: Arc::new(Mutex::new(QueueStats::default())),
        }
    }
    
    /// Submit a new task to the queue
    pub fn submit(&self, task: TaskBox) -> Result<TaskId> {
        let task_id = TaskId::new_v4();
        let priority_task = PriorityTask {
            task,
            task_id,
            submitted_at: Instant::now(),
        };
        
        // Add to pending queue
        {
            let mut pending = self.pending.lock()
                .map_err(|_| TaskError::LockError("Failed to acquire pending queue lock".to_string()))?;
            pending.push(priority_task);
        }
        
        // Update statistics
        {
            let mut stats = self.stats.lock()
                .map_err(|_| TaskError::LockError("Failed to acquire stats lock".to_string()))?;
            stats.total_submitted += 1;
            stats.current_pending += 1;
        }
        
        // Create initial result
        let result = TaskResult::new(task_id);
        {
            let mut completed = self.completed.lock()
                .map_err(|_| TaskError::LockError("Failed to acquire completed queue lock".to_string()))?;
            completed.insert(task_id, result);
        }
        
        // Notify waiting workers
        self.task_available.notify_one();
        
        Ok(task_id)
    }
    
    /// Get the next task to execute (blocks if no tasks available)
    pub fn next_task(&self) -> Result<Option<(TaskId, TaskBox)>> {
        let mut pending = self.pending.lock()
            .map_err(|_| TaskError::LockError("Failed to acquire pending queue lock".to_string()))?;
        
        if let Some(priority_task) = pending.pop() {
            // Move task to running state
            let task_id = priority_task.task_id;
            let task = priority_task.task;
            
            {
                let mut running = self.running.lock()
                    .map_err(|_| TaskError::LockError("Failed to acquire running queue lock".to_string()))?;
                running.insert(task_id, task);
            }
            
            // Update statistics
            {
                let mut stats = self.stats.lock()
                    .map_err(|_| TaskError::LockError("Failed to acquire stats lock".to_string()))?;
                stats.current_pending -= 1;
                stats.current_running += 1;
            }
            
            // Update task result status
            self.update_task_status(task_id, TaskStatus::Running)?;
            
            // Get the task back from running queue to return it
            let running = self.running.lock()
                .map_err(|_| TaskError::LockError("Failed to acquire running queue lock".to_string()))?;
            if let Some(task) = running.get(&task_id) {
                // We need to clone the task for execution while keeping it in running state
                // This is a limitation of our current design - in a real system you might
                // handle this differently
                Ok(Some((task_id, TaskBox::new(task.downcast_ref::<crate::task::traits::MathTask>().unwrap().clone()))))
            } else {
                Err(TaskError::TaskNotFound(task_id))
            }
        } else {
            Ok(None)
        }
    }
    
    /// Wait for the next task to become available
    pub fn wait_for_task(&self, timeout: Option<Duration>) -> Result<Option<(TaskId, TaskBox)>> {
        let pending = self.pending.lock()
            .map_err(|_| TaskError::LockError("Failed to acquire pending queue lock".to_string()))?;
        
        let pending = if pending.is_empty() {
            if let Some(timeout) = timeout {
                self.task_available.wait_timeout(pending, timeout)
                    .map_err(|_| TaskError::LockError("Condition variable wait failed".to_string()))?
                    .0
            } else {
                self.task_available.wait(pending)
                    .map_err(|_| TaskError::LockError("Condition variable wait failed".to_string()))?
            }
        } else {
            pending
        };
        
        drop(pending);
        self.next_task()
    }
    
    /// Mark a task as completed with a result
    pub fn complete_task(&self, task_id: TaskId, output: String) -> Result<()> {
        // Remove from running queue
        {
            let mut running = self.running.lock()
                .map_err(|_| TaskError::LockError("Failed to acquire running queue lock".to_string()))?;
            running.remove(&task_id);
        }
        
        // Update result
        {
            let mut completed = self.completed.lock()
                .map_err(|_| TaskError::LockError("Failed to acquire completed queue lock".to_string()))?;
            if let Some(result) = completed.get_mut(&task_id) {
                result.mark_completed(output);
            } else {
                return Err(TaskError::TaskNotFound(task_id));
            }
        }
        
        // Update statistics
        {
            let mut stats = self.stats.lock()
                .map_err(|_| TaskError::LockError("Failed to acquire stats lock".to_string()))?;
            stats.current_running -= 1;
            stats.total_completed += 1;
        }
        
        // Notify waiters
        self.task_completed.notify_all();
        
        Ok(())
    }
    
    /// Mark a task as failed with an error
    pub fn fail_task(&self, task_id: TaskId, error: String) -> Result<()> {
        // Remove from running queue
        {
            let mut running = self.running.lock()
                .map_err(|_| TaskError::LockError("Failed to acquire running queue lock".to_string()))?;
            running.remove(&task_id);
        }
        
        // Update result
        {
            let mut completed = self.completed.lock()
                .map_err(|_| TaskError::LockError("Failed to acquire completed queue lock".to_string()))?;
            if let Some(result) = completed.get_mut(&task_id) {
                result.mark_failed(error);
            } else {
                return Err(TaskError::TaskNotFound(task_id));
            }
        }
        
        // Update statistics
        {
            let mut stats = self.stats.lock()
                .map_err(|_| TaskError::LockError("Failed to acquire stats lock".to_string()))?;
            stats.current_running -= 1;
            stats.total_failed += 1;
        }
        
        // Notify waiters
        self.task_completed.notify_all();
        
        Ok(())
    }
    
    /// Get the result of a task
    pub fn get_result(&self, task_id: TaskId) -> Option<TaskResult> {
        let completed = self.completed.lock().ok()?;
        completed.get(&task_id).cloned()
    }
    
    /// Wait for a task to complete and return its result
    pub fn wait_for_result(&self, task_id: TaskId, timeout: Option<Duration>) -> Result<TaskResult> {
        let completed = self.completed.lock()
            .map_err(|_| TaskError::LockError("Failed to acquire completed queue lock".to_string()))?;
        
        // Check if task is already completed
        if let Some(result) = completed.get(&task_id) {
            if result.is_terminal() {
                return Ok(result.clone());
            }
        }
        
        // Wait for completion
        let completed = if let Some(timeout) = timeout {
            let (completed, timeout_result) = self.task_completed.wait_timeout_while(
                completed,
                timeout,
                |completed| {
                    completed.get(&task_id)
                        .map(|r| !r.is_terminal())
                        .unwrap_or(true)
                }
            ).map_err(|_| TaskError::LockError("Condition variable wait failed".to_string()))?;
            
            if timeout_result.timed_out() {
                return Err(TaskError::Timeout {
                    task_name: task_id.to_string(),
                    duration: timeout,
                });
            }
            
            completed
        } else {
            self.task_completed.wait_while(completed, |completed| {
                completed.get(&task_id)
                    .map(|r| !r.is_terminal())
                    .unwrap_or(true)
            }).map_err(|_| TaskError::LockError("Condition variable wait failed".to_string()))?
        };
        
        // Return the result
        completed.get(&task_id)
            .cloned()
            .ok_or(TaskError::TaskNotFound(task_id))
    }
    
    /// Cancel a pending task
    pub fn cancel_task(&self, task_id: TaskId) -> Result<bool> {
        // Try to remove from pending queue first
        {
            let mut pending = self.pending.lock()
                .map_err(|_| TaskError::LockError("Failed to acquire pending queue lock".to_string()))?;
            
            // Convert heap to vec, remove task, and rebuild heap
            let mut tasks: Vec<_> = pending.drain().collect();
            let original_len = tasks.len();
            tasks.retain(|t| t.task_id != task_id);
            
            if tasks.len() < original_len {
                // Task was found and removed
                for task in tasks {
                    pending.push(task);
                }
                
                // Update result
                {
                    let mut completed = self.completed.lock()
                        .map_err(|_| TaskError::LockError("Failed to acquire completed queue lock".to_string()))?;
                    if let Some(result) = completed.get_mut(&task_id) {
                        result.mark_cancelled();
                    }
                }
                
                // Update statistics
                {
                    let mut stats = self.stats.lock()
                        .map_err(|_| TaskError::LockError("Failed to acquire stats lock".to_string()))?;
                    stats.current_pending -= 1;
                    stats.total_cancelled += 1;
                }
                
                self.task_completed.notify_all();
                return Ok(true);
            }
        }
        
        // Task not found in pending queue
        Ok(false)
    }
    
    /// Get current queue statistics
    pub fn stats(&self) -> Result<QueueStats> {
        let stats = self.stats.lock()
            .map_err(|_| TaskError::LockError("Failed to acquire stats lock".to_string()))?;
        Ok(stats.clone())
    }
    
    /// Get the number of pending tasks
    pub fn pending_count(&self) -> Result<usize> {
        let pending = self.pending.lock()
            .map_err(|_| TaskError::LockError("Failed to acquire pending queue lock".to_string()))?;
        Ok(pending.len())
    }
    
    /// Get the number of running tasks
    pub fn running_count(&self) -> Result<usize> {
        let running = self.running.lock()
            .map_err(|_| TaskError::LockError("Failed to acquire running queue lock".to_string()))?;
        Ok(running.len())
    }
    
    /// Get the number of completed tasks
    pub fn completed_count(&self) -> Result<usize> {
        let completed = self.completed.lock()
            .map_err(|_| TaskError::LockError("Failed to acquire completed queue lock".to_string()))?;
        Ok(completed.len())
    }
    
    /// Clear all completed tasks from memory
    pub fn clear_completed(&self) -> Result<usize> {
        let mut completed = self.completed.lock()
            .map_err(|_| TaskError::LockError("Failed to acquire completed queue lock".to_string()))?;
        let count = completed.len();
        completed.clear();
        Ok(count)
    }
    
    /// Update task status (internal helper)
    fn update_task_status(&self, task_id: TaskId, status: TaskStatus) -> Result<()> {
        let mut completed = self.completed.lock()
            .map_err(|_| TaskError::LockError("Failed to acquire completed queue lock".to_string()))?;
        
        if let Some(result) = completed.get_mut(&task_id) {
            match status {
                TaskStatus::Running => result.mark_started(),
                _ => {} // Other status updates handled by specific methods
            }
        }
        
        Ok(())
    }
}

impl Default for TaskQueue {
    fn default() -> Self {
        Self::new()
    }
}

// Implement Clone for TaskQueue to allow sharing across threads
impl Clone for TaskQueue {
    fn clone(&self) -> Self {
        Self {
            pending: Arc::clone(&self.pending),
            running: Arc::clone(&self.running),
            completed: Arc::clone(&self.completed),
            task_available: Arc::clone(&self.task_available),
            task_completed: Arc::clone(&self.task_completed),
            stats: Arc::clone(&self.stats),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::task::traits::{MathTask, Task};
    use std::thread;
    use std::time::Duration;
    
    #[test]
    fn test_task_submission() {
        let queue = TaskQueue::new();
        let task = MathTask {
            operation: "add".to_string(),
            operands: vec![1.0, 2.0, 3.0],
        };
        
        let task_id = queue.submit(TaskBox::new(task)).unwrap();
        assert_eq!(queue.pending_count().unwrap(), 1);
        
        let result = queue.get_result(task_id).unwrap();
        assert_eq!(result.status, TaskStatus::Pending);
    }
    
    #[test]
    fn test_task_execution_flow() {
        let queue = TaskQueue::new();
        let task = MathTask {
            operation: "multiply".to_string(),
            operands: vec![2.0, 3.0],
        };
        
        let task_id = queue.submit(TaskBox::new(task)).unwrap();
        
        // Get next task
        let (retrieved_id, _retrieved_task) = queue.next_task().unwrap().unwrap();
        assert_eq!(retrieved_id, task_id);
        assert_eq!(queue.running_count().unwrap(), 1);
        assert_eq!(queue.pending_count().unwrap(), 0);
        
        // Complete the task
        queue.complete_task(task_id, "6.0".to_string()).unwrap();
        assert_eq!(queue.running_count().unwrap(), 0);
        
        let result = queue.get_result(task_id).unwrap();
        assert_eq!(result.status, TaskStatus::Completed);
        assert_eq!(result.output, Some("6.0".to_string()));
    }
    
    #[test]
    fn test_priority_ordering() {
        let queue = TaskQueue::new();
        
        // Submit tasks with different priorities
        let low_priority = MathTask {
            operation: "add".to_string(),
            operands: vec![1.0, 1.0],
        };
        let mut low_task_box = TaskBox::new(low_priority);
        low_task_box.metadata_mut().priority = 1;
        
        let high_priority = MathTask {
            operation: "multiply".to_string(),
            operands: vec![2.0, 2.0],
        };
        let mut high_task_box = TaskBox::new(high_priority);
        high_task_box.metadata_mut().priority = 10;
        
        let _low_id = queue.submit(low_task_box).unwrap();
        let high_id = queue.submit(high_task_box).unwrap();
        
        // Higher priority task should come first
        let (retrieved_id, _) = queue.next_task().unwrap().unwrap();
        assert_eq!(retrieved_id, high_id);
    }
    
    #[test]
    fn test_task_cancellation() {
        let queue = TaskQueue::new();
        let task = MathTask {
            operation: "add".to_string(),
            operands: vec![1.0, 2.0],
        };
        
        let task_id = queue.submit(TaskBox::new(task)).unwrap();
        assert_eq!(queue.pending_count().unwrap(), 1);
        
        let cancelled = queue.cancel_task(task_id).unwrap();
        assert!(cancelled);
        assert_eq!(queue.pending_count().unwrap(), 0);
        
        let result = queue.get_result(task_id).unwrap();
        assert_eq!(result.status, TaskStatus::Cancelled);
    }
    
    #[test]
    fn test_concurrent_access() {
        let queue = Arc::new(TaskQueue::new());
        let mut handles = vec![];
        
        // Spawn multiple threads submitting tasks
        for i in 0..10 {
            let queue_clone = Arc::clone(&queue);
            let handle = thread::spawn(move || {
                let task = MathTask {
                    operation: "add".to_string(),
                    operands: vec![i as f64, i as f64],
                };
                queue_clone.submit(TaskBox::new(task)).unwrap()
            });
            handles.push(handle);
        }
        
        // Wait for all submissions
        let mut task_ids = vec![];
        for handle in handles {
            task_ids.push(handle.join().unwrap());
        }
        
        assert_eq!(queue.pending_count().unwrap(), 10);
        
        // Process all tasks
        for task_id in task_ids {
            if let Some((id, _task)) = queue.next_task().unwrap() {
                queue.complete_task(id, "result".to_string()).unwrap();
            }
        }
        
        assert_eq!(queue.running_count().unwrap(), 0);
        assert_eq!(queue.completed_count().unwrap(), 10);
    }
}