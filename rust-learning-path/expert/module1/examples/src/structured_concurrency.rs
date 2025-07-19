use std::time::Duration;
use tokio::task::JoinSet;

/// Demonstrates basic structured concurrency with JoinSet
async fn join_set_example() {
    println!("    JoinSet Example:");
    
    let mut set = JoinSet::new();
    
    // Spawn multiple tasks
    for i in 0..5 {
        set.spawn(async move {
            let delay = Duration::from_millis(50 * (i + 1));
            tokio::time::sleep(delay).await;
            format!("Task {} completed after {:?}", i, delay)
        });
    }
    
    // Wait for all tasks to complete
    while let Some(result) = set.join_next().await {
        match result {
            Ok(message) => println!("      {}", message),
            Err(e) => eprintln!("      Task failed: {}", e),
        }
    }
}

/// Demonstrates error handling in structured concurrency
async fn error_handling_example() {
    println!("    Error Handling in Structured Concurrency:");
    
    let mut set = JoinSet::new();
    
    // Spawn tasks, some of which will fail
    for i in 0..5 {
        set.spawn(async move {
            tokio::time::sleep(Duration::from_millis(50)).await;
            if i == 2 {
                panic!("Task {} panicked!", i);
            } else if i == 3 {
                return Err(format!("Task {} returned an error", i));
            }
            Ok(format!("Task {} succeeded", i))
        });
    }
    
    // Collect results, handling both panics and errors
    let mut successes = Vec::new();
    let mut failures = Vec::new();
    
    while let Some(result) = set.join_next().await {
        match result {
            Ok(Ok(message)) => {
                println!("      Success: {}", message);
                successes.push(message);
            }
            Ok(Err(error)) => {
                println!("      Task error: {}", error);
                failures.push(error);
            }
            Err(join_error) => {
                if join_error.is_panic() {
                    println!("      Task panicked: {:?}", join_error);
                } else if join_error.is_cancelled() {
                    println!("      Task was cancelled");
                } else {
                    println!("      Task join error: {}", join_error);
                }
                failures.push(format!("Join error: {}", join_error));
            }
        }
    }
    
    println!("      Summary: {} successes, {} failures", successes.len(), failures.len());
}

