//! Macro definitions for the task execution DSL.
//!
//! This module contains declarative macros that provide a convenient syntax
//! for defining tasks and configuring the execution engine.

/// Macro for defining mathematical tasks with a clean syntax
#[macro_export]
macro_rules! math_task {
    ($op:ident, $($arg:expr),+ $(,)?) => {
        {
            use $crate::ffi::MathOperation;
            use $crate::engine::MathTask;
            
            let operation = match stringify!($op) {
                "factorial" => MathOperation::Factorial,
                "fibonacci" => MathOperation::Fibonacci,
                "sqrt" => MathOperation::SquareRoot,
                "gcd" => MathOperation::GreatestCommonDivisor,
                _ => panic!("Unknown math operation: {}", stringify!($op)),
            };
            
            MathTask::new(operation, vec![$($arg),+])
        }
    };
}

/// Macro for defining string tasks with a clean syntax
#[macro_export]
macro_rules! string_task {
    ($op:ident, $input:expr) => {
        {
            use $crate::ffi::StringOperation;
            use $crate::engine::StringTask;
            
            let operation = match stringify!($op) {
                "reverse" => StringOperation::Reverse,
                "uppercase" => StringOperation::Uppercase,
                "hash" => StringOperation::Hash,
                _ => panic!("Unknown string operation: {}", stringify!($op)),
            };
            
            StringTask::new(operation, $input.to_string())
        }
    };
}

/// Macro for defining array tasks with a clean syntax
#[macro_export]
macro_rules! array_task {
    ($op:ident, [$($elem:expr),* $(,)?]) => {
        {
            use $crate::ffi::ArrayOperation;
            use $crate::engine::ArrayTask;
            
            let operation = match stringify!($op) {
                "sum" => ArrayOperation::Sum,
                "max" => ArrayOperation::Max,
                "sort" => ArrayOperation::Sort,
                _ => panic!("Unknown array operation: {}", stringify!($op)),
            };
            
            ArrayTask::new(operation, vec![$($elem),*])
        }
    };
}

/// Macro for configuring task execution options
#[macro_export]
macro_rules! task_options {
    (
        $(priority: $priority:expr,)?
        $(timeout: $timeout:expr,)?
        $(retry_count: $retry_count:expr,)?
        $(retry_delay: $retry_delay:expr,)?
        $(memory_mb: $memory_mb:expr,)?
        $(cpu_cores: $cpu_cores:expr,)?
        $(io_bandwidth_mbps: $io_bandwidth_mbps:expr,)?
        $(network_bandwidth_mbps: $network_bandwidth_mbps:expr,)?
    ) => {
        {
            use $crate::traits::{ExecutionOptions, ResourceRequirements};
            use std::time::Duration;
            
            let mut options = ExecutionOptions::default();
            let mut requirements = ResourceRequirements::default();
            
            $(options.priority = $priority;)?
            $(options.timeout = Some($timeout);)?
            $(options.retry_count = $retry_count;)?
            $(options.retry_delay = $retry_delay;)?
            $(requirements.memory_mb = $memory_mb;)?
            $(requirements.cpu_cores = $cpu_cores;)?
            $(requirements.io_bandwidth_mbps = $io_bandwidth_mbps;)?
            $(requirements.network_bandwidth_mbps = $network_bandwidth_mbps;)?
            
            options.resource_requirements = requirements;
            options
        }
    };
}

/// Macro for creating engine configuration
#[macro_export]
macro_rules! engine_config {
    (
        $(workers: $workers:expr,)?
        $(queue_size: $queue_size:expr,)?
        $(enable_work_stealing: $work_stealing:expr,)?
        $(enable_metrics: $metrics:expr,)?
        $(task_timeout: $timeout:expr,)?
        $(health_check_interval: $health_interval:expr,)?
    ) => {
        {
            use $crate::traits::ExecutorConfig;
            use std::time::Duration;
            
            let mut config = ExecutorConfig::default();
            
            $(config.worker_threads = $workers;)?
            $(config.queue_size = $queue_size;)?
            $(config.enable_work_stealing = $work_stealing;)?
            $(config.enable_metrics = $metrics;)?
            $(config.task_timeout = Some($timeout);)?
            $(config.health_check_interval = $health_interval;)?
            
            config
        }
    };
}

