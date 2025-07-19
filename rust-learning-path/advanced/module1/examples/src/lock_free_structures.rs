use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};
use std::ptr;
use std::sync::Arc;
use std::thread;

/// A lock-free stack implementation using atomic pointers
pub struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

struct Node<T> {
    data: T,
    next: *mut Node<T>,
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
                Err(_) => continue, // Retry
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
                Err(_) => continue, // Retry
            }
        }
    }

    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire).is_null()
    }
}

unsafe impl<T: Send> Send for LockFreeStack<T> {}
unsafe impl<T: Send> Sync for LockFreeStack<T> {}

impl<T> Drop for LockFreeStack<T> {
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}

/// A lock-free queue implementation (simplified version)
pub struct LockFreeQueue<T> {
    head: AtomicPtr<QueueNode<T>>,
    tail: AtomicPtr<QueueNode<T>>,
}

struct QueueNode<T> {
    data: Option<T>,
    next: AtomicPtr<QueueNode<T>>,
}

impl<T> LockFreeQueue<T> {
    pub fn new() -> Self {
        let dummy = Box::into_raw(Box::new(QueueNode {
            data: None,
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        LockFreeQueue {
            head: AtomicPtr::new(dummy),
            tail: AtomicPtr::new(dummy),
        }
    }

    pub fn enqueue(&self, data: T) {
        let new_node = Box::into_raw(Box::new(QueueNode {
            data: Some(data),
            next: AtomicPtr::new(ptr::null_mut()),
        }));

        loop {
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*tail).next.load(Ordering::Acquire) };

            if tail == self.tail.load(Ordering::Acquire) {
                if next.is_null() {
                    match unsafe { (*tail).next.compare_exchange_weak(
                        next,
                        new_node,
                        Ordering::Release,
                        Ordering::Relaxed,
                    ) } {
                        Ok(_) => break,
                        Err(_) => continue,
                    }
                } else {
                    // Help advance tail
                    let _ = self.tail.compare_exchange_weak(
                        tail,
                        next,
                        Ordering::Release,
                        Ordering::Relaxed,
                    );
                }
            }
        }

        // Advance tail
        let _ = self.tail.compare_exchange_weak(
            self.tail.load(Ordering::Acquire),
            new_node,
            Ordering::Release,
            Ordering::Relaxed,
        );
    }

    pub fn dequeue(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            let tail = self.tail.load(Ordering::Acquire);
            let next = unsafe { (*head).next.load(Ordering::Acquire) };

            if head == self.head.load(Ordering::Acquire) {
                if head == tail {
                    if next.is_null() {
                        return None; // Queue is empty
                    }
                    // Help advance tail
                    let _ = self.tail.compare_exchange_weak(
                        tail,
                        next,
                        Ordering::Release,
                        Ordering::Relaxed,
                    );
                } else {
                    if next.is_null() {
                        continue;
                    }

                    let data = unsafe { (*next).data.take() };

                    match self.head.compare_exchange_weak(
                        head,
                        next,
                        Ordering::Release,
                        Ordering::Relaxed,
                    ) {
                        Ok(_) => {
                            unsafe { Box::from_raw(head) }; // Free old head
                            return data;
                        }
                        Err(_) => continue,
                    }
                }
            }
        }
    }
}

unsafe impl<T: Send> Send for LockFreeQueue<T> {}
unsafe impl<T: Send> Sync for LockFreeQueue<T> {}

impl<T> Drop for LockFreeQueue<T> {
    fn drop(&mut self) {
        while self.dequeue().is_some() {}
        // Free the dummy node
        let head = self.head.load(Ordering::Relaxed);
        if !head.is_null() {
            unsafe { Box::from_raw(head) };
        }
    }
}

/// A simple atomic counter with additional operations
pub struct AtomicCounter {
    value: AtomicUsize,
}

impl AtomicCounter {
    pub fn new(initial: usize) -> Self {
        AtomicCounter {
            value: AtomicUsize::new(initial),
        }
    }

    pub fn increment(&self) -> usize {
        self.value.fetch_add(1, Ordering::SeqCst)
    }

    pub fn decrement(&self) -> usize {
        self.value.fetch_sub(1, Ordering::SeqCst)
    }

