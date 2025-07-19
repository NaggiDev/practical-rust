//! Configuration structures and utilities for the DSL.
//!
//! This module provides configuration types that work with the DSL macros
//! to provide a complete configuration system.

use crate::traits::{ExecutorConfig, ResourceRequirements};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;

/// Complete engine configuration that can be loaded from files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineConfiguration {
    pub executor: ExecutorSettings,
    pub resources: ResourceSettings,
    pub monitoring: MonitoringSettings,
    pub logging: LoggingSettings,
    pub features: FeatureSettings,
}

impl Default for EngineConfiguration {
    fn default() -> Self {
        Self {
            executor: ExecutorSettings::default(),
            resources: ResourceSettings::default(),
            monitoring: MonitoringSettings::default(),
            logging: LoggingSettings::default(),
            features: FeatureSettings::default(),
        }
    }
}

/// Executor-specific settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutorSettings {
    pub worker_threads: usize,
    pub queue_size: usize,
    pub max_concurrent_tasks: usize,
    pub enable_work_stealing: bool,
    pub task_timeout_seconds: Option<u64>,
    pub shutdown_timeout_seconds: u64,
}

impl Default for ExecutorSettings {
    fn default() -> Self {
        Self {
            worker_threads: num_cpus::get(),
            queue_size: 1000,
            max_concurrent_tasks: 100,
            enable_work_stealing: true,
            task_timeout_seconds: Some(300), // 5 minutes
            shutdown_timeout_seconds: 30,
        }
    }
}

impl From<ExecutorSettings> for ExecutorConfig {
    fn from(settings: ExecutorSettings) -> Self {
        Self {
            worker_threads: settings.worker_threads,
            queue_size: settings.queue_size,
            max_concurrent_tasks: settings.max_concurrent_tasks,
            enable_work_stealing: settings.enable_work_stealing,
            task_timeout: settings.task_timeout_seconds.map(Duration::from_secs),
            enable_metrics: true, // Always enable metrics
            health_check_interval: Duration::from_secs(30),
        }
    }
}

/// Resource management settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResourceSettings {
    pub total_memory_mb: u32,
    pub max_memory_per_task_mb: u32,
    pub max_cpu_per_task: f32,
    pub enable_resource_tracking: bool,
    pub resource_cleanup_interval_seconds: u64,
}

impl Default for ResourceSettings {
    fn default() -> Self {
        Self {
            total_memory_mb: 1024, // 1GB
            max_memory_per_task_mb: 100,
            max_cpu_per_task: 1.0,
            enable_resource_tracking: true,
            resource_cleanup_interval_seconds: 60,
        }
    }
}

/// Monitoring and metrics settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringSettings {
    pub enable_metrics: bool,
    pub metrics_collection_interval_seconds: u64,
    pub health_check_interval_seconds: u64,
    pub performance_tracking: bool,
    pub export_metrics: bool,
    pub metrics_export_interval_seconds: u64,
}

impl Default for MonitoringSettings {
    fn default() -> Self {
        Self {
            enable_metrics: true,
            metrics_collection_interval_seconds: 10,
            health_check_interval_seconds: 30,
            performance_tracking: true,
            export_metrics: false,
            metrics_export_interval_seconds: 60,
        }
    }
}

/// Logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LoggingSettings {
    pub level: String,
    pub enable_file_logging: bool,
    pub log_file_path: Option<String>,
    pub max_log_file_size_mb: u64,
    pub log_rotation_count: u32,
    pub enable_structured_logging: bool,
}

impl Default for LoggingSettings {
    fn default() -> Self {
        Self {
            level: "info".to_string(),
            enable_file_logging: false,
            log_file_path: None,
            max_log_file_size_mb: 100,
            log_rotation_count: 5,
            enable_structured_logging: true,
        }
    }
}

/// Feature flags and experimental settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeatureSettings {
    pub enable_ffi_operations: bool,
    pub enable_unsafe_optimizations: bool,
    pub enable_experimental_features: bool,
    pub debug_mode: bool,
    pub profiling_enabled: bool,
}

