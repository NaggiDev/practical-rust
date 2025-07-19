use std::time::Duration;
use tokio::task;

/// Demonstrates basic task spawning
async fn basic_spawning_example() {
    println!("    Basic Task Spawning:");
    
    // Spawn a simple task
    let handle = task::spawn(async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        "Task completed"
    });
    
    // Wait for the task to complete
    let result = handle.await.unwrap();
    println!("      {}", result);
    
    // Spawn multiple tasks
    let mut handles = Vec::new();
    for i in 0..3 {
        let handle = task::spawn(async move {
            tokio::time::sleep(Duration::from_millis(50 * i)).await;
            format!("Task {} completed", i)
        });
        handles.push(handle);
    }
    
    // Wait for all tasks
    for handle in handles {
        let result = handle.await.unwrap();
        println!("      {}", result);
    }
}

/// Demonstrates blocking task spawning
async fn blocking_spawning_example() {
    println!("    Blocking Task Spawning:");
    
    let start = std::time::Instant::now();
    
    // Spawn a blocking task that won't block the async runtime
    let blocking_handle = task::spawn_blocking(|| {
        // Simulate CPU-intensive work
        std::thread::sleep(Duration::from_millis(100));
        "Blocking task completed"
    });
    
    // Spawn a regular async task that runs concurrently
    let async_handle = task::spawn(async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        "Async task completed"
    });
    
    // Both should complete around the same time
    let (blocking_result, async_result) = tokio::join!(blocking_handle, async_handle);
    
    let elapsed = start.elapsed();
    println!("      Both tasks completed in {:?}", elapsed);
    println!("      {}", blocking_result.unwrap());
    println!("      {}", async_result.unwrap());
}

/// Demonstrates task local storage
tokio::task_local! {
    static REQUEST_ID: u64;
    static USER_NAME: String;
}

async fn task_local_example() {
    println!("    Task Local Storage:");
    
    // Set task-local values and run some work
    REQUEST_ID.scope(12345, async {
        USER_NAME.scope("Alice".to_string(), async {
            process_request().await;
            log_operation().await;
        }).await;
    }).await;
    
    // Different task-local context
    REQUEST_ID.scope(67890, async {
        USER_NAME.scope("Bob".to_string(), async {
            process_request().await;
            log_operation().await;
        }).await;
    }).await;
}

async fn process_request() {
    REQUEST_ID.with(|id| {
        USER_NAME.with(|name| {
            println!("      Processing request {} for user {}", id, name);
        });
    });
}

async fn log_operation() {
    REQUEST_ID.with(|id| {
        USER_NAME.with(|name| {
            println!("      Logging operation for request {} (user: {})", id, name);
        });
    });
}

/// Demonstrates task yielding
async fn yielding_example() {
    println!("    Task Yielding:");
    
    let start = std::time::Instant::now();
    
    // A task that yields control periodically
    let yielding_task = task::spawn(async {
        for i in 0..5 {
            println!("      Yielding task iteration {}", i);
            // Yield control to other tasks
            tokio::task::yield_now().await;
        }
        "Yielding task completed"
    });
    
    // Another task that runs concurrently
    let other_task = task::spawn(async {
        for i in 0..3 {
            println!("      Other task iteration {}", i);
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        "Other task completed"
    });
    
    let (result1, result2) = tokio::join!(yielding_task, other_task);
    
    let elapsed = start.elapsed();
    println!("      Tasks completed in {:?}", elapsed);
    println!("      {}", result1.unwrap());
    println!("      {}", result2.unwrap());
}

/// Demonstrates task abortion
async fn abortion_example() {
    println!("    Task Abortion:");
    
    // Spawn a long-running task
    let handle = task::spawn(async {
        for i in 0..10 {
            println!("      Long task iteration {}", i);
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        "Long task completed"
    });
    
    // Let it run for a bit, then abort it
    tokio::time::sleep(Duration::from_millis(250)).await;
    handle.abort();
    
    // Check if it was aborted
    match handle.await {
        Ok(result) => println!("      Task completed: {}", result),
        Err(e) if e.is_cancelled() => println!("      Task was aborted"),
        Err(e) => println!("      Task failed: {}", e),
    }
}

/// Demonstrates task priorities (using spawn_local for single-threaded context)
async fn priority_example() {
    println!("    Task Scheduling Order:");
    
    // Spawn tasks in different orders to see scheduling behavior
    let handles: Vec<_> = (0..5).map(|i| {
        task::spawn(async move {
            // Different delays to see interleaving
            tokio::time::sleep(Duration::from_millis(10 * (5 - i))).await;
            println!("      Task {} completed", i);
            i
        })
    }).collect();
    
    // Wait for all tasks
    for handle in handles {
        handle.await.unwrap();
    }
}

pub async fn run_examples() {
    basic_spawning_example().await;
    blocking_spawning_example().await;
    task_local_example().await;
    yielding_example().await;
    abortion_example().await;
    priority_example().await;
}