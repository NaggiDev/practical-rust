# Rust Concepts Reference

This comprehensive reference document provides detailed explanations of all Rust concepts covered in the learning path, organized by complexity level. Each concept includes practical examples, best practices, and links to official Rust documentation.

## Table of Contents

### [Basic Level Concepts](#basic-level-concepts)
1. [Variables and Mutability](#variables-and-mutability)
2. [Data Types](#data-types)
3. [Functions](#functions)
4. [Control Flow](#control-flow)
5. [Ownership Basics](#ownership-basics)
6. [References and Borrowing](#references-and-borrowing)
7. [Structs](#structs)
8. [Enums](#enums)
9. [Pattern Matching](#pattern-matching)
10. [Error Handling with Result and Option](#error-handling-with-result-and-option)
11. [Collections](#collections)
12. [String Handling](#string-handling)

### [Intermediate Level Concepts](#intermediate-level-concepts)
1. [Advanced Ownership](#advanced-ownership)
2. [Lifetimes](#lifetimes)
3. [Traits](#traits)
4. [Generics](#generics)
5. [Advanced Collections](#advanced-collections)
6. [Iterators](#iterators)
7. [Closures](#closures)
8. [Custom Error Types](#custom-error-types)
9. [Modules and Packages](#modules-and-packages)
10. [Testing](#testing)

### [Advanced Level Concepts](#advanced-level-concepts)
1. [Concurrency and Threading](#concurrency-and-threading)
2. [Unsafe Rust](#unsafe-rust)
3. [Advanced Traits](#advanced-traits)
4. [Macros](#macros)
5. [Foreign Function Interface (FFI)](#foreign-function-interface-ffi)
6. [Memory Management](#memory-management)
7. [Smart Pointers](#smart-pointers)

### [Expert Level Concepts](#expert-level-concepts)
1. [Async Programming](#async-programming)
2. [Advanced Memory Management](#advanced-memory-management)
3. [Compiler Internals](#compiler-internals)
4. [Performance Optimization](#performance-optimization)
5. [Advanced Async Patterns](#advanced-async-patterns)

---

## Basic Level Concepts

### Variables and Mutability

Variables in Rust are immutable by default, which helps prevent bugs and makes code more predictable.

#### Immutable Variables

```rust
fn main() {
    let x = 5;
    println!("The value of x is: {}", x);
    // x = 6; // This would cause a compile error
}
```

#### Mutable Variables

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);
}
```

#### Constants

```rust
const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;

fn main() {
    println!("Three hours in seconds: {}", THREE_HOURS_IN_SECONDS);
}
```

#### Shadowing

```rust
fn main() {
    let x = 5;
    let x = x + 1; // Shadowing the previous x
    
    {
        let x = x * 2; // Shadowing in inner scope
        println!("The value of x in the inner scope is: {}", x); // 12
    }
    
    println!("The value of x is: {}", x); // 6
}
```

**Official Documentation**: [Variables and Mutability](https://doc.rust-lang.org/book/ch03-01-variables-and-mutability.html)

### Data Types

Rust is a statically typed language, meaning it must know the types of all variables at compile time.

#### Scalar Types

```rust
fn main() {
    // Integer types
    let decimal = 98_222;
    let hex = 0xff;
    let octal = 0o77;
    let binary = 0b1111_0000;
    let byte = b'A';
    
    // Floating-point types
    let x = 2.0; // f64 (default)
    let y: f32 = 3.0; // f32
    
    // Boolean type
    let t = true;
    let f: bool = false;
    
    // Character type
    let c = 'z';
    let z: char = 'ℤ';
    let heart_eyed_cat = '😻';
}
```

#### Compound Types

```rust
fn main() {
    // Tuple type
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (x, y, z) = tup; // Destructuring
    let five_hundred = tup.0; // Accessing by index
    
    // Array type
    let a = [1, 2, 3, 4, 5];
    let months = ["January", "February", "March"];
    let a: [i32; 5] = [1, 2, 3, 4, 5]; // Type annotation
    let a = [3; 5]; // [3, 3, 3, 3, 3]
    
    let first = a[0];
    let second = a[1];
}
```

**Official Documentation**: [Data Types](https://doc.rust-lang.org/book/ch03-02-data-types.html)

### Functions

Functions are defined using the `fn` keyword and follow snake_case naming convention.

#### Basic Function Definition

```rust
fn main() {
    println!("Hello, world!");
    another_function();
    function_with_parameter(5);
    print_labeled_measurement(5, 'h');
}

fn another_function() {
    println!("Another function.");
}

fn function_with_parameter(x: i32) {
    println!("The value of x is: {}", x);
}

fn print_labeled_measurement(value: i32, unit_label: char) {
    println!("The measurement is: {}{}", value, unit_label);
}
```

#### Functions with Return Values

```rust
fn five() -> i32 {
    5 // No semicolon - this is an expression
}

fn plus_one(x: i32) -> i32 {
    x + 1 // Expression, not statement
}

fn main() {
    let x = five();
    println!("The value of x is: {}", x);
    
    let x = plus_one(5);
    println!("The value of x is: {}", x);
}
```

#### Statements vs Expressions

```rust
fn main() {
    let y = {
        let x = 3;
        x + 1 // Expression - no semicolon
    };
    
    println!("The value of y is: {}", y);
}
```

**Official Documentation**: [Functions](https://doc.rust-lang.org/book/ch03-03-how-functions-work.html)

### Control Flow

Control flow constructs allow you to control the execution path of your program.

#### if Expressions

```rust
fn main() {
    let number = 3;
    
    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false");
    }
    
    // Multiple conditions
    let number = 6;
    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3");
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }
    
    // Using if in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}
```

#### Loops

```rust
fn main() {
    // loop - infinite loop
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2; // Return value from loop
        }
    };
    println!("The result is {}", result);
    
    // while loop
    let mut number = 3;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    println!("LIFTOFF!!!");
    
    // for loop
    let a = [10, 20, 30, 40, 50];
    for element in a {
        println!("the value is: {}", element);
    }
    
    // for loop with range
    for number in (1..4).rev() {
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

**Official Documentation**: [Control Flow](https://doc.rust-lang.org/book/ch03-05-control-flow.html)

### Ownership Basics

Ownership is Rust's most unique feature, enabling memory safety without garbage collection.

#### Ownership Rules

1. Each value in Rust has an owner
2. There can only be one owner at a time
3. When the owner goes out of scope, the value will be dropped

```rust
fn main() {
    {                      // s is not valid here, it's not yet declared
        let s = "hello";   // s is valid from this point forward
        // do stuff with s
    }                      // this scope is now over, and s is no longer valid
    
    // String type - heap allocated
    let mut s = String::from("hello");
    s.push_str(", world!"); // push_str() appends a literal to a String
    println!("{}", s);
}
```

#### Move Semantics

```rust
fn main() {
    let s1 = String::from("hello");
    let s2 = s1; // s1 is moved to s2
    
    // println!("{}, world!", s1); // This would cause a compile error
    println!("{}, world!", s2); // This works
    
    // Clone for deep copy
    let s1 = String::from("hello");
    let s2 = s1.clone();
    println!("s1 = {}, s2 = {}", s1, s2); // Both work
}
```

#### Copy Trait

```rust
fn main() {
    let x = 5;
    let y = x; // Copy, not move (integers implement Copy)
    println!("x = {}, y = {}", x, y); // Both work
}
```

**Official Documentation**: [Understanding Ownership](https://doc.rust-lang.org/book/ch04-01-what-is-ownership.html)

### References and Borrowing

References allow you to use a value without taking ownership of it.

#### Immutable References

```rust
fn main() {
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // Pass reference
    println!("The length of '{}' is {}.", s1, len);
}

fn calculate_length(s: &String) -> usize { // s is a reference to a String
    s.len()
} // Here, s goes out of scope. But because it does not have ownership of what
  // it refers to, it is not dropped.
```

#### Mutable References

```rust
fn main() {
    let mut s = String::from("hello");
    change(&mut s);
    println!("{}", s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}
```

#### Reference Rules

```rust
fn main() {
    let mut s = String::from("hello");
    
    // You can have multiple immutable references
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    
    // But only one mutable reference
    let r3 = &mut s;
    println!("{}", r3);
    
    // Cannot have mutable and immutable references simultaneously
    // let r4 = &s; // This would cause an error if r3 is still used
}
```

**Official Documentation**: [References and Borrowing](https://doc.rust-lang.org/book/ch04-02-references-and-borrowing.html)

### Structs

Structs let you create custom data types that group related data together.

#### Defining and Instantiating Structs

```rust
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        email: String::from("someone@example.com"),
        username: String::from("someusername123"),
        active: true,
        sign_in_count: 1,
    };
    
    // Mutable struct
    let mut user2 = User {
        email: String::from("another@example.com"),
        username: String::from("anotherusername567"),
        active: true,
        sign_in_count: 1,
    };
    
    user2.email = String::from("anotheremail@example.com");
    
    // Struct update syntax
    let user3 = User {
        email: String::from("another@example.com"),
        ..user1 // Use remaining fields from user1
    };
}

fn build_user(email: String, username: String) -> User {
    User {
        email,    // Field init shorthand
        username, // Field init shorthand
        active: true,
        sign_in_count: 1,
    }
}
```

#### Tuple Structs

```rust
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

fn main() {
    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);
}
```

#### Unit-Like Structs

```rust
struct AlwaysEqual;

fn main() {
    let subject = AlwaysEqual;
}
```

#### Methods

```rust
#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn width(&self) -> bool {
        self.width > 0
    }
    
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // Associated function (like static method)
    fn square(size: u32) -> Self {
        Self {
            width: size,
            height: size,
        }
    }
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("The area of the rectangle is {} square pixels.", rect1.area());
    
    if rect1.width() {
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
    
    let sq = Rectangle::square(3);
}
```

**Official Documentation**: [Using Structs](https://doc.rust-lang.org/book/ch05-00-structs.html)

### Enums

Enums allow you to define a type by enumerating its possible variants.

#### Defining Enums

```rust
enum IpAddrKind {
    V4,
    V6,
}

enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
    
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}
```

#### Methods on Enums

```rust
impl Message {
    fn call(&self) {
        // method body would be defined here
    }
}

fn main() {
    let m = Message::Write(String::from("hello"));
    m.call();
}
```

#### The Option Enum

```rust
fn main() {
    let some_number = Some(5);
    let some_char = Some('e');
    let absent_number: Option<i32> = None;
    
    // You must handle the None case
    match some_number {
        Some(value) => println!("Got a value: {}", value),
        None => println!("Got nothing"),
    }
}
```

**Official Documentation**: [Enums and Pattern Matching](https://doc.rust-lang.org/book/ch06-00-enums.html)

### Pattern Matching

Pattern matching with `match` is a powerful control flow construct.

#### Basic Match

```rust
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter,
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter => 25,
    }
}
```

#### Patterns that Bind to Values

```rust
#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // --snip--
}

enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => 1,
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("State quarter from {:?}!", state);
            25
        }
    }
}
```

#### Matching with Option<T>

```rust
fn plus_one(x: Option<i32>) -> Option<i32> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn main() {
    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);
}
```

#### Catch-all Patterns

```rust
fn main() {
    let dice_roll = 9;
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        other => move_player(other), // Catch-all with binding
    }
    
    // Or use _ for catch-all without binding
    match dice_roll {
        3 => add_fancy_hat(),
        7 => remove_fancy_hat(),
        _ => reroll(), // Catch-all without binding
    }
}

fn add_fancy_hat() {}
fn remove_fancy_hat() {}
fn move_player(num_spaces: u8) {}
fn reroll() {}
```

#### if let

```rust
fn main() {
    let config_max = Some(3u8);
    
    // Instead of this verbose match:
    match config_max {
        Some(max) => println!("The maximum is configured to be {}", max),
        _ => (),
    }
    
    // Use if let:
    if let Some(max) = config_max {
        println!("The maximum is configured to be {}", max);
    }
}
```

**Official Documentation**: [Pattern Matching](https://doc.rust-lang.org/book/ch06-02-match.html)

### Error Handling with Result and Option

Rust groups errors into two major categories: recoverable and unrecoverable errors.

#### Result<T, E>

```rust
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let greeting_file_result = File::open("hello.txt");
    
    let greeting_file = match greeting_file_result {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => {
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
```

#### Shortcuts for Panic on Error

```rust
use std::fs::File;

fn main() {
    // unwrap() panics if Result is Err
    let greeting_file = File::open("hello.txt").unwrap();
    
    // expect() panics with custom message
    let greeting_file = File::open("hello.txt")
        .expect("hello.txt should be included in this project");
}
```

#### Propagating Errors

```rust
use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> Result<String, io::Error> {
    let username_file_result = File::open("hello.txt");
    
    let mut username_file = match username_file_result {
        Ok(file) => file,
        Err(e) => return Err(e),
    };
    
    let mut username = String::new();
    
    match username_file.read_to_string(&mut username) {
        Ok(_) => Ok(username),
        Err(e) => Err(e),
    }
}

// Shortcut with ? operator
fn read_username_from_file_short() -> Result<String, io::Error> {
    let mut username_file = File::open("hello.txt")?;
    let mut username = String::new();
    username_file.read_to_string(&mut username)?;
    Ok(username)
}

// Even shorter
fn read_username_from_file_shorter() -> Result<String, io::Error> {
    let mut username = String::new();
    File::open("hello.txt")?.read_to_string(&mut username)?;
    Ok(username)
}
```

**Official Documentation**: [Error Handling](https://doc.rust-lang.org/book/ch09-00-error-handling.html)

### Collections

Rust's standard library includes several useful data structures called collections.

#### Vectors

```rust
fn main() {
    // Creating vectors
    let v: Vec<i32> = Vec::new();
    let v = vec![1, 2, 3]; // vec! macro
    
    // Adding elements
    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    
    // Reading elements
    let v = vec![1, 2, 3, 4, 5];
    
    let third: &i32 = &v[2]; // Panics if index out of bounds
    println!("The third element is {}", third);
    
    let third: Option<&i32> = v.get(2); // Returns None if out of bounds
    match third {
        Some(third) => println!("The third element is {}", third),
        None => println!("There is no third element."),
    }
    
    // Iterating
    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    
    // Mutable iteration
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50;
    }
}
```

#### Hash Maps

```rust
use std::collections::HashMap;

fn main() {
    // Creating hash maps
    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
    
    // From vectors
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];
    let mut scores: HashMap<_, _> =
        teams.into_iter().zip(initial_scores.into_iter()).collect();
    
    // Accessing values
    let team_name = String::from("Blue");
    let score = scores.get(&team_name).copied().unwrap_or(0);
    
    // Iterating
    for (key, value) in &scores {
        println!("{}: {}", key, value);
    }
    
    // Updating values
    scores.insert(String::from("Blue"), 25); // Overwrite
    
    // Only insert if key doesn't exist
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    
    // Update based on old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }
    println!("{:?}", map);
}
```

**Official Documentation**: [Common Collections](https://doc.rust-lang.org/book/ch08-00-common-collections.html)

### String Handling

Rust has two main string types: `String` and `&str`.

#### Creating Strings

```rust
fn main() {
    // Different ways to create strings
    let mut s = String::new();
    
    let data = "initial contents";
    let s = data.to_string();
    let s = "initial contents".to_string();
    let s = String::from("initial contents");
    
    // UTF-8 encoded
    let hello = String::from("السلام عليكم");
    let hello = String::from("Dobrý den");
    let hello = String::from("Hello");
    let hello = String::from("שָׁלוֹם");
    let hello = String::from("नमस्ते");
    let hello = String::from("こんにちは");
    let hello = String::from("안녕하세요");
    let hello = String::from("你好");
    let hello = String::from("Olá");
    let hello = String::from("Здравствуйте");
    let hello = String::from("Hola");
}
```

#### Updating Strings

```rust
fn main() {
    // push_str and push
    let mut s = String::from("foo");
    s.push_str("bar");
    s.push('!');
    
    // Concatenation with +
    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");
    let s3 = s1 + &s2; // s1 has been moved and can no longer be used
    
    // format! macro
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");
    let s = format!("{}-{}-{}", s1, s2, s3);
}
```

#### String Slices

```rust
fn main() {
    let s = String::from("hello world");
    
    let hello = &s[0..5];
    let world = &s[6..11];
    let slice = &s[0..2];
    let slice = &s[..2]; // Same as above
    let slice = &s[3..]; // From index 3 to end
    let slice = &s[..]; // Entire string
    
    // Be careful with Unicode
    let hello = "Здравствуйте";
    let s = &hello[0..4]; // This works (4 bytes = 2 Cyrillic chars)
    // let s = &hello[0..1]; // This would panic!
}

fn first_word(s: &String) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}
```

#### Iterating Over Strings

```rust
fn main() {
    // By characters
    for c in "Зд".chars() {
        println!("{}", c);
    }
    
    // By bytes
    for b in "Зд".bytes() {
        println!("{}", b);
    }
}
```

**Official Documentation**: [Storing UTF-8 Encoded Text with Strings](https://doc.rust-lang.org/book/ch08-02-strings.html)

---

## Intermediate Level Concepts

### Advanced Ownership

Building on basic ownership concepts, advanced ownership covers more complex scenarios.

#### Lifetimes in Function Signatures

```rust
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
```

#### Lifetime Annotations in Struct Definitions

```rust
struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    fn level(&self) -> i32 {
        3
    }
    
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("Attention please: {}", announcement);
        self.part
    }
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    };
}
```

**Official Documentation**: [Validating References with Lifetimes](https://doc.rust-lang.org/book/ch10-03-lifetime-syntax.html)

### Lifetimes

Lifetimes ensure that references are valid for as long as needed.

#### Lifetime Elision Rules

```rust
// These functions don't need explicit lifetime annotations
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    &s[..]
}

// Rule 1: Each parameter gets its own lifetime
// Rule 2: If there's exactly one input lifetime, it's assigned to all outputs
// Rule 3: If there's &self or &mut self, its lifetime is assigned to all outputs
```

#### Static Lifetime

```rust
fn main() {
    let s: &'static str = "I have a static lifetime.";
    // String literals always have 'static lifetime
}
```

### Traits

Traits define shared behavior in an abstract way.

#### Defining Traits

```rust
pub trait Summary {
    fn summarize(&self) -> String;
    
    // Default implementation
    fn summarize_author(&self) -> String {
        String::from("(Read more...)")
    }
}

pub struct NewsArticle {
    pub headline: String,
    pub location: String,
    pub author: String,
    pub content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
}

pub struct Tweet {
    pub username: String,
    pub content: String,
    pub reply: bool,
    pub retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}
```

#### Traits as Parameters

```rust
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}

// Trait bound syntax
pub fn notify<T: Summary>(item: &T) {
    println!("Breaking news! {}", item.summarize());
}

// Multiple trait bounds
pub fn notify(item: &(impl Summary + Display)) {
    // ...
}

pub fn notify<T: Summary + Display>(item: &T) {
    // ...
}

// where clauses for complex bounds
fn some_function<T, U>(t: &T, u: &U) -> i32
where
    T: Display + Clone,
    U: Clone + Debug,
{
    // ...
}
```

#### Returning Types that Implement Traits

```rust
fn returns_summarizable() -> impl Summary {
    Tweet {
        username: String::from("horse_ebooks"),
        content: String::from(
            "of course, as you probably already know, people",
        ),
        reply: false,
        retweet: false,
    }
}
```

**Official Documentation**: [Traits: Defining Shared Behavior](https://doc.rust-lang.org/book/ch10-02-traits.html)

### Generics

Generics allow you to write flexible, reusable code.

#### Generic Functions

```rust
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    
    for item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let result = largest(&char_list);
    println!("The largest char is {}", result);
}
```

#### Generic Structs

```rust
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// Implementation for specific type
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Multiple generic types
struct Point<T, U> {
    x: T,
    y: U,
}

impl<T, U> Point<T, U> {
    fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
        Point {
            x: self.x,
            y: other.y,
        }
    }
}

fn main() {
    let integer = Point { x: 5, y: 10 };
    let float = Point { x: 1.0, y: 4.0 };
    let mixed = Point { x: 5, y: 4.0 };
    
    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}
```

#### Generic Enums

```rust
enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

**Official Documentation**: [Generic Types, Traits, and Lifetimes](https://doc.rust-lang.org/book/ch10-00-generics.html)

---

*This document continues with Advanced and Expert level concepts...*