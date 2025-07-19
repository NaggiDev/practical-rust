# Rust Concepts: Custom Data Structure Project

This document explains the key Rust concepts demonstrated in the Custom Data Structure project.

## Generics

### What are Generics?

Generics allow you to write code that works with multiple types while maintaining type safety. Instead of writing separate implementations for each type, you can write one generic implementation.

```rust
// Without generics - need separate structs for each type
struct IntVec {
    data: Vec<i32>,
}

struct StringVec {
    data: Vec<String>,
}

// With generics - one struct works for all types
struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}
```

### Generic Type Parameters

The `<T>` syntax introduces a type parameter. `T` is a placeholder that gets replaced with concrete types when the generic is used:

```rust
let int_vec: MyVec<i32> = MyVec::new();     // T becomes i32
let string_vec: MyVec<String> = MyVec::new(); // T becomes String
```

### Generic Methods

Methods on generic structs can also use the type parameter:

```rust
impl<T> MyVec<T> {
    pub fn push(&mut self, item: T) {  // T is the same type as the struct
        // Implementation
    }
    
    pub fn get(&self, index: usize) -> Option<&T> {  // Returns reference to T
        // Implementation
    }
}
```

### Benefits of Generics

1. **Code Reuse**: Write once, use with many types
2. **Type Safety**: Compiler ensures type correctness
3. **Performance**: No runtime overhead (zero-cost abstractions)
4. **Expressiveness**: Clear intent about what types are expected

## Traits

### What are Traits?

Traits define shared behavior that types can implement. They're similar to interfaces in other languages but more powerful.

```rust
trait Display {
    fn fmt(&self) -> String;
}

// Any type can implement this trait
impl Display for MyVec<i32> {
    fn fmt(&self) -> String {
        format!("MyVec with {} elements", self.len())
    }
}
```

### Standard Library Traits

The project implements several important standard library traits:

#### Debug Trait
Allows printing with `{:?}` format specifier:

```rust
impl<T: fmt::Debug> fmt::Debug for MyVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries((0..self.len).map(|i| unsafe { &*self.ptr.as_ptr().add(i) }))
            .finish()
    }
}
```

Note the trait bound `T: fmt::Debug` - this means T must also implement Debug.

#### Index and IndexMut Traits
Enable array-like access with `[]` syntax:

```rust
impl<T> Index<usize> for MyVec<T> {
    type Output = T;  // Associated type
    
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("Index out of bounds")
    }
}
```

#### Drop Trait
Defines cleanup behavior when a value goes out of scope:

```rust
impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        // Clean up allocated memory and drop contained elements
    }
}
```

#### Clone Trait
Enables creating deep copies:

```rust
impl<T: Clone> Clone for MyVec<T> {
    fn clone(&self) -> Self {
        // Create a new MyVec with cloned elements
    }
}
```

### Iterator Traits

#### Iterator Trait
Defines how to iterate over a collection:

```rust
impl<'a, T> Iterator for MyVecIter<'a, T> {
    type Item = &'a T;  // What each iteration yields
    
    fn next(&mut self) -> Option<Self::Item> {
        // Return next item or None when done
    }
}
```

#### IntoIterator Trait
Converts a type into an iterator:

```rust
impl<'a, T> IntoIterator for &'a MyVec<T> {
    type Item = &'a T;
    type IntoIter = MyVecIter<'a, T>;
    
    fn into_iter(self) -> Self::IntoIter {
        MyVecIter { vec: self, index: 0 }
    }
}
```

### Trait Bounds

Trait bounds constrain generic types to only those that implement specific traits:

```rust
// T must implement Clone
impl<T: Clone> Clone for MyVec<T> {
    // ...
}

// T must implement Debug
impl<T: fmt::Debug> fmt::Debug for MyVec<T> {
    // ...
}

// Multiple bounds
fn process<T: Clone + Debug>(item: T) {
    // T must implement both Clone and Debug
}
```

## Memory Management

