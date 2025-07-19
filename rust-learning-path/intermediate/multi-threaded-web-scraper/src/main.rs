mod result;
mod scraper;
mod worker;

use clap::{Arg, Command};
use result::{ScrapeResults, ScraperError};
use std::time::{Duration, Instant};
use worker::{ThreadPoolConfig, ThreadPoolScraper, WorkerResult};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("Multi-threaded Web Scraper")
        .version("1.0")
        .author("Rust Learning Path")
        .about("A concurrent web scraper built with Rust")
        .arg(
            Arg::new("urls")
                .short('u')
                .long("urls")
                .value_name("URL")
                .help("URLs to scrape")
                .required(true)
                .num_args(1..)
        )
        .arg(
            Arg::new("threads")
                .short('t')
                .long("threads")
                .value_name("NUMBER")
                .help("Number of worker threads")
                .default_value("4")
        )
        .arg(
            Arg::new("timeout")
                .long("timeout")
                .value_name("SECONDS")
                .help("Request timeout in seconds")
                .default_value("10")
        )
        .get_matches();

    // Parse command line arguments
    let urls: Vec<String> = matches
        .get_many::<String>("urls")
        .unwrap()
        .map(|s| s.to_string())
        .collect();

    let num_threads: usize = matches
        .get_one::<String>("threads")
        .unwrap()
        .parse()
        .map_err(|_| "Invalid thread count")?;

    let timeout_secs: u64 = matches
        .get_one::<String>("timeout")
        .unwrap()
        .parse()
        .map_err(|_| "Invalid timeout value")?;

    println!("Starting multi-threaded web scraper...");
    println!("URLs to scrape: {}", urls.len());
    println!("Worker threads: {}", num_threads);
    println!("Request timeout: {}s", timeout_secs);
    println!();

    // Run the scraper
    let results = run_scraper(urls, num_threads, timeout_secs)?;
    
    // Print results
    results.print_summary();

    Ok(())
}

fn run_scraper(
    urls: Vec<String>,
    num_threads: usize,
    timeout_secs: u64,
) -> Result<ScrapeResults, ScraperError> {
    let start_time = Instant::now();
    
    // Configure the thread pool
    let mut config = ThreadPoolConfig::default();
    config.num_threads = num_threads;
    config.scraper_config.timeout = Duration::from_secs(timeout_secs);

    // Create the thread pool scraper
    let scraper = ThreadPoolScraper::new(config)?;

    // Submit all URLs for processing
    let total_urls = urls.len();
    scraper.submit_urls(urls)?;

    // Collect results
    let mut results = ScrapeResults::new();
    results.total_urls = total_urls;
    
    let mut completed = 0;
    let mut workers_finished = 0;
    let expected_workers = num_threads;

    println!("Processing URLs...");

    // Process results as they come in
    while completed < total_urls || workers_finished < expected_workers {
        match scraper.receive_result_timeout(Duration::from_secs(1))? {
            Some(WorkerResult::Success(result)) => {
                completed += 1;
                println!("✓ Completed {}/{}: {} ({}ms)", 
                    completed, total_urls, result.url, result.response_time.as_millis());
                results.add_success(result);
            }
            Some(WorkerResult::Error(error)) => {
                completed += 1;
                println!("✗ Failed {}/{}: {} - {}", 
                    completed, total_urls, error.url, error.error);
                results.add_failure(error);
            }
            Some(WorkerResult::WorkerFinished) => {
                workers_finished += 1;
                println!("Worker finished ({}/{})", workers_finished, expected_workers);
            }
            None => {
                // Timeout - print progress
                if completed < total_urls {
                    println!("Progress: {}/{} URLs completed...", completed, total_urls);
                }
            }
        }
    }

    results.total_time = start_time.elapsed();

    // Shutdown the thread pool
    scraper.shutdown()?;

    Ok(results)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_run_scraper_with_invalid_urls() {
        let urls = vec!["not-a-valid-url".to_string()];
        let result = run_scraper(urls, 1, 5);
        
        // Should not panic, but may return errors for invalid URLs
        assert!(result.is_ok());
        
        let results = result.unwrap();
        assert_eq!(results.total_urls, 1);
        assert_eq!(results.successful.len(), 0);
        assert_eq!(results.failed.len(), 1);
    }

    #[test]
    fn test_run_scraper_empty_urls() {
        let urls = vec![];
        let result = run_scraper(urls, 1, 5);
        
        assert!(result.is_ok());
        let results = result.unwrap();
        assert_eq!(results.total_urls, 0);
    }
}