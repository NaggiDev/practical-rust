//! Custom memory allocator implementation.
//!
//! This module demonstrates unsafe Rust by implementing a custom memory
//! allocator that can be used for task-specific memory management.

use crate::error::{Result, AllocationError, EngineError};
use std::alloc::{GlobalAlloc, Layout};
use std::ptr::{self, NonNull};
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::sync::Mutex;

/// A simple bump allocator for demonstration purposes
pub struct BumpAllocator {
    memory: AtomicPtr<u8>,
    size: usize,
    offset: AtomicUsize,
    allocated_bytes: AtomicUsize,
    allocation_count: AtomicUsize,
}

impl BumpAllocator {
    /// Create a new bump allocator with the specified size
    pub fn new(size: usize) -> Result<Self> {
        if size == 0 {
            return Err(EngineError::memory("Allocator size must be greater than 0"));
        }

        // Allocate memory using the system allocator
        let layout = Layout::from_size_align(size, 8)
            .map_err(|e| EngineError::memory(format!("Invalid layout: {}", e)))?;

        let memory = unsafe { std::alloc::alloc(layout) };
        if memory.is_null() {
            return Err(EngineError::memory("Failed to allocate memory"));
        }

        Ok(Self {
            memory: AtomicPtr::new(memory),
            size,
            offset: AtomicUsize::new(0),
            allocated_bytes: AtomicUsize::new(0),
            allocation_count: AtomicUsize::new(0),
        })
    }

    /// Allocate memory with the specified layout
    pub fn allocate(&self, layout: Layout) -> Result<NonNull<u8>> {
        let size = layout.size();
        let align = layout.align();

        if size == 0 {
            return Err(EngineError::from(AllocationError {
                size,
                align,
                message: "Cannot allocate zero bytes".to_string(),
            }));
        }

        // Calculate aligned offset
        let current_offset = self.offset.load(Ordering::Relaxed);
        let aligned_offset = align_up(current_offset, align);
        let new_offset = aligned_offset + size;

        if new_offset > self.size {
            return Err(EngineError::from(AllocationError {
                size,
                align,
                message: format!(
                    "Out of memory: requested {}, available {}",
                    size,
                    self.size - current_offset
                ),
            }));
        }

        // Try to update the offset atomically
        match self.offset.compare_exchange_weak(
            current_offset,
            new_offset,
            Ordering::Relaxed,
            Ordering::Relaxed,
        ) {
            Ok(_) => {
                // Success, calculate the pointer
                let base_ptr = self.memory.load(Ordering::Relaxed);
                let ptr = unsafe { base_ptr.add(aligned_offset) };

                // Update statistics
                self.allocated_bytes.fetch_add(size, Ordering::Relaxed);
                self.allocation_count.fetch_add(1, Ordering::Relaxed);

                // Safety: We've ensured the pointer is valid and within bounds
                unsafe { Ok(NonNull::new_unchecked(ptr)) }
            }
            Err(_) => {
                // Retry with updated offset
                self.allocate(layout)
            }
        }
    }

    /// Deallocate memory (no-op for bump allocator)
    pub fn deallocate(&self, _ptr: NonNull<u8>, _layout: Layout) {
        // Bump allocators don't support individual deallocation
        // In a real implementation, you might track deallocations for statistics
    }

    /// Reset the allocator (deallocate all memory)
    pub fn reset(&self) {
        self.offset.store(0, Ordering::Relaxed);
        self.allocated_bytes.store(0, Ordering::Relaxed);
        self.allocation_count.store(0, Ordering::Relaxed);
    }

    /// Get allocation statistics
    pub fn statistics(&self) -> AllocationStatistics {
        AllocationStatistics {
            total_size: self.size,
            allocated_bytes: self.allocated_bytes.load(Ordering::Relaxed),
            available_bytes: self.size - self.offset.load(Ordering::Relaxed),
            allocation_count: self.allocation_count.load(Ordering::Relaxed),
            utilization: self.offset.load(Ordering::Relaxed) as f64 / self.size as f64,
        }
    }
}

