# Concurrency Examples

This directory contains comprehensive examples demonstrating Rust's concurrency features. These examples accompany Module 1: Concurrency in the Advanced Level of the Rust Learning Path.

## Prerequisites

- Rust 1.70+ installed
- Basic understanding of Rust ownership and borrowing
- Completion of Intermediate Level modules

## Running the Examples

### Run All Examples
```bash
cargo run
```

### Run Tests
```bash
cargo test
```

### Run Specific Test Module
```bash
cargo test basic_threads
cargo test message_passing
cargo test shared_state
cargo test atomic_operations
cargo test lock_free_structures
```

### Run with Output
```bash
cargo test -- --nocapture
```

## Example Categories

### 1. Basic Threads (`src/basic_threads.rs`)

Demonstrates fundamental thread operations:
- Creating and joining threads
- Moving data into threads
- Thread builders for custom configuration
- Managing multiple threads

**Key Concepts:**
- `std::thread::spawn`
- `JoinHandle`
- `move` closures
- Thread configuration

### 2. Message Passing (`src/message_passing.rs`)

Shows communication between threads using channels:
- Basic channel communication
- Multiple producers, single consumer
- Bounded channels (sync_channel)
- Non-blocking and timed receives
- Work distribution patterns

**Key Concepts:**
- `std::sync::mpsc`
- Channel types: `Sender`, `Receiver`
- Bounded vs unbounded channels
- Error handling with channels

### 3. Shared State (`src/shared_state.rs`)

Demonstrates shared mutable state synchronization:
- Mutex for exclusive access
- RwLock for read-heavy workloads
- Condition variables for coordination
- Producer-consumer patterns
- Mutex poisoning and recovery

**Key Concepts:**
- `std::sync::Mutex`
- `std::sync::RwLock`
- `std::sync::Condvar`
- `Arc` for shared ownership
- Deadlock prevention

### 4. Atomic Operations (`src/atomic_operations.rs`)

Covers lock-free synchronization primitives:
- Basic atomic types and operations
- Compare-and-swap operations
- Memory ordering semantics
- Lock-free algorithms
- Performance considerations

**Key Concepts:**
- `std::sync::atomic`
- Memory ordering: `Relaxed`, `Acquire`, `Release`, `SeqCst`
- `compare_exchange` vs `compare_exchange_weak`
- Atomic flags and counters

### 5. Lock-Free Data Structures (`src/lock_free_structures.rs`)

Implements advanced lock-free data structures:
- Lock-free stack
- Lock-free queue (simplified)
- Atomic counter with additional operations
- Performance comparisons

**Key Concepts:**
- ABA problem considerations
- Memory management in lock-free structures
- Hazard pointers (conceptual)
- Performance trade-offs

## Learning Objectives

After working through these examples, you should understand:

1. **Thread Safety**: How Rust's type system prevents data races
2. **Synchronization Patterns**: When to use different synchronization primitives
3. **Performance Trade-offs**: Lock-based vs lock-free approaches
4. **Memory Ordering**: How memory ordering affects correctness
5. **Error Handling**: Dealing with poisoned mutexes and channel errors

## Common Patterns

### Producer-Consumer
```rust
use std::sync::mpsc;
use std::thread;

let (tx, rx) = mpsc::channel();

// Producer
thread::spawn(move || {
    for i in 0..10 {
        tx.send(i).unwrap();
    }
});

// Consumer
for received in rx {
    println!("Got: {}", received);
}
```

### Shared Counter
```rust
use std::sync::{Arc, Mutex};
use std::thread;

let counter = Arc::new(Mutex::new(0));
let mut handles = vec![];

for _ in 0..10 {
    let counter = Arc::clone(&counter);
    let handle = thread::spawn(move || {
        let mut num = counter.lock().unwrap();
        *num += 1;
    });
    handles.push(handle);
}

for handle in handles {
    handle.join().unwrap();
}
```

### Atomic Operations
```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

let counter = Arc::new(AtomicUsize::new(0));

// Lock-free increment
counter.fetch_add(1, Ordering::SeqCst);

// Compare and swap
let current = counter.load(Ordering::SeqCst);
counter.compare_exchange(current, current + 1, Ordering::SeqCst, Ordering::SeqCst);
```

## Best Practices

1. **Prefer Message Passing**: Use channels when possible for cleaner code
2. **Minimize Shared State**: Reduce the scope of shared mutable data
3. **Choose Appropriate Ordering**: Use the weakest memory ordering that's correct
4. **Handle Errors**: Always handle potential errors from synchronization primitives
5. **Avoid Deadlocks**: Acquire locks in consistent order
6. **Test Thoroughly**: Concurrent code requires extensive testing

## Performance Notes

- **Channels**: Good for loose coupling, some overhead
- **Mutex**: Simple but can cause contention
- **RwLock**: Better for read-heavy workloads
- **Atomics**: Fastest but limited to simple operations
- **Lock-free**: Complex but can offer best performance

## Debugging Tips

1. Use `RUST_BACKTRACE=1` for better panic information
2. Consider using `std::thread::yield_now()` in tight loops
3. Use `std::sync::Barrier` for coordinating thread starts
4. Profile with tools like `perf` or `cargo flamegraph`

## Further Reading

- [The Rust Book - Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Rust Atomics and Locks](https://marabos.nl/atomics/)
- [The Rustonomicon - Concurrency](https://doc.rust-lang.org/nomicon/concurrency.html)
- [std::sync Documentation](https://doc.rust-lang.org/std/sync/index.html)