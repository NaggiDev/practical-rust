# Custom Runtime Project

## Overview

In this Expert Level project, you'll build a simple async runtime from scratch to understand how Rust's async ecosystem works under the hood. This project will teach you about futures, executors, wakers, and the polling mechanism that powers async Rust.

## Learning Objectives

By completing this project, you will:

- Understand how futures work at a low level
- Learn about the polling mechanism and wakers
- Implement a basic executor that can run async tasks
- Understand how async runtimes like Tokio work internally
- Practice working with `Pin`, `Context`, and `Waker`
- Learn about task scheduling and cooperative multitasking

## Prerequisites

Before starting this project, you should have completed:
- Module 1: Async Programming (futures, async/await basics)
- Understanding of Rust's ownership system and lifetimes
- Familiarity with `Pin` and `Unpin` traits
- Basic understanding of concurrency concepts

## Project Structure

```
custom-runtime/
├── README.md           # This file
├── src/
│   ├── main.rs         # Example usage of the runtime
│   ├── lib.rs          # Library entry point
│   ├── executor.rs     # The main executor implementation
│   ├── task.rs         # Task abstraction
│   ├── waker.rs        # Custom waker implementation
│   └── timer.rs        # Simple timer future for testing
├── tests/
│   └── integration_tests.rs  # Integration tests
├── Cargo.toml          # Project dependencies
└── CONCEPTS.md         # Detailed concept explanations
```

## Step-by-Step Implementation Guide

### Step 1: Project Setup and Basic Structure

First, let's set up the project structure and understand what we're building.

**Concepts Applied**: Project organization, module system

**Tasks**:
1. Review the `Cargo.toml` file and understand the minimal dependencies
2. Examine the module structure in `lib.rs`
3. Run the initial tests to see what functionality we need to implement

**Key Learning Points**:
- How async runtimes are structured
- The relationship between executors, tasks, and futures
- Why we need custom wakers

### Step 2: Implement the Task Abstraction

Create a `Task` struct that wraps futures and provides the interface our executor needs.

**Concepts Applied**: 
- `Pin` and heap allocation for futures
- Type erasure with `dyn Future`
- `Send` and `Sync` bounds for thread safety

**Tasks**:
1. Implement the `Task` struct in `src/task.rs`
2. Create methods for creating and polling tasks
3. Understand why we need `Pin<Box<dyn Future>>`

**Key Learning Points**:
- Why futures need to be pinned
- How to work with trait objects for futures
- The importance of `Send` bounds in async code

### Step 3: Create a Custom Waker

Implement a custom `Waker` that can notify our executor when a task is ready to make progress.

**Concepts Applied**:
- `Waker` and `RawWaker` APIs
- Atomic operations for thread-safe signaling
- `Arc` for shared ownership

**Tasks**:
1. Implement a `TaskWaker` in `src/waker.rs`
2. Create the `RawWaker` vtable functions
3. Integrate with atomic signaling for task readiness

**Key Learning Points**:
- How wakers enable cooperative multitasking
- The relationship between wakers and task scheduling
- Why atomic operations are needed for cross-thread signaling

### Step 4: Build the Executor

Create the main executor that can spawn and run async tasks to completion.

**Concepts Applied**:
- Task queues and scheduling
- Polling loop implementation
- Context creation and management

**Tasks**:
1. Implement the `Executor` struct in `src/executor.rs`
2. Create methods for spawning tasks
3. Implement the main polling loop
4. Handle task completion and cleanup

**Key Learning Points**:
- How executors schedule and run tasks
- The polling model of async Rust
- Cooperative vs preemptive multitasking

### Step 5: Create a Timer Future

Implement a simple timer future to test our runtime with actual async operations.

**Concepts Applied**:
- Custom `Future` implementation
- Working with `Poll::Ready` and `Poll::Pending`
- Integration with system time

**Tasks**:
1. Implement a `Timer` future in `src/timer.rs`
2. Create proper `Future` trait implementation
3. Test the timer with our custom runtime

**Key Learning Points**:
- How to implement custom futures
- The polling state machine pattern
- Integration between futures and wakers

### Step 6: Integration and Testing

Put all the pieces together and create comprehensive tests.

**Concepts Applied**:
- Integration testing for async code
- Runtime usage patterns
- Performance considerations

**Tasks**:
1. Complete the integration tests
2. Create example usage in `main.rs`
3. Test with multiple concurrent tasks
4. Benchmark against standard runtimes (optional)

**Key Learning Points**:
- How all components work together
- Common patterns in async runtime usage
- Performance characteristics of different approaches

## Extension Challenges

Once you've completed the basic implementation, try these advanced challenges:

1. **Multi-threaded Executor**: Extend your executor to use multiple threads with work stealing
2. **I/O Integration**: Add support for async I/O operations using `mio` or similar
3. **Task Priorities**: Implement a priority-based task scheduler
4. **Metrics and Monitoring**: Add instrumentation to track task execution times
5. **Compatibility Layer**: Create adapters to run your runtime with existing async code

## Testing Your Implementation

Run the tests to verify your implementation:

```bash
# Run unit tests
cargo test

# Run with output to see executor behavior
cargo test -- --nocapture

# Run the example
cargo run
```

## Resources and Further Reading

- [The Rust Async Book](https://rust-lang.github.io/async-book/)
- [Futures Explained in 200 Lines of Rust](https://cfsamson.github.io/books-futures-explained/)
- [Pin and suffering](https://fasterthanli.me/articles/pin-and-suffering)
- [Tokio's Architecture](https://tokio.rs/blog/2020-04-prelude)
- [The Waker API](https://doc.rust-lang.org/std/task/struct.Waker.html)

## Success Criteria

You've successfully completed this project when:

- [ ] All tests pass
- [ ] Your executor can run multiple concurrent tasks
- [ ] Timer futures work correctly with your runtime
- [ ] You can explain how polling, waking, and scheduling work
- [ ] Your code follows Rust best practices for async programming
- [ ] You understand the trade-offs in runtime design

This project represents advanced async programming concepts. Take your time to understand each component and how they interact. The goal is deep understanding of how async Rust works at the system level.