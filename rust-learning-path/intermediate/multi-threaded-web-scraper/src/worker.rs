use crate::result::{ScrapeError, ScrapeResult, ScraperError};
use crate::scraper::{ScraperConfig, WebScraper};
use std::sync::mpsc::{self, Receiver, Sender};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

/// Message types for communication between main thread and workers
#[derive(Debug)]
pub enum WorkerMessage {
    /// A URL to scrape
    ScrapeUrl(String),
    /// Signal to shutdown the worker
    Shutdown,
}

/// Result messages sent back from workers to main thread
#[derive(Debug)]
pub enum WorkerResult {
    /// Successful scrape result
    Success(ScrapeResult),
    /// Failed scrape attempt
    Error(ScrapeError),
    /// Worker has finished processing and is shutting down
    WorkerFinished,
}

/// Configuration for the thread pool
#[derive(Debug, Clone)]
pub struct ThreadPoolConfig {
    pub num_threads: usize,
    pub scraper_config: ScraperConfig,
}

impl Default for ThreadPoolConfig {
    fn default() -> Self {
        Self {
            num_threads: num_cpus::get().max(2),
            scraper_config: ScraperConfig::default(),
        }
    }
}

/// Multi-threaded web scraper using a thread pool
pub struct ThreadPoolScraper {
    workers: Vec<Worker>,
    sender: Option<Sender<WorkerMessage>>,
    result_receiver: Receiver<WorkerResult>,
}

impl ThreadPoolScraper {
    /// Create a new thread pool scraper
    pub fn new(config: ThreadPoolConfig) -> Result<Self, ScraperError> {
        let (work_sender, work_receiver) = mpsc::channel();
        let (result_sender, result_receiver) = mpsc::channel();
        
        // Wrap the work receiver in Arc<Mutex<>> so it can be shared among workers
        let work_receiver = Arc::new(Mutex::new(work_receiver));
        
        let mut workers = Vec::with_capacity(config.num_threads);
        
        // Create worker threads
        for id in 0..config.num_threads {
            let worker = Worker::new(
                id,
                Arc::clone(&work_receiver),
                result_sender.clone(),
                config.scraper_config.clone(),
            )?;
            workers.push(worker);
        }
        
        Ok(Self {
            workers,
            sender: Some(work_sender),
            result_receiver,
        })
    }

    /// Submit a URL for scraping
    pub fn submit_url(&self, url: String) -> Result<(), ScraperError> {
        if let Some(sender) = &self.sender {
            sender
                .send(WorkerMessage::ScrapeUrl(url))
                .map_err(|e| ScraperError::ChannelError(e.to_string()))?;
        }
        Ok(())
    }

    /// Submit multiple URLs for scraping
    pub fn submit_urls(&self, urls: Vec<String>) -> Result<(), ScraperError> {
        for url in urls {
            self.submit_url(url)?;
        }
        Ok(())
    }

    /// Receive a result from the workers (blocking)
    pub fn receive_result(&self) -> Result<WorkerResult, ScraperError> {
        self.result_receiver
            .recv()
            .map_err(|e| ScraperError::ChannelError(e.to_string()))
    }

    /// Receive a result with timeout
    pub fn receive_result_timeout(&self, timeout: Duration) -> Result<Option<WorkerResult>, ScraperError> {
        match self.result_receiver.recv_timeout(timeout) {
            Ok(result) => Ok(Some(result)),
            Err(mpsc::RecvTimeoutError::Timeout) => Ok(None),
            Err(mpsc::RecvTimeoutError::Disconnected) => {
                Err(ScraperError::ChannelError("Channel disconnected".to_string()))
            }
        }
    }

