use std::time::Duration;
use tokio::runtime::Runtime;

/// Demonstrates manual runtime creation and usage
fn manual_runtime_example() {
    println!("    Manual Runtime:");
    let rt = Runtime::new().unwrap();
    
    rt.block_on(async {
        println!("      Running on manually created Tokio runtime");
        tokio::time::sleep(Duration::from_millis(50)).await;
        println!("      Manual runtime task completed");
    });
}

/// Demonstrates custom runtime configuration
fn custom_runtime_example() {
    println!("    Custom Runtime Configuration:");
    let rt = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(2)
        .thread_name("custom-worker")
        .thread_stack_size(3 * 1024 * 1024)
        .enable_all()
        .build()
        .unwrap();
    
    rt.block_on(async {
        println!("      Running on custom configured runtime");
        
        // Spawn multiple tasks to demonstrate multi-threading
        let mut handles = Vec::new();
        for i in 0..4 {
            let handle = tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(100)).await;
                format!("Task {} completed on thread {:?}", i, std::thread::current().id())
            });
            handles.push(handle);
        }
        
        // Wait for all tasks
        for handle in handles {
            let result = handle.await.unwrap();
            println!("      {}", result);
        }
    });
}

/// Demonstrates current thread runtime
fn current_thread_runtime_example() {
    println!("    Current Thread Runtime:");
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();
    
    rt.block_on(async {
        println!("      Running on current thread runtime");
        
        // All tasks will run on the same thread
        let handle1 = tokio::spawn(async {
            tokio::time::sleep(Duration::from_millis(50)).await;
            format!("Task 1 on thread {:?}", std::thread::current().id())
        });
        
        let handle2 = tokio::spawn(async {
            tokio::time::sleep(Duration::from_millis(50)).await;
            format!("Task 2 on thread {:?}", std::thread::current().id())
        });
        
        let (result1, result2) = tokio::join!(handle1, handle2);
        println!("      {}", result1.unwrap());
        println!("      {}", result2.unwrap());
    });
}

/// Demonstrates runtime handle usage
async fn runtime_handle_example() {
    println!("    Runtime Handle:");
    let handle = tokio::runtime::Handle::current();
    
    // Spawn a task on the current runtime from within an async context
    let spawned = handle.spawn(async {
        tokio::time::sleep(Duration::from_millis(50)).await;
        "Spawned task completed"
    });
    
    let result = spawned.await.unwrap();
    println!("      {}", result);
}

/// Demonstrates blocking task execution
async fn blocking_task_example() {
    println!("    Blocking Tasks:");
    
    // CPU-intensive work that would block the async runtime
    let blocking_result = tokio::task::spawn_blocking(|| {
        // Simulate CPU-intensive work
        let mut sum = 0;
        for i in 0..1_000_000 {
            sum += i;
        }
        sum
    }).await.unwrap();
    
    println!("      Blocking task result: {}", blocking_result);
    
    // Compare with regular spawn (this would block the runtime if it did CPU work)
    let async_result = tokio::spawn(async {
        // This is fine because we're using async sleep, not blocking sleep
        tokio::time::sleep(Duration::from_millis(50)).await;
        "Async task completed"
    }).await.unwrap();
    
    println!("      Async task result: {}", async_result);
}

/// Demonstrates different runtime types comparison
async fn runtime_comparison() {
    println!("    Runtime Comparison:");
    
    // Tokio (already running in tokio context)
    let start = std::time::Instant::now();
    tokio::time::sleep(Duration::from_millis(50)).await;
    println!("      Tokio sleep took: {:?}", start.elapsed());
    
    // Note: We can't easily demonstrate async-std and smol here since we're already
    // in a tokio context, but here's how you would use them:
    
    println!("      Other runtimes would be used like:");
    println!("        async-std: async_std::task::sleep(Duration::from_millis(50)).await");
    println!("        smol: smol::Timer::after(Duration::from_millis(50)).await");
}

pub async fn run_examples() {
    // Manual runtime (this will create a new runtime)
    manual_runtime_example();
    
    // Custom runtime configuration
    custom_runtime_example();
    
    // Current thread runtime
    current_thread_runtime_example();
    
    // Runtime handle (using current runtime)
    runtime_handle_example().await;
    
    // Blocking tasks
    blocking_task_example().await;
    
    // Runtime comparison
    runtime_comparison().await;
}