impl Drop for BumpAllocator {
    fn drop(&mut self) {
        let memory = self.memory.load(Ordering::Relaxed);
        if !memory.is_null() {
            let layout = Layout::from_size_align(self.size, 8).unwrap();
            unsafe {
                std::alloc::dealloc(memory, layout);
            }
        }
    }
}

// Safety: BumpAllocator can be safely shared between threads
unsafe impl Send for BumpAllocator {}
unsafe impl Sync for BumpAllocator {}

/// A free-list allocator that supports deallocation
pub struct FreeListAllocator {
    memory: *mut u8,
    size: usize,
    free_list: Mutex<Vec<FreeBlock>>,
    allocated_bytes: AtomicUsize,
    allocation_count: AtomicUsize,
}

impl FreeListAllocator {
    /// Create a new free-list allocator
    pub fn new(size: usize) -> Result<Self> {
        if size == 0 {
            return Err(EngineError::memory("Allocator size must be greater than 0"));
        }

        let layout = Layout::from_size_align(size, 8)
            .map_err(|e| EngineError::memory(format!("Invalid layout: {}", e)))?;

        let memory = unsafe { std::alloc::alloc(layout) };
        if memory.is_null() {
            return Err(EngineError::memory("Failed to allocate memory"));
        }

        // Initialize with one large free block
        let mut free_list = Vec::new();
        free_list.push(FreeBlock {
            ptr: memory,
            size,
        });

        Ok(Self {
            memory,
            size,
            free_list: Mutex::new(free_list),
            allocated_bytes: AtomicUsize::new(0),
            allocation_count: AtomicUsize::new(0),
        })
    }

    /// Allocate memory with the specified layout
    pub fn allocate(&self, layout: Layout) -> Result<NonNull<u8>> {
        let size = layout.size();
        let align = layout.align();

        if size == 0 {
            return Err(EngineError::from(AllocationError {
                size,
                align,
                message: "Cannot allocate zero bytes".to_string(),
            }));
        }

        let mut free_list = self.free_list.lock().unwrap();

        // Find a suitable free block
        for (i, block) in free_list.iter().enumerate() {
            let aligned_ptr = align_up(block.ptr as usize, align) as *mut u8;
            let padding = aligned_ptr as usize - block.ptr as usize;
            let required_size = padding + size;

            if required_size <= block.size {
                // Found a suitable block
                let allocated_ptr = aligned_ptr;
                
                // Remove the block from the free list
                let mut block = free_list.remove(i);

                // If there's remaining space, add it back to the free list
                if block.size > required_size {
                    let remaining_ptr = unsafe { block.ptr.add(required_size) };
                    let remaining_size = block.size - required_size;
                    
                    free_list.push(FreeBlock {
                        ptr: remaining_ptr,
                        size: remaining_size,
                    });
                }

                // Update statistics
                self.allocated_bytes.fetch_add(size, Ordering::Relaxed);
                self.allocation_count.fetch_add(1, Ordering::Relaxed);

                // Safety: We've verified the pointer is valid and aligned
                return unsafe { Ok(NonNull::new_unchecked(allocated_ptr)) };
            }
        }

        Err(EngineError::from(AllocationError {
            size,
            align,
            message: "No suitable free block found".to_string(),
        }))
    }

    /// Deallocate memory
    pub fn deallocate(&self, ptr: NonNull<u8>, layout: Layout) {
        let size = layout.size();
        let mut free_list = self.free_list.lock().unwrap();

        // Add the block back to the free list
        free_list.push(FreeBlock {
            ptr: ptr.as_ptr(),
            size,
        });

        // Update statistics
        self.allocated_bytes.fetch_sub(size, Ordering::Relaxed);

        // TODO: Coalesce adjacent free blocks for better memory utilization
    }

    /// Get allocation statistics
    pub fn statistics(&self) -> AllocationStatistics {
        let free_list = self.free_list.lock().unwrap();
        let free_bytes: usize = free_list.iter().map(|block| block.size).sum();

        AllocationStatistics {
            total_size: self.size,
            allocated_bytes: self.allocated_bytes.load(Ordering::Relaxed),
            available_bytes: free_bytes,
            allocation_count: self.allocation_count.load(Ordering::Relaxed),
            utilization: (self.size - free_bytes) as f64 / self.size as f64,
        }
    }
}

