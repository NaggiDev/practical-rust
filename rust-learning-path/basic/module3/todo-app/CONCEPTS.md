# Rust Concepts in the To-Do List Application

This document explains the key Rust concepts used in the To-Do List application, focusing on structs and enums.

## Structs in Rust

### What are Structs?

Structs (short for "structures") are custom data types that let you package together and name multiple related values. They're similar to classes in object-oriented languages but with some key differences. In Rust, structs are a fundamental way to create new types.

### Types of Structs in Rust

1. **Named-Field Structs**: The most common type, with named fields.
   ```rust
   struct Task {
       id: usize,
       title: String,
       status: TaskStatus,
   }
   ```

2. **Tuple Structs**: Structs without named fields, just the types.
   ```rust
   struct Point(i32, i32);
   ```

3. **Unit Structs**: Structs without any fields, useful for implementing traits.
   ```rust
   struct UnitStruct;
   ```

### Struct Implementation in Our Project

In our To-Do List application, we use two main structs:

#### 1. The `Task` Struct

```rust
pub struct Task {
    pub id: usize,
    pub title: String,
    pub description: Option<String>,
    pub status: TaskStatus,
    pub due_date: Option<DateTime<Local>>,
    pub created_at: DateTime<Local>,
}
```

This struct represents a single task with:
- A unique identifier (`id`)
- A title (`title`)
- An optional description (`description`)
- A status (`status`, which is an enum)
- An optional due date (`due_date`)
- A creation timestamp (`created_at`)

#### 2. The `TaskList` Struct

```rust
pub struct TaskList {
    tasks: Vec<Task>,
    next_id: usize,
}
```

This struct manages a collection of tasks with:
- A vector of tasks (`tasks`)
- A counter for generating the next task ID (`next_id`)

### Methods on Structs

Rust allows you to define methods on structs using `impl` blocks:

```rust
impl Task {
    pub fn new(id: usize, title: String, description: Option<String>, due_date: Option<DateTime<Local>>) -> Self {
        Task {
            id,
            title,
            description,
            status: TaskStatus::Pending,
            due_date,
            created_at: Local::now(),
        }
    }
    
    pub fn update_status(&mut self, status: TaskStatus) {
        self.status = status;
    }
}
```

Key points about methods:
- The `Self` return type refers to the type the `impl` block is for
- The first parameter is often `&self` or `&mut self`, giving access to the struct instance
- Methods can take ownership (`self`), borrow immutably (`&self`), or borrow mutably (`&mut self`)

## Enums in Rust

### What are Enums?

Enums (short for "enumerations") allow you to define a type by enumerating its possible variants. They're particularly powerful in Rust because each variant can have different associated data.

### Basic Enum Usage

```rust
enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}
```

This defines a type `TaskStatus` that can only be one of three values.

### Enum Implementation in Our Project

In our To-Do List application, we use the `TaskStatus` enum:

```rust
pub enum TaskStatus {
    Pending,
    InProgress,
    Completed,
}
```

This enum represents the possible states of a task.

### Pattern Matching with Enums

Enums are often used with pattern matching via the `match` expression:

```rust
match task.status {
    TaskStatus::Pending => println!("Task is pending"),
    TaskStatus::InProgress => println!("Task is in progress"),
    TaskStatus::Completed => println!("Task is completed"),
}
```

In our project, we use pattern matching in the `Display` implementation for `TaskStatus`:

```rust
impl fmt::Display for TaskStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TaskStatus::Pending => write!(f, "Pending"),
            TaskStatus::InProgress => write!(f, "In Progress"),
            TaskStatus::Completed => write!(f, "Completed"),
        }
    }
}
```

## Ownership Concepts in the Project

### Ownership and Borrowing

Our project demonstrates several key ownership concepts:

1. **Ownership Transfer**: When we add a task to the task list, ownership of the title string is transferred:
   ```rust
   pub fn add_task(&mut self, title: String, ...) -> &Task {
       // title's ownership moves into the new Task
   }
   ```

2. **Borrowing**: Many methods borrow data rather than taking ownership:
   ```rust
   pub fn get_task(&self, id: usize) -> Option<&Task> {
       // Returns a borrowed reference to a task
   }
   ```

3. **Mutable Borrowing**: Some methods need to modify data:
   ```rust
   pub fn get_task_mut(&mut self, id: usize) -> Option<&mut Task> {
       // Returns a mutable reference to a task
   }
   ```

### Option Type

The `Option` type is used extensively to handle cases where a value might be absent:

```rust
pub description: Option<String>,
pub due_date: Option<DateTime<Local>>,
```

This avoids null references and forces explicit handling of the absence of values.

## Collections

### Vectors

Our `TaskList` uses a vector to store tasks:

```rust
tasks: Vec<Task>,
```

Vectors in Rust are resizable arrays that can only store elements of the same type.

### Working with Vectors

The project demonstrates several vector operations:

1. **Adding elements**:
   ```rust
   self.tasks.push(task);
   ```

2. **Finding elements**:
   ```rust
   self.tasks.iter().find(|task| task.id == id)
   ```

3. **Removing elements**:
   ```rust
   self.tasks.remove(pos);
   ```

4. **Filtering elements**:
   ```rust
   self.tasks.iter()
       .filter(|task| task.status == status)
       .collect()
   ```

## Traits

### Display Trait

We implement the `Display` trait for both `Task` and `TaskStatus` to provide string representations:

```rust
impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Task #{}: {} [{}]", self.id, self.title, self.status)
    }
}
```

This allows us to use these types with the `{}` format specifier in `println!` and similar macros.

### Debug Trait

We derive the `Debug` trait for our types:

```rust
#[derive(Debug, Clone, PartialEq)]
pub enum TaskStatus {
    // ...
}

#[derive(Debug, Clone)]
pub struct Task {
    // ...
}
```

This enables the use of the `{:?}` format specifier for debugging output.

## Conclusion

This To-Do List application demonstrates several fundamental Rust concepts:

1. **Structs** for creating custom data types
2. **Enums** for representing a fixed set of variants
3. **Methods** for adding behavior to types
4. **Ownership and borrowing** for memory safety
5. **Collections** for managing groups of values
6. **Traits** for shared behavior

Understanding these concepts is crucial for effective Rust programming and forms the foundation for more advanced topics in the language.