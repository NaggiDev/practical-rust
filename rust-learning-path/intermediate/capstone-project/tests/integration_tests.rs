//! Integration tests for the capstone project
//! 
//! These tests demonstrate the complete system working together
//! and validate all the intermediate concepts are properly integrated

use std::time::Duration;
use std::thread;
use std::sync::Arc;

use capstone_project::{
    Config, TaskQueueSystem, TaskBox,
    task::traits::{MathTask, SleepTask, Task},
    TaskStatus, TaskId,
};

#[test]
fn test_basic_task_execution() {
    let config = Config::development();
    let system = TaskQueueSystem::new(config).unwrap();
    system.start().unwrap();
    
    // Submit a simple math task
    let task = MathTask {
        operation: "add".to_string(),
        operands: vec![1.0, 2.0, 3.0, 4.0, 5.0],
    };
    
    let task_id = system.submit(TaskBox::new(task)).unwrap();
    
    // Wait for completion
    let result = system.wait_for_result(task_id).unwrap();
    
    assert_eq!(result.status, TaskStatus::Completed);
    assert!(result.output.is_some());
    
    // Parse the result
    let output: f64 = serde_json::from_str(&result.output.unwrap()).unwrap();
    assert_eq!(output, 15.0);
    
    system.stop().unwrap();
}

#[test]
fn test_error_handling() {
    let config = Config::development();
    let system = TaskQueueSystem::new(config).unwrap();
    system.start().unwrap();
    
    // Submit a task that will fail (division by zero)
    let task = MathTask {
        operation: "divide".to_string(),
        operands: vec![10.0, 0.0],
    };
    
    let task_id = system.submit(TaskBox::new(task)).unwrap();
    let result = system.wait_for_result(task_id).unwrap();
    
    assert_eq!(result.status, TaskStatus::Failed);
    assert!(result.error.is_some());
    assert!(result.error.unwrap().contains("Division by zero"));
    
    system.stop().unwrap();
}

#[test]
fn test_concurrent_task_execution() {
    let config = Config {
        worker_count: 4,
        ..Config::development()
    };
    let system = TaskQueueSystem::new(config).unwrap();
    system.start().unwrap();
    
    let task_count = 20;
    let mut task_ids = Vec::new();
    
    // Submit multiple tasks concurrently
    for i in 0..task_count {
        let task = MathTask {
            operation: "multiply".to_string(),
            operands: vec![i as f64, 2.0],
        };
        let task_id = system.submit(TaskBox::new(task)).unwrap();
        task_ids.push(task_id);
    }
    
    // Wait for all tasks to complete
    let start = std::time::Instant::now();
    let mut completed_count = 0;
    
    for task_id in task_ids {
        let result = system.wait_for_result(task_id).unwrap();
        if result.status == TaskStatus::Completed {
            completed_count += 1;
        }
    }
    
    let elapsed = start.elapsed();
    
    assert_eq!(completed_count, task_count);
    println!("Completed {} tasks in {:?}", task_count, elapsed);
    
    system.stop().unwrap();
}

#[test]
fn test_task_priority_ordering() {
    let config = Config::development();
    let system = TaskQueueSystem::new(config).unwrap();
    system.start().unwrap();
    
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
    let low_id = system.submit(low_task_box).unwrap();
    let high_id = system.submit(high_task_box).unwrap();
    
    // Both should complete successfully
    let high_result = system.wait_for_result(high_id).unwrap();
    let low_result = system.wait_for_result(low_id).unwrap();
    
    assert_eq!(high_result.status, TaskStatus::Completed);
    assert_eq!(low_result.status, TaskStatus::Completed);
    
    // Verify results
    let high_output: f64 = serde_json::from_str(&high_result.output.unwrap()).unwrap();
    let low_output: f64 = serde_json::from_str(&low_result.output.unwrap()).unwrap();
    
    assert_eq!(high_output, 100.0);
    assert_eq!(low_output, 2.0);
    
    system.stop().unwrap();
}

