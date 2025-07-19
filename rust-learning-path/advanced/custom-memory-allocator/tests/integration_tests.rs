//! Integration tests for custom memory allocators
//! 
//! These tests validate the behavior of the allocators in various scenarios,
//! including edge cases and stress testing.

use custom_allocator::{BumpAllocator, FreeListAllocator};
use std::alloc::{GlobalAlloc, Layout};
use std::thread;
use std::sync::Arc;

#[test]
fn test_bump_allocator_out_of_memory() {
    let allocator = BumpAllocator::new(100).unwrap();
    
    unsafe {
        let layout = Layout::from_size_align(64, 8).unwrap();
        
        // First allocation should succeed
        let ptr1 = allocator.alloc(layout);
        assert!(!ptr1.is_null());
        
        // Second allocation should fail (not enough space)
        let ptr2 = allocator.alloc(layout);
        assert!(ptr2.is_null());
    }
}

#[test]
fn test_bump_allocator_alignment_edge_cases() {
    let allocator = BumpAllocator::new(1024).unwrap();
    
    unsafe {
        // Test various alignment requirements
        let alignments = [1, 2, 4, 8, 16, 32, 64];
        
        for &align in &alignments {
            let layout = Layout::from_size_align(1, align).unwrap();
            let ptr = allocator.alloc(layout);
            assert!(!ptr.is_null(), "Failed to allocate with alignment {}", align);
            assert_eq!(ptr as usize % align, 0, "Pointer not aligned to {} bytes", align);
        }
    }
}

#[test]
fn test_free_list_allocator_fragmentation() {
    let allocator = FreeListAllocator::new(1024).unwrap();
    
    unsafe {
        let layout = Layout::from_size_align(64, 8).unwrap();
        let mut ptrs = Vec::new();
        
        // Allocate several blocks
        for _ in 0..10 {
            let ptr = allocator.alloc(layout);
            assert!(!ptr.is_null());
            ptrs.push(ptr);
        }
        
        // Deallocate every other block to create fragmentation
        for i in (0..ptrs.len()).step_by(2) {
            allocator.dealloc(ptrs[i], layout);
        }
        
        // Try to allocate again - should reuse freed blocks
        for _ in 0..5 {
            let ptr = allocator.alloc(layout);
            assert!(!ptr.is_null());
        }
        
        // Clean up remaining allocations
        for i in (1..ptrs.len()).step_by(2) {
            allocator.dealloc(ptrs[i], layout);
        }
    }
}

#[test]
fn test_free_list_allocator_different_sizes() {
    let allocator = FreeListAllocator::new(2048).unwrap();
    
    unsafe {
        // Allocate blocks of different sizes
        let small_layout = Layout::from_size_align(32, 8).unwrap();
        let medium_layout = Layout::from_size_align(128, 8).unwrap();
        let large_layout = Layout::from_size_align(512, 8).unwrap();
        
        let small_ptr = allocator.alloc(small_layout);
        let medium_ptr = allocator.alloc(medium_layout);
        let large_ptr = allocator.alloc(large_layout);
        
        assert!(!small_ptr.is_null());
        assert!(!medium_ptr.is_null());
        assert!(!large_ptr.is_null());
        
        // Deallocate in different order
        allocator.dealloc(medium_ptr, medium_layout);
        allocator.dealloc(small_ptr, small_layout);
        allocator.dealloc(large_ptr, large_layout);
        
        // Allocate again with different sizes
        let new_medium_ptr = allocator.alloc(medium_layout);
        assert!(!new_medium_ptr.is_null());
        
        allocator.dealloc(new_medium_ptr, medium_layout);
    }
}

