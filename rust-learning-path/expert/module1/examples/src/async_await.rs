use std::time::Duration;

/// Simulates fetching data from a remote source
async fn fetch_data(source: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Simulate network delay
    tokio::time::sleep(Duration::from_millis(100)).await;
    
    // Simulate potential failure
    if source == "bad_source" {
        return Err("Failed to fetch data".into());
    }
    
    Ok(format!("Data from {}", source))
}

/// Processes data asynchronously
async fn process_data(data: String) -> String {
    tokio::time::sleep(Duration::from_millis(50)).await;
    format!("Processed: {}", data)
}

/// Demonstrates error handling in async functions
async fn handle_errors() -> Result<(), Box<dyn std::error::Error>> {
    // Using ? operator with async operations
    let data = fetch_data("good_source").await?;
    println!("    Successfully fetched: {}", data);
    
    // Pattern matching with async results
    match fetch_data("bad_source").await {
        Ok(result) => println!("    Unexpected success: {}", result),
        Err(e) => println!("    Expected error: {}", e),
    }
    
    Ok(())
}

/// Demonstrates async blocks
async fn async_blocks_example() {
    let future = async {
        let data = fetch_data("source1").await.unwrap();
        process_data(data).await
    };
    
    let result = future.await;
    println!("    Async block result: {}", result);
}

/// Demonstrates async closures (using async move)
async fn async_closures_example() {
    let data = "shared_data".to_string();
    
    let future = async move {
        tokio::time::sleep(Duration::from_millis(50)).await;
        format!("Processed: {}", data)
    };
    
    let result = future.await;
    println!("    Async closure result: {}", result);
}

/// Demonstrates chaining async operations
async fn chaining_example() {
    let result = fetch_data("source")
        .await
        .and_then(|data| async move { Ok(process_data(data).await) }.await)
        .unwrap_or_else(|e| format!("Error: {}", e));
    
    println!("    Chained operations result: {}", result);
}

pub async fn run_examples() {
    println!("  Basic Async/Await:");
    
    // Simple async function call
    match fetch_data("example").await {
        Ok(data) => println!("    Fetched: {}", data),
        Err(e) => println!("    Error: {}", e),
    }
    
    // Error handling
    if let Err(e) = handle_errors().await {
        println!("    Error in handle_errors: {}", e);
    }
    
    // Async blocks
    async_blocks_example().await;
    
    // Async closures
    async_closures_example().await;
    
    // Chaining operations
    chaining_example().await;
    
    // Concurrent operations
    println!("  Concurrent Operations:");
    let start = std::time::Instant::now();
    
    let (result1, result2, result3) = tokio::join!(
        fetch_data("source1"),
        fetch_data("source2"),
        fetch_data("source3")
    );
    
    let elapsed = start.elapsed();
    println!("    Three concurrent fetches completed in {:?}", elapsed);
    println!("    Results: {:?}, {:?}, {:?}", result1, result2, result3);
}