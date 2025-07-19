use std::time::{Duration, Instant};
use tokio::time::timeout;
use futures::stream::{self, StreamExt};
use tokio_util::sync::CancellationToken;

/// Test basic async/await functionality
#[tokio::test]
async fn test_basic_async_await() {
    async fn simple_async_function() -> i32 {
        tokio::time::sleep(Duration::from_millis(10)).await;
        42
    }
    
    let result = simple_async_function().await;
    assert_eq!(result, 42);
}

/// Test error handling in async functions
#[tokio::test]
async fn test_async_error_handling() {
    async fn fallible_function(should_fail: bool) -> Result<String, &'static str> {
        tokio::time::sleep(Duration::from_millis(10)).await;
        if should_fail {
            Err("Something went wrong")
        } else {
            Ok("Success".to_string())
        }
    }
    
    // Test success case
    let result = fallible_function(false).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "Success");
    
    // Test error case
    let result = fallible_function(true).await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Something went wrong");
}

/// Test concurrent execution with tokio::join!
#[tokio::test]
async fn test_concurrent_execution() {
    async fn timed_task(id: u32, duration_ms: u64) -> (u32, Duration) {
        let start = Instant::now();
        tokio::time::sleep(Duration::from_millis(duration_ms)).await;
        (id, start.elapsed())
    }
    
    let start = Instant::now();
    let (result1, result2, result3) = tokio::join!(
        timed_task(1, 50),
        timed_task(2, 50),
        timed_task(3, 50)
    );
    let total_time = start.elapsed();
    
    // All tasks should complete
    assert_eq!(result1.0, 1);
    assert_eq!(result2.0, 2);
    assert_eq!(result3.0, 3);
    
    // Total time should be roughly the same as individual task time (concurrent execution)
    assert!(total_time < Duration::from_millis(100)); // Should be much less than 150ms (3 * 50ms)
}

/// Test timeout functionality
#[tokio::test]
async fn test_timeout() {
    async fn quick_task() -> &'static str {
        tokio::time::sleep(Duration::from_millis(10)).await;
        "completed"
    }
    
    async fn slow_task() -> &'static str {
        tokio::time::sleep(Duration::from_millis(200)).await;
        "completed"
    }
    
    // Quick task should complete within timeout
    let result = timeout(Duration::from_millis(50), quick_task()).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "completed");
    
    // Slow task should timeout
    let result = timeout(Duration::from_millis(50), slow_task()).await;
    assert!(result.is_err());
}

/// Test cancellation with CancellationToken
#[tokio::test]
async fn test_cancellation() {
    let token = CancellationToken::new();
    let child_token = token.child_token();
    
    let task = tokio::spawn(async move {
        let mut counter = 0;
        loop {
            tokio::select! {
                _ = child_token.cancelled() => {
                    return counter;
                }
                _ = tokio::time::sleep(Duration::from_millis(10)) => {
                    counter += 1;
                    if counter >= 100 {
                        return counter; // Shouldn't reach here due to cancellation
                    }
                }
            }
        }
    });
    
    // Let the task run for a bit, then cancel
    tokio::time::sleep(Duration::from_millis(25)).await;
    token.cancel();
    
    let result = task.await.unwrap();
    assert!(result < 100); // Should be cancelled before reaching 100
    assert!(result > 0);   // Should have run for some iterations
}

/// Test stream processing
#[tokio::test]
async fn test_stream_processing() {
    let stream = stream::iter(vec![1, 2, 3, 4, 5]);
    
    // Test basic collection
    let items: Vec<i32> = stream.collect().await;
    assert_eq!(items, vec![1, 2, 3, 4, 5]);
    
    // Test stream transformation
    let stream = stream::iter(vec![1, 2, 3, 4, 5]);
    let doubled: Vec<i32> = stream
        .map(|x| x * 2)
        .filter(|&x| x > 4)
        .collect()
        .await;
    assert_eq!(doubled, vec![6, 8, 10]);
}

/// Test async stream processing with buffer_unordered
#[tokio::test]
async fn test_async_stream_processing() {
    async fn process_item(item: i32) -> i32 {
        tokio::time::sleep(Duration::from_millis(10)).await;
        item * item
    }
    
    let start = Instant::now();
    let results: Vec<i32> = stream::iter(vec![1, 2, 3, 4, 5])
        .map(|item| process_item(item))
        .buffer_unordered(3) // Process up to 3 items concurrently
        .collect()
        .await;
    let elapsed = start.elapsed();
    
    // Results should contain all squared values (order may vary due to buffer_unordered)
    let mut sorted_results = results;
    sorted_results.sort();
    assert_eq!(sorted_results, vec![1, 4, 9, 16, 25]);
    
    // Should complete faster than sequential processing
    assert!(elapsed < Duration::from_millis(40)); // Much less than 5 * 10ms
}

