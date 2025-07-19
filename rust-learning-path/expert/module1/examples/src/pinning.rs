use std::pin::Pin;
use std::marker::PhantomPinned;
use std::future::Future;
use std::task::{Context, Poll};

/// A self-referential struct that demonstrates pinning
struct SelfReferential {
    data: String,
    pointer: *const String,
    _pin: PhantomPinned,
}

impl SelfReferential {
    fn new(data: String) -> Pin<Box<Self>> {
        let mut boxed = Box::pin(SelfReferential {
            data,
            pointer: std::ptr::null(),
            _pin: PhantomPinned,
        });
        
        // Safe because we're pinning the data
        let ptr = &boxed.data as *const String;
        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).pointer = ptr;
        }
        
        boxed
    }
    
    fn get_data(&self) -> &str {
        &self.data
    }
    
    fn get_pointer_data(&self) -> &str {
        unsafe { &*self.pointer }
    }
}

/// A future that demonstrates pinning in async context
struct PinnedFuture {
    state: String,
    self_ref: *const String,
    _pin: PhantomPinned,
}

impl PinnedFuture {
    fn new(state: String) -> Pin<Box<Self>> {
        let mut boxed = Box::pin(PinnedFuture {
            state,
            self_ref: std::ptr::null(),
            _pin: PhantomPinned,
        });
        
        let ptr = &boxed.state as *const String;
        unsafe {
            let mut_ref = Pin::as_mut(&mut boxed);
            Pin::get_unchecked_mut(mut_ref).self_ref = ptr;
        }
        
        boxed
    }
}

impl Future for PinnedFuture {
    type Output = String;
    
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        // Access the self-referenced data
        let data = unsafe { &*self.self_ref };
        Poll::Ready(format!("Completed with state: {}", data))
    }
}

/// Demonstrates working with Pin<&mut T>
async fn pin_mut_example() {
    let mut value = 42;
    let pinned = Pin::new(&mut value);
    
    // Can't move the pinned value
    // let moved = pinned; // This would be an error if value was !Unpin
    
    println!("    Pinned value: {}", *pinned);
}

/// Demonstrates Pin::as_mut and Pin::get_mut
async fn pin_operations_example() {
    let mut data = vec![1, 2, 3, 4, 5];
    let mut pinned = Pin::new(&mut data);
    
    // Safe because Vec<T> implements Unpin
    pinned.as_mut().get_mut().push(6);
    
    println!("    Modified pinned vector: {:?}", pinned.get_ref());
}

/// Demonstrates unsafe pinning operations
async fn unsafe_pinning_example() {
    let pinned_future = PinnedFuture::new("test_state".to_string());
    let result = pinned_future.await;
    println!("    Pinned future result: {}", result);
}

pub async fn run_examples() {
    println!("  Self-Referential Struct:");
    let self_ref = SelfReferential::new("Hello, Pin!".to_string());
    println!("    Data: {}", self_ref.get_data());
    println!("    Pointer data: {}", self_ref.get_pointer_data());
    
    println!("  Pin Operations:");
    pin_mut_example().await;
    pin_operations_example().await;
    
    println!("  Unsafe Pinning:");
    unsafe_pinning_example().await;
    
    println!("  Pin in Practice:");
    // This demonstrates why pinning is important for async
    let future = async {
        let local_data = "local".to_string();
        // If this future contained self-references, they would need to be pinned
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
        format!("Processed: {}", local_data)
    };
    
    let result = future.await;
    println!("    Async with local data: {}", result);
}