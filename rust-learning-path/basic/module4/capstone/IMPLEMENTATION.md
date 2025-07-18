# RustyNotes Implementation Guide

This guide provides step-by-step instructions for implementing the RustyNotes application. Follow these steps to build a complete command-line note-taking application that demonstrates all the concepts from the Basic Level of the Rust Learning Path.

## Step 1: Project Setup

1. Create a new Rust project:
   ```bash
   cargo new rusty_notes
   cd rusty_notes
   ```

2. Add the required dependencies to `Cargo.toml`:
   ```toml
   [package]
   name = "rusty_notes"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   chrono = "0.4"
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   uuid = { version = "1.0", features = ["v4"] }
   dirs = "4.0"
   ```

## Step 2: Define the Note Struct

Create a new file `src/note.rs` with the following content:

```rust
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

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
        let now = Utc::now();
        Self {
            id: Uuid::new_v4().to_string(),
            title,
            content,
            tags,
            category,
            created_at: now,
            updated_at: now,
        }
    }

    pub fn update(&mut self, title: Option<String>, content: Option<String>, 
                 tags: Option<Vec<String>>, category: Option<Option<String>>) {
        if let Some(title) = title {
            self.title = title;
        }
        
        if let Some(content) = content {
            self.content = content;
        }
        
        if let Some(tags) = tags {
            self.tags = tags;
        }
        
        if let Some(category) = category {
            self.category = category;
        }
        
        self.updated_at = Utc::now();
    }
}

impl fmt::Display for Note {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "ID: {}", self.id)?;
        writeln!(f, "Title: {}", self.title)?;
        writeln!(f, "Content: {}", self.content)?;
        writeln!(f, "Tags: {}", self.tags.join(", "))?;
        
        if let Some(category) = &self.category {
            writeln!(f, "Category: {}", category)?;
        } else {
            writeln!(f, "Category: None")?;
        }
        
        writeln!(f, "Created: {}", self.created_at)?;
        writeln!(f, "Updated: {}", self.updated_at)?;
        
        Ok(())
    }
}
```

## Step 3: Implement Error Handling

Create a new file `src/error.rs` with the following content:

```rust
use std::fmt;
use std::io;

#[derive(Debug)]
pub enum NoteError {
    Io(io::Error),
    NotFound(String),
    InvalidInput(String),
    Serialization(String),
}

impl fmt::Display for NoteError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            NoteError::Io(err) => write!(f, "I/O error: {}", err),
            NoteError::NotFound(id) => write!(f, "Note not found: {}", id),
            NoteError::InvalidInput(msg) => write!(f, "Invalid input: {}", msg),
            NoteError::Serialization(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for NoteError {}

impl From<io::Error> for NoteError {
    fn from(err: io::Error) -> Self {
        NoteError::Io(err)
    }
}

impl From<serde_json::Error> for NoteError {
    fn from(err: serde_json::Error) -> Self {
        NoteError::Serialization(err.to_string())
    }
}
```

## Step 4: Implement Storage

Create a new file `src/storage.rs` with the following content:

```rust
use crate::error::NoteError;
use crate::note::Note;
use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{self, Read, Write};
use std::path::{Path, PathBuf};

pub struct Storage {
    notes: HashMap<String, Note>,
    storage_path: PathBuf,
}

impl Storage {
    pub fn new(storage_path: PathBuf) -> Result<Self, NoteError> {
        let notes = if storage_path.exists() {
            let mut file = File::open(&storage_path)?;
            let mut contents = String::new();
            file.read_to_string(&mut contents)?;
            
            if contents.is_empty() {
                HashMap::new()
            } else {
                serde_json::from_str(&contents)?
            }
        } else {
            // Ensure the parent directory exists
            if let Some(parent) = storage_path.parent() {
                fs::create_dir_all(parent)?;
            }
            HashMap::new()
        };
        
        Ok(Self {
            notes,
            storage_path,
        })
    }
    
    pub fn add_note(&mut self, note: Note) -> Result<(), NoteError> {
        let id = note.id.clone();
        self.notes.insert(id, note);
        self.save()?;
        Ok(())
    }
    
    pub fn get_note(&self, id: &str) -> Option<&Note> {
        self.notes.get(id)
    }
    
    pub fn get_note_mut(&mut self, id: &str) -> Option<&mut Note> {
        self.notes.get_mut(id)
    }
    
    pub fn delete_note(&mut self, id: &str) -> Result<(), NoteError> {
        if self.notes.remove(id).is_none() {
            return Err(NoteError::NotFound(id.to_string()));
        }
        self.save()?;
        Ok(())
    }
    
    pub fn list_notes(&self) -> Vec<&Note> {
        self.notes.values().collect()
    }
    
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
    
    pub fn save(&self) -> Result<(), NoteError> {
        let json = serde_json::to_string_pretty(&self.notes)?;
        let mut file = File::create(&self.storage_path)?;
        file.write_all(json.as_bytes())?;
        Ok(())
    }
}
```

