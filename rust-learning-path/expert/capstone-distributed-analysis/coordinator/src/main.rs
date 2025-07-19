use anyhow::Result;
use clap::Parser;
use std::sync::Arc;
use tracing::{info, error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod api;
mod scheduler;
mod aggregator;
mod storage;
mod metrics;
mod config;

use crate::config::CoordinatorConfig;

#[derive(Parser)]
#[command(name = "coordinator")]
#[command(about = "Distributed Code Analysis Coordinator")]
struct Args {
    #[arg(short, long, default_value = "coordinator.toml")]
    config: String,
    
    #[arg(short, long, default_value = "info")]
    log_level: String,
    
    #[arg(long)]
    bind_address: Option<String>,
    
    #[arg(long)]
    port: Option<u16>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    
    // Initialize tracing
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| format!("coordinator={}", args.log_level).into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    info!("Starting Distributed Code Analysis Coordinator");

    // Load configuration
    let config = CoordinatorConfig::load(&args.config)?;
    info!("Loaded configuration from {}", args.config);

    // Override config with command line arguments
    let mut config = config;
    if let Some(bind_address) = args.bind_address {
        config.server.bind_address = bind_address;
    }
    if let Some(port) = args.port {
        config.server.port = port;
    }

    // Initialize storage
    let storage = Arc::new(storage::Storage::new(&config.database).await?);
    info!("Initialized storage backend");

    // Initialize metrics
    let metrics_handle = metrics::init_metrics(&config.metrics)?;
    info!("Initialized metrics collection");

    // Initialize scheduler
    let scheduler = Arc::new(scheduler::Scheduler::new(
        storage.clone(),
        config.scheduler.clone(),
    ).await?);
    info!("Initialized job scheduler");

    // Initialize results aggregator
    let aggregator = Arc::new(aggregator::Aggregator::new(
        storage.clone(),
        config.aggregator.clone(),
    ).await?);
    info!("Initialized results aggregator");

    // Start background tasks
    let scheduler_handle = {
        let scheduler = scheduler.clone();
        tokio::spawn(async move {
            if let Err(e) = scheduler.run().await {
                error!("Scheduler error: {}", e);
            }
        })
    };

    let aggregator_handle = {
        let aggregator = aggregator.clone();
        tokio::spawn(async move {
            if let Err(e) = aggregator.run().await {
                error!("Aggregator error: {}", e);
            }
        })
    };

    // Start API server
    let api_server = api::Server::new(
        scheduler.clone(),
        aggregator.clone(),
        storage.clone(),
        config.server.clone(),
    );

    info!("Starting API server on {}:{}", config.server.bind_address, config.server.port);
    
    // Run the server and wait for shutdown
    tokio::select! {
        result = api_server.run() => {
            if let Err(e) = result {
                error!("API server error: {}", e);
            }
        }
        _ = tokio::signal::ctrl_c() => {
            info!("Received shutdown signal");
        }
    }

    info!("Shutting down coordinator...");

    // Graceful shutdown
    scheduler_handle.abort();
    aggregator_handle.abort();
    
    // Wait for background tasks to complete
    let _ = tokio::join!(scheduler_handle, aggregator_handle);

    info!("Coordinator shutdown complete");
    Ok(())
}

/// Initialize panic handler for better error reporting
fn init_panic_handler() {
    std::panic::set_hook(Box::new(|panic_info| {
        let backtrace = std::backtrace::Backtrace::capture();
        error!("Panic occurred: {}\nBacktrace:\n{}", panic_info, backtrace);
    }));
}