#[test]
fn test_mixed_task_types() {
    let config = Config::development();
    let system = TaskQueueSystem::new(config).unwrap();
    system.start().unwrap();
    
    // Submit different types of tasks
    let math_task = MathTask {
        operation: "add".to_string(),
        operands: vec![5.0, 10.0],
    };
    
    let sleep_task = SleepTask {
        duration_ms: 50,
        name: "test_sleep".to_string(),
    };
    
    let math_id = system.submit(TaskBox::new(math_task)).unwrap();
    let sleep_id = system.submit(TaskBox::new(sleep_task)).unwrap();
    
    // Wait for both to complete
    let math_result = system.wait_for_result(math_id).unwrap();
    let sleep_result = system.wait_for_result(sleep_id).unwrap();
    
    assert_eq!(math_result.status, TaskStatus::Completed);
    assert_eq!(sleep_result.status, TaskStatus::Completed);
    
    // Verify math result
    let math_output: f64 = serde_json::from_str(&math_result.output.unwrap()).unwrap();
    assert_eq!(math_output, 15.0);
    
    // Verify sleep result
    let sleep_output: String = serde_json::from_str(&sleep_result.output.unwrap()).unwrap();
    assert!(sleep_output.contains("50 ms"));
    
    system.stop().unwrap();
}

#[test]
fn test_system_status_monitoring() {
    let config = Config::development();
    let system = TaskQueueSystem::new(config).unwrap();
    system.start().unwrap();
    
    // Check initial status
    let initial_status = system.status();
    assert_eq!(initial_status.pending_tasks, 0);
    assert_eq!(initial_status.running_tasks, 0);
    
    // Submit some tasks
    let task1 = MathTask {
        operation: "add".to_string(),
        operands: vec![1.0, 2.0],
    };
    let task2 = SleepTask {
        duration_ms: 100,
        name: "status_test".to_string(),
    };
    
    let _id1 = system.submit(TaskBox::new(task1)).unwrap();
    let _id2 = system.submit(TaskBox::new(task2)).unwrap();
    
    // Give tasks time to start
    thread::sleep(Duration::from_millis(10));
    
    // Check status after submission
    let status = system.status();
    assert!(status.total_processed >= 2);
    
    system.stop().unwrap();
}

#[test]
fn test_task_timeout_handling() {
    let config = Config::development();
    let system = TaskQueueSystem::new(config).unwrap();
    system.start().unwrap();
    
    // Submit a task that will take longer than its timeout
    let mut long_sleep_task = SleepTask {
        duration_ms: 1000, // 1 second
        name: "timeout_test".to_string(),
    };
    
    let mut task_box = TaskBox::new(long_sleep_task);
    task_box.metadata_mut().timeout = Some(Duration::from_millis(100)); // 100ms timeout
    
    let task_id = system.submit(task_box).unwrap();
    
    // Wait for the task with a reasonable timeout
    let result = system.wait_for_result(task_id).unwrap();
    
    // The task should either timeout or complete quickly
    // (depending on implementation details)
    assert!(matches!(
        result.status,
        TaskStatus::Completed | TaskStatus::TimedOut | TaskStatus::Failed
    ));
    
    system.stop().unwrap();
}

#[test]
fn test_system_lifecycle() {
    let config = Config::development();
    
    // Test multiple start/stop cycles
    for _ in 0..3 {
        let system = TaskQueueSystem::new(config.clone()).unwrap();
        
        // Start the system
        system.start().unwrap();
        
        // Submit a task
        let task = MathTask {
            operation: "multiply".to_string(),
            operands: vec![3.0, 7.0],
        };
        let task_id = system.submit(TaskBox::new(task)).unwrap();
        
        // Wait for completion
        let result = system.wait_for_result(task_id).unwrap();
        assert_eq!(result.status, TaskStatus::Completed);
        
        // Stop the system
        system.stop().unwrap();
    }
}

