# Advanced Ownership Examples

This directory contains practical examples demonstrating the advanced ownership concepts covered in Module 1.

## Running the Examples

Each example can be run using the Rust compiler:

```bash
# Run the borrowing example
rustc borrowing.rs
./borrowing

# Run the lifetimes example
rustc lifetimes.rs
./lifetimes

# Run the smart pointers example
rustc smart_pointers.rs
./smart_pointers
```

## Running the Tests

The examples include comprehensive tests that validate understanding of the concepts. You can run these tests using:

```bash
# Run tests for borrowing
rustc --test borrowing.rs
./borrowing

# Run tests for lifetimes
rustc --test lifetimes.rs
./lifetimes

# Run tests for smart pointers
rustc --test smart_pointers.rs
./smart_pointers

# Run the comprehensive ownership tests
rustc --test ownership_tests.rs
./ownership_tests
```

## Example Files

1. **borrowing.rs**: Demonstrates Rust's borrowing rules, including immutable and mutable borrows, and non-lexical lifetimes.

2. **lifetimes.rs**: Shows how to use lifetime annotations in functions and structs, and explains lifetime elision rules.

3. **smart_pointers.rs**: Covers various smart pointers in Rust, including Box<T>, Rc<T>, Arc<T>, RefCell<T>, and how to combine them.

4. **ownership_tests.rs**: Comprehensive tests for all advanced ownership concepts, providing a way to validate your understanding.

## Learning Approach

For each example:

1. Read through the code and comments to understand the concepts
2. Run the example to see the output
3. Run the tests to verify your understanding
4. Try modifying the code to experiment with the concepts
5. Check if your modifications compile and work as expected

This hands-on approach will help reinforce your understanding of Rust's advanced ownership system.