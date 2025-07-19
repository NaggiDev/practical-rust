use std::sync::{Arc, Mutex, RwLock, Condvar};
use std::thread;
use std::time::Duration;
use std::collections::HashMap;

/// Demonstrates basic Mutex usage for shared state
pub fn basic_mutex_example() {
    println!("=== Basic Mutex Example ===");
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = vec![];

    for i in 0..10 {
        let counter = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                let mut num = counter.lock().unwrap();
                *num += 1;
            }
            println!("Thread {} completed", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", *counter.lock().unwrap());
}

/// Demonstrates RwLock for read-heavy workloads
pub fn rwlock_example() {
    println!("\n=== RwLock Example ===");
    
    let data = Arc::new(RwLock::new(HashMap::new()));
    let mut handles = vec![];

    // Insert initial data
    {
        let mut map = data.write().unwrap();
        for i in 0..5 {
            map.insert(format!("key{}", i), i * 10);
        }
    }

    // Spawn multiple readers
    for i in 0..5 {
        let data = Arc::clone(&data);
        let handle = thread::spawn(move || {
            for j in 0..3 {
                let reader = data.read().unwrap();
                let key = format!("key{}", j);
                if let Some(value) = reader.get(&key) {
                    println!("Reader {}: {} = {}", i, key, value);
                }
                thread::sleep(Duration::from_millis(10));
            }
        });
        handles.push(handle);
    }

    // Spawn a writer
    let data_writer = Arc::clone(&data);
    let writer_handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        let mut writer = data_writer.write().unwrap();
        writer.insert("new_key".to_string(), 999);
        println!("Writer: Added new_key = 999");
    });
    handles.push(writer_handle);

    // Another reader after writer
    let data_final = Arc::clone(&data);
    let final_reader = thread::spawn(move || {
        thread::sleep(Duration::from_millis(100));
        let reader = data_final.read().unwrap();
        println!("Final reader: map has {} entries", reader.len());
        for (key, value) in reader.iter() {
            println!("  {} = {}", key, value);
        }
    });
    handles.push(final_reader);

    for handle in handles {
        handle.join().unwrap();
    }
}

/// Demonstrates Condvar for thread coordination
pub fn condvar_example() {
    println!("\n=== Condvar Example ===");
    
    let pair = Arc::new((Mutex::new(false), Condvar::new()));
    let pair2 = Arc::clone(&pair);

    // Spawn a thread that waits for the condition
    let waiter = thread::spawn(move || {
        let (lock, cvar) = &*pair2;
        let mut started = lock.lock().unwrap();
        
        while !*started {
            println!("Waiter: Waiting for condition...");
            started = cvar.wait(started).unwrap();
        }
        
        println!("Waiter: Condition met, proceeding!");
    });

    // Main thread sets the condition after a delay
    thread::sleep(Duration::from_millis(500));
    
    let (lock, cvar) = &*pair;
    let mut started = lock.lock().unwrap();
    *started = true;
    cvar.notify_one();
    println!("Main: Notified waiter");

    waiter.join().unwrap();
}

/// Demonstrates producer-consumer pattern with Condvar
pub fn producer_consumer_example() {
    println!("\n=== Producer-Consumer Example ===");
    
    let buffer = Arc::new((Mutex::new(Vec::new()), Condvar::new()));
    const BUFFER_SIZE: usize = 5;
    
    let buffer_producer = Arc::clone(&buffer);
    let producer = thread::spawn(move || {
        for i in 0..10 {
            let (lock, cvar) = &*buffer_producer;
            let mut buf = lock.lock().unwrap();
            
            // Wait while buffer is full
            while buf.len() >= BUFFER_SIZE {
                println!("Producer: Buffer full, waiting...");
                buf = cvar.wait(buf).unwrap();
            }
            
            buf.push(i);
            println!("Producer: Added item {}, buffer size: {}", i, buf.len());
            cvar.notify_all(); // Notify consumers
            
            thread::sleep(Duration::from_millis(100));
        }
    });

    let buffer_consumer = Arc::clone(&buffer);
    let consumer = thread::spawn(move || {
        for _ in 0..10 {
            let (lock, cvar) = &*buffer_consumer;
            let mut buf = lock.lock().unwrap();
            
            // Wait while buffer is empty
            while buf.is_empty() {
                println!("Consumer: Buffer empty, waiting...");
                buf = cvar.wait(buf).unwrap();
            }
            
            let item = buf.remove(0);
            println!("Consumer: Consumed item {}, buffer size: {}", item, buf.len());
            cvar.notify_all(); // Notify producer
            
            thread::sleep(Duration::from_millis(150));
        }
    });

    producer.join().unwrap();
    consumer.join().unwrap();
}

