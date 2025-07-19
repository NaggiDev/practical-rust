use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicI32, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;

/// Demonstrates basic atomic operations
pub fn basic_atomic_example() {
    println!("=== Basic Atomic Example ===");
    
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
            println!("Thread {} completed", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", counter.load(Ordering::SeqCst));
}

/// Demonstrates different atomic operations
pub fn atomic_operations_example() {
    println!("\n=== Atomic Operations Example ===");
    
    let value = Arc::new(AtomicI32::new(100));
    let mut handles = vec![];

    // Thread that adds
    let value_add = Arc::clone(&value);
    let add_handle = thread::spawn(move || {
        for i in 1..=5 {
            let old = value_add.fetch_add(i, Ordering::SeqCst);
            println!("Add: {} + {} = {}", old, i, old + i);
            thread::sleep(Duration::from_millis(50));
        }
    });
    handles.push(add_handle);

    // Thread that subtracts
    let value_sub = Arc::clone(&value);
    let sub_handle = thread::spawn(move || {
        for i in 1..=3 {
            let old = value_sub.fetch_sub(i * 2, Ordering::SeqCst);
            println!("Sub: {} - {} = {}", old, i * 2, old - (i * 2));
            thread::sleep(Duration::from_millis(75));
        }
    });
    handles.push(sub_handle);

    // Thread that uses compare_exchange
    let value_cas = Arc::clone(&value);
    let cas_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(200));
        let current = value_cas.load(Ordering::SeqCst);
        println!("CAS: Attempting to change {} to 200", current);
        
        match value_cas.compare_exchange(current, 200, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(old) => println!("CAS: Successfully changed {} to 200", old),
            Err(actual) => println!("CAS: Failed, actual value was {}", actual),
        }
    });
    handles.push(cas_handle);

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final value: {}", value.load(Ordering::SeqCst));
}