## Step 5: Implement Application Logic

Create a new file `src/app.rs` with the following content:

```rust
use crate::error::NoteError;
use crate::note::Note;
use crate::storage::Storage;
use std::path::PathBuf;

pub struct App {
    storage: Storage,
}

impl App {
    pub fn new(storage_path: Option<PathBuf>) -> Result<Self, NoteError> {
        let storage_path = match storage_path {
            Some(path) => path,
            None => {
                let mut path = dirs::data_dir().unwrap_or_else(|| PathBuf::from("."));
                path.push("rusty_notes");
                path.push("notes.json");
                path
            }
        };
        
        let storage = Storage::new(storage_path)?;
        
        Ok(Self {
            storage,
        })
    }
    
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
    
    pub fn get_note(&self, id: &str) -> Result<&Note, NoteError> {
        self.storage.get_note(id)
            .ok_or_else(|| NoteError::NotFound(id.to_string()))
    }
    
    pub fn update_note(&mut self, id: &str, title: Option<String>, content: Option<String>,
                      tags: Option<Vec<String>>, category: Option<Option<String>>) -> Result<(), NoteError> {
        let note = self.storage.get_note_mut(id)
            .ok_or_else(|| NoteError::NotFound(id.to_string()))?;
            
        note.update(title, content, tags, category);
        self.storage.save()?;
        
        Ok(())
    }
    
    pub fn delete_note(&mut self, id: &str) -> Result<(), NoteError> {
        self.storage.delete_note(id)
    }
    
    pub fn list_notes(&self) -> Vec<&Note> {
        self.storage.list_notes()
    }
    
    pub fn search_notes(&self, query: &str) -> Vec<&Note> {
        self.storage.search_notes(query)
    }
}
```

## Step 6: Implement the Command-Line Interface

Update `src/main.rs` with the following content:

```rust
mod app;
mod error;
mod note;
mod storage;

use app::App;
use error::NoteError;
use std::io::{self, Write};

fn main() -> Result<(), NoteError> {
    println!("Welcome to RustyNotes!");
    println!("Type 'help' to see available commands.");
    
    let mut app = App::new(None)?;
    
    loop {
        print!("> ");
        io::stdout().flush()?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        
        let input = input.trim();
        if input.is_empty() {
            continue;
        }
        
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        
        match command {
            "add" => {
                println!("Enter note title:");
                let title = read_line()?;
                
                println!("Enter note content:");
                let content = read_line()?;
                
                println!("Enter tags (comma-separated):");
                let tags_input = read_line()?;
                let tags = tags_input
                    .split(',')
                    .map(|s| s.trim().to_string())
                    .filter(|s| !s.is_empty())
                    .collect();
                
                println!("Enter category (optional):");
                let category_input = read_line()?;
                let category = if category_input.is_empty() {
                    None
                } else {
                    Some(category_input)
                };
                
                match app.add_note(title, content, tags, category) {
                    Ok(id) => println!("Note added with ID: {}", id),
                    Err(e) => println!("Error: {}", e),
                }
            },
            "list" => {
                let notes = app.list_notes();
                if notes.is_empty() {
                    println!("No notes found.");
                } else {
                    println!("Found {} notes:", notes.len());
                    for note in notes {
                        println!("ID: {} | Title: {} | Tags: {}", 
                                note.id, note.title, note.tags.join(", "));
                    }
                }
            },
            "view" => {
                if parts.len() < 2 {
                    println!("Usage: view <note_id>");
                    continue;
                }
                
                let id = parts[1];
                match app.get_note(id) {
                    Ok(note) => println!("{}", note),
                    Err(e) => println!("Error: {}", e),
                }
            },
            "update" => {
                if parts.len() < 2 {
                    println!("Usage: update <note_id>");
                    continue;
                }
                
                let id = parts[1];
                
                // Check if the note exists
                if app.get_note(id).is_err() {
                    println!("Note not found: {}", id);
                    continue;
                }
                
                println!("Enter new title (leave empty to keep current):");
                let title_input = read_line()?;
                let title = if title_input.is_empty() { None } else { Some(title_input) };
                
                println!("Enter new content (leave empty to keep current):");
                let content_input = read_line()?;
                let content = if content_input.is_empty() { None } else { Some(content_input) };
                
                println!("Enter new tags (comma-separated, leave empty to keep current):");
                let tags_input = read_line()?;
                let tags = if tags_input.is_empty() {
                    None
                } else {
                    Some(tags_input
                        .split(',')
                        .map(|s| s.trim().to_string())
                        .filter(|s| !s.is_empty())
                        .collect())
                };
                
                println!("Enter new category (leave empty to keep current, type 'none' to remove):");
                let category_input = read_line()?;
                let category = if category_input.is_empty() {
                    None
                } else if category_input.to_lowercase() == "none" {
                    Some(None)
                } else {
                    Some(Some(category_input))
                };
                
                match app.update_note(id, title, content, tags, category) {
                    Ok(_) => println!("Note updated successfully."),
                    Err(e) => println!("Error: {}", e),
                }
            },
            "delete" => {
                if parts.len() < 2 {
                    println!("Usage: delete <note_id>");
                    continue;
                }
                
                let id = parts[1];
                match app.delete_note(id) {
                    Ok(_) => println!("Note deleted successfully."),
                    Err(e) => println!("Error: {}", e),
                }
            },
            "search" => {
                if parts.len() < 2 {
                    println!("Usage: search <query>");
                    continue;
                }
                
                let query = &input[7..]; // Skip "search "
                let notes = app.search_notes(query);
                
                if notes.is_empty() {
                    println!("No notes found matching '{}'.", query);
                } else {
                    println!("Found {} notes matching '{}':", notes.len(), query);
                    for note in notes {
                        println!("ID: {} | Title: {} | Tags: {}", 
                                note.id, note.title, note.tags.join(", "));
                    }
                }
            },
            "help" => {
                println!("Available commands:");
                println!("  add                  - Add a new note");
                println!("  list                 - List all notes");
                println!("  view <note_id>       - View a specific note");
                println!("  update <note_id>     - Update a note");
                println!("  delete <note_id>     - Delete a note");
                println!("  search <query>       - Search notes");
                println!("  help                 - Show this help message");
                println!("  exit                 - Exit the application");
            },
            "exit" => {
                println!("Goodbye!");
                break;
            },
            _ => {
                println!("Unknown command. Type 'help' to see available commands.");
            }
        }
    }
    
    Ok(())
}

fn read_line() -> Result<String, NoteError> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
```