impl Drop for FreeListAllocator {
    fn drop(&mut self) {
        if !self.memory.is_null() {
            let layout = Layout::from_size_align(self.size, 8).unwrap();
            unsafe {
                std::alloc::dealloc(self.memory, layout);
            }
        }
    }
}

// Safety: FreeListAllocator uses proper synchronization
unsafe impl Send for FreeListAllocator {}
unsafe impl Sync for FreeListAllocator {}

/// Represents a free block in the free-list allocator
#[derive(Debug, Clone)]
struct FreeBlock {
    ptr: *mut u8,
    size: usize,
}

/// Allocation statistics
#[derive(Debug, Clone)]
pub struct AllocationStatistics {
    pub total_size: usize,
    pub allocated_bytes: usize,
    pub available_bytes: usize,
    pub allocation_count: usize,
    pub utilization: f64,
}

/// Align a value up to the specified alignment
fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bump_allocator_creation() {
        let allocator = BumpAllocator::new(1024).unwrap();
        let stats = allocator.statistics();
        assert_eq!(stats.total_size, 1024);
        assert_eq!(stats.allocated_bytes, 0);
        assert_eq!(stats.allocation_count, 0);
    }

    #[test]
    fn test_bump_allocator_allocation() {
        let allocator = BumpAllocator::new(1024).unwrap();
        let layout = Layout::from_size_align(64, 8).unwrap();
        
        let ptr1 = allocator.allocate(layout).unwrap();
        let ptr2 = allocator.allocate(layout).unwrap();
        
        assert_ne!(ptr1.as_ptr(), ptr2.as_ptr());
        
        let stats = allocator.statistics();
        assert_eq!(stats.allocated_bytes, 128);
        assert_eq!(stats.allocation_count, 2);
    }

    #[test]
    fn test_bump_allocator_out_of_memory() {
        let allocator = BumpAllocator::new(64).unwrap();
        let layout = Layout::from_size_align(128, 8).unwrap();
        
        assert!(allocator.allocate(layout).is_err());
    }

    #[test]
    fn test_bump_allocator_reset() {
        let allocator = BumpAllocator::new(1024).unwrap();
        let layout = Layout::from_size_align(64, 8).unwrap();
        
        let _ptr = allocator.allocate(layout).unwrap();
        assert!(allocator.statistics().allocated_bytes > 0);
        
        allocator.reset();
        let stats = allocator.statistics();
        assert_eq!(stats.allocated_bytes, 0);
        assert_eq!(stats.allocation_count, 0);
    }

    #[test]
    fn test_free_list_allocator_creation() {
        let allocator = FreeListAllocator::new(1024).unwrap();
        let stats = allocator.statistics();
        assert_eq!(stats.total_size, 1024);
        assert_eq!(stats.available_bytes, 1024);
    }

    #[test]
    fn test_free_list_allocator_allocation_deallocation() {
        let allocator = FreeListAllocator::new(1024).unwrap();
        let layout = Layout::from_size_align(64, 8).unwrap();
        
        let ptr = allocator.allocate(layout).unwrap();
        let stats_after_alloc = allocator.statistics();
        assert_eq!(stats_after_alloc.allocated_bytes, 64);
        
        allocator.deallocate(ptr, layout);
        let stats_after_dealloc = allocator.statistics();
        assert_eq!(stats_after_dealloc.allocated_bytes, 0);
    }

    #[test]
    fn test_align_up() {
        assert_eq!(align_up(0, 8), 0);
        assert_eq!(align_up(1, 8), 8);
        assert_eq!(align_up(7, 8), 8);
        assert_eq!(align_up(8, 8), 8);
        assert_eq!(align_up(9, 8), 16);
    }

    #[test]
    fn test_zero_size_allocator() {
        assert!(BumpAllocator::new(0).is_err());
        assert!(FreeListAllocator::new(0).is_err());
    }

    #[test]
    fn test_zero_size_allocation() {
        let allocator = BumpAllocator::new(1024).unwrap();
        let layout = Layout::from_size_align(0, 1).unwrap();
        assert!(allocator.allocate(layout).is_err());
    }
}