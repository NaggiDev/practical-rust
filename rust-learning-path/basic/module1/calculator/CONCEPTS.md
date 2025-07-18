# Rust Concepts in the Command Line Calculator

This document explains the key Rust concepts used in the Command Line Calculator project.

## 1. Variables and Data Types

### Variables
In Rust, variables are immutable by default. To make them mutable, you use the `mut` keyword:

```rust
let mut input = String::new();
```

### Primitive Types
The calculator uses several primitive types:
- `f64`: 64-bit floating-point numbers for calculations
- `&str`: String slices for processing input
- `String`: Owned string type for storing user input

## 2. Enums and Pattern Matching

### Enums
Enums in Rust are a way to define a type that can be one of several variants:

```rust
enum Operation {
    Add,
    Subtract,
    Multiply,
    Divide,
}
```

### Pattern Matching
The `match` expression allows you to compare a value against a series of patterns:

```rust
match operation {
    Operation::Add => left + right,
    Operation::Subtract => left - right,
    Operation::Multiply => left * right,
    Operation::Divide => left / right,
}
```

## 3. Structs

Structs are used to create custom data types that group related values:

```rust
struct Calculation {
    left_operand: f64,
    right_operand: f64,
    operation: Operation,
}
```

## 4. Error Handling

### Custom Error Types
Rust allows you to define custom error types using enums:

```rust
enum CalculatorError {
    InvalidInput(String),
    DivisionByZero,
    UnknownOperation(String),
}
```

### Result Type
The `Result<T, E>` type is used for operations that can fail:

```rust
fn parse_input(input: &str) -> Result<Calculation, CalculatorError> {
    // Implementation
}
```

### Error Propagation
The `?` operator can be used to propagate errors (not shown in the starter code but can be used in the implementation).

## 5. Functions and Methods

Functions in Rust are defined using the `fn` keyword:

```rust
fn perform_calculation(calculation: &Calculation) -> Result<f64, CalculatorError> {
    // Implementation
}
```

## 6. String Manipulation

The calculator project involves several string operations:
- Reading strings from standard input
- Parsing strings into tokens
- Converting strings to numeric types

## 7. Documentation Comments

Rust has a special syntax for documentation comments that can be processed by tools like `rustdoc`:

```rust
/// Parse the input string into a Calculation struct
/// 
/// # Arguments
/// * `input` - A string slice containing the expression to parse
```

## 8. Ownership and Borrowing

The calculator demonstrates Rust's ownership system:
- Passing references to functions (`&str`, `&Calculation`)
- Creating owned values (`String`, `Calculation`)
- Borrowing values for operations

## 9. Standard Library Usage

The project uses several components from Rust's standard library:
- `std::io` for input/output operations
- String manipulation functions
- Parsing functions for converting strings to numbers