use std::time::Duration;
use thiserror::Error;

/// Represents the result of scraping a single URL
#[derive(Debug, Clone)]
pub struct ScrapeResult {
    pub url: String,
    pub title: Option<String>,
    pub links: Vec<String>,
    pub response_time: Duration,
    pub status_code: u16,
}

/// Aggregated results from multiple scraping operations
#[derive(Debug, Default)]
pub struct ScrapeResults {
    pub successful: Vec<ScrapeResult>,
    pub failed: Vec<ScrapeError>,
    pub total_urls: usize,
    pub total_time: Duration,
}

impl ScrapeResults {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add_success(&mut self, result: ScrapeResult) {
        self.successful.push(result);
    }

    pub fn add_failure(&mut self, error: ScrapeError) {
        self.failed.push(error);
    }

    pub fn success_rate(&self) -> f64 {
        if self.total_urls == 0 {
            return 0.0;
        }
        self.successful.len() as f64 / self.total_urls as f64
    }

    pub fn print_summary(&self) {
        println!("\n=== Scraping Summary ===");
        println!("Total URLs processed: {}", self.total_urls);
        println!("Successful: {}", self.successful.len());
        println!("Failed: {}", self.failed.len());
        println!("Success rate: {:.1}%", self.success_rate() * 100.0);
        println!("Total time: {:?}", self.total_time);

        if !self.successful.is_empty() {
            println!("\n=== Successful Results ===");
            for result in &self.successful {
                println!("URL: {}", result.url);
                println!("  Title: {}", result.title.as_deref().unwrap_or("No title"));
                println!("  Links found: {}", result.links.len());
                println!("  Response time: {:?}", result.response_time);
                println!("  Status: {}", result.status_code);
                println!();
            }
        }

        if !self.failed.is_empty() {
            println!("=== Failed URLs ===");
            for error in &self.failed {
                println!("URL: {}", error.url);
                println!("  Error: {}", error.error);
                println!();
            }
        }
    }
}

/// Represents an error that occurred while scraping a URL
#[derive(Debug, Clone)]
pub struct ScrapeError {
    pub url: String,
    pub error: String,
}

/// Custom error types for the scraper
#[derive(Error, Debug)]
pub enum ScraperError {
    #[error("HTTP request failed: {0}")]
    HttpError(#[from] reqwest::Error),
    
    #[error("Invalid URL: {0}")]
    InvalidUrl(#[from] url::ParseError),
    
    #[error("HTML parsing failed: {0}")]
    ParseError(String),
    
    #[error("Thread communication error: {0}")]
    ChannelError(String),
}

impl From<ScraperError> for ScrapeError {
    fn from(error: ScraperError) -> Self {
        ScrapeError {
            url: String::new(), // URL will be set by the caller
            error: error.to_string(),
        }
    }
}