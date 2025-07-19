//! Custom allocator implementations
//! 
//! This module contains different allocator implementations for learning purposes.
//! Each allocator demonstrates different concepts in unsafe Rust and memory management.

use std::alloc::{GlobalAlloc, Layout};
use std::ptr::{self, NonNull};
use std::sync::Mutex;
use crate::{AllocatorStats, utils::*};

/// A simple bump allocator that allocates memory sequentially
/// 
/// This allocator is simple but doesn't support deallocation of individual blocks.
/// It's useful for scenarios where you allocate many objects and free them all at once.
/// 
/// # Safety
/// 
/// This allocator uses unsafe code to manage raw memory. The safety invariants are:
/// - The memory region must be valid for the lifetime of the allocator
/// - Allocated pointers must not be used after the allocator is dropped
/// - The allocator is not thread-safe without external synchronization
pub struct BumpAllocator {
    memory: *mut u8,
    size: usize,
    offset: Mutex<usize>,
    stats: AllocatorStats,
}

impl BumpAllocator {
    /// Create a new bump allocator with the given memory size
    /// 
    /// # Safety
    /// 
    /// The caller must ensure that the allocated memory is properly freed
    /// when the allocator is no longer needed.
    pub fn new(size: usize) -> Result<Self, std::alloc::AllocError> {
        // Allocate memory using the system allocator
        let layout = Layout::from_size_align(size, 8)
            .map_err(|_| std::alloc::AllocError)?;
        
        let memory = unsafe { std::alloc::alloc(layout) };
        if memory.is_null() {
            return Err(std::alloc::AllocError);
        }

        Ok(BumpAllocator {
            memory,
            size,
            offset: Mutex::new(0),
            stats: AllocatorStats::new(),
        })
    }

    /// Get allocation statistics
    pub fn stats(&self) -> (usize, usize, usize, usize, usize) {
        self.stats.get_stats()
    }

    /// Reset the allocator, effectively freeing all allocated memory
    /// 
    /// # Safety
    /// 
    /// After calling this method, all previously allocated pointers become invalid.
    /// Using them will result in undefined behavior.
    pub unsafe fn reset(&self) {
        let mut offset = self.offset.lock().unwrap();
        *offset = 0;
        // Note: We don't update stats here as the memory is still "allocated" from the system
    }
}

unsafe impl GlobalAlloc for BumpAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut offset = self.offset.lock().unwrap();
        
        // Calculate the aligned offset for this allocation
        let current_addr = self.memory as usize + *offset;
        let aligned_addr = align_up(current_addr, layout.align());
        let aligned_offset = aligned_addr - self.memory as usize;
        
        // Check if we have enough space
        if aligned_offset + layout.size() > self.size {
            return ptr::null_mut();
        }
        
        // Update the offset for the next allocation
        *offset = aligned_offset + layout.size();
        
        // Record the allocation
        self.stats.record_allocation(layout.size());
        
        // Return the aligned pointer
        self.memory.add(aligned_offset)
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, layout: Layout) {
        // Bump allocators don't support individual deallocation
        // We still record it for statistics
        self.stats.record_deallocation(layout.size());
    }
}

impl Drop for BumpAllocator {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align_unchecked(self.size, 8);
            std::alloc::dealloc(self.memory, layout);
        }
    }
}

// Safety: BumpAllocator can be safely sent between threads
unsafe impl Send for BumpAllocator {}
// Safety: BumpAllocator can be safely shared between threads (it uses Mutex internally)
unsafe impl Sync for BumpAllocator {}

/// A free list allocator that can reuse freed memory blocks
/// 
/// This allocator maintains a linked list of free memory blocks and can reuse them
/// when new allocations are requested. It's more complex than a bump allocator
/// but supports proper deallocation.
/// 
/// # Safety
/// 
/// This allocator uses extensive unsafe code to manage a free list using raw pointers.
/// The safety invariants are:
/// - Free list pointers must always point to valid memory or be null
/// - The free list must not contain cycles
/// - Allocated memory must not overlap with free list nodes
pub struct FreeListAllocator {
    memory: *mut u8,
    size: usize,
    free_list: Mutex<*mut FreeBlock>,
    stats: AllocatorStats,
}