/// Demonstrates compare_exchange_weak for retry loops
pub fn compare_exchange_weak_example() {
    println!("\n=== Compare Exchange Weak Example ===");
    
    let value = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for i in 0..5 {
        let value = Arc::clone(&value);
        let handle = thread::spawn(move || {
            let mut attempts = 0;
            loop {
                let current = value.load(Ordering::SeqCst);
                let new_value = current + i + 1;
                
                attempts += 1;
                match value.compare_exchange_weak(
                    current, 
                    new_value, 
                    Ordering::SeqCst, 
                    Ordering::SeqCst
                ) {
                    Ok(_) => {
                        println!("Thread {}: Updated {} to {} (attempts: {})", 
                                i, current, new_value, attempts);
                        break;
                    }
                    Err(_) => {
                        // Retry - compare_exchange_weak can fail spuriously
                        continue;
                    }
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

/// Demonstrates memory ordering effects
pub fn memory_ordering_example() {
    println!("\n=== Memory Ordering Example ===");
    
    let data = Arc::new(AtomicUsize::new(0));
    let ready = Arc::new(AtomicBool::new(false));

    let data_writer = Arc::clone(&data);
    let ready_writer = Arc::clone(&ready);

    // Writer thread
    let writer = thread::spawn(move || {
        // Write data first
        data_writer.store(42, Ordering::Relaxed);
        println!("Writer: Data written");
        
        // Then signal ready with Release ordering
        ready_writer.store(true, Ordering::Release);
        println!("Writer: Ready signal sent");
    });

    let data_reader = Arc::clone(&data);
    let ready_reader = Arc::clone(&ready);

    // Reader thread
    let reader = thread::spawn(move || {
        // Wait for ready signal with Acquire ordering
        while !ready_reader.load(Ordering::Acquire) {
            thread::yield_now();
        }
        
        // Now read data - guaranteed to see the write due to Release-Acquire
        let value = data_reader.load(Ordering::Relaxed);
        println!("Reader: Read value {}", value);
        value
    });

    writer.join().unwrap();
    let result = reader.join().unwrap();
    
    assert_eq!(result, 42);
    println!("Memory ordering example completed successfully");
}

/// Demonstrates relaxed ordering for performance
pub fn relaxed_ordering_example() {
    println!("\n=== Relaxed Ordering Example ===");
    
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    let start = std::time::Instant::now();

    for i in 0..8 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1_000_000 {
                counter.fetch_add(1, Ordering::Relaxed);
            }
            println!("Thread {} completed", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Relaxed ordering: {} operations in {:?}", 
             counter.load(Ordering::Relaxed), duration);
}

/// Demonstrates sequential consistency
pub fn sequential_consistency_example() {
    println!("\n=== Sequential Consistency Example ===");
    
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    let start = std::time::Instant::now();

    for i in 0..8 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..1_000_000 {
                counter.fetch_add(1, Ordering::SeqCst);
            }
            println!("Thread {} completed", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    let duration = start.elapsed();
    println!("Sequential consistency: {} operations in {:?}", 
             counter.load(Ordering::SeqCst), duration);
}

/// Atomic flag for signaling between threads
pub struct AtomicFlag {
    flag: AtomicBool,
}

impl AtomicFlag {
    pub fn new() -> Self {
        AtomicFlag {
            flag: AtomicBool::new(false),
        }
    }

    pub fn set(&self) {
        self.flag.store(true, Ordering::Release);
    }

    pub fn is_set(&self) -> bool {
        self.flag.load(Ordering::Acquire)
    }

    pub fn wait(&self) {
        while !self.is_set() {
            thread::yield_now();
        }
    }

    pub fn clear(&self) {
        self.flag.store(false, Ordering::Release);
    }

    /// Test and set operation - returns previous value
    pub fn test_and_set(&self) -> bool {
        self.flag.swap(true, Ordering::AcqRel)
    }
}

/// Demonstrates the atomic flag
pub fn atomic_flag_example() {
    println!("\n=== Atomic Flag Example ===");
    
    let flag = Arc::new(AtomicFlag::new());
    let flag_waiter = Arc::clone(&flag);

    let waiter = thread::spawn(move || {
        println!("Waiter: Waiting for flag...");
        flag_waiter.wait();
        println!("Waiter: Flag is set!");
    });

    // Set flag after delay
    thread::sleep(Duration::from_millis(100));
    flag.set();
    println!("Main: Flag set");

    waiter.join().unwrap();
}

/// Lock-free increment with retry
pub fn lock_free_increment(counter: &AtomicUsize, amount: usize) -> usize {
    let mut current = counter.load(Ordering::Relaxed);
    loop {
        let new_value = current + amount;
        match counter.compare_exchange_weak(
            current,
            new_value,
            Ordering::Release,
            Ordering::Relaxed,
        ) {
            Ok(_) => return new_value,
            Err(actual) => current = actual,
        }
    }
}

/// Demonstrates lock-free operations
pub fn lock_free_example() {
    println!("\n=== Lock-Free Example ===");
    
    let counter = Arc::new(AtomicUsize::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for j in 1..=100 {
                let new_value = lock_free_increment(&counter, j);
                if j % 25 == 0 {
                    println!("Thread {}: Incremented to {}", i, new_value);
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", counter.load(Ordering::SeqCst));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_atomic_operations() {
        let counter = AtomicUsize::new(0);
        
        assert_eq!(counter.fetch_add(5, Ordering::SeqCst), 0);
        assert_eq!(counter.load(Ordering::SeqCst), 5);
        
        assert_eq!(counter.fetch_sub(2, Ordering::SeqCst), 5);
        assert_eq!(counter.load(Ordering::SeqCst), 3);
        
        assert_eq!(counter.swap(10, Ordering::SeqCst), 3);
        assert_eq!(counter.load(Ordering::SeqCst), 10);
    }

    #[test]
    fn test_compare_exchange() {
        let value = AtomicI32::new(42);
        
        // Successful compare_exchange
        match value.compare_exchange(42, 100, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(old) => assert_eq!(old, 42),
            Err(_) => panic!("Should have succeeded"),
        }
        
        assert_eq!(value.load(Ordering::SeqCst), 100);
        
        // Failed compare_exchange
        match value.compare_exchange(42, 200, Ordering::SeqCst, Ordering::SeqCst) {
            Ok(_) => panic!("Should have failed"),
            Err(actual) => assert_eq!(actual, 100),
        }
        
        assert_eq!(value.load(Ordering::SeqCst), 100);
    }

    #[test]
    fn test_atomic_flag() {
        let flag = AtomicFlag::new();
        
        assert!(!flag.is_set());
        
        flag.set();
        assert!(flag.is_set());
        
        flag.clear();
        assert!(!flag.is_set());
        
        // Test and set
        assert!(!flag.test_and_set()); // Was false
        assert!(flag.is_set()); // Now true
        assert!(flag.test_and_set()); // Was true
    }

    #[test]
    fn test_concurrent_atomic_operations() {
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

        assert_eq!(counter.load(Ordering::SeqCst), 10000);
    }

    #[test]
    fn test_memory_ordering_release_acquire() {
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
        
        assert_eq!(result, 42);
    }

    #[test]
    fn test_lock_free_increment() {
        let counter = AtomicUsize::new(10);
        
        let result = lock_free_increment(&counter, 5);
        assert_eq!(result, 15);
        assert_eq!(counter.load(Ordering::SeqCst), 15);
    }

    #[test]
    fn test_concurrent_lock_free_increment() {
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                for i in 1..=100 {
                    lock_free_increment(&counter, i);
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        // Each thread adds 1+2+...+100 = 5050
        // 10 threads = 50500
        assert_eq!(counter.load(Ordering::SeqCst), 50500);
    }
}