impl Default for FeatureSettings {
    fn default() -> Self {
        Self {
            enable_ffi_operations: true,
            enable_unsafe_optimizations: false,
            enable_experimental_features: false,
            debug_mode: false,
            profiling_enabled: false,
        }
    }
}

/// Task-specific configuration templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskTemplate {
    pub name: String,
    pub description: String,
    pub default_priority: u8,
    pub default_timeout_seconds: Option<u64>,
    pub default_retry_count: u32,
    pub resource_requirements: TaskResourceRequirements,
    pub tags: Vec<String>,
}

/// Resource requirements for task templates
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TaskResourceRequirements {
    pub memory_mb: u32,
    pub cpu_cores: f32,
    pub io_bandwidth_mbps: u32,
    pub network_bandwidth_mbps: u32,
}

impl From<TaskResourceRequirements> for ResourceRequirements {
    fn from(req: TaskResourceRequirements) -> Self {
        Self {
            memory_mb: req.memory_mb,
            cpu_cores: req.cpu_cores,
            io_bandwidth_mbps: req.io_bandwidth_mbps,
            network_bandwidth_mbps: req.network_bandwidth_mbps,
        }
    }
}

/// Configuration builder for fluent API
pub struct ConfigurationBuilder {
    config: EngineConfiguration,
}

impl ConfigurationBuilder {
    pub fn new() -> Self {
        Self {
            config: EngineConfiguration::default(),
        }
    }

    pub fn workers(mut self, count: usize) -> Self {
        self.config.executor.worker_threads = count;
        self
    }

    pub fn queue_size(mut self, size: usize) -> Self {
        self.config.executor.queue_size = size;
        self
    }

    pub fn enable_work_stealing(mut self, enabled: bool) -> Self {
        self.config.executor.enable_work_stealing = enabled;
        self
    }

    pub fn task_timeout(mut self, timeout: Duration) -> Self {
        self.config.executor.task_timeout_seconds = Some(timeout.as_secs());
        self
    }

    pub fn total_memory_mb(mut self, memory: u32) -> Self {
        self.config.resources.total_memory_mb = memory;
        self
    }

    pub fn enable_metrics(mut self, enabled: bool) -> Self {
        self.config.monitoring.enable_metrics = enabled;
        self
    }

    pub fn enable_ffi(mut self, enabled: bool) -> Self {
        self.config.features.enable_ffi_operations = enabled;
        self
    }

    pub fn debug_mode(mut self, enabled: bool) -> Self {
        self.config.features.debug_mode = enabled;
        self
    }

    pub fn build(self) -> EngineConfiguration {
        self.config
    }
}

impl Default for ConfigurationBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Configuration loader that can read from various sources
pub struct ConfigurationLoader;

impl ConfigurationLoader {
    /// Load configuration from JSON file
    pub fn from_json_file(path: &str) -> Result<EngineConfiguration, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let config: EngineConfiguration = serde_json::from_str(&content)?;
        Ok(config)
    }

    /// Load configuration from JSON string
    pub fn from_json_str(json: &str) -> Result<EngineConfiguration, Box<dyn std::error::Error>> {
        let config: EngineConfiguration = serde_json::from_str(json)?;
        Ok(config)
    }

    /// Load configuration from environment variables
    pub fn from_env() -> EngineConfiguration {
        let mut config = EngineConfiguration::default();

        // Executor settings
        if let Ok(workers) = std::env::var("ENGINE_WORKERS") {
            if let Ok(count) = workers.parse::<usize>() {
                config.executor.worker_threads = count;
            }
        }

        if let Ok(queue_size) = std::env::var("ENGINE_QUEUE_SIZE") {
            if let Ok(size) = queue_size.parse::<usize>() {
                config.executor.queue_size = size;
            }
        }

        if let Ok(work_stealing) = std::env::var("ENGINE_WORK_STEALING") {
            config.executor.enable_work_stealing = work_stealing.to_lowercase() == "true";
        }

        // Resource settings
        if let Ok(memory) = std::env::var("ENGINE_TOTAL_MEMORY_MB") {
            if let Ok(mb) = memory.parse::<u32>() {
                config.resources.total_memory_mb = mb;
            }
        }

        // Feature settings
        if let Ok(ffi) = std::env::var("ENGINE_ENABLE_FFI") {
            config.features.enable_ffi_operations = ffi.to_lowercase() == "true";
        }

        if let Ok(debug) = std::env::var("ENGINE_DEBUG_MODE") {
            config.features.debug_mode = debug.to_lowercase() == "true";
        }

        config
    }

    /// Save configuration to JSON file
    pub fn save_to_json_file(
        config: &EngineConfiguration,
        path: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let json = serde_json::to_string_pretty(config)?;
        std::fs::write(path, json)?;
        Ok(())
    }
}

