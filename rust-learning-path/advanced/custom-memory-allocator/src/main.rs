//! Custom Memory Allocator Demo
//! 
//! This binary demonstrates the usage of custom allocators and provides
//! benchmarks to compare their performance characteristics.

use custom_allocator::{BumpAllocator, FreeListAllocator};
use std::alloc::{GlobalAlloc, Layout};
use std::time::Instant;

fn main() {
    println!("Custom Memory Allocator Demo");
    println!("============================\n");

    // Demo bump allocator
    demo_bump_allocator();
    println!();

    // Demo free list allocator
    demo_free_list_allocator();
    println!();

    // Performance comparison
    performance_comparison();
}

fn demo_bump_allocator() {
    println!("Bump Allocator Demo:");
    println!("-------------------");

    let allocator = BumpAllocator::new(4096).expect("Failed to create bump allocator");
    
    unsafe {
        // Allocate some memory blocks
        let layout1 = Layout::from_size_align(64, 8).unwrap();
        let layout2 = Layout::from_size_align(128, 16).unwrap();
        let layout3 = Layout::from_size_align(32, 4).unwrap();

        println!("Allocating memory blocks...");
        
        let ptr1 = allocator.alloc(layout1);
        println!("Allocated 64 bytes at {:p}", ptr1);
        
        let ptr2 = allocator.alloc(layout2);
        println!("Allocated 128 bytes at {:p}", ptr2);
        
        let ptr3 = allocator.alloc(layout3);
        println!("Allocated 32 bytes at {:p}", ptr3);

        // Show statistics
        let (total_alloc, total_dealloc, current_alloc, alloc_count, dealloc_count) = allocator.stats();
        println!("\nBump Allocator Statistics:");
        println!("  Total allocated: {} bytes", total_alloc);
        println!("  Total deallocated: {} bytes", total_dealloc);
        println!("  Currently allocated: {} bytes", current_alloc);
        println!("  Allocation count: {}", alloc_count);
        println!("  Deallocation count: {}", dealloc_count);

        // Demonstrate that deallocation doesn't free memory in bump allocator
        allocator.dealloc(ptr1, layout1);
        allocator.dealloc(ptr2, layout2);
        allocator.dealloc(ptr3, layout3);

        let (total_alloc, total_dealloc, current_alloc, alloc_count, dealloc_count) = allocator.stats();
        println!("\nAfter deallocation:");
        println!("  Total allocated: {} bytes", total_alloc);
        println!("  Total deallocated: {} bytes", total_dealloc);
        println!("  Currently allocated: {} bytes", current_alloc);
        println!("  Allocation count: {}", alloc_count);
        println!("  Deallocation count: {}", dealloc_count);
        println!("  Note: Bump allocator doesn't actually free individual blocks");
    }
}

fn demo_free_list_allocator() {
    println!("Free List Allocator Demo:");
    println!("------------------------");

    let allocator = FreeListAllocator::new(4096).expect("Failed to create free list allocator");
    
    unsafe {
        let layout = Layout::from_size_align(64, 8).unwrap();

        println!("Allocating and deallocating memory blocks...");
        
        // Allocate some blocks
        let ptr1 = allocator.alloc(layout);
        let ptr2 = allocator.alloc(layout);
        let ptr3 = allocator.alloc(layout);
        
        println!("Allocated blocks at: {:p}, {:p}, {:p}", ptr1, ptr2, ptr3);

        let (total_alloc, total_dealloc, current_alloc, alloc_count, dealloc_count) = allocator.stats();
        println!("\nAfter allocation:");
        println!("  Total allocated: {} bytes", total_alloc);
        println!("  Currently allocated: {} bytes", current_alloc);
        println!("  Allocation count: {}", alloc_count);

        // Deallocate middle block
        allocator.dealloc(ptr2, layout);
        println!("\nDeallocated middle block");

        let (total_alloc, total_dealloc, current_alloc, alloc_count, dealloc_count) = allocator.stats();
        println!("  Total allocated: {} bytes", total_alloc);
        println!("  Total deallocated: {} bytes", total_dealloc);
        println!("  Currently allocated: {} bytes", current_alloc);
        println!("  Deallocation count: {}", dealloc_count);

        // Allocate again - should reuse the freed block
        let ptr4 = allocator.alloc(layout);
        println!("\nAllocated new block at: {:p}", ptr4);
        
        if ptr4 == ptr2 {
            println!("Success! Reused the previously freed block");
        } else {
            println!("Used a different block (this is also valid behavior)");
        }

        // Clean up remaining allocations
        allocator.dealloc(ptr1, layout);
        allocator.dealloc(ptr3, layout);
        allocator.dealloc(ptr4, layout);

        let (total_alloc, total_dealloc, current_alloc, alloc_count, dealloc_count) = allocator.stats();
        println!("\nFinal statistics:");
        println!("  Total allocated: {} bytes", total_alloc);
        println!("  Total deallocated: {} bytes", total_dealloc);
        println!("  Currently allocated: {} bytes", current_alloc);
        println!("  Allocation count: {}", alloc_count);
        println!("  Deallocation count: {}", dealloc_count);
    }
}

