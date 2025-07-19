use hyper::service::{make_service_fn, service_fn};
use hyper::{Body, Request, Response, Server};
use std::convert::Infallible;
use std::net::SocketAddr;
use tracing::{info, error};
use anyhow::Result;

use super::handler::handle_request;

pub struct HttpServer {
    config: crate::config::HttpConfig,
}

impl HttpServer {
    pub fn new(config: crate::config::HttpConfig) -> Self {
        Self { config }
    }

    pub async fn run(&self, mut shutdown: tokio::sync::broadcast::Receiver<()>) -> Result<()> {
        let addr = self.config.bind_addr;
        
        let make_svc = make_service_fn(|_conn| async {
            Ok::<_, Infallible>(service_fn(handle_request))
        });

        let server = Server::bind(&addr).serve(make_svc);
        info!("HTTP server listening on {}", addr);

        let graceful = server.with_graceful_shutdown(async {
            shutdown.recv().await.ok();
            info!("HTTP server shutting down");
        });

        if let Err(e) = graceful.await {
            error!("HTTP server error: {}", e);
        }

        Ok(())
    }
}