/// Predefined configuration templates
pub struct ConfigurationTemplates;

impl ConfigurationTemplates {
    /// High-performance configuration for CPU-intensive tasks
    pub fn high_performance() -> EngineConfiguration {
        ConfigurationBuilder::new()
            .workers(num_cpus::get() * 2)
            .queue_size(5000)
            .enable_work_stealing(true)
            .task_timeout(Duration::from_secs(600))
            .total_memory_mb(4096)
            .enable_metrics(true)
            .enable_ffi(true)
            .build()
    }

    /// Memory-optimized configuration for memory-intensive tasks
    pub fn memory_optimized() -> EngineConfiguration {
        ConfigurationBuilder::new()
            .workers(num_cpus::get())
            .queue_size(1000)
            .enable_work_stealing(false)
            .task_timeout(Duration::from_secs(1800))
            .total_memory_mb(8192)
            .enable_metrics(true)
            .enable_ffi(true)
            .build()
    }

    /// Development configuration with debugging enabled
    pub fn development() -> EngineConfiguration {
        ConfigurationBuilder::new()
            .workers(2)
            .queue_size(100)
            .enable_work_stealing(false)
            .task_timeout(Duration::from_secs(60))
            .total_memory_mb(512)
            .enable_metrics(true)
            .enable_ffi(true)
            .debug_mode(true)
            .build()
    }

    /// Production configuration with optimizations
    pub fn production() -> EngineConfiguration {
        ConfigurationBuilder::new()
            .workers(num_cpus::get())
            .queue_size(2000)
            .enable_work_stealing(true)
            .task_timeout(Duration::from_secs(300))
            .total_memory_mb(2048)
            .enable_metrics(true)
            .enable_ffi(true)
            .debug_mode(false)
            .build()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_configuration_builder() {
        let config = ConfigurationBuilder::new()
            .workers(8)
            .queue_size(2000)
            .enable_work_stealing(true)
            .total_memory_mb(1024)
            .build();

        assert_eq!(config.executor.worker_threads, 8);
        assert_eq!(config.executor.queue_size, 2000);
        assert_eq!(config.executor.enable_work_stealing, true);
        assert_eq!(config.resources.total_memory_mb, 1024);
    }

    #[test]
    fn test_json_serialization() {
        let config = EngineConfiguration::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: EngineConfiguration = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.executor.worker_threads, deserialized.executor.worker_threads);
        assert_eq!(config.resources.total_memory_mb, deserialized.resources.total_memory_mb);
    }

    #[test]
    fn test_configuration_templates() {
        let high_perf = ConfigurationTemplates::high_performance();
        let memory_opt = ConfigurationTemplates::memory_optimized();
        let dev = ConfigurationTemplates::development();
        let prod = ConfigurationTemplates::production();

        assert!(high_perf.executor.worker_threads >= num_cpus::get());
        assert!(memory_opt.resources.total_memory_mb > high_perf.resources.total_memory_mb);
        assert!(dev.features.debug_mode);
        assert!(!prod.features.debug_mode);
    }

    #[test]
    fn test_executor_config_conversion() {
        let settings = ExecutorSettings {
            worker_threads: 4,
            queue_size: 500,
            max_concurrent_tasks: 50,
            enable_work_stealing: false,
            task_timeout_seconds: Some(120),
            shutdown_timeout_seconds: 15,
        };

        let config: ExecutorConfig = settings.into();
        assert_eq!(config.worker_threads, 4);
        assert_eq!(config.queue_size, 500);
        assert_eq!(config.max_concurrent_tasks, 50);
        assert_eq!(config.enable_work_stealing, false);
        assert_eq!(config.task_timeout, Some(Duration::from_secs(120)));
    }
}