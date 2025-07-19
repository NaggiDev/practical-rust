use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;
use tokio::signal;
use tracing::{info, error};

mod config;
mod echo;
mod http;
mod chat;
mod balancer;
mod monitoring;
mod utils;

use config::ServerConfig;

#[derive(Parser)]
#[command(name = "async-server")]
#[command(about = "A high-performance asynchronous network server")]
struct Cli {
    /// Configuration file path
    #[arg(short, long)]
    config: Option<PathBuf>,
    
    /// Enable HTTP server
    #[arg(long, default_value = "true")]
    http: bool,
    
    /// Enable Echo server
    #[arg(long, default_value = "true")]
    echo: bool,
    
    /// Enable Chat server
    #[arg(long, default_value = "true")]
    chat: bool,
    
    /// Enable Load Balancer
    #[arg(long, default_value = "false")]
    balancer: bool,
    
    /// Enable Monitoring
    #[arg(long, default_value = "true")]
    monitoring: bool,
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt()
        .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
        .init();

    let cli = Cli::parse();
    
    // Load configuration
    let config = if let Some(config_path) = cli.config {
        let config_str = tokio::fs::read_to_string(config_path).await?;
        serde_json::from_str(&config_str)?
    } else {
        ServerConfig::default()
    };

    info!("Starting async server with configuration: {:?}", config);

    // Create shutdown channel
    let (shutdown_tx, _) = tokio::sync::broadcast::channel(1);

    // Start servers based on CLI flags
    let mut tasks = Vec::new();

    if cli.echo {
        let echo_server = echo::EchoServer::new(config.echo.clone());
        let shutdown_rx = shutdown_tx.subscribe();
        tasks.push(tokio::spawn(async move {
            if let Err(e) = echo_server.run(shutdown_rx).await {
                error!("Echo server error: {}", e);
            }
        }));
    }

    if cli.http {
        let http_server = http::HttpServer::new(config.http.clone());
        let shutdown_rx = shutdown_tx.subscribe();
        tasks.push(tokio::spawn(async move {
            if let Err(e) = http_server.run(shutdown_rx).await {
                error!("HTTP server error: {}", e);
            }
        }));
    }

    if cli.chat {
        let chat_server = chat::ChatServer::new(config.chat.clone());
        let shutdown_rx = shutdown_tx.subscribe();
        tasks.push(tokio::spawn(async move {
            if let Err(e) = chat_server.run(shutdown_rx).await {
                error!("Chat server error: {}", e);
            }
        }));
    }

    if cli.balancer {
        let balancer_server = balancer::BalancerServer::new(config.balancer.clone());
        let shutdown_rx = shutdown_tx.subscribe();
        tasks.push(tokio::spawn(async move {
            if let Err(e) = balancer_server.run(shutdown_rx).await {
                error!("Balancer server error: {}", e);
            }
        }));
    }

    if cli.monitoring {
        let monitoring_server = monitoring::MonitoringServer::new(config.monitoring.clone());
        let shutdown_rx = shutdown_tx.subscribe();
        tasks.push(tokio::spawn(async move {
            if let Err(e) = monitoring_server.run(shutdown_rx).await {
                error!("Monitoring server error: {}", e);
            }
        }));
    }

    // Wait for shutdown signal
    tokio::select! {
        _ = signal::ctrl_c() => {
            info!("Received Ctrl+C, shutting down...");
        }
        _ = wait_for_sigterm() => {
            info!("Received SIGTERM, shutting down...");
        }
    }

    // Send shutdown signal to all servers
    let _ = shutdown_tx.send(());

    // Wait for all servers to shut down
    for task in tasks {
        if let Err(e) = task.await {
            error!("Error waiting for server shutdown: {}", e);
        }
    }

    info!("All servers shut down successfully");
    Ok(())
}

#[cfg(unix)]
async fn wait_for_sigterm() {
    use tokio::signal::unix::{signal, SignalKind};
    let mut sigterm = signal(SignalKind::terminate()).expect("Failed to create SIGTERM handler");
    sigterm.recv().await;
}

#[cfg(not(unix))]
async fn wait_for_sigterm() {
    // On non-Unix systems, just wait forever
    std::future::pending::<()>().await;
}