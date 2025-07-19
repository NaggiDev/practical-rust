//! Work-stealing thread pool implementation.
//!
//! This module implements an advanced thread pool with work stealing capabilities,
//! demonstrating advanced concurrency patterns in Rust.

use crate::error::{Result, EngineError};
use crossbeam::deque::{Injector, Stealer, Worker};
use parking_lot::{Mutex, Condvar};
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tokio::sync::oneshot;

/// A work-stealing thread pool for task execution
pub struct WorkStealingThreadPool {
    workers: Vec<WorkerThread>,
    global_queue: Arc<Injector<Task>>,
    stealers: Vec<Stealer<Task>>,
    active_tasks: Arc<AtomicUsize>,
    queued_tasks: Arc<AtomicUsize>,
    shutdown: Arc<AtomicBool>,
    shutdown_condvar: Arc<(Mutex<bool>, Condvar)>,
}

impl WorkStealingThreadPool {
    /// Create a new work-stealing thread pool
    pub fn new(num_workers: usize, _queue_size: usize) -> Result<Self> {
        if num_workers == 0 {
            return Err(EngineError::thread_pool("Number of workers must be greater than 0"));
        }

        let global_queue = Arc::new(Injector::new());
        let active_tasks = Arc::new(AtomicUsize::new(0));
        let queued_tasks = Arc::new(AtomicUsize::new(0));
        let shutdown = Arc::new(AtomicBool::new(false));
        let shutdown_condvar = Arc::new((Mutex::new(false), Condvar::new()));

        let mut workers = Vec::with_capacity(num_workers);
        let mut stealers = Vec::with_capacity(num_workers);

        // Create workers
        for id in 0..num_workers {
            let worker_queue = Worker::new_fifo();
            let stealer = worker_queue.stealer();
            stealers.push(stealer);

            let worker = WorkerThread::new(
                id,
                worker_queue,
                global_queue.clone(),
                stealers.clone(),
                active_tasks.clone(),
                queued_tasks.clone(),
                shutdown.clone(),
                shutdown_condvar.clone(),
            )?;

            workers.push(worker);
        }

        Ok(Self {
            workers,
            global_queue,
            stealers,
            active_tasks,
            queued_tasks,
            shutdown,
            shutdown_condvar,
        })
    }

    /// Submit a task to the thread pool
    pub fn submit<F>(&self, task: F) -> Result<()>
    where
        F: FnOnce() + Send + 'static,
    {
        if self.shutdown.load(Ordering::Relaxed) {
            return Err(EngineError::thread_pool("Thread pool is shutting down"));
        }

        let task = Task::new(task);
        self.global_queue.push(task);
        self.queued_tasks.fetch_add(1, Ordering::Relaxed);

        // Notify workers that work is available
        let (lock, cvar) = &*self.shutdown_condvar;
        let _guard = lock.lock();
        cvar.notify_all();

        Ok(())
    }

    /// Get the number of active tasks
    pub fn active_tasks(&self) -> usize {
        self.active_tasks.load(Ordering::Relaxed)
    }

    /// Get the number of queued tasks
    pub fn queued_tasks(&self) -> usize {
        self.queued_tasks.load(Ordering::Relaxed)
    }

    /// Shutdown the thread pool gracefully
    pub async fn shutdown(&self) -> Result<()> {
        // Signal shutdown
        self.shutdown.store(true, Ordering::Relaxed);

        // Wake up all workers
        let (lock, cvar) = &*self.shutdown_condvar;
        {
            let mut shutdown_flag = lock.lock();
            *shutdown_flag = true;
        }
        cvar.notify_all();

        // Wait for all workers to finish
        // Note: In a real implementation, we'd need to handle thread joining properly
        // For this example, we'll use a simple timeout
        let timeout = Duration::from_secs(5);
        let start = std::time::Instant::now();

        while self.active_tasks.load(Ordering::Relaxed) > 0 && start.elapsed() < timeout {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }

        Ok(())
    }
}

/// Individual worker thread
struct WorkerThread {
    id: usize,
    handle: Option<thread::JoinHandle<()>>,
}

impl WorkerThread {
    fn new(
        id: usize,
        worker_queue: Worker<Task>,
        global_queue: Arc<Injector<Task>>,
        stealers: Vec<Stealer<Task>>,
        active_tasks: Arc<AtomicUsize>,
        queued_tasks: Arc<AtomicUsize>,
        shutdown: Arc<AtomicBool>,
        shutdown_condvar: Arc<(Mutex<bool>, Condvar)>,
    ) -> Result<Self> {
        let handle = thread::Builder::new()
            .name(format!("worker-{}", id))
            .spawn(move || {
                Self::worker_loop(
                    id,
                    worker_queue,
                    global_queue,
                    stealers,
                    active_tasks,
                    queued_tasks,
                    shutdown,
                    shutdown_condvar,
                );
            })
            .map_err(|e| EngineError::thread_pool(format!("Failed to spawn worker thread: {}", e)))?;

        Ok(Self {
            id,
            handle: Some(handle),
        })
    }

