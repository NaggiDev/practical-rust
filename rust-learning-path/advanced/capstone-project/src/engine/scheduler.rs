//! Task scheduler implementation.
//!
//! This module provides task scheduling capabilities for the execution engine.

use crate::traits::{Schedulable, Task};
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::time::{Duration, Instant};

/// A task scheduler that manages task execution order
pub struct TaskScheduler {
    ready_queue: BinaryHeap<ScheduledTask>,
    waiting_queue: Vec<ScheduledTask>,
}

impl TaskScheduler {
    /// Create a new task scheduler
    pub fn new() -> Self {
        Self {
            ready_queue: BinaryHeap::new(),
            waiting_queue: Vec::new(),
        }
    }

    /// Schedule a task for execution
    pub fn schedule_task<T: Task + Schedulable>(&mut self, task: T) {
        let scheduled_task = ScheduledTask::new(Box::new(task));
        
        if scheduled_task.can_execute() {
            self.ready_queue.push(scheduled_task);
        } else {
            self.waiting_queue.push(scheduled_task);
        }
    }

    /// Get the next ready task
    pub fn next_ready_task(&mut self) -> Option<Box<dyn Task<Output = ()>>> {
        // Move any newly ready tasks from waiting to ready queue
        self.update_ready_tasks();
        
        self.ready_queue.pop().map(|scheduled_task| scheduled_task.task)
    }

    /// Check if there are any ready tasks
    pub fn has_ready_tasks(&self) -> bool {
        !self.ready_queue.is_empty()
    }

    /// Get the number of ready tasks
    pub fn ready_task_count(&self) -> usize {
        self.ready_queue.len()
    }

    /// Get the number of waiting tasks
    pub fn waiting_task_count(&self) -> usize {
        self.waiting_queue.len()
    }

    /// Update the ready queue by moving tasks from waiting queue
    fn update_ready_tasks(&mut self) {
        let mut i = 0;
        while i < self.waiting_queue.len() {
            if self.waiting_queue[i].can_execute() {
                let task = self.waiting_queue.remove(i);
                self.ready_queue.push(task);
            } else {
                i += 1;
            }
        }
    }
}

impl Default for TaskScheduler {
    fn default() -> Self {
        Self::new()
    }
}

/// A scheduled task wrapper
struct ScheduledTask {
    task: Box<dyn Task<Output = ()>>,
    priority: u8,
    estimated_duration: Duration,
    scheduled_at: Instant,
    dependencies: Vec<u64>,
}

impl ScheduledTask {
    fn new<T: Task + Schedulable + 'static>(task: T) -> Self {
        let priority = task.priority();
        let estimated_duration = task.estimated_duration();
        let dependencies = task.dependencies();
        
        Self {
            task: Box::new(TaskWrapper::new(task)),
            priority,
            estimated_duration,
            scheduled_at: Instant::now(),
            dependencies,
        }
    }

    fn can_execute(&self) -> bool {
        // For simplicity, we'll assume dependencies are always satisfied
        // In a real implementation, you'd check dependency completion
        self.dependencies.is_empty()
    }
}

impl PartialEq for ScheduledTask {
    fn eq(&self, other: &Self) -> bool {
        self.priority == other.priority
    }
}

impl Eq for ScheduledTask {}

impl PartialOrd for ScheduledTask {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for ScheduledTask {
    fn cmp(&self, other: &Self) -> Ordering {
        // Higher priority tasks come first
        self.priority.cmp(&other.priority)
            .then_with(|| other.scheduled_at.cmp(&self.scheduled_at)) // Earlier tasks first for same priority
    }
}

/// Wrapper to make any Task + Schedulable compatible with our scheduler
struct TaskWrapper<T> {
    inner: T,
}

impl<T> TaskWrapper<T> {
    fn new(task: T) -> Self {
        Self { inner: task }
    }
}

impl<T: Task + Schedulable> Task for TaskWrapper<T> {
    type Output = ();

    fn id(&self) -> u64 {
        self.inner.id()
    }

    fn name(&self) -> &str {
        self.inner.name()
    }