/// Test task spawning and joining
#[tokio::test]
async fn test_task_spawning() {
    let mut handles = Vec::new();
    
    // Spawn multiple tasks
    for i in 0..5 {
        let handle = tokio::spawn(async move {
            tokio::time::sleep(Duration::from_millis(10)).await;
            i * 2
        });
        handles.push(handle);
    }
    
    // Wait for all tasks and collect results
    let mut results = Vec::new();
    for handle in handles {
        let result = handle.await.unwrap();
        results.push(result);
    }
    
    assert_eq!(results, vec![0, 2, 4, 6, 8]);
}

/// Test blocking task execution
#[tokio::test]
async fn test_blocking_tasks() {
    let result = tokio::task::spawn_blocking(|| {
        // Simulate CPU-intensive work
        let mut sum = 0;
        for i in 0..1000 {
            sum += i;
        }
        sum
    }).await.unwrap();
    
    assert_eq!(result, 499500); // Sum of 0..1000
}

/// Test select! macro
#[tokio::test]
async fn test_select() {
    let result = tokio::select! {
        val = async {
            tokio::time::sleep(Duration::from_millis(10)).await;
            "first"
        } => val,
        val = async {
            tokio::time::sleep(Duration::from_millis(50)).await;
            "second"
        } => val,
    };
    
    // First future should complete first
    assert_eq!(result, "first");
}

/// Test graceful shutdown pattern
#[tokio::test]
async fn test_graceful_shutdown() {
    let (shutdown_tx, mut shutdown_rx) = tokio::sync::mpsc::channel::<()>(1);
    
    let task = tokio::spawn(async move {
        let mut counter = 0;
        loop {
            tokio::select! {
                _ = shutdown_rx.recv() => {
                    return counter;
                }
                _ = tokio::time::sleep(Duration::from_millis(10)) => {
                    counter += 1;
                    if counter >= 100 {
                        return counter; // Shouldn't reach here
                    }
                }
            }
        }
    });
    
    // Let the task run for a bit
    tokio::time::sleep(Duration::from_millis(25)).await;
    
    // Send shutdown signal
    shutdown_tx.send(()).await.unwrap();
    
    let result = task.await.unwrap();
    assert!(result > 0);   // Should have run some iterations
    assert!(result < 100); // Should have been shut down before completing
}

/// Test error propagation in async contexts
#[tokio::test]
async fn test_error_propagation() {
    async fn operation_that_fails() -> Result<i32, &'static str> {
        tokio::time::sleep(Duration::from_millis(10)).await;
        Err("Operation failed")
    }
    
    async fn operation_that_succeeds() -> Result<i32, &'static str> {
        tokio::time::sleep(Duration::from_millis(10)).await;
        Ok(42)
    }
    
    async fn chain_operations() -> Result<i32, &'static str> {
        let _result1 = operation_that_succeeds().await?;
        let _result2 = operation_that_fails().await?; // This should cause early return
        Ok(100) // Should never reach here
    }
    
    let result = chain_operations().await;
    assert!(result.is_err());
    assert_eq!(result.unwrap_err(), "Operation failed");
}

/// Test async closure and move semantics
#[tokio::test]
async fn test_async_closures() {
    let data = "test_data".to_string();
    
    let future = async move {
        tokio::time::sleep(Duration::from_millis(10)).await;
        format!("Processed: {}", data)
    };
    
    let result = future.await;
    assert_eq!(result, "Processed: test_data");
}

/// Test performance characteristics
#[tokio::test]
async fn test_performance_characteristics() {
    // Test that concurrent operations are actually faster than sequential
    async fn work_item(duration_ms: u64) -> u64 {
        tokio::time::sleep(Duration::from_millis(duration_ms)).await;
        duration_ms
    }
    
    // Sequential execution
    let start = Instant::now();
    let _r1 = work_item(20).await;
    let _r2 = work_item(20).await;
    let _r3 = work_item(20).await;
    let sequential_time = start.elapsed();
    
    // Concurrent execution
    let start = Instant::now();
    let (_r1, _r2, _r3) = tokio::join!(
        work_item(20),
        work_item(20),
        work_item(20)
    );
    let concurrent_time = start.elapsed();
    
    // Concurrent should be significantly faster
    assert!(concurrent_time < sequential_time);
    assert!(concurrent_time < Duration::from_millis(40)); // Should be close to 20ms, not 60ms
}