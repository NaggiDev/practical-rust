//! Basic usage example for the Distributed Task Queue System
//! 
//! This example demonstrates how to use the capstone project and shows
//! all the intermediate concepts working together in a practical application.

use std::time::Duration;
use std::thread;

use capstone_project::{
    Config, TaskQueueSystem, TaskBox,
    task::traits::{MathTask, SleepTask, Task},
    TaskStatus,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();
    
    println!("🚀 Distributed Task Queue System - Basic Usage Example");
    println!("=====================================================");
    
    // Create a development configuration
    let config = Config::development();
    println!("📋 Configuration:");
    println!("   Workers: {}", config.worker_count);
    println!("   Storage: {}", config.storage_type);
    println!("   Default timeout: {:?}", config.default_timeout);
    
    // Create and start the task queue system
    println!("\n🔧 Starting task queue system...");
    let system = TaskQueueSystem::new(config)?;
    system.start()?;
    println!("✅ System started successfully!");
    
    // Demonstrate basic task submission and execution
    println!("\n📝 Submitting basic tasks...");
    demonstrate_basic_tasks(&system)?;
    
    // Demonstrate different task types
    println!("\n🔄 Demonstrating different task types...");
    demonstrate_task_types(&system)?;
    
    // Demonstrate concurrent execution
    println!("\n⚡ Demonstrating concurrent execution...");
    demonstrate_concurrent_execution(&system)?;
    
    // Demonstrate priority handling
    println!("\n🎯 Demonstrating task priorities...");
    demonstrate_priority_handling(&system)?;
    
    // Demonstrate error handling
    println!("\n❌ Demonstrating error handling...");
    demonstrate_error_handling(&system)?;
    
    // Show system status
    println!("\n📊 Final system status:");
    show_system_status(&system);
    
    // Clean shutdown
    println!("\n🛑 Shutting down system...");
    system.stop()?;
    println!("✅ System shutdown complete!");
    
    Ok(())
}

fn demonstrate_basic_tasks(system: &TaskQueueSystem) -> Result<(), Box<dyn std::error::Error>> {
    // Submit a simple addition task
    let add_task = MathTask {
        operation: "add".to_string(),
        operands: vec![10.0, 20.0, 30.0],
    };
    
    let task_id = system.submit(TaskBox::new(add_task))?;
    println!("   Submitted addition task: {}", task_id);
    
    // Wait for the result
    let result = system.wait_for_result(task_id)?;
    
    match result.status {
        TaskStatus::Completed => {
            let output: f64 = serde_json::from_str(&result.output.unwrap())?;
            println!("   ✅ Addition result: {}", output);
            if let Some(duration) = result.duration {
                println!("   ⏱️  Execution time: {:?}", duration);
            }
        }
        _ => {
            println!("   ❌ Task failed: {:?}", result.error);
        }
    }
    
    Ok(())
}

fn demonstrate_task_types(system: &TaskQueueSystem) -> Result<(), Box<dyn std::error::Error>> {
    // Math task
    let math_task = MathTask {
        operation: "multiply".to_string(),
        operands: vec![7.0, 8.0],
    };
    
    // Sleep task
    let sleep_task = SleepTask {
        duration_ms: 200,
        name: "demo_sleep".to_string(),
    };
    
    let math_id = system.submit(TaskBox::new(math_task))?;
    let sleep_id = system.submit(TaskBox::new(sleep_task))?;
    
    println!("   Submitted math task: {}", math_id);
    println!("   Submitted sleep task: {}", sleep_id);
    
    // Wait for both tasks
    let math_result = system.wait_for_result(math_id)?;
    let sleep_result = system.wait_for_result(sleep_id)?;
    
    // Display results
    if math_result.status == TaskStatus::Completed {
        let output: f64 = serde_json::from_str(&math_result.output.unwrap())?;
        println!("   ✅ Math result: {}", output);
    }
    
    if sleep_result.status == TaskStatus::Completed {
        let output: String = serde_json::from_str(&sleep_result.output.unwrap())?;
        println!("   ✅ Sleep result: {}", output);
    }
    
    Ok(())
}

fn demonstrate_concurrent_execution(system: &TaskQueueSystem) -> Result<(), Box<dyn std::error::Error>> {
    let task_count = 10;
    let mut task_ids = Vec::new();
    
    println!("   Submitting {} tasks concurrently...", task_count);
    
    let start_time = std::time::Instant::now();
    
    // Submit multiple tasks
    for i in 0..task_count {
        let task = MathTask {
            operation: "add".to_string(),
            operands: vec![i as f64, (i + 1) as f64, (i + 2) as f64],
        };
        let task_id = system.submit(TaskBox::new(task))?;
        task_ids.push(task_id);
    }
    
    // Wait for all tasks to complete
    let mut completed_count = 0;
    for task_id in task_ids {
        let result = system.wait_for_result(task_id)?;
        if result.status == TaskStatus::Completed {
            completed_count += 1;
        }
    }
    
    let total_time = start_time.elapsed();
    
    println!("   ✅ Completed {}/{} tasks in {:?}", completed_count, task_count, total_time);
    println!("   📈 Average time per task: {:?}", total_time / task_count);
    
    Ok(())
}

