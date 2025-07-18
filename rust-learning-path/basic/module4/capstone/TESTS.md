# RustyNotes Test Suite

This document outlines the comprehensive test suite for the RustyNotes application. The tests are designed to validate all aspects of the application's functionality and ensure that it meets the requirements.

## Test Organization

The test suite is organized into three main categories:

1. **Unit Tests**: Testing individual components in isolation
2. **Integration Tests**: Testing how components work together
3. **End-to-End Tests**: Testing the application as a whole

## Unit Tests

### Note Tests (`tests/note_tests.rs`)

These tests validate the functionality of the `Note` struct and its methods.

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

#[test]
fn test_display_format() {
    let note = Note::new(
        "Test Title".to_string(),
        "Test Content".to_string(),
        vec!["test".to_string()],
        Some("Test Category".to_string())
    );
    
    let display_string = format!("{}", note);
    
    assert!(display_string.contains("ID: "));
    assert!(display_string.contains("Title: Test Title"));
    assert!(display_string.contains("Content: Test Content"));
    assert!(display_string.contains("Tags: test"));
    assert!(display_string.contains("Category: Test Category"));
    assert!(display_string.contains("Created: "));
    assert!(display_string.contains("Updated: "));
}
```

### Storage Tests (`tests/storage_tests.rs`)

These tests validate the functionality of the `Storage` struct and its methods.

```rust
#[test]
fn test_create_storage() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("test_notes.json");
    
    let storage = Storage::new(storage_path).unwrap();
    assert_eq!(storage.list_notes().len(), 0);
}

#[test]
fn test_add_note() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("test_notes.json");
    
    let mut storage = Storage::new(storage_path).unwrap();
    
    let note = Note::new(
        "Test Note".to_string(),
        "Test Content".to_string(),
        vec!["test".to_string()],
        None
    );
    
    let id = note.id.clone();
    storage.add_note(note).unwrap();
    
    assert_eq!(storage.list_notes().len(), 1);
    assert!(storage.get_note(&id).is_some());
}

#[test]
fn test_get_note() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("test_notes.json");
    
    let mut storage = Storage::new(storage_path).unwrap();
    
    let note = Note::new(
        "Test Note".to_string(),
        "Test Content".to_string(),
        vec!["test".to_string()],
        None
    );
    
    let id = note.id.clone();
    storage.add_note(note).unwrap();
    
    let retrieved_note = storage.get_note(&id).unwrap();
    assert_eq!(retrieved_note.title, "Test Note");
    assert_eq!(retrieved_note.content, "Test Content");
}

#[test]
fn test_delete_note() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("test_notes.json");
    
    let mut storage = Storage::new(storage_path).unwrap();
    
    let note = Note::new(
        "Test Note".to_string(),
        "Test Content".to_string(),
        vec!["test".to_string()],
        None
    );
    
    let id = note.id.clone();
    storage.add_note(note).unwrap();
    
    assert_eq!(storage.list_notes().len(), 1);
    
    storage.delete_note(&id).unwrap();
    assert_eq!(storage.list_notes().len(), 0);
    assert!(storage.get_note(&id).is_none());
}

#[test]
fn test_search_notes() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("test_notes.json");
    
    let mut storage = Storage::new(storage_path).unwrap();
    
    // Add three notes
    let note1 = Note::new(
        "Apple Note".to_string(),
        "This is about apples.".to_string(),
        vec!["fruit".to_string()],
        Some("Food".to_string())
    );
    
    let note2 = Note::new(
        "Banana Note".to_string(),
        "This is about bananas.".to_string(),
        vec!["fruit".to_string()],
        Some("Food".to_string())
    );
    
    let note3 = Note::new(
        "Car Note".to_string(),
        "This is about cars.".to_string(),
        vec!["vehicle".to_string()],
        Some("Transport".to_string())
    );
    
    storage.add_note(note1).unwrap();
    storage.add_note(note2).unwrap();
    storage.add_note(note3).unwrap();
    
    // Search by title
    let results = storage.search_notes("Apple");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "Apple Note");
    
    // Search by content
    let results = storage.search_notes("bananas");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "Banana Note");
    
    // Search by tag
    let results = storage.search_notes("fruit");
    assert_eq!(results.len(), 2);
    
    // Search by category
    let results = storage.search_notes("Transport");
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].title, "Car Note");
}

