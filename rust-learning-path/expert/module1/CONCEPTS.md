# Async Programming Concepts

This document provides detailed explanations of asynchronous programming concepts in Rust, with practical examples and best practices.

## Table of Contents

1. [Futures and the Future Trait](#futures-and-the-future-trait)
2. [Async/Await Syntax](#asyncawait-syntax)
3. [Pinning and Self-Referential Structs](#pinning-and-self-referential-structs)
4. [Async Runtimes](#async-runtimes)
5. [Task Scheduling and Execution](#task-scheduling-and-execution)
6. [Async Streams and Iterators](#async-streams-and-iterators)
7. [Cancellation and Timeouts](#cancellation-and-timeouts)
8. [Structured Concurrency](#structured-concurrency)
9. [Performance Considerations](#performance-considerations)

## Futures and the Future Trait

A Future in Rust represents a value that will be available at some point in the future. Unlike promises in other languages, Rust futures are lazy - they don't do any work until they're polled.

### The Future Trait

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

trait Future {
    type Output;
    
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

### Basic Future Implementation

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

struct DelayFuture {
    when: Instant,
}

impl DelayFuture {
    fn new(duration: Duration) -> Self {
        DelayFuture {
            when: Instant::now() + duration,
        }
    }
}

impl Future for DelayFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            Poll::Ready(())
        } else {
            // Wake up the task when the delay is over
            let waker = cx.waker().clone();
            let when = self.when;
            std::thread::spawn(move || {
                std::thread::sleep(when - Instant::now());
                waker.wake();
            });
            Poll::Pending
        }
    }
}

// Usage
async fn example() {
    DelayFuture::new(Duration::from_secs(1)).await;
    println!("One second has passed!");
}
```

### Future Combinators

```rust
use std::future::Future;

// Combining futures
async fn combine_futures() {
    let future1 = async { 42 };
    let future2 = async { "hello" };
    
    // Run futures concurrently
    let (result1, result2) = tokio::join!(future1, future2);
    println!("Results: {} and {}", result1, result2);
    
    // Select the first to complete
    let result = tokio::select! {
        val = async { 1 } => val,
        val = async { 2 } => val,
    };
    println!("First result: {}", result);
}
```

## Async/Await Syntax

The async/await syntax provides a more readable way to work with futures, making asynchronous code look similar to synchronous code.

### Async Functions

```rust
// Async function that returns a Future
async fn fetch_data() -> Result<String, Box<dyn std::error::Error>> {
    let response = reqwest::get("https://api.example.com/data").await?;
    let text = response.text().await?;
    Ok(text)
}

// Async block
let future = async {
    let data = fetch_data().await?;
    process_data(data)
};
```

### Error Handling in Async Code

```rust
use std::error::Error;

async fn handle_errors() -> Result<(), Box<dyn Error>> {
    // Using ? operator with async operations
    let data = fetch_data().await?;
    
    // Pattern matching with async results
    match try_operation().await {
        Ok(result) => println!("Success: {}", result),
        Err(e) => eprintln!("Error: {}", e),
    }
    
    Ok(())
}

async fn try_operation() -> Result<i32, &'static str> {
    // Simulate an async operation that might fail
    tokio::time::sleep(Duration::from_millis(100)).await;
    Ok(42)
}
```

## Pinning and Self-Referential Structs

Pinning is crucial for async Rust because futures often contain self-references that must not be moved in memory.

### Understanding Pin

```rust
use std::pin::Pin;
use std::marker::PhantomPinned;

struct SelfReferential {
    data: String,
    pointer: *const String,
    _pin: PhantomPinned,
}

impl SelfReferential {
    fn new(data: String) -> Pin<Box<Self>> {
        let mut boxed = Box::pin(SelfReferential {
            data,
            pointer: std::ptr::null(),
            _pin: PhantomPinned,
        });
        
        // Safe because we're pinning the data
        let ptr = &boxed.data as *const String;
        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).pointer = ptr;
        }
        
        boxed
    }
    
    fn get_data(&self) -> &str {
        &self.data
    }
    
    fn get_pointer_data(&self) -> &str {
        unsafe { &*self.pointer }
    }
}
```

### Pin in Async Context

```rust
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

struct AsyncStruct {
    state: String,
    future: Pin<Box<dyn Future<Output = ()>>>,
}

impl Future for AsyncStruct {
    type Output = String;
    
    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Poll the inner future
        match self.future.as_mut().poll(cx) {
            Poll::Ready(()) => Poll::Ready(self.state.clone()),
            Poll::Pending => Poll::Pending,
        }
    }
}
```

## Async Runtimes

Async runtimes provide the infrastructure to execute futures. Different runtimes have different characteristics and use cases.

### Tokio Runtime

```rust
use tokio::runtime::Runtime;

// Creating a runtime manually
fn manual_runtime() {
    let rt = Runtime::new().unwrap();
    
    rt.block_on(async {
        println!("Running on Tokio runtime");
        tokio::time::sleep(Duration::from_millis(100)).await;
    });
}

// Using the tokio macro
#[tokio::main]
async fn main() {
    println!("Running with tokio::main");
    
    // Spawn tasks on the runtime
    let handle = tokio::spawn(async {
        tokio::time::sleep(Duration::from_secs(1)).await;
        "Task completed"
    });
    
    let result = handle.await.unwrap();
    println!("{}", result);
}

// Multi-threaded runtime configuration
fn custom_runtime() {
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .thread_name("my-async-worker")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap();
    
    rt.block_on(async {
        // Your async code here
    });
}
```

### Comparing Runtimes

```rust
// Tokio - Full-featured, production-ready
#[tokio::main]
async fn tokio_example() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    // Handle connections...
}

// async-std - std-like API
#[async_std::main]
async fn async_std_example() {
    let listener = async_std::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
    // Handle connections...
}

// smol - Lightweight runtime
fn smol_example() {
    smol::block_on(async {
        let listener = smol::net::TcpListener::bind("127.0.0.1:8080").await.unwrap();
        // Handle connections...
    });
}
```

## Task Scheduling and Execution

Understanding how tasks are scheduled and executed is crucial for writing efficient async code.

### Task Spawning

```rust
use tokio::task;

async fn task_spawning_examples() {
    // Spawn a task on the current runtime
    let handle = task::spawn(async {
        expensive_computation().await
    });
    
    // Spawn a blocking task
    let blocking_handle = task::spawn_blocking(|| {
        // CPU-intensive work that would block the async runtime
        std::thread::sleep(Duration::from_secs(1));
        42
    });
    
    // Wait for both tasks
    let (async_result, blocking_result) = tokio::join!(handle, blocking_handle);
    
    println!("Async result: {:?}", async_result);
    println!("Blocking result: {:?}", blocking_result);
}

async fn expensive_computation() -> i32 {
    // Simulate async work
    tokio::time::sleep(Duration::from_millis(500)).await;
    100
}
```

### Task Local Storage

```rust
use tokio::task_local;

task_local! {
    static REQUEST_ID: u64;
}

async fn handle_request(id: u64) {
    REQUEST_ID.scope(id, async {
        process_request().await;
    }).await;
}

async fn process_request() {
    REQUEST_ID.with(|id| {
        println!("Processing request {}", id);
    });
    
    // Call other async functions that can access REQUEST_ID
    log_operation().await;
}

async fn log_operation() {
    REQUEST_ID.with(|id| {
        println!("Logging operation for request {}", id);
    });
}
```

## Async Streams and Iterators

Streams are the async equivalent of iterators, allowing you to process sequences of data asynchronously.

### Basic Stream Usage

```rust
use futures::stream::{self, StreamExt};
use tokio_stream::wrappers::IntervalStream;

async fn stream_examples() {
    // Create a stream from an iterator
    let stream = stream::iter(vec![1, 2, 3, 4, 5]);
    
    // Process each item asynchronously
    stream
        .for_each(|item| async move {
            println!("Processing item: {}", item);
            tokio::time::sleep(Duration::from_millis(100)).await;
        })
        .await;
    
    // Transform stream items
    let doubled: Vec<i32> = stream::iter(vec![1, 2, 3, 4, 5])
        .map(|x| x * 2)
        .collect()
        .await;
    
    println!("Doubled: {:?}", doubled);
}

// Creating a custom stream
use std::pin::Pin;
use std::task::{Context, Poll};
use futures::Stream;

struct CounterStream {
    current: usize,
    max: usize,
}

impl CounterStream {
    fn new(max: usize) -> Self {
        CounterStream { current: 0, max }
    }
}

impl Stream for CounterStream {
    type Item = usize;
    
    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Poll::Ready(Some(current))
        } else {
            Poll::Ready(None)
        }
    }
}
```

### Interval Streams

```rust
use tokio::time::{interval, Duration};
use tokio_stream::wrappers::IntervalStream;

