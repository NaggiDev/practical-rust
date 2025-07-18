# Library Management System

## Project Overview

This project is part of the Intermediate level in the Rust Learning Path. It focuses on building a command-line library management system that demonstrates advanced ownership concepts, borrowing, and lifetime management in Rust.

## Learning Objectives

By completing this project, you will:
- Understand how ownership and borrowing work in complex data structures
- Learn to manage data with different lifetimes
- Implement proper error handling for a real-world application
- Design and implement a system with multiple interacting components
- Practice using Rust's reference system to safely share data

## Prerequisites

Before starting this project, you should:
- Have completed the Basic level projects
- Be familiar with Rust fundamentals (variables, control flow, functions)
- Understand basic ownership concepts
- Have experience with structs, enums, and error handling

## Project Structure

```
library-management-system/
├── README.md           # Project overview and instructions
├── src/                # Source code directory
│   ├── main.rs         # Entry point for the application
│   ├── lib.rs          # Library code
│   ├── models.rs       # Data models for library items
│   └── utils.rs        # Utility functions
├── tests/              # Test directory
│   └── integration_tests.rs  # Integration tests
├── Cargo.toml          # Project dependencies and metadata
└── CONCEPTS.md         # Detailed explanation of Rust concepts used
```

## Step-by-Step Instructions

### Step 1: Define the Data Models

In this step, you'll create the core data structures for the library management system.

1. Create a `models.rs` file with the following structures:
   - `Book`: Represents a book with title, author, ISBN, and availability status
   - `User`: Represents a library user with name, ID, and borrowed items
   - `Library`: The main structure that owns all books and manages users

```rust
// TODO: Implement the Book struct with appropriate fields
// TODO: Implement the User struct with appropriate fields
// TODO: Implement the Library struct that will own the collection of books and users
```

**Concepts Applied:**
- Ownership: The Library owns all books and manages their lifecycle
- Structs: Organizing related data into meaningful structures
- Option type: Managing the presence or absence of values (like borrowed status)

### Step 2: Implement Book Management Functions

In this step, you'll implement functions to manage books in the library.

```rust
// TODO: Implement functions to add books to the library
// TODO: Implement functions to remove books from the library
// TODO: Implement functions to search for books by various criteria
```

**Concepts Applied:**
- Methods: Implementing functionality tied to specific types
- Borrowing: Using references to access data without taking ownership
- Collections: Managing groups of items efficiently

### Step 3: Implement User Management Functions

In this step, you'll implement functions to manage library users.

```rust
// TODO: Implement functions to register new users
// TODO: Implement functions to remove users
// TODO: Implement functions to update user information
```

**Concepts Applied:**
- Mutable borrowing: Modifying data through references
- Error handling: Managing invalid operations
- Ownership transfer: Moving data between structures when needed

### Step 4: Implement Borrowing and Returning System

In this step, you'll implement the core functionality of borrowing and returning books.

```rust
// TODO: Implement function for users to borrow books
// TODO: Implement function for users to return books
// TODO: Implement validation to prevent invalid operations
```

**Concepts Applied:**
- Shared vs. mutable references: Understanding Rust's borrowing rules
- Lifetimes: Managing references with different lifespans
- Reference counting: Using Rc/Arc when appropriate for shared ownership

### Step 5: Implement the Command Line Interface

In this step, you'll create a user-friendly command-line interface.

```rust
// TODO: Implement a menu system for user interaction
// TODO: Implement command parsing for different operations
// TODO: Implement display functions for showing library status
```

**Concepts Applied:**
- Input/Output: Reading from and writing to the console
- Error handling: Providing user-friendly error messages
- String manipulation: Processing and formatting text

## Testing Your Implementation

This project includes tests to validate your implementation:

```bash
cargo test
```

The tests will verify that:
- Books can be added to and removed from the library
- Users can be registered and removed
- Books can be borrowed and returned correctly
- Invalid operations (like borrowing an already borrowed book) are properly handled
- The system maintains data consistency throughout operations

## Extension Challenges

Once you've completed the basic implementation, try these extension challenges:

1. **Implement Due Dates**: Add a system for tracking when books are due to be returned
2. **Multiple Copies**: Allow the library to have multiple copies of the same book
3. **Reservations**: Implement a system for users to reserve books that are currently borrowed
4. **Persistent Storage**: Save the library state to a file and load it when the program starts
5. **Categories and Tags**: Add a system for categorizing books and searching by category

## Next Steps

After completing this project, you should move on to:
- The Multi-threaded Web Scraper project to learn about concurrency
- Explore more advanced ownership patterns in the Rust Book
- Practice implementing custom traits for your library types

## Resources

- [Rust Book Chapter on Ownership](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Rust Book Chapter on Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [Rust Book Chapter on Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Rust By Example: Lifetimes](https://doc.rust-lang.org/rust-by-example/scope/lifetime.html)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)