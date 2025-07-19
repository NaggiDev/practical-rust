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

## Rust's Concurrency Model

Rust's approach to concurrency is built on two key concepts:
- **Fearless Concurrency**: The type system prevents data races at compile time
- **Zero-cost Abstractions**: High-level concurrency primitives with no runtime overhead

### Send and Sync Traits

These marker traits are fundamental to Rust's concurrency safety:

- `Send`: Types that can be transferred between threads
- `Sync`: Types that can be safely shared between threads

```rust
// Most types are Send and Sync by default
let data = vec![1, 2, 3, 4, 5];
// Vec<i32> is both Send and Sync

// Some types are not Send (like Rc<T>)
use std::rc::Rc;
let rc_data = Rc::new(42);
// Rc<i32> is neither Send nor Sync

// Some types are Send but not Sync (like Cell<T>)
use std::cell::Cell;
let cell_data = Cell::new(42);
// Cell<i32> is Send but not Sync
```

## Concepts Covered

### 1. Thread Creation and Management

#### Basic Thread Creation

```rust
use std::thread;
use std::time::Duration;

fn basic_threads() {
    // Spawn a new thread
    let handle = thread::spawn(|| {
        for i in 1..10 {
            println!("Thread: {}", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    // Do work in main thread
    for i in 1..5 {
        println!("Main: {}", i);
        thread::sleep(Duration::from_millis(1));
    }

    // Wait for thread to complete
    handle.join().unwrap();
}
```

#### Moving Data into Threads

```rust
use std::thread;

fn move_data_to_threads() {
    let data = vec![1, 2, 3, 4, 5];
    
    let handle = thread::spawn(move || {
        println!("Thread received: {:?}", data);
        data.iter().sum::<i32>()
    });

    let result = handle.join().unwrap();
    println!("Sum: {}", result);
}
```

### 2. Message Passing Between Threads

#### Using Channels

```rust
use std::sync::mpsc;
use std::thread;

fn message_passing() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec![
            String::from("Hello"),
            String::from("from"),
            String::from("thread"),
        ];

        for msg in messages {
            tx.send(msg).unwrap();
        }
    });

    for received in rx {
        println!("Received: {}", received);
    }
}
```

#### Multiple Producers

```rust
use std::sync::mpsc;
use std::thread;

fn multiple_producers() {
    let (tx, rx) = mpsc::channel();

    for i in 0..3 {
        let tx_clone = tx.clone();
        thread::spawn(move || {
            tx_clone.send(format!("Message from thread {}", i)).unwrap();
        });
    }

    // Drop the original sender
    drop(tx);

    for received in rx {
        println!("Received: {}", received);
    }
}
```

### 3. Shared State Concurrency

#### Using Mutex for Shared Data

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn shared_state_mutex() {
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

    println!("Result: {}", *counter.lock().unwrap());
}
```

#### Using RwLock for Read-Heavy Workloads

```rust
use std::sync::{Arc, RwLock};
use std::thread;

fn shared_state_rwlock() {
    let data = Arc::new(RwLock::new(vec![1, 2, 3, 4, 5]));
    let mut handles = vec![];

    // Multiple readers
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let reader = data.read().unwrap();
            println!("Reader {}: {:?}", i, *reader);
        });
        handles.push(handle);
    }

    // One writer
    let data_writer = Arc::clone(&data);
    let writer_handle = thread::spawn(move || {
        let mut writer = data_writer.write().unwrap();
        writer.push(6);
        println!("Writer added element");
    });
    handles.push(writer_handle);

    for handle in handles {
        handle.join().unwrap();
    }
}
```

### 4. Atomic Operations

#### Basic Atomic Types

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn atomic_operations() {
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final count: {}", counter.load(Ordering::SeqCst));
}
```

#### Compare and Swap Operations

```rust
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn compare_and_swap() {
    let value = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let value = Arc::clone(&value);
        let handle = thread::spawn(move || {
            loop {
                let current = value.load(Ordering::SeqCst);
                let new_value = current + i;
                
                match value.compare_exchange_weak(
                    current, 
                    new_value, 
                    Ordering::SeqCst, 
                    Ordering::SeqCst
                ) {
                    Ok(_) => {
                        println!("Thread {} updated value to {}", i, new_value);
                        break;
                    }
                    Err(_) => continue, // Retry
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final value: {}", value.load(Ordering::SeqCst));
}
```

### 5. Memory Ordering

Understanding memory ordering is crucial for atomic operations:

