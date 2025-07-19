//! Command-line interface for the Distributed Task Queue System
//! 
//! This demonstrates the complete integration of all intermediate concepts

use std::time::Duration;
use clap::{Parser, Subcommand};

use capstone_project::{
    Config, TaskQueueSystem, TaskBox,
    task::traits::{MathTask, SleepTask, Task},
};

#[derive(Parser)]
#[command(name = "task-queue")]
#[command(about = "A distributed task queue system demonstrating intermediate Rust concepts")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
    
    /// Configuration file path
    #[arg(short, long)]
    config: Option<String>,
    
    /// Number of worker threads
    #[arg(short, long)]
    workers: Option<usize>,
    
    /// Enable verbose logging
    #[arg(short, long)]
    verbose: bool,
}

#[derive(Subcommand)]
enum Commands {
    /// Start the task queue server
    Server {
        /// Port to listen on
        #[arg(short, long, default_value = "8080")]
        port: u16,
    },
    /// Submit a math task
    Math {
        /// Operation (add, multiply, subtract, divide)
        #[arg(short, long)]
        operation: String,
        
        /// Operands for the operation
        operands: Vec<f64>,
    },
    /// Submit a sleep task
    Sleep {
        /// Duration to sleep in milliseconds
        #[arg(short, long)]
        duration: u64,
        
        /// Task name
        #[arg(short, long, default_value = "sleep_task")]
        name: String,
    },
    /// Show system status
    Status,
    /// Run example demonstrations
    Demo {
        /// Which demo to run
        #[arg(short, long, default_value = "basic")]
        demo: String,
    },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();
    
    // Initialize logging
    if cli.verbose {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("debug")).init();
    } else {
        env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    }
    
    // Load configuration
    let mut config = if let Some(config_path) = cli.config {
        Config::from_file(config_path)?
    } else {
        Config::development()
    };
    
    // Override worker count if specified
    if let Some(workers) = cli.workers {
        config.worker_count = workers;
    }
    
    match cli.command {
        Commands::Server { port } => {
            println!("Starting task queue server on port {}", port);
            run_server(config, port)
        }
        Commands::Math { operation, operands } => {
            run_math_task(config, operation, operands)
        }
        Commands::Sleep { duration, name } => {
            run_sleep_task(config, duration, name)
        }
        Commands::Status => {
            show_status(config)
        }
        Commands::Demo { demo } => {
            run_demo(config, demo)
        }
    }
}

fn run_server(config: Config, _port: u16) -> Result<(), Box<dyn std::error::Error>> {
    println!("Task Queue Server Configuration:");
    println!("  Workers: {}", config.worker_count);
    println!("  Storage: {}", config.storage_type);
    println!("  Storage Path: {:?}", config.storage_path);
    
    let system = TaskQueueSystem::new(config)?;
    system.start()?;
    
    println!("Server started. Press Ctrl+C to stop.");
    
    // In a real implementation, this would run a web server or accept network connections
    // For now, we'll just demonstrate with some example tasks
    
    // Submit some example tasks
    let math_task = MathTask {
        operation: "add".to_string(),
        operands: vec![1.0, 2.0, 3.0, 4.0, 5.0],
    };
    
    let task_id = system.submit(TaskBox::new(math_task))?;
    println!("Submitted math task with ID: {}", task_id);
    
    // Wait for completion
    let result = system.wait_for_result(task_id)?;
    println!("Task completed: {:?}", result);
    
    // Keep server running
    std::thread::sleep(Duration::from_secs(60));
    
    system.stop()?;
    println!("Server stopped.");
    
    Ok(())
}

fn run_math_task(config: Config, operation: String, operands: Vec<f64>) -> Result<(), Box<dyn std::error::Error>> {
    println!("Executing math task: {} with operands {:?}", operation, operands);
    
    let system = TaskQueueSystem::new(config)?;
    system.start()?;
    
    let task = MathTask { operation, operands };
    let task_id = system.submit(TaskBox::new(task))?;
    
    println!("Task submitted with ID: {}", task_id);
    
    // Wait for result
    let result = system.wait_for_result(task_id)?;
    
    match result.status {
        capstone_project::TaskStatus::Completed => {
            println!("Task completed successfully!");
            if let Some(output) = result.output {
                println!("Result: {}", output);
            }
        }
        capstone_project::TaskStatus::Failed => {
            println!("Task failed!");
            if let Some(error) = result.error {
                println!("Error: {}", error);
            }
        }
        _ => {
            println!("Task in unexpected state: {:?}", result.status);
        }
    }
    
    system.stop()?;
    Ok(())
}

fn run_sleep_task(config: Config, duration: u64, name: String) -> Result<(), Box<dyn std::error::Error>> {
    println!("Executing sleep task '{}' for {} ms", name, duration);
    
    let system = TaskQueueSystem::new(config)?;
    system.start()?;
    
    let task = SleepTask {
        duration_ms: duration,
        name,
    };
    let task_id = system.submit(TaskBox::new(task))?;
    
    println!("Task submitted with ID: {}", task_id);
    
    let start = std::time::Instant::now();
    let result = system.wait_for_result(task_id)?;
    let elapsed = start.elapsed();
    
    println!("Task completed in {:?}", elapsed);
    println!("Result: {:?}", result);
    
    system.stop()?;
    Ok(())
}