async fn interval_example() {
    let mut interval_stream = IntervalStream::new(interval(Duration::from_secs(1)));
    
    // Take only 5 ticks
    let mut count = 0;
    while let Some(_tick) = interval_stream.next().await {
        println!("Tick {}", count);
        count += 1;
        if count >= 5 {
            break;
        }
    }
}
```

## Cancellation and Timeouts

Proper cancellation and timeout handling is essential for robust async applications.

### Timeout Handling

```rust
use tokio::time::{timeout, Duration};

async fn timeout_examples() {
    // Simple timeout
    match timeout(Duration::from_secs(1), slow_operation()).await {
        Ok(result) => println!("Operation completed: {:?}", result),
        Err(_) => println!("Operation timed out"),
    }
    
    // Timeout with error handling
    let result = timeout(Duration::from_secs(2), fallible_operation()).await;
    match result {
        Ok(Ok(value)) => println!("Success: {}", value),
        Ok(Err(e)) => println!("Operation failed: {}", e),
        Err(_) => println!("Operation timed out"),
    }
}

async fn slow_operation() -> String {
    tokio::time::sleep(Duration::from_secs(2)).await;
    "Completed".to_string()
}

async fn fallible_operation() -> Result<i32, &'static str> {
    tokio::time::sleep(Duration::from_millis(500)).await;
    Ok(42)
}
```

### Cancellation Tokens

```rust
use tokio_util::sync::CancellationToken;

