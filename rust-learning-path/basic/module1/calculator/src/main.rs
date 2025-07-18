use std::io;
use calculator::{parse_input, perform_calculation, CalculatorError};

fn main() {
    println!("Welcome to the Rust Calculator!");
    println!("Enter expressions (e.g., '5 + 3', '10 * 2')");
    println!("Type 'exit' to quit");
    
    loop {
        println!("\nEnter an expression:");
        
        // Read user input
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                // Check if the user wants to exit
                let trimmed_input = input.trim();
                if trimmed_input.eq_ignore_ascii_case("exit") {
                    println!("Goodbye!");
                    break;
                }
                
                // Process the input
                match parse_input(&input) {
                    Ok(calculation) => {
                        // Perform the calculation
                        match perform_calculation(&calculation) {
                            Ok(result) => println!("Result: {}", result),
                            Err(err) => print_error(err),
                        }
                    }
                    Err(err) => print_error(err),
                }
            }
            Err(error) => println!("Error reading input: {}", error),
        }
    }
}



/// Print a user-friendly error message
/// 
/// # Arguments
/// * `error` - The CalculatorError to print
fn print_error(error: CalculatorError) {
    println!("{}", error);
}