fn show_status(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    let system = TaskQueueSystem::new(config)?;
    let status = system.status();
    
    println!("Task Queue System Status:");
    println!("  Pending tasks: {}", status.pending_tasks);
    println!("  Running tasks: {}", status.running_tasks);
    println!("  Completed tasks: {}", status.completed_tasks);
    println!("  Failed tasks: {}", status.failed_tasks);
    println!("  Total processed: {}", status.total_processed);
    
    Ok(())
}

fn run_demo(config: Config, demo: String) -> Result<(), Box<dyn std::error::Error>> {
    match demo.as_str() {
        "basic" => run_basic_demo(config),
        "concurrent" => run_concurrent_demo(config),
        "priority" => run_priority_demo(config),
        "error" => run_error_demo(config),
        _ => {
            println!("Unknown demo: {}", demo);
            println!("Available demos: basic, concurrent, priority, error");
            Ok(())
        }
    }
}

fn run_basic_demo(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running basic demo...");
    
    let system = TaskQueueSystem::new(config)?;
    system.start()?;
    
    // Submit a few different tasks
    let tasks = vec![
        TaskBox::new(MathTask {
            operation: "add".to_string(),
            operands: vec![1.0, 2.0, 3.0],
        }),
        TaskBox::new(MathTask {
            operation: "multiply".to_string(),
            operands: vec![4.0, 5.0],
        }),
        TaskBox::new(SleepTask {
            duration_ms: 100,
            name: "quick_sleep".to_string(),
        }),
    ];
    
    let mut task_ids = Vec::new();
    for task in tasks {
        let task_id = system.submit(task)?;
        task_ids.push(task_id);
        println!("Submitted task: {}", task_id);
    }
    
    // Wait for all tasks to complete
    for task_id in task_ids {
        let result = system.wait_for_result(task_id)?;
        println!("Task {} completed: {:?}", task_id, result.status);
        if let Some(output) = result.output {
            println!("  Output: {}", output);
        }
    }
    
    system.stop()?;
    println!("Basic demo completed!");
    
    Ok(())
}

fn run_concurrent_demo(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running concurrent demo with {} workers...", config.worker_count);
    
    let system = TaskQueueSystem::new(config)?;
    system.start()?;
    
    // Submit many tasks concurrently
    let task_count = 20;
    let mut task_ids = Vec::new();
    
    for i in 0..task_count {
        let task = MathTask {
            operation: "add".to_string(),
            operands: vec![i as f64, (i + 1) as f64],
        };
        let task_id = system.submit(TaskBox::new(task))?;
        task_ids.push(task_id);
    }
    
    println!("Submitted {} tasks", task_count);
    
    // Wait for all to complete
    let start = std::time::Instant::now();
    for task_id in task_ids {
        let _result = system.wait_for_result(task_id)?;
    }
    let elapsed = start.elapsed();
    
    println!("All {} tasks completed in {:?}", task_count, elapsed);
    println!("Average time per task: {:?}", elapsed / task_count);
    
    system.stop()?;
    Ok(())
}

fn run_priority_demo(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running priority demo...");
    
    let system = TaskQueueSystem::new(config)?;
    system.start()?;
    
    // Submit tasks with different priorities
    let low_priority_task = MathTask {
        operation: "add".to_string(),
        operands: vec![1.0, 1.0],
    };
    let mut low_task_box = TaskBox::new(low_priority_task);
    low_task_box.metadata_mut().priority = 1;
    
    let high_priority_task = MathTask {
        operation: "multiply".to_string(),
        operands: vec![10.0, 10.0],
    };
    let mut high_task_box = TaskBox::new(high_priority_task);
    high_task_box.metadata_mut().priority = 10;
    
    // Submit low priority first, then high priority
    let low_id = system.submit(low_task_box)?;
    let high_id = system.submit(high_task_box)?;
    
    println!("Submitted low priority task: {}", low_id);
    println!("Submitted high priority task: {}", high_id);
    
    // Both should complete, but high priority should be processed first
    let high_result = system.wait_for_result(high_id)?;
    let low_result = system.wait_for_result(low_id)?;
    
    println!("High priority task result: {:?}", high_result.output);
    println!("Low priority task result: {:?}", low_result.output);
    
    system.stop()?;
    Ok(())
}

fn run_error_demo(config: Config) -> Result<(), Box<dyn std::error::Error>> {
    println!("Running error handling demo...");
    
    let system = TaskQueueSystem::new(config)?;
    system.start()?;
    
    // Submit a task that will fail
    let failing_task = MathTask {
        operation: "divide".to_string(),
        operands: vec![10.0, 0.0], // Division by zero
    };
    
    let task_id = system.submit(TaskBox::new(failing_task))?;
    println!("Submitted failing task: {}", task_id);
    
    let result = system.wait_for_result(task_id)?;
    
    match result.status {
        capstone_project::TaskStatus::Failed => {
            println!("Task failed as expected!");
            if let Some(error) = result.error {
                println!("Error message: {}", error);
            }
        }
        _ => {
            println!("Unexpected task status: {:?}", result.status);
        }
    }
    
    system.stop()?;
    Ok(())
}