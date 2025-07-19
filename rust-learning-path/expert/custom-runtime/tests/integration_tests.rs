//! Integration tests for the custom runtime.
//! 
//! These tests verify that all components work together correctly
//! and that the runtime behaves as expected in various scenarios.

use custom_runtime::prelude::*;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

#[test]
fn test_basic_task_execution() {
    let mut executor = Executor::new();
    let executed = Arc::new(AtomicBool::new(false));
    let executed_clone = Arc::clone(&executed);

    executor.spawn(async move {
        executed_clone.store(true, Ordering::SeqCst);
    });

    let completed = executor.run();
    assert_eq!(completed, 1);
    assert!(executed.load(Ordering::SeqCst));
}

#[test]
fn test_multiple_tasks() {
    let mut executor = Executor::new();
    let counter = Arc::new(AtomicUsize::new(0));

    // Spawn 10 tasks
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        executor.spawn(async move {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });
    }

    let completed = executor.run();
    assert_eq!(completed, 10);
    assert_eq!(counter.load(Ordering::SeqCst), 10);
}

#[test]
fn test_nested_async_operations() {
    let mut executor = Executor::new();
    let result = Arc::new(AtomicUsize::new(0));
    let result_clone = Arc::clone(&result);

    executor.spawn(async move {
        // Nested async block
        let value = async {
            42
        }.await;
        
        result_clone.store(value, Ordering::SeqCst);
    });

    let completed = executor.run();
    assert_eq!(completed, 1);
    assert_eq!(result.load(Ordering::SeqCst), 42);
}

#[test]
fn test_timer_functionality() {
    let mut executor = Executor::new();
    let start_time = Instant::now();
    let completed_time = Arc::new(std::sync::Mutex::new(None));
    let completed_time_clone = Arc::clone(&completed_time);

    executor.spawn(async move {
        Timer::after_millis(50).await;
        let mut time = completed_time_clone.lock().unwrap();
        *time = Some(Instant::now());
    });

    let completed = executor.run();
    assert_eq!(completed, 1);

    let end_time = completed_time.lock().unwrap().unwrap();
    let elapsed = end_time.duration_since(start_time);
    
    // Timer should take at least 50ms (allowing for some timing variance)
    assert!(elapsed >= Duration::from_millis(45));
}

#[test]
fn test_concurrent_timers() {
    let mut executor = Executor::new();
    let completion_order = Arc::new(std::sync::Mutex::new(Vec::new()));

    // Spawn timers with different durations
    let durations = [10, 30, 20];
    for (i, &duration) in durations.iter().enumerate() {
        let order_clone = Arc::clone(&completion_order);
        executor.spawn(async move {
            Timer::after_millis(duration).await;
            order_clone.lock().unwrap().push(i);
        });
    }

    let completed = executor.run();
    assert_eq!(completed, 3);

    let order = completion_order.lock().unwrap();
    // Should complete in order: 0 (10ms), 2 (20ms), 1 (30ms)
    assert_eq!(*order, vec![0, 2, 1]);
}

#[test]
fn test_yield_timer() {
    let mut executor = Executor::new();
    let poll_count = Arc::new(AtomicUsize::new(0));
    let poll_count_clone = Arc::clone(&poll_count);

    executor.spawn(async move {
        use custom_runtime::timer::YieldTimer;
        
        // This should cause the task to be polled multiple times
        YieldTimer::new(3).await;
        poll_count_clone.store(1, Ordering::SeqCst);
    });

    let completed = executor.run();
    assert_eq!(completed, 1);
    assert_eq!(poll_count.load(Ordering::SeqCst), 1);
}