    fn execute(&self) -> std::pin::Pin<Box<dyn std::future::Future<Output = crate::error::Result<Self::Output>> + Send + '_>> {
        Box::pin(async move {
            let _ = self.inner.execute().await?;
            Ok(())
        })
    }

    fn estimated_cost(&self) -> u32 {
        self.inner.estimated_cost()
    }

    fn priority(&self) -> u8 {
        self.inner.priority()
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self.inner.as_any()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::engine::MathTask;
    use crate::ffi::MathOperation;
    use crate::traits::Schedulable;
    use std::any::Any;
    use std::future::Future;
    use std::pin::Pin;

    // Test task that implements Schedulable
    #[derive(Debug, Clone)]
    struct TestTask {
        id: u64,
        priority: u8,
        duration: Duration,
        dependencies: Vec<u64>,
    }

    impl TestTask {
        fn new(id: u64, priority: u8) -> Self {
            Self {
                id,
                priority,
                duration: Duration::from_millis(100),
                dependencies: Vec::new(),
            }
        }

        fn with_dependencies(mut self, deps: Vec<u64>) -> Self {
            self.dependencies = deps;
            self
        }
    }

    impl Task for TestTask {
        type Output = i32;

        fn id(&self) -> u64 {
            self.id
        }

        fn name(&self) -> &str {
            "TestTask"
        }

        fn execute(&self) -> Pin<Box<dyn Future<Output = crate::error::Result<Self::Output>> + Send + '_>> {
            Box::pin(async move { Ok(42) })
        }

        fn priority(&self) -> u8 {
            self.priority
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    impl Schedulable for TestTask {
        fn priority(&self) -> u8 {
            self.priority
        }

        fn estimated_duration(&self) -> Duration {
            self.duration
        }

        fn dependencies(&self) -> Vec<u64> {
            self.dependencies.clone()
        }
    }

    #[test]
    fn test_scheduler_creation() {
        let scheduler = TaskScheduler::new();
        assert_eq!(scheduler.ready_task_count(), 0);
        assert_eq!(scheduler.waiting_task_count(), 0);
        assert!(!scheduler.has_ready_tasks());
    }

    #[test]
    fn test_task_scheduling() {
        let mut scheduler = TaskScheduler::new();
        
        let task1 = TestTask::new(1, 5);
        let task2 = TestTask::new(2, 3);
        let task3 = TestTask::new(3, 8);
        
        scheduler.schedule_task(task1);
        scheduler.schedule_task(task2);
        scheduler.schedule_task(task3);
        
        assert_eq!(scheduler.ready_task_count(), 3);
        assert!(scheduler.has_ready_tasks());
    }

    #[test]
    fn test_priority_ordering() {
        let mut scheduler = TaskScheduler::new();
        
        // Add tasks with different priorities
        scheduler.schedule_task(TestTask::new(1, 3)); // Low priority
        scheduler.schedule_task(TestTask::new(2, 8)); // High priority
        scheduler.schedule_task(TestTask::new(3, 5)); // Medium priority
        
        // Should get highest priority task first
        let task1 = scheduler.next_ready_task().unwrap();
        assert_eq!(task1.id(), 2); // Highest priority
        
        let task2 = scheduler.next_ready_task().unwrap();
        assert_eq!(task2.id(), 3); // Medium priority
        
        let task3 = scheduler.next_ready_task().unwrap();
        assert_eq!(task3.id(), 1); // Lowest priority
        
        assert!(!scheduler.has_ready_tasks());
    }

    #[test]
    fn test_dependency_handling() {
        let mut scheduler = TaskScheduler::new();
        
        // Task with dependencies should go to waiting queue
        let task_with_deps = TestTask::new(1, 5).with_dependencies(vec![100, 200]);
        let task_without_deps = TestTask::new(2, 3);
        
        scheduler.schedule_task(task_with_deps);
        scheduler.schedule_task(task_without_deps);
        
        assert_eq!(scheduler.ready_task_count(), 1); // Only task without deps
        assert_eq!(scheduler.waiting_task_count(), 1); // Task with deps
    }
}