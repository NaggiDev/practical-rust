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