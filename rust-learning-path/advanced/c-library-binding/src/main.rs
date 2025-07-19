use c_library_binding::{arrays, math, strings, memory};

fn main() {
    println!("=== C Library Binding Demo ===\n");

    // Demonstrate mathematical operations
    println!("Mathematical Operations:");
    println!("5 + 3 = {}", math::add(5, 3));
    println!("4 * 7 = {}", math::multiply(4, 7));
    
    match math::factorial(5) {
        Ok(result) => println!("5! = {}", result),
        Err(e) => println!("Error calculating factorial: {}", e),
    }
    
    match math::factorial(25) {
        Ok(result) => println!("25! = {}", result),
        Err(e) => println!("Error calculating 25!: {}", e),
    }
    println!();

    // Demonstrate string operations
    println!("String Operations:");
    match strings::reverse("Hello, World!") {
        Ok(result) => println!("Reversed: '{}'", result),
        Err(e) => println!("Error reversing string: {}", e),
    }
    
    match strings::uppercase("hello, world!") {
        Ok(result) => println!("Uppercase: '{}'", result),
        Err(e) => println!("Error converting to uppercase: {}", e),
    }
    
    match strings::length("Hello, World!") {
        Ok(len) => println!("Length: {}", len),
        Err(e) => println!("Error getting length: {}", e),
    }
    println!();

    // Demonstrate array operations
    println!("Array Operations:");
    let numbers = [1, 5, 3, 9, 2, 7, 4];
    println!("Array: {:?}", numbers);
    println!("Sum: {}", arrays::sum(&numbers));
    
    match arrays::find_maximum(&numbers) {
        Ok(max) => println!("Maximum: {}", max),
        Err(e) => println!("Error finding maximum: {}", e),
    }
    
    let empty_array: &[i32] = &[];
    println!("Empty array sum: {}", arrays::sum(empty_array));
    match arrays::find_maximum(empty_array) {
        Ok(max) => println!("Empty array maximum: {}", max),
        Err(e) => println!("Error with empty array: {}", e),
    }
    println!();

    // Demonstrate memory management
    println!("Memory Management:");
    match memory::CAllocatedString::new(50) {
        Ok(c_string) => {
            println!("Successfully allocated C string with 50 bytes");
            // The string will be automatically freed when it goes out of scope
        }
        Err(e) => println!("Error allocating C string: {}", e),
    }
    
    println!("\n=== Demo Complete ===");
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_full_workflow() {
        // Test a complete workflow using multiple FFI functions
        let input = "rust";
        
        // Get original length
        let original_length = strings::length(input).unwrap();
        
        // Reverse the string
        let reversed = strings::reverse(input).unwrap();
        
        // Convert to uppercase
        let uppercase = strings::uppercase(&reversed).unwrap();
        
        // Verify length is preserved
        let final_length = strings::length(&uppercase).unwrap();
        
        assert_eq!(original_length, final_length);
        assert_eq!(uppercase, "TSUR");
    }

    #[test]
    fn test_error_propagation() {
        // Test that errors are properly propagated from C to Rust
        let result = math::factorial(30);
        assert!(result.is_err());
        
        let empty_array: &[i32] = &[];
        let result = arrays::find_maximum(empty_array);
        assert!(result.is_err());
    }

    #[test]
    fn test_memory_safety() {
        // Test that memory is properly managed
        for _ in 0..100 {
            let _c_string = memory::CAllocatedString::new(1000).unwrap();
            // Each allocation should be automatically freed
        }
        // If there were memory leaks, this test would eventually fail
    }
}