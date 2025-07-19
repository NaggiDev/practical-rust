use std::time::{Duration, Instant};
use futures::stream::{self, StreamExt};
use std::sync::Arc;

/// Demonstrates the difference between blocking and non-blocking operations
async fn blocking_vs_nonblocking_example() {
    println!("    Blocking vs Non-blocking:");
    
    // BAD: This would block the entire async runtime
    async fn bad_example() {
        println!("      BAD: Using blocking sleep in async context");
        let start = Instant::now();
        // std::thread::sleep(Duration::from_millis(100)); // Don't actually do this!
        // Instead, we'll simulate the concept
        println!("      (Simulated blocking operation - would block runtime)");
        let elapsed = start.elapsed();
        println!("      Blocking operation took: {:?}", elapsed);
    }
    
    // GOOD: Using async sleep
    async fn good_example() {
        println!("      GOOD: Using async sleep");
        let start = Instant::now();
        tokio::time::sleep(Duration::from_millis(100)).await;
        let elapsed = start.elapsed();
        println!("      Async operation took: {:?}", elapsed);
    }
    
    bad_example().await;
    good_example().await;
}

/// Demonstrates sequential vs concurrent execution
async fn sequential_vs_concurrent_example() {
    println!("    Sequential vs Concurrent Execution:");
    
    async fn fetch_data(id: u32) -> String {
        tokio::time::sleep(Duration::from_millis(100)).await;
        format!("Data {}", id)
    }
    
    // Sequential execution
    let start = Instant::now();
    let result1 = fetch_data(1).await;
    let result2 = fetch_data(2).await;
    let result3 = fetch_data(3).await;
    let sequential_time = start.elapsed();
    
    println!("      Sequential results: {}, {}, {}", result1, result2, result3);
    println!("      Sequential time: {:?}", sequential_time);
    
    // Concurrent execution
    let start = Instant::now();
    let (result1, result2, result3) = tokio::join!(
        fetch_data(1),
        fetch_data(2),
        fetch_data(3)
    );
    let concurrent_time = start.elapsed();
    
    println!("      Concurrent results: {}, {}, {}", result1, result2, result3);
    println!("      Concurrent time: {:?}", concurrent_time);
    println!("      Speedup: {:.2}x", sequential_time.as_millis() as f64 / concurrent_time.as_millis() as f64);
}

/// Demonstrates efficient buffering and batching
async fn buffering_example() {
    println!("    Buffering and Batching:");
    
    async fn process_item(item: i32) -> i32 {
        // Simulate some async work
        tokio::time::sleep(Duration::from_millis(10)).await;
        item * 2
    }
    
    let items: Vec<i32> = (0..20).collect();
    
    // Without buffering - processes one at a time
    let start = Instant::now();
    let mut results = Vec::new();
    for item in &items {
        results.push(process_item(*item).await);
    }
    let unbuffered_time = start.elapsed();
    println!("      Unbuffered processing took: {:?}", unbuffered_time);
    
    // With buffering - processes multiple items concurrently
    let start = Instant::now();
    let results: Vec<i32> = stream::iter(items)
        .map(|item| process_item(item))
        .buffer_unordered(5) // Process up to 5 items concurrently
        .collect()
        .await;
    let buffered_time = start.elapsed();
    
    println!("      Buffered processing took: {:?}", buffered_time);
    println!("      Speedup: {:.2}x", unbuffered_time.as_millis() as f64 / buffered_time.as_millis() as f64);
    println!("      Processed {} items", results.len());
}

/// Demonstrates memory-efficient string processing
async fn memory_efficiency_example() {
    println!("    Memory Efficiency:");
    
    // Inefficient: lots of allocations
    async fn inefficient_processing(input: &str) -> String {
        let mut result = String::new();
        for line in input.lines() {
            if !line.is_empty() {
                // Each push_str might reallocate
                result.push_str(&line.to_uppercase());
                result.push('\n');
            }
        }
        result
    }
    
    // Efficient: pre-allocate and minimize allocations
    async fn efficient_processing(input: &str) -> String {
        let lines: Vec<&str> = input.lines().filter(|line| !line.is_empty()).collect();
        let estimated_size = lines.iter().map(|line| line.len()).sum::<usize>() + lines.len();
        
        let mut result = String::with_capacity(estimated_size);
        for line in lines {
            result.push_str(&line.to_uppercase());
            result.push('\n');
        }
        result
    }
    
    let test_input = "hello\nworld\n\nrust\nasync\n\nprogramming\n";
    
    let start = Instant::now();
    let result1 = inefficient_processing(test_input).await;
    let inefficient_time = start.elapsed();
    
    let start = Instant::now();
    let result2 = efficient_processing(test_input).await;
    let efficient_time = start.elapsed();
    
    println!("      Inefficient processing: {:?}", inefficient_time);
    println!("      Efficient processing: {:?}", efficient_time);
    println!("      Results match: {}", result1 == result2);
}

