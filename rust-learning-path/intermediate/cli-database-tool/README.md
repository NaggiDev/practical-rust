# Command Line Database Tool

## Project Overview

Build a command-line database tool that allows users to create, read, update, and delete records in a simple file-based database. This project focuses on advanced error handling patterns, file I/O operations, and data persistence in Rust.

## Learning Objectives

By completing this project, you will learn:
- Advanced error handling with custom error types
- File I/O operations and data persistence
- JSON serialization and deserialization
- Command-line argument parsing
- Error propagation and conversion patterns
- Testing file operations and error conditions

## Prerequisites

Before starting this project, you should be familiar with:
- Basic Rust syntax and ownership concepts
- Structs, enums, and pattern matching
- Result and Option types
- Basic error handling with `?` operator
- Working with collections (Vec, HashMap)

## Project Structure

```
cli-database-tool/
├── README.md           # This file
├── src/
│   ├── main.rs         # Entry point and CLI interface
│   ├── database.rs     # Database operations and storage
│   ├── record.rs       # Record data structure
│   └── error.rs        # Custom error types
├── tests/
│   └── integration_tests.rs  # Integration tests
├── Cargo.toml          # Project dependencies
└── CONCEPTS.md         # Detailed concept explanations
```

## Step-by-Step Implementation Guide

### Step 1: Project Setup and Basic Structure

**Objective**: Set up the project structure and define the basic data types.

**Tasks**:
1. Initialize the Cargo project
2. Define the `Record` struct for database entries
3. Set up the basic CLI interface structure
4. Create placeholder modules for database operations

**Concepts Applied**: Project organization, module system, struct definitions

### Step 2: Custom Error Types

**Objective**: Implement a comprehensive error handling system using custom error types.

**Tasks**:
1. Define custom error types for different failure scenarios
2. Implement error conversion traits
3. Create helper functions for error creation
4. Add error display formatting

**Concepts Applied**: Custom error types, trait implementation, error conversion, Display trait

### Step 3: Database Storage Implementation

**Objective**: Implement file-based storage with JSON serialization.

**Tasks**:
1. Implement database initialization and file creation
2. Add JSON serialization/deserialization for records
3. Implement CRUD operations with proper error handling
4. Add file locking mechanisms for data integrity

**Concepts Applied**: File I/O, JSON handling, error propagation, resource management

### Step 4: Command Line Interface

**Objective**: Create a user-friendly CLI for database operations.

**Tasks**:
1. Parse command-line arguments
2. Implement interactive command processing
3. Add input validation and error reporting
4. Create help and usage information

**Concepts Applied**: Command-line parsing, user input handling, pattern matching

### Step 5: Testing and Validation

**Objective**: Implement comprehensive tests for all database operations.

**Tasks**:
1. Write unit tests for individual functions
2. Create integration tests for complete workflows
3. Test error conditions and edge cases
4. Add property-based testing for data integrity

**Concepts Applied**: Unit testing, integration testing, error testing, test organization

## Getting Started

1. **Clone or create the project directory**:
   ```bash
   mkdir cli-database-tool
   cd cli-database-tool
   ```

2. **Initialize the Cargo project**:
   ```bash
   cargo init
   ```

3. **Follow the step-by-step guide** starting with Step 1

4. **Run tests frequently**:
   ```bash
   cargo test
   ```

5. **Test the CLI manually**:
   ```bash
   cargo run -- --help
   ```

## Extension Challenges

Once you complete the basic implementation, try these extensions:

1. **Query System**: Add support for filtering and searching records
2. **Backup and Restore**: Implement database backup and restore functionality
3. **Multiple Tables**: Support for multiple tables/collections
4. **Indexing**: Add simple indexing for faster lookups
5. **Transactions**: Implement basic transaction support
6. **Network Interface**: Add a simple HTTP API interface

## Success Criteria

Your implementation should:
- ✅ Handle all error conditions gracefully
- ✅ Persist data reliably to disk
- ✅ Provide clear error messages to users
- ✅ Pass all unit and integration tests
- ✅ Follow Rust best practices for error handling
- ✅ Include comprehensive documentation

## Resources

- [Rust Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [Serde JSON Documentation](https://docs.serde.rs/serde_json/)
- [Clap CLI Parser](https://docs.rs/clap/latest/clap/)
- [Rust File I/O](https://doc.rust-lang.org/std/fs/index.html)