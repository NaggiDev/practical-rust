use std::sync::mpsc;
use std::thread;
use std::time::Duration;

/// Demonstrates basic channel communication
pub fn basic_channel_example() {
    println!("=== Basic Channel Example ===");
    
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let messages = vec![
            "Hello".to_string(),
            "from".to_string(),
            "the".to_string(),
            "thread".to_string(),
        ];

        for (i, msg) in messages.into_iter().enumerate() {
            println!("Sending: {}", msg);
            tx.send(msg).unwrap();
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Receive messages
    for received in rx {
        println!("Received: {}", received);
    }
}

/// Demonstrates multiple producers with one receiver
pub fn multiple_producers_example() {
    println!("\n=== Multiple Producers Example ===");
    
    let (tx, rx) = mpsc::channel();
    let mut handles = vec![];

    for i in 0..3 {
        let tx_clone = tx.clone();
        let handle = thread::spawn(move || {
            for j in 0..3 {
                let msg = format!("Producer {} - Message {}", i, j);
                println!("Sending: {}", msg);
                tx_clone.send(msg).unwrap();
                thread::sleep(Duration::from_millis(50));
            }
        });
        handles.push(handle);
    }

    // Drop the original sender so the receiver knows when all senders are done
    drop(tx);

    // Wait for all producers to finish
    for handle in handles {
        handle.join().unwrap();
    }

    // Receive all messages
    for received in rx {
        println!("Received: {}", received);
    }
}

/// Demonstrates bounded channels (sync_channel)
pub fn bounded_channel_example() {
    println!("\n=== Bounded Channel Example ===");
    
    let (tx, rx) = mpsc::sync_channel(2); // Buffer size of 2

    let producer = thread::spawn(move || {
        for i in 0..5 {
            let msg = format!("Message {}", i);
            println!("Attempting to send: {}", msg);
            tx.send(msg).unwrap();
            println!("Sent successfully");
            thread::sleep(Duration::from_millis(100));
        }
    });

    // Slow consumer
    thread::sleep(Duration::from_millis(500));
    
    for received in rx {
        println!("Received: {}", received);
        thread::sleep(Duration::from_millis(200)); // Slow processing
    }

    producer.join().unwrap();
}

/// Demonstrates try_recv for non-blocking receive
pub fn non_blocking_receive_example() {
    println!("\n=== Non-blocking Receive Example ===");
    
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(500));
        tx.send("Delayed message".to_string()).unwrap();
    });

    // Try to receive immediately (will fail)
    match rx.try_recv() {
        Ok(msg) => println!("Received immediately: {}", msg),
        Err(mpsc::TryRecvError::Empty) => println!("No message available yet"),
        Err(mpsc::TryRecvError::Disconnected) => println!("Channel disconnected"),
    }

    // Wait and try again
    thread::sleep(Duration::from_millis(600));
    match rx.try_recv() {
        Ok(msg) => println!("Received after delay: {}", msg),
        Err(mpsc::TryRecvError::Empty) => println!("Still no message"),
        Err(mpsc::TryRecvError::Disconnected) => println!("Channel disconnected"),
    }
}

/// Demonstrates recv_timeout for timed receive
pub fn timed_receive_example() {
    println!("\n=== Timed Receive Example ===");
    
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        thread::sleep(Duration::from_millis(800));
        tx.send("Eventually sent".to_string()).unwrap();
    });

    // Try to receive with timeout
    match rx.recv_timeout(Duration::from_millis(500)) {
        Ok(msg) => println!("Received within timeout: {}", msg),
        Err(mpsc::RecvTimeoutError::Timeout) => println!("Timeout occurred"),
        Err(mpsc::RecvTimeoutError::Disconnected) => println!("Channel disconnected"),
    }

    // Try again with longer timeout
    match rx.recv_timeout(Duration::from_millis(500)) {
        Ok(msg) => println!("Received with longer timeout: {}", msg),
        Err(mpsc::RecvTimeoutError::Timeout) => println!("Timeout occurred again"),
        Err(mpsc::RecvTimeoutError::Disconnected) => println!("Channel disconnected"),
    }
}

