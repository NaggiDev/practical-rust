//! Example usage of our custom async runtime.
//! 
//! This demonstrates how to use the executor to run async tasks,
//! including timers and cooperative multitasking.

use custom_runtime::prelude::*;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::Duration;

fn main() {
    println!("=== Custom Runtime Demo ===\n");

    // Example 1: Simple async tasks
    println!("1. Running simple async tasks:");
    run_simple_tasks();

    println!("\n{}\n", "=".repeat(40));

    // Example 2: Timer-based tasks
    println!("2. Running timer-based tasks:");
    run_timer_tasks();

    println!("\n{}\n", "=".repeat(40));

    // Example 3: Cooperative multitasking
    println!("3. Demonstrating cooperative multitasking:");
    run_cooperative_tasks();

    println!("\n{}\n", "=".repeat(40));

    // Example 4: Running complex async operations
    println!("4. Running complex async operations:");
    run_complex_tasks();

    println!("\n=== Demo Complete ===");
}

/// Demonstrate basic async task execution
fn run_simple_tasks() {
    let mut executor = Executor::new();
    let counter = Arc::new(AtomicUsize::new(0));

    // Spawn several simple tasks
    for i in 0..3 {
        let counter_clone = Arc::clone(&counter);
        executor.spawn(async move {
            println!("  Task {} starting", i);
            counter_clone.fetch_add(1, Ordering::SeqCst);
            println!("  Task {} completed", i);
        });
    }

    println!("  Spawned {} tasks", executor.task_count());
    let completed = executor.run();
    println!("  Completed {} tasks", completed);
    println!("  Final counter value: {}", counter.load(Ordering::SeqCst));
}

/// Demonstrate timer-based async operations
fn run_timer_tasks() {
    let mut executor = Executor::new();

    // Spawn tasks with different timer durations
    executor.spawn(async {
        println!("  Short timer task starting");
        Timer::after_millis(10).await;
        println!("  Short timer task completed");
    });

    executor.spawn(async {
        println!("  Medium timer task starting");
        Timer::after_millis(50).await;
        println!("  Medium timer task completed");
    });

    executor.spawn(async {
        println!("  Long timer task starting");
        Timer::after_millis(100).await;
        println!("  Long timer task completed");
    });

    println!("  Spawned {} timer tasks", executor.task_count());
    let completed = executor.run();
    println!("  Completed {} tasks", completed);
}

/// Demonstrate cooperative multitasking with yield points
fn run_cooperative_tasks() {
    let mut executor = Executor::new();

    // Create tasks that yield control at different points
    for i in 0..3 {
        executor.spawn(async move {
            println!("  Cooperative task {} starting", i);
            
            // Use YieldTimer to demonstrate yielding
            use custom_runtime::timer::YieldTimer;
            YieldTimer::new(2).await;
            
            println!("  Cooperative task {} completed", i);
        });
    }

    println!("  Spawned {} cooperative tasks", executor.task_count());
    
    // Run step by step to see the interleaving
    let mut total_completed = 0;
    let mut iteration = 0;
    
    while executor.has_tasks() {
        iteration += 1;
        println!("  --- Iteration {} ---", iteration);
        println!("    Tasks in queue: {}", executor.task_count());
        println!("    Ready tasks: {}", executor.ready_task_count());
        
        let completed = executor.run_once();
        total_completed += completed;
        
        if completed > 0 {
            println!("    Completed {} tasks this iteration", completed);
        }
        
        // Prevent infinite loops in case of issues
        if iteration > 20 {
            println!("    Breaking after 20 iterations to prevent infinite loop");
            break;
        }
    }
    
    println!("  Total completed: {} tasks", total_completed);
}

/// Demonstrate more complex async patterns
fn run_complex_tasks() {
    let mut executor = Executor::new();
    let shared_data = Arc::new(AtomicUsize::new(0));

    // Task that does multiple async operations
    let data_clone = Arc::clone(&shared_data);
    executor.spawn(async move {
        println!("  Complex task 1: Starting multi-step operation");
        
        // Step 1: Short delay
        Timer::after_millis(20).await;
        data_clone.fetch_add(10, Ordering::SeqCst);
        println!("  Complex task 1: Completed step 1, data = {}", data_clone.load(Ordering::SeqCst));
        
        // Step 2: Another delay
        Timer::after_millis(30).await;
        data_clone.fetch_add(20, Ordering::SeqCst);
        println!("  Complex task 1: Completed step 2, data = {}", data_clone.load(Ordering::SeqCst));
        
        println!("  Complex task 1: All steps completed");
    });

    // Task that waits for the first task to make progress
    let data_clone = Arc::clone(&shared_data);
    executor.spawn(async move {
        println!("  Complex task 2: Waiting for shared data to change");
        
        // Busy wait for data to change (not efficient, but demonstrates polling)
        loop {
            Timer::after_millis(5).await;
            let current_value = data_clone.load(Ordering::SeqCst);
            if current_value >= 30 {
                println!("  Complex task 2: Data reached {}, completing", current_value);
                break;
            }
        }
    });

    // Simple task that runs concurrently
    executor.spawn(async {
        println!("  Simple concurrent task: Starting");
        Timer::after_millis(40).await;
        println!("  Simple concurrent task: Completed");
    });

    println!("  Spawned {} complex tasks", executor.task_count());
    let completed = executor.run();
    println!("  Completed {} tasks", completed);
    println!("  Final shared data value: {}", shared_data.load(Ordering::SeqCst));
}