async fn cancellation_example() {
    let token = CancellationToken::new();
    let child_token = token.child_token();
    
    // Spawn a task that can be cancelled
    let task = tokio::spawn(async move {
        loop {
            tokio::select! {
                _ = child_token.cancelled() => {
                    println!("Task was cancelled");
                    break;
                }
                _ = tokio::time::sleep(Duration::from_millis(100)) => {
                    println!("Task is running...");
                }
            }
        }
    });
    
    // Cancel the task after 1 second
    tokio::time::sleep(Duration::from_secs(1)).await;
    token.cancel();
    
    // Wait for the task to complete
    task.await.unwrap();
}
```

## Structured Concurrency

Structured concurrency ensures that spawned tasks are properly managed and cleaned up.

### Task Groups

```rust
use tokio::task::JoinSet;

async fn structured_concurrency_example() {
    let mut set = JoinSet::new();
    
    // Spawn multiple tasks
    for i in 0..5 {
        set.spawn(async move {
            tokio::time::sleep(Duration::from_millis(i * 100)).await;
            format!("Task {} completed", i)
        });
    }
    
    // Wait for all tasks to complete
    while let Some(result) = set.join_next().await {
        match result {
            Ok(message) => println!("{}", message),
            Err(e) => eprintln!("Task failed: {}", e),
        }
    }
}

