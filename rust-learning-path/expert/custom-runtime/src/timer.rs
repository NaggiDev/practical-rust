//! A simple timer future for testing our custom runtime.
//! 
//! This provides a basic async timer that completes after a specified duration.
//! It demonstrates how to implement custom futures and integrate with our waker system.

use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

/// A simple timer future that completes after a specified duration.
/// 
/// This timer uses busy-waiting (checking the current time on each poll)
/// rather than integrating with a proper timer system. In a real runtime,
/// you'd typically use epoll/kqueue or similar for efficient waiting.
pub struct Timer {
    /// When this timer should complete
    deadline: Instant,
    
    /// Whether this timer has been polled before
    /// Used to print debug information on first poll
    first_poll: bool,
}

impl Timer {
    /// Create a new timer that will complete after the specified duration.
    /// 
    /// The timer starts counting from when this function is called,
    /// not from when it's first polled.
    pub fn new(duration: Duration) -> Self {
        Self {
            deadline: Instant::now() + duration,
            first_poll: true,
        }
    }

    /// Create a timer that completes after the specified number of milliseconds.
    pub fn after_millis(millis: u64) -> Self {
        Self::new(Duration::from_millis(millis))
    }

    /// Create a timer that completes after the specified number of seconds.
    pub fn after_secs(secs: u64) -> Self {
        Self::new(Duration::from_secs(secs))
    }

    /// Check if this timer has expired.
    pub fn is_expired(&self) -> bool {
        Instant::now() >= self.deadline
    }

    /// Get the remaining time until this timer expires.
    /// Returns None if the timer has already expired.
    pub fn remaining(&self) -> Option<Duration> {
        let now = Instant::now();
        if now >= self.deadline {
            None
        } else {
            Some(self.deadline - now)
        }
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Print debug info on first poll
        if self.first_poll {
            if let Some(remaining) = self.remaining() {
                println!("Timer: Starting, will complete in {:?}", remaining);
            }
            self.first_poll = false;
        }

        // Check if the timer has expired
        if self.is_expired() {
            println!("Timer: Completed!");
            Poll::Ready(())
        } else {
            // Timer hasn't expired yet
            if let Some(remaining) = self.remaining() {
                println!("Timer: Still waiting, {:?} remaining", remaining);
            }
            
            // In a real implementation, we would register this timer with
            // a timer wheel or similar data structure, and the waker would
            // be called when the timer expires.
            // 
            // For this simple implementation, we immediately wake ourselves
            // so we'll be polled again soon. This creates a busy-wait loop,
            // which is inefficient but demonstrates the polling mechanism.
            cx.waker().wake_by_ref();
            
            Poll::Pending
        }
    }
}

/// A timer that yields control back to the executor a specified number of times.
/// 
/// This is useful for testing cooperative multitasking - the timer will return
/// Poll::Pending the specified number of times before completing.
pub struct YieldTimer {
    /// Number of times left to yield
    yields_remaining: usize,
}

impl YieldTimer {
    /// Create a new yield timer that will yield the specified number of times.
    pub fn new(yields: usize) -> Self {
        Self {
            yields_remaining: yields,
        }
    }
}

impl Future for YieldTimer {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.yields_remaining == 0 {
            println!("YieldTimer: Completed after all yields");
            Poll::Ready(())
        } else {
            println!("YieldTimer: Yielding, {} yields remaining", self.yields_remaining);
            self.yields_remaining -= 1;
            
            // Wake ourselves immediately so we'll be polled again
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}

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
    fn test_timer_creation() {
        let timer = Timer::new(Duration::from_millis(100));
        assert!(!timer.is_expired()); // Should not be expired immediately
        assert!(timer.remaining().is_some());
    }

    #[test]
    fn test_timer_expiration() {
        let timer = Timer::new(Duration::from_millis(0)); // Expires immediately
        assert!(timer.is_expired());
        assert!(timer.remaining().is_none());
    }

    #[test]
    fn test_timer_polling_expired() {
        let mut timer = Timer::new(Duration::from_millis(0)); // Expires immediately
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);
        
        match Pin::new(&mut timer).poll(&mut cx) {
            Poll::Ready(()) => {}, // Expected
            Poll::Pending => panic!("Expired timer should return Ready"),
        }
    }

    #[test]
    fn test_timer_polling_not_expired() {
        let mut timer = Timer::new(Duration::from_secs(10)); // Long duration
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);
        
        match Pin::new(&mut timer).poll(&mut cx) {
            Poll::Ready(()) => panic!("Non-expired timer should return Pending"),
            Poll::Pending => {}, // Expected
        }
    }

    #[test]
    fn test_yield_timer() {
        let mut timer = YieldTimer::new(2);
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);
        
        // First poll should return Pending
        match Pin::new(&mut timer).poll(&mut cx) {
            Poll::Pending => {},
            Poll::Ready(()) => panic!("Should yield first time"),
        }
        
        // Second poll should return Pending
        match Pin::new(&mut timer).poll(&mut cx) {
            Poll::Pending => {},
            Poll::Ready(()) => panic!("Should yield second time"),
        }
        
        // Third poll should return Ready
        match Pin::new(&mut timer).poll(&mut cx) {
            Poll::Ready(()) => {},
            Poll::Pending => panic!("Should complete after all yields"),
        }
    }

    #[test]
    fn test_yield_timer_zero_yields() {
        let mut timer = YieldTimer::new(0);
        let waker = noop_waker();
        let mut cx = Context::from_waker(&waker);
        
        // Should complete immediately
        match Pin::new(&mut timer).poll(&mut cx) {
            Poll::Ready(()) => {},
            Poll::Pending => panic!("Zero-yield timer should complete immediately"),
        }
    }
}