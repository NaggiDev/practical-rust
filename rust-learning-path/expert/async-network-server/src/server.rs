use tokio::net::{TcpListener, TcpStream};
use tracing::{info, warn, error, debug};
use std::sync::Arc;
use std::sync::atomic::{AtomicU64, Ordering};

use crate::connection::ConnectionHandler;
use crate::error::{ServerError, ServerResult};

/// The main server struct that manages the TCP listener and connection handling
/// 
/// This demonstrates several async patterns:
/// - Using Arc for shared state across async tasks
/// - Atomic counters for thread-safe metrics
/// - Spawning tasks for concurrent connection handling
/// - Proper resource management and cleanup
pub struct Server {
    listener: TcpListener,
    connection_count: Arc<AtomicU64>,
    active_connections: Arc<AtomicU64>,
}

impl Server {
    /// Create a new server instance
    pub fn new(listener: TcpListener) -> Self {
        Self {
            listener,
            connection_count: Arc::new(AtomicU64::new(0)),
            active_connections: Arc::new(AtomicU64::new(0)),
        }
    }
    
    /// Run the server, accepting connections in a loop
    /// 
    /// This is the main server loop that demonstrates:
    /// - Async iteration with while loops
    /// - Error handling that doesn't crash the server
    /// - Spawning concurrent tasks for each connection
    /// - Sharing state between the main loop and connection handlers
    pub async fn run(self) -> ServerResult<()> {
        info!("Server started, waiting for connections...");
        
        loop {
            // Accept a new connection
            // The ? operator propagates errors up to the caller
            match self.listener.accept().await {
                Ok((stream, addr)) => {
                    let conn_id = self.connection_count.fetch_add(1, Ordering::Relaxed);
                    self.active_connections.fetch_add(1, Ordering::Relaxed);
                    
                    info!("New connection {} from {}", conn_id, addr);
                    
                    // Clone Arc references for the spawned task
                    let active_connections = Arc::clone(&self.active_connections);
                    
                    // Spawn a new task to handle this connection
                    // This allows the server to handle multiple connections concurrently
                    tokio::spawn(async move {
                        let handler = ConnectionHandler::new(conn_id, stream);
                        
                        // Handle the connection and log any errors
                        if let Err(e) = handler.handle().await {
                            warn!("Connection {} error: {}", conn_id, e);
                        }
                        
                        // Decrement active connection count when done
                        let remaining = active_connections.fetch_sub(1, Ordering::Relaxed) - 1;
                        debug!("Connection {} closed, {} active connections remaining", conn_id, remaining);
                    });
                }
                Err(e) => {
                    error!("Failed to accept connection: {}", e);
                    // Continue accepting other connections even if one fails
                    continue;
                }
            }
        }
    }
    
    /// Get current server statistics
    pub fn stats(&self) -> ServerStats {
        ServerStats {
            total_connections: self.connection_count.load(Ordering::Relaxed),
            active_connections: self.active_connections.load(Ordering::Relaxed),
        }
    }
}

/// Server statistics structure
#[derive(Debug, Clone)]
pub struct ServerStats {
    pub total_connections: u64,
    pub active_connections: u64,
}

// TODO: Implement graceful shutdown
// This would involve:
// 1. Adding a shutdown channel or atomic flag
// 2. Modifying the main loop to check for shutdown signals
// 3. Waiting for active connections to complete
// 4. Cleaning up resources properly

// TODO: Add connection limits and rate limiting
// This could include:
// 1. Maximum concurrent connections
// 2. Rate limiting per IP address
// 3. Connection timeout handling
// 4. Resource usage monitoring