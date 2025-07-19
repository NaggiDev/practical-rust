use std::collections::HashMap;
use std::net::SocketAddr;
use std::sync::atomic::{AtomicUsize, Ordering};

#[derive(Debug)]
pub struct BackendInfo {
    pub addr: SocketAddr,
    pub healthy: bool,
    pub request_count: AtomicUsize,
}

pub struct LoadBalancingStrategy {
    backends: HashMap<SocketAddr, BackendInfo>,
    current_index: AtomicUsize,
}

impl LoadBalancingStrategy {
    pub fn new(backend_addrs: Vec<SocketAddr>) -> Self {
        let mut backends = HashMap::new();
        
        for addr in backend_addrs {
            backends.insert(addr, BackendInfo {
                addr,
                healthy: true, // Assume healthy initially
                request_count: AtomicUsize::new(0),
            });
        }

        Self {
            backends,
            current_index: AtomicUsize::new(0),
        }
    }

    /// Round-robin selection of healthy backends
    pub fn next_backend(&self) -> Option<SocketAddr> {
        let healthy_backends: Vec<_> = self.backends
            .values()
            .filter(|backend| backend.healthy)
            .collect();

        if healthy_backends.is_empty() {
            return None;
        }

        let index = self.current_index.fetch_add(1, Ordering::Relaxed) % healthy_backends.len();
        let selected = healthy_backends[index];
        
        // Increment request count for the selected backend
        selected.request_count.fetch_add(1, Ordering::Relaxed);
        
        Some(selected.addr)
    }

    /// Least connections selection (alternative strategy)
    pub fn least_connections_backend(&self) -> Option<SocketAddr> {
        self.backends
            .values()
            .filter(|backend| backend.healthy)
            .min_by_key(|backend| backend.request_count.load(Ordering::Relaxed))
            .map(|backend| {
                backend.request_count.fetch_add(1, Ordering::Relaxed);
                backend.addr
            })
    }

    pub fn mark_healthy(&mut self, addr: SocketAddr) {
        if let Some(backend) = self.backends.get_mut(&addr) {
            if !backend.healthy {
                tracing::info!("Backend {} is now healthy", addr);
                backend.healthy = true;
            }
        }
    }

    pub fn mark_unhealthy(&mut self, addr: SocketAddr) {
        if let Some(backend) = self.backends.get_mut(&addr) {
            if backend.healthy {
                tracing::warn!("Backend {} is now unhealthy", addr);
                backend.healthy = false;
            }
        }
    }

    pub fn get_all_backends(&self) -> Vec<SocketAddr> {
        self.backends.keys().copied().collect()
    }

    pub fn get_healthy_backends(&self) -> Vec<SocketAddr> {
        self.backends
            .values()
            .filter(|backend| backend.healthy)
            .map(|backend| backend.addr)
            .collect()
    }

    pub fn get_backend_stats(&self) -> HashMap<SocketAddr, (bool, usize)> {
        self.backends
            .iter()
            .map(|(addr, info)| {
                (*addr, (info.healthy, info.request_count.load(Ordering::Relaxed)))
            })
            .collect()
    }
}