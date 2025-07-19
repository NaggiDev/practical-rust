use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CoordinatorConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub scheduler: SchedulerConfig,
    pub aggregator: AggregatorConfig,
    pub metrics: MetricsConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub bind_address: String,
    pub port: u16,
    pub max_connections: u32,
    pub request_timeout_seconds: u64,
    pub enable_cors: bool,
    pub websocket_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub connection_timeout_seconds: u64,
    pub enable_migrations: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SchedulerConfig {
    pub max_queue_size: u32,
    pub worker_timeout_seconds: u64,
    pub job_timeout_seconds: u64,
    pub heartbeat_interval_seconds: u64,
    pub cleanup_interval_seconds: u64,
    pub load_balancing_strategy: LoadBalancingStrategy,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AggregatorConfig {
    pub result_cache_size: u32,
    pub result_retention_days: u32,
    pub batch_size: u32,
    pub flush_interval_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MetricsConfig {
    pub enabled: bool,
    pub bind_address: String,
    pub port: u16,
    pub collection_interval_seconds: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum LoadBalancingStrategy {
    RoundRobin,
    LeastLoaded,
    Random,
    Weighted,
}

impl Default for CoordinatorConfig {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                bind_address: "0.0.0.0".to_string(),
                port: 8080,
                max_connections: 1000,
                request_timeout_seconds: 30,
                enable_cors: true,
                websocket_enabled: true,
            },
            database: DatabaseConfig {
                url: "sqlite:coordinator.db".to_string(),
                max_connections: 10,
                connection_timeout_seconds: 30,
                enable_migrations: true,
            },
            scheduler: SchedulerConfig {
                max_queue_size: 10000,
                worker_timeout_seconds: 300,
                job_timeout_seconds: 3600,
                heartbeat_interval_seconds: 30,
                cleanup_interval_seconds: 300,
                load_balancing_strategy: LoadBalancingStrategy::LeastLoaded,
            },
            aggregator: AggregatorConfig {
                result_cache_size: 1000,
                result_retention_days: 30,
                batch_size: 100,
                flush_interval_seconds: 60,
            },
            metrics: MetricsConfig {
                enabled: true,
                bind_address: "0.0.0.0".to_string(),
                port: 9090,
                collection_interval_seconds: 10,
            },
        }
    }
}

impl CoordinatorConfig {
    pub fn load(path: &str) -> Result<Self> {
        let config_path = std::path::Path::new(path);
        
        if !config_path.exists() {
            tracing::warn!("Configuration file {} not found, using defaults", path);
            return Ok(Self::default());
        }

        let config_str = std::fs::read_to_string(config_path)?;
        let config: CoordinatorConfig = toml::from_str(&config_str)?;
        
        // Validate configuration
        config.validate()?;
        
        Ok(config)
    }

    pub fn save(&self, path: &str) -> Result<()> {
        let config_str = toml::to_string_pretty(self)?;
        std::fs::write(path, config_str)?;
        Ok(())
    }

    fn validate(&self) -> Result<()> {
        if self.server.port == 0 {
            anyhow::bail!("Server port cannot be 0");
        }

        if self.database.max_connections == 0 {
            anyhow::bail!("Database max_connections cannot be 0");
        }

        if self.scheduler.max_queue_size == 0 {
            anyhow::bail!("Scheduler max_queue_size cannot be 0");
        }

        if self.metrics.enabled && self.metrics.port == 0 {
            anyhow::bail!("Metrics port cannot be 0 when metrics are enabled");
        }

        Ok(())
    }

    pub fn server_timeout(&self) -> Duration {
        Duration::from_secs(self.server.request_timeout_seconds)
    }

    pub fn worker_timeout(&self) -> Duration {
        Duration::from_secs(self.scheduler.worker_timeout_seconds)
    }

    pub fn job_timeout(&self) -> Duration {
        Duration::from_secs(self.scheduler.job_timeout_seconds)
    }

    pub fn heartbeat_interval(&self) -> Duration {
        Duration::from_secs(self.scheduler.heartbeat_interval_seconds)
    }

    pub fn cleanup_interval(&self) -> Duration {
        Duration::from_secs(self.scheduler.cleanup_interval_seconds)
    }
}