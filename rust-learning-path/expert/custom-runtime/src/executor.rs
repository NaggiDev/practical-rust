//! The main executor implementation for our custom runtime.
//! 
//! The executor is responsible for:
//! - Spawning new tasks
//! - Scheduling and polling ready tasks
//! - Managing task lifecycle
//! - Running the main event loop

use crate::task::Task;
use crate::waker::TaskWaker;
use std::collections::VecDeque;
use std::future::Future;
use std::task::{Context, Poll};

/// A simple single-threaded executor for async tasks.
/// 
/// This executor uses a basic round-robin scheduling approach:
/// 1. Tasks are stored in a queue
/// 2. Ready tasks are polled in order
/// 3. Completed tasks are removed
/// 4. The process repeats until all tasks are done
pub struct Executor {
    /// Queue of tasks waiting to be executed
    /// We use VecDeque for efficient push/pop operations
    task_queue: VecDeque<Task>,
}

impl Executor {
    /// Create a new executor with an empty task queue.
    pub fn new() -> Self {
        Self {
            task_queue: VecDeque::new(),
        }
    }

    /// Spawn a new task on this executor.
    /// 
    /// The task will be added to the queue and executed when the
    /// executor runs. The future must return () for simplicity.
    pub fn spawn<F>(&mut self, future: F)
    where
        F: Future<Output = ()> + Send + 'static,
    {
        let task = Task::new(future);
        self.task_queue.push_back(task);
    }

    /// Run the executor until all tasks complete.
    /// 
    /// This is the main event loop that:
    /// 1. Checks each task to see if it's ready
    /// 2. Polls ready tasks
    /// 3. Removes completed tasks
    /// 4. Repeats until no tasks remain
    /// 
    /// Returns the number of tasks that were executed.
    pub fn run(&mut self) -> usize {
        let mut completed_tasks = 0;

        // Keep running while we have tasks
        while !self.task_queue.is_empty() {
            let mut tasks_to_retry = VecDeque::new();
            
            // Process all current tasks
            while let Some(mut task) = self.task_queue.pop_front() {
                if task.is_ready() {
                    // Create a waker for this task
                    let ready_handle = task.ready_handle();
                    let task_waker = TaskWaker::new(ready_handle);
                    let waker = task_waker.into_waker();
                    let mut cx = Context::from_waker(&waker);

                    // Poll the task
                    match task.poll(&mut cx) {
                        Poll::Ready(()) => {
                            // Task completed, don't add it back to the queue
                            completed_tasks += 1;
                        }
                        Poll::Pending => {
                            // Task is not ready, mark it as such and re-queue
                            task.set_not_ready();
                            tasks_to_retry.push_back(task);
                        }
                    }
                } else {
                    // Task is not ready, just re-queue it
                    tasks_to_retry.push_back(task);
                }
            }

            // Add all pending tasks back to the queue
            self.task_queue = tasks_to_retry;

            // If no tasks are ready and none completed this iteration,
            // we might be in a deadlock situation
            if self.task_queue.iter().all(|task| !task.is_ready()) && !self.task_queue.is_empty() {
                eprintln!("Warning: All tasks are pending and none are ready. Possible deadlock.");
                break;
            }
        }

        completed_tasks
    }

    /// Run a single iteration of the executor.
    /// 
    /// This polls all ready tasks once and returns the number of
    /// tasks that completed in this iteration.
    /// 
    /// Useful for more fine-grained control over execution.
    pub fn run_once(&mut self) -> usize {
        let mut completed_tasks = 0;
        let mut tasks_to_retry = VecDeque::new();
        
        // Process all current tasks once
        while let Some(mut task) = self.task_queue.pop_front() {
            if task.is_ready() {
                // Create a waker for this task
                let ready_handle = task.ready_handle();
                let task_waker = TaskWaker::new(ready_handle);
                let waker = task_waker.into_waker();
                let mut cx = Context::from_waker(&waker);

                // Poll the task
                match task.poll(&mut cx) {
                    Poll::Ready(()) => {
                        // Task completed
                        completed_tasks += 1;
                    }
                    Poll::Pending => {
                        // Task is not ready, mark it as such and re-queue
                        task.set_not_ready();
                        tasks_to_retry.push_back(task);
                    }
                }
            } else {
                // Task is not ready, just re-queue it
                tasks_to_retry.push_back(task);
            }
        }

        // Add all pending tasks back to the queue
        self.task_queue = tasks_to_retry;
        completed_tasks
    }

    /// Check if the executor has any tasks remaining.
    pub fn has_tasks(&self) -> bool {
        !self.task_queue.is_empty()
    }

    /// Get the number of tasks currently in the queue.
    pub fn task_count(&self) -> usize {
        self.task_queue.len()
    }

    /// Get the number of ready tasks.
    pub fn ready_task_count(&self) -> usize {
        self.task_queue.iter().filter(|task| task.is_ready()).count()
    }
}

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;

    #[test]
    fn test_executor_creation() {
        let executor = Executor::new();
        assert_eq!(executor.task_count(), 0);
        assert!(!executor.has_tasks());
    }

    #[test]
    fn test_spawn_and_run_simple_task() {
        let mut executor = Executor::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        executor.spawn(async move {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        assert_eq!(executor.task_count(), 1);
        let completed = executor.run();
        
        assert_eq!(completed, 1);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
        assert!(!executor.has_tasks());
    }

    #[test]
    fn test_multiple_tasks() {
        let mut executor = Executor::new();
        let counter = Arc::new(AtomicUsize::new(0));

        // Spawn multiple tasks
        for _ in 0..5 {
            let counter_clone = Arc::clone(&counter);
            executor.spawn(async move {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            });
        }

        assert_eq!(executor.task_count(), 5);
        let completed = executor.run();
        
        assert_eq!(completed, 5);
        assert_eq!(counter.load(Ordering::SeqCst), 5);
        assert!(!executor.has_tasks());
    }

    #[test]
    fn test_run_once() {
        let mut executor = Executor::new();
        let counter = Arc::new(AtomicUsize::new(0));

        // Spawn a task that completes immediately
        let counter_clone = Arc::clone(&counter);
        executor.spawn(async move {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        // Run once should complete the task
        let completed = executor.run_once();
        assert_eq!(completed, 1);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
        assert!(!executor.has_tasks());
    }

    #[test]
    fn test_nested_async_blocks() {
        let mut executor = Executor::new();
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = Arc::clone(&counter);

        executor.spawn(async move {
            // Nested async operation
            async {
                counter_clone.fetch_add(1, Ordering::SeqCst);
            }.await;
            
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });

        let completed = executor.run();
        assert_eq!(completed, 1);
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }
}