## Step 7: Implement Tests

Create a new file `tests/note_tests.rs` with the following content:

```rust
#[cfg(test)]
mod tests {
    use rusty_notes::note::Note;
    
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
    
    #[test]
    fn test_update_note() {
        let mut note = Note::new(
            "Original Title".to_string(),
            "Original Content".to_string(),
            vec!["original".to_string()],
            Some("Original".to_string())
        );
        
        let original_id = note.id.clone();
        let original_created_at = note.created_at;
        
        // Update with new values
        note.update(
            Some("Updated Title".to_string()),
            Some("Updated Content".to_string()),
            Some(vec!["updated".to_string()]),
            Some(Some("Updated".to_string()))
        );
        
        // Check that values were updated
        assert_eq!(note.title, "Updated Title");
        assert_eq!(note.content, "Updated Content");
        assert_eq!(note.tags, vec!["updated".to_string()]);
        assert_eq!(note.category, Some("Updated".to_string()));
        
        // Check that id and created_at weren't changed
        assert_eq!(note.id, original_id);
        assert_eq!(note.created_at, original_created_at);
        
        // Check that updated_at was changed
        assert!(note.updated_at > original_created_at);
    }
    
    #[test]
    fn test_partial_update() {
        let mut note = Note::new(
            "Original Title".to_string(),
            "Original Content".to_string(),
            vec!["original".to_string()],
            Some("Original".to_string())
        );
        
        // Update only the title
        note.update(
            Some("Updated Title".to_string()),
            None,
            None,
            None
        );
        
        // Check that only the title was updated
        assert_eq!(note.title, "Updated Title");
        assert_eq!(note.content, "Original Content");
        assert_eq!(note.tags, vec!["original".to_string()]);
        assert_eq!(note.category, Some("Original".to_string()));
    }
    
    #[test]
    fn test_remove_category() {
        let mut note = Note::new(
            "Title".to_string(),
            "Content".to_string(),
            vec!["tag".to_string()],
            Some("Category".to_string())
        );
        
        // Remove the category
        note.update(None, None, None, Some(None));
        
        // Check that the category was removed
        assert_eq!(note.category, None);
    }
}
```

Create a new file `tests/app_tests.rs` with the following content:

