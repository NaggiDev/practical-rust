use std::thread;
use std::time::Duration;

/// Demonstrates basic thread creation and joining
pub fn basic_thread_example() {
    println!("=== Basic Thread Example ===");
    
    let handle = thread::spawn(|| {
        for i in 1..6 {
            println!("Thread: count {}", i);
            thread::sleep(Duration::from_millis(100));
        }
        "Thread completed"
    });

    // Do work in main thread
    for i in 1..4 {
        println!("Main: count {}", i);
        thread::sleep(Duration::from_millis(150));
    }

    // Wait for thread to complete and get result
    let result = handle.join().unwrap();
    println!("Thread result: {}", result);
}

/// Demonstrates moving data into threads
pub fn move_data_example() {
    println!("\n=== Move Data Example ===");
    
    let data = vec![1, 2, 3, 4, 5];
    println!("Original data: {:?}", data);
    
    let handle = thread::spawn(move || {
        println!("Thread received: {:?}", data);
        let sum: i32 = data.iter().sum();
        sum
    });

    let result = handle.join().unwrap();
    println!("Sum calculated in thread: {}", result);
}

/// Demonstrates thread builder for custom configuration
pub fn thread_builder_example() {
    println!("\n=== Thread Builder Example ===");
    
    let builder = thread::Builder::new()
        .name("custom-thread".into())
        .stack_size(4 * 1024 * 1024); // 4MB stack

    let handle = builder.spawn(|| {
        println!("Custom thread: {}", thread::current().name().unwrap_or("unnamed"));
        42
    }).unwrap();

    let result = handle.join().unwrap();
    println!("Custom thread result: {}", result);
}

/// Demonstrates spawning multiple threads
pub fn multiple_threads_example() {
    println!("\n=== Multiple Threads Example ===");
    
    let mut handles = vec![];

    for i in 0..5 {
        let handle = thread::spawn(move || {
            println!("Thread {} starting", i);
            thread::sleep(Duration::from_millis(100 * i as u64));
            println!("Thread {} finishing", i);
            i * 2
        });
        handles.push(handle);
    }

    println!("All threads spawned, waiting for completion...");

    for (i, handle) in handles.into_iter().enumerate() {
        let result = handle.join().unwrap();
        println!("Thread {} result: {}", i, result);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_thread_creation() {
        let handle = thread::spawn(|| {
            42
        });

        let result = handle.join().unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_move_data_to_thread() {
        let data = vec![1, 2, 3];
        let expected_sum = data.iter().sum::<i32>();

        let handle = thread::spawn(move || {
            data.iter().sum::<i32>()
        });

        let result = handle.join().unwrap();
        assert_eq!(result, expected_sum);
    }

    #[test]
    fn test_multiple_threads() {
        let mut handles = vec![];

        for i in 0..10 {
            let handle = thread::spawn(move || i * 2);
            handles.push(handle);
        }

        let results: Vec<i32> = handles
            .into_iter()
            .map(|h| h.join().unwrap())
            .collect();

        let expected: Vec<i32> = (0..10).map(|i| i * 2).collect();
        assert_eq!(results, expected);
    }
}