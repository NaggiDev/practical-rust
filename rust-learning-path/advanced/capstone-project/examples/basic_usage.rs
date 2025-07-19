//! Basic usage example for the capstone project.
//!
//! This example demonstrates the fundamental features of the task execution engine.

use capstone_project::*;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("🚀 Capstone Project - Basic Usage Example");
    println!("==========================================");

    // Create a task engine with default configuration
    let engine = TaskEngine::builder()
        .workers(4)
        .queue_size(1000)
        .enable_work_stealing(true)
        .build()?;

    println!("✅ Created task engine with {} workers", engine.config().worker_threads);

    // Example 1: Mathematical operations
    println!("\n📊 Mathematical Operations:");
    
    let factorial_result = engine.submit_math_task(MathOperation::Factorial, vec![5]).await?;
    println!("  5! = {}", factorial_result);

    let fibonacci_result = engine.submit_math_task(MathOperation::Fibonacci, vec![10]).await?;
    println!("  fibonacci(10) = {}", fibonacci_result);

    let gcd_result = engine.submit_math_task(MathOperation::GreatestCommonDivisor, vec![48, 18]).await?;
    println!("  gcd(48, 18) = {}", gcd_result);

    // Example 2: String operations
    println!("\n📝 String Operations:");
    
    let reversed = engine.submit_string_task(StringOperation::Reverse, "hello world".to_string()).await?;
    println!("  reverse('hello world') = '{}'", reversed);

    let uppercase = engine.submit_string_task(StringOperation::Uppercase, "rust is awesome".to_string()).await?;
    println!("  uppercase('rust is awesome') = '{}'", uppercase);

    let hash = engine.submit_string_task(StringOperation::Hash, "capstone project".to_string()).await?;
    println!("  hash('capstone project') = {}", hash);

    // Example 3: Array operations
    println!("\n📋 Array Operations:");
    
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6, 5, 3];
    println!("  input array: {:?}", numbers);

    let sum = engine.submit_array_task(ArrayOperation::Sum, numbers.clone()).await?;
    println!("  sum = {}", sum);

    let max = engine.submit_array_task(ArrayOperation::Max, numbers.clone()).await?;
    println!("  max = {}", max);

    let _sorted_first = engine.submit_array_task(ArrayOperation::Sort, numbers.clone()).await?;
    println!("  sorted (first element after sort) = {}", _sorted_first);

    // Example 4: Batch operations using macros
    println!("\n🔄 Batch Operations (using macros):");
    
    // Using the math_task! macro
    let task1 = math_task!(factorial, 6);
    let task2 = math_task!(fibonacci, 8);
    
    // Using the string_task! macro
    let task3 = string_task!(reverse, "macro magic");
    let task4 = string_task!(uppercase, "batch processing");

    // Submit tasks concurrently
    let (result1, result2, result3, result4) = tokio::join!(
        engine.submit(task1),
        engine.submit(task2),
        engine.submit(task3),
        engine.submit(task4)
    );

    println!("  factorial(6) = {}", result1?);
    println!("  fibonacci(8) = {}", result2?);
    println!("  reverse('macro magic') = '{}'", result3?);
    println!("  uppercase('batch processing') = '{}'", result4?);

    // Example 5: Engine statistics
    println!("\n📈 Engine Statistics:");
    let stats = engine.statistics();
    println!("  Tasks submitted: {}", stats.tasks_submitted);
    println!("  Tasks completed: {}", stats.tasks_completed);
    println!("  Success rate: {:.2}%", stats.success_rate() * 100.0);
    println!("  Average execution time: {:?}", stats.average_execution_time);
    println!("  Throughput: {:.2} tasks/second", stats.throughput_per_second);

    // Graceful shutdown
    println!("\n🛑 Shutting down engine...");
    engine.shutdown().await?;
    println!("✅ Engine shutdown complete");

    Ok(())
}