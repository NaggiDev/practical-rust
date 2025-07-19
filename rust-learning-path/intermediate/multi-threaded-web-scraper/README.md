# Multi-threaded Web Scraper

## Project Overview

This project teaches you how to build a concurrent web scraper in Rust that can fetch multiple web pages simultaneously using threads. You'll learn about Rust's threading model, synchronization primitives, and how to safely share data between threads.

## Learning Objectives

By completing this project, you will understand:

- How to create and manage threads in Rust
- Thread synchronization using `Arc` and `Mutex`
- Channel-based communication between threads
- Error handling in concurrent contexts
- HTTP client usage with the `reqwest` crate
- HTML parsing with the `scraper` crate

## Prerequisites

Before starting this project, you should have completed:
- Basic Level projects (understanding of ownership, error handling, structs)
- Intermediate Module 1: Advanced Ownership (borrowing, lifetimes, reference counting)

## Project Structure

```
multi-threaded-web-scraper/
├── README.md           # This file
├── src/
│   ├── main.rs         # Entry point and CLI interface
│   ├── scraper.rs      # Core scraping logic
│   ├── worker.rs       # Worker thread implementation
│   └── result.rs       # Result data structures
├── tests/
│   └── integration_tests.rs  # Integration tests
├── Cargo.toml          # Dependencies and project metadata
└── CONCEPTS.md         # Detailed concept explanations
```

## Step-by-Step Implementation Guide

### Step 1: Project Setup and Basic Structure

**Objective**: Set up the project structure and define core data types.

**Tasks**:
1. Initialize the Cargo project
2. Add required dependencies to `Cargo.toml`
3. Define the basic data structures for scraping results
4. Create a simple CLI interface

**Concepts Applied**: Project organization, external crates, struct definitions

**Implementation**:

Start by examining the `Cargo.toml` file to understand the dependencies:

```toml
[package]
name = "multi-threaded-web-scraper"
version = "0.1.0"
edition = "2021"

[dependencies]
reqwest = { version = "0.11", features = ["blocking"] }
scraper = "0.18"
clap = { version = "4.0", features = ["derive"] }
url = "2.4"
```

Then look at the basic structure in `src/result.rs` to understand how we'll store scraping results.

### Step 2: Implement Single-threaded Scraper

**Objective**: Create a basic web scraper that works on a single thread.

**Tasks**:
1. Implement HTTP request functionality
2. Add HTML parsing to extract links and titles
3. Create error handling for network requests
4. Test the single-threaded version

**Concepts Applied**: HTTP clients, HTML parsing, error propagation, Result types

**Implementation**:

Examine `src/scraper.rs` to see how we handle HTTP requests and HTML parsing. Pay attention to how errors are handled and propagated.

### Step 3: Add Multi-threading Support

**Objective**: Convert the single-threaded scraper to use multiple worker threads.

**Tasks**:
1. Create a thread pool using `std::thread`
2. Implement work distribution using channels
3. Use `Arc<Mutex<T>>` for shared state
4. Handle thread synchronization and cleanup

**Concepts Applied**: Threading, channels, `Arc`, `Mutex`, shared ownership

**Implementation**:

Study `src/worker.rs` to understand how worker threads are implemented and how they communicate with the main thread.

### Step 4: Implement Result Aggregation

**Objective**: Safely collect and aggregate results from multiple threads.

**Tasks**:
1. Design thread-safe result collection
2. Implement progress reporting
3. Add graceful shutdown handling
4. Create summary statistics

**Concepts Applied**: Thread synchronization, atomic operations, concurrent data structures

### Step 5: Add Advanced Features

**Objective**: Enhance the scraper with advanced concurrent features.

**Tasks**:
1. Implement rate limiting to avoid overwhelming servers
2. Add retry logic for failed requests
3. Create configurable thread pool size
4. Add URL deduplication across threads

**Concepts Applied**: Advanced synchronization, backpressure, concurrent collections

## Running the Project

```bash
# Build the project
cargo build

# Run with default settings
cargo run -- --urls "https://example.com" "https://httpbin.org/html"

# Run with custom thread count
cargo run -- --urls "https://example.com" --threads 4

# Run tests
cargo test
```

## Extension Challenges

1. **Async Version**: Rewrite the scraper using `tokio` and async/await
2. **Depth Crawling**: Add support for crawling links found on scraped pages
3. **Robots.txt Respect**: Implement robots.txt parsing and compliance
4. **Performance Metrics**: Add detailed timing and performance statistics
5. **Custom User Agents**: Implement rotating user agents for requests

## Key Concepts Covered

- **Threading**: Creating and managing OS threads
- **Synchronization**: Using `Arc`, `Mutex`, and channels for thread communication
- **Shared Ownership**: Understanding when and how to share data between threads
- **Error Handling**: Managing errors in concurrent contexts
- **HTTP Clients**: Making HTTP requests with external crates
- **HTML Parsing**: Extracting data from HTML documents

## Resources

- [Rust Book - Concurrency](https://doc.rust-lang.org/book/ch16-00-fearless-concurrency.html)
- [std::thread documentation](https://doc.rust-lang.org/std/thread/)
- [std::sync documentation](https://doc.rust-lang.org/std/sync/)
- [reqwest crate documentation](https://docs.rs/reqwest/)
- [scraper crate documentation](https://docs.rs/scraper/)

## Next Steps

After completing this project, you'll be ready to tackle more advanced concurrency topics in the Advanced Level, including:
- Custom thread pools
- Lock-free data structures
- Advanced synchronization primitives
- Performance optimization techniques