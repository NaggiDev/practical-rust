use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Client, Request, Response, Server, StatusCode};
use std::convert::Infallible;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::{info, error, warn};
use anyhow::Result;

use super::strategy::LoadBalancingStrategy;

pub struct BalancerServer {
    config: crate::config::BalancerConfig,
    strategy: Arc<RwLock<LoadBalancingStrategy>>,
    client: Client<hyper::client::HttpConnector>,
}

impl BalancerServer {
    pub fn new(config: crate::config::BalancerConfig) -> Self {
        let strategy = LoadBalancingStrategy::new(config.backends.clone());
        
        Self {
            config,
            strategy: Arc::new(RwLock::new(strategy)),
            client: Client::new(),
        }
    }

    pub async fn run(&self, mut shutdown: tokio::sync::broadcast::Receiver<()>) -> Result<()> {
        let addr = self.config.bind_addr;
        let strategy = Arc::clone(&self.strategy);
        let client = self.client.clone();
        
        // Start health check task
        let health_check_strategy = Arc::clone(&self.strategy);
        let health_check_client = self.client.clone();
        let health_check_interval = self.config.health_check_interval;
        
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(
                tokio::time::Duration::from_secs(health_check_interval)
            );
            
            loop {
                interval.tick().await;
                
                let backends = {
                    let strategy_guard = health_check_strategy.read().await;
                    strategy_guard.get_all_backends()
                };
                
                for backend in backends {
                    let client = health_check_client.clone();
                    let strategy = Arc::clone(&health_check_strategy);
                    
                    tokio::spawn(async move {
                        let health_url = format!("http://{}/health", backend);
                        
                        match client.get(health_url.parse().unwrap()).await {
                            Ok(response) if response.status().is_success() => {
                                let mut strategy_guard = strategy.write().await;
                                strategy_guard.mark_healthy(backend);
                            }
                            _ => {
                                let mut strategy_guard = strategy.write().await;
                                strategy_guard.mark_unhealthy(backend);
                            }
                        }
                    });
                }
            }
        });

        let make_svc = make_service_fn(move |_conn| {
            let strategy = Arc::clone(&strategy);
            let client = client.clone();
            
            async move {
                Ok::<_, Infallible>(service_fn(move |req| {
                    handle_proxy_request(req, Arc::clone(&strategy), client.clone())
                }))
            }
        });

        let server = Server::bind(&addr).serve(make_svc);
        info!("Load balancer listening on {}", addr);

        let graceful = server.with_graceful_shutdown(async {
            shutdown.recv().await.ok();
            info!("Load balancer shutting down");
        });

        if let Err(e) = graceful.await {
            error!("Load balancer error: {}", e);
        }

        Ok(())
    }
}

async fn handle_proxy_request(
    mut req: Request<Body>,
    strategy: Arc<RwLock<LoadBalancingStrategy>>,
    client: Client<hyper::client::HttpConnector>,
) -> Result<Response<Body>, Infallible> {
    let backend = {
        let strategy_guard = strategy.read().await;
        strategy_guard.next_backend()
    };

    let backend = match backend {
        Some(addr) => addr,
        None => {
            warn!("No healthy backends available");
            return Ok(Response::builder()
                .status(StatusCode::SERVICE_UNAVAILABLE)
                .body(Body::from("No healthy backends available"))
                .unwrap());
        }
    };

    // Modify the request URI to point to the backend
    let uri_string = format!("http://{}{}", backend, req.uri().path_and_query().map(|x| x.as_str()).unwrap_or("/"));
    
    match uri_string.parse() {
        Ok(uri) => {
            *req.uri_mut() = uri;
            
            match client.request(req).await {
                Ok(response) => Ok(response),
                Err(e) => {
                    error!("Backend request failed: {}", e);
                    
                    // Mark backend as unhealthy
                    {
                        let mut strategy_guard = strategy.write().await;
                        strategy_guard.mark_unhealthy(backend);
                    }
                    
                    Ok(Response::builder()
                        .status(StatusCode::BAD_GATEWAY)
                        .body(Body::from("Backend request failed"))
                        .unwrap())
                }
            }
        }
        Err(e) => {
            error!("Invalid URI: {}", e);
            Ok(Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from("Invalid backend URI"))
                .unwrap())
        }
    }
}