#[test]
fn test_allocator_thread_safety() {
    let allocator = Arc::new(FreeListAllocator::new(4096).unwrap());
    let mut handles = Vec::new();
    
    // Spawn multiple threads that allocate and deallocate memory
    for thread_id in 0..4 {
        let allocator_clone = Arc::clone(&allocator);
        let handle = thread::spawn(move || {
            unsafe {
                let layout = Layout::from_size_align(64, 8).unwrap();
                let mut ptrs = Vec::new();
                
                // Allocate some memory
                for _ in 0..100 {
                    let ptr = allocator_clone.alloc(layout);
                    if !ptr.is_null() {
                        ptrs.push(ptr);
                    }
                }
                
                // Write to allocated memory to ensure it's valid
                for &ptr in &ptrs {
                    if !ptr.is_null() {
                        std::ptr::write(ptr, thread_id as u8);
                        let value = std::ptr::read(ptr);
                        assert_eq!(value, thread_id as u8);
                    }
                }
                
                // Deallocate memory
                for ptr in ptrs {
                    if !ptr.is_null() {
                        allocator_clone.dealloc(ptr, layout);
                    }
                }
            }
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Check final statistics
    let (total_alloc, total_dealloc, current_alloc, _, _) = allocator.stats();
    assert_eq!(current_alloc, 0, "Memory leak detected: {} bytes still allocated", current_alloc);
    assert_eq!(total_alloc, total_dealloc, "Allocation/deallocation mismatch");
}

#[test]
fn test_zero_size_allocation() {
    let allocator = BumpAllocator::new(1024).unwrap();
    
    unsafe {
        let layout = Layout::from_size_align(0, 1).unwrap();
        let ptr = allocator.alloc(layout);
        
        // Zero-size allocations should return a non-null, well-aligned pointer
        assert!(!ptr.is_null());
        
        // Deallocating should not crash
        allocator.dealloc(ptr, layout);
    }
}

#[test]
fn test_large_alignment() {
    let allocator = FreeListAllocator::new(4096).unwrap();
    
    unsafe {
        // Test allocation with large alignment requirement
        let layout = Layout::from_size_align(64, 256).unwrap();
        let ptr = allocator.alloc(layout);
        
        assert!(!ptr.is_null());
        assert_eq!(ptr as usize % 256, 0, "Pointer not aligned to 256 bytes");
        
        allocator.dealloc(ptr, layout);
    }
}

#[test]
fn test_allocator_statistics_accuracy() {
    let allocator = FreeListAllocator::new(2048).unwrap();
    
    unsafe {
        let layout1 = Layout::from_size_align(64, 8).unwrap();
        let layout2 = Layout::from_size_align(128, 8).unwrap();
        
        // Initial state
        let (total_alloc, total_dealloc, current_alloc, alloc_count, dealloc_count) = allocator.stats();
        assert_eq!(total_alloc, 0);
        assert_eq!(total_dealloc, 0);
        assert_eq!(current_alloc, 0);
        assert_eq!(alloc_count, 0);
        assert_eq!(dealloc_count, 0);
        
        // Allocate first block
        let ptr1 = allocator.alloc(layout1);
        assert!(!ptr1.is_null());
        
        let (total_alloc, total_dealloc, current_alloc, alloc_count, dealloc_count) = allocator.stats();
        assert_eq!(total_alloc, 64);
        assert_eq!(total_dealloc, 0);
        assert_eq!(current_alloc, 64);
        assert_eq!(alloc_count, 1);
        assert_eq!(dealloc_count, 0);
        
        // Allocate second block
        let ptr2 = allocator.alloc(layout2);
        assert!(!ptr2.is_null());
        
        let (total_alloc, total_dealloc, current_alloc, alloc_count, dealloc_count) = allocator.stats();
        assert_eq!(total_alloc, 192); // 64 + 128
        assert_eq!(total_dealloc, 0);
        assert_eq!(current_alloc, 192);
        assert_eq!(alloc_count, 2);
        assert_eq!(dealloc_count, 0);
        
        // Deallocate first block
        allocator.dealloc(ptr1, layout1);
        
        let (total_alloc, total_dealloc, current_alloc, alloc_count, dealloc_count) = allocator.stats();
        assert_eq!(total_alloc, 192);
        assert_eq!(total_dealloc, 64);
        assert_eq!(current_alloc, 128);
        assert_eq!(alloc_count, 2);
        assert_eq!(dealloc_count, 1);
        
        // Deallocate second block
        allocator.dealloc(ptr2, layout2);
        
        let (total_alloc, total_dealloc, current_alloc, alloc_count, dealloc_count) = allocator.stats();
        assert_eq!(total_alloc, 192);
        assert_eq!(total_dealloc, 192);
        assert_eq!(current_alloc, 0);
        assert_eq!(alloc_count, 2);
        assert_eq!(dealloc_count, 2);
    }
}

// Property-based tests using proptest
#[cfg(test)]
mod property_tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn test_bump_allocator_alignment_property(
            size in 1usize..=256,
            align_power in 0u32..=6u32
        ) {
            let align = 1usize << align_power;
            if let Ok(layout) = Layout::from_size_align(size, align) {
                let allocator = BumpAllocator::new(4096).unwrap();
                unsafe {
                    let ptr = allocator.alloc(layout);
                    if !ptr.is_null() {
                        prop_assert_eq!(ptr as usize % align, 0);
                    }
                }
            }
        }

        #[test]
        fn test_free_list_allocator_alloc_dealloc_property(
            sizes in prop::collection::vec(1usize..=128, 1..=20)
        ) {
            let allocator = FreeListAllocator::new(8192).unwrap();
            let mut ptrs = Vec::new();
            
            unsafe {
                // Allocate all blocks
                for &size in &sizes {
                    if let Ok(layout) = Layout::from_size_align(size, 8) {
                        let ptr = allocator.alloc(layout);
                        if !ptr.is_null() {
                            ptrs.push((ptr, layout));
                        }
                    }
                }
                
                // Deallocate all blocks
                for (ptr, layout) in ptrs {
                    allocator.dealloc(ptr, layout);
                }
                
                // Check that current allocation is zero
                let (_, _, current_alloc, _, _) = allocator.stats();
                prop_assert_eq!(current_alloc, 0);
            }
        }
    }
}