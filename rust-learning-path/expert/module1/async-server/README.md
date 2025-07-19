# Asynchronous Network Server Project

Build a high-performance asynchronous network server that can handle thousands of concurrent connections efficiently using Rust's async/await system and the Tokio runtime.

## Learning Objectives

By completing this project, you will:

- Implement async/await for non-blocking network operations
- Work with Tokio for async I/O and networking
- Manage many concurrent connections efficiently
- Handle backpressure and implement load balancing
- Implement graceful shutdown mechanisms
- Test asynchronous network code
- Measure and optimize async performance
- Understand async runtime behavior under load

## Project Overview

You'll build a multi-protocol network server that supports:

1. **HTTP Server** - Handle HTTP requests asynchronously
2. **Echo Server** - TCP echo server for testing concurrent connections
3. **Chat Server** - WebSocket-based chat server with message broadcasting
4. **Load Balancer** - Distribute requests across multiple backend servers
5. **Monitoring** - Real-time metrics and health checks

## Prerequisites

- Completion of Module 1 concepts and examples
- Understanding of TCP/IP networking basics
- Familiarity with HTTP protocol
- Basic knowledge of WebSockets

## Project Structure

```
async-server/
├── Cargo.toml
├── README.md
├── src/
│   ├── main.rs              # Main server entry point
│   ├── lib.rs               # Library exports
│   ├── config.rs            # Server configuration
│   ├── http/                # HTTP server implementation
│   │   ├── mod.rs
│   │   ├── server.rs
│   │   ├── handler.rs
│   │   └── middleware.rs
│   ├── echo/                # Echo server implementation
│   │   ├── mod.rs
│   │   └── server.rs
│   ├── chat/                # Chat server implementation
│   │   ├── mod.rs
│   │   ├── server.rs
│   │   └── room.rs
│   ├── balancer/            # Load balancer implementation
│   │   ├── mod.rs
│   │   ├── server.rs
│   │   └── strategy.rs
│   ├── monitoring/          # Monitoring and metrics
│   │   ├── mod.rs
│   │   ├── metrics.rs
│   │   └── health.rs
│   └── utils/               # Utility functions
│       ├── mod.rs
│       ├── shutdown.rs
│       └── connection.rs
├── tests/                   # Integration tests
│   ├── http_tests.rs
│   ├── echo_tests.rs
│   ├── chat_tests.rs
│   └── load_tests.rs
├── benches/                 # Performance benchmarks
│   └── server_bench.rs
└── examples/                # Usage examples
    ├── simple_client.rs
    ├── load_test.rs
    └── chat_client.rs
```

## Implementation Steps

### Step 1: Project Setup and Basic Structure

**Objective**: Set up the project structure and basic async server framework.

**Tasks**:
1. Create the Cargo.toml with necessary dependencies
2. Implement basic configuration management
3. Set up logging and error handling
4. Create the main server entry point with graceful shutdown

**Key Concepts Applied**:
- Tokio runtime setup and configuration
- Async main function
- Graceful shutdown patterns
- Error handling in async contexts

**Implementation Guide**:

Start by creating the `Cargo.toml`:

```toml
[package]
name = "async-server"
version = "0.1.0"
edition = "2021"

[dependencies]
tokio = { version = "1.0", features = ["full"] }
tokio-util = "0.7"
futures = "0.3"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
tracing = "0.1"
tracing-subscriber = "0.3"
anyhow = "1.0"
clap = { version = "4.0", features = ["derive"] }
uuid = { version = "1.0", features = ["v4"] }

# HTTP specific
hyper = { version = "0.14", features = ["full"] }
tower = "0.4"
tower-http = { version = "0.4", features = ["full"] }

# WebSocket specific
tokio-tungstenite = "0.20"
tungstenite = "0.20"

# Metrics and monitoring
metrics = "0.21"
metrics-exporter-prometheus = "0.12"

[dev-dependencies]
tokio-test = "0.4"
criterion = { version = "0.5", features = ["html_reports"] }

[[bench]]
name = "server_bench"
harness = false
```

Create the basic configuration structure:

```rust
// src/config.rs
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub http: HttpConfig,
    pub echo: EchoConfig,
    pub chat: ChatConfig,
    pub balancer: BalancerConfig,
    pub monitoring: MonitoringConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HttpConfig {
    pub bind_addr: SocketAddr,
    pub max_connections: usize,
    pub request_timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EchoConfig {
    pub bind_addr: SocketAddr,
    pub max_connections: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatConfig {
    pub bind_addr: SocketAddr,
    pub max_rooms: usize,
    pub max_clients_per_room: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BalancerConfig {
    pub bind_addr: SocketAddr,
    pub backends: Vec<SocketAddr>,
    pub health_check_interval: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MonitoringConfig {
    pub bind_addr: SocketAddr,
    pub metrics_path: String,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            http: HttpConfig {
                bind_addr: "127.0.0.1:8080".parse().unwrap(),
                max_connections: 1000,
                request_timeout: 30,
            },
            echo: EchoConfig {
                bind_addr: "127.0.0.1:8081".parse().unwrap(),
                max_connections: 1000,
            },
            chat: ChatConfig {
                bind_addr: "127.0.0.1:8082".parse().unwrap(),
                max_rooms: 100,
                max_clients_per_room: 50,
            },
            balancer: BalancerConfig {
                bind_addr: "127.0.0.1:8083".parse().unwrap(),
                backends: vec![
                    "127.0.0.1:8080".parse().unwrap(),
                ],
                health_check_interval: 30,
            },
            monitoring: MonitoringConfig {
                bind_addr: "127.0.0.1:9090".parse().unwrap(),
                metrics_path: "/metrics".to_string(),
            },
        }
    }
}
```

