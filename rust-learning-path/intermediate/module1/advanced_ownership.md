# Advanced Ownership in Rust

## Introduction

Rust's ownership system is one of its most distinctive features, providing memory safety without a garbage collector. In the Basic level, you learned about the fundamental ownership rules. Now, we'll explore more advanced aspects of ownership, including complex borrowing patterns, lifetimes, and reference counting.

## Borrowing Rules Revisited

### Immutable and Mutable Borrowing

Rust's borrowing rules can be summarized as:
1. You can have either one mutable reference or any number of immutable references
2. References must always be valid

```rust
fn main() {
    let mut value = 10;
    
    // Multiple immutable borrows are allowed
    let ref1 = &value;
    let ref2 = &value;
    println!("References: {} {}", ref1, ref2);
    
    // After immutable references are no longer used, we can create a mutable reference
    let ref_mut = &mut value;
    *ref_mut += 10;
    println!("After mutation: {}", value);
    
    // This would NOT compile - can't have immutable and mutable references at the same time
    // let ref3 = &value;
    // println!("Values: {} {}", ref_mut, ref3);
}
```

### Non-Lexical Lifetimes (NLL)

Rust's borrow checker uses non-lexical lifetimes to determine when references are no longer used:

```rust
fn main() {
    let mut value = 5;
    
    let reference = &value;
    println!("Value: {}", reference);
    // reference is no longer used after this point
    
    // This works! Even though reference is still in scope,
    // the borrow checker knows it's no longer used
    let reference_mut = &mut value;
    *reference_mut += 1;
    
    println!("New value: {}", value);
}
```

## Lifetimes

Lifetimes are Rust's way of ensuring that references are always valid. They're part of the type system but usually inferred by the compiler.

### Lifetime Annotations

When the compiler can't infer lifetimes, you need to annotate them explicitly:

```rust
// 'a is a lifetime parameter
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("long string is long");
    let string2 = String::from("short");
    
    let result = longest(&string1, &string2);
    println!("The longest string is: {}", result);
}
```

### Lifetime Elision

Rust has rules for inferring lifetimes in functions:

1. Each parameter that is a reference gets its own lifetime parameter
2. If there is exactly one input lifetime parameter, that lifetime is assigned to all output lifetime parameters
3. If there are multiple input lifetime parameters, but one of them is `&self` or `&mut self`, the lifetime of self is assigned to all output lifetime parameters

```rust
// These two functions are equivalent
fn first_word(s: &str) -> &str {
    // ...
}

fn first_word<'a>(s: &'a str) -> &'a str {
    // ...
}
```

### Lifetimes in Structs

When a struct holds references, it needs lifetime annotations:

```rust
struct Excerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let excerpt = Excerpt {
        part: first_sentence,
    };
    
    println!("Excerpt: {}", excerpt.part);
}
```

## Smart Pointers

Smart pointers are data structures that act like pointers but have additional metadata and capabilities.

### Box<T>

`Box<T>` is the simplest smart pointer, used for allocating values on the heap:

```rust
fn main() {
    // Allocate an integer on the heap
    let b = Box::new(5);
    println!("b = {}", b);
    
    // Useful for recursive types
    enum List {
        Cons(i32, Box<List>),
        Nil,
    }
    
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
}
```

### Rc<T> (Reference Counting)

`Rc<T>` enables multiple ownership by keeping track of the number of references to a value:

```rust
use std::rc::Rc;

fn main() {
    // Create a reference-counted string
    let text = Rc::new(String::from("Hello, world!"));
    println!("Reference count: {}", Rc::strong_count(&text)); // 1
    
    // Create a clone (increases the reference count)
    let text2 = Rc::clone(&text);
    println!("Reference count: {}", Rc::strong_count(&text)); // 2
    
    // Create another clone
    let text3 = Rc::clone(&text);
    println!("Reference count: {}", Rc::strong_count(&text)); // 3
    
    // text3 goes out of scope, reference count decreases
    drop(text3);
    println!("Reference count: {}", Rc::strong_count(&text)); // 2
    
    // When text and text2 go out of scope, the reference count reaches 0
    // and the memory is freed
}
```

