use std::time::Duration;
use tokio::time::timeout;
use tokio_util::sync::CancellationToken;

/// Demonstrates basic timeout functionality
async fn timeout_example() {
    println!("    Timeout Examples:");
    
    // Operation that completes within timeout
    match timeout(Duration::from_millis(200), quick_operation()).await {
        Ok(result) => println!("      Quick operation result: {}", result),
        Err(_) => println!("      Quick operation timed out"),
    }
    
    // Operation that times out
    match timeout(Duration::from_millis(100), slow_operation()).await {
        Ok(result) => println!("      Slow operation result: {}", result),
        Err(_) => println!("      Slow operation timed out (expected)"),
    }
}

async fn quick_operation() -> String {
    tokio::time::sleep(Duration::from_millis(50)).await;
    "Quick operation completed".to_string()
}

async fn slow_operation() -> String {
    tokio::time::sleep(Duration::from_millis(200)).await;
    "Slow operation completed".to_string()
}

/// Demonstrates timeout with error handling
async fn timeout_with_errors_example() {
    println!("    Timeout with Error Handling:");
    
    let result = timeout(Duration::from_millis(150), fallible_operation()).await;
    match result {
        Ok(Ok(value)) => println!("      Operation succeeded: {}", value),
        Ok(Err(e)) => println!("      Operation failed: {}", e),
        Err(_) => println!("      Operation timed out"),
    }
}

async fn fallible_operation() -> Result<i32, &'static str> {
    tokio::time::sleep(Duration::from_millis(100)).await;
    // Simulate random success/failure
    if std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis() % 2 == 0 {
        Ok(42)
    } else {
        Err("Operation failed")
    }
}

/// Demonstrates cancellation tokens
async fn cancellation_token_example() {
    println!("    Cancellation Token:");
    
    let token = CancellationToken::new();
    let child_token = token.child_token();
    
    // Spawn a task that can be cancelled
    let task = tokio::spawn(async move {
        let mut counter = 0;
        loop {
            tokio::select! {
                _ = child_token.cancelled() => {
                    println!("      Task was cancelled after {} iterations", counter);
                    break;
                }
                _ = tokio::time::sleep(Duration::from_millis(50)) => {
                    counter += 1;
                    println!("      Task iteration {}", counter);
                    if counter >= 10 {
                        println!("      Task completed naturally");
                        break;
                    }
                }
            }
        }
    });
    
    // Cancel the task after some time
    tokio::time::sleep(Duration::from_millis(200)).await;
    token.cancel();
    
    // Wait for the task to complete
    task.await.unwrap();
}

/// Demonstrates hierarchical cancellation
async fn hierarchical_cancellation_example() {
    println!("    Hierarchical Cancellation:");
    
    let parent_token = CancellationToken::new();
    
    // Create child tokens
    let child1_token = parent_token.child_token();
    let child2_token = parent_token.child_token();
    
    // Spawn tasks with child tokens
    let task1 = tokio::spawn(async move {
        cancellable_worker("Worker 1", child1_token).await;
    });
    
    let task2 = tokio::spawn(async move {
        cancellable_worker("Worker 2", child2_token).await;
    });
    
    // Let workers run for a bit
    tokio::time::sleep(Duration::from_millis(150)).await;
    
    // Cancel all children by cancelling the parent
    println!("      Cancelling parent token...");
    parent_token.cancel();
    
    // Wait for all tasks to complete
    let _ = tokio::join!(task1, task2);
}

async fn cancellable_worker(name: &str, token: CancellationToken) {
    let mut counter = 0;
    loop {
        tokio::select! {
            _ = token.cancelled() => {
                println!("      {} was cancelled after {} iterations", name, counter);
                break;
            }
            _ = tokio::time::sleep(Duration::from_millis(50)) => {
                counter += 1;
                println!("      {} iteration {}", name, counter);
            }
        }
    }
}

/// Demonstrates select-based cancellation
async fn select_cancellation_example() {
    println!("    Select-based Cancellation:");
    
    let (cancel_tx, mut cancel_rx) = tokio::sync::mpsc::channel::<()>(1);
    
    // Spawn a task that listens for cancellation
    let task = tokio::spawn(async move {
        let mut counter = 0;
        loop {
            tokio::select! {
                _ = cancel_rx.recv() => {
                    println!("      Received cancellation signal after {} iterations", counter);
                    break;
                }
                _ = tokio::time::sleep(Duration::from_millis(50)) => {
                    counter += 1;
                    println!("      Select task iteration {}", counter);
                    if counter >= 10 {
                        println!("      Select task completed naturally");
                        break;
                    }
                }
            }
        }
    });
    
    // Send cancellation signal after some time
    tokio::time::sleep(Duration::from_millis(175)).await;
    let _ = cancel_tx.send(()).await;
    
    // Wait for the task to complete
    task.await.unwrap();
}

/// Demonstrates graceful shutdown pattern
async fn graceful_shutdown_example() {
    println!("    Graceful Shutdown:");
    
    let (shutdown_tx, shutdown_rx) = tokio::sync::broadcast::channel(1);
    
    // Spawn multiple worker tasks
    let mut tasks = Vec::new();
    for i in 0..3 {
        let mut shutdown_rx = shutdown_rx.resubscribe();
        let task = tokio::spawn(async move {
            let mut counter = 0;
            loop {
                tokio::select! {
                    _ = shutdown_rx.recv() => {
                        println!("      Worker {} shutting down gracefully after {} iterations", i, counter);
                        // Perform cleanup here
                        tokio::time::sleep(Duration::from_millis(25)).await; // Simulate cleanup
                        println!("      Worker {} cleanup completed", i);
                        break;
                    }
                    _ = tokio::time::sleep(Duration::from_millis(75)) => {
                        counter += 1;
                        println!("      Worker {} iteration {}", i, counter);
                    }
                }
            }
        });
        tasks.push(task);
    }
    
    // Simulate shutdown signal after some time
    tokio::time::sleep(Duration::from_millis(200)).await;
    println!("      Sending shutdown signal...");
    let _ = shutdown_tx.send(());
    
    // Wait for all workers to shut down gracefully
    for task in tasks {
        task.await.unwrap();
    }
    
    println!("      All workers have shut down gracefully");
}

pub async fn run_examples() {
    timeout_example().await;
    timeout_with_errors_example().await;
    cancellation_token_example().await;
    hierarchical_cancellation_example().await;
    select_cancellation_example().await;
    graceful_shutdown_example().await;
}