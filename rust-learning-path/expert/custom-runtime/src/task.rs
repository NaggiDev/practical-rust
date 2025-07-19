//! Task abstraction for our custom runtime.
//! 
//! A Task wraps a future and provides the interface our executor needs
//! to poll and manage async operations.

use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{Context, Poll};

/// A task represents a unit of async work that can be executed by our runtime.
/// 
/// Tasks wrap futures in a way that allows the executor to:
/// - Poll them when they're ready
/// - Track their completion status
/// - Handle waking and scheduling
pub struct Task {
    /// The future being executed, pinned and boxed for type erasure
    future: Pin<Box<dyn Future<Output = ()> + Send>>,
    
    /// Whether this task is ready to be polled
    /// This is set by the waker when the task should be scheduled
    ready: Arc<AtomicBool>,
}

impl Task {
    /// Create a new task from a future.
    /// 
    /// The future must:
    /// - Return () (unit type) for simplicity
    /// - Be Send so it can be moved between threads
    /// - Be 'static so it doesn't borrow data with shorter lifetimes
    pub fn new<F>(future: F) -> Self
    where
        F: Future<Output = ()> + Send + 'static,
    {
        Self {
            future: Box::pin(future),
            ready: Arc::new(AtomicBool::new(true)), // Start ready for first poll
        }
    }

    /// Check if this task is ready to be polled.
    /// 
    /// Tasks become ready when:
    /// - They're first created
    /// - Their waker is called (indicating progress can be made)
    pub fn is_ready(&self) -> bool {
        self.ready.load(Ordering::Acquire)
    }

    /// Mark this task as not ready.
    /// 
    /// This is called after polling when the task returns Poll::Pending
    pub fn set_not_ready(&self) {
        self.ready.store(false, Ordering::Release);
    }

    /// Get a handle to the ready flag for use by wakers.
    /// 
    /// The waker will use this to signal when the task should be polled again
    pub fn ready_handle(&self) -> Arc<AtomicBool> {
        Arc::clone(&self.ready)
    }

    /// Poll this task with the given context.
    /// 
    /// Returns:
    /// - Poll::Ready(()) if the task completed
    /// - Poll::Pending if the task is waiting for something
    pub fn poll(&mut self, cx: &mut Context<'_>) -> Poll<()> {
        // Poll the underlying future
        // The future is already pinned, so we can poll it directly
        self.future.as_mut().poll(cx)
    }
}

// Task must be Send to be moved between threads in a multi-threaded executor
// This is automatically derived since all fields are Send
unsafe impl Send for Task {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::task::{RawWaker, RawWakerVTable, Waker};

    // Helper function to create a no-op waker for testing
    fn noop_waker() -> Waker {
        fn noop(_: *const ()) {}
        fn clone(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &VTABLE)
        }
        
        const VTABLE: RawWakerVTable = RawWakerVTable::new(clone, noop, noop, noop);
        
        unsafe { Waker::from_raw(RawWaker::new(std::ptr::null(), &VTABLE)) }
    }

    #[test]
    fn test_task_creation() {
        let task = Task::new(async {
            // Simple async block that completes immediately
        });
        
        // New tasks should be ready for their first poll
        assert!(task.is_ready());
    }

    #[test]
    fn test_task_ready_state() {
        let task = Task::new(async {});
        
        // Initially ready
        assert!(task.is_ready());
        
        // Can be marked as not ready
        task.set_not_ready();
        assert!(!task.is_ready());
        
        // Waker can mark it ready again
        let ready_handle = task.ready_handle();
        ready_handle.store(true, Ordering::Release);
        assert!(task.is_ready());
    }

    #[test]
    fn test_task_polling() {
        let mut task = Task::new(async {
            // This async block completes immediately
        });
        
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);
        
        // Should complete on first poll
        match task.poll(&mut cx) {
            Poll::Ready(()) => {}, // Expected
            Poll::Pending => panic!("Task should have completed"),
        }
    }
}