fn performance_comparison() {
    println!("Performance Comparison:");
    println!("----------------------");

    const NUM_ALLOCATIONS: usize = 10000;
    const ALLOCATION_SIZE: usize = 64;

    // Benchmark bump allocator
    let bump_allocator = BumpAllocator::new(NUM_ALLOCATIONS * ALLOCATION_SIZE * 2)
        .expect("Failed to create bump allocator");
    
    let start = Instant::now();
    unsafe {
        let layout = Layout::from_size_align(ALLOCATION_SIZE, 8).unwrap();
        let mut ptrs = Vec::with_capacity(NUM_ALLOCATIONS);
        
        for _ in 0..NUM_ALLOCATIONS {
            let ptr = bump_allocator.alloc(layout);
            ptrs.push(ptr);
        }
        
        for ptr in ptrs {
            bump_allocator.dealloc(ptr, layout);
        }
    }
    let bump_duration = start.elapsed();

    // Benchmark free list allocator
    let free_list_allocator = FreeListAllocator::new(NUM_ALLOCATIONS * ALLOCATION_SIZE * 2)
        .expect("Failed to create free list allocator");
    
    let start = Instant::now();
    unsafe {
        let layout = Layout::from_size_align(ALLOCATION_SIZE, 8).unwrap();
        let mut ptrs = Vec::with_capacity(NUM_ALLOCATIONS);
        
        for _ in 0..NUM_ALLOCATIONS {
            let ptr = free_list_allocator.alloc(layout);
            ptrs.push(ptr);
        }
        
        for ptr in ptrs {
            free_list_allocator.dealloc(ptr, layout);
        }
    }
    let free_list_duration = start.elapsed();

    // Benchmark system allocator
    let start = Instant::now();
    unsafe {
        let layout = Layout::from_size_align(ALLOCATION_SIZE, 8).unwrap();
        let mut ptrs = Vec::with_capacity(NUM_ALLOCATIONS);
        
        for _ in 0..NUM_ALLOCATIONS {
            let ptr = std::alloc::alloc(layout);
            ptrs.push(ptr);
        }
        
        for ptr in ptrs {
            std::alloc::dealloc(ptr, layout);
        }
    }
    let system_duration = start.elapsed();

    println!("Allocating and deallocating {} blocks of {} bytes each:", NUM_ALLOCATIONS, ALLOCATION_SIZE);
    println!("  Bump allocator:      {:?}", bump_duration);
    println!("  Free list allocator: {:?}", free_list_duration);
    println!("  System allocator:    {:?}", system_duration);
    
    println!("\nRelative performance (lower is better):");
    println!("  Bump allocator:      {:.2}x", bump_duration.as_nanos() as f64 / system_duration.as_nanos() as f64);
    println!("  Free list allocator: {:.2}x", free_list_duration.as_nanos() as f64 / system_duration.as_nanos() as f64);
    println!("  System allocator:    1.00x (baseline)");

    println!("\nNote: These benchmarks are simplified and may not reflect real-world performance.");
    println!("Actual performance depends on allocation patterns, memory fragmentation, and system load.");
}