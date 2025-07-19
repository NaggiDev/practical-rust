use criterion::{black_box, criterion_group, criterion_main, Criterion};
use async_network_server::handler::HttpHandler;
use tokio::runtime::Runtime;

/// Benchmark for HTTP request handling performance
/// 
/// This demonstrates:
/// - Performance testing of async code
/// - Using criterion for benchmarking
/// - Measuring request processing throughput
/// - Identifying performance bottlenecks

fn benchmark_request_parsing(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let handler = HttpHandler::new();
    
    c.bench_function("parse_get_request", |b| {
        b.to_async(&rt).iter(|| async {
            let request_data = "GET / HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: benchmark\r\n\r\n";
            black_box(handler.handle_request(request_data).await)
        })
    });
}

fn benchmark_json_response(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let handler = HttpHandler::new();
    
    c.bench_function("json_api_response", |b| {
        b.to_async(&rt).iter(|| async {
            let request_data = "GET /api/status HTTP/1.1\r\nHost: localhost:8080\r\n\r\n";
            black_box(handler.handle_request(request_data).await)
        })
    });
}

fn benchmark_echo_request(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let handler = HttpHandler::new();
    
    c.bench_function("echo_post_request", |b| {
        b.to_async(&rt).iter(|| async {
            let request_data = "POST /api/echo HTTP/1.1\r\nHost: localhost:8080\r\nContent-Length: 26\r\n\r\n{\"message\": \"benchmark\"}";
            black_box(handler.handle_request(request_data).await)
        })
    });
}

criterion_group!(
    benches,
    benchmark_request_parsing,
    benchmark_json_response,
    benchmark_echo_request
);
criterion_main!(benches);