use serde_json::json;
use std::collections::HashMap;
use tracing::debug;

use crate::error::{ServerError, ServerResult};

/// HTTP request structure
/// 
/// This demonstrates parsing HTTP requests in Rust:
/// - Extracting method, path, and headers
/// - Handling different HTTP methods
/// - Basic request validation
#[derive(Debug, Clone)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

/// HTTP response structure
/// 
/// This demonstrates building HTTP responses:
/// - Setting status codes and headers
/// - Formatting response body
/// - Following HTTP protocol standards
#[derive(Debug, Clone)]
pub struct HttpResponse {
    pub status_code: u16,
    pub status_text: String,
    pub headers: HashMap<String, String>,
    pub body: String,
}

impl HttpResponse {
    /// Convert the response to a properly formatted HTTP response string
    pub fn to_string(&self) -> String {
        let mut response = format!("HTTP/1.1 {} {}\r\n", self.status_code, self.status_text);
        
        // Add headers
        for (key, value) in &self.headers {
            response.push_str(&format!("{}: {}\r\n", key, value));
        }
        
        // Add content length header
        response.push_str(&format!("Content-Length: {}\r\n", self.body.len()));
        
        // End headers and add body
        response.push_str("\r\n");
        response.push_str(&self.body);
        
        response
    }
}

/// Handles HTTP request processing
/// 
/// This demonstrates async request handling patterns:
/// - Request parsing and validation
/// - Route matching and dispatch
/// - Response generation
/// - Error handling for malformed requests
pub struct HttpHandler;

impl HttpHandler {
    pub fn new() -> Self {
        Self
    }
    
    /// Handle an HTTP request asynchronously
    /// 
    /// This method demonstrates:
    /// - Parsing raw HTTP request data
    /// - Route matching and method dispatch
    /// - Generating appropriate responses
    /// - Error handling for invalid requests
    pub async fn handle_request(&self, request_data: &str) -> ServerResult<String> {
        debug!("Processing HTTP request");
        
        // Parse the HTTP request
        let request = self.parse_request(request_data)?;
        
        // Route the request based on method and path
        let response = self.route_request(request).await?;
        
        Ok(response.to_string())
    }
    
    /// Parse raw HTTP request data into an HttpRequest struct
    /// 
    /// This demonstrates:
    /// - String parsing and manipulation
    /// - HTTP protocol understanding
    /// - Error handling for malformed requests
    fn parse_request(&self, data: &str) -> ServerResult<HttpRequest> {
        let lines: Vec<&str> = data.lines().collect();
        
        if lines.is_empty() {
            return Err(ServerError::HttpParsing("Empty request".to_string()));
        }
        
        // Parse the request line (e.g., "GET /path HTTP/1.1")
        let request_line_parts: Vec<&str> = lines[0].split_whitespace().collect();
        if request_line_parts.len() < 2 {
            return Err(ServerError::HttpParsing("Invalid request line".to_string()));
        }
        
        let method = request_line_parts[0].to_string();
        let path = request_line_parts[1].to_string();
        
        // Parse headers
        let mut headers = HashMap::new();
        let mut body_start = 1;
        
        for (i, line) in lines.iter().enumerate().skip(1) {
            if line.is_empty() {
                body_start = i + 1;
                break;
            }
            
            if let Some(colon_pos) = line.find(':') {
                let key = line[..colon_pos].trim().to_string();
                let value = line[colon_pos + 1..].trim().to_string();
                headers.insert(key, value);
            }
        }
        
        // Extract body (if any)
        let body = if body_start < lines.len() {
            lines[body_start..].join("\n")
        } else {
            String::new()
        };
        
        Ok(HttpRequest {
            method,
            path,
            headers,
            body,
        })
    }
    
    /// Route the request to the appropriate handler
    /// 
    /// This demonstrates:
    /// - Pattern matching for route dispatch
    /// - Async method calls
    /// - RESTful API design patterns
    async fn route_request(&self, request: HttpRequest) -> ServerResult<HttpResponse> {
        match (request.method.as_str(), request.path.as_str()) {
            ("GET", "/") => self.handle_root().await,
            ("GET", "/health") => self.handle_health().await,
            ("GET", "/api/status") => self.handle_api_status().await,
            ("POST", "/api/echo") => self.handle_echo(request).await,
            _ => self.handle_not_found().await,
        }
    }
    
    /// Handle root path requests
    async fn handle_root(&self) -> ServerResult<HttpResponse> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "text/html".to_string());
        
        let body = r#"
<!DOCTYPE html>
<html>
<head>
    <title>Async Network Server</title>
</head>
<body>
    <h1>Welcome to the Async Network Server!</h1>
    <p>This server demonstrates async programming in Rust.</p>
    <ul>
        <li><a href="/health">Health Check</a></li>
        <li><a href="/api/status">API Status</a></li>
    </ul>
</body>
</html>
        "#.trim();
        
        Ok(HttpResponse {
            status_code: 200,
            status_text: "OK".to_string(),
            headers,
            body: body.to_string(),
        })
    }
    
    /// Handle health check requests
    async fn handle_health(&self) -> ServerResult<HttpResponse> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        let body = json!({
            "status": "healthy",
            "timestamp": chrono::Utc::now().to_rfc3339(),
            "version": "1.0.0"
        }).to_string();
        
        Ok(HttpResponse {
            status_code: 200,
            status_text: "OK".to_string(),
            headers,
            body,
        })
    }
    
    /// Handle API status requests
    async fn handle_api_status(&self) -> ServerResult<HttpResponse> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        let body = json!({
            "api_version": "v1",
            "server": "async-network-server",
            "features": ["async", "tokio", "http"],
            "endpoints": ["/", "/health", "/api/status", "/api/echo"]
        }).to_string();
        
        Ok(HttpResponse {
            status_code: 200,
            status_text: "OK".to_string(),
            headers,
            body,
        })
    }
    
    /// Handle echo requests (POST)
    async fn handle_echo(&self, request: HttpRequest) -> ServerResult<HttpResponse> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        let body = json!({
            "method": request.method,
            "path": request.path,
            "headers": request.headers,
            "body": request.body,
            "echo": "Request received and processed asynchronously"
        }).to_string();
        
        Ok(HttpResponse {
            status_code: 200,
            status_text: "OK".to_string(),
            headers,
            body,
        })
    }
    
    /// Handle 404 Not Found
    async fn handle_not_found(&self) -> ServerResult<HttpResponse> {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        
        let body = json!({
            "error": "Not Found",
            "message": "The requested resource was not found on this server"
        }).to_string();
        
        Ok(HttpResponse {
            status_code: 404,
            status_text: "Not Found".to_string(),
            headers,
            body,
        })
    }
}

// TODO: Add middleware support
// This would include:
// 1. Request/response middleware chain
// 2. Authentication and authorization
// 3. Request logging and metrics
// 4. CORS handling
// 5. Rate limiting per endpoint

// TODO: Add request body parsing
// This would include:
// 1. JSON request body parsing
// 2. Form data handling
// 3. File upload support
// 4. Content-Type validation