/// Macro for batch task submission
#[macro_export]
macro_rules! submit_batch {
    ($engine:expr, [$($task:expr),* $(,)?]) => {
        {
            use futures::future::join_all;
            
            let tasks = vec![$($task),*];
            let futures: Vec<_> = tasks.into_iter()
                .map(|task| $engine.submit(task))
                .collect();
            
            join_all(futures)
        }
    };
}

/// Macro for creating a task pipeline
#[macro_export]
macro_rules! task_pipeline {
    ($engine:expr, $($task:expr => $next:expr),+ $(,)?) => {
        {
            async move {
                let mut results = Vec::new();
                
                $(
                    let result = $engine.submit($task).await?;
                    results.push(result);
                    let $next = result; // Make result available for next task
                )+
                
                Ok::<Vec<_>, $crate::error::EngineError>(results)
            }
        }
    };
}

/// Macro for conditional task execution
#[macro_export]
macro_rules! conditional_task {
    (if $condition:expr => $task:expr) => {
        {
            if $condition {
                Some($task)
            } else {
                None
            }
        }
    };
    (if $condition:expr => $task:expr, else => $else_task:expr) => {
        {
            if $condition {
                $task
            } else {
                $else_task
            }
        }
    };
}

/// Macro for retry logic with exponential backoff
#[macro_export]
macro_rules! retry_task {
    ($engine:expr, $task:expr, retries: $retries:expr) => {
        {
            async move {
                let mut attempts = 0;
                let max_retries = $retries;
                
                loop {
                    match $engine.submit($task.clone()).await {
                        Ok(result) => return Ok(result),
                        Err(e) if attempts < max_retries => {
                            attempts += 1;
                            let delay = std::time::Duration::from_millis(100 * (1 << attempts));
                            tokio::time::sleep(delay).await;
                        }
                        Err(e) => return Err(e),
                    }
                }
            }
        }
    };
}

/// Macro for parallel task execution with different types
#[macro_export]
macro_rules! parallel_tasks {
    ($engine:expr, {
        $(math: $math_task:expr,)*
        $(string: $string_task:expr,)*
        $(array: $array_task:expr,)*
    }) => {
        {
            async move {
                let mut futures = Vec::new();
                
                $(
                    futures.push(Box::pin($engine.submit_math_task($math_task.operation, $math_task.args.clone())));
                )*
                $(
                    futures.push(Box::pin($engine.submit_string_task($string_task.operation, $string_task.input.clone())));
                )*
                $(
                    futures.push(Box::pin($engine.submit_array_task($array_task.operation, $array_task.array.clone())));
                )*
                
                futures::future::join_all(futures).await
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ffi::{MathOperation, StringOperation, ArrayOperation};
    use std::time::Duration;

    #[test]
    fn test_math_task_macro() {
        let task = math_task!(factorial, 5);
        // This would normally be tested with the actual task execution
        // For now, we just verify the macro compiles
    }

    #[test]
    fn test_string_task_macro() {
        let task = string_task!(reverse, "hello");
        // This would normally be tested with the actual task execution
        // For now, we just verify the macro compiles
    }

    #[test]
    fn test_array_task_macro() {
        let task = array_task!(sum, [1, 2, 3, 4, 5]);
        // This would normally be tested with the actual task execution
        // For now, we just verify the macro compiles
    }

    #[test]
    fn test_task_options_macro() {
        let options = task_options! {
            priority: 5,
            timeout: Duration::from_secs(30),
            retry_count: 3,
            memory_mb: 100,
            cpu_cores: 2.0,
        };
        
        assert_eq!(options.priority, 5);
        assert_eq!(options.timeout, Some(Duration::from_secs(30)));
        assert_eq!(options.retry_count, 3);
        assert_eq!(options.resource_requirements.memory_mb, 100);
        assert_eq!(options.resource_requirements.cpu_cores, 2.0);
    }

    #[test]
    fn test_engine_config_macro() {
        let config = engine_config! {
            workers: 8,
            queue_size: 2000,
            enable_work_stealing: true,
            enable_metrics: false,
        };
        
        assert_eq!(config.worker_threads, 8);
        assert_eq!(config.queue_size, 2000);
        assert_eq!(config.enable_work_stealing, true);
        assert_eq!(config.enable_metrics, false);
    }

    #[test]
    fn test_conditional_task_macro() {
        let condition = true;
        let task = math_task!(factorial, 5);
        
        let conditional = conditional_task!(if condition => task);
        assert!(conditional.is_some());
        
        let conditional = conditional_task!(if false => task);
        assert!(conditional.is_none());
    }
}