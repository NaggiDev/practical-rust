//! Example usage and demonstration of the thread pool

use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use thread_pool::ThreadPool;

fn main() {
    println!("=== Thread Pool Demonstration ===\n");

    // Example 1: Basic usage
    println!("1. Basic Thread Pool Usage");
    basic_usage_example();

    println!("\n" + "=".repeat(50).as_str() + "\n");

    // Example 2: Concurrent counter
    println!("2. Concurrent Counter Example");
    concurrent_counter_example();

    println!("\n" + "=".repeat(50).as_str() + "\n");

    // Example 3: CPU-intensive tasks
    println!("3. CPU-Intensive Tasks Example");
    cpu_intensive_example();

    println!("\n" + "=".repeat(50).as_str() + "\n");

    // Example 4: Error handling
    println!("4. Error Handling Example");
    error_handling_example();

    println!("\n=== Demonstration Complete ===");
}

fn basic_usage_example() {
    println!("Creating a thread pool with 4 workers...");
    let pool = ThreadPool::new(4).unwrap();

    println!("Submitting 8 simple tasks...");
    for i in 0..8 {
        pool.execute(move || {
            println!("Task {} is running on thread {:?}", i, thread::current().id());
            thread::sleep(Duration::from_millis(500));
            println!("Task {} completed", i);
        }).unwrap();
    }

    // Give tasks time to complete
    thread::sleep(Duration::from_secs(3));
    println!("Basic usage example completed");
}

fn concurrent_counter_example() {
    println!("Testing thread safety with a shared counter...");
    let pool = ThreadPool::new(3).unwrap();
    let counter = Arc::new(Mutex::new(0));

    let num_tasks = 20;
    println!("Submitting {} increment tasks...", num_tasks);

    for i in 0..num_tasks {
        let counter = Arc::clone(&counter);
        pool.execute(move || {
            // Simulate some work
            thread::sleep(Duration::from_millis(50));
            
            let mut num = counter.lock().unwrap();
            *num += 1;
            println!("Task {} incremented counter to {}", i, *num);
        }).unwrap();
    }

    // Wait for all tasks to complete
    thread::sleep(Duration::from_secs(2));

    let final_count = *counter.lock().unwrap();
    println!("Final counter value: {} (expected: {})", final_count, num_tasks);
    
    if final_count == num_tasks {
        println!("✅ Thread safety test passed!");
    } else {
        println!("❌ Thread safety test failed!");
    }
}

fn cpu_intensive_example() {
    println!("Running CPU-intensive tasks...");
    let pool = ThreadPool::new(2).unwrap();

    let start_time = std::time::Instant::now();

    for i in 0..4 {
        pool.execute(move || {
            println!("Starting CPU-intensive task {}", i);
            
            // Simulate CPU-intensive work (calculating prime numbers)
            let mut count = 0;
            for n in 2..100_000 {
                if is_prime(n) {
                    count += 1;
                }
            }
            
            println!("Task {} found {} prime numbers", i, count);
        }).unwrap();
    }

    // Wait for all tasks to complete
    thread::sleep(Duration::from_secs(5));
    
    let elapsed = start_time.elapsed();
    println!("CPU-intensive tasks completed in {:?}", elapsed);
}

fn error_handling_example() {
    println!("Testing error handling and panic recovery...");
    let pool = ThreadPool::new(2).unwrap();

    // Submit a task that will panic
    pool.execute(|| {
        println!("This task will panic!");
        panic!("Intentional panic for testing");
    }).unwrap();

    // Submit a normal task after the panicking one
    pool.execute(|| {
        println!("This task should still execute despite the previous panic");
        thread::sleep(Duration::from_millis(100));
        println!("Normal task completed successfully");
    }).unwrap();

    // Test invalid pool size
    match ThreadPool::new(0) {
        Ok(_) => println!("❌ Should have failed to create pool with size 0"),
        Err(e) => println!("✅ Correctly rejected pool size 0: {}", e),
    }

    thread::sleep(Duration::from_secs(1));
    println!("Error handling example completed");
}

// Helper function to check if a number is prime
fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=(n as f64).sqrt() as u32 {
        if n % i == 0 {
            return false;
        }
    }
    true
}