/// Demonstrates handling mutex poisoning
pub fn mutex_poisoning_example() {
    println!("\n=== Mutex Poisoning Example ===");
    
    let mutex = Arc::new(Mutex::new(0));
    let mutex_clone = Arc::clone(&mutex);

    // Thread that panics while holding the mutex
    let handle = thread::spawn(move || {
        let mut data = mutex_clone.lock().unwrap();
        *data = 42;
        panic!("Oops! Thread panicked while holding mutex");
    });

    // Wait for the thread to panic
    let result = handle.join();
    println!("Thread result: {:?}", result);

    // Try to use the mutex after poisoning
    match mutex.lock() {
        Ok(data) => {
            println!("Mutex not poisoned, data: {}", *data);
        }
        Err(poisoned) => {
            println!("Mutex is poisoned!");
            let data = poisoned.into_inner();
            println!("Recovered data: {}", *data);
        }
    }
}

/// Thread-safe counter with multiple operations
pub struct ThreadSafeCounter {
    value: Arc<Mutex<i32>>,
}

impl ThreadSafeCounter {
    pub fn new() -> Self {
        ThreadSafeCounter {
            value: Arc::new(Mutex::new(0)),
        }
    }

    pub fn increment(&self) {
        let mut val = self.value.lock().unwrap();
        *val += 1;
    }

    pub fn decrement(&self) {
        let mut val = self.value.lock().unwrap();
        *val -= 1;
    }

    pub fn get(&self) -> i32 {
        let val = self.value.lock().unwrap();
        *val
    }

    pub fn add(&self, amount: i32) {
        let mut val = self.value.lock().unwrap();
        *val += amount;
    }
}

impl Clone for ThreadSafeCounter {
    fn clone(&self) -> Self {
        ThreadSafeCounter {
            value: Arc::clone(&self.value),
        }
    }
}

/// Demonstrates the thread-safe counter
pub fn thread_safe_counter_example() {
    println!("\n=== Thread-Safe Counter Example ===");
    
    let counter = ThreadSafeCounter::new();
    let mut handles = vec![];

    // Incrementing threads
    for i in 0..5 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                counter.increment();
            }
            println!("Increment thread {} completed", i);
        });
        handles.push(handle);
    }

    // Decrementing threads
    for i in 0..3 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            for _ in 0..50 {
                counter.decrement();
            }
            println!("Decrement thread {} completed", i);
        });
        handles.push(handle);
    }

    // Adding threads
    for i in 0..2 {
        let counter = counter.clone();
        let handle = thread::spawn(move || {
            counter.add(25);
            println!("Add thread {} completed", i);
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final counter value: {}", counter.get());
    // Expected: (5 * 100) - (3 * 50) + (2 * 25) = 500 - 150 + 50 = 400
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mutex_basic() {
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

        assert_eq!(*counter.lock().unwrap(), 10);
    }

    #[test]
    fn test_rwlock_multiple_readers() {
        let data = Arc::new(RwLock::new(42));
        let mut handles = vec![];

        // Multiple readers should be able to read simultaneously
        for _ in 0..5 {
            let data = Arc::clone(&data);
            let handle = thread::spawn(move || {
                let reader = data.read().unwrap();
                *reader
            });
            handles.push(handle);
        }

        let results: Vec<i32> = handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect();

        assert_eq!(results, vec![42; 5]);
    }

    #[test]
    fn test_rwlock_writer() {
        let data = Arc::new(RwLock::new(0));
        let data_clone = Arc::clone(&data);

        let writer = thread::spawn(move || {
            let mut writer = data_clone.write().unwrap();
            *writer = 42;
        });

        writer.join().unwrap();

        let reader = data.read().unwrap();
        assert_eq!(*reader, 42);
    }

    #[test]
    fn test_thread_safe_counter() {
        let counter = ThreadSafeCounter::new();
        let mut handles = vec![];

        for _ in 0..10 {
            let counter = counter.clone();
            let handle = thread::spawn(move || {
                for _ in 0..100 {
                    counter.increment();
                }
            });
            handles.push(handle);
        }

        for handle in handles {
            handle.join().unwrap();
        }

        assert_eq!(counter.get(), 1000);
    }

    #[test]
    fn test_condvar_notification() {
        let pair = Arc::new((Mutex::new(false), Condvar::new()));
        let pair2 = Arc::clone(&pair);

        let waiter = thread::spawn(move || {
            let (lock, cvar) = &*pair2;
            let mut started = lock.lock().unwrap();
            while !*started {
                started = cvar.wait(started).unwrap();
            }
            *started
        });

        // Give waiter time to start waiting
        thread::sleep(Duration::from_millis(10));

        let (lock, cvar) = &*pair;
        let mut started = lock.lock().unwrap();
        *started = true;
        cvar.notify_one();

        let result = waiter.join().unwrap();
        assert!(result);
    }

    #[test]
    fn test_mutex_poisoning_recovery() {
        let mutex = Arc::new(Mutex::new(42));
        let mutex_clone = Arc::clone(&mutex);

        let handle = thread::spawn(move || {
            let _data = mutex_clone.lock().unwrap();
            panic!("Intentional panic");
        });

        // Thread should panic
        assert!(handle.join().is_err());

        // Mutex should be poisoned but recoverable
        match mutex.lock() {
            Ok(_) => panic!("Mutex should be poisoned"),
            Err(poisoned) => {
                let data = poisoned.into_inner();
                assert_eq!(*data, 42);
            }
        }
    }
}