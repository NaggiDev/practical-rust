//! Task execution traits and implementations.

use super::{Task, TaskExecutor, Schedulable, Monitorable, HealthStatus};
use crate::error::Result;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use std::sync::atomic::{AtomicU64, AtomicUsize, Ordering};
use std::sync::Arc;
use std::time::{Duration, Instant};

/// Advanced task executor with scheduling and monitoring capabilities
pub trait AdvancedExecutor: TaskExecutor + Schedulable + Monitorable {
    /// Submit a task for execution with custom scheduling options
    fn submit_with_options<T: Task>(
        &self,
        task: T,
        options: ExecutionOptions,
    ) -> Pin<Box<dyn Future<Output = Result<T::Output>> + Send + '_>>;

    /// Cancel a running task by ID
    fn cancel_task(&self, task_id: u64) -> Pin<Box<dyn Future<Output = Result<bool>> + Send + '_>>;

    /// Get execution statistics
    fn statistics(&self) -> ExecutionStatistics;

    /// Configure the executor
    fn configure(&self, config: ExecutorConfig) -> Result<()>;
}

/// Options for task execution
#[derive(Debug, Clone)]
pub struct ExecutionOptions {
    pub priority: u8,
    pub timeout: Option<Duration>,
    pub retry_count: u32,
    pub retry_delay: Duration,
    pub resource_requirements: ResourceRequirements,
}

impl Default for ExecutionOptions {
    fn default() -> Self {
        Self {
            priority: 0,
            timeout: None,
            retry_count: 0,
            retry_delay: Duration::from_millis(100),
            resource_requirements: ResourceRequirements::default(),
        }
    }
}

/// Resource requirements for task execution
#[derive(Debug, Clone)]
pub struct ResourceRequirements {
    pub memory_mb: u32,
    pub cpu_cores: f32,
    pub io_bandwidth_mbps: u32,
    pub network_bandwidth_mbps: u32,
}

impl Default for ResourceRequirements {
    fn default() -> Self {
        Self {
            memory_mb: 1,
            cpu_cores: 0.1,
            io_bandwidth_mbps: 0,
            network_bandwidth_mbps: 0,
        }
    }
}

/// Execution statistics for monitoring
#[derive(Debug, Clone)]
pub struct ExecutionStatistics {
    pub tasks_submitted: u64,
    pub tasks_completed: u64,
    pub tasks_failed: u64,
    pub tasks_cancelled: u64,
    pub average_execution_time: Duration,
    pub average_queue_time: Duration,
    pub throughput_per_second: f64,
    pub error_rate: f64,
    pub uptime: Duration,
}

impl ExecutionStatistics {
    pub fn new() -> Self {
        Self {
            tasks_submitted: 0,
            tasks_completed: 0,
            tasks_failed: 0,
            tasks_cancelled: 0,
            average_execution_time: Duration::ZERO,
            average_queue_time: Duration::ZERO,
            throughput_per_second: 0.0,
            error_rate: 0.0,
            uptime: Duration::ZERO,
        }
    }

    pub fn success_rate(&self) -> f64 {
        if self.tasks_submitted == 0 {
            0.0
        } else {
            self.tasks_completed as f64 / self.tasks_submitted as f64
        }
    }
}

/// Configuration for executor behavior
#[derive(Debug, Clone)]
pub struct ExecutorConfig {
    pub max_concurrent_tasks: usize,
    pub queue_size: usize,
    pub worker_threads: usize,
    pub enable_work_stealing: bool,
    pub enable_metrics: bool,
    pub health_check_interval: Duration,
    pub task_timeout: Option<Duration>,
}

impl Default for ExecutorConfig {
    fn default() -> Self {
        Self {
            max_concurrent_tasks: 100,
            queue_size: 1000,
            worker_threads: num_cpus::get(),
            enable_work_stealing: true,
            enable_metrics: true,
            health_check_interval: Duration::from_secs(30),
            task_timeout: Some(Duration::from_secs(300)), // 5 minutes
        }
    }
}

/// Statistics tracker for executors
#[derive(Debug)]
pub struct StatisticsTracker {
    tasks_submitted: AtomicU64,
    tasks_completed: AtomicU64,
    tasks_failed: AtomicU64,
    tasks_cancelled: AtomicU64,
    total_execution_time: AtomicU64, // in nanoseconds
    total_queue_time: AtomicU64,     // in nanoseconds
    start_time: Instant,
}

impl StatisticsTracker {
    pub fn new() -> Self {
        Self {
            tasks_submitted: AtomicU64::new(0),
            tasks_completed: AtomicU64::new(0),
            tasks_failed: AtomicU64::new(0),
            tasks_cancelled: AtomicU64::new(0),
            total_execution_time: AtomicU64::new(0),
            total_queue_time: AtomicU64::new(0),
            start_time: Instant::now(),
        }
    }

    pub fn task_submitted(&self) {
        self.tasks_submitted.fetch_add(1, Ordering::Relaxed);
    }

    pub fn task_completed(&self, execution_time: Duration, queue_time: Duration) {
        self.tasks_completed.fetch_add(1, Ordering::Relaxed);
        self.total_execution_time
            .fetch_add(execution_time.as_nanos() as u64, Ordering::Relaxed);
        self.total_queue_time
            .fetch_add(queue_time.as_nanos() as u64, Ordering::Relaxed);
    }

    pub fn task_failed(&self) {
        self.tasks_failed.fetch_add(1, Ordering::Relaxed);
    }

    pub fn task_cancelled(&self) {
        self.tasks_cancelled.fetch_add(1, Ordering::Relaxed);
    }

    pub fn get_statistics(&self) -> ExecutionStatistics {
        let submitted = self.tasks_submitted.load(Ordering::Relaxed);
        let completed = self.tasks_completed.load(Ordering::Relaxed);
        let failed = self.tasks_failed.load(Ordering::Relaxed);
        let cancelled = self.tasks_cancelled.load(Ordering::Relaxed);
        let total_exec_time = self.total_execution_time.load(Ordering::Relaxed);
        let total_queue_time = self.total_queue_time.load(Ordering::Relaxed);

        let avg_execution_time = if completed > 0 {
            Duration::from_nanos(total_exec_time / completed)
        } else {
            Duration::ZERO
        };

        let avg_queue_time = if completed > 0 {
            Duration::from_nanos(total_queue_time / completed)
        } else {
            Duration::ZERO
        };

        let uptime = self.start_time.elapsed();
        let throughput = if uptime.as_secs() > 0 {
            completed as f64 / uptime.as_secs() as f64
        } else {
            0.0
        };

        let error_rate = if submitted > 0 {
            failed as f64 / submitted as f64
        } else {
            0.0
        };

        ExecutionStatistics {
            tasks_submitted: submitted,
            tasks_completed: completed,
            tasks_failed: failed,
            tasks_cancelled: cancelled,
            average_execution_time: avg_execution_time,
            average_queue_time: avg_queue_time,
            throughput_per_second: throughput,
            error_rate,
            uptime,
        }
    }
}

impl Default for StatisticsTracker {
    fn default() -> Self {
        Self::new()
    }
}