    fn worker_loop(
        id: usize,
        worker_queue: Worker<Task>,
        global_queue: Arc<Injector<Task>>,
        stealers: Vec<Stealer<Task>>,
        active_tasks: Arc<AtomicUsize>,
        queued_tasks: Arc<AtomicUsize>,
        shutdown: Arc<AtomicBool>,
        shutdown_condvar: Arc<(Mutex<bool>, Condvar)>,
    ) {
        loop {
            // Check for shutdown
            if shutdown.load(Ordering::Relaxed) {
                break;
            }

            // Try to find work
            let task = Self::find_task(&worker_queue, &global_queue, &stealers);

            match task {
                Some(task) => {
                    // Execute the task
                    active_tasks.fetch_add(1, Ordering::Relaxed);
                    queued_tasks.fetch_sub(1, Ordering::Relaxed);

                    task.execute();

                    active_tasks.fetch_sub(1, Ordering::Relaxed);
                }
                None => {
                    // No work available, wait for notification or timeout
                    let (lock, cvar) = &*shutdown_condvar;
                    let mut shutdown_flag = lock.lock();
                    
                    if !*shutdown_flag {
                        let _ = cvar.wait_for(&mut shutdown_flag, Duration::from_millis(100));
                    }
                }
            }
        }
    }

    fn find_task(
        worker_queue: &Worker<Task>,
        global_queue: &Arc<Injector<Task>>,
        stealers: &[Stealer<Task>],
    ) -> Option<Task> {
        // First, try to pop from local queue
        if let Some(task) = worker_queue.pop() {
            return Some(task);
        }

        // Then, try to steal from global queue
        loop {
            match global_queue.steal_batch_and_pop(worker_queue) {
                crossbeam::deque::Steal::Success(task) => return Some(task),
                crossbeam::deque::Steal::Empty => break,
                crossbeam::deque::Steal::Retry => continue,
            }
        }

        // Finally, try to steal from other workers
        for stealer in stealers {
            loop {
                match stealer.steal_batch_and_pop(worker_queue) {
                    crossbeam::deque::Steal::Success(task) => return Some(task),
                    crossbeam::deque::Steal::Empty => break,
                    crossbeam::deque::Steal::Retry => continue,
                }
            }
        }

        None
    }
}

impl Drop for WorkerThread {
    fn drop(&mut self) {
        if let Some(handle) = self.handle.take() {
            let _ = handle.join();
        }
    }
}

/// Task wrapper for the thread pool
struct Task {
    task: Box<dyn FnOnce() + Send>,
}

impl Task {
    fn new<F>(task: F) -> Self
    where
        F: FnOnce() + Send + 'static,
    {
        Self {
            task: Box::new(task),
        }
    }

    fn execute(self) {
        (self.task)();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU32, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    #[tokio::test]
    async fn test_thread_pool_creation() {
        let pool = WorkStealingThreadPool::new(4, 100).unwrap();
        assert_eq!(pool.active_tasks(), 0);
        assert_eq!(pool.queued_tasks(), 0);
    }

    #[tokio::test]
    async fn test_task_execution() {
        let pool = WorkStealingThreadPool::new(2, 100).unwrap();
        let counter = Arc::new(AtomicU32::new(0));
        
        // Submit multiple tasks
        for _ in 0..10 {
            let counter_clone = counter.clone();
            pool.submit(move || {
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }).unwrap();
        }

        // Wait for tasks to complete
        tokio::time::sleep(Duration::from_millis(100)).await;
        
        assert_eq!(counter.load(Ordering::Relaxed), 10);
    }

    #[tokio::test]
    async fn test_work_stealing() {
        let pool = WorkStealingThreadPool::new(4, 100).unwrap();
        let counter = Arc::new(AtomicU32::new(0));
        
        // Submit many tasks to test work stealing
        for _ in 0..100 {
            let counter_clone = counter.clone();
            pool.submit(move || {
                // Simulate some work
                thread::sleep(Duration::from_millis(1));
                counter_clone.fetch_add(1, Ordering::Relaxed);
            }).unwrap();
        }

        // Wait for all tasks to complete
        let start = std::time::Instant::now();
        while counter.load(Ordering::Relaxed) < 100 && start.elapsed() < Duration::from_secs(5) {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        
        assert_eq!(counter.load(Ordering::Relaxed), 100);
    }

    #[tokio::test]
    async fn test_shutdown() {
        let pool = WorkStealingThreadPool::new(2, 100).unwrap();
        
        // Submit a task
        let counter = Arc::new(AtomicU32::new(0));
        let counter_clone = counter.clone();
        pool.submit(move || {
            counter_clone.fetch_add(1, Ordering::Relaxed);
        }).unwrap();

        // Wait a bit for the task to execute
        tokio::time::sleep(Duration::from_millis(50)).await;

        // Shutdown the pool
        pool.shutdown().await.unwrap();
        
        // Verify task was executed
        assert_eq!(counter.load(Ordering::Relaxed), 1);
        
        // Verify we can't submit new tasks after shutdown
        assert!(pool.submit(|| {}).is_err());
    }

    #[test]
    fn test_invalid_worker_count() {
        assert!(WorkStealingThreadPool::new(0, 100).is_err());
    }
}