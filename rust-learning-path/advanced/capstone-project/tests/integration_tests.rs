//! Integration tests for the capstone project.
//!
//! These tests verify that all components work together correctly.

use capstone_project::*;
use std::time::Duration;

#[tokio::test]
async fn test_engine_basic_functionality() {
    let engine = TaskEngine::new().unwrap();

    // Test mathematical operations
    let factorial = engine.submit_math_task(MathOperation::Factorial, vec![5]).await.unwrap();
    assert_eq!(factorial, 120);

    let fibonacci = engine.submit_math_task(MathOperation::Fibonacci, vec![7]).await.unwrap();
    assert_eq!(fibonacci, 13);

    // Test string operations
    let reversed = engine.submit_string_task(StringOperation::Reverse, "test".to_string()).await.unwrap();
    assert_eq!(reversed, "tset");

    let uppercase = engine.submit_string_task(StringOperation::Uppercase, "hello".to_string()).await.unwrap();
    assert_eq!(uppercase, "HELLO");

    // Test array operations
    let sum = engine.submit_array_task(ArrayOperation::Sum, vec![1, 2, 3, 4, 5]).await.unwrap();
    assert_eq!(sum, 15);

    let max = engine.submit_array_task(ArrayOperation::Max, vec![3, 7, 2, 9, 1]).await.unwrap();
    assert_eq!(max, 9);
}

#[tokio::test]
async fn test_concurrent_task_execution() {
    let engine = TaskEngine::builder()
        .workers(4)
        .queue_size(100)
        .build()
        .unwrap();

    // Submit multiple tasks concurrently
    let mut handles = Vec::new();
    
    for i in 0..20 {
        let engine_clone = &engine;
        let handle = tokio::spawn(async move {
            engine_clone.submit_math_task(MathOperation::Factorial, vec![i % 10]).await
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    let mut results = Vec::new();
    for handle in handles {
        let result = handle.await.unwrap().unwrap();
        results.push(result);
    }

    // Verify we got results for all tasks
    assert_eq!(results.len(), 20);
    
    // Verify some known results
    assert!(results.contains(&1)); // 0! or 1!
    assert!(results.contains(&120)); // 5!
}

#[tokio::test]
async fn test_error_handling() {
    let engine = TaskEngine::new().unwrap();

    // Test invalid factorial (too large)
    let result = engine.submit_math_task(MathOperation::Factorial, vec![25]).await;
    assert!(result.is_err());

    // Test invalid fibonacci (negative)
    let result = engine.submit_math_task(MathOperation::Fibonacci, vec![-1]).await;
    assert!(result.is_err());

    // Test empty string operations should work
    let result = engine.submit_string_task(StringOperation::Hash, "".to_string()).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_engine_statistics() {
    let engine = TaskEngine::new().unwrap();

    // Submit some tasks
    let _ = engine.submit_math_task(MathOperation::Factorial, vec![3]).await;
    let _ = engine.submit_string_task(StringOperation::Reverse, "test".to_string()).await;
    let _ = engine.submit_array_task(ArrayOperation::Sum, vec![1, 2, 3]).await;

    let stats = engine.statistics();
    assert!(stats.tasks_submitted >= 3);
    assert!(stats.tasks_completed >= 3);
    assert!(stats.success_rate() > 0.0);
}

#[tokio::test]
async fn test_engine_configuration() {
    let engine = TaskEngine::builder()
        .workers(8)
        .queue_size(2000)
        .enable_work_stealing(false)
        .task_timeout(Duration::from_secs(60))
        .build()
        .unwrap();

    let config = engine.config();
    assert_eq!(config.worker_threads, 8);
    assert_eq!(config.queue_size, 2000);
    assert_eq!(config.enable_work_stealing, false);
    assert_eq!(config.task_timeout, Some(Duration::from_secs(60)));
}

#[tokio::test]
async fn test_macro_usage() {
    let engine = TaskEngine::new().unwrap();

    // Test math_task! macro
    let task = math_task!(factorial, 4);
    let result = engine.submit(task).await.unwrap();
    assert_eq!(result, 24);

    // Test string_task! macro
    let task = string_task!(uppercase, "macro");
    let result = engine.submit(task).await.unwrap();
    assert_eq!(result, "MACRO");

    // Test array_task! macro
    let task = array_task!(sum, [10, 20, 30]);
    let result = engine.submit(task).await.unwrap();
    assert_eq!(result, 60);
}

#[tokio::test]
async fn test_task_options_macro() {
    let options = task_options! {
        priority: 5,
        timeout: Duration::from_secs(30),
        retry_count: 3,
        memory_mb: 100,
        cpu_cores: 2.0,
    };

    assert_eq!(options.priority, 5);
    assert_eq!(options.timeout, Some(Duration::from_secs(30)));
    assert_eq!(options.retry_count, 3);
    assert_eq!(options.resource_requirements.memory_mb, 100);
    assert_eq!(options.resource_requirements.cpu_cores, 2.0);
}

#[tokio::test]
async fn test_engine_config_macro() {
    let config = engine_config! {
        workers: 6,
        queue_size: 1500,
        enable_work_stealing: true,
        enable_metrics: false,
    };

    assert_eq!(config.worker_threads, 6);
    assert_eq!(config.queue_size, 1500);
    assert_eq!(config.enable_work_stealing, true);
    assert_eq!(config.enable_metrics, false);
}

#[tokio::test]
async fn test_engine_shutdown() {
    let engine = TaskEngine::new().unwrap();

    // Submit a task
    let _ = engine.submit_math_task(MathOperation::Factorial, vec![3]).await.unwrap();

    // Shutdown should complete without error
    engine.shutdown().await.unwrap();
}

#[tokio::test]
async fn test_large_batch_processing() {
    let engine = TaskEngine::builder()
        .workers(8)
        .queue_size(5000)
        .build()
        .unwrap();

    // Submit a large batch of tasks
    let mut handles = Vec::new();
    
    for i in 0..100 {
        let engine_ref = &engine;
        let handle = tokio::spawn(async move {
            let math_result = engine_ref.submit_math_task(MathOperation::Fibonacci, vec![i % 15]).await?;
            let string_result = engine_ref.submit_string_task(StringOperation::Hash, format!("task-{}", i)).await?;
            let array_result = engine_ref.submit_array_task(ArrayOperation::Sum, vec![i, i+1, i+2]).await?;
            
            Ok::<(i64, String, i64), EngineError>((math_result, string_result, array_result))
        });
        handles.push(handle);
    }

    // Wait for all tasks to complete
    let mut completed = 0;
    for handle in handles {
        if handle.await.unwrap().is_ok() {
            completed += 1;
        }
    }

    // Verify most tasks completed successfully
    assert!(completed >= 95); // Allow for some potential failures

    let stats = engine.statistics();
    assert!(stats.tasks_submitted >= 300); // 3 tasks per iteration * 100 iterations
    assert!(stats.success_rate() > 0.9); // At least 90% success rate
}