    pub fn add(&self, amount: usize) -> usize {
        self.value.fetch_add(amount, Ordering::SeqCst)
    }

    pub fn get(&self) -> usize {
        self.value.load(Ordering::SeqCst)
    }

    pub fn set(&self, value: usize) -> usize {
        self.value.swap(value, Ordering::SeqCst)
    }

    pub fn compare_and_swap(&self, expected: usize, new: usize) -> Result<usize, usize> {
        self.value.compare_exchange(expected, new, Ordering::SeqCst, Ordering::SeqCst)
    }
}

/// Demonstrates the lock-free stack
pub fn lock_free_stack_example() {
    println!("=== Lock-Free Stack Example ===");
    
    let stack = Arc::new(LockFreeStack::new());
    let mut handles = vec![];

    // Producer threads
    for i in 0..5 {
        let stack = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            for j in 0..10 {
                let value = i * 10 + j;
                stack.push(value);
                println!("Thread {} pushed: {}", i, value);
            }
        });
        handles.push(handle);
    }

    // Consumer threads
    for i in 0..3 {
        let stack = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            let mut count = 0;
            while count < 15 {
                if let Some(value) = stack.pop() {
                    println!("Consumer {} popped: {}", i, value);
                    count += 1;
                } else {
                    thread::yield_now();
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Pop remaining items
    let mut remaining = vec![];
    while let Some(value) = stack.pop() {
        remaining.push(value);
    }
    
    println!("Remaining items in stack: {:?}", remaining);
    println!("Stack is empty: {}", stack.is_empty());
}

/// Demonstrates the lock-free queue
pub fn lock_free_queue_example() {
    println!("\n=== Lock-Free Queue Example ===");
    
    let queue = Arc::new(LockFreeQueue::new());
    let mut handles = vec![];

    // Producer threads
    for i in 0..3 {
        let queue = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            for j in 0..10 {
                let value = i * 100 + j;
                queue.enqueue(value);
                println!("Producer {} enqueued: {}", i, value);
                thread::sleep(std::time::Duration::from_millis(10));
            }
        });
        handles.push(handle);
    }

    // Consumer threads
    for i in 0..2 {
        let queue = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            let mut count = 0;
            while count < 15 {
                if let Some(value) = queue.dequeue() {
                    println!("Consumer {} dequeued: {}", i, value);
                    count += 1;
                } else {
                    thread::yield_now();
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Dequeue remaining items
    let mut remaining = vec![];
    while let Some(value) = queue.dequeue() {
        remaining.push(value);
    }
    
    println!("Remaining items in queue: {:?}", remaining);
}

/// Demonstrates the atomic counter
pub fn atomic_counter_example() {
    println!("\n=== Atomic Counter Example ===");
    
    let counter = Arc::new(AtomicCounter::new(100));
    let mut handles = vec![];

    // Incrementing threads
    for i in 0..5 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..20 {
                let old = counter.increment();
                if old % 10 == 0 {
                    println!("Thread {} incremented from {}", i, old);
                }
            }
        });
        handles.push(handle);
    }

    // Decrementing threads
    for i in 0..3 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..15 {
                let old = counter.decrement();
                if old % 10 == 0 {
                    println!("Thread {} decremented from {}", i + 5, old);
                }
            }
        });
        handles.push(handle);
    }

    // Adding threads
    for i in 0..2 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let old = counter.add(25);
            println!("Thread {} added 25 from {}", i + 8, old);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", counter.get());
    // Expected: 100 + (5*20) - (3*15) + (2*25) = 100 + 100 - 45 + 50 = 205
}