```rust
#[cfg(test)]
mod tests {
    use rusty_notes::app::App;
    use rusty_notes::error::NoteError;
    use std::path::PathBuf;
    use std::fs;
    
    fn setup() -> (App, PathBuf) {
        let test_dir = PathBuf::from("test_data");
        let storage_path = test_dir.join("test_notes.json");
        
        // Clean up from previous tests
        let _ = fs::remove_file(&storage_path);
        let _ = fs::create_dir_all(&test_dir);
        
        let app = App::new(Some(storage_path.clone())).unwrap();
        (app, storage_path)
    }
    
    fn cleanup(path: PathBuf) {
        let _ = fs::remove_file(path);
    }
    
    #[test]
    fn test_add_note() {
        let (mut app, path) = setup();
        
        let id = app.add_note(
            "Test Note".to_string(),
            "This is a test note.".to_string(),
            vec!["test".to_string()],
            Some("Testing".to_string())
        ).unwrap();
        
        let note = app.get_note(&id).unwrap();
        assert_eq!(note.title, "Test Note");
        
        cleanup(path);
    }
    
    #[test]
    fn test_empty_title_error() {
        let (mut app, path) = setup();
        
        let result = app.add_note(
            "".to_string(),
            "Content".to_string(),
            vec![],
            None
        );
        
        assert!(matches!(result, Err(NoteError::InvalidInput(_))));
        
        cleanup(path);
    }
    
    #[test]
    fn test_update_note() {
        let (mut app, path) = setup();
        
        let id = app.add_note(
            "Original".to_string(),
            "Content".to_string(),
            vec![],
            None
        ).unwrap();
        
        app.update_note(
            &id,
            Some("Updated".to_string()),
            None,
            None,
            None
        ).unwrap();
        
        let note = app.get_note(&id).unwrap();
        assert_eq!(note.title, "Updated");
        
        cleanup(path);
    }
    
    #[test]
    fn test_delete_note() {
        let (mut app, path) = setup();
        
        let id = app.add_note(
            "To Delete".to_string(),
            "This note will be deleted.".to_string(),
            vec![],
            None
        ).unwrap();
        
        app.delete_note(&id).unwrap();
        
        let result = app.get_note(&id);
        assert!(matches!(result, Err(NoteError::NotFound(_))));
        
        cleanup(path);
    }
    
    #[test]
    fn test_search_notes() {
        let (mut app, path) = setup();
        
        app.add_note(
            "Apple Note".to_string(),
            "This note is about apples.".to_string(),
            vec!["fruit".to_string()],
            Some("Food".to_string())
        ).unwrap();
        
        app.add_note(
            "Banana Note".to_string(),
            "This note is about bananas.".to_string(),
            vec!["fruit".to_string()],
            Some("Food".to_string())
        ).unwrap();
        
        app.add_note(
            "Car Note".to_string(),
            "This note is about cars.".to_string(),
            vec!["vehicle".to_string()],
            Some("Transport".to_string())
        ).unwrap();
        
        // Search by title
        let results = app.search_notes("Apple");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Apple Note");
        
        // Search by content
        let results = app.search_notes("bananas");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Banana Note");
        
        // Search by tag
        let results = app.search_notes("fruit");
        assert_eq!(results.len(), 2);
        
        // Search by category
        let results = app.search_notes("Transport");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].title, "Car Note");
        
        cleanup(path);
    }
}
```

## Step 8: Update the Project for Library Support

To make the tests work, we need to expose our modules as a library. Update `src/lib.rs` with:

```rust
pub mod app;
pub mod error;
pub mod note;
pub mod storage;
```

## Step 9: Build and Run the Application

1. Build the application:
   ```bash
   cargo build
   ```

2. Run the application:
   ```bash
   cargo run
   ```

3. Run the tests:
   ```bash
   cargo test
   ```

## Extension Challenges

Once you've completed the basic implementation, try these extensions:

1. **Markdown Support**: Add support for formatting notes with Markdown.
   - Add a dependency on a Markdown parsing crate like `pulldown-cmark`
   - Implement a function to render Markdown content
   - Add a command to view notes with formatted Markdown

2. **Note Encryption**: Implement encryption for sensitive notes.
   - Add a dependency on an encryption crate like `aes-gcm`
   - Add a password option when creating notes
   - Encrypt the content of password-protected notes
   - Require the password to view encrypted notes

3. **Backup and Restore**: Create a simple backup and restore system.
   - Implement commands to export notes to a backup file
   - Implement a command to import notes from a backup
   - Add options to merge or replace existing notes during import

4. **Text User Interface**: Add a simple TUI for a more interactive experience.
   - Add a dependency on a TUI crate like `cursive` or `tui-rs`
   - Implement a menu-based interface
   - Add keyboard shortcuts for common operations
   - Create a more visual note browsing experience

## Concepts Applied

This project applies all the concepts from the Basic Level of the Rust Learning Path:

- **Variables and Data Types**: Using various Rust types throughout the application
- **Functions and Methods**: Organizing code into functions and implementing methods on structs
- **Control Flow**: Using if/else, match, and loops for program logic
- **Error Handling**: Using Result and Option types, custom error types
- **Structs and Enums**: Defining custom data structures
- **Collections**: Using Vec and HashMap to store and manipulate data
- **Ownership and Borrowing**: Managing ownership throughout the application
- **File I/O**: Reading from and writing to files
- **External Crates**: Using dependencies from crates.io
- **Testing**: Writing unit and integration tests