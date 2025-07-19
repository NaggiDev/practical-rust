//! Custom Memory Allocator Library
//! 
//! This library provides custom memory allocators for learning purposes.
//! It demonstrates unsafe Rust concepts and memory management techniques.

use std::alloc::{GlobalAlloc, Layout};
use std::ptr;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Mutex;

pub mod allocator;
pub mod utils;

pub use allocator::{BumpAllocator, FreeListAllocator};
pub use utils::{align_up, is_aligned};

/// Statistics for tracking allocator behavior
#[derive(Debug, Default)]
pub struct AllocatorStats {
    pub total_allocated: AtomicUsize,
    pub total_deallocated: AtomicUsize,
    pub current_allocated: AtomicUsize,
    pub allocation_count: AtomicUsize,
    pub deallocation_count: AtomicUsize,
}

impl AllocatorStats {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn record_allocation(&self, size: usize) {
        self.total_allocated.fetch_add(size, Ordering::Relaxed);
        self.current_allocated.fetch_add(size, Ordering::Relaxed);
        self.allocation_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn record_deallocation(&self, size: usize) {
        self.total_deallocated.fetch_add(size, Ordering::Relaxed);
        self.current_allocated.fetch_sub(size, Ordering::Relaxed);
        self.deallocation_count.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_stats(&self) -> (usize, usize, usize, usize, usize) {
        (
            self.total_allocated.load(Ordering::Relaxed),
            self.total_deallocated.load(Ordering::Relaxed),
            self.current_allocated.load(Ordering::Relaxed),
            self.allocation_count.load(Ordering::Relaxed),
            self.deallocation_count.load(Ordering::Relaxed),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocator_stats() {
        let stats = AllocatorStats::new();
        
        stats.record_allocation(100);
        stats.record_allocation(200);
        
        let (total_alloc, total_dealloc, current_alloc, alloc_count, dealloc_count) = stats.get_stats();
        
        assert_eq!(total_alloc, 300);
        assert_eq!(total_dealloc, 0);
        assert_eq!(current_alloc, 300);
        assert_eq!(alloc_count, 2);
        assert_eq!(dealloc_count, 0);
        
        stats.record_deallocation(100);
        
        let (total_alloc, total_dealloc, current_alloc, alloc_count, dealloc_count) = stats.get_stats();
        
        assert_eq!(total_alloc, 300);
        assert_eq!(total_dealloc, 100);
        assert_eq!(current_alloc, 200);
        assert_eq!(alloc_count, 2);
        assert_eq!(dealloc_count, 1);
    }
}