#[test]
fn test_persistence() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("test_notes.json");
    
    // Create storage and add a note
    {
        let mut storage = Storage::new(storage_path.clone()).unwrap();
        
        let note = Note::new(
            "Persistent Note".to_string(),
            "This note should persist.".to_string(),
            vec!["test".to_string()],
            None
        );
        
        let id = note.id.clone();
        storage.add_note(note).unwrap();
        
        // Storage is dropped here, which should save the notes
    }
    
    // Create a new storage instance and check if the note is there
    {
        let storage = Storage::new(storage_path).unwrap();
        
        assert_eq!(storage.list_notes().len(), 1);
        
        let note = storage.list_notes()[0];
        assert_eq!(note.title, "Persistent Note");
        assert_eq!(note.content, "This note should persist.");
    }
}
```

### Error Tests (`tests/error_tests.rs`)

These tests validate the error handling functionality.

```rust
#[test]
fn test_not_found_error() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("test_notes.json");
    
    let mut storage = Storage::new(storage_path).unwrap();
    
    let result = storage.delete_note("non_existent_id");
    assert!(matches!(result, Err(NoteError::NotFound(_))));
}

#[test]
fn test_invalid_input_error() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("test_notes.json");
    
    let mut app = App::new(Some(storage_path)).unwrap();
    
    let result = app.add_note(
        "".to_string(),  // Empty title
        "Content".to_string(),
        vec![],
        None
    );
    
    assert!(matches!(result, Err(NoteError::InvalidInput(_))));
}

#[test]
fn test_io_error() {
    // Try to create storage with a path that can't be written to
    let result = Storage::new(PathBuf::from("/root/impossible_path/notes.json"));
    
    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e, NoteError::Io(_)));
    }
}

#[test]
fn test_error_display() {
    let io_error = NoteError::Io(io::Error::new(io::ErrorKind::NotFound, "file not found"));
    let not_found_error = NoteError::NotFound("test_id".to_string());
    let invalid_input_error = NoteError::InvalidInput("empty title".to_string());
    let serialization_error = NoteError::Serialization("invalid JSON".to_string());
    
    assert!(format!("{}", io_error).contains("I/O error"));
    assert!(format!("{}", not_found_error).contains("Note not found: test_id"));
    assert!(format!("{}", invalid_input_error).contains("Invalid input: empty title"));
    assert!(format!("{}", serialization_error).contains("Serialization error: invalid JSON"));
}
```

## Integration Tests

### App Tests (`tests/app_tests.rs`)

These tests validate the functionality of the `App` struct, which integrates the `Note` and `Storage` components.

```rust
#[test]
fn test_app_creation() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("test_notes.json");
    
    let app = App::new(Some(storage_path)).unwrap();
    assert_eq!(app.list_notes().len(), 0);
}

#[test]
fn test_add_note() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("test_notes.json");
    
    let mut app = App::new(Some(storage_path)).unwrap();
    
    let id = app.add_note(
        "Test Note".to_string(),
        "This is a test note.".to_string(),
        vec!["test".to_string()],
        Some("Testing".to_string())
    ).unwrap();
    
    let note = app.get_note(&id).unwrap();
    assert_eq!(note.title, "Test Note");
    assert_eq!(note.content, "This is a test note.");
    assert_eq!(note.tags, vec!["test".to_string()]);
    assert_eq!(note.category, Some("Testing".to_string()));
}

#[test]
fn test_update_note() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("test_notes.json");
    
    let mut app = App::new(Some(storage_path)).unwrap();
    
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
    assert_eq!(note.content, "Content");  // Unchanged
}

#[test]
fn test_delete_note() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("test_notes.json");
    
    let mut app = App::new(Some(storage_path)).unwrap();
    
    let id = app.add_note(
        "To Delete".to_string(),
        "This note will be deleted.".to_string(),
        vec![],
        None
    ).unwrap();
    
    assert_eq!(app.list_notes().len(), 1);
    
    app.delete_note(&id).unwrap();
    
    assert_eq!(app.list_notes().len(), 0);
    assert!(app.get_note(&id).is_err());
}

