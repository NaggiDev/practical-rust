# Simple Text Processor

## Project Overview

In this project, you will build a command-line text processing tool that can perform various transformations on text files. This project will help you practice Rust's string manipulation capabilities, file I/O operations, and error handling techniques.

## Learning Objectives

By completing this project, you will:

- Work with files in Rust (reading and writing)
- Manipulate strings using Rust's standard library
- Implement error handling for file operations
- Create modular code with functions
- Practice using Result and Option types

## Project Requirements

Your text processor should:

1. Read text from files or standard input
2. Apply one or more transformations to the text
3. Output the transformed text to a file or standard output
4. Handle errors gracefully (file not found, permission issues, etc.)
5. Support multiple transformation operations

## Getting Started

1. Create a new Rust project:
   ```bash
   cargo new text-processor
   cd text-processor
   ```

2. Open `src/main.rs` and replace its contents with the starter code provided below.

3. Follow the step-by-step instructions in the code comments to complete the implementation.

## Starter Code

```rust
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;

fn main() {
    println!("Simple Text Processor");
    println!("---------------------");
    
    // TODO: Parse command-line arguments
    
    // TODO: Read input text (from file or stdin)
    
    // TODO: Apply transformations
    
    // TODO: Write output (to file or stdout)
}

// TODO: Implement a function to read text from a file or stdin

// TODO: Implement a function to write text to a file or stdout

// TODO: Implement transformation functions (uppercase, lowercase, etc.)

// TODO: Implement error handling
```

## Step-by-Step Instructions

1. **Parse command-line arguments**
   - Implement basic argument parsing to determine:
     - Input source (file path or stdin)
     - Output destination (file path or stdout)
     - Transformation operations to apply

2. **Read input text**
   - Create a function that can read text from either a file or standard input
   - Handle potential errors (file not found, permission issues)
   - Return the text as a String

3. **Implement transformation functions**
   - Create functions for various text transformations:
     - Convert to uppercase
     - Convert to lowercase
     - Count words, lines, and characters
     - Replace specific patterns
     - Remove duplicate lines
     - Sort lines alphabetically

4. **Apply transformations**
   - Apply the selected transformations to the input text
   - Chain multiple transformations if requested

5. **Write output**
   - Create a function that can write text to either a file or standard output
   - Handle potential errors (permission issues, disk full)

6. **Implement error handling**
   - Use Result and Option types to handle errors
   - Provide meaningful error messages to the user
   - Ensure resources are properly cleaned up

## Testing Your Implementation

Test your text processor with various inputs and transformations:

- Basic transformations: uppercase, lowercase, line count
- File operations: reading from files, writing to files
- Error cases: file not found, permission denied
- Complex transformations: sorting, replacing patterns

## Extension Challenges

After completing the basic implementation, try these extensions:

1. Add support for regular expressions in search and replace operations
2. Implement more advanced transformations (word frequency count, text statistics)
3. Add support for processing multiple files at once
4. Create a configuration file format for storing common transformation sequences

## Concepts Applied

- File I/O operations
- String manipulation
- Error handling with Result
- Command-line argument parsing
- Function composition
- Resource management

## Resources

- [The Rust Book - Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)
- [The Rust Book - Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [Rust By Example - File I/O](https://doc.rust-lang.org/rust-by-example/std_misc/file.html)
- [Rust Documentation - std::fs](https://doc.rust-lang.org/std/fs/index.html)
- [Rust Documentation - std::io](https://doc.rust-lang.org/std/io/index.html)