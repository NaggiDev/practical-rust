use std::time::Duration;

mod basic_futures;
mod async_await;
mod pinning;
mod runtimes;
mod task_scheduling;
mod streams;
mod cancellation;
mod structured_concurrency;
mod performance;

#[tokio::main]
async fn main() {
    println!("=== Async Programming Examples ===\n");

    println!("1. Basic Futures:");
    basic_futures::run_examples().await;
    
    println!("\n2. Async/Await:");
    async_await::run_examples().await;
    
    println!("\n3. Pinning:");
    pinning::run_examples().await;
    
    println!("\n4. Runtimes:");
    runtimes::run_examples().await;
    
    println!("\n5. Task Scheduling:");
    task_scheduling::run_examples().await;
    
    println!("\n6. Streams:");
    streams::run_examples().await;
    
    println!("\n7. Cancellation:");
    cancellation::run_examples().await;
    
    println!("\n8. Structured Concurrency:");
    structured_concurrency::run_examples().await;
    
    println!("\n9. Performance:");
    performance::run_examples().await;
    
    println!("\n=== All examples completed! ===");
}