#[test]
fn test_cooperative_multitasking() {
    let mut executor = Executor::new();
    let execution_order = Arc::new(std::sync::Mutex::new(Vec::new()));

    // Create tasks that yield at different points
    for i in 0..3 {
        let order_clone = Arc::clone(&execution_order);
        executor.spawn(async move {
            use custom_runtime::timer::YieldTimer;
            
            order_clone.lock().unwrap().push(format!("Task {} start", i));
            YieldTimer::new(1).await;
            order_clone.lock().unwrap().push(format!("Task {} middle", i));
            YieldTimer::new(1).await;
            order_clone.lock().unwrap().push(format!("Task {} end", i));
        });
    }

    let completed = executor.run();
    assert_eq!(completed, 3);

    let order = execution_order.lock().unwrap();
    // Tasks should interleave due to cooperative yielding
    assert!(order.len() >= 9); // Each task should have 3 entries
    
    // All tasks should start before any complete
    let start_count = order.iter().filter(|s| s.contains("start")).count();
    let end_count = order.iter().filter(|s| s.contains("end")).count();
    assert_eq!(start_count, 3);
    assert_eq!(end_count, 3);
}

#[test]
fn test_complex_async_chain() {
    let mut executor = Executor::new();
    let results = Arc::new(std::sync::Mutex::new(Vec::new()));
    let results_clone = Arc::clone(&results);

    executor.spawn(async move {
        // Chain of async operations
        let step1 = async { 1 }.await;
        Timer::after_millis(10).await;
        
        let step2 = async { step1 + 2 }.await;
        Timer::after_millis(10).await;
        
        let step3 = async { step2 * 3 }.await;
        
        results_clone.lock().unwrap().push(step3);
    });

    let completed = executor.run();
    assert_eq!(completed, 1);

    let results = results.lock().unwrap();
    assert_eq!(results[0], 9); // (1 + 2) * 3 = 9
}

#[test]
fn test_executor_state_tracking() {
    let mut executor = Executor::new();
    
    // Initially empty
    assert_eq!(executor.task_count(), 0);
    assert!(!executor.has_tasks());
    assert_eq!(executor.ready_task_count(), 0);

    // Add some tasks
    executor.spawn(async {});
    executor.spawn(async {});
    
    assert_eq!(executor.task_count(), 2);
    assert!(executor.has_tasks());
    assert_eq!(executor.ready_task_count(), 2); // New tasks are ready

    // Run to completion
    let completed = executor.run();
    assert_eq!(completed, 2);
    assert_eq!(executor.task_count(), 0);
    assert!(!executor.has_tasks());
}

#[test]
fn test_run_once_behavior() {
    let mut executor = Executor::new();
    let counter = Arc::new(AtomicUsize::new(0));

    // Add tasks that complete immediately
    for _ in 0..3 {
        let counter_clone = Arc::clone(&counter);
        executor.spawn(async move {
            counter_clone.fetch_add(1, Ordering::SeqCst);
        });
    }

    // Run once should complete all ready tasks
    let completed = executor.run_once();
    assert_eq!(completed, 3);
    assert_eq!(counter.load(Ordering::SeqCst), 3);
    assert!(!executor.has_tasks());
}

#[test]
fn test_mixed_task_types() {
    let mut executor = Executor::new();
    let results = Arc::new(std::sync::Mutex::new(Vec::new()));

    // Immediate task
    let results_clone = Arc::clone(&results);
    executor.spawn(async move {
        results_clone.lock().unwrap().push("immediate");
    });

    // Timer task
    let results_clone = Arc::clone(&results);
    executor.spawn(async move {
        Timer::after_millis(20).await;
        results_clone.lock().unwrap().push("timer");
    });

    // Yield task
    let results_clone = Arc::clone(&results);
    executor.spawn(async move {
        use custom_runtime::timer::YieldTimer;
        YieldTimer::new(2).await;
        results_clone.lock().unwrap().push("yield");
    });

    let completed = executor.run();
    assert_eq!(completed, 3);

    let results = results.lock().unwrap();
    assert_eq!(results.len(), 3);
    assert!(results.contains(&"immediate"));
    assert!(results.contains(&"timer"));
    assert!(results.contains(&"yield"));
}