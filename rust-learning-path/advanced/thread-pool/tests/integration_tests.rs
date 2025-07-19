//! Integration tests for the thread pool implementation

use std::sync::{Arc, Mutex, Barrier};
use std::thread;
use std::time::{Duration, Instant};
use thread_pool::{ThreadPool, ThreadPoolError};

#[test]
fn test_thread_pool_basic_functionality() {
    let pool = ThreadPool::new(4).unwrap();
    let counter = Arc::new(Mutex::new(0));

    for _ in 0..10 {
        let counter = Arc::clone(&counter);
        pool.execute(move || {
            let mut num = counter.lock().unwrap();
            *num += 1;
        }).unwrap();
    }

    // Wait for all jobs to complete
    thread::sleep(Duration::from_millis(500));

    let final_count = *counter.lock().unwrap();
    assert_eq!(final_count, 10);
}

#[test]
fn test_thread_pool_concurrent_execution() {
    let pool = ThreadPool::new(3).unwrap();
    let barrier = Arc::new(Barrier::new(3));
    let start_time = Arc::new(Mutex::new(None));

    for _ in 0..3 {
        let barrier = Arc::clone(&barrier);
        let start_time = Arc::clone(&start_time);
        
        pool.execute(move || {
            // All threads should start roughly at the same time
            barrier.wait();
            
            let mut time = start_time.lock().unwrap();
            if time.is_none() {
                *time = Some(Instant::now());
            }
            
            // Simulate some work
            thread::sleep(Duration::from_millis(100));
        }).unwrap();
    }

    // Wait for all jobs to complete
    thread::sleep(Duration::from_millis(500));

    // If executed concurrently, all jobs should have started around the same time
    let start_time = start_time.lock().unwrap();
    assert!(start_time.is_some());
}

#[test]
fn test_thread_pool_error_handling() {
    // Test invalid pool size
    let result = ThreadPool::new(0);
    assert!(result.is_err());
    
    match result {
        Err(ThreadPoolError::CreationFailed(_)) => {}, // Expected
        _ => panic!("Expected CreationFailed error"),
    }
}

#[test]
fn test_thread_pool_panic_recovery() {
    let pool = ThreadPool::new(2).unwrap();
    let counter = Arc::new(Mutex::new(0));

    // Submit a task that panics
    pool.execute(|| {
        panic!("Test panic");
    }).unwrap();

    // Submit a normal task - this should still work
    let counter_clone = Arc::clone(&counter);
    pool.execute(move || {
        let mut num = counter_clone.lock().unwrap();
        *num += 1;
    }).unwrap();

    // Wait for tasks to complete
    thread::sleep(Duration::from_millis(500));

    // The normal task should have executed despite the panic
    let final_count = *counter.lock().unwrap();
    assert_eq!(final_count, 1);
}

#[test]
fn test_thread_pool_shutdown() {
    let mut pool = ThreadPool::new(2).unwrap();
    let counter = Arc::new(Mutex::new(0));

    // Submit some long-running tasks
    for _ in 0..5 {
        let counter = Arc::clone(&counter);
        pool.execute(move || {
            thread::sleep(Duration::from_millis(100));
            let mut num = counter.lock().unwrap();
            *num += 1;
        }).unwrap();
    }

    // Shutdown the pool
    pool.shutdown();

    // All submitted tasks should have completed
    let final_count = *counter.lock().unwrap();
    assert_eq!(final_count, 5);
}

#[test]
fn test_thread_pool_drop_behavior() {
    let counter = Arc::new(Mutex::new(0));

    {
        let pool = ThreadPool::new(2).unwrap();
        
        for _ in 0..3 {
            let counter = Arc::clone(&counter);
            pool.execute(move || {
                thread::sleep(Duration::from_millis(100));
                let mut num = counter.lock().unwrap();
                *num += 1;
            }).unwrap();
        }
        
        // Pool will be dropped here, should wait for all jobs
    }

    // All jobs should have completed when pool was dropped
    let final_count = *counter.lock().unwrap();
    assert_eq!(final_count, 3);
}

#[test]
fn test_thread_pool_size() {
    let pool = ThreadPool::new(7).unwrap();
    assert_eq!(pool.size(), 7);
}

#[test]
fn test_thread_pool_heavy_load() {
    let pool = ThreadPool::new(4).unwrap();
    let counter = Arc::new(Mutex::new(0));
    let num_jobs = 100;

    let start_time = Instant::now();

    for _ in 0..num_jobs {
        let counter = Arc::clone(&counter);
        pool.execute(move || {
            // Simulate some work
            thread::sleep(Duration::from_millis(10));
            let mut num = counter.lock().unwrap();
            *num += 1;
        }).unwrap();
    }

    // Wait for all jobs to complete
    thread::sleep(Duration::from_secs(5));

    let elapsed = start_time.elapsed();
    let final_count = *counter.lock().unwrap();

    assert_eq!(final_count, num_jobs);
    
    // With 4 threads, 100 jobs of 10ms each should complete in roughly 250ms
    // (plus overhead), definitely less than 5 seconds
    assert!(elapsed < Duration::from_secs(5));
    
    println!("Heavy load test: {} jobs completed in {:?}", num_jobs, elapsed);
}

#[test]
fn test_thread_pool_job_ordering() {
    let pool = ThreadPool::new(1).unwrap(); // Single thread for deterministic ordering
    let results = Arc::new(Mutex::new(Vec::new()));

    for i in 0..5 {
        let results = Arc::clone(&results);
        pool.execute(move || {
            thread::sleep(Duration::from_millis(50)); // Ensure jobs don't complete instantly
            results.lock().unwrap().push(i);
        }).unwrap();
    }

    // Wait for all jobs to complete
    thread::sleep(Duration::from_millis(500));

    let final_results = results.lock().unwrap();
    assert_eq!(final_results.len(), 5);
    
    // With a single worker thread, jobs should complete in order
    assert_eq!(*final_results, vec![0, 1, 2, 3, 4]);
}