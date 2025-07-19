use c_library_binding::{arrays, math, strings, memory};
use std::thread;
use std::sync::Arc;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Advanced FFI Patterns ===\n");

    // Pattern 1: Chaining FFI operations
    println!("1. Chaining FFI Operations:");
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    let sum = arrays::sum(&numbers);
    let max = arrays::find_maximum(&numbers)?;
    
    let sum_str = sum.to_string();
    let max_str = max.to_string();
    
    let sum_reversed = strings::reverse(&sum_str)?;
    let max_upper = strings::uppercase(&max_str)?;
    
    println!("   Numbers: {:?}", numbers);
    println!("   Sum: {} -> Reversed: '{}'", sum, sum_reversed);
    println!("   Max: {} -> Uppercase: '{}'", max, max_upper);
    println!();

    // Pattern 2: Memory management with RAII
    println!("2. Safe Memory Management:");
    {
        let c_string = memory::CAllocatedString::new(256)?;
        println!("   Allocated 256 bytes for C string");
        // String is automatically freed when it goes out of scope
    }
    println!("   C string automatically freed");
    println!();

    // Pattern 3: Batch processing
    println!("3. Batch Processing:");
    let test_strings = vec!["hello", "world", "rust", "ffi"];
    let mut results = Vec::new();
    
    for s in &test_strings {
        let reversed = strings::reverse(s)?;
        let upper = strings::uppercase(&reversed)?;
        results.push(upper);
    }
    
    println!("   Original: {:?}", test_strings);
    println!("   Processed: {:?}", results);
    println!();

    // Pattern 4: Error recovery
    println!("4. Error Recovery:");
    let test_values = vec![5, 10, 25, 3, 7]; // 25 will cause factorial error
    
    for &val in &test_values {
        match math::factorial(val) {
            Ok(result) => println!("   {}! = {}", val, result),
            Err(e) => {
                println!("   {}! failed: {}", val, e);
                // Continue processing despite error
            }
        }
    }
    println!();

    // Pattern 5: Multi-threading (if C library is thread-safe)
    println!("5. Multi-threading:");
    let shared_data = Arc::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    let mut handles = vec![];
    
    for i in 0..3 {
        let data = Arc::clone(&shared_data);
        let handle = thread::spawn(move || {
            let thread_sum = arrays::sum(&data);
            let thread_max = arrays::find_maximum(&data).unwrap_or(0);
            println!("   Thread {}: sum={}, max={}", i, thread_sum, thread_max);
            (thread_sum, thread_max)
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let (sum, max) = handle.join().unwrap();
        // All threads should produce the same results
        assert_eq!(sum, 55);
        assert_eq!(max, 10);
    }
    println!("   All threads completed successfully");
    println!();

    // Pattern 6: Data transformation pipeline
    println!("6. Data Transformation Pipeline:");
    let input_data = "hello world from rust ffi";
    
    // Step 1: Get length
    let original_length = strings::length(input_data)?;
    
    // Step 2: Convert to uppercase
    let upper_case = strings::uppercase(input_data)?;
    
    // Step 3: Reverse the string
    let final_result = strings::reverse(&upper_case)?;
    
    // Step 4: Verify length is preserved
    let final_length = strings::length(&final_result)?;
    
    println!("   Original: '{}'", input_data);
    println!("   Uppercase: '{}'", upper_case);
    println!("   Final: '{}'", final_result);
    println!("   Length preserved: {} -> {}", original_length, final_length);
    
    assert_eq!(original_length, final_length);
    println!();

    println!("=== All Advanced Patterns Completed Successfully ===");
    Ok(())
}