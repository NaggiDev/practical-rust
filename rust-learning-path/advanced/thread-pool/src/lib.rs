//! # Thread Pool Implementation
//!
//! A custom thread pool implementation that demonstrates advanced concurrency concepts
//! in Rust, including thread management, work distribution, and graceful shutdown.

use std::sync::{mpsc, Arc, Mutex};
use std::thread;

mod worker;
pub use worker::Worker;

/// Type alias for a job that can be executed by the thread pool
type Job = Box<dyn FnOnce() + Send + 'static>;

/// Messages that can be sent to worker threads
pub enum Message {
    NewJob(Job),
    Terminate,
}

/// A thread pool for executing jobs concurrently
///
/// # Examples
///
/// ```
/// use thread_pool::ThreadPool;
/// use std::sync::{Arc, Mutex};
///
/// let pool = ThreadPool::new(4).unwrap();
/// let counter = Arc::new(Mutex::new(0));
///
/// for _ in 0..10 {
///     let counter = Arc::clone(&counter);
///     pool.execute(move || {
///         let mut num = counter.lock().unwrap();
///         *num += 1;
///     }).unwrap();
/// }
/// 
/// // Pool will be dropped here, waiting for all jobs to complete
/// ```
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Message>>,
}

/// Errors that can occur when working with the thread pool
#[derive(Debug)]
pub enum ThreadPoolError {
    /// Failed to create the thread pool
    CreationFailed(String),
    /// Failed to execute a job
    ExecutionFailed(String),
    /// Thread pool has been shut down
    ShutDown,
}

impl std::fmt::Display for ThreadPoolError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ThreadPoolError::CreationFailed(msg) => write!(f, "Thread pool creation failed: {}", msg),
            ThreadPoolError::ExecutionFailed(msg) => write!(f, "Job execution failed: {}", msg),
            ThreadPoolError::ShutDown => write!(f, "Thread pool has been shut down"),
        }
    }
}

impl std::error::Error for ThreadPoolError {}

impl ThreadPool {
    /// Create a new ThreadPool with the specified number of threads.
    ///
    /// # Arguments
    ///
    /// * `size` - The number of worker threads to create
    ///
    /// # Returns
    ///
    /// * `Ok(ThreadPool)` - A new thread pool instance
    /// * `Err(ThreadPoolError)` - If thread pool creation fails
    ///
    /// # Examples
    ///
    /// ```
    /// use thread_pool::ThreadPool;
    ///
    /// let pool = ThreadPool::new(4).unwrap();
    /// ```
    pub fn new(size: usize) -> Result<ThreadPool, ThreadPoolError> {
        if size == 0 {
            return Err(ThreadPoolError::CreationFailed(
                "Thread pool size must be greater than 0".to_string(),
            ));
        }

        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);

        for id in 0..size {
            // TODO: Create worker threads
            // Each worker should:
            // 1. Have a unique ID
            // 2. Share the receiver to get jobs
            // 3. Handle potential thread creation failures
            
            match Worker::new(id, Arc::clone(&receiver)) {
                Ok(worker) => workers.push(worker),
                Err(e) => {
                    return Err(ThreadPoolError::CreationFailed(
                        format!("Failed to create worker {}: {}", id, e)
                    ));
                }
            }
        }

        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }

    /// Execute a job on the thread pool.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that implements FnOnce() + Send + 'static
    ///
    /// # Returns
    ///
    /// * `Ok(())` - Job was successfully queued
    /// * `Err(ThreadPoolError)` - If the job couldn't be queued
    ///
    /// # Examples
    ///
    /// ```
    /// use thread_pool::ThreadPool;
    ///
    /// let pool = ThreadPool::new(2).unwrap();
    /// pool.execute(|| {
    ///     println!("Hello from thread pool!");
    /// }).unwrap();
    /// ```
    pub fn execute<F>(&self, f: F) -> Result<(), ThreadPoolError>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);

        match &self.sender {
            Some(sender) => {
                sender.send(Message::NewJob(job))
                    .map_err(|_| ThreadPoolError::ExecutionFailed(
                        "Failed to send job to workers".to_string()
                    ))?;
                Ok(())
            }
            None => Err(ThreadPoolError::ShutDown),
        }
    }

    /// Get the number of worker threads in the pool
    pub fn size(&self) -> usize {
        self.workers.len()
    }

    /// Gracefully shutdown the thread pool
    ///
    /// This method will:
    /// 1. Stop accepting new jobs
    /// 2. Signal all workers to terminate
    /// 3. Wait for all workers to finish their current jobs
    pub fn shutdown(&mut self) {
        println!("Shutting down thread pool...");

        // Drop the sender to stop accepting new jobs
        if let Some(sender) = self.sender.take() {
            // Send terminate message to all workers
            for _ in &self.workers {
                let _ = sender.send(Message::Terminate);
            }
        }

        // Wait for all workers to finish
        for worker in &mut self.workers {
            println!("Shutting down worker {}", worker.id());
            
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }

        println!("Thread pool shutdown complete.");
    }
}

impl Drop for ThreadPool {
    /// Automatically shutdown the thread pool when it goes out of scope
    fn drop(&mut self) {
        self.shutdown();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};
    use std::time::Duration;

    #[test]
    fn test_thread_pool_creation() {
        let pool = ThreadPool::new(4);
        assert!(pool.is_ok());
        assert_eq!(pool.unwrap().size(), 4);
    }

    #[test]
    fn test_zero_size_pool() {
        let pool = ThreadPool::new(0);
        assert!(pool.is_err());
    }

    #[test]
    fn test_execute_job() {
        let pool = ThreadPool::new(2).unwrap();
        let counter = Arc::new(Mutex::new(0));
        let counter_clone = Arc::clone(&counter);

        pool.execute(move || {
            let mut num = counter_clone.lock().unwrap();
            *num += 1;
        }).unwrap();

        // Give the job time to execute
        thread::sleep(Duration::from_millis(100));

        let final_count = *counter.lock().unwrap();
        assert_eq!(final_count, 1);
    }

    #[test]
    fn test_multiple_jobs() {
        let pool = ThreadPool::new(4).unwrap();
        let counter = Arc::new(Mutex::new(0));

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            pool.execute(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }).unwrap();
        }

        // Give jobs time to execute
        thread::sleep(Duration::from_millis(500));

        let final_count = *counter.lock().unwrap();
        assert_eq!(final_count, 10);
    }
}