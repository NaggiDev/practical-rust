# Rust Concepts Applied in RustyNotes

This document outlines the key Rust concepts applied in the RustyNotes capstone project.

## Basic Syntax and Control Flow

- **Variables and Mutability**: Using `let` and `mut` for variable declarations
- **Functions**: Defining and calling functions with parameters and return values
- **Control Flow**: Using `if`, `else`, `match`, and loops
- **String Manipulation**: Working with String and &str types

Example from the project:
```rust
match command {
    "add" => {
        // Command handling code
    },
    "list" => {
        // Command handling code
    },
    // Other commands
    _ => {
        println!("Unknown command. Type 'help' to see available commands.");
    }
}
```

## Ownership and Borrowing

- **Ownership Rules**: Understanding who owns data and when it's dropped
- **Borrowing**: Using references (`&` and `&mut`) to access data without taking ownership
- **Lifetimes**: Implicit lifetimes in function signatures

Example from the project:
```rust
pub fn get_note(&self, id: &str) -> Option<&Note> {
    self.notes.get(id)
}

pub fn get_note_mut(&mut self, id: &str) -> Option<&mut Note> {
    self.notes.get_mut(id)
}
```

## Structs and Methods

- **Struct Definition**: Creating custom data types with fields
- **Method Implementation**: Adding behavior to structs with `impl` blocks
- **Associated Functions**: Functions associated with a type (like `new()`)

Example from the project:
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: String,
    pub title: String,
    pub content: String,
    pub tags: Vec<String>,
    pub category: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Note {
    pub fn new(title: String, content: String, tags: Vec<String>, category: Option<String>) -> Self {
        // Implementation
    }
    
    pub fn update(&mut self, title: Option<String>, content: Option<String>, 
                 tags: Option<Vec<String>>, category: Option<Option<String>>) {
        // Implementation
    }
}
```

## Enums and Pattern Matching

- **Enum Definition**: Creating custom types with variants
- **Pattern Matching**: Using `match` to handle different enum variants
- **Option and Result**: Working with Rust's built-in enums for error handling

Example from the project:
```rust
#[derive(Debug)]
pub enum NoteError {
    Io(io::Error),
    NotFound(String),
    InvalidInput(String),
    Serialization(String),
}

// Using match with Option
match app.get_note(id) {
    Ok(note) => println!("{}", note),
    Err(e) => println!("Error: {}", e),
}
```

## Collections

- **Vectors**: Using `Vec<T>` for dynamic arrays
- **HashMaps**: Using `HashMap<K, V>` for key-value storage
- **Iterators**: Using iterator methods like `filter`, `map`, and `collect`

Example from the project:
```rust
pub fn search_notes(&self, query: &str) -> Vec<&Note> {
    let query = query.to_lowercase();
    self.notes.values()
        .filter(|note| {
            note.title.to_lowercase().contains(&query) || 
            note.content.to_lowercase().contains(&query) ||
            note.tags.iter().any(|tag| tag.to_lowercase().contains(&query)) ||
            note.category.as_ref().map_or(false, |cat| cat.to_lowercase().contains(&query))
        })
        .collect()
}
```

## Error Handling

- **Result Type**: Using `Result<T, E>` for operations that might fail
- **Error Propagation**: Using the `?` operator to propagate errors
- **Custom Error Types**: Creating application-specific error types

Example from the project:
```rust
pub fn add_note(&mut self, title: String, content: String, 
               tags: Vec<String>, category: Option<String>) -> Result<String, NoteError> {
    if title.is_empty() {
        return Err(NoteError::InvalidInput("Title cannot be empty".to_string()));
    }
    
    let note = Note::new(title, content, tags, category);
    let id = note.id.clone();
    self.storage.add_note(note)?;
    
    Ok(id)
}
```

## Traits and Trait Implementations

- **Trait Implementation**: Implementing traits like `Display` and `Error`
- **Derive Attributes**: Using `#[derive(...)]` to automatically implement traits

Example from the project:
```rust
impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Title: {}\n", self.title)?;
        // More formatting code
    }
}

impl std::error::Error for NoteError {}

impl From<io::Error> for NoteError {
    fn from(err: io::Error) -> Self {
        NoteError::Io(err)
    }
}
```

## File I/O

- **File Operations**: Reading from and writing to files
- **Path Handling**: Working with file paths in a cross-platform way
- **Serialization**: Converting between Rust data structures and JSON

Example from the project:
```rust
pub fn save(&self) -> io::Result<()> {
    let json = serde_json::to_string_pretty(&self.notes)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;
        
    let mut file = File::create(&self.storage_path)?;
    file.write_all(json.as_bytes())?;
    
    Ok(())
}
```

## Testing

- **Unit Tests**: Testing individual components in isolation
- **Integration Tests**: Testing how components work together
- **Test Organization**: Structuring tests in modules and separate files

Example from the project:
```rust
#[test]
fn test_create_note() {
    let title = "Test Note".to_string();
    let content = "This is a test note.".to_string();
    let tags = vec!["test".to_string(), "example".to_string()];
    let category = Some("Testing".to_string());
    
    let note = Note::new(title.clone(), content.clone(), tags.clone(), category.clone());
    
    assert_eq!(note.title, title);
    assert_eq!(note.content, content);
    assert_eq!(note.tags, tags);
    assert_eq!(note.category, category);
    assert!(!note.id.is_empty());
}
```

## External Crates and Dependencies

- **Using External Libraries**: Adding and using dependencies from crates.io
- **Feature Flags**: Enabling specific features of dependencies
- **Cargo.toml**: Managing project metadata and dependencies

Example from the project:
```toml
[dependencies]
chrono = "0.4"
serde = { version = "1.0", features = ["derive"] }
serde_json = "1.0"
uuid = { version = "1.0", features = ["v4"] }
dirs = "4.0"
```