// Graceful shutdown pattern
async fn graceful_shutdown_example() {
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::mpsc::channel(1);
    
    // Spawn worker tasks
    let mut tasks = Vec::new();
    for i in 0..3 {
        let mut shutdown_rx = shutdown_rx.clone();
        let task = tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = shutdown_rx.recv() => {
                        println!("Worker {} shutting down", i);
                        break;
                    }
                    _ = tokio::time::sleep(Duration::from_secs(1)) => {
                        println!("Worker {} is working", i);
                    }
                }
            }
        });
        tasks.push(task);
    }
    
    // Simulate shutdown signal after 5 seconds
    tokio::time::sleep(Duration::from_secs(5)).await;
    drop(shutdown_tx); // This closes the channel
    
    // Wait for all workers to shut down
    for task in tasks {
        task.await.unwrap();
    }
    
    println!("All workers have shut down gracefully");
}
```

## Performance Considerations

Writing efficient async code requires understanding the performance characteristics of different patterns.

### Avoiding Common Pitfalls

```rust
// BAD: Blocking the async runtime
async fn bad_blocking() {
    std::thread::sleep(Duration::from_secs(1)); // This blocks the entire runtime!
}

// GOOD: Using async sleep
async fn good_async_sleep() {
    tokio::time::sleep(Duration::from_secs(1)).await;
}

// BAD: Sequential async operations when they could be concurrent
async fn bad_sequential() {
    let result1 = fetch_data("url1").await;
    let result2 = fetch_data("url2").await; // Waits for result1 to complete
    (result1, result2)
}

// GOOD: Concurrent async operations
async fn good_concurrent() {
    let future1 = fetch_data("url1");
    let future2 = fetch_data("url2");
    tokio::join!(future1, future2) // Both run concurrently
}

async fn fetch_data(url: &str) -> String {
    // Simulate network request
    tokio::time::sleep(Duration::from_millis(100)).await;
    format!("Data from {}", url)
}
```

### Buffering and Batching

```rust
use futures::stream::{self, StreamExt};

async fn buffering_example() {
    let stream = stream::iter(0..1000);
    
    // Process items in batches
    let results: Vec<Vec<i32>> = stream
        .chunks(10) // Process 10 items at a time
        .map(|chunk| async move {
            // Process the chunk asynchronously
            tokio::time::sleep(Duration::from_millis(10)).await;
            chunk.into_iter().map(|x| x * 2).collect()
        })
        .buffer_unordered(5) // Allow up to 5 concurrent chunk processors
        .collect()
        .await;
    
    println!("Processed {} batches", results.len());
}
```

### Memory Management

```rust
// Avoiding unnecessary allocations in hot paths
async fn efficient_string_processing(input: &str) -> String {
    // Use string slices and avoid unnecessary clones
    let processed = input
        .lines()
        .filter(|line| !line.is_empty())
        .collect::<Vec<_>>()
        .join("\n");
    
    processed
}

// Using Arc for shared data in concurrent contexts
use std::sync::Arc;

async fn shared_data_example() {
    let shared_data = Arc::new(vec![1, 2, 3, 4, 5]);
    
    let mut tasks = Vec::new();
    for i in 0..3 {
        let data = Arc::clone(&shared_data);
        let task = tokio::spawn(async move {
            // Each task can read the shared data without cloning
            let sum: i32 = data.iter().sum();
            println!("Task {}: sum = {}", i, sum);
        });
        tasks.push(task);
    }
    
    // Wait for all tasks
    for task in tasks {
        task.await.unwrap();
    }
}
```

## Testing Async Code

Testing asynchronous code requires special considerations and tools.

### Basic Async Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;
    use tokio::test;
    
    #[tokio::test]
    async fn test_async_function() {
        let result = fetch_data("test").await;
        assert_eq!(result, "Data from test");
    }
    
    #[tokio::test]
    async fn test_timeout() {
        let result = timeout(Duration::from_millis(50), slow_operation()).await;
        assert!(result.is_err()); // Should timeout
    }
    
    #[tokio::test]
    async fn test_concurrent_operations() {
        let start = std::time::Instant::now();
        let (result1, result2) = tokio::join!(
            fetch_data("url1"),
            fetch_data("url2")
        );
        let elapsed = start.elapsed();
        
        // Both operations should complete in roughly the same time as one
        assert!(elapsed < Duration::from_millis(150));
        assert_eq!(result1, "Data from url1");
        assert_eq!(result2, "Data from url2");
    }
}
```

This comprehensive guide covers the essential concepts of async programming in Rust. Each concept builds upon the previous ones, providing a solid foundation for understanding and implementing asynchronous systems in Rust.