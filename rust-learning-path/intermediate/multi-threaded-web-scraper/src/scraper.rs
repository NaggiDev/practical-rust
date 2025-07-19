use crate::result::{ScrapeResult, ScraperError};
use reqwest::blocking::Client;
use scraper::{Html, Selector};
use std::time::{Duration, Instant};
use url::Url;

/// Configuration for the web scraper
#[derive(Debug, Clone)]
pub struct ScraperConfig {
    pub timeout: Duration,
    pub user_agent: String,
    pub max_redirects: usize,
}

impl Default for ScraperConfig {
    fn default() -> Self {
        Self {
            timeout: Duration::from_secs(10),
            user_agent: "Rust Web Scraper 1.0".to_string(),
            max_redirects: 5,
        }
    }
}

/// Single-threaded web scraper implementation
pub struct WebScraper {
    client: Client,
    config: ScraperConfig,
}

impl WebScraper {
    /// Create a new WebScraper with default configuration
    pub fn new() -> Result<Self, ScraperError> {
        Self::with_config(ScraperConfig::default())
    }

    /// Create a new WebScraper with custom configuration
    pub fn with_config(config: ScraperConfig) -> Result<Self, ScraperError> {
        let client = Client::builder()
            .timeout(config.timeout)
            .user_agent(&config.user_agent)
            .redirect(reqwest::redirect::Policy::limited(config.max_redirects))
            .build()?;

        Ok(Self { client, config })
    }

    /// Scrape a single URL and return the results
    pub fn scrape_url(&self, url: &str) -> Result<ScrapeResult, ScraperError> {
        let start_time = Instant::now();
        
        // Validate URL format
        let parsed_url = Url::parse(url)?;
        
        // Make HTTP request
        let response = self.client.get(url).send()?;
        let status_code = response.status().as_u16();
        let response_time = start_time.elapsed();
        
        // Get response body
        let body = response.text()?;
        
        // Parse HTML
        let document = Html::parse_document(&body);
        
        // Extract title
        let title = self.extract_title(&document);
        
        // Extract links
        let links = self.extract_links(&document, &parsed_url)?;
        
        Ok(ScrapeResult {
            url: url.to_string(),
            title,
            links,
            response_time,
            status_code,
        })
    }

    /// Extract the page title from HTML document
    fn extract_title(&self, document: &Html) -> Option<String> {
        let title_selector = Selector::parse("title").ok()?;
        document
            .select(&title_selector)
            .next()
            .map(|element| element.text().collect::<String>().trim().to_string())
            .filter(|title| !title.is_empty())
    }

    /// Extract all links from HTML document and convert to absolute URLs
    fn extract_links(&self, document: &Html, base_url: &Url) -> Result<Vec<String>, ScraperError> {
        let link_selector = Selector::parse("a[href]")
            .map_err(|e| ScraperError::ParseError(format!("Invalid selector: {}", e)))?;
        
        let mut links = Vec::new();
        
        for element in document.select(&link_selector) {
            if let Some(href) = element.value().attr("href") {
                // Convert relative URLs to absolute URLs
                match base_url.join(href) {
                    Ok(absolute_url) => {
                        let url_str = absolute_url.to_string();
                        // Only include HTTP/HTTPS URLs
                        if url_str.starts_with("http://") || url_str.starts_with("https://") {
                            links.push(url_str);
                        }
                    }
                    Err(_) => {
                        // Skip invalid URLs
                        continue;
                    }
                }
            }
        }
        
        // Remove duplicates while preserving order
        links.sort();
        links.dedup();
        
        Ok(links)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scraper_creation() {
        let scraper = WebScraper::new();
        assert!(scraper.is_ok());
    }

    #[test]
    fn test_custom_config() {
        let config = ScraperConfig {
            timeout: Duration::from_secs(5),
            user_agent: "Test Agent".to_string(),
            max_redirects: 3,
        };
        
        let scraper = WebScraper::with_config(config);
        assert!(scraper.is_ok());
    }

    #[test]
    fn test_invalid_url() {
        let scraper = WebScraper::new().unwrap();
        let result = scraper.scrape_url("not-a-valid-url");
        assert!(result.is_err());
    }

    #[test]
    fn test_title_extraction() {
        let scraper = WebScraper::new().unwrap();
        let html = r#"
            <!DOCTYPE html>
            <html>
            <head><title>Test Page</title></head>
            <body><p>Content</p></body>
            </html>
        "#;
        
        let document = Html::parse_document(html);
        let title = scraper.extract_title(&document);
        assert_eq!(title, Some("Test Page".to_string()));
    }

    #[test]
    fn test_link_extraction() {
        let scraper = WebScraper::new().unwrap();
        let html = r#"
            <!DOCTYPE html>
            <html>
            <body>
                <a href="https://example.com">Absolute Link</a>
                <a href="/relative">Relative Link</a>
                <a href="mailto:test@example.com">Email Link</a>
            </body>
            </html>
        "#;
        
        let document = Html::parse_document(html);
        let base_url = Url::parse("https://test.com").unwrap();
        let links = scraper.extract_links(&document, &base_url).unwrap();
        
        assert!(links.contains(&"https://example.com".to_string()));
        assert!(links.contains(&"https://test.com/relative".to_string()));
        // Email links should be filtered out
        assert!(!links.iter().any(|link| link.contains("mailto:")));
    }
}