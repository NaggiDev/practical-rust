# Rust Async Runtime Concepts

This document explains the key concepts demonstrated in the Custom Runtime project. Understanding these concepts is crucial for working with async Rust at an advanced level.

## Table of Contents

1. [Futures and the Future Trait](#futures-and-the-future-trait)
2. [Polling and State Machines](#polling-and-state-machines)
3. [Pin and Unpin](#pin-and-unpin)
4. [Wakers and Context](#wakers-and-context)
5. [Executors and Task Scheduling](#executors-and-task-scheduling)
6. [Cooperative vs Preemptive Multitasking](#cooperative-vs-preemptive-multitasking)
7. [Memory Management in Async Code](#memory-management-in-async-code)
8. [Performance Considerations](#performance-considerations)

## Futures and the Future Trait

### What is a Future?

A `Future` in Rust represents a computation that will complete at some point in the future. Unlike promises in other languages, Rust futures are "lazy" - they don't do any work until they're polled.

```rust
pub trait Future {
    type Output;
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>;
}
```

### Key Characteristics

- **Lazy**: Futures don't execute until polled
- **Zero-cost**: When compiled, futures become state machines with no runtime overhead
- **Composable**: Futures can be combined and chained
- **Single-threaded by default**: A future runs on one thread unless explicitly moved

### Example from Our Code

```rust
impl Future for Timer {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if self.is_expired() {
            Poll::Ready(())
        } else {
            cx.waker().wake_by_ref();
            Poll::Pending
        }
    }
}
```

## Polling and State Machines

### The Polling Model

Rust's async system is based on cooperative polling:

1. **Poll**: The executor asks "are you ready?"
2. **Ready**: Future returns a value and completes
3. **Pending**: Future isn't ready, registers a waker, and yields control
4. **Wake**: When ready, the waker notifies the executor to poll again

### State Machine Transformation

When you write async code, the compiler transforms it into a state machine:

```rust
// This async function...
async fn example() {
    let x = some_async_operation().await;
    let y = another_async_operation(x).await;
    x + y
}

// ...becomes roughly equivalent to this state machine:
enum ExampleFuture {
    Start,
    WaitingForFirst(SomeAsyncOperation),
    WaitingForSecond(AnotherAsyncOperation, i32),
    Done,
}
```

### Benefits

- **Efficiency**: No heap allocation for simple futures
- **Predictable**: No hidden control flow
- **Composable**: State machines can be nested and combined

## Pin and Unpin

### Why Pin Exists

Some futures contain self-referential data - pointers to their own fields. Moving such futures would invalidate these pointers, causing memory safety issues.

```rust
struct SelfReferential {
    data: String,
    pointer: *const String, // Points to `data`
}
```

### Pin<T>

`Pin<T>` is a wrapper that prevents moving the wrapped value:

```rust
fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output>
```

### Unpin Trait

Types that are safe to move even when pinned implement `Unpin`:

```rust
// Most types are Unpin
impl<T> Unpin for MyStruct<T> where T: Unpin {}

// Some futures are !Unpin (not Unpin)
// These contain self-references and can't be moved
```

### In Our Code

```rust
// We use Pin<Box<dyn Future>> to handle any future type
future: Pin<Box<dyn Future<Output = ()> + Send>>,

// When polling, we work with pinned references
self.future.as_mut().poll(cx)
```

## Wakers and Context

### The Waker System

Wakers enable efficient async I/O by allowing futures to register callbacks:

1. **Future polls**: Returns `Poll::Pending` and stores the waker
2. **I/O completes**: System calls the waker
3. **Executor wakes**: Polls the future again

### Context

`Context` provides the future with access to the waker:

```rust
fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    // Get the waker from context
    let waker = cx.waker();
    
    // Register for notification
    register_waker(waker.clone());
    
    Poll::Pending
}
```

### Our Custom Waker

```rust
pub struct TaskWaker {
    ready: Arc<AtomicBool>,
}

impl TaskWaker {
    fn wake(&self) {
        self.ready.store(true, Ordering::Release);
    }
}
```

### Waker Vtable

Wakers use a vtable for dynamic dispatch:

```rust
static VTABLE: RawWakerVTable = RawWakerVTable::new(
    clone_waker,    // Clone the waker
    wake_waker,     // Wake by consuming
    wake_by_ref_waker, // Wake by reference
    drop_waker,     // Clean up resources
);
```

## Executors and Task Scheduling

### Executor Responsibilities

An executor manages the lifecycle of async tasks:

1. **Spawning**: Accept new futures as tasks
2. **Scheduling**: Decide which tasks to poll when
3. **Polling**: Drive futures to completion
4. **Cleanup**: Remove completed tasks

### Our Simple Executor

```rust
pub fn run(&mut self) -> usize {
    while !self.task_queue.is_empty() {
        // Poll all ready tasks
        for task in ready_tasks {
            match task.poll() {
                Poll::Ready(()) => completed += 1,
                Poll::Pending => requeue_task(task),
            }
        }
    }
}
```

### Scheduling Strategies

Different executors use different scheduling approaches:

- **Round-robin**: Our simple approach, fair but not optimal
- **Priority-based**: High-priority tasks run first
- **Work-stealing**: Multi-threaded executors balance load
- **Reactor pattern**: I/O-driven scheduling (epoll/kqueue)

### Real-World Executors

- **Tokio**: Multi-threaded, work-stealing, I/O integrated
- **async-std**: Similar to Tokio with different API
- **smol**: Lightweight single-threaded executor
- **Embassy**: Embedded systems executor

## Cooperative vs Preemptive Multitasking

### Cooperative Multitasking

In async Rust, tasks voluntarily yield control:

```rust
async fn cooperative_task() {
    // Do some work
    process_data();
    
    // Yield control point
    some_async_operation().await;
    
    // Do more work
    process_more_data();
}
```

### Benefits

- **Predictable**: No unexpected context switches
- **Efficient**: No kernel involvement for task switching
- **Safe**: No data races within a single thread

### Drawbacks

- **Blocking**: One blocking task stops all tasks
- **Fairness**: Poorly written tasks can starve others
- **CPU-bound**: Not ideal for compute-heavy workloads

### Yield Points

Tasks yield control at `.await` points:

```rust
// These are yield points
timer.await;
file.read().await;
channel.recv().await;

// This is NOT a yield point (blocks the executor)
std::thread::sleep(Duration::from_secs(1));
```

## Memory Management in Async Code

### Heap Allocation

Our executor uses `Pin<Box<dyn Future>>` for type erasure:

```rust
// Each task allocates a future on the heap
let task = Task::new(future); // Box::pin(future) inside
```

### Zero-Cost Abstractions

Simple async functions often compile to stack-allocated state machines:

```rust
async fn simple() {
    let x = 42;
    some_operation(x).await;
}
// No heap allocation needed!
```

### Lifetime Management

Async code must carefully manage lifetimes:

```rust
async fn borrow_data(data: &str) -> usize {
    // data must live for the entire async operation
    some_async_operation().await;
    data.len()
}
```

### Send and Sync Bounds

Futures that cross thread boundaries need `Send`:

```rust
fn spawn<F>(future: F) 
where 
    F: Future<Output = ()> + Send + 'static
{
    // Can be moved to another thread
}
```

## Performance Considerations

### Polling Overhead

Each poll has some overhead:

- Function call
- State machine transition
- Waker management

### Batching

Real executors batch operations:

```rust
// Instead of polling one task at a time
for task in tasks {
    task.poll();
}

// Batch multiple operations
let ready_tasks: Vec<_> = tasks.iter()
    .filter(|t| t.is_ready())
    .collect();
```

### I/O Integration

Production runtimes integrate with OS I/O:

- **Linux**: epoll
- **macOS**: kqueue  
- **Windows**: IOCP

### Memory Usage

Consider memory usage patterns:

- **Task overhead**: Each task has some fixed cost
- **Future size**: Large futures use more stack/heap
- **Waker storage**: Each pending future stores a waker

### CPU Usage

Async is great for I/O-bound workloads but consider:

- **CPU-bound tasks**: May need `spawn_blocking`
- **Busy waiting**: Our timer example is inefficient
- **Context switching**: Even cooperative switching has costs

## Best Practices

### Do

- Use `.await` for yield points
- Implement `Future` for custom async primitives
- Understand the polling model
- Profile async code for performance
- Use appropriate executor for your use case

### Don't

- Block in async code without `spawn_blocking`
- Ignore `Pin` requirements
- Assume futures run immediately
- Create unnecessary heap allocations
- Forget about `Send` bounds for multi-threaded executors

## Further Reading

- [The Rust Async Book](https://rust-lang.github.io/async-book/)
- [Futures Explained in 200 Lines of Rust](https://cfsamson.github.io/books-futures-explained/)
- [Pin and suffering](https://fasterthanli.me/articles/pin-and-suffering)
- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Async Rust: Cooperative vs Preemptive scheduling](https://ryhl.io/blog/async-what-is-blocking/)

Understanding these concepts deeply will make you proficient in async Rust and help you write efficient, correct async code.