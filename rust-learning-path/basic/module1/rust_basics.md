# Rust Basics: Syntax, Variables, Data Types, and Control Flow

This document provides an overview of fundamental Rust concepts that form the foundation of the language. Each section includes explanations and code examples to help you understand and apply these concepts.

## Table of Contents

1. [Syntax Fundamentals](#syntax-fundamentals)
2. [Variables and Mutability](#variables-and-mutability)
3. [Data Types](#data-types)
4. [Control Flow](#control-flow)
5. [Functions](#functions)
6. [Comments and Documentation](#comments-and-documentation)

## Syntax Fundamentals

### Basic Program Structure

Every Rust program starts with a main function, which is the entry point of the program:

```rust
fn main() {
    // Your code goes here
    println!("Hello, world!");
}
```

### Statements and Expressions

Rust distinguishes between statements and expressions:

- **Statements** perform actions but don't return values
- **Expressions** evaluate to a value

```rust
fn main() {
    // This is a statement
    let x = 5;
    
    // This is an expression that returns a value
    let y = {
        let a = 3;
        a + 1  // Note: no semicolon means this is an expression that returns a value
    };
    
    println!("The value of y is: {}", y);  // Output: The value of y is: 4
}
```

### Semicolons

Semicolons (`;`) are used to terminate statements. Omitting a semicolon at the end of a block makes it an expression that returns a value:

```rust
fn main() {
    let x = 5;  // Statement, ends with semicolon
    
    let y = if x > 0 { "positive" } else { "negative" };  // Expression used in assignment
    
    println!("x is {}", y);  // Output: x is positive
}
```

## Variables and Mutability

### Variable Declaration

Variables in Rust are immutable by default:

```rust
fn main() {
    let x = 5;
    // x = 6;  // This would cause a compilation error
    println!("The value of x is: {}", x);
}
```

### Mutability

To make a variable mutable, use the `mut` keyword:

```rust
fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    
    x = 6;  // This works because x is mutable
    println!("The value of x is now: {}", x);
}
```

### Shadowing

Rust allows variable shadowing, which lets you declare a new variable with the same name as a previous variable:

```rust
fn main() {
    let x = 5;
    
    let x = x + 1;  // Shadows the previous x
    
    {
        let x = x * 2;  // Shadows x within this scope
        println!("The value of x in the inner scope is: {}", x);  // Output: 12
    }
    
    println!("The value of x is: {}", x);  // Output: 6
}
```

### Constants

Constants are always immutable and must be annotated with a type:

```rust
const MAX_POINTS: u32 = 100_000;

fn main() {
    println!("The maximum points are: {}", MAX_POINTS);
}
```

## Data Types

### Scalar Types

#### Integers

Rust has several integer types with explicit sizes:

```rust
fn main() {
    let a: i8 = 127;    // 8-bit signed integer
    let b: u8 = 255;    // 8-bit unsigned integer
    let c: i16 = 32767; // 16-bit signed integer
    let d: u16 = 65535; // 16-bit unsigned integer
    let e: i32 = 2147483647;  // 32-bit signed integer (default)
    let f: u32 = 4294967295;  // 32-bit unsigned integer
    let g: i64 = 9223372036854775807;  // 64-bit signed integer
    let h: u64 = 18446744073709551615; // 64-bit unsigned integer
    let i: i128 = 170141183460469231731687303715884105727;  // 128-bit signed integer
    let j: u128 = 340282366920938463463374607431768211455;  // 128-bit unsigned integer
    
    // isize and usize depend on the architecture (32 bits on 32-bit systems, 64 bits on 64-bit systems)
    let k: isize = 9223372036854775807;  // Pointer-sized signed integer
    let l: usize = 18446744073709551615; // Pointer-sized unsigned integer
    
    println!("Integer literals can use _ as a visual separator: {}", 1_000_000);
    
    // Integer literals can use different bases
    println!("Decimal: {}", 98_222);
    println!("Hex: {}", 0xff);
    println!("Octal: {}", 0o77);
    println!("Binary: {}", 0b1111_0000);
    println!("Byte (u8 only): {}", b'A');  // 65
}
```

#### Floating-Point Numbers

Rust has two floating-point types:

```rust
fn main() {
    let x = 2.0;      // f64 (default)
    let y: f32 = 3.0; // f32
    
    // Basic floating-point operations
    let sum = 5.0 + 10.0;       // addition
    let difference = 95.5 - 4.3; // subtraction
    let product = 4.0 * 30.0;    // multiplication
    let quotient = 56.7 / 32.2;  // division
    let remainder = 43.5 % 5.0;  // remainder
    
    println!("sum: {}, difference: {}, product: {}, quotient: {}, remainder: {}", 
             sum, difference, product, quotient, remainder);
}
```

#### Booleans

The boolean type has two values: `true` and `false`:

```rust
fn main() {
    let t = true;
    let f: bool = false;
    
    // Booleans are often used in conditionals
    if t {
        println!("This will print");
    }
    
    if !f {
        println!("This will also print");
    }
}
```

#### Characters

Rust's `char` type represents a Unicode Scalar Value:

```rust
fn main() {
    let c = 'z';
    let z: char = 'ℤ';  // Unicode character
    let heart_eyed_cat = '😻';  // Unicode emoji
    
    println!("Characters: {}, {}, {}", c, z, heart_eyed_cat);
    
    // char is 4 bytes in size and can represent more than just ASCII
    println!("Size of char: {} bytes", std::mem::size_of::<char>());
}
```

### Compound Types

#### Tuples

Tuples group together values of different types:

```rust
fn main() {
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // Destructuring a tuple
    let (x, y, z) = tup;
    println!("The values are: {}, {}, {}", x, y, z);
    
    // Accessing tuple elements with dot notation
    println!("First value: {}", tup.0);
    println!("Second value: {}", tup.1);
    println!("Third value: {}", tup.2);
    
    // Unit tuple has zero elements
    let unit = ();
}
```

#### Arrays

Arrays have a fixed length and elements of the same type:

```rust
fn main() {
    // Array with explicit type and size
    let a: [i32; 5] = [1, 2, 3, 4, 5];
    
    // Array with repeated values: [3, 3, 3, 3, 3]
    let b = [3; 5];
    
    // Accessing array elements
    println!("First element: {}", a[0]);
    println!("Second element: {}", a[1]);
    
    // Arrays are stack allocated
    println!("Array occupies {} bytes", std::mem::size_of_val(&a));
    
    // Slices are references to a portion of an array
    let slice = &a[1..3];
    println!("Slice: {:?}", slice);  // Output: [2, 3]
}
```

### Strings

Rust has two main string types: `String` and `&str`:

```rust
fn main() {
    // String literal (str slice)
    let s1 = "Hello";  // &str type
    
    // String type (growable, heap-allocated)
    let mut s2 = String::from("Hello");
    s2.push_str(", world!");
    
    println!("s1: {}", s1);
    println!("s2: {}", s2);
    
    // Converting between String and &str
    let s3 = s1.to_string();  // &str to String
    let s4: &str = &s2;       // String to &str via reference
    
    println!("s3: {}", s3);
    println!("s4: {}", s4);
}
```

## Control Flow

### If Expressions

Conditional execution with `if`, `else if`, and `else`:

```rust
fn main() {
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
    
    // If is an expression, so it can be used in a let statement
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("The value of number is: {}", number);
}
```

### Loops

Rust has three kinds of loops: `loop`, `while`, and `for`.

#### Infinite Loop with `loop`

```rust
fn main() {
    let mut counter = 0;
    
    let result = loop {
        counter += 1;
        
        if counter == 10 {
            break counter * 2;  // Return a value from the loop
        }
    };
    
    println!("The result is: {}", result);  // Output: 20
}
```

#### Conditional Loop with `while`

```rust
fn main() {
    let mut number = 3;
    
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }
    
    println!("LIFTOFF!!!");
}
```

#### Collection Iteration with `for`

```rust
fn main() {
    // Iterating over an array
    let a = [10, 20, 30, 40, 50];
    
    for element in a.iter() {
        println!("The value is: {}", element);
    }
    
    // Iterating over a range
    for number in 1..4 {  // 1, 2, 3 (exclusive upper bound)
        println!("{}!", number);
    }
    
    // Iterating over an inclusive range
    for number in 1..=3 {  // 1, 2, 3 (inclusive upper bound)
        println!("{}!", number);
    }
    
    // Counting down with rev()
    for number in (1..4).rev() {  // 3, 2, 1
        println!("{}!", number);
    }
    println!("LIFTOFF!!!");
}
```

### Match Expressions

Pattern matching with `match`:

```rust
fn main() {
    let number = 13;
    
    match number {
        // Match a single value
        1 => println!("One!"),
        
        // Match multiple values
        2 | 3 | 5 | 7 | 11 | 13 => println!("This is a prime"),
        
        // Match a range
        14..=19 => println!("A teen"),
        
        // Default case
        _ => println!("Ain't special"),
    }
    
    // Match with binding
    let boolean = true;
    let binary = match boolean {
        false => 0,
        true => 1,
    };
    println!("{} -> {}", boolean, binary);
}
```

### If Let

Simplified match for single pattern matching:

```rust
fn main() {
    let some_value = Some(3);
    
    // Using match
    match some_value {
        Some(3) => println!("three"),
        _ => (),
    }
    
    // Equivalent if let syntax
    if let Some(3) = some_value {
        println!("three");
    }
    
    // If let with else
    let another_value = Some(5);
    if let Some(x) = another_value {
        println!("Value is: {}", x);
    } else {
        println!("No value");
    }
}
```

## Functions

### Function Declaration

Functions are declared using the `fn` keyword:

```rust
fn main() {
    println!("Hello from main!");
    
    another_function();
    
    function_with_parameters(5, 'h');
    
    let result = function_with_return_value(5);
    println!("The result is: {}", result);
}

fn another_function() {
    println!("Hello from another function!");
}

fn function_with_parameters(x: i32, y: char) {
    println!("The value of x is: {}", x);
    println!("The value of y is: {}", y);
}

fn function_with_return_value(x: i32) -> i32 {
    x + 1  // Note: no semicolon means this is an expression that returns a value
}
```

### Early Return with `return`

```rust
fn main() {
    let result = absolute_value(-5);
    println!("The absolute value is: {}", result);
}

fn absolute_value(x: i32) -> i32 {
    if x < 0 {
        return -x;  // Early return
    }
    x  // Return x if it's positive or zero
}
```

## Comments and Documentation

### Regular Comments

```rust
fn main() {
    // This is a single-line comment
    
    /* This is a
       multi-line comment */
    
    let x = 5;  // Comments can appear at the end of lines
    
    // Comments can be used to explain complex code
    let y = {
        let a = 3;
        a + 1  // This expression returns 4
    };
}
```

### Documentation Comments

Documentation comments support Markdown and are used to generate documentation:

```rust
/// This function adds one to the number given.
///
/// # Examples
///
/// ```
/// let five = 5;
/// let six = add_one(5);
/// assert_eq!(6, six);
/// ```
fn add_one(x: i32) -> i32 {
    x + 1
}

/**
 * This is another style of documentation comment.
 *
 * # Examples
 *
 * ```
 * let result = complex_function(true, 42);
 * assert_eq!(result, "42 is the answer");
 * ```
 */
fn complex_function(condition: bool, number: i32) -> String {
    if condition {
        format!("{} is the answer", number)
    } else {
        format!("{} is not the answer", number)
    }
}

fn main() {
    let result = add_one(5);
    println!("5 + 1 = {}", result);
    
    let message = complex_function(true, 42);
    println!("{}", message);
}
```

## Conclusion

This document covers the fundamental syntax and concepts of Rust programming. Understanding these basics is essential for building more complex applications in Rust. As you progress through the learning path, you'll build on these concepts and learn how they work together in real-world applications.

For more detailed information, refer to:
- [The Rust Book - Common Programming Concepts](https://doc.rust-lang.org/book/ch03-00-common-programming-concepts.html)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)