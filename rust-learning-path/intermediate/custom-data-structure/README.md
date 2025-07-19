# Custom Data Structure Project

## Learning Objectives

In this project, you'll build a custom dynamic array (vector-like) data structure from scratch. This project will teach you:

- **Traits**: How to implement standard library traits for your custom types
- **Generics**: Creating type-safe, reusable data structures
- **Memory Management**: Working with raw pointers and manual memory allocation
- **Iterator Pattern**: Implementing custom iterators
- **Operator Overloading**: Making your data structure feel native to Rust

## Project Overview

You'll implement `MyVec<T>`, a simplified version of Rust's `Vec<T>` that demonstrates core concepts of generic data structures. Your implementation will support:

- Dynamic resizing
- Generic type storage
- Standard collection operations (push, pop, get, set)
- Iterator support
- Index operator overloading

## Prerequisites

Before starting this project, you should understand:
- Basic Rust syntax and ownership
- Structs and methods
- Generic types basics
- Basic trait usage

## What You'll Build

A custom vector implementation with the following features:

```rust
let mut vec = MyVec::new();
vec.push(1);
vec.push(2);
vec.push(3);

assert_eq!(vec[0], 1);
assert_eq!(vec.len(), 3);

for item in &vec {
    println!("{}", item);
}
```

## Step-by-Step Implementation

### Step 1: Basic Structure and Memory Management
- Define the `MyVec<T>` struct
- Implement basic memory allocation
- Add `new()` and `with_capacity()` constructors

### Step 2: Core Operations
- Implement `push()` and `pop()` methods
- Add `len()` and `capacity()` methods
- Handle dynamic resizing

### Step 3: Access Operations
- Implement `get()` and `get_mut()` methods
- Add bounds checking
- Implement the `Index` and `IndexMut` traits

### Step 4: Iterator Implementation
- Create a custom iterator struct
- Implement the `Iterator` trait
- Implement `IntoIterator` for different reference types

### Step 5: Standard Library Traits
- Implement `Debug` for easy printing
- Implement `Clone` for the vector
- Implement `Drop` for proper cleanup

### Step 6: Advanced Features
- Add `insert()` and `remove()` methods
- Implement `extend()` for adding multiple elements
- Add `clear()` and `truncate()` methods

## Key Concepts Covered

### Traits
- **Standard Library Traits**: `Debug`, `Clone`, `Drop`, `Index`, `IndexMut`
- **Iterator Traits**: `Iterator`, `IntoIterator`
- **Custom Traits**: Creating your own trait definitions

### Generics
- **Generic Structs**: `MyVec<T>` works with any type `T`
- **Generic Methods**: Methods that work with generic types
- **Trait Bounds**: Constraining generic types with trait requirements

### Memory Management
- **Raw Pointers**: Using `*mut T` for manual memory management
- **Allocation**: Using `std::alloc` for memory allocation
- **Safety**: Ensuring memory safety with unsafe blocks

## Getting Started

1. Read through this README completely
2. Look at the starter code in `src/main.rs`
3. Run the tests with `cargo test` to see what needs to be implemented
4. Follow the step-by-step guide, implementing one feature at a time
5. Run tests frequently to validate your progress

## Testing Your Implementation

Run the test suite with:
```bash
cargo test
```

Run the example program with:
```bash
cargo run
```

## Extension Challenges

Once you've completed the basic implementation, try these challenges:

1. **Performance Optimization**: Implement more efficient growth strategies
2. **Additional Methods**: Add `retain()`, `dedup()`, and `sort()` methods
3. **Specialized Iterators**: Implement `DoubleEndedIterator` and `ExactSizeIterator`
4. **Memory Efficiency**: Implement `shrink_to_fit()` to reduce memory usage
5. **Comparison Traits**: Implement `PartialEq` and `Eq` for vector comparison

## Resources

- [The Rust Programming Language - Generic Types](https://doc.rust-lang.org/book/ch10-01-syntax.html)
- [The Rust Programming Language - Traits](https://doc.rust-lang.org/book/ch10-02-traits.html)
- [The Rustonomicon - Vec Implementation](https://doc.rust-lang.org/nomicon/vec/vec.html)
- [Rust by Example - Generics](https://doc.rust-lang.org/rust-by-example/generics.html)