/// Work item for demonstrating work distribution
#[derive(Debug, Clone)]
pub struct WorkItem {
    pub id: usize,
    pub data: String,
}

/// Demonstrates distributing work across multiple threads
pub fn work_distribution_example() {
    println!("\n=== Work Distribution Example ===");
    
    let (work_tx, work_rx) = mpsc::channel();
    let (result_tx, result_rx) = mpsc::channel();

    // Create worker threads
    let num_workers = 3;
    let mut worker_handles = vec![];

    for worker_id in 0..num_workers {
        let work_rx = work_rx.clone();
        let result_tx = result_tx.clone();

        let handle = thread::spawn(move || {
            while let Ok(work_item) = work_rx.recv() {
                let WorkItem { id, data } = work_item;
                println!("Worker {} processing item {}", worker_id, id);
                
                // Simulate work
                thread::sleep(Duration::from_millis(200));
                
                let result = format!("Worker {} processed: {}", worker_id, data);
                result_tx.send((id, result)).unwrap();
            }
            println!("Worker {} shutting down", worker_id);
        });
        
        worker_handles.push(handle);
    }

    // Drop the original receivers so workers know when to stop
    drop(work_rx);
    drop(result_tx);

    // Send work items
    for i in 0..10 {
        let work_item = WorkItem {
            id: i,
            data: format!("Task {}", i),
        };
        work_tx.send(work_item).unwrap();
    }

    // Drop sender to signal no more work
    drop(work_tx);

    // Collect results
    let mut results = vec![];
    while let Ok((id, result)) = result_rx.recv() {
        println!("Result {}: {}", id, result);
        results.push((id, result));
    }

    // Wait for all workers to finish
    for handle in worker_handles {
        handle.join().unwrap();
    }

    println!("Processed {} work items", results.len());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_channel() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            tx.send(42).unwrap();
        });

        let received = rx.recv().unwrap();
        assert_eq!(received, 42);
    }

    #[test]
    fn test_multiple_messages() {
        let (tx, rx) = mpsc::channel();

        thread::spawn(move || {
            for i in 0..5 {
                tx.send(i).unwrap();
            }
        });

        let mut received = vec![];
        for msg in rx {
            received.push(msg);
        }

        assert_eq!(received, vec![0, 1, 2, 3, 4]);
    }

    #[test]
    fn test_multiple_producers() {
        let (tx, rx) = mpsc::channel();
        let mut handles = vec![];

        for i in 0..3 {
            let tx_clone = tx.clone();
            let handle = thread::spawn(move || {
                tx_clone.send(i).unwrap();
            });
            handles.push(handle);
        }

        drop(tx); // Important: drop original sender

        for handle in handles {
            handle.join().unwrap();
        }

        let mut received = vec![];
        for msg in rx {
            received.push(msg);
        }

        received.sort(); // Order is not guaranteed
        assert_eq!(received, vec![0, 1, 2]);
    }

    #[test]
    fn test_bounded_channel() {
        let (tx, rx) = mpsc::sync_channel(1);

        // First send should succeed immediately
        tx.send(1).unwrap();

        // Second send would block, so we test try_send
        match tx.try_send(2) {
            Ok(_) => panic!("Should have failed due to full buffer"),
            Err(mpsc::TrySendError::Full(_)) => {
                // Expected behavior
            }
            Err(_) => panic!("Unexpected error"),
        }

        // Receive one message to make space
        assert_eq!(rx.recv().unwrap(), 1);

        // Now second send should succeed
        tx.send(2).unwrap();
        assert_eq!(rx.recv().unwrap(), 2);
    }

    #[test]
    fn test_try_recv() {
        let (tx, rx) = mpsc::channel();

        // Should be empty initially
        match rx.try_recv() {
            Err(mpsc::TryRecvError::Empty) => {}, // Expected
            _ => panic!("Should be empty"),
        }

        tx.send(42).unwrap();

        // Should now have a message
        match rx.try_recv() {
            Ok(42) => {}, // Expected
            _ => panic!("Should have received 42"),
        }
    }
}