use tokio::net::TcpStream;
use tokio::io::{AsyncReadExt, AsyncWriteExt, BufReader, AsyncBufReadExt};
use tracing::{debug, warn};
use std::time::Duration;

use crate::handler::HttpHandler;
use crate::error::{ServerError, ServerResult};

/// Handles individual TCP connections
/// 
/// This demonstrates async I/O patterns:
/// - Reading from streams asynchronously
/// - Handling connection timeouts
/// - Proper resource cleanup
/// - Error propagation in async contexts
pub struct ConnectionHandler {
    id: u64,
    stream: TcpStream,
}

impl ConnectionHandler {
    pub fn new(id: u64, stream: TcpStream) -> Self {
        Self { id, stream }
    }
    
    /// Handle the connection lifecycle
    /// 
    /// This method demonstrates:
    /// - Async I/O operations with timeouts
    /// - HTTP request parsing
    /// - Response generation and sending
    /// - Connection keep-alive handling
    pub async fn handle(mut self) -> ServerResult<()> {
        debug!("Handling connection {}", self.id);
        
        // Set up timeout for the entire connection
        let timeout_duration = Duration::from_secs(30);
        
        // Handle the connection with a timeout
        match tokio::time::timeout(timeout_duration, self.handle_requests()).await {
            Ok(result) => result,
            Err(_) => {
                warn!("Connection {} timed out", self.id);
                Err(ServerError::timeout(&format!("Connection {} timed out", self.id)))
            }
        }
    }
    
    /// Handle HTTP requests on this connection
    /// 
    /// This demonstrates:
    /// - Reading HTTP requests from a stream
    /// - Parsing request headers and body
    /// - Generating appropriate responses
    /// - Connection persistence (HTTP/1.1 keep-alive)
    async fn handle_requests(&mut self) -> ServerResult<()> {
        let mut buffer = vec![0; 4096];
        
        loop {
            // Read data from the connection
            match self.stream.read(&mut buffer).await {
                Ok(0) => {
                    // Connection closed by client
                    debug!("Connection {} closed by client", self.id);
                    break;
                }
                Ok(bytes_read) => {
                    debug!("Connection {} received {} bytes", self.id, bytes_read);
                    
                    // Parse the HTTP request
                    let request_data = &buffer[..bytes_read];
                    let request_str = String::from_utf8_lossy(request_data);
                    
                    // Create HTTP handler and process the request
                    let handler = HttpHandler::new();
                    let response = handler.handle_request(&request_str).await?;
                    
                    // Send the response
                    self.stream.write_all(response.as_bytes()).await
                        .map_err(|e| ServerError::from_io_error(e, "Failed to write response"))?;
                    
                    // For simplicity, we'll close the connection after each request
                    // TODO: Implement HTTP/1.1 keep-alive support
                    debug!("Response sent for connection {}", self.id);
                    break;
                }
                Err(e) => {
                    warn!("Error reading from connection {}: {}", self.id, e);
                    return Err(ServerError::from_io_error(e, "Failed to read from connection"));
                }
            }
        }
        
        Ok(())
    }
}

// TODO: Implement connection pooling
// This would involve:
// 1. Reusing connections for multiple requests
// 2. Managing connection state (idle, active, closing)
// 3. Implementing connection limits per client
// 4. Adding connection health checks

// TODO: Add WebSocket upgrade support
// This would include:
// 1. Detecting WebSocket upgrade requests
// 2. Performing the WebSocket handshake
// 3. Switching to WebSocket frame handling
// 4. Managing WebSocket connection lifecycle