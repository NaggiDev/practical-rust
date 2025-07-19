use std::net::SocketAddr;
use tokio::net::TcpListener;
use tracing::{info, error};

pub mod server;
pub mod handler;
pub mod connection;
pub mod error;

use server::Server;
use error::ServerError;

/// Entry point for the async network server
/// 
/// This demonstrates the basic structure of an async Rust application:
/// - Setting up the Tokio runtime (handled by #[tokio::main])
/// - Initializing logging and tracing
/// - Creating and starting the server
/// - Handling shutdown signals gracefully
#[tokio::main]
async fn main() -> Result<(), ServerError> {
    // Initialize tracing for observability
    tracing_subscriber::fmt::init();
    
    info!("Starting async network server...");
    
    // TODO: Parse command line arguments for configuration
    let addr: SocketAddr = "127.0.0.1:8080".parse()
        .map_err(|e| ServerError::Configuration(format!("Invalid address: {}", e)))?;
    
    // Create TCP listener - this is the foundation of our server
    let listener = TcpListener::bind(&addr).await
        .map_err(|e| ServerError::Network(format!("Failed to bind to {}: {}", addr, e)))?;
    
    info!("Server listening on {}", addr);
    
    // Create and configure the server
    let server = Server::new(listener);
    
    // TODO: Set up graceful shutdown handling
    // For now, we'll run the server indefinitely
    match server.run().await {
        Ok(_) => {
            info!("Server shutdown gracefully");
            Ok(())
        }
        Err(e) => {
            error!("Server error: {}", e);
            Err(e)
        }
    }
}

// TODO: Implement signal handling for graceful shutdown
// This would typically involve:
// 1. Setting up signal handlers for SIGINT, SIGTERM
// 2. Using tokio::select! to wait for either server completion or shutdown signal
// 3. Coordinating graceful shutdown across all server components