### Arc<T> (Atomic Reference Counting)

`Arc<T>` is like `Rc<T>` but safe to use in concurrent situations:

```rust
use std::sync::Arc;
use std::thread;

fn main() {
    let counter = Arc::new(vec![1, 2, 3]);
    let mut handles = vec![];
    
    for i in 0..3 {
        // Clone the Arc to increase the reference count
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            println!("Thread {}: {:?}", i, counter_clone);
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
}
```

### RefCell<T> and Interior Mutability

`RefCell<T>` allows mutable borrows checked at runtime instead of compile time:

```rust
use std::cell::RefCell;

fn main() {
    let data = RefCell::new(5);
    
    // Borrow mutably and modify the value
    *data.borrow_mut() += 1;
    
    // Borrow immutably and print the value
    println!("Value: {}", data.borrow());
    
    // This would panic at runtime - can't borrow mutably and immutably at the same time
    // let mut_borrow = data.borrow_mut();
    // let borrow = data.borrow();
}
```

### Combining Rc<T> and RefCell<T>

A common pattern is to combine `Rc<T>` and `RefCell<T>` to have multiple owners of mutable data:

```rust
use std::cell::RefCell;
use std::rc::Rc;

fn main() {
    let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));
    
    // Create a clone of the Rc
    let shared_data2 = Rc::clone(&shared_data);
    
    // Modify the data through the first reference
    shared_data.borrow_mut().push(4);
    
    // Modify the data through the second reference
    shared_data2.borrow_mut().push(5);
    
    // Both references see the changes
    println!("shared_data: {:?}", shared_data.borrow());
    println!("shared_data2: {:?}", shared_data2.borrow());
}
```

### Weak<T> and Preventing Reference Cycles

`Weak<T>` helps prevent memory leaks from reference cycles:

```rust
use std::cell::RefCell;
use std::rc::{Rc, Weak};

// A tree structure where nodes can have a parent and multiple children
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,
    children: RefCell<Vec<Rc<Node>>>,
}

fn main() {
    // Create a leaf node with no parent or children
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });
    
    // Create a branch node with the leaf as a child
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]),
    });
    
    // Set the leaf's parent to the branch
    *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
    
    // Print the leaf's parent value
    println!("Leaf's parent value: {}", leaf.parent.borrow().upgrade().unwrap().value);
}
```

## Common Ownership Patterns

### Ownership Transfer

Transferring ownership is a fundamental concept in Rust:

```rust
fn main() {
    let s1 = String::from("hello");
    
    // Ownership of s1 is moved to s2
    let s2 = s1;
    
    // This would not compile because s1 is no longer valid
    // println!("s1: {}", s1);
    
    // s2 is valid
    println!("s2: {}", s2);
    
    // Ownership of s2 is moved to the function
    take_ownership(s2);
    
    // This would not compile because s2 is no longer valid
    // println!("s2: {}", s2);
    
    // The function gives us back ownership
    let s3 = give_ownership();
    println!("s3: {}", s3);
}

fn take_ownership(s: String) {
    println!("Taking ownership of: {}", s);
    // s goes out of scope and is dropped
}

fn give_ownership() -> String {
    let s = String::from("yours");
    s // Return ownership to the caller
}
```

### Borrowing with References

References allow you to use values without taking ownership:

```rust
fn main() {
    let s1 = String::from("hello");
    
    // Pass a reference to s1
    let len = calculate_length(&s1);
    
    // s1 is still valid here
    println!("The length of '{}' is {}.", s1, len);
    
    // Mutable borrowing
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("Changed string: {}", s2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
    // s goes out of scope, but it doesn't have ownership, so nothing happens
}

fn change(s: &mut String) {
    s.push_str(", world");
}
```

## Conclusion

Advanced ownership in Rust provides powerful tools for managing memory and resources. By understanding lifetimes, smart pointers, and borrowing patterns, you can write safe and efficient code that handles complex ownership scenarios.

In the next sections, we'll apply these concepts to practical examples and explore how they're used in real-world Rust applications.