```rust
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::thread;

fn memory_ordering_example() {
    let data = Arc::new(AtomicUsize::new(0));
    let ready = Arc::new(AtomicBool::new(false));

    let data_clone = Arc::clone(&data);
    let ready_clone = Arc::clone(&ready);

    // Writer thread
    let writer = thread::spawn(move || {
        data_clone.store(42, Ordering::Relaxed);
        ready_clone.store(true, Ordering::Release); // Release ordering
    });

    // Reader thread
    let reader = thread::spawn(move || {
        while !ready.load(Ordering::Acquire) { // Acquire ordering
            thread::yield_now();
        }
        let value = data.load(Ordering::Relaxed);
        println!("Read value: {}", value);
    });

    writer.join().unwrap();
    reader.join().unwrap();
}
```

### 6. Deadlock Prevention

#### Ordered Lock Acquisition

```rust
use std::sync::{Arc, Mutex};
use std::thread;

fn prevent_deadlock() {
    let resource1 = Arc::new(Mutex::new(1));
    let resource2 = Arc::new(Mutex::new(2));

    let r1_clone = Arc::clone(&resource1);
    let r2_clone = Arc::clone(&resource2);

    let handle1 = thread::spawn(move || {
        // Always acquire locks in the same order
        let _lock1 = r1_clone.lock().unwrap();
        println!("Thread 1: acquired resource1");
        
        let _lock2 = r2_clone.lock().unwrap();
        println!("Thread 1: acquired resource2");
    });

    let r1_clone = Arc::clone(&resource1);
    let r2_clone = Arc::clone(&resource2);

    let handle2 = thread::spawn(move || {
        // Same order prevents deadlock
        let _lock1 = r1_clone.lock().unwrap();
        println!("Thread 2: acquired resource1");
        
        let _lock2 = r2_clone.lock().unwrap();
        println!("Thread 2: acquired resource2");
    });

    handle1.join().unwrap();
    handle2.join().unwrap();
}
```

### 7. Thread-Safe Data Structures

#### Lock-Free Stack

```rust
use std::sync::atomic::{AtomicPtr, Ordering};
use std::ptr;

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

pub struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

impl<T> LockFreeStack<T> {
    pub fn new() -> Self {
        LockFreeStack {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }

    pub fn push(&self, data: T) {
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: ptr::null_mut(),
        }));

        loop {
            let head = self.head.load(Ordering::Acquire);
            unsafe {
                (*new_node).next = head;
            }

            match self.head.compare_exchange_weak(
                head,
                new_node,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(_) => continue,
            }
        }
    }

    pub fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }

            let next = unsafe { (*head).next };

            match self.head.compare_exchange_weak(
                head,
                next,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    let data = unsafe { Box::from_raw(head).data };
                    return Some(data);
                }
                Err(_) => continue,
            }
        }
    }
}

unsafe impl<T: Send> Send for LockFreeStack<T> {}
unsafe impl<T: Send> Sync for LockFreeStack<T> {}
```

## Code Examples

### [Comprehensive Examples](examples/README.md)

Work through detailed code examples covering all concurrency concepts:

- **[Basic Threads](examples/src/basic_threads.rs)**: Thread creation, joining, and data movement
- **[Message Passing](examples/src/message_passing.rs)**: Channels, producers, consumers, and work distribution
- **[Shared State](examples/src/shared_state.rs)**: Mutex, RwLock, Condvar, and synchronization patterns
- **[Atomic Operations](examples/src/atomic_operations.rs)**: Lock-free programming and memory ordering
- **[Lock-Free Structures](examples/src/lock_free_structures.rs)**: Advanced data structures and performance

**To run the examples:**
```bash
cd examples
cargo run    # Run all examples
cargo test   # Run all tests
```

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

## Learning Path

1. **Study the Concepts**: Read through the concept explanations above
2. **Run the Examples**: Work through the code examples to see concepts in action
3. **Complete the Project**: Build the thread pool to apply your knowledge
4. **Experiment**: Modify examples and create your own concurrent programs

## Resources

- [The Rust Book - Fearless Concurrency](https://doc.rust-lang.org/book/ch16-00-concurrency.html)
- [Rust Atomics and Locks](https://marabos.nl/atomics/)
- [Rust By Example - Concurrency](https://doc.rust-lang.org/rust-by-example/std_misc/threads.html)
- [std::sync Documentation](https://doc.rust-lang.org/std/sync/index.html)
- [std::thread Documentation](https://doc.rust-lang.org/std/thread/index.html)
- [std::sync::atomic Documentation](https://doc.rust-lang.org/std/sync/atomic/index.html)

## Next Steps

After completing this module, proceed to [Module 2: Unsafe Rust](../module2/index.md) to learn when and how to use unsafe Rust safely.