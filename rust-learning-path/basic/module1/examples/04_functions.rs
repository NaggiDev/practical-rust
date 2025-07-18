// Example: Functions
fn main() {
    println!("Hello from main!");
    
    // Calling a function without parameters
    another_function();
    
    // Calling a function with parameters
    function_with_parameters(5, 'h');
    
    // Calling a function with a return value
    let result = function_with_return_value(5);
    println!("The result is: {}", result);
    
    // Using early return
    let abs_value = absolute_value(-42);
    println!("The absolute value of -42 is: {}", abs_value);
    
    // Function with expression body
    let sum = add(10, 20);
    println!("10 + 20 = {}", sum);
    
    // Function with multiple return values using a tuple
    let stats = calculate_statistics(&[1, 2, 3, 4, 5]);
    println!("Statistics - sum: {}, average: {}, min: {}, max: {}", 
             stats.0, stats.1, stats.2, stats.3);
    
    // Function with named parameters
    print_coordinates(x: 3, y: 5);
}

// Function without parameters or return value
fn another_function() {
    println!("Hello from another function!");
}

// Function with parameters
fn function_with_parameters(x: i32, y: char) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

// Function with return value
fn function_with_return_value(x: i32) -> i32 {
    x + 1  // Expression that returns a value
}

// Function with early return
fn absolute_value(x: i32) -> i32 {
    if x < 0 {
        return -x;  // Early return
    }
    x  // Return x if it's positive or zero
}

// Function with expression body (no return keyword needed)
fn add(a: i32, b: i32) -> i32 {
    a + b
}

// Function that returns multiple values using a tuple
fn calculate_statistics(numbers: &[i32]) -> (i32, f64, i32, i32) {
    let sum: i32 = numbers.iter().sum();
    let avg: f64 = sum as f64 / numbers.len() as f64;
    let min: i32 = *numbers.iter().min().unwrap_or(&0);
    let max: i32 = *numbers.iter().max().unwrap_or(&0);
    
    (sum, avg, min, max)
}

// Function with named parameters (this is not actually supported in Rust,
// but we can simulate it with a comment to show the parameter names)
fn print_coordinates(/* x: */ x: i32, /* y: */ y: i32) {
    println!("Coordinates: ({}, {})", x, y);
}