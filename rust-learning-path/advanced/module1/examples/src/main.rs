use concurrency_examples::*;

fn main() {
    println!("🦀 Rust Concurrency Examples 🦀\n");

    // Basic thread examples
    basic_thread_example();
    move_data_example();
    thread_builder_example();
    multiple_threads_example();

    // Message passing examples
    basic_channel_example();
    multiple_producers_example();
    bounded_channel_example();
    non_blocking_receive_example();
    timed_receive_example();
    work_distribution_example();

    // Shared state examples
    basic_mutex_example();
    rwlock_example();
    condvar_example();
    producer_consumer_example();
    mutex_poisoning_example();
    thread_safe_counter_example();

    // Atomic operations examples
    basic_atomic_example();
    atomic_operations_example();
    compare_exchange_weak_example();
    memory_ordering_example();
    relaxed_ordering_example();
    sequential_consistency_example();
    atomic_flag_example();
    lock_free_example();

    // Lock-free data structures examples
    lock_free_stack_example();
    lock_free_queue_example();
    atomic_counter_example();
    performance_comparison();

    println!("\n🎉 All concurrency examples completed!");
    println!("\nKey takeaways:");
    println!("• Rust's type system prevents data races at compile time");
    println!("• Use channels for message passing between threads");
    println!("• Use Mutex/RwLock for shared mutable state");
    println!("• Atomic operations provide lock-free synchronization");
    println!("• Memory ordering is crucial for correctness");
    println!("• Lock-free data structures can improve performance");
}