fn demonstrate_priority_handling(system: &TaskQueueSystem) -> Result<(), Box<dyn std::error::Error>> {
    // Create tasks with different priorities
    let low_priority_task = MathTask {
        operation: "add".to_string(),
        operands: vec![1.0, 2.0],
    };
    let mut low_task_box = TaskBox::new(low_priority_task);
    low_task_box.metadata_mut().priority = 1;
    
    let high_priority_task = MathTask {
        operation: "multiply".to_string(),
        operands: vec![5.0, 6.0],
    };
    let mut high_task_box = TaskBox::new(high_priority_task);
    high_task_box.metadata_mut().priority = 10;
    
    let medium_priority_task = MathTask {
        operation: "subtract".to_string(),
        operands: vec![20.0, 5.0],
    };
    let mut medium_task_box = TaskBox::new(medium_priority_task);
    medium_task_box.metadata_mut().priority = 5;
    
    // Submit in reverse priority order
    let low_id = system.submit(low_task_box)?;
    let high_id = system.submit(high_task_box)?;
    let medium_id = system.submit(medium_task_box)?;
    
    println!("   Submitted tasks with priorities: Low(1), High(10), Medium(5)");
    
    // Wait for all tasks
    let high_result = system.wait_for_result(high_id)?;
    let medium_result = system.wait_for_result(medium_id)?;
    let low_result = system.wait_for_result(low_id)?;
    
    println!("   ✅ High priority result: {:?}", high_result.output);
    println!("   ✅ Medium priority result: {:?}", medium_result.output);
    println!("   ✅ Low priority result: {:?}", low_result.output);
    
    Ok(())
}

fn demonstrate_error_handling(system: &TaskQueueSystem) -> Result<(), Box<dyn std::error::Error>> {
    // Submit a task that will fail
    let failing_task = MathTask {
        operation: "divide".to_string(),
        operands: vec![42.0, 0.0], // Division by zero
    };
    
    let task_id = system.submit(TaskBox::new(failing_task))?;
    println!("   Submitted failing task (division by zero): {}", task_id);
    
    let result = system.wait_for_result(task_id)?;
    
    match result.status {
        TaskStatus::Failed => {
            println!("   ✅ Error handled correctly!");
            println!("   📝 Error message: {}", result.error.unwrap_or_else(|| "Unknown error".to_string()));
        }
        _ => {
            println!("   ❌ Expected task to fail, but got: {:?}", result.status);
        }
    }
    
    // Submit a task with unknown operation
    let unknown_task = MathTask {
        operation: "unknown_operation".to_string(),
        operands: vec![1.0, 2.0],
    };
    
    let task_id = system.submit(TaskBox::new(unknown_task))?;
    println!("   Submitted task with unknown operation: {}", task_id);
    
    let result = system.wait_for_result(task_id)?;
    
    match result.status {
        TaskStatus::Failed => {
            println!("   ✅ Unknown operation error handled correctly!");
            println!("   📝 Error message: {}", result.error.unwrap_or_else(|| "Unknown error".to_string()));
        }
        _ => {
            println!("   ❌ Expected task to fail, but got: {:?}", result.status);
        }
    }
    
    Ok(())
}

fn show_system_status(system: &TaskQueueSystem) {
    let status = system.status();
    
    println!("   Pending tasks: {}", status.pending_tasks);
    println!("   Running tasks: {}", status.running_tasks);
    println!("   Completed tasks: {}", status.completed_tasks);
    println!("   Failed tasks: {}", status.failed_tasks);
    println!("   Total processed: {}", status.total_processed);
}

// Helper function to create a custom task for demonstration
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct CustomTask {
    message: String,
    repeat_count: usize,
}

impl Task for CustomTask {
    type Output = String;
    type Error = String;
    
    fn execute(&self) -> Result<Self::Output, Self::Error> {
        let mut result = String::new();
        for i in 0..self.repeat_count {
            if i > 0 {
                result.push_str(", ");
            }
            result.push_str(&format!("{} #{}", self.message, i + 1));
        }
        Ok(result)
    }
    
    fn name(&self) -> &str {
        "custom_task"
    }
    
    fn timeout(&self) -> Option<Duration> {
        Some(Duration::from_secs(10))
    }
    
    fn priority(&self) -> i32 {
        3
    }
    
    fn tags(&self) -> Vec<String> {
        vec!["custom".to_string(), "demo".to_string()]
    }
}

#[allow(dead_code)]
fn demonstrate_custom_task(system: &TaskQueueSystem) -> Result<(), Box<dyn std::error::Error>> {
    let custom_task = CustomTask {
        message: "Hello World".to_string(),
        repeat_count: 3,
    };
    
    let task_id = system.submit(TaskBox::new(custom_task))?;
    println!("   Submitted custom task: {}", task_id);
    
    let result = system.wait_for_result(task_id)?;
    
    if result.status == TaskStatus::Completed {
        let output: String = serde_json::from_str(&result.output.unwrap())?;
        println!("   ✅ Custom task result: {}", output);
    }
    
    Ok(())
}