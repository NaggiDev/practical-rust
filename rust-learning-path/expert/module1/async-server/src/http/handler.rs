use hyper::{Body, Method, Request, Response, StatusCode};
use serde_json::json;
use std::convert::Infallible;
use tracing::{info, warn};

pub async fn handle_request(req: Request<Body>) -> Result<Response<Body>, Infallible> {
    let method = req.method();
    let path = req.uri().path();
    
    info!("HTTP {} {}", method, path);

    let response = match (method, path) {
        (&Method::GET, "/") => {
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(json!({
                    "message": "Welcome to Async Server",
                    "version": "0.1.0",
                    "endpoints": [
                        "GET /",
                        "GET /health",
                        "POST /echo",
                        "GET /stats"
                    ]
                }).to_string()))
                .unwrap()
        }
        
        (&Method::GET, "/health") => {
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(json!({
                    "status": "healthy",
                    "timestamp": chrono::Utc::now().to_rfc3339()
                }).to_string()))
                .unwrap()
        }
        
        (&Method::POST, "/echo") => {
            let body_bytes = hyper::body::to_bytes(req.into_body()).await.unwrap_or_default();
            let body_str = String::from_utf8_lossy(&body_bytes);
            
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(json!({
                    "echo": body_str,
                    "length": body_bytes.len()
                }).to_string()))
                .unwrap()
        }
        
        (&Method::GET, "/stats") => {
            Response::builder()
                .status(StatusCode::OK)
                .header("content-type", "application/json")
                .body(Body::from(json!({
                    "server": "async-server",
                    "uptime": "TODO: implement uptime tracking",
                    "requests_handled": "TODO: implement request counting",
                    "active_connections": "TODO: implement connection tracking"
                }).to_string()))
                .unwrap()
        }
        
        _ => {
            warn!("Not found: {} {}", method, path);
            Response::builder()
                .status(StatusCode::NOT_FOUND)
                .header("content-type", "application/json")
                .body(Body::from(json!({
                    "error": "Not Found",
                    "message": format!("The requested resource {} was not found", path)
                }).to_string()))
                .unwrap()
        }
    };

    Ok(response)
}