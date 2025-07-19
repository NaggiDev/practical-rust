//! Lock-free task queue implementation.
//!
//! This module provides a high-performance, lock-free task queue
//! for the task execution engine.

use std::sync::atomic::{AtomicUsize, Ordering};
use crossbeam::queue::SegQueue;

/// A lock-free task queue
pub struct LockFreeTaskQueue<T> {
    queue: SegQueue<T>,
    size: AtomicUsize,
    capacity: usize,
}

impl<T> LockFreeTaskQueue<T> {
    /// Create a new lock-free task queue with the specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            queue: SegQueue::new(),
            size: AtomicUsize::new(0),
            capacity,
        }
    }

    /// Push a task to the queue
    pub fn push(&self, task: T) -> Result<(), T> {
        let current_size = self.size.load(Ordering::Relaxed);
        if current_size >= self.capacity {
            return Err(task);
        }

        self.queue.push(task);
        self.size.fetch_add(1, Ordering::Relaxed);
        Ok(())
    }

    /// Pop a task from the queue
    pub fn pop(&self) -> Option<T> {
        match self.queue.pop() {
            Some(task) => {
                self.size.fetch_sub(1, Ordering::Relaxed);
                Some(task)
            }
            None => None,
        }
    }

    /// Get the current size of the queue
    pub fn len(&self) -> usize {
        self.size.load(Ordering::Relaxed)
    }

    /// Check if the queue is empty
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Check if the queue is full
    pub fn is_full(&self) -> bool {
        self.len() >= self.capacity
    }

    /// Get the capacity of the queue
    pub fn capacity(&self) -> usize {
        self.capacity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue_creation() {
        let queue: LockFreeTaskQueue<i32> = LockFreeTaskQueue::new(10);
        assert_eq!(queue.capacity(), 10);
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
        assert!(!queue.is_full());
    }

    #[test]
    fn test_push_pop() {
        let queue = LockFreeTaskQueue::new(5);
        
        // Push some items
        assert!(queue.push(1).is_ok());
        assert!(queue.push(2).is_ok());
        assert!(queue.push(3).is_ok());
        
        assert_eq!(queue.len(), 3);
        assert!(!queue.is_empty());
        
        // Pop items
        assert_eq!(queue.pop(), Some(1));
        assert_eq!(queue.pop(), Some(2));
        assert_eq!(queue.pop(), Some(3));
        
        assert_eq!(queue.len(), 0);
        assert!(queue.is_empty());
        assert_eq!(queue.pop(), None);
    }

    #[test]
    fn test_capacity_limit() {
        let queue = LockFreeTaskQueue::new(2);
        
        assert!(queue.push(1).is_ok());
        assert!(queue.push(2).is_ok());
        
        // Should fail when at capacity
        assert_eq!(queue.push(3), Err(3));
        assert!(queue.is_full());
    }
}