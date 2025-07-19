//! Core task execution engine implementation.
//!
//! This module contains the main TaskEngine implementation that combines
//! all the advanced Rust concepts: concurrency, unsafe Rust, FFI, and macros.

pub mod thread_pool;
pub mod task_queue;
pub mod scheduler;

use crate::error::{Result, EngineError};
use crate::ffi::{MathOperation, StringOperation, ArrayOperation};
use crate::traits::{Task, TaskExecutor, ExecutorConfig, ExecutionStatistics, StatisticsTracker};
use crate::memory::BasicResourceManager;
use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::{mpsc, oneshot};

/// Main task execution engine
pub struct TaskEngine {
    thread_pool: Arc<thread_pool::WorkStealingThreadPool>,
    resource_manager: Arc<BasicResourceManager>,
    statistics: Arc<StatisticsTracker>,
    config: ExecutorConfig,
    shutdown_sender: Option<oneshot::Sender<()>>,
}

impl TaskEngine {
    /// Create a new TaskEngine with default configuration
    pub fn new() -> Result<Self> {
        Self::with_config(ExecutorConfig::default())
    }

    /// Create a new TaskEngine with custom configuration
    pub fn with_config(config: ExecutorConfig) -> Result<Self> {
        let thread_pool = Arc::new(thread_pool::WorkStealingThreadPool::new(
            config.worker_threads,
            config.queue_size,
        )?);

        let resource_manager = Arc::new(BasicResourceManager::new(1000)); // 1000 units capacity
        let statistics = Arc::new(StatisticsTracker::new());

        Ok(Self {
            thread_pool,
            resource_manager,
            statistics,
            config,
            shutdown_sender: None,
        })
    }

    /// Create a builder for configuring the engine
    pub fn builder() -> TaskEngineBuilder {
        TaskEngineBuilder::new()
    }

    /// Submit a mathematical task for execution
    pub async fn submit_math_task(
        &self,
        operation: MathOperation,
        args: Vec<i64>,
    ) -> Result<i64> {
        let task = MathTask::new(operation, args);
        self.submit(task).await
    }

    /// Submit a string processing task for execution
    pub async fn submit_string_task(
        &self,
        operation: StringOperation,
        input: String,
    ) -> Result<String> {
        let task = StringTask::new(operation, input);
        self.submit(task).await
    }

    /// Submit an array processing task for execution
    pub async fn submit_array_task(
        &self,
        operation: ArrayOperation,
        array: Vec<i64>,
    ) -> Result<i64> {
        let task = ArrayTask::new(operation, array);
        self.submit(task).await
    }

    /// Get execution statistics
    pub fn statistics(&self) -> ExecutionStatistics {
        self.statistics.get_statistics()
    }

    /// Get current configuration
    pub fn config(&self) -> &ExecutorConfig {
        &self.config
    }
}

impl TaskExecutor for TaskEngine {
    fn execute_task<T: Task>(
        &self,
        task: T,
    ) -> Pin<Box<dyn Future<Output = Result<T::Output>> + Send + '_>> {
        Box::pin(async move {
            let task_id = task.id();
            let estimated_cost = task.estimated_cost();
            
            // Record task submission
            self.statistics.task_submitted();
            let submit_time = Instant::now();

            // Allocate resources
            let _resource_handle = self.resource_manager
                .allocate_resources(task_id, estimated_cost)
                .map_err(|e| EngineError::task_execution(format!("Resource allocation failed: {}", e)))?;

            // Execute the task
            let execution_start = Instant::now();
            let queue_time = execution_start.duration_since(submit_time);
            
            match task.execute().await {
                Ok(result) => {
                    let execution_time = execution_start.elapsed();
                    self.statistics.task_completed(execution_time, queue_time);
                    Ok(result)
                }
                Err(e) => {
                    self.statistics.task_failed();
                    Err(e)
                }
            }
        })
    }

    fn active_tasks(&self) -> usize {
        self.thread_pool.active_tasks()
    }

    fn queued_tasks(&self) -> usize {
        self.thread_pool.queued_tasks()
    }

    fn shutdown(&self) -> Pin<Box<dyn Future<Output = Result<()>> + Send + '_>> {
        Box::pin(async move {
            self.thread_pool.shutdown().await
        })
    }
}

/// Builder for TaskEngine configuration
pub struct TaskEngineBuilder {
    config: ExecutorConfig,
}

impl TaskEngineBuilder {
    pub fn new() -> Self {
        Self {
            config: ExecutorConfig::default(),
        }
    }

    pub fn workers(mut self, count: usize) -> Self {
        self.config.worker_threads = count;
        self
    }

    pub fn queue_size(mut self, size: usize) -> Self {
        self.config.queue_size = size;
        self
    }

    pub fn enable_work_stealing(mut self, enabled: bool) -> Self {
        self.config.enable_work_stealing = enabled;
        self
    }

    pub fn task_timeout(mut self, timeout: Duration) -> Self {
        self.config.task_timeout = Some(timeout);
        self
    }

    pub fn build(self) -> Result<TaskEngine> {
        TaskEngine::with_config(self.config)
    }
}

impl Default for TaskEngineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Mathematical task implementation
#[derive(Debug, Clone)]
pub struct MathTask {
    id: u64,
    operation: MathOperation,
    args: Vec<i64>,
}

impl MathTask {
    pub fn new(operation: MathOperation, args: Vec<i64>) -> Self {
        Self {
            id: generate_task_id(),
            operation,
            args,
        }
    }
}

impl Task for MathTask {
    type Output = i64;

