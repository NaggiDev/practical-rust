use criterion::{black_box, criterion_group, criterion_main, Criterion};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use thread_pool::ThreadPool;

fn benchmark_thread_pool_vs_spawn(c: &mut Criterion) {
    let mut group = c.benchmark_group("thread_pool_vs_spawn");
    
    group.bench_function("thread_pool_100_tasks", |b| {
        b.iter(|| {
            let pool = ThreadPool::new(4).unwrap();
            let counter = Arc::new(Mutex::new(0));
            
            for _ in 0..100 {
                let counter = Arc::clone(&counter);
                pool.execute(move || {
                    let mut num = counter.lock().unwrap();
                    *num += 1;
                }).unwrap();
            }
            
            // Wait for completion
            thread::sleep(Duration::from_millis(100));
            black_box(counter);
        });
    });
    
    group.bench_function("thread_spawn_100_tasks", |b| {
        b.iter(|| {
            let counter = Arc::new(Mutex::new(0));
            let mut handles = vec![];
            
            for _ in 0..100 {
                let counter = Arc::clone(&counter);
                let handle = thread::spawn(move || {
                    let mut num = counter.lock().unwrap();
                    *num += 1;
                });
                handles.push(handle);
            }
            
            for handle in handles {
                handle.join().unwrap();
            }
            
            black_box(counter);
        });
    });
    
    group.finish();
}

fn benchmark_different_pool_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("pool_sizes");
    
    for size in [1, 2, 4, 8, 16].iter() {
        group.bench_with_input(format!("pool_size_{}", size), size, |b, &size| {
            b.iter(|| {
                let pool = ThreadPool::new(size).unwrap();
                let counter = Arc::new(Mutex::new(0));
                
                for _ in 0..50 {
                    let counter = Arc::clone(&counter);
                    pool.execute(move || {
                        // Simulate some work
                        let mut sum = 0;
                        for i in 0..1000 {
                            sum += i;
                        }
                        
                        let mut num = counter.lock().unwrap();
                        *num += sum;
                    }).unwrap();
                }
                
                thread::sleep(Duration::from_millis(100));
                black_box(counter);
            });
        });
    }
    
    group.finish();
}

criterion_group!(benches, benchmark_thread_pool_vs_spawn, benchmark_different_pool_sizes);
criterion_main!(benches);