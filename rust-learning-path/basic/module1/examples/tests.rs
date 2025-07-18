// Tests for basic Rust concepts
// Run with: rustc tests.rs && ./tests

// Test functions for variables and data types
fn test_variables() {
    // Test immutable variables
    let x = 5;
    assert_eq!(x, 5);
    
    // Test mutable variables
    let mut y = 5;
    y = 10;
    assert_eq!(y, 10);
    
    // Test shadowing
    let z = 5;
    let z = z + 1;
    assert_eq!(z, 6);
    
    {
        let z = z * 2;
        assert_eq!(z, 12);
    }
    
    assert_eq!(z, 6);
    
    println!("✓ Variables tests passed");
}

fn test_data_types() {
    // Test integer types
    let a: i8 = 127;
    let b: u8 = 255;
    assert_eq!(a, 127);
    assert_eq!(b, 255);
    
    // Test floating-point types
    let x = 2.0;      // f64
    let y: f32 = 3.0; // f32
    assert_eq!(x, 2.0);
    assert_eq!(y, 3.0);
    
    // Test boolean type
    let t = true;
    let f: bool = false;
    assert_eq!(t, true);
    assert_eq!(f, false);
    
    // Test character type
    let c = 'z';
    assert_eq!(c, 'z');
    
    // Test tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    assert_eq!(tup.0, 500);
    assert_eq!(tup.1, 6.4);
    assert_eq!(tup.2, 1);
    
    // Test array type
    let arr: [i32; 5] = [1, 2, 3, 4, 5];
    assert_eq!(arr[0], 1);
    assert_eq!(arr[4], 5);
    assert_eq!(arr.len(), 5);
    
    println!("✓ Data types tests passed");
}

fn test_control_flow() {
    // Test if expression
    let number = 6;
    let result = if number % 2 == 0 { "even" } else { "odd" };
    assert_eq!(result, "even");
    
    // Test loop with break value
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };
    assert_eq!(result, 20);
    
    // Test while loop
    let mut sum = 0;
    let mut i = 1;
    while i <= 5 {
        sum += i;
        i += 1;
    }
    assert_eq!(sum, 15);  // 1 + 2 + 3 + 4 + 5 = 15
    
    // Test for loop
    let a = [10, 20, 30, 40, 50];
    let mut sum = 0;
    for element in a.iter() {
        sum += element;
    }
    assert_eq!(sum, 150);  // 10 + 20 + 30 + 40 + 50 = 150
    
    // Test match expression
    let dice_roll = 4;
    let result = match dice_roll {
        3 => "three",
        4 => "four",
        _ => "other",
    };
    assert_eq!(result, "four");
    
    println!("✓ Control flow tests passed");
}

fn test_functions() {
    // Test function with return value
    assert_eq!(add(2, 3), 5);
    
    // Test function with early return
    assert_eq!(absolute_value(-5), 5);
    assert_eq!(absolute_value(5), 5);
    
    // Test function with multiple return values
    let stats = calculate_statistics(&[1, 2, 3, 4, 5]);
    assert_eq!(stats.0, 15);  // sum
    assert_eq!(stats.1, 3.0);  // average
    assert_eq!(stats.2, 1);  // min
    assert_eq!(stats.3, 5);  // max
    
    println!("✓ Functions tests passed");
}

// Helper functions for tests
fn add(a: i32, b: i32) -> i32 {
    a + b
}

fn absolute_value(x: i32) -> i32 {
    if x < 0 {
        return -x;
    }
    x
}

fn calculate_statistics(numbers: &[i32]) -> (i32, f64, i32, i32) {
    let sum: i32 = numbers.iter().sum();
    let avg: f64 = sum as f64 / numbers.len() as f64;
    let min: i32 = *numbers.iter().min().unwrap_or(&0);
    let max: i32 = *numbers.iter().max().unwrap_or(&0);
    
    (sum, avg, min, max)
}

// Main function to run all tests
fn main() {
    println!("Running tests for basic Rust concepts...\n");
    
    test_variables();
    test_data_types();
    test_control_flow();
    test_functions();
    
    println!("\nAll tests passed!");
}