/// A node in the free list
/// 
/// This struct is stored directly in the free memory blocks.
/// It forms a linked list of available memory blocks.
#[repr(C)]
struct FreeBlock {
    size: usize,
    next: *mut FreeBlock,
}

impl FreeListAllocator {
    /// Create a new free list allocator with the given memory size
    /// 
    /// # Safety
    /// 
    /// The caller must ensure that the allocated memory is properly freed
    /// when the allocator is no longer needed.
    pub fn new(size: usize) -> Result<Self, std::alloc::AllocError> {
        // Allocate memory using the system allocator
        let layout = Layout::from_size_align(size, 8)
            .map_err(|_| std::alloc::AllocError)?;
        
        let memory = unsafe { std::alloc::alloc(layout) };
        if memory.is_null() {
            return Err(std::alloc::AllocError);
        }

        // Initialize the free list with the entire memory block
        let free_list = unsafe {
            let free_block = memory as *mut FreeBlock;
            (*free_block).size = size;
            (*free_block).next = ptr::null_mut();
            free_block
        };

        Ok(FreeListAllocator {
            memory,
            size,
            free_list: Mutex::new(free_list),
            stats: AllocatorStats::new(),
        })
    }

    /// Get allocation statistics
    pub fn stats(&self) -> (usize, usize, usize, usize, usize) {
        self.stats.get_stats()
    }

    /// Find a suitable free block for the given layout
    /// 
    /// Returns a pointer to the previous block (for unlinking) and the block itself.
    /// If the returned previous pointer is null, the block is the first in the list.
    unsafe fn find_free_block(&self, layout: Layout) -> Option<(*mut FreeBlock, *mut FreeBlock)> {
        let mut current = *self.free_list.lock().unwrap();
        let mut previous: *mut FreeBlock = ptr::null_mut();

        while !current.is_null() {
            let block_addr = current as usize;
            let aligned_addr = align_up(block_addr, layout.align());
            let padding = aligned_addr - block_addr;
            let required_size = padding + layout.size();

            if (*current).size >= required_size {
                return Some((previous, current));
            }

            previous = current;
            current = (*current).next;
        }

        None
    }

    /// Remove a block from the free list
    unsafe fn remove_from_free_list(&self, previous: *mut FreeBlock, block: *mut FreeBlock) {
        if previous.is_null() {
            // Removing the first block
            let mut free_list = self.free_list.lock().unwrap();
            *free_list = (*block).next;
        } else {
            // Removing a block in the middle or end
            (*previous).next = (*block).next;
        }
    }

    /// Add a block to the free list
    unsafe fn add_to_free_list(&self, block: *mut FreeBlock, size: usize) {
        let mut free_list = self.free_list.lock().unwrap();
        
        (*block).size = size;
        (*block).next = *free_list;
        *free_list = block;
    }
}

unsafe impl GlobalAlloc for FreeListAllocator {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        // Ensure minimum size for free block metadata
        let min_size = std::mem::size_of::<FreeBlock>();
        let alloc_size = layout.size().max(min_size);
        let alloc_layout = Layout::from_size_align_unchecked(alloc_size, layout.align());

        if let Some((previous, block)) = self.find_free_block(alloc_layout) {
            let block_addr = block as usize;
            let aligned_addr = align_up(block_addr, layout.align());
            let padding = aligned_addr - block_addr;
            let total_size = padding + alloc_size;

            // Remove the block from the free list
            self.remove_from_free_list(previous, block);

            // If there's leftover space, add it back to the free list
            let remaining_size = (*block).size - total_size;
            if remaining_size >= min_size {
                let new_free_block = (block as *mut u8).add(total_size) as *mut FreeBlock;
                self.add_to_free_list(new_free_block, remaining_size);
            }

            // Record the allocation
            self.stats.record_allocation(alloc_size);

            return aligned_addr as *mut u8;
        }

