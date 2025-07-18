# Command Line Calculator

## Project Overview

In this project, you will build a simple command-line calculator that can perform basic arithmetic operations. This project will help you practice Rust fundamentals including variables, functions, control flow, and basic error handling.

## Learning Objectives

By completing this project, you will:

- Parse and process user input in Rust
- Work with numeric types and operations
- Implement error handling for invalid inputs
- Create and use functions
- Practice control flow with match expressions

## Project Requirements

Your calculator should:

1. Accept arithmetic expressions from the command line
2. Support basic operations: addition, subtraction, multiplication, division
3. Handle errors gracefully (division by zero, invalid input)
4. Display the result or an appropriate error message
5. Support both integer and floating-point calculations

## Getting Started

1. Create a new Rust project:
   ```bash
   cargo new calculator
   cd calculator
   ```

2. Open `src/main.rs` and replace its contents with the starter code provided below.

3. Follow the step-by-step instructions in the code comments to complete the implementation.

## Starter Code

```rust
fn main() {
    println!("Welcome to the Rust Calculator!");
    println!("Enter an expression (e.g., '5 + 3', '10 * 2'):");
    
    // TODO: Read user input
    
    // TODO: Parse the input into operands and operator
    
    // TODO: Perform the calculation
    
    // TODO: Display the result
}

// TODO: Implement a function to parse the input

// TODO: Implement a function to perform the calculation
```

## Step-by-Step Instructions

1. **Read user input**
   - Use the `std::io` module to read a line of input from the user
   - Trim whitespace from the input

2. **Parse the input**
   - Split the input string into tokens (operands and operator)
   - Convert the operands to numbers
   - Handle potential parsing errors

3. **Perform the calculation**
   - Implement a function that takes two operands and an operator
   - Use a match expression to handle different operators
   - Return a Result type to handle potential errors (like division by zero)

4. **Display the result**
   - Show the result to the user if the calculation was successful
   - Display an appropriate error message if something went wrong

## Testing Your Implementation

Test your calculator with various inputs:

- Basic operations: `5 + 3`, `10 - 7`, `4 * 6`, `8 / 2`
- Floating-point numbers: `3.5 + 2.1`, `10.0 / 3.0`
- Error cases: `5 / 0` (division by zero), `abc + 5` (invalid input)

## Extension Challenges

After completing the basic implementation, try these extensions:

1. Add support for more operations (exponentiation, modulo, etc.)
2. Implement support for parentheses to control order of operations
3. Add memory functions (store and recall values)
4. Support more complex expressions with multiple operations

## Concepts Applied

- Variables and data types
- Functions and return values
- Error handling with Result
- Pattern matching with match expressions
- String parsing and conversion
- User input and output

## Resources

- [The Rust Book - Using Structs to Structure Related Data](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [The Rust Book - Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [The Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)