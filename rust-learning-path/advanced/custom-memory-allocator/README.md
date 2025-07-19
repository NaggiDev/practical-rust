# Custom Memory Allocator Project

## Project Overview

In this advanced Rust project, you'll implement a custom memory allocator to understand how Rust manages memory at the lowest level. This project will teach you about unsafe Rust, raw pointers, memory layout, and the allocator API.

## Learning Objectives

By completing this project, you will:

- Understand Rust's memory allocation system
- Learn to work with unsafe Rust code safely
- Implement raw pointer manipulation
- Understand memory alignment and layout
- Learn about the GlobalAlloc trait
- Practice writing comprehensive tests for unsafe code

## Prerequisites

Before starting this project, you should be comfortable with:

- Rust ownership and borrowing
- Basic concurrency concepts
- Understanding of memory management concepts
- Experience with unsafe Rust basics

## Project Structure

```
custom-memory-allocator/
├── README.md           # This file
├── src/
│   ├── main.rs         # Example usage and benchmarks
│   ├── lib.rs          # Library entry point
│   ├── allocator.rs    # Main allocator implementation
│   └── utils.rs        # Helper utilities
├── tests/
│   └── integration_tests.rs  # Integration tests
├── Cargo.toml          # Project configuration
└── CONCEPTS.md         # Detailed concept explanations
```

## Step-by-Step Implementation Guide

### Step 1: Understanding Memory Allocation Basics

**Concepts Applied**: Memory layout, alignment, unsafe Rust basics

**Objective**: Set up the project structure and understand the fundamentals of memory allocation.

**Tasks**:
1. Review the provided starter code in `src/lib.rs`
2. Understand the `GlobalAlloc` trait and its methods
3. Learn about memory alignment and layout requirements
4. Implement basic utility functions for memory calculations

**Key Learning Points**:
- Memory must be properly aligned for different data types
- Allocation and deallocation must be paired correctly
- Unsafe code requires careful reasoning about safety invariants

### Step 2: Implementing a Simple Bump Allocator

**Concepts Applied**: Raw pointers, unsafe blocks, memory layout

**Objective**: Implement a basic bump allocator that allocates memory sequentially.

**Tasks**:
1. Implement the `BumpAllocator` struct
2. Implement `GlobalAlloc` trait methods (`alloc` and `dealloc`)
3. Handle memory alignment correctly
4. Add safety documentation for unsafe code

**Key Learning Points**:
- Bump allocators are simple but don't support deallocation
- Proper alignment is crucial for memory safety
- Unsafe code must be documented with safety invariants

### Step 3: Adding Memory Tracking and Debugging

**Concepts Applied**: Atomic operations, debugging unsafe code

**Objective**: Add tracking capabilities to monitor allocator behavior.

**Tasks**:
1. Add allocation counters using atomic operations
2. Implement memory usage tracking
3. Add debug assertions for safety checks
4. Create helper functions for allocation statistics

**Key Learning Points**:
- Atomic operations are necessary for thread-safe counters
- Debug assertions help catch bugs in unsafe code
- Tracking allocations helps understand memory usage patterns

### Step 4: Implementing a Free List Allocator

**Concepts Applied**: Linked lists with raw pointers, memory reuse

**Objective**: Implement a more sophisticated allocator that can reuse freed memory.

**Tasks**:
1. Design a free list data structure using raw pointers
2. Implement allocation from the free list
3. Implement deallocation by adding blocks to the free list
4. Handle memory coalescing for adjacent free blocks

**Key Learning Points**:
- Free lists enable memory reuse
- Pointer arithmetic requires careful bounds checking
- Memory coalescing improves allocation efficiency

### Step 5: Thread Safety and Synchronization

**Concepts Applied**: Mutexes, thread safety in unsafe code

**Objective**: Make the allocator thread-safe for concurrent use.

**Tasks**:
1. Add synchronization primitives to protect allocator state
2. Implement thread-safe allocation and deallocation
3. Consider lock-free alternatives for performance
4. Test concurrent allocation scenarios

**Key Learning Points**:
- Allocators must be thread-safe in multi-threaded programs
- Locks can become bottlenecks in allocation-heavy code
- Lock-free data structures are complex but can improve performance

### Step 6: Testing and Validation

**Concepts Applied**: Testing unsafe code, property-based testing

**Objective**: Create comprehensive tests to validate allocator correctness.

**Tasks**:
1. Write unit tests for individual allocator methods
2. Create integration tests for realistic usage scenarios
3. Implement stress tests for concurrent allocation
4. Add property-based tests for allocation invariants

**Key Learning Points**:
- Testing unsafe code requires extra care
- Stress testing helps find race conditions
- Property-based testing can catch edge cases

## Extension Challenges

Once you've completed the basic implementation, try these advanced challenges:

1. **Memory Pool Allocator**: Implement an allocator that manages fixed-size memory pools
2. **Slab Allocator**: Create an allocator optimized for objects of specific sizes
3. **NUMA-Aware Allocator**: Implement an allocator that considers NUMA topology
4. **Custom Drop Implementation**: Add automatic cleanup for allocated objects
5. **Memory Debugging Tools**: Add features like leak detection and use-after-free detection

## Performance Considerations

- Minimize lock contention in multi-threaded scenarios
- Consider cache locality when designing data structures
- Profile your allocator against standard allocators
- Measure allocation and deallocation latency

## Safety Guidelines

When working with unsafe code:

1. Always document safety invariants
2. Minimize the scope of unsafe blocks
3. Use debug assertions to check invariants
4. Test thoroughly, especially edge cases
5. Consider using tools like Miri for additional validation

## Resources

- [The Rustonomicon - Implementing Vec](https://doc.rust-lang.org/nomicon/vec/vec.html)
- [GlobalAlloc Documentation](https://doc.rust-lang.org/std/alloc/trait.GlobalAlloc.html)
- [Layout Documentation](https://doc.rust-lang.org/std/alloc/struct.Layout.html)
- [Unsafe Rust Guidelines](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)

## Getting Started

1. Read through this README completely
2. Review the starter code in `src/lib.rs`
3. Read `CONCEPTS.md` for detailed explanations
4. Start with Step 1 and work through each step sequentially
5. Run tests frequently: `cargo test`
6. Use `cargo run` to see the allocator in action

Remember: This project involves unsafe code. Take your time to understand each concept thoroughly before moving to the next step.