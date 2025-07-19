# Async Programming Examples

This directory contains comprehensive examples demonstrating asynchronous programming concepts in Rust. These examples accompany the Expert Level Module 1: Async Programming.

## Overview

The examples are organized into modules, each focusing on specific aspects of async programming:

- **basic_futures.rs** - Custom Future implementations and basic future operations
- **async_await.rs** - Async/await syntax, error handling, and chaining
- **pinning.rs** - Pin, self-referential structs, and memory safety
- **runtimes.rs** - Different async runtimes and their configurations
- **task_scheduling.rs** - Task spawning, scheduling, and management
- **streams.rs** - Async streams, transformations, and processing
- **cancellation.rs** - Timeouts, cancellation tokens, and graceful shutdown
- **structured_concurrency.rs** - JoinSet, error handling, and resource management
- **performance.rs** - Performance patterns, optimization, and best practices

## Running the Examples

### Prerequisites

Make sure you have Rust installed with Cargo. The examples use several async crates:

- `tokio` - The main async runtime
- `tokio-stream` - Stream utilities
- `tokio-util` - Additional utilities including cancellation
- `futures` - Future combinators and utilities
- `reqwest` - HTTP client (for some examples)
- `async-std` and `smol` - Alternative runtimes (for comparison)

### Running All Examples

```bash
cd rust-learning-path/expert/module1/examples
cargo run
```

This will run all example modules in sequence, demonstrating each concept with output.

### Running Tests

The examples include comprehensive tests that validate the async concepts:

```bash
cargo test
```

### Running Individual Examples

You can also run specific parts by modifying the `main.rs` file to comment out sections you don't want to run.

## Example Categories

### 1. Basic Futures (`basic_futures.rs`)

Demonstrates:
- Custom Future trait implementations
- Manual polling and waking
- Future combinators (`join!`, `select!`)
- Basic async building blocks

Key concepts:
- `Poll::Ready` vs `Poll::Pending`
- Waker mechanism
- Future composition

### 2. Async/Await (`async_await.rs`)

Demonstrates:
- Async function syntax
- Error handling with `?` operator
- Async blocks and closures
- Chaining async operations

Key concepts:
- Async function desugaring
- Error propagation in async contexts
- Async move semantics

### 3. Pinning (`pinning.rs`)

Demonstrates:
- Self-referential structs
- Pin<T> and Unpin trait
- Safe and unsafe pinning operations
- Why pinning is necessary for async

Key concepts:
- Memory safety with self-references
- Pin projections
- Unpin vs !Unpin types

### 4. Runtimes (`runtimes.rs`)

Demonstrates:
- Tokio runtime configuration
- Multi-threaded vs single-threaded runtimes
- Runtime handles and spawning
- Blocking task execution
- Comparison of different runtimes

Key concepts:
- Runtime architecture
- Thread pool management
- Blocking vs non-blocking operations

### 5. Task Scheduling (`task_scheduling.rs`)

Demonstrates:
- Task spawning with `tokio::spawn`
- Task-local storage
- Task yielding and cooperation
- Task abortion and cleanup

Key concepts:
- Cooperative scheduling
- Task lifecycle management
- Local storage patterns

### 6. Streams (`streams.rs`)

Demonstrates:
- Stream trait implementation
- Stream transformations and combinators
- Async stream processing
- Custom stream creation
- Error handling in streams

Key concepts:
- Lazy evaluation
- Backpressure handling
- Stream composition

### 7. Cancellation (`cancellation.rs`)

Demonstrates:
- Timeout operations
- Cancellation tokens
- Hierarchical cancellation
- Graceful shutdown patterns
- Select-based cancellation

Key concepts:
- Cooperative cancellation
- Resource cleanup
- Shutdown coordination

### 8. Structured Concurrency (`structured_concurrency.rs`)

Demonstrates:
- JoinSet for managing multiple tasks
- Error handling across task groups
- Task abortion and cleanup
- Resource management patterns
- Hierarchical task organization

Key concepts:
- Task lifetime management
- Error aggregation
- Resource cleanup guarantees

### 9. Performance (`performance.rs`)

Demonstrates:
- Blocking vs non-blocking patterns
- Sequential vs concurrent execution
- Buffering and batching strategies
- Memory efficiency techniques
- Common performance pitfalls

Key concepts:
- Concurrency vs parallelism
- Resource pooling
- Performance monitoring

## Key Learning Points

### Understanding Futures

Futures in Rust are:
- **Lazy** - They don't do work until polled
- **Zero-cost** - No heap allocation required
- **Composable** - Can be combined in various ways
- **Safe** - Memory safety guaranteed through the type system

### Async/Await Benefits

- **Readability** - Async code looks like sync code
- **Efficiency** - No callback hell or complex state machines
- **Composability** - Easy to combine and chain operations
- **Error handling** - Standard Rust error handling works

### Runtime Considerations

- **Choose the right runtime** for your use case
- **Configure appropriately** - thread count, stack size, etc.
- **Understand blocking** - Use `spawn_blocking` for CPU work
- **Monitor performance** - Profile and measure your async code

### Best Practices

1. **Avoid blocking** the async runtime
2. **Use structured concurrency** for task management
3. **Handle cancellation** gracefully
4. **Minimize allocations** in hot paths
5. **Test thoroughly** - async code has unique failure modes

## Common Patterns

### Error Handling
```rust
async fn handle_errors() -> Result<(), Box<dyn std::error::Error>> {
    let data = fetch_data().await?;
    let processed = process_data(data).await?;
    save_data(processed).await?;
    Ok(())
}
```

### Concurrent Operations
```rust
let (result1, result2, result3) = tokio::join!(
    operation1(),
    operation2(),
    operation3()
);
```

### Graceful Shutdown
```rust
tokio::select! {
    _ = shutdown_signal.recv() => {
        // Cleanup and exit
        break;
    }
    result = work() => {
        // Process result
    }
}
```

### Stream Processing
```rust
let results: Vec<_> = stream
    .map(|item| process_async(item))
    .buffer_unordered(10)
    .collect()
    .await;
```

## Further Reading

- [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Futures Explained](https://cfsamson.github.io/books-futures-explained/)
- [Pin and Unpin](https://doc.rust-lang.org/std/pin/index.html)

## Exercises

After running these examples, try:

1. Implement your own custom Future
2. Create a stream that generates data from an external source
3. Build a simple async server using the patterns shown
4. Implement a graceful shutdown mechanism for a multi-task application
5. Optimize a sequential operation to run concurrently

These examples provide a solid foundation for understanding async programming in Rust and preparing for the more advanced projects in this module.