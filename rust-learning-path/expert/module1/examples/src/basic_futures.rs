use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};
use std::time::{Duration, Instant};

/// A simple future that completes after a specified duration
pub struct DelayFuture {
    when: Instant,
}

impl DelayFuture {
    pub fn new(duration: Duration) -> Self {
        DelayFuture {
            when: Instant::now() + duration,
        }
    }
}

impl Future for DelayFuture {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if Instant::now() >= self.when {
            Poll::Ready(())
        } else {
            // In a real implementation, you'd use a proper timer
            // This is just for demonstration
            let waker = cx.waker().clone();
            let when = self.when;
            std::thread::spawn(move || {
                let now = Instant::now();
                if when > now {
                    std::thread::sleep(when - now);
                }
                waker.wake();
            });
            Poll::Pending
        }
    }
}

/// A future that yields a value after computation
pub struct ComputeFuture {
    value: Option<i32>,
}

impl ComputeFuture {
    pub fn new(value: i32) -> Self {
        ComputeFuture { value: Some(value) }
    }
}

impl Future for ComputeFuture {
    type Output = i32;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.value.take() {
            Some(value) => {
                // Simulate some computation
                let result = value * value;
                Poll::Ready(result)
            }
            None => Poll::Ready(0), // Already computed
        }
    }
}

pub async fn run_examples() {
    println!("  Basic Future Implementation:");
    
    // Using our custom DelayFuture
    let start = Instant::now();
    DelayFuture::new(Duration::from_millis(100)).await;
    println!("    DelayFuture completed in {:?}", start.elapsed());
    
    // Using our ComputeFuture
    let result = ComputeFuture::new(5).await;
    println!("    ComputeFuture result: {}", result);
    
    // Combining futures
    println!("  Combining Futures:");
    let future1 = async { 42 };
    let future2 = async { "hello" };
    
    let (result1, result2) = tokio::join!(future1, future2);
    println!("    Combined results: {} and {}", result1, result2);
    
    // Using select to get the first completed future
    let result = tokio::select! {
        val = async { 
            tokio::time::sleep(Duration::from_millis(50)).await;
            1 
        } => val,
        val = async { 
            tokio::time::sleep(Duration::from_millis(100)).await;
            2 
        } => val,
    };
    println!("    First completed future result: {}", result);
}