/// Demonstrates aborting tasks in a JoinSet
async fn abort_example() {
    println!("    Task Abortion Example:");
    
    let mut set = JoinSet::new();
    
    // Spawn long-running tasks
    for i in 0..3 {
        set.spawn(async move {
            for j in 0..10 {
                println!("      Task {} iteration {}", i, j);
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
            format!("Task {} completed all iterations", i)
        });
    }
    
    // Let tasks run for a bit, then abort them all
    tokio::time::sleep(Duration::from_millis(250)).await;
    println!("      Aborting all tasks...");
    set.abort_all();
    
    // Check results
    while let Some(result) = set.join_next().await {
        match result {
            Ok(message) => println!("      Completed: {}", message),
            Err(e) if e.is_cancelled() => println!("      Task was aborted"),
            Err(e) => println!("      Task error: {}", e),
        }
    }
}

/// Demonstrates scoped tasks (simulated with proper cleanup)
async fn scoped_tasks_example() {
    println!("    Scoped Tasks Example:");
    
    // This function ensures all spawned tasks complete before returning
    async fn scoped_work() -> Vec<String> {
        let mut set = JoinSet::new();
        let mut results = Vec::new();
        
        // Spawn tasks within this scope
        for i in 0..4 {
            set.spawn(async move {
                tokio::time::sleep(Duration::from_millis(50 * (i + 1))).await;
                format!("Scoped task {} result", i)
            });
        }
        
        // All tasks must complete before this function returns
        while let Some(result) = set.join_next().await {
            match result {
                Ok(message) => {
                    println!("      {}", message);
                    results.push(message);
                }
                Err(e) => println!("      Scoped task error: {}", e),
            }
        }
        
        results
    }
    
    let results = scoped_work().await;
    println!("      All scoped tasks completed. Results: {}", results.len());
}

/// Demonstrates resource cleanup with structured concurrency
async fn resource_cleanup_example() {
    println!("    Resource Cleanup Example:");
    
    struct Resource {
        id: u32,
    }
    
    impl Resource {
        fn new(id: u32) -> Self {
            println!("      Resource {} created", id);
            Resource { id }
        }
    }
    
    impl Drop for Resource {
        fn drop(&mut self) {
            println!("      Resource {} cleaned up", self.id);
        }
    }
    
    async fn work_with_resource(resource: Resource) -> String {
        tokio::time::sleep(Duration::from_millis(100)).await;
        format!("Work completed with resource {}", resource.id)
    }
    
    // Resources will be properly cleaned up even if tasks are cancelled
    let mut set = JoinSet::new();
    
    for i in 0..3 {
        let resource = Resource::new(i);
        set.spawn(async move {
            work_with_resource(resource).await
        });
    }
    
    // Let some tasks complete, then abort the rest
    tokio::time::sleep(Duration::from_millis(150)).await;
    set.abort_all();
    
    while let Some(result) = set.join_next().await {
        match result {
            Ok(message) => println!("      {}", message),
            Err(e) if e.is_cancelled() => println!("      Task cancelled (resource cleaned up)"),
            Err(e) => println!("      Task error: {}", e),
        }
    }
}

/// Demonstrates hierarchical task management
async fn hierarchical_tasks_example() {
    println!("    Hierarchical Task Management:");
    
    async fn parent_task(id: u32) -> String {
        println!("      Parent task {} starting", id);
        
        let mut child_set = JoinSet::new();
        
        // Spawn child tasks
        for i in 0..3 {
            let child_id = id * 10 + i;
            child_set.spawn(async move {
                tokio::time::sleep(Duration::from_millis(50)).await;
                format!("Child task {} completed", child_id)
            });
        }
        
        // Wait for all children
        let mut child_results = Vec::new();
        while let Some(result) = child_set.join_next().await {
            match result {
                Ok(message) => {
                    println!("        {}", message);
                    child_results.push(message);
                }
                Err(e) => println!("        Child task error: {}", e),
            }
        }
        
        format!("Parent task {} completed with {} children", id, child_results.len())
    }
    
    let mut parent_set = JoinSet::new();
    
    // Spawn parent tasks
    for i in 0..2 {
        parent_set.spawn(parent_task(i));
    }
    
    // Wait for all parent tasks (and their children)
    while let Some(result) = parent_set.join_next().await {
        match result {
            Ok(message) => println!("      {}", message),
            Err(e) => println!("      Parent task error: {}", e),
        }
    }
}

/// Demonstrates timeout with structured concurrency
async fn timeout_with_structure_example() {
    println!("    Timeout with Structured Concurrency:");
    
    let mut set = JoinSet::new();
    
    // Spawn tasks with different durations
    for i in 0..4 {
        set.spawn(async move {
            let duration = Duration::from_millis(100 * (i + 1));
            tokio::time::sleep(duration).await;
            format!("Task {} completed after {:?}", i, duration)
        });
    }
    
    // Set a timeout for the entire group
    let timeout_duration = Duration::from_millis(250);
    let start = std::time::Instant::now();
    
    match tokio::time::timeout(timeout_duration, async {
        let mut results = Vec::new();
        while let Some(result) = set.join_next().await {
            match result {
                Ok(message) => results.push(message),
                Err(e) => println!("      Task error: {}", e),
            }
        }
        results
    }).await {
        Ok(results) => {
            println!("      All tasks completed within timeout:");
            for result in results {
                println!("        {}", result);
            }
        }
        Err(_) => {
            println!("      Timeout reached after {:?}", start.elapsed());
            set.abort_all();
            // Clean up remaining tasks
            while let Some(result) = set.join_next().await {
                if let Err(e) = result {
                    if e.is_cancelled() {
                        println!("      Remaining task was cancelled");
                    }
                }
            }
        }
    }
}

pub async fn run_examples() {
    join_set_example().await;
    error_handling_example().await;
    abort_example().await;
    scoped_tasks_example().await;
    resource_cleanup_example().await;
    hierarchical_tasks_example().await;
    timeout_with_structure_example().await;
}