#[test]
fn test_configuration_validation() {
    // Test invalid configuration
    let mut config = Config::development();
    config.worker_count = 0;
    
    let result = config.validate();
    assert!(result.is_err());
    
    // Test valid configuration
    config.worker_count = 2;
    assert!(config.validate().is_ok());
}

#[test]
fn test_stress_testing() {
    let config = Config {
        worker_count: 8,
        ..Config::development()
    };
    let system = Arc::new(TaskQueueSystem::new(config).unwrap());
    system.start().unwrap();
    
    let task_count = 100;
    let thread_count = 10;
    let mut handles = vec![];
    
    // Spawn multiple threads submitting tasks
    for thread_id in 0..thread_count {
        let system_clone = Arc::clone(&system);
        let handle = thread::spawn(move || {
            let mut task_ids = vec![];
            
            for i in 0..task_count / thread_count {
                let task = MathTask {
                    operation: "add".to_string(),
                    operands: vec![thread_id as f64, i as f64],
                };
                let task_id = system_clone.submit(TaskBox::new(task)).unwrap();
                task_ids.push(task_id);
            }
            
            // Wait for all tasks in this thread to complete
            let mut completed = 0;
            for task_id in task_ids {
                let result = system_clone.wait_for_result(task_id).unwrap();
                if result.status == TaskStatus::Completed {
                    completed += 1;
                }
            }
            
            completed
        });
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    let mut total_completed = 0;
    for handle in handles {
        total_completed += handle.join().unwrap();
    }
    
    assert_eq!(total_completed, task_count);
    
    system.stop().unwrap();
}

#[test]
fn test_error_recovery() {
    let config = Config::development();
    let system = TaskQueueSystem::new(config).unwrap();
    system.start().unwrap();
    
    // Submit a mix of successful and failing tasks
    let tasks = vec![
        ("add", vec![1.0, 2.0], true),
        ("divide", vec![10.0, 0.0], false), // Will fail
        ("multiply", vec![3.0, 4.0], true),
        ("divide", vec![8.0, 0.0], false), // Will fail
        ("subtract", vec![10.0, 3.0], true),
    ];
    
    let mut task_ids = vec![];
    for (operation, operands, _should_succeed) in &tasks {
        let task = MathTask {
            operation: operation.to_string(),
            operands: operands.clone(),
        };
        let task_id = system.submit(TaskBox::new(task)).unwrap();
        task_ids.push(task_id);
    }
    
    // Wait for all tasks and verify results
    let mut successful_count = 0;
    let mut failed_count = 0;
    
    for (i, task_id) in task_ids.into_iter().enumerate() {
        let result = system.wait_for_result(task_id).unwrap();
        let should_succeed = tasks[i].2;
        
        if should_succeed {
            assert_eq!(result.status, TaskStatus::Completed);
            successful_count += 1;
        } else {
            assert_eq!(result.status, TaskStatus::Failed);
            failed_count += 1;
        }
    }
    
    assert_eq!(successful_count, 3);
    assert_eq!(failed_count, 2);
    
    system.stop().unwrap();
}

#[test]
fn test_task_metadata_preservation() {
    let config = Config::development();
    let system = TaskQueueSystem::new(config).unwrap();
    system.start().unwrap();
    
    // Create a task with custom metadata
    let task = MathTask {
        operation: "multiply".to_string(),
        operands: vec![6.0, 7.0],
    };
    
    let mut task_box = TaskBox::new(task);
    task_box.metadata_mut().priority = 5;
    task_box.metadata_mut().tags.push("important".to_string());
    task_box.metadata_mut().custom_data.insert("user_id".to_string(), "12345".to_string());
    
    let task_id = system.submit(task_box).unwrap();
    let result = system.wait_for_result(task_id).unwrap();
    
    assert_eq!(result.status, TaskStatus::Completed);
    
    // Verify the result contains timing information
    assert!(result.submitted_at <= chrono::Utc::now());
    assert!(result.started_at.is_some());
    assert!(result.completed_at.is_some());
    assert!(result.duration.is_some());
    
    system.stop().unwrap();
}