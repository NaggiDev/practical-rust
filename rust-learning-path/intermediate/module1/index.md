# Module 1: Advanced Ownership

Welcome to Module 1 of the Intermediate Level! This module takes a deep dive into Rust's ownership system, exploring borrowing, lifetimes, and reference counting.

## Learning Objectives

By completing this module, you will:

- Master Rust's borrowing rules and lifetime system
- Understand when and how to use reference counting
- Learn to work with complex ownership scenarios
- Implement data structures with explicit lifetimes
- Use smart pointers effectively

## Concepts Covered

- Lifetime annotations and elision
- Complex borrowing patterns
- Smart pointers (Box, Rc, Arc)
- Interior mutability with RefCell and Mutex
- The Deref trait
- Weak references
- Memory leaks and prevention

## Projects

### [Library Management System](library-system/README.md)

Build a library management system that demonstrates complex ownership relationships between books, patrons, and loans.

**Skills practiced:**
- Modeling complex relationships between entities
- Managing shared ownership with reference counting
- Implementing borrowing rules that mirror real-world constraints
- Working with collections of references
- Handling cyclic references
- Implementing clean error handling

## Resources

- [The Rust Book - Advanced Features](https://doc.rust-lang.org/book/ch19-00-advanced-features.html)
- [The Rust Book - Smart Pointers](https://doc.rust-lang.org/book/ch15-00-smart-pointers.html)
- [Rust By Example - Scoping Rules](https://doc.rust-lang.org/rust-by-example/scope.html)

## Next Steps

After completing this module, proceed to [Module 2: Traits and Generics](../module2/index.md) to learn how to create flexible and reusable code in Rust.