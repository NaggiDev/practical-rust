# RustyNotes - Basic Level Capstone Project

## Project Overview

RustyNotes is a command-line note-taking application that brings together all the concepts you've learned in the Basic Level of the Rust Learning Path. This application allows users to create, read, update, and delete notes with tags, categories, and search functionality.

## Learning Objectives

By completing this project, you will:

- Apply all Basic Level concepts in a comprehensive application
- Gain experience building a complete Rust project from scratch
- Practice test-driven development
- Learn to structure a larger Rust application
- Implement error handling throughout an application
- Work with external crates for enhanced functionality

## Project Features

RustyNotes includes the following features:

1. **Note Management**:
   - Create notes with title, content, tags, and optional category
   - View notes by ID or list all notes
   - Update existing notes
   - Delete notes

2. **Organization**:
   - Tag notes with multiple keywords
   - Categorize notes
   - Search notes by content, title, tags, or category

3. **Data Persistence**:
   - Save notes to a JSON file
   - Load notes from a JSON file on startup
   - Automatic saving when changes are made

4. **User Interface**:
   - Command-line interface with clear instructions
   - Formatted note display
   - Error messages for invalid operations

## Getting Started

1. Create a new Rust project:
   ```bash
   cargo new rusty_notes
   cd rusty_notes
   ```

2. Add the required dependencies to `Cargo.toml`:
   ```toml
   [dependencies]
   chrono = "0.4"
   serde = { version = "1.0", features = ["derive"] }
   serde_json = "1.0"
   uuid = { version = "1.0", features = ["v4"] }
   dirs = "4.0"
   ```

3. Follow the step-by-step instructions in the Implementation Guide to build the application.

## Project Structure

```
rusty_notes/
├── Cargo.toml
├── src/
│   ├── main.rs         # Entry point and CLI handling
│   ├── note.rs         # Note struct and methods
│   ├── app.rs          # Application logic
│   ├── storage.rs      # Data persistence
│   └── error.rs        # Error handling
└── tests/
    ├── note_tests.rs   # Tests for Note functionality
    ├── app_tests.rs    # Tests for application logic
    └── storage_tests.rs # Tests for data persistence
```