**Testing Requirements**:
- Configuration loading and validation
- Basic server startup and shutdown
- Graceful shutdown signal handling

### Step 2: Echo Server Implementation

**Objective**: Implement a high-performance TCP echo server to understand basic async networking.

**Tasks**:
1. Create TCP listener with async accept loop
2. Handle multiple concurrent connections
3. Implement connection management and cleanup
4. Add connection limiting and backpressure handling

**Key Concepts Applied**:
- `TcpListener` and `TcpStream` async operations
- Connection pooling and management
- Backpressure and flow control
- Resource cleanup and connection limits

**Implementation Guide**:

```rust
// src/echo/server.rs
use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
use tracing::{info, warn, error};

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

    pub async fn run(&self, shutdown: tokio::sync::broadcast::Receiver<()>) -> anyhow::Result<()> {
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

        Ok(())
    }
}

async fn handle_echo_connection(mut stream: TcpStream, addr: std::net::SocketAddr) -> anyhow::Result<()> {
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
```

**Testing Requirements**:
- Single connection echo functionality
- Multiple concurrent connections
- Connection limit enforcement
- Graceful connection cleanup

### Step 3: HTTP Server Implementation

**Objective**: Build an async HTTP server using Hyper and Tower middleware.

**Tasks**:
1. Set up Hyper server with async handlers
2. Implement routing and middleware
3. Add request/response processing
4. Handle different content types and methods

**Key Concepts Applied**:
- HTTP protocol handling with async I/O
- Middleware patterns in async contexts
- Request routing and handler composition
- Error handling for HTTP requests

**Testing Requirements**:
- Basic HTTP GET/POST requests
- Concurrent request handling
- Middleware functionality
- Error response handling

### Step 4: WebSocket Chat Server

**Objective**: Implement a real-time chat server using WebSockets.

**Tasks**:
1. WebSocket connection handling
2. Chat room management
3. Message broadcasting
4. Client connection lifecycle

**Key Concepts Applied**:
- WebSocket protocol with async streams
- Broadcast channels for message distribution
- State management across connections
- Real-time communication patterns

**Testing Requirements**:
- WebSocket connection establishment
- Message broadcasting to multiple clients
- Room management functionality
- Connection cleanup on disconnect

### Step 5: Load Balancer

**Objective**: Create a load balancer that distributes requests across backend servers.

**Tasks**:
1. Backend health checking
2. Request forwarding
3. Load balancing strategies
4. Failover handling

**Key Concepts Applied**:
- Proxy patterns with async I/O
- Health monitoring with periodic tasks
- Request routing and forwarding
- Circuit breaker patterns

**Testing Requirements**:
- Request distribution across backends
- Health check functionality
- Failover behavior
- Performance under load

### Step 6: Monitoring and Metrics

**Objective**: Add comprehensive monitoring and metrics collection.

**Tasks**:
1. Metrics collection and export
2. Health check endpoints
3. Performance monitoring
4. Real-time dashboards

**Key Concepts Applied**:
- Metrics collection in async contexts
- Prometheus integration
- Performance measurement
- Observability patterns

**Testing Requirements**:
- Metrics accuracy
- Health check responses
- Performance measurement
- Dashboard functionality

### Step 7: Integration and Performance Testing

**Objective**: Comprehensive testing and performance optimization.

**Tasks**:
1. Load testing with multiple concurrent clients
2. Performance benchmarking
3. Memory usage optimization
4. Latency measurement and optimization

**Key Concepts Applied**:
- Load testing methodologies
- Performance profiling
- Memory management in async code
- Optimization techniques

**Testing Requirements**:
- High concurrency load tests
- Performance benchmarks
- Memory leak detection
- Latency measurements

## Performance Goals

Your server should achieve:

- **Throughput**: Handle 10,000+ concurrent connections
- **Latency**: Sub-millisecond response times for simple requests
- **Memory**: Efficient memory usage with connection pooling
- **CPU**: Optimal CPU utilization across available cores

## Extension Challenges

After completing the basic implementation:

1. **TLS Support**: Add HTTPS and secure WebSocket support
2. **Rate Limiting**: Implement per-client rate limiting
3. **Caching**: Add response caching with TTL
4. **Clustering**: Support multiple server instances
5. **Protocol Support**: Add HTTP/2 and gRPC support
6. **Database Integration**: Add async database operations
7. **Message Queues**: Integrate with Redis or RabbitMQ
8. **Containerization**: Docker and Kubernetes deployment

## Key Learning Outcomes

By completing this project, you will understand:

- How to build high-performance async network servers
- Proper resource management in concurrent environments
- Load balancing and failover strategies
- Real-time communication patterns
- Performance optimization techniques
- Testing strategies for async code
- Production deployment considerations

## Resources

- [Tokio Tutorial](https://tokio.rs/tokio/tutorial)
- [Hyper Documentation](https://hyper.rs/)
- [Tower Middleware](https://github.com/tower-rs/tower)
- [WebSocket RFC](https://tools.ietf.org/html/rfc6455)
- [Load Balancing Strategies](https://en.wikipedia.org/wiki/Load_balancing_(computing))

This project provides hands-on experience with all the major concepts of async programming in Rust, preparing you for building production-grade network services.