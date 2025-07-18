// Example: Variables and Mutability
fn main() {
    // Immutable variable (default)
    let x = 5;
    println!("The value of x is: {}", x);
    
    // This would cause a compilation error:
    // x = 6;
    
    // Mutable variable
    let mut y = 5;
    println!("The value of y is: {}", y);
    
    y = 6;
    println!("The value of y is now: {}", y);
    
    // Shadowing
    let z = 5;
    let z = z + 1;  // Shadows the previous z
    
    {
        let z = z * 2;  // Shadows z within this scope
        println!("The value of z in the inner scope is: {}", z);  // Output: 12
    }
    
    println!("The value of z is: {}", z);  // Output: 6
    
    // Constants
    const MAX_POINTS: u32 = 100_000;
    println!("The maximum points are: {}", MAX_POINTS);
    
    // Type annotations
    let a: i32 = 5;
    let b: f64 = 2.0;
    let c: bool = true;
    let d: char = 'z';
    
    println!("a: {}, b: {}, c: {}, d: {}", a, b, c, d);
}