    /// Shutdown all workers and wait for them to finish
    pub fn shutdown(mut self) -> Result<(), ScraperError> {
        // Send shutdown signal to all workers
        if let Some(sender) = self.sender.take() {
            for _ in 0..self.workers.len() {
                sender
                    .send(WorkerMessage::Shutdown)
                    .map_err(|e| ScraperError::ChannelError(e.to_string()))?;
            }
        }

        // Wait for all workers to finish
        for worker in self.workers {
            if let Some(thread) = worker.thread {
                thread
                    .join()
                    .map_err(|_| ScraperError::ChannelError("Failed to join worker thread".to_string()))?;
            }
        }

        Ok(())
    }
}

/// Individual worker thread
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(
        id: usize,
        receiver: Arc<Mutex<Receiver<WorkerMessage>>>,
        sender: Sender<WorkerResult>,
        config: ScraperConfig,
    ) -> Result<Self, ScraperError> {
        let thread = thread::spawn(move || {
            // Create a scraper instance for this worker
            let scraper = match WebScraper::with_config(config) {
                Ok(scraper) => scraper,
                Err(e) => {
                    eprintln!("Worker {}: Failed to create scraper: {}", id, e);
                    return;
                }
            };

            println!("Worker {} started", id);

            loop {
                // Receive work from the shared channel
                let message = {
                    let receiver = receiver.lock().unwrap();
                    receiver.recv()
                };

                match message {
                    Ok(WorkerMessage::ScrapeUrl(url)) => {
                        println!("Worker {} processing: {}", id, url);
                        
                        // Perform the scraping
                        match scraper.scrape_url(&url) {
                            Ok(result) => {
                                if let Err(e) = sender.send(WorkerResult::Success(result)) {
                                    eprintln!("Worker {}: Failed to send result: {}", id, e);
                                    break;
                                }
                            }
                            Err(e) => {
                                let error = ScrapeError {
                                    url: url.clone(),
                                    error: e.to_string(),
                                };
                                if let Err(e) = sender.send(WorkerResult::Error(error)) {
                                    eprintln!("Worker {}: Failed to send error: {}", id, e);
                                    break;
                                }
                            }
                        }
                    }
                    Ok(WorkerMessage::Shutdown) => {
                        println!("Worker {} shutting down", id);
                        break;
                    }
                    Err(e) => {
                        eprintln!("Worker {}: Channel error: {}", id, e);
                        break;
                    }
                }
            }

            // Notify that this worker is finished
            if let Err(e) = sender.send(WorkerResult::WorkerFinished) {
                eprintln!("Worker {}: Failed to send finish signal: {}", id, e);
            }

            println!("Worker {} finished", id);
        });

        Ok(Worker {
            id,
            thread: Some(thread),
        })
    }
}

// Add num_cpus as a dependency for getting CPU count
// For now, we'll use a simple fallback
mod num_cpus {
    pub fn get() -> usize {
        // Simple fallback - in a real implementation, you'd use the num_cpus crate
        std::thread::available_parallelism()
            .map(|n| n.get())
            .unwrap_or(4)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_thread_pool_creation() {
        let config = ThreadPoolConfig {
            num_threads: 2,
            scraper_config: ScraperConfig::default(),
        };
        
        let pool = ThreadPoolScraper::new(config);
        assert!(pool.is_ok());
        
        // Clean shutdown
        if let Ok(pool) = pool {
            let _ = pool.shutdown();
        }
    }

    #[test]
    fn test_worker_message_types() {
        let url_message = WorkerMessage::ScrapeUrl("https://example.com".to_string());
        let shutdown_message = WorkerMessage::Shutdown;
        
        match url_message {
            WorkerMessage::ScrapeUrl(url) => assert_eq!(url, "https://example.com"),
            _ => panic!("Wrong message type"),
        }
        
        match shutdown_message {
            WorkerMessage::Shutdown => (),
            _ => panic!("Wrong message type"),
        }
    }

    #[test]
    fn test_thread_pool_config_default() {
        let config = ThreadPoolConfig::default();
        assert!(config.num_threads >= 2);
    }
}