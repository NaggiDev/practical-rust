# Rust Concepts in the Simple Text Processor

This document explains the key Rust concepts used in the Simple Text Processor project.

## 1. String Manipulation

### String Types
Rust has two main string types:
- `String`: A growable, heap-allocated data structure
- `&str`: A string slice, which is a reference to a sequence of UTF-8 bytes

```rust
// String example
let mut s = String::from("hello");
s.push_str(", world!");

// String slice example
let hello = &s[0..5];
```

### Common String Operations
The text processor uses several string manipulation methods:

```rust
// Converting case
let uppercase = text.to_uppercase();
let lowercase = text.to_lowercase();

// Splitting text
let lines: Vec<&str> = text.lines().collect();
let words: Vec<&str> = text.split_whitespace().collect();

// Replacing text
let replaced = text.replace("old", "new");

// Trimming whitespace
let trimmed = text.trim();
```

## 2. File I/O

### Reading Files
Rust provides several ways to read files:

```rust
// Reading an entire file into a string
let contents = fs::read_to_string("file.txt")?;

// Reading a file with more control
let mut file = File::open("file.txt")?;
let mut contents = String::new();
file.read_to_string(&mut contents)?;
```

### Writing Files
Similarly, there are multiple ways to write to files:

```rust
// Writing a string to a file
fs::write("file.txt", "Hello, world!")?;

// Writing with more control
let mut file = File::create("file.txt")?;
file.write_all("Hello, world!".as_bytes())?;
```

## 3. Error Handling

### Result Type
The `Result<T, E>` type is used for operations that can fail:

```rust
fn read_file(path: &str) -> Result<String, io::Error> {
    fs::read_to_string(path)
}
```

### Error Propagation
The `?` operator is used to propagate errors up the call stack:

```rust
fn process_file(path: &str) -> Result<(), io::Error> {
    let contents = fs::read_to_string(path)?;
    // Process contents...
    Ok(())
}
```

### Custom Error Types
You can define custom error types to represent application-specific errors:

```rust
enum TextProcessorError {
    IoError(io::Error),
    InvalidOperation(String),
}

impl From<io::Error> for TextProcessorError {
    fn from(error: io::Error) -> Self {
        TextProcessorError::IoError(error)
    }
}
```

## 4. Command-Line Argument Parsing

Rust provides access to command-line arguments through `std::env::args()`:

```rust
let args: Vec<String> = std::env::args().collect();
if args.len() > 1 {
    let command = &args[1];
    // Process command...
}
```

## 5. Traits and Generics

### Traits
Traits define shared behavior across types:

```rust
trait TextTransformation {
    fn transform(&self, input: &str) -> String;
}

struct Uppercase;
impl TextTransformation for Uppercase {
    fn transform(&self, input: &str) -> String {
        input.to_uppercase()
    }
}
```

### Generics
Generics allow for flexible, reusable code:

```rust
fn apply_transformation<T: TextTransformation>(transformer: &T, input: &str) -> String {
    transformer.transform(input)
}
```

## 6. Closures

Closures are anonymous functions that can capture their environment:

```rust
let to_uppercase = |s: &str| s.to_uppercase();
let uppercase_text = to_uppercase(&text);
```

## 7. Iterators

Iterators provide a way to process sequences of items:

```rust
// Count words in text
let word_count = text.split_whitespace().count();

// Filter lines containing a pattern
let matching_lines: Vec<&str> = text.lines()
    .filter(|line| line.contains("pattern"))
    .collect();

// Sort lines
let mut lines: Vec<&str> = text.lines().collect();
lines.sort();
```

## 8. Resource Management

Rust's ownership system ensures resources are properly managed:

```rust
// File is automatically closed when it goes out of scope
{
    let file = File::open("file.txt")?;
    // Work with file...
} // File is closed here
```

## 9. Standard Library Usage

The project uses several components from Rust's standard library:
- `std::fs` for file system operations
- `std::io` for input/output operations
- `std::path` for working with file paths
- `std::env` for accessing environment variables and command-line arguments