### Raw Pointers

The project uses raw pointers (`*mut T`) for manual memory management:

```rust
struct MyVec<T> {
    ptr: NonNull<T>,  // Non-null raw pointer
    len: usize,       // Current number of elements
    cap: usize,       // Allocated capacity
}
```

### Manual Allocation

Using `std::alloc` for memory management:

```rust
use std::alloc::{self, Layout};

// Allocate memory
let layout = Layout::array::<T>(capacity).expect("Failed to create layout");
let ptr = unsafe {
    let ptr = alloc::alloc(layout) as *mut T;
    NonNull::new(ptr).expect("Failed to allocate memory")
};

// Deallocate memory
unsafe {
    alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
}
```

### Unsafe Code

Raw pointer operations require `unsafe` blocks:

```rust
unsafe {
    // Write to memory location
    ptr::write(self.ptr.as_ptr().add(self.len), item);
    
    // Read from memory location
    ptr::read(self.ptr.as_ptr().add(self.len))
    
    // Get reference to memory location
    &*self.ptr.as_ptr().add(index)
}
```

### Memory Safety Considerations

1. **Bounds Checking**: Always verify indices are within bounds
2. **Initialization**: Only read from initialized memory locations
3. **Cleanup**: Properly drop elements and deallocate memory
4. **Aliasing**: Ensure no conflicting mutable references

## Lifetimes

### Lifetime Parameters

Lifetimes ensure references remain valid:

```rust
pub struct MyVecIter<'a, T> {
    vec: &'a MyVec<T>,  // Reference with lifetime 'a
    index: usize,
}

impl<'a, T> Iterator for MyVecIter<'a, T> {
    type Item = &'a T;  // Returned references have same lifetime
    
    fn next(&mut self) -> Option<Self::Item> {
        // Implementation ensures returned reference lives as long as 'a
    }
}
```

### Lifetime Elision

In many cases, Rust can infer lifetimes:

```rust
// Explicit lifetime
fn get<'a>(&'a self, index: usize) -> Option<&'a T>

// Elided lifetime (equivalent)
fn get(&self, index: usize) -> Option<&T>
```

## Advanced Concepts

### Associated Types

Traits can have associated types that are determined by the implementor:

```rust
trait Iterator {
    type Item;  // Associated type
    fn next(&mut self) -> Option<Self::Item>;
}

impl<'a, T> Iterator for MyVecIter<'a, T> {
    type Item = &'a T;  // Concrete type for this implementation
    // ...
}
```

### Operator Overloading

Traits enable operator overloading:

```rust
use std::ops::{Index, IndexMut};

impl<T> Index<usize> for MyVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &T {
        // Enables vec[index] syntax
    }
}
```

### Zero-Cost Abstractions

Rust's generics and traits compile to the same code as if you wrote type-specific versions manually. There's no runtime overhead for using these abstractions.

## Best Practices

1. **Use Trait Bounds**: Constrain generics appropriately
2. **Implement Standard Traits**: Make your types feel native to Rust
3. **Handle Edge Cases**: Empty collections, out-of-bounds access, etc.
4. **Memory Safety**: Always validate unsafe operations
5. **Documentation**: Document generic parameters and trait bounds
6. **Testing**: Test with multiple types to ensure generics work correctly

## Common Patterns

### Builder Pattern with Generics
```rust
impl<T> MyVec<T> {
    pub fn new() -> Self { /* ... */ }
    pub fn with_capacity(cap: usize) -> Self { /* ... */ }
}
```

### Method Chaining
```rust
let vec = MyVec::new()
    .push(1)
    .push(2)
    .push(3);
```

### Generic Functions
```rust
fn process_collection<T, C>(collection: C) 
where 
    C: IntoIterator<Item = T>,
    T: std::fmt::Debug,
{
    for item in collection {
        println!("{:?}", item);
    }
}
```

This project demonstrates how generics and traits work together to create flexible, reusable, and type-safe code that feels natural to use in Rust.