use concurrency_examples::*;
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

/// Integration test for basic thread operations
#[test]
fn test_thread_communication_patterns() {
    // Test 1: Basic thread with return value
    let handle = thread::spawn(|| {
        thread::sleep(Duration::from_millis(10));
        42
    });
    
    let result = handle.join().unwrap();
    assert_eq!(result, 42);

    // Test 2: Multiple threads with shared counter
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    assert_eq!(*counter.lock().unwrap(), 1000);
}

/// Integration test for channel communication
#[test]
fn test_channel_patterns() {
    // Test 1: Basic send/receive
    let (tx, rx) = mpsc::channel();
    
    thread::spawn(move || {
        tx.send("Hello from thread").unwrap();
    });
    
    let received = rx.recv().unwrap();
    assert_eq!(received, "Hello from thread");

    // Test 2: Multiple producers
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    for i in 0..5 {
        let tx = tx.clone();
        let handle = thread::spawn(move || {
            tx.send(i).unwrap();
        });
        handles.push(handle);
    }

    drop(tx); // Close the channel

    for handle in handles {
        handle.join().unwrap();
    }

    let mut received = vec![];
    for msg in rx {
        received.push(msg);
    }

    received.sort();
    assert_eq!(received, vec![0, 1, 2, 3, 4]);
}

/// Integration test for atomic operations
#[test]
fn test_atomic_patterns() {
    use std::sync::atomic::{AtomicUsize, Ordering};

    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    // Test concurrent atomic increments
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

    assert_eq!(counter.load(Ordering::SeqCst), 10000);

    // Test compare_exchange
    let value = Arc::new(AtomicUsize::new(42));
    let success = value.compare_exchange(42, 100, Ordering::SeqCst, Ordering::SeqCst);
    assert_eq!(success, Ok(42));
    assert_eq!(value.load(Ordering::SeqCst), 100);
}

