use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::{info, warn, error};
use anyhow::Result;

pub struct EchoServer {
    config: crate::config::EchoConfig,
    active_connections: Arc<AtomicUsize>,
}

impl EchoServer {
    pub fn new(config: crate::config::EchoConfig) -> Self {
        Self {
            config,
            active_connections: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub async fn run(&self, mut shutdown: tokio::sync::broadcast::Receiver<()>) -> Result<()> {
        let listener = TcpListener::bind(self.config.bind_addr).await?;
        info!("Echo server listening on {}", self.config.bind_addr);

        loop {
            tokio::select! {
                result = listener.accept() => {
                    match result {
                        Ok((stream, addr)) => {
                            let current_connections = self.active_connections.load(Ordering::Relaxed);
                            
                            if current_connections >= self.config.max_connections {
                                warn!("Connection limit reached, rejecting connection from {}", addr);
                                drop(stream);
                                continue;
                            }

                            let connections = Arc::clone(&self.active_connections);
                            tokio::spawn(async move {
                                connections.fetch_add(1, Ordering::Relaxed);
                                if let Err(e) = handle_echo_connection(stream, addr).await {
                                    error!("Error handling connection from {}: {}", addr, e);
                                }
                                connections.fetch_sub(1, Ordering::Relaxed);
                            });
                        }
                        Err(e) => {
                            error!("Failed to accept connection: {}", e);
                        }
                    }
                }
                _ = shutdown.recv() => {
                    info!("Echo server shutting down");
                    break;
                }
            }
        }

        // Wait for active connections to finish
        while self.active_connections.load(Ordering::Relaxed) > 0 {
            tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
        }

        Ok(())
    }

    pub fn active_connections(&self) -> usize {
        self.active_connections.load(Ordering::Relaxed)
    }
}

async fn handle_echo_connection(mut stream: TcpStream, addr: std::net::SocketAddr) -> Result<()> {
    info!("New echo connection from {}", addr);
    
    let mut buffer = vec![0; 1024];
    
    loop {
        match stream.read(&mut buffer).await {
            Ok(0) => {
                info!("Connection closed by {}", addr);
                break;
            }
            Ok(n) => {
                // Echo the data back
                if let Err(e) = stream.write_all(&buffer[..n]).await {
                    error!("Failed to write to {}: {}", addr, e);
                    break;
                }
            }
            Err(e) => {
                error!("Failed to read from {}: {}", addr, e);
                break;
            }
        }
    }

    Ok(())
}