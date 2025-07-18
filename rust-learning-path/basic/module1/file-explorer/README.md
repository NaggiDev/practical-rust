# File System Explorer

## Project Overview

This project is part of the Basic level in the Rust Learning Path. It focuses on building a simple command-line file system explorer that allows users to navigate directories, list files, and perform basic file operations.

## Learning Objectives

By completing this project, you will:
- Learn how to interact with the file system using Rust's standard library
- Understand how to handle paths and directories in a cross-platform way
- Implement error handling for file operations
- Practice working with command-line arguments
- Gain experience with Rust's ownership model in the context of file I/O

## Prerequisites

Before starting this project, you should:
- Have completed the Command Line Calculator project
- Be familiar with basic Rust syntax, variables, and control flow
- Understand basic error handling with Result and Option types

## Project Structure

```
file-explorer/
├── README.md           # Project overview and instructions
├── src/                # Source code directory
│   └── main.rs         # Entry point for the application
├── tests/              # Test directory
│   └── integration_tests.rs  # Integration tests
├── Cargo.toml          # Project dependencies and metadata
└── CONCEPTS.md         # Detailed explanation of Rust concepts used
```

## Step-by-Step Instructions

### Step 1: Setting Up the Project

First, let's set up the project structure and define the main functionality:

```rust
use std::env;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

fn main() -> io::Result<()> {
    // Get the current directory as the starting point
    let current_dir = env::current_dir()?;
    
    // Start the explorer with the current directory
    explore_directory(current_dir)?;
    
    Ok(())
}

fn explore_directory(path: PathBuf) -> io::Result<()> {
    println!("Exploring directory: {}", path.display());
    
    // TODO: Implement directory exploration functionality
    
    Ok(())
}
```

**Concepts Applied:**
- `std::fs`: Rust's file system module for interacting with files and directories
- `std::path`: Module for working with file paths in a cross-platform way
- `io::Result`: Return type for I/O operations that might fail

### Step 2: Listing Directory Contents

Now, let's implement the functionality to list the contents of a directory:

```rust
fn list_directory_contents(path: &Path) -> io::Result<()> {
    let entries = fs::read_dir(path)?;
    
    println!("\nContents of {}:", path.display());
    println!("{:<15} {:<10} {}", "Name", "Type", "Size (bytes)");
    println!("{:-<40}", "");
    
    // TODO: Iterate through directory entries and display information
    
    Ok(())
}
```

**Concepts Applied:**
- `fs::read_dir`: Function to read the contents of a directory
- Error propagation with the `?` operator
- String formatting with `println!` macro

### Step 3: Implementing Navigation Commands

Next, let's implement commands to navigate through the file system:

```rust
fn handle_command(command: &str, current_dir: &mut PathBuf) -> io::Result<bool> {
    // TODO: Implement command handling for navigation
    // Commands like:
    // - "ls" to list contents
    // - "cd <dir>" to change directory
    // - "pwd" to print current directory
    // - "exit" to quit the program
    
    Ok(true) // Return false to exit the program
}
```

**Concepts Applied:**
- String parsing and command handling
- Mutable references for updating the current directory
- Control flow with match expressions

### Step 4: Adding File Operations

Finally, let's add some basic file operations:

```rust
fn file_operation(operation: &str, args: &[&str], current_dir: &PathBuf) -> io::Result<()> {
    // TODO: Implement file operations like:
    // - "info <file>" to display file information
    // - "cat <file>" to display file contents
    // - "mkdir <dir>" to create a directory
    
    Ok(())
}
```

**Concepts Applied:**
- Working with file metadata
- Reading file contents
- Creating directories
- Error handling for different file operations

## Testing Your Implementation

This project includes tests to validate your implementation:

```bash
cargo test
```

The tests will verify that:
- The program can correctly list directory contents
- Navigation commands work as expected
- File operations handle errors appropriately
- The program works with different path formats

## Extension Challenges

Once you've completed the basic implementation, try these extension challenges:

1. **File Search**: Add a command to search for files by name or extension
2. **File Copying**: Implement commands to copy files between directories
3. **Recursive Directory Listing**: Add an option to list directories recursively
4. **File Permissions**: Display and modify file permissions (platform-specific)

## Next Steps

After completing this project, you should move on to:
- The Simple Text Processor project in Module 2
- Explore more advanced file I/O operations in Rust

## Resources

- [Rust Standard Library - std::fs](https://doc.rust-lang.org/std/fs/index.html)
- [Rust Standard Library - std::path](https://doc.rust-lang.org/std/path/index.html)
- [Rust By Example - File I/O](https://doc.rust-lang.org/rust-by-example/std_misc/file.html)
- [The Rust Programming Language - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)