/// Integration test for lock-free data structures
#[test]
fn test_lock_free_structures() {
    // Test lock-free stack
    let stack = Arc::new(LockFreeStack::new());
    let mut handles = vec![];

    // Push from multiple threads
    for i in 0..5 {
        let stack = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            for j in 0..10 {
                stack.push(i * 10 + j);
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Pop all items
    let mut count = 0;
    while stack.pop().is_some() {
        count += 1;
    }

    assert_eq!(count, 50);
    assert!(stack.is_empty());
}

/// Integration test for producer-consumer pattern
#[test]
fn test_producer_consumer_pattern() {
    let (tx, rx) = mpsc::channel();
    let produced_items = Arc::new(Mutex::new(Vec::new()));
    let consumed_items = Arc::new(Mutex::new(Vec::new()));

    // Producer
    let produced_clone = Arc::clone(&produced_items);
    let producer = thread::spawn(move || {
        for i in 0..100 {
            tx.send(i).unwrap();
            produced_clone.lock().unwrap().push(i);
            if i % 10 == 0 {
                thread::sleep(Duration::from_millis(1));
            }
        }
    });

    // Consumer
    let consumed_clone = Arc::clone(&consumed_items);
    let consumer = thread::spawn(move || {
        for item in rx {
            consumed_clone.lock().unwrap().push(item);
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();

    let produced = produced_items.lock().unwrap();
    let consumed = consumed_items.lock().unwrap();

    assert_eq!(produced.len(), 100);
    assert_eq!(consumed.len(), 100);
    
    // All items should be produced and consumed
    let mut produced_sorted = produced.clone();
    let mut consumed_sorted = consumed.clone();
    produced_sorted.sort();
    consumed_sorted.sort();
    
    assert_eq!(produced_sorted, consumed_sorted);
}

/// Integration test for thread synchronization with barriers
#[test]
fn test_thread_synchronization() {
    use std::sync::Barrier;

    let barrier = Arc::new(Barrier::new(5));
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let barrier = Arc::clone(&barrier);
        let counter = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            // Phase 1: Each thread increments counter
            {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
            
            // Wait for all threads to complete phase 1
            barrier.wait();
            
            // Phase 2: All threads should see the same counter value
            let final_count = *counter.lock().unwrap();
            assert_eq!(final_count, 5);
            
            i * 2 // Return thread-specific result
        });
        
        handles.push(handle);
    }

    let results: Vec<i32> = handles
        .into_iter()
        .map(|h| h.join().unwrap())
        .collect();

    let expected: Vec<i32> = (0..5).map(|i| i * 2).collect();
    assert_eq!(results.len(), expected.len());
    
    // Results should contain all expected values (order may vary)
    for expected_val in expected {
        assert!(results.contains(&expected_val));
    }
}

/// Integration test for error handling in concurrent code
#[test]
fn test_concurrent_error_handling() {
    // Test channel disconnection
    let (tx, rx) = mpsc::channel::<i32>();
    
    drop(tx); // Close sender
    
    match rx.recv() {
        Ok(_) => panic!("Should have failed"),
        Err(mpsc::RecvError) => {}, // Expected
    }

    // Test try_recv on empty channel
    let (tx, rx) = mpsc::channel::<i32>();
    
    match rx.try_recv() {
        Ok(_) => panic!("Should be empty"),
        Err(mpsc::TryRecvError::Empty) => {}, // Expected
        Err(mpsc::TryRecvError::Disconnected) => panic!("Should be empty, not disconnected"),
    }

    drop(tx);
    
    match rx.try_recv() {
        Ok(_) => panic!("Should be disconnected"),
        Err(mpsc::TryRecvError::Empty) => panic!("Should be disconnected, not empty"),
        Err(mpsc::TryRecvError::Disconnected) => {}, // Expected
    }
}

/// Integration test for performance characteristics
#[test]
fn test_performance_characteristics() {
    use std::time::Instant;
    use std::sync::atomic::{AtomicUsize, Ordering};

    const OPERATIONS: usize = 100_000;
    const THREADS: usize = 4;

    // Test atomic operations performance
    let atomic_counter = Arc::new(AtomicUsize::new(0));
    let start = Instant::now();
    
    let mut handles = vec![];
    for _ in 0..THREADS {
        let counter = Arc::clone(&atomic_counter);
        let handle = thread::spawn(move || {
            for _ in 0..OPERATIONS / THREADS {
                counter.fetch_add(1, Ordering::Relaxed);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let atomic_duration = start.elapsed();
    assert_eq!(atomic_counter.load(Ordering::SeqCst), OPERATIONS);

    // Test mutex performance
    let mutex_counter = Arc::new(Mutex::new(0usize));
    let start = Instant::now();
    
    let mut handles = vec![];
    for _ in 0..THREADS {
        let counter = Arc::clone(&mutex_counter);
        let handle = thread::spawn(move || {
            for _ in 0..OPERATIONS / THREADS {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let mutex_duration = start.elapsed();
    assert_eq!(*mutex_counter.lock().unwrap(), OPERATIONS);

    // Atomic operations should generally be faster for simple operations
    println!("Atomic: {:?}, Mutex: {:?}", atomic_duration, mutex_duration);
    
    // Both should complete successfully regardless of performance
    assert!(atomic_duration > Duration::from_nanos(0));
    assert!(mutex_duration > Duration::from_nanos(0));
}

/// Integration test for memory ordering effects
#[test]
fn test_memory_ordering_effects() {
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

    // Test Release-Acquire ordering
    for _ in 0..100 { // Run multiple times to catch potential issues
        let data = Arc::new(AtomicUsize::new(0));
        let ready = Arc::new(AtomicBool::new(false));

        let data_writer = Arc::clone(&data);
        let ready_writer = Arc::clone(&ready);

        let writer = thread::spawn(move || {
            data_writer.store(42, Ordering::Relaxed);
            ready_writer.store(true, Ordering::Release);
        });

        let data_reader = Arc::clone(&data);
        let ready_reader = Arc::clone(&ready);

        let reader = thread::spawn(move || {
            while !ready_reader.load(Ordering::Acquire) {
                thread::yield_now();
            }
            data_reader.load(Ordering::Relaxed)
        });

        writer.join().unwrap();
        let result = reader.join().unwrap();
        
        // Due to Release-Acquire ordering, reader should always see 42
        assert_eq!(result, 42);
    }
}