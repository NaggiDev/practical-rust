# Rust Basics Examples

This directory contains example code demonstrating fundamental Rust concepts. Each file focuses on a specific concept and includes detailed comments explaining the code.

## Running the Examples

To run any of the example files, use the Rust compiler (`rustc`) followed by the file name:

```bash
rustc 01_variables.rs
./01_variables  # On Windows: 01_variables.exe
```

## Example Files

1. **01_variables.rs** - Demonstrates variables, mutability, shadowing, and constants
2. **02_data_types.rs** - Shows Rust's data types including integers, floats, booleans, characters, tuples, and arrays
3. **03_control_flow.rs** - Illustrates control flow with if/else, loops, match expressions, and if let
4. **04_functions.rs** - Covers function declaration, parameters, return values, and expressions

## Tests

The `tests.rs` file contains tests for all the basic concepts covered in the examples. Run it to validate your understanding:

```bash
rustc tests.rs
./tests  # On Windows: tests.exe
```

If all tests pass, you'll see confirmation messages for each concept area.

## Learning Path

These examples are part of Module 1: Rust Basics in the Basic Level of the Rust Learning Path. After studying these examples and running the tests, you should have a good understanding of Rust's fundamental concepts.

Next, try applying these concepts in the [Command Line Calculator](../calculator/README.md) project to reinforce your learning.

## Additional Resources

- [The Rust Book - Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
- [Rust by Example - Primitives](https://doc.rust-lang.org/rust-by-example/primitives.html)
- [Rust Standard Library Documentation](https://doc.rust-lang.org/std/)