use futures::stream::{self, StreamExt};
use std::time::Duration;
use tokio_stream::wrappers::IntervalStream;
use std::pin::Pin;
use std::task::{Context, Poll};
use futures::Stream;

/// A custom stream that generates numbers
struct CounterStream {
    current: usize,
    max: usize,
}

impl CounterStream {
    fn new(max: usize) -> Self {
        CounterStream { current: 0, max }
    }
}

impl Stream for CounterStream {
    type Item = usize;
    
    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Poll::Ready(Some(current))
        } else {
            Poll::Ready(None)
        }
    }
}

/// Demonstrates basic stream operations
async fn basic_stream_example() {
    println!("    Basic Stream Operations:");
    
    // Create a stream from an iterator
    let stream = stream::iter(vec![1, 2, 3, 4, 5]);
    
    // Collect all items
    let items: Vec<i32> = stream.collect().await;
    println!("      Collected items: {:?}", items);
    
    // Process each item
    let stream = stream::iter(vec![1, 2, 3, 4, 5]);
    stream
        .for_each(|item| async move {
            println!("      Processing item: {}", item);
        })
        .await;
}

/// Demonstrates stream transformations
async fn stream_transformations_example() {
    println!("    Stream Transformations:");
    
    let stream = stream::iter(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    
    let results: Vec<i32> = stream
        .filter(|&x| async move { x % 2 == 0 }) // Keep even numbers
        .map(|x| x * 2) // Double them
        .take(3) // Take only first 3
        .collect()
        .await;
    
    println!("      Transformed results: {:?}", results);
}

/// Demonstrates async stream processing
async fn async_stream_processing_example() {
    println!("    Async Stream Processing:");
    
    let stream = stream::iter(vec!["url1", "url2", "url3", "url4"]);
    
    // Process each item asynchronously
    let results: Vec<String> = stream
        .map(|url| async move {
            // Simulate async work (like HTTP request)
            tokio::time::sleep(Duration::from_millis(50)).await;
            format!("Data from {}", url)
        })
        .buffer_unordered(2) // Process up to 2 items concurrently
        .collect()
        .await;
    
    println!("      Async processing results:");
    for result in results {
        println!("        {}", result);
    }
}

/// Demonstrates custom stream implementation
async fn custom_stream_example() {
    println!("    Custom Stream:");
    
    let mut counter_stream = CounterStream::new(5);
    
    while let Some(value) = counter_stream.next().await {
        println!("      Counter value: {}", value);
    }
}

/// Demonstrates interval streams
async fn interval_stream_example() {
    println!("    Interval Stream:");
    
    let interval = tokio::time::interval(Duration::from_millis(100));
    let mut interval_stream = IntervalStream::new(interval);
    
    // Take only 3 ticks
    let mut count = 0;
    while let Some(_tick) = interval_stream.next().await {
        println!("      Tick {}", count);
        count += 1;
        if count >= 3 {
            break;
        }
    }
}

/// Demonstrates stream error handling
async fn stream_error_handling_example() {
    println!("    Stream Error Handling:");
    
    let stream = stream::iter(vec![
        Ok(1),
        Ok(2),
        Err("Error at 3"),
        Ok(4),
        Err("Error at 5"),
    ]);
    
    // Collect successful items, handle errors
    let mut successes = Vec::new();
    let mut errors = Vec::new();
    
    stream
        .for_each(|result| async {
            match result {
                Ok(value) => {
                    successes.push(value);
                    println!("      Success: {}", value);
                }
                Err(e) => {
                    errors.push(e);
                    println!("      Error: {}", e);
                }
            }
        })
        .await;
    
    println!("      Successes: {:?}", successes);
    println!("      Errors: {:?}", errors);
}

/// Demonstrates stream chunking and batching
async fn stream_chunking_example() {
    println!("    Stream Chunking:");
    
    let stream = stream::iter(0..15);
    
    // Process items in chunks of 4
    let chunks: Vec<Vec<i32>> = stream
        .chunks(4)
        .collect()
        .await;
    
    println!("      Chunks:");
    for (i, chunk) in chunks.iter().enumerate() {
        println!("        Chunk {}: {:?}", i, chunk);
    }
}

/// Demonstrates stream folding and reduction
async fn stream_folding_example() {
    println!("    Stream Folding:");
    
    let stream = stream::iter(vec![1, 2, 3, 4, 5]);
    
    // Sum all values
    let sum = stream.fold(0, |acc, x| async move { acc + x }).await;
    println!("      Sum: {}", sum);
    
    // Find maximum
    let stream = stream::iter(vec![3, 1, 4, 1, 5, 9, 2, 6]);
    let max = stream
        .fold(None, |acc: Option<i32>, x| async move {
            match acc {
                None => Some(x),
                Some(current_max) => Some(current_max.max(x)),
            }
        })
        .await;
    
    println!("      Maximum: {:?}", max);
}

/// Demonstrates stream merging
async fn stream_merging_example() {
    println!("    Stream Merging:");
    
    let stream1 = stream::iter(vec![1, 3, 5]);
    let stream2 = stream::iter(vec![2, 4, 6]);
    
    // Merge streams (order not guaranteed)
    let merged: Vec<i32> = stream1
        .merge(stream2)
        .collect()
        .await;
    
    println!("      Merged stream: {:?}", merged);
}

pub async fn run_examples() {
    basic_stream_example().await;
    stream_transformations_example().await;
    async_stream_processing_example().await;
    custom_stream_example().await;
    interval_stream_example().await;
    stream_error_handling_example().await;
    stream_chunking_example().await;
    stream_folding_example().await;
    stream_merging_example().await;
}