/// Demonstrates shared data patterns
async fn shared_data_example() {
    println!("    Shared Data Patterns:");
    
    // Using Arc for shared immutable data
    let shared_data = Arc::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    
    let mut tasks = Vec::new();
    for i in 0..3 {
        let data = Arc::clone(&shared_data);
        let task = tokio::spawn(async move {
            // Each task can read the shared data without cloning the entire vector
            let sum: i32 = data.iter().sum();
            let avg = sum as f64 / data.len() as f64;
            (i, sum, avg)
        });
        tasks.push(task);
    }
    
    // Wait for all tasks and collect results
    for task in tasks {
        let (task_id, sum, avg) = task.await.unwrap();
        println!("      Task {}: sum = {}, avg = {:.2}", task_id, sum, avg);
    }
}

/// Demonstrates avoiding common async pitfalls
async fn pitfalls_example() {
    println!("    Common Pitfalls and Solutions:");
    
    // Pitfall 1: Holding locks across await points
    println!("      Pitfall 1: Lock management");
    let data = Arc::new(tokio::sync::Mutex::new(vec![1, 2, 3]));
    
    // BAD: Holding lock across await
    // let guard = data.lock().await;
    // tokio::time::sleep(Duration::from_millis(10)).await; // This would hold the lock!
    // drop(guard);
    
    // GOOD: Release lock before await
    let value = {
        let guard = data.lock().await;
        guard[0] // Get the value we need
    }; // Lock is released here
    tokio::time::sleep(Duration::from_millis(10)).await;
    println!("      Retrieved value: {}", value);
    
    // Pitfall 2: Not using spawn_blocking for CPU-intensive work
    println!("      Pitfall 2: CPU-intensive work");
    
    let start = Instant::now();
    let result = tokio::task::spawn_blocking(|| {
        // CPU-intensive work that would block the async runtime
        let mut sum = 0;
        for i in 0..1_000_000 {
            sum += i;
        }
        sum
    }).await.unwrap();
    let elapsed = start.elapsed();
    
    println!("      CPU-intensive task result: {}", result);
    println!("      Time taken: {:?}", elapsed);
}

/// Demonstrates performance monitoring
async fn monitoring_example() {
    println!("    Performance Monitoring:");
    
    async fn monitored_operation(id: u32) -> (u32, Duration) {
        let start = Instant::now();
        
        // Simulate work with varying duration
        let work_duration = Duration::from_millis(50 + (id * 25) % 100);
        tokio::time::sleep(work_duration).await;
        
        let elapsed = start.elapsed();
        (id, elapsed)
    }
    
    let start = Instant::now();
    let mut tasks = Vec::new();
    
    // Spawn multiple monitored operations
    for i in 0..5 {
        tasks.push(tokio::spawn(monitored_operation(i)));
    }
    
    // Collect timing information
    let mut total_work_time = Duration::ZERO;
    for task in tasks {
        let (id, duration) = task.await.unwrap();
        println!("      Operation {} took: {:?}", id, duration);
        total_work_time += duration;
    }
    
    let wall_clock_time = start.elapsed();
    let efficiency = total_work_time.as_millis() as f64 / wall_clock_time.as_millis() as f64;
    
    println!("      Total work time: {:?}", total_work_time);
    println!("      Wall clock time: {:?}", wall_clock_time);
    println!("      Concurrency efficiency: {:.2}x", efficiency);
}

/// Demonstrates resource pooling
async fn resource_pooling_example() {
    println!("    Resource Pooling:");
    
    // Simulate a connection pool
    struct Connection {
        id: u32,
    }
    
    impl Connection {
        fn new(id: u32) -> Self {
            println!("      Creating connection {}", id);
            Connection { id }
        }
        
        async fn query(&self, query: &str) -> String {
            tokio::time::sleep(Duration::from_millis(50)).await;
            format!("Connection {} executed: {}", self.id, query)
        }
    }
    
    // Simple pool implementation
    let pool = Arc::new(tokio::sync::Mutex::new(Vec::new()));
    
    // Initialize pool
    {
        let mut pool_guard = pool.lock().await;
        for i in 0..3 {
            pool_guard.push(Connection::new(i));
        }
    }
    
    async fn use_connection(pool: Arc<tokio::sync::Mutex<Vec<Connection>>>, query: String) -> String {
        // Get connection from pool
        let conn = {
            let mut pool_guard = pool.lock().await;
            pool_guard.pop()
        };
        
        match conn {
            Some(connection) => {
                let result = connection.query(&query).await;
                
                // Return connection to pool
                {
                    let mut pool_guard = pool.lock().await;
                    pool_guard.push(connection);
                }
                
                result
            }
            None => "No connections available".to_string(),
        }
    }
    
    // Use connections concurrently
    let mut tasks = Vec::new();
    for i in 0..5 {
        let pool_clone = Arc::clone(&pool);
        let query = format!("SELECT * FROM table_{}", i);
        tasks.push(tokio::spawn(use_connection(pool_clone, query)));
    }
    
    for task in tasks {
        let result = task.await.unwrap();
        println!("      {}", result);
    }
}

pub async fn run_examples() {
    blocking_vs_nonblocking_example().await;
    sequential_vs_concurrent_example().await;
    buffering_example().await;
    memory_efficiency_example().await;
    shared_data_example().await;
    pitfalls_example().await;
    monitoring_example().await;
    resource_pooling_example().await;
}