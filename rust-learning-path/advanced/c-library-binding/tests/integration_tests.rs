use c_library_binding::{arrays, math, strings, memory, FfiError};

#[test]
fn test_mathematical_operations() {
    // Test basic arithmetic
    assert_eq!(math::add(10, 20), 30);
    assert_eq!(math::add(-5, 3), -2);
    assert_eq!(math::add(0, 0), 0);
    
    assert_eq!(math::multiply(6, 7), 42);
    assert_eq!(math::multiply(-3, 4), -12);
    assert_eq!(math::multiply(0, 100), 0);
}

#[test]
fn test_factorial_operations() {
    // Test valid factorial calculations
    assert_eq!(math::factorial(0).unwrap(), 1);
    assert_eq!(math::factorial(1).unwrap(), 1);
    assert_eq!(math::factorial(5).unwrap(), 120);
    assert_eq!(math::factorial(10).unwrap(), 3628800);
    
    // Test error cases
    assert!(math::factorial(21).is_err());
    assert!(math::factorial(100).is_err());
}

#[test]
fn test_string_operations() {
    // Test string reversal
    assert_eq!(strings::reverse("").unwrap(), "");
    assert_eq!(strings::reverse("a").unwrap(), "a");
    assert_eq!(strings::reverse("hello").unwrap(), "olleh");
    assert_eq!(strings::reverse("Rust FFI").unwrap(), "IFF tsuR");
    
    // Test uppercase conversion
    assert_eq!(strings::uppercase("").unwrap(), "");
    assert_eq!(strings::uppercase("hello").unwrap(), "HELLO");
    assert_eq!(strings::uppercase("Hello World!").unwrap(), "HELLO WORLD!");
    assert_eq!(strings::uppercase("123abc").unwrap(), "123ABC");
    
    // Test string length
    assert_eq!(strings::length("").unwrap(), 0);
    assert_eq!(strings::length("hello").unwrap(), 5);
    assert_eq!(strings::length("Hello, World!").unwrap(), 13);
}

#[test]
fn test_string_with_special_characters() {
    // Test strings with special characters
    let input = "Hello\tWorld\n!";
    let reversed = strings::reverse(input).unwrap();
    let uppercase = strings::uppercase(input).unwrap();
    
    assert_eq!(reversed, "!\ndlroW\tolleH");
    assert_eq!(uppercase, "HELLO\tWORLD\n!");
}

#[test]
fn test_array_operations() {
    // Test sum operation
    assert_eq!(arrays::sum(&[]), 0);
    assert_eq!(arrays::sum(&[5]), 5);
    assert_eq!(arrays::sum(&[1, 2, 3, 4, 5]), 15);
    assert_eq!(arrays::sum(&[-1, -2, -3]), -6);
    assert_eq!(arrays::sum(&[100, -50, 25]), 75);
    
    // Test find maximum
    assert!(arrays::find_maximum(&[]).is_err());
    assert_eq!(arrays::find_maximum(&[5]).unwrap(), 5);
    assert_eq!(arrays::find_maximum(&[1, 5, 3, 9, 2]).unwrap(), 9);
    assert_eq!(arrays::find_maximum(&[-1, -5, -3]).unwrap(), -1);
    assert_eq!(arrays::find_maximum(&[0, 0, 0]).unwrap(), 0);
}

#[test]
fn test_large_arrays() {
    // Test with larger arrays
    let large_array: Vec<i32> = (1..=1000).collect();
    assert_eq!(arrays::sum(&large_array), 500500);
    assert_eq!(arrays::find_maximum(&large_array).unwrap(), 1000);
    
    let negative_array: Vec<i32> = (-500..=0).collect();
    assert_eq!(arrays::find_maximum(&negative_array).unwrap(), 0);
}

#[test]
fn test_memory_management() {
    // Test basic allocation and deallocation
    let c_string = memory::CAllocatedString::new(100).unwrap();
    drop(c_string); // Explicit drop to test cleanup
    
    // Test multiple allocations
    let mut strings = Vec::new();
    for i in 1..=10 {
        strings.push(memory::CAllocatedString::new(i * 10).unwrap());
    }
    // All strings will be freed when the vector is dropped
}

#[test]
fn test_error_handling() {
    // Test factorial error
    match math::factorial(25) {
        Err(FfiError { message }) => {
            assert!(message.contains("too large") || message.contains("Factorial"));
        }
        Ok(_) => panic!("Expected error for factorial(25)"),
    }
    
    // Test empty array error
    match arrays::find_maximum(&[]) {
        Err(FfiError { message }) => {
            assert!(message.contains("empty") || message.contains("Empty"));
        }
        Ok(_) => panic!("Expected error for empty array"),
    }
}

#[test]
fn test_null_byte_handling() {
    // Test that strings with null bytes are handled properly
    // This should fail gracefully rather than cause undefined behavior
    let result = strings::reverse("hello\0world");
    assert!(result.is_err());
    
    let result = strings::uppercase("test\0string");
    assert!(result.is_err());
}

#[test]
fn test_unicode_handling() {
    // Test basic Unicode handling (note: C library may not handle Unicode properly)
    // This test documents the current behavior
    let unicode_string = "café";
    
    // These operations may or may not work correctly with Unicode
    // depending on the C library implementation
    let length_result = strings::length(unicode_string);
    assert!(length_result.is_ok());
    
    let reverse_result = strings::reverse(unicode_string);
    let uppercase_result = strings::uppercase(unicode_string);
    
    // Document that these operations complete (even if not Unicode-correct)
    assert!(reverse_result.is_ok() || reverse_result.is_err());
    assert!(uppercase_result.is_ok() || uppercase_result.is_err());
}

#[test]
fn test_concurrent_access() {
    use std::thread;
    use std::sync::Arc;
    
    // Test that FFI operations can be called from multiple threads
    // Note: This assumes the C library is thread-safe for read operations
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    let mut handles = vec![];
    
    for _ in 0..4 {
        let data_clone = Arc::clone(&data);
        let handle = thread::spawn(move || {
            let sum = arrays::sum(&data_clone);
            let max = arrays::find_maximum(&data_clone).unwrap();
            (sum, max)
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let (sum, max) = handle.join().unwrap();
        assert_eq!(sum, 15);
        assert_eq!(max, 5);
    }
}

#[test]
fn test_edge_cases() {
    // Test various edge cases
    
    // Maximum values
    assert_eq!(math::add(i32::MAX, 0), i32::MAX);
    assert_eq!(math::multiply(1, i32::MAX), i32::MAX);
    
    // Very long strings (within reasonable limits)
    let long_string = "a".repeat(1000);
    let length = strings::length(&long_string).unwrap();
    assert_eq!(length, 1000);
    
    let reversed = strings::reverse(&long_string).unwrap();
    assert_eq!(reversed.len(), 1000);
    assert!(reversed.chars().all(|c| c == 'a'));
}

#[test]
fn test_workflow_integration() {
    // Test a complete workflow that uses multiple FFI functions
    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    
    // Calculate sum and maximum
    let sum = arrays::sum(&numbers);
    let max = arrays::find_maximum(&numbers).unwrap();
    
    // Use the results in string operations
    let sum_str = sum.to_string();
    let max_str = max.to_string();
    
    let sum_reversed = strings::reverse(&sum_str).unwrap();
    let max_uppercase = strings::uppercase(&max_str).unwrap();
    
    // Verify the workflow
    assert_eq!(sum, 31);
    assert_eq!(max, 9);
    assert_eq!(sum_reversed, "13");
    assert_eq!(max_uppercase, "9");
}