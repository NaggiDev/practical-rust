# Module 1: Concurrency

Welcome to Module 1 of the Advanced Level! This module focuses on concurrent programming in Rust, exploring threads, synchronization primitives, and atomic operations.

## Learning Objectives

By completing this module, you will:

- Understand Rust's concurrency model and guarantees
- Master thread creation and management
- Learn various synchronization techniques
- Work with atomic operations for lock-free concurrency
- Implement thread-safe data structures
- Detect and prevent race conditions and deadlocks

## Concepts Covered

- Thread creation and joining
- Message passing between threads
- Shared state concurrency
- Mutex and RwLock for synchronization
- Atomic types and operations
- Thread safety and Send/Sync traits
- Deadlock prevention
- Thread pools and work stealing

## Projects

### [Thread Pool Implementation](thread-pool/README.md)

Build a custom thread pool that efficiently manages a collection of worker threads for executing tasks concurrently.

**Skills practiced:**
- Creating and managing multiple threads
- Implementing channels for communication
- Designing thread-safe data structures
- Handling task submission and execution
- Implementing graceful shutdown
- Testing concurrent code
- Measuring and optimizing performance

## Resources

- [The Rust Book - Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Rust Atomics and Locks](https://marabos.nl/atomics/)
- [Rust By Example - Concurrency](https://doc.rust-lang.org/rust-by-example/std_misc/threads.html)
- [std::sync Documentation](https://doc.rust-lang.org/std/sync/index.html)

## Next Steps

After completing this module, proceed to [Module 2: Unsafe Rust](../module2/index.md) to learn when and how to use unsafe Rust safely.