/// Performance comparison between lock-free and mutex-based approaches
pub fn performance_comparison() {
    println!("\n=== Performance Comparison ===");
    
    use std::sync::Mutex;
    use std::time::Instant;

    const OPERATIONS: usize = 1_000_000;
    const THREADS: usize = 4;

    // Lock-free counter test
    let atomic_counter = Arc::new(AtomicCounter::new(0));
    let start = Instant::now();
    
    let mut handles = vec![];
    for _ in 0..THREADS {
        let counter = Arc::clone(&atomic_counter);
        let handle = thread::spawn(move || {
            for _ in 0..OPERATIONS / THREADS {
                counter.increment();
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let atomic_duration = start.elapsed();
    println!("Lock-free: {} operations in {:?}", 
             atomic_counter.get(), atomic_duration);

    // Mutex-based counter test
    let mutex_counter = Arc::new(Mutex::new(0usize));
    let start = Instant::now();
    
    let mut handles = vec![];
    for _ in 0..THREADS {
        let counter = Arc::clone(&mutex_counter);
        let handle = thread::spawn(move || {
            for _ in 0..OPERATIONS / THREADS {
                let mut val = counter.lock().unwrap();
                *val += 1;
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let mutex_duration = start.elapsed();
    println!("Mutex-based: {} operations in {:?}", 
             *mutex_counter.lock().unwrap(), mutex_duration);

    let speedup = mutex_duration.as_nanos() as f64 / atomic_duration.as_nanos() as f64;
    println!("Lock-free speedup: {:.2}x", speedup);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lock_free_stack_basic() {
        let stack = LockFreeStack::new();
        
        assert!(stack.is_empty());
        assert_eq!(stack.pop(), None);
        
        stack.push(1);
        stack.push(2);
        stack.push(3);
        
        assert!(!stack.is_empty());
        assert_eq!(stack.pop(), Some(3));
        assert_eq!(stack.pop(), Some(2));
        assert_eq!(stack.pop(), Some(1));
        assert_eq!(stack.pop(), None);
        assert!(stack.is_empty());
    }

    #[test]
    fn test_lock_free_stack_concurrent() {
        let stack = Arc::new(LockFreeStack::new());
        let mut handles = vec![];

        // Push from multiple threads
        for i in 0..10 {
            let stack = Arc::clone(&stack);
            let handle = thread::spawn(move || {
                for j in 0..100 {
                    stack.push(i * 100 + j);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Count items
        let mut count = 0;
        while stack.pop().is_some() {
            count += 1;
        }

        assert_eq!(count, 1000);
    }

    #[test]
    fn test_lock_free_queue_basic() {
        let queue = LockFreeQueue::new();
        
        assert_eq!(queue.dequeue(), None);
        
        queue.enqueue(1);
        queue.enqueue(2);
        queue.enqueue(3);
        
        assert_eq!(queue.dequeue(), Some(1));
        assert_eq!(queue.dequeue(), Some(2));
        assert_eq!(queue.dequeue(), Some(3));
        assert_eq!(queue.dequeue(), None);
    }

    #[test]
    fn test_lock_free_queue_concurrent() {
        let queue = Arc::new(LockFreeQueue::new());
        let mut handles = vec![];

        // Enqueue from multiple threads
        for i in 0..5 {
            let queue = Arc::clone(&queue);
            let handle = thread::spawn(move || {
                for j in 0..20 {
                    queue.enqueue(i * 20 + j);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Count items
        let mut count = 0;
        let mut items = vec![];
        while let Some(item) = queue.dequeue() {
            items.push(item);
            count += 1;
        }

        assert_eq!(count, 100);
        // Items should be in FIFO order within each thread's contributions
    }

    #[test]
    fn test_atomic_counter() {
        let counter = AtomicCounter::new(10);
        
        assert_eq!(counter.get(), 10);
        assert_eq!(counter.increment(), 10);
        assert_eq!(counter.get(), 11);
        assert_eq!(counter.decrement(), 11);
        assert_eq!(counter.get(), 10);
        assert_eq!(counter.add(5), 10);
        assert_eq!(counter.get(), 15);
        assert_eq!(counter.set(100), 15);
        assert_eq!(counter.get(), 100);
    }

    #[test]
    fn test_atomic_counter_concurrent() {
        let counter = Arc::new(AtomicCounter::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for _ in 0..1000 {
                    counter.increment();
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(counter.get(), 10000);
    }

    #[test]
    fn test_compare_and_swap() {
        let counter = AtomicCounter::new(42);
        
        // Successful CAS
        assert_eq!(counter.compare_and_swap(42, 100), Ok(42));
        assert_eq!(counter.get(), 100);
        
        // Failed CAS
        assert_eq!(counter.compare_and_swap(42, 200), Err(100));
        assert_eq!(counter.get(), 100);
    }
}