#[test]
fn test_search_notes() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("test_notes.json");
    
    let mut app = App::new(Some(storage_path)).unwrap();
    
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
}
```

## End-to-End Tests

### CLI Tests (`tests/cli_tests.rs`)

These tests validate the command-line interface functionality by simulating user input.

```rust
#[test]
fn test_cli_add_note() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("test_notes.json");
    
    let mut app = App::new(Some(storage_path)).unwrap();
    
    // Simulate user input for adding a note
    let input = vec![
        "Test Note",
        "This is a test note.",
        "test,cli",
        "Testing"
    ];
    
    let id = simulate_add_note(&mut app, input).unwrap();
    
    // Verify the note was added correctly
    let note = app.get_note(&id).unwrap();
    assert_eq!(note.title, "Test Note");
    assert_eq!(note.content, "This is a test note.");
    assert_eq!(note.tags, vec!["test".to_string(), "cli".to_string()]);
    assert_eq!(note.category, Some("Testing".to_string()));
}

#[test]
fn test_cli_update_note() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("test_notes.json");
    
    let mut app = App::new(Some(storage_path)).unwrap();
    
    // Add a note first
    let id = app.add_note(
        "Original".to_string(),
        "Content".to_string(),
        vec![],
        None
    ).unwrap();
    
    // Simulate user input for updating the note
    let input = vec![
        "Updated Title",
        "",  // Keep original content
        "new,tags",
        ""   // Keep original category
    ];
    
    simulate_update_note(&mut app, &id, input).unwrap();
    
    // Verify the note was updated correctly
    let note = app.get_note(&id).unwrap();
    assert_eq!(note.title, "Updated Title");
    assert_eq!(note.content, "Content");  // Unchanged
    assert_eq!(note.tags, vec!["new".to_string(), "tags".to_string()]);
    assert_eq!(note.category, None);  // Unchanged
}

#[test]
fn test_cli_search_notes() {
    let temp_dir = tempdir().unwrap();
    let storage_path = temp_dir.path().join("test_notes.json");
    
    let mut app = App::new(Some(storage_path)).unwrap();
    
    // Add some notes
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
    
    // Simulate search command
    let results = simulate_search(&app, "fruit").unwrap();
    
    // Verify search results
    assert_eq!(results.len(), 2);
    assert!(results.iter().any(|note| note.title == "Apple Note"));
    assert!(results.iter().any(|note| note.title == "Banana Note"));
}

// Helper functions to simulate CLI commands

fn simulate_add_note(app: &mut App, input: Vec<&str>) -> Result<String, NoteError> {
    let title = input[0].to_string();
    let content = input[1].to_string();
    
    let tags = input[2]
        .split(',')
        .map(|s| s.trim().to_string())
        .filter(|s| !s.is_empty())
        .collect();
    
    let category = if input[3].is_empty() {
        None
    } else {
        Some(input[3].to_string())
    };
    
    app.add_note(title, content, tags, category)
}

fn simulate_update_note(app: &mut App, id: &str, input: Vec<&str>) -> Result<(), NoteError> {
    let title = if input[0].is_empty() { None } else { Some(input[0].to_string()) };
    let content = if input[1].is_empty() { None } else { Some(input[1].to_string()) };
    
    let tags = if input[2].is_empty() {
        None
    } else {
        Some(input[2]
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect())
    };
    
    let category = if input[3].is_empty() {
        None
    } else if input[3].to_lowercase() == "none" {
        Some(None)
    } else {
        Some(Some(input[3].to_string()))
    };
    
    app.update_note(id, title, content, tags, category)
}

fn simulate_search(app: &App, query: &str) -> Result<Vec<&Note>, NoteError> {
    Ok(app.search_notes(query))
}
```

## Running the Tests

To run the entire test suite:

```bash
cargo test
```

To run a specific test:

```bash
cargo test test_add_note
```

To run tests with output:

```bash
cargo test -- --nocapture
```

## Test Coverage

The test suite covers:

- **Core Functionality**: All CRUD operations for notes
- **Error Handling**: All error conditions and edge cases
- **Search Functionality**: Various search criteria and combinations
- **Persistence**: Saving and loading notes from storage
- **CLI Interface**: Command parsing and execution

By running this comprehensive test suite, you can ensure that the RustyNotes application functions correctly and meets all the requirements specified in the project.