    fn id(&self) -> u64 {
        self.id
    }

    fn name(&self) -> &str {
        "MathTask"
    }

    fn execute(&self) -> Pin<Box<dyn Future<Output = Result<Self::Output>> + Send + '_>> {
        Box::pin(async move {
            crate::ffi::execute_math_operation(self.operation.clone(), &self.args)
        })
    }

    fn estimated_cost(&self) -> u32 {
        match self.operation {
            MathOperation::Factorial => 10,
            MathOperation::Fibonacci => 15,
            MathOperation::SquareRoot => 5,
            MathOperation::GreatestCommonDivisor => 8,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// String processing task implementation
#[derive(Debug, Clone)]
pub struct StringTask {
    id: u64,
    operation: StringOperation,
    input: String,
}

impl StringTask {
    pub fn new(operation: StringOperation, input: String) -> Self {
        Self {
            id: generate_task_id(),
            operation,
            input,
        }
    }
}

impl Task for StringTask {
    type Output = String;

    fn id(&self) -> u64 {
        self.id
    }

    fn name(&self) -> &str {
        "StringTask"
    }

    fn execute(&self) -> Pin<Box<dyn Future<Output = Result<Self::Output>> + Send + '_>> {
        Box::pin(async move {
            crate::ffi::execute_string_operation(self.operation.clone(), self.input.clone())
        })
    }

    fn estimated_cost(&self) -> u32 {
        match self.operation {
            StringOperation::Reverse => 2,
            StringOperation::Uppercase => 3,
            StringOperation::Hash => 5,
        }
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Array processing task implementation
#[derive(Debug, Clone)]
pub struct ArrayTask {
    id: u64,
    operation: ArrayOperation,
    array: Vec<i64>,
}

impl ArrayTask {
    pub fn new(operation: ArrayOperation, array: Vec<i64>) -> Self {
        Self {
            id: generate_task_id(),
            operation,
            array,
        }
    }
}

impl Task for ArrayTask {
    type Output = i64;

    fn id(&self) -> u64 {
        self.id
    }

    fn name(&self) -> &str {
        "ArrayTask"
    }

    fn execute(&self) -> Pin<Box<dyn Future<Output = Result<Self::Output>> + Send + '_>> {
        Box::pin(async move {
            crate::ffi::execute_array_operation(self.operation.clone(), self.array.clone())
        })
    }

    fn estimated_cost(&self) -> u32 {
        let base_cost = match self.operation {
            ArrayOperation::Sum => 1,
            ArrayOperation::Max => 1,
            ArrayOperation::Sort => 5,
        };
        base_cost + (self.array.len() as u32 / 100) // Scale with array size
    }

    fn as_any(&self) -> &dyn Any {
        self
    }
}

/// Generate unique task IDs
fn generate_task_id() -> u64 {
    use std::sync::atomic::{AtomicU64, Ordering};
    static COUNTER: AtomicU64 = AtomicU64::new(1);
    COUNTER.fetch_add(1, Ordering::Relaxed)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ffi::{MathOperation, StringOperation, ArrayOperation};

    #[tokio::test]
    async fn test_engine_creation() {
        let engine = TaskEngine::new().unwrap();
        assert_eq!(engine.active_tasks(), 0);
        assert_eq!(engine.queued_tasks(), 0);
    }

    #[tokio::test]
    async fn test_engine_builder() {
        let engine = TaskEngine::builder()
            .workers(4)
            .queue_size(500)
            .enable_work_stealing(true)
            .build()
            .unwrap();

        assert_eq!(engine.config().worker_threads, 4);
        assert_eq!(engine.config().queue_size, 500);
        assert_eq!(engine.config().enable_work_stealing, true);
    }

    #[tokio::test]
    async fn test_math_task_execution() {
        let engine = TaskEngine::new().unwrap();
        let result = engine.submit_math_task(MathOperation::Factorial, vec![5]).await.unwrap();
        assert_eq!(result, 120);
    }

    #[tokio::test]
    async fn test_string_task_execution() {
        let engine = TaskEngine::new().unwrap();
        let result = engine.submit_string_task(StringOperation::Reverse, "hello".to_string()).await.unwrap();
        assert_eq!(result, "olleh");
    }

    #[tokio::test]
    async fn test_array_task_execution() {
        let engine = TaskEngine::new().unwrap();
        let result = engine.submit_array_task(ArrayOperation::Sum, vec![1, 2, 3, 4, 5]).await.unwrap();
        assert_eq!(result, 15);
    }

    #[tokio::test]
    async fn test_task_statistics() {
        let engine = TaskEngine::new().unwrap();
        
        // Submit a few tasks
        let _ = engine.submit_math_task(MathOperation::Factorial, vec![3]).await;
        let _ = engine.submit_string_task(StringOperation::Uppercase, "test".to_string()).await;
        
        let stats = engine.statistics();
        assert!(stats.tasks_submitted >= 2);
    }

    #[test]
    fn test_task_id_generation() {
        let id1 = generate_task_id();
        let id2 = generate_task_id();
        assert_ne!(id1, id2);
        assert!(id2 > id1);
    }

    #[test]
    fn test_task_cost_estimation() {
        let math_task = MathTask::new(MathOperation::Factorial, vec![5]);
        let string_task = StringTask::new(StringOperation::Hash, "test".to_string());
        let array_task = ArrayTask::new(ArrayOperation::Sort, vec![1, 2, 3]);

        assert!(math_task.estimated_cost() > 0);
        assert!(string_task.estimated_cost() > 0);
        assert!(array_task.estimated_cost() > 0);
    }
}