        // No suitable block found
        ptr::null_mut()
    }

    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout) {
        let min_size = std::mem::size_of::<FreeBlock>();
        let dealloc_size = layout.size().max(min_size);

        // Add the block back to the free list
        let free_block = ptr as *mut FreeBlock;
        self.add_to_free_list(free_block, dealloc_size);

        // Record the deallocation
        self.stats.record_deallocation(dealloc_size);
    }
}

impl Drop for FreeListAllocator {
    fn drop(&mut self) {
        unsafe {
            let layout = Layout::from_size_align_unchecked(self.size, 8);
            std::alloc::dealloc(self.memory, layout);
        }
    }
}

// Safety: FreeListAllocator can be safely sent between threads
unsafe impl Send for FreeListAllocator {}
// Safety: FreeListAllocator can be safely shared between threads (it uses Mutex internally)
unsafe impl Sync for FreeListAllocator {}

#[cfg(test)]
mod tests {
    use super::*;
    use std::alloc::Layout;

    #[test]
    fn test_bump_allocator_creation() {
        let allocator = BumpAllocator::new(1024).unwrap();
        let (total_alloc, _, current_alloc, _, _) = allocator.stats();
        assert_eq!(total_alloc, 0);
        assert_eq!(current_alloc, 0);
    }

    #[test]
    fn test_bump_allocator_basic_allocation() {
        let allocator = BumpAllocator::new(1024).unwrap();
        
        unsafe {
            let layout = Layout::from_size_align(64, 8).unwrap();
            let ptr1 = allocator.alloc(layout);
            assert!(!ptr1.is_null());
            
            let ptr2 = allocator.alloc(layout);
            assert!(!ptr2.is_null());
            assert_ne!(ptr1, ptr2);
            
            let (total_alloc, _, current_alloc, alloc_count, _) = allocator.stats();
            assert_eq!(total_alloc, 128);
            assert_eq!(current_alloc, 128);
            assert_eq!(alloc_count, 2);
        }
    }

    #[test]
    fn test_bump_allocator_alignment() {
        let allocator = BumpAllocator::new(1024).unwrap();
        
        unsafe {
            let layout = Layout::from_size_align(1, 16).unwrap();
            let ptr = allocator.alloc(layout);
            assert!(!ptr.is_null());
            assert_eq!(ptr as usize % 16, 0);
        }
    }

    #[test]
    fn test_free_list_allocator_creation() {
        let allocator = FreeListAllocator::new(1024).unwrap();
        let (total_alloc, _, current_alloc, _, _) = allocator.stats();
        assert_eq!(total_alloc, 0);
        assert_eq!(current_alloc, 0);
    }

    #[test]
    fn test_free_list_allocator_alloc_dealloc() {
        let allocator = FreeListAllocator::new(1024).unwrap();
        
        unsafe {
            let layout = Layout::from_size_align(64, 8).unwrap();
            let ptr = allocator.alloc(layout);
            assert!(!ptr.is_null());
            
            let (total_alloc, total_dealloc, current_alloc, _, _) = allocator.stats();
            assert_eq!(total_alloc, 64);
            assert_eq!(total_dealloc, 0);
            assert_eq!(current_alloc, 64);
            
            allocator.dealloc(ptr, layout);
            
            let (total_alloc, total_dealloc, current_alloc, _, _) = allocator.stats();
            assert_eq!(total_alloc, 64);
            assert_eq!(total_dealloc, 64);
            assert_eq!(current_alloc, 0);
        }
    }

    #[test]
    fn test_free_list_allocator_reuse() {
        let allocator = FreeListAllocator::new(1024).unwrap();
        
        unsafe {
            let layout = Layout::from_size_align(64, 8).unwrap();
            
            // Allocate and deallocate
            let ptr1 = allocator.alloc(layout);
            assert!(!ptr1.is_null());
            allocator.dealloc(ptr1, layout);
            
            // Allocate again - should reuse the same memory
            let ptr2 = allocator.alloc(layout);
            assert!(!ptr2.is_null());
            assert_eq!(ptr1, ptr2);
        }
    }
}