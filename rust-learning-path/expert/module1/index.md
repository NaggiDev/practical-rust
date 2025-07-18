# Module 1: Async Programming

Welcome to Module 1 of the Expert Level! This module focuses on asynchronous programming in Rust, exploring futures, async/await syntax, and asynchronous runtimes.

## Learning Objectives

By completing this module, you will:

- Understand Rust's async/await system and how it works
- Learn about futures and how they're composed
- Master asynchronous programming patterns
- Work with tokio and other async runtimes
- Implement efficient non-blocking I/O
- Debug and profile async code

## Concepts Covered

- Futures and the Future trait
- The async/await syntax
- Pinning and self-referential structs
- Async runtimes (tokio, async-std, smol)
- Task scheduling and execution
- Async streams and iterators
- Cancellation and timeouts
- Structured concurrency
- Async performance considerations

## Projects

### [Asynchronous Network Server](async-server/README.md)

Build a high-performance asynchronous network server that can handle thousands of concurrent connections efficiently.

**Skills practiced:**
- Implementing async/await for non-blocking operations
- Working with tokio for async I/O
- Managing many concurrent connections
- Handling backpressure and load balancing
- Implementing graceful shutdown
- Testing asynchronous code
- Measuring and optimizing async performance

## Resources

- [Asynchronous Programming in Rust](https://rust-lang.github.io/async-book/)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Futures Explained in 200 Lines of Rust](https://cfsamson.github.io/books-futures-explained/)
- [std::future Documentation](https://doc.rust-lang.org/std/future/index.html)

## Next Steps

After completing this module, proceed to [Module 2: Custom Runtimes](../module2/index.md) to learn how to build your own async runtime from scratch.