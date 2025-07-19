//! Custom waker implementation for our runtime.
//! 
//! Wakers are the mechanism by which async operations signal that they're
//! ready to make progress. Our custom waker integrates with our task
//! scheduling system.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::task::{RawWaker, RawWakerVTable, Waker};

/// A custom waker that signals task readiness through an atomic boolean.
/// 
/// When an async operation (like a timer or I/O) completes, it calls
/// wake() on the waker, which sets the ready flag to true. The executor
/// can then check this flag to know when to poll the task again.
pub struct TaskWaker {
    /// Shared flag indicating whether the associated task is ready to poll
    ready: Arc<AtomicBool>,
}

impl TaskWaker {
    /// Create a new TaskWaker with the given ready flag.
    /// 
    /// The ready flag is shared between the task and its waker,
    /// allowing the waker to signal when the task should be polled.
    pub fn new(ready: Arc<AtomicBool>) -> Self {
        Self { ready }
    }

    /// Convert this TaskWaker into a standard library Waker.
    /// 
    /// This creates the low-level RawWaker with the appropriate vtable
    /// and wraps it in the safe Waker interface.
    pub fn into_waker(self) -> Waker {
        let raw_waker = self.into_raw_waker();
        unsafe { Waker::from_raw(raw_waker) }
    }

    /// Convert this TaskWaker into a RawWaker.
    /// 
    /// RawWaker is the low-level interface that the Waker uses internally.
    /// We need to provide a vtable with function pointers for the waker operations.
    fn into_raw_waker(self) -> RawWaker {
        // Box the TaskWaker and convert to a raw pointer
        let ptr = Box::into_raw(Box::new(self)) as *const ();
        RawWaker::new(ptr, &VTABLE)
    }

    /// Wake the associated task by marking it as ready.
    /// 
    /// This is called when an async operation completes and the task
    /// should be polled again.
    fn wake(&self) {
        self.ready.store(true, Ordering::Release);
    }
}

/// The vtable for our custom waker.
/// 
/// This provides function pointers for the four operations that can be
/// performed on a waker: wake, wake_by_ref, clone, and drop.
static VTABLE: RawWakerVTable = RawWakerVTable::new(
    clone_waker,
    wake_waker,
    wake_by_ref_waker,
    drop_waker,
);

/// Clone a waker from its raw pointer.
/// 
/// This creates a new RawWaker that shares the same TaskWaker data.
/// Both wakers will signal the same task when woken.
unsafe fn clone_waker(ptr: *const ()) -> RawWaker {
    // Convert back to TaskWaker reference
    let waker = &*(ptr as *const TaskWaker);
    
    // Create a new TaskWaker with the same ready flag
    let cloned = TaskWaker::new(Arc::clone(&waker.ready));
    
    // Convert to RawWaker
    cloned.into_raw_waker()
}

/// Wake a task by consuming the waker.
/// 
/// This is called when wake() is called on the Waker.
/// The waker is consumed in the process.
unsafe fn wake_waker(ptr: *const ()) {
    // Convert back to owned TaskWaker
    let waker = Box::from_raw(ptr as *mut TaskWaker);
    
    // Wake the task
    waker.wake();
    
    // TaskWaker is dropped automatically
}

/// Wake a task by reference without consuming the waker.
/// 
/// This is called when wake_by_ref() is called on the Waker.
/// The waker remains valid after this call.
unsafe fn wake_by_ref_waker(ptr: *const ()) {
    // Convert back to TaskWaker reference (don't take ownership)
    let waker = &*(ptr as *const TaskWaker);
    
    // Wake the task
    waker.wake();
}

/// Drop a waker and clean up its resources.
/// 
/// This is called when the Waker is dropped.
unsafe fn drop_waker(ptr: *const ()) {
    // Convert back to owned TaskWaker and drop it
    let _waker = Box::from_raw(ptr as *mut TaskWaker);
    // Automatic drop happens here
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_waker_creation() {
        let ready = Arc::new(AtomicBool::new(false));
        let task_waker = TaskWaker::new(Arc::clone(&ready));
        let _waker = task_waker.into_waker();
        
        // Should not panic and waker should be created successfully
    }

    #[test]
    fn test_waker_wake() {
        let ready = Arc::new(AtomicBool::new(false));
        let task_waker = TaskWaker::new(Arc::clone(&ready));
        let waker = task_waker.into_waker();
        
        // Initially not ready
        assert!(!ready.load(Ordering::Acquire));
        
        // Wake should set ready to true
        waker.wake();
        assert!(ready.load(Ordering::Acquire));
    }

    #[test]
    fn test_waker_wake_by_ref() {
        let ready = Arc::new(AtomicBool::new(false));
        let task_waker = TaskWaker::new(Arc::clone(&ready));
        let waker = task_waker.into_waker();
        
        // Initially not ready
        assert!(!ready.load(Ordering::Acquire));
        
        // Wake by ref should set ready to true
        waker.wake_by_ref();
        assert!(ready.load(Ordering::Acquire));
        
        // Waker should still be usable
        ready.store(false, Ordering::Release);
        waker.wake_by_ref();
        assert!(ready.load(Ordering::Acquire));
    }

    #[test]
    fn test_waker_clone() {
        let ready = Arc::new(AtomicBool::new(false));
        let task_waker = TaskWaker::new(Arc::clone(&ready));
        let waker1 = task_waker.into_waker();
        let waker2 = waker1.clone();
        
        // Both wakers should affect the same ready flag
        waker1.wake_by_ref();
        assert!(ready.load(Ordering::Acquire));
        
        ready.store(false, Ordering::Release);
        waker2.wake();
        assert!(ready.load(Ordering::Acquire));
    }
}