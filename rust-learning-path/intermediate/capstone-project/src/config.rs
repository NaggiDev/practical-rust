//! Configuration management for the task queue system
//! 
//! This module demonstrates configuration patterns and validation

use std::path::PathBuf;
use std::time::Duration;
use serde::{Serialize, Deserialize};

/// Main configuration for the task queue system
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    /// Number of worker threads
    pub worker_count: usize,
    
    /// Storage backend type ("json" or "memory")
    pub storage_type: String,
    
    /// Path for file-based storage
    pub storage_path: PathBuf,
    
    /// Default task timeout
    pub default_timeout: Duration,
    
    /// Maximum number of retries for failed tasks
    pub max_retries: u32,
    
    /// Worker thread configuration
    pub worker_config: WorkerConfig,
    
    /// Monitoring configuration
    pub monitoring: MonitoringConfig,
}

/// Worker thread configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkerConfig {
    /// How long to wait for new tasks before checking for shutdown
    pub poll_interval: Duration,
    
    /// Maximum time to wait for graceful shutdown
    pub shutdown_timeout: Duration,
    
    /// Whether to enable worker thread names for debugging
    pub enable_thread_names: bool,
}

/// Monitoring and logging configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    /// Whether to enable detailed logging
    pub enable_logging: bool,
    
    /// Log level (error, warn, info, debug, trace)
    pub log_level: String,
    
    /// Whether to collect performance metrics
    pub collect_metrics: bool,
    
    /// How often to report statistics
    pub stats_interval: Duration,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            worker_count: num_cpus::get(),
            storage_type: "memory".to_string(),
            storage_path: PathBuf::from("./task_storage"),
            default_timeout: Duration::from_secs(300), // 5 minutes
            max_retries: 3,
            worker_config: WorkerConfig::default(),
            monitoring: MonitoringConfig::default(),
        }
    }
}

impl Default for WorkerConfig {
    fn default() -> Self {
        Self {
            poll_interval: Duration::from_millis(100),
            shutdown_timeout: Duration::from_secs(30),
            enable_thread_names: true,
        }
    }
}

impl Default for MonitoringConfig {
    fn default() -> Self {
        Self {
            enable_logging: true,
            log_level: "info".to_string(),
            collect_metrics: true,
            stats_interval: Duration::from_secs(60),
        }
    }
}

impl Config {
    /// Load configuration from a file
    pub fn from_file<P: AsRef<std::path::Path>>(path: P) -> crate::error::Result<Self> {
        let content = std::fs::read_to_string(path)?;
        let config: Config = serde_json::from_str(&content)?;
        config.validate()?;
        Ok(config)
    }
    
    /// Save configuration to a file
    pub fn to_file<P: AsRef<std::path::Path>>(&self, path: P) -> crate::error::Result<()> {
        let content = serde_json::to_string_pretty(self)?;
        std::fs::write(path, content)?;
        Ok(())
    }
    
    /// Validate the configuration
    pub fn validate(&self) -> crate::error::Result<()> {
        if self.worker_count == 0 {
            return Err(crate::error::TaskError::ConfigurationError(
                "Worker count must be greater than 0".to_string()
            ));
        }
        
        if !matches!(self.storage_type.as_str(), "json" | "memory") {
            return Err(crate::error::TaskError::ConfigurationError(
                format!("Invalid storage type: {}", self.storage_type)
            ));
        }
        
        if self.storage_type == "json" && self.storage_path.as_os_str().is_empty() {
            return Err(crate::error::TaskError::ConfigurationError(
                "Storage path must be specified for JSON storage".to_string()
            ));
        }
        
        if !matches!(self.monitoring.log_level.as_str(), "error" | "warn" | "info" | "debug" | "trace") {
            return Err(crate::error::TaskError::ConfigurationError(
                format!("Invalid log level: {}", self.monitoring.log_level)
            ));
        }
        
        Ok(())
    }
    
    /// Create a configuration optimized for development
    pub fn development() -> Self {
        Self {
            worker_count: 2,
            storage_type: "memory".to_string(),
            monitoring: MonitoringConfig {
                log_level: "debug".to_string(),
                stats_interval: Duration::from_secs(10),
                ..Default::default()
            },
            ..Default::default()
        }
    }
    
    /// Create a configuration optimized for production
    pub fn production() -> Self {
        Self {
            worker_count: num_cpus::get() * 2,
            storage_type: "json".to_string(),
            storage_path: PathBuf::from("/var/lib/task-queue"),
            monitoring: MonitoringConfig {
                log_level: "info".to_string(),
                stats_interval: Duration::from_secs(300),
                ..Default::default()
            },
            ..Default::default()
        }
    }
}

// Add num_cpus as a dependency placeholder (would be in Cargo.toml)
mod num_cpus {
    pub fn get() -> usize {
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;
    
    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert!(config.validate().is_ok());
        assert!(config.worker_count > 0);
        assert_eq!(config.storage_type, "memory");
    }
    
    #[test]
    fn test_config_validation() {
        let mut config = Config::default();
        
        // Test invalid worker count
        config.worker_count = 0;
        assert!(config.validate().is_err());
        
        // Test invalid storage type
        config.worker_count = 1;
        config.storage_type = "invalid".to_string();
        assert!(config.validate().is_err());
        
        // Test invalid log level
        config.storage_type = "memory".to_string();
        config.monitoring.log_level = "invalid".to_string();
        assert!(config.validate().is_err());
    }
    
    #[test]
    fn test_config_serialization() {
        let config = Config::development();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: Config = serde_json::from_str(&json).unwrap();
        
        assert_eq!(config.worker_count, deserialized.worker_count);
        assert_eq!(config.storage_type, deserialized.storage_type);
    }
    
    #[test]
    fn test_config_file_operations() {
        let dir = tempdir().unwrap();
        let config_path = dir.path().join("config.json");
        
        let config = Config::development();
        config.to_file(&config_path).unwrap();
        
        let loaded_config = Config::from_file(&config_path).unwrap();
        assert_eq!(config.worker_count, loaded_config.worker_count);
        assert_eq!(config.storage_type, loaded_config.storage_type);
    }
    
    #[test]
    fn test_preset_configs() {
        let dev_config = Config::development();
        assert_eq!(dev_config.worker_count, 2);
        assert_eq!(dev_config.monitoring.log_level, "debug");
        
        let prod_config = Config::production();
        assert!(prod_config.worker_count >= 4);
        assert_eq!(prod_config.monitoring.log_level, "info");
        assert_eq!(prod_config.storage_type, "json");
    }
}