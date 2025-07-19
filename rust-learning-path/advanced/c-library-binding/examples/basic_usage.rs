use c_library_binding::{arrays, math, strings};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Basic FFI Usage Examples ===\n");

    // Example 1: Simple mathematical operations
    println!("1. Mathematical Operations:");
    let a = 15;
    let b = 7;
    println!("   {} + {} = {}", a, b, math::add(a, b));
    println!("   {} * {} = {}", a, b, math::multiply(a, b));
    
    match math::factorial(6) {
        Ok(result) => println!("   6! = {}", result),
        Err(e) => println!("   Error: {}", e),
    }
    println!();

    // Example 2: String manipulations
    println!("2. String Operations:");
    let original = "Rust FFI";
    
    match strings::reverse(original) {
        Ok(reversed) => println!("   '{}' reversed: '{}'", original, reversed),
        Err(e) => println!("   Error reversing: {}", e),
    }
    
    match strings::uppercase(original) {
        Ok(upper) => println!("   '{}' uppercase: '{}'", original, upper),
        Err(e) => println!("   Error converting to uppercase: {}", e),
    }
    
    match strings::length(original) {
        Ok(len) => println!("   '{}' length: {}", original, len),
        Err(e) => println!("   Error getting length: {}", e),
    }
    println!();

    // Example 3: Array processing
    println!("3. Array Operations:");
    let data = [10, 25, 5, 30, 15, 8];
    println!("   Array: {:?}", data);
    println!("   Sum: {}", arrays::sum(&data));
    
    match arrays::find_maximum(&data) {
        Ok(max) => println!("   Maximum: {}", max),
        Err(e) => println!("   Error finding maximum: {}", e),
    }
    println!();

    // Example 4: Error handling demonstration
    println!("4. Error Handling:");
    match math::factorial(25) {
        Ok(result) => println!("   25! = {}", result),
        Err(e) => println!("   Expected error for 25!: {}", e),
    }
    
    let empty_array: &[i32] = &[];
    match arrays::find_maximum(empty_array) {
        Ok(max) => println!("   Empty array max: {}", max),
        Err(e) => println!("   Expected error for empty array: {}", e),
    }

    Ok(())
}