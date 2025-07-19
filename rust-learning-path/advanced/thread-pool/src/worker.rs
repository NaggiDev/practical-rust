//! Worker thread implementation for the thread pool

use crate::Message;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;

/// A worker thread that processes jobs from the thread pool
pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>,
}

/// Errors that can occur when creating a worker
#[derive(Debug)]
pub enum WorkerError {
    ThreadCreationFailed(String),
}

impl std::fmt::Display for WorkerError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkerError::ThreadCreationFailed(msg) => {
                write!(f, "Failed to create worker thread: {}", msg)
            }
        }
    }
}

impl std::error::Error for WorkerError {}

impl Worker {
    /// Create a new worker thread
    ///
    /// # Arguments
    ///
    /// * `id` - Unique identifier for this worker
    /// * `receiver` - Shared receiver for getting jobs from the thread pool
    ///
    /// # Returns
    ///
    /// * `Ok(Worker)` - A new worker instance
    /// * `Err(WorkerError)` - If worker creation fails
    pub fn new(
        id: usize,
        receiver: Arc<Mutex<mpsc::Receiver<Message>>>,
    ) -> Result<Worker, WorkerError> {
        // TODO: Implement worker thread creation
        // The worker should:
        // 1. Spawn a new thread
        // 2. In the thread loop:
        //    - Wait for messages from the receiver
        //    - Execute jobs when received
        //    - Handle terminate messages
        //    - Handle potential panics gracefully
        
        let thread = thread::Builder::new()
            .name(format!("worker-{}", id))
            .spawn(move || {
                println!("Worker {} starting", id);
                
                loop {
                    // Lock the receiver to get the next message
                    let message = receiver.lock().unwrap().recv();
                    
                    match message {
                        Ok(Message::NewJob(job)) => {
                            println!("Worker {} got a job; executing.", id);
                            
                            // Execute the job
                            // We use std::panic::catch_unwind to prevent worker panics
                            // from crashing the entire thread pool
                            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                                job();
                            }));
                            
                            if let Err(_) = result {
                                eprintln!("Worker {} panicked while executing job", id);
                            }
                        }
                        Ok(Message::Terminate) => {
                            println!("Worker {} was told to terminate.", id);
                            break;
                        }
                        Err(_) => {
                            println!("Worker {} disconnected; shutting down.", id);
                            break;
                        }
                    }
                }
                
                println!("Worker {} shutting down", id);
            })
            .map_err(|e| WorkerError::ThreadCreationFailed(e.to_string()))?;

        Ok(Worker {
            id,
            thread: Some(thread),
        })
    }

    /// Get the worker's ID
    pub fn id(&self) -> usize {
        self.id
    }
}