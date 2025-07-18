# Basic To-Do List Application

## Project Overview

In this project, you will build a command-line to-do list application that demonstrates Rust's ownership system, structs, and enums. This application will allow users to create, read, update, and delete tasks, showcasing how to manage data with Rust's type system.

## Learning Objectives

By completing this project, you will:

- Understand how to define and use structs in Rust
- Learn how to implement methods on structs
- Work with enums and pattern matching
- Practice managing collections of data
- Apply Rust's ownership rules in a practical context
- Implement basic CRUD (Create, Read, Update, Delete) operations

## Prerequisites

- Completion of Module 1 (Rust Basics) and Module 2 (Functions and Error Handling)
- Understanding of basic Rust syntax, control flow, and error handling
- Familiarity with vectors and basic string manipulation

## Project Structure

```
todo-app/
├── Cargo.toml           # Project dependencies and metadata
├── src/
│   ├── main.rs          # Entry point for the application
│   ├── task.rs          # Task struct and related functionality
│   └── task_list.rs     # TaskList struct for managing collections of tasks
└── tests/
    └── integration_tests.rs  # Integration tests for the application
```

## Step-by-Step Instructions

### Step 1: Project Setup

1. Create a new Rust project:
   ```bash
   cargo new todo-app
   cd todo-app
   ```

2. Update the `Cargo.toml` file with necessary dependencies:
   ```toml
   [package]
   name = "todo-app"
   version = "0.1.0"
   edition = "2021"

   [dependencies]
   chrono = "0.4"  # For handling dates and times
   ```

### Step 2: Define the Task Struct and Enums

In this step, you'll create the core data structures for your application.

1. Create a new file `src/task.rs` and define the `Task` struct and related enums:
   ```rust
   // TODO: Implement the Task struct with fields for id, title, description, status, and due_date
   // TODO: Implement the TaskStatus enum with values for Pending, InProgress, and Completed
   // TODO: Implement methods for the Task struct
   ```

2. Implement methods for the `Task` struct:
   ```rust
   // TODO: Implement a new() method to create a new task
   // TODO: Implement methods to update task status, title, and description
   // TODO: Implement a display method to format the task for output
   ```

### Step 3: Implement the TaskList

1. Create a new file `src/task_list.rs` to manage collections of tasks:
   ```rust
   // TODO: Implement the TaskList struct with a vector of Tasks
   // TODO: Implement methods for adding, removing, and updating tasks
   // TODO: Implement methods for filtering tasks by status
   ```

### Step 4: Create the Command-Line Interface

1. Update `src/main.rs` to implement the command-line interface:
   ```rust
   // TODO: Import the task and task_list modules
   // TODO: Implement command parsing for add, list, update, and delete operations
   // TODO: Implement the main function with a loop for processing commands
   ```

### Step 5: Implement Error Handling

1. Update your code to handle potential errors:
   ```rust
   // TODO: Implement error handling for user input
   // TODO: Handle errors when tasks are not found
   // TODO: Use Result and Option types appropriately
   ```

### Step 6: Write Tests

1. Create tests in `tests/integration_tests.rs`:
   ```rust
   // TODO: Write tests for adding tasks
   // TODO: Write tests for updating tasks
   // TODO: Write tests for deleting tasks
   // TODO: Write tests for listing and filtering tasks
   ```

## Concepts Applied

### Structs

Structs in Rust are used to create custom data types that group related values. In this project, you'll use structs to represent tasks and the task list.

Key concepts:
- Defining struct fields with appropriate types
- Creating instances of structs
- Accessing and modifying struct fields
- Implementing methods on structs using `impl` blocks

### Enums

Enums in Rust allow you to define a type that can be one of several variants. In this project, you'll use enums to represent task status.

Key concepts:
- Defining enum variants
- Pattern matching with enums using `match`
- Associating data with enum variants
- Using enums to represent state

### Ownership and Borrowing

This project will give you practical experience with Rust's ownership system:

Key concepts:
- Ownership of task data
- Borrowing references to tasks
- Mutable and immutable references
- Using the `&` and `&mut` syntax

### Collections

You'll work with Rust's collection types to store and manage multiple tasks:

Key concepts:
- Using `Vec<T>` to store a collection of tasks
- Adding and removing elements from collections
- Iterating over collections
- Filtering and transforming collections

## Extension Challenges

Once you've completed the basic implementation, try these extensions:

1. **Persistence**: Add functionality to save and load the task list from a file.
2. **Due Dates**: Enhance the `Task` struct to include due dates and implement sorting by due date.
3. **Categories**: Add support for categorizing tasks and filtering by category.
4. **Priority Levels**: Implement a priority system for tasks and allow sorting by priority.
5. **Subtasks**: Modify the design to support hierarchical tasks (tasks with subtasks).

## Resources

- [The Rust Book - Using Structs to Structure Related Data](https://doc.rust-lang.org/book/ch05-00-structs.html)
- [The Rust Book - Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)
- [The Rust Book - Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)
- [Rust Documentation for chrono crate](https://docs.rs/chrono/)