//! Criterion benchmarks for the high-performance data processing library
//! 
//! These benchmarks provide detailed performance measurements using the Criterion
//! benchmarking framework, which provides statistical analysis of performance.

use criterion::{black_box, criterion_group, criterion_main, Criterion, BenchmarkId, Throughput};
use high_performance_data_processing::*;
use tempfile::NamedTempFile;
use std::io::Write;

/// Benchmark SIMD operations with different array sizes
fn bench_simd_operations(c: &mut Criterion) {
    let simd_ops = SimdOperations::new();
    
    let mut group = c.benchmark_group("simd_operations");
    
    // Test different array sizes to see scaling behavior
    for size in [100, 1000, 10000, 100000].iter() {
        let a: Vec<f64> = (0..*size).map(|i| i as f64).collect();
        let b: Vec<f64> = (0..*size).map(|i| (i * 2) as f64).collect();
        
        group.throughput(Throughput::Elements(*size as u64));
        
        // Benchmark array addition
        group.bench_with_input(
            BenchmarkId::new("add_arrays", size),
            size,
            |bench, _| {
                bench.iter(|| {
                    simd_ops.add_arrays(black_box(&a), black_box(&b)).unwrap()
                });
            },
        );
        
        // Benchmark array multiplication
        group.bench_with_input(
            BenchmarkId::new("multiply_arrays", size),
            size,
            |bench, _| {
                bench.iter(|| {
                    simd_ops.multiply_arrays(black_box(&a), black_box(&b)).unwrap()
                });
            },
        );
        
        // Benchmark dot product
        group.bench_with_input(
            BenchmarkId::new("dot_product", size),
            size,
            |bench, _| {
                bench.iter(|| {
                    simd_ops.dot_product(black_box(&a), black_box(&b)).unwrap()
                });
            },
        );
        
        // Benchmark sum operation
        group.bench_with_input(
            BenchmarkId::new("sum_array", size),
            size,
            |bench, _| {
                bench.iter(|| {
                    simd_ops.sum_array(black_box(&a))
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark parallel processing with different thread counts and data sizes
fn bench_parallel_processing(c: &mut Criterion) {
    let mut group = c.benchmark_group("parallel_processing");
    
    // Test different data sizes
    for size in [1000, 10000, 100000].iter() {
        let data: Vec<i32> = (0..*size).collect();
        
        group.throughput(Throughput::Elements(*size as u64));
        
        // Test different thread counts
        for threads in [1, 2, 4].iter() {
            let processor = ParallelProcessor::new(Some(*threads));
            
            // Benchmark parallel map
            group.bench_with_input(
                BenchmarkId::new(format!("parallel_map_{}threads", threads), size),
                size,
                |bench, _| {
                    bench.iter(|| {
                        processor.parallel_map(black_box(&data), |&x| {
                            // Simulate computational work
                            (0..10).fold(x, |acc, i| acc.wrapping_add(i))
                        })
                    });
                },
            );
            
            // Benchmark parallel aggregation
            let float_data: Vec<f64> = data.iter().map(|&x| x as f64).collect();
            group.bench_with_input(
                BenchmarkId::new(format!("parallel_aggregate_{}threads", threads), size),
                size,
                |bench, _| {
                    bench.iter(|| {
                        processor.parallel_aggregate(black_box(&float_data))
                    });
                },
            );
        }
    }
    
    group.finish();
}

/// Benchmark memory-mapped file operations
fn bench_memory_mapped_io(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_mapped_io");
    
    // Create test files of different sizes
    let processor = MemoryMappedProcessor::new();
    let mut temp_files = Vec::new();
    
    for size in [1000, 10000, 50000].iter() {
        let temp_file = NamedTempFile::new().unwrap();
        processor.create_sample_dataset(temp_file.path(), *size).unwrap();
        temp_files.push((temp_file, *size));
    }
    
    for (temp_file, size) in &temp_files {
        group.throughput(Throughput::Elements(*size as u64));
        
        // Benchmark CSV processing
        group.bench_with_input(
            BenchmarkId::new("process_csv_file", size),
            size,
            |bench, _| {
                bench.iter(|| {
                    processor.process_csv_file(black_box(temp_file.path())).unwrap()
                });
            },
        );
        
        // Benchmark streaming processing
        group.bench_with_input(
            BenchmarkId::new("stream_process_file", size),
            size,
            |bench, _| {
                bench.iter(|| {
                    processor.stream_process_file(
                        black_box(temp_file.path()),
                        |_chunk| Ok(())
                    ).unwrap()
                });
            },
        );
        
        // Benchmark pattern searching
        group.bench_with_input(
            BenchmarkId::new("search_pattern", size),
            size,
            |bench, _| {
                bench.iter(|| {
                    processor.search_pattern(
                        black_box(temp_file.path()),
                        black_box(b"1000")
                    ).unwrap()
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark the complete data processing pipeline
fn bench_complete_pipeline(c: &mut Criterion) {
    let mut group = c.benchmark_group("complete_pipeline");
    
    // Create test files
    let mmap_processor = MemoryMappedProcessor::new();
    let mut temp_files = Vec::new();
    
    for size in [5000, 25000].iter() {
        let temp_file = NamedTempFile::new().unwrap();
        mmap_processor.create_sample_dataset(temp_file.path(), *size).unwrap();
        temp_files.push((temp_file, *size));
    }
    
    // Test different optimization configurations
    let configs = vec![
        ("no_optimizations", ProcessingConfig {
            use_simd: false,
            use_parallel: false,
            use_memory_map: false,
            thread_count: Some(1),
            chunk_size: 1024,
        }),
        ("simd_only", ProcessingConfig {
            use_simd: true,
            use_parallel: false,
            use_memory_map: false,
            thread_count: Some(1),
            chunk_size: 1024,
        }),
        ("parallel_only", ProcessingConfig {
            use_simd: false,
            use_parallel: true,
            use_memory_map: false,
            thread_count: Some(2),
            chunk_size: 1024,
        }),
        ("memory_map_only", ProcessingConfig {
            use_simd: false,
            use_parallel: false,
            use_memory_map: true,
            thread_count: Some(1),
            chunk_size: 1024,
        }),
        ("all_optimizations", ProcessingConfig::default()),
    ];
    
    for (temp_file, size) in &temp_files {
        group.throughput(Throughput::Elements(*size as u64));
        
        for (config_name, config) in &configs {
            let processor = DataProcessor::new(config.clone());
            
            group.bench_with_input(
                BenchmarkId::new(format!("pipeline_{}", config_name), size),
                size,
                |bench, _| {
                    bench.iter(|| {
                        processor.process_csv_file(black_box(temp_file.path())).unwrap()
                    });
                },
            );
        }
    }
    
    group.finish();
}

/// Benchmark scalar vs SIMD operations for comparison
fn bench_simd_vs_scalar(c: &mut Criterion) {
    let mut group = c.benchmark_group("simd_vs_scalar");
    
    let size = 10000;
    let a: Vec<f64> = (0..size).map(|i| i as f64).collect();
    let b: Vec<f64> = (0..size).map(|i| (i * 2) as f64).collect();
    
    group.throughput(Throughput::Elements(size as u64));
    
    let simd_ops = SimdOperations::new();
    
    // SIMD addition
    group.bench_function("simd_add", |bench| {
        bench.iter(|| {
            simd_ops.add_arrays(black_box(&a), black_box(&b)).unwrap()
        });
    });
    
    // Scalar addition
    group.bench_function("scalar_add", |bench| {
        bench.iter(|| {
            let result: Vec<f64> = a.iter()
                .zip(b.iter())
                .map(|(x, y)| x + y)
                .collect();
            black_box(result)
        });
    });
    
    // SIMD dot product
    group.bench_function("simd_dot_product", |bench| {
        bench.iter(|| {
            simd_ops.dot_product(black_box(&a), black_box(&b)).unwrap()
        });
    });
    
    // Scalar dot product
    group.bench_function("scalar_dot_product", |bench| {
        bench.iter(|| {
            let result: f64 = a.iter()
                .zip(b.iter())
                .map(|(x, y)| x * y)
                .sum();
            black_box(result)
        });
    });
    
    group.finish();
}

/// Benchmark different chunk sizes for parallel processing
fn bench_chunk_sizes(c: &mut Criterion) {
    let mut group = c.benchmark_group("chunk_sizes");
    
    let size = 100000;
    let data: Vec<i32> = (0..size).collect();
    let processor = ParallelProcessor::new(Some(4));
    
    group.throughput(Throughput::Elements(size as u64));
    
    // Test different chunk sizes
    for chunk_size in [100, 1000, 10000, 50000].iter() {
        group.bench_with_input(
            BenchmarkId::new("process_chunks", chunk_size),
            chunk_size,
            |bench, &chunk_size| {
                bench.iter(|| {
                    processor.process_chunks(black_box(&data), chunk_size, |chunk| {
                        chunk.iter().map(|&x| x * 2).sum::<i32>()
                    })
                });
            },
        );
    }
    
    group.finish();
}

/// Benchmark memory allocation patterns
fn bench_memory_patterns(c: &mut Criterion) {
    let mut group = c.benchmark_group("memory_patterns");
    
    let size = 10000;
    
    // Pre-allocated vector reuse
    group.bench_function("preallocated_reuse", |bench| {
        let mut result = Vec::with_capacity(size);
        bench.iter(|| {
            result.clear();
            for i in 0..size {
                result.push(black_box(i as f64 * 2.0));
            }
            black_box(&result);
        });
    });
    
    // New allocation each time
    group.bench_function("new_allocation", |bench| {
        bench.iter(|| {
            let mut result = Vec::new();
            for i in 0..size {
                result.push(black_box(i as f64 * 2.0));
            }
            black_box(result);
        });
    });
    
    // Iterator collect
    group.bench_function("iterator_collect", |bench| {
        bench.iter(|| {
            let result: Vec<f64> = (0..size)
                .map(|i| black_box(i as f64 * 2.0))
                .collect();
            black_box(result);
        });
    });
    
    group.finish();
}

/// Custom benchmark configuration
fn custom_criterion() -> Criterion {
    Criterion::default()
        .sample_size(100)           // Number of samples per benchmark
        .measurement_time(std::time::Duration::from_secs(10))  // Time to spend on each benchmark
        .warm_up_time(std::time::Duration::from_secs(3))       // Warm-up time
}

criterion_group!(
    name = benches;
    config = custom_criterion();
    targets = 
        bench_simd_operations,
        bench_parallel_processing,
        bench_memory_mapped_io,
        bench_complete_pipeline,
        bench_simd_vs_scalar,
        bench_chunk_sizes,
        bench_memory_patterns
);

criterion_main!(benches);