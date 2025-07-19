# Step-by-Step Implementation Guide

This guide walks you through implementing the Custom Data Structure project step by step. Each step builds on the previous one, allowing you to understand the concepts gradually.

## Step 1: Basic Structure and Memory Management

### Goal
Set up the basic `MyVec<T>` struct and implement memory allocation.

### Tasks
1. Define the `MyVec<T>` struct with fields for pointer, length, and capacity
2. Implement `new()` constructor for empty vector
3. Implement `with_capacity()` constructor for pre-allocated vector
4. Add basic accessor methods: `len()`, `capacity()`, `is_empty()`

### Key Concepts
- Generic struct definition
- Raw pointers (`NonNull<T>`)
- Memory layout and allocation
- Constructor patterns

### Code to Implement
```rust
pub struct MyVec<T> {
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

impl<T> MyVec<T> {
    pub fn new() -> Self { /* TODO */ }
    pub fn with_capacity(capacity: usize) -> Self { /* TODO */ }
    pub fn len(&self) -> usize { /* TODO */ }
    pub fn capacity(&self) -> usize { /* TODO */ }
    pub fn is_empty(&self) -> bool { /* TODO */ }
}
```

### Test Your Implementation
```rust
let vec: MyVec<i32> = MyVec::new();
assert_eq!(vec.len(), 0);
assert!(vec.is_empty());

let vec_with_cap: MyVec<i32> = MyVec::with_capacity(10);
assert_eq!(vec_with_cap.capacity(), 10);
```

## Step 2: Core Operations

### Goal
Implement the fundamental operations for adding and removing elements.

### Tasks
1. Implement `push()` method to add elements
2. Implement `pop()` method to remove elements
3. Implement `grow()` helper method for dynamic resizing
4. Handle memory reallocation when capacity is exceeded

### Key Concepts
- Unsafe pointer operations
- Memory reallocation strategies
- Growth algorithms (doubling capacity)
- Writing to and reading from raw memory

### Code to Implement
```rust
impl<T> MyVec<T> {
    pub fn push(&mut self, item: T) { /* TODO */ }
    pub fn pop(&mut self) -> Option<T> { /* TODO */ }
    fn grow(&mut self) { /* TODO */ }
}
```

### Test Your Implementation
```rust
let mut vec = MyVec::new();
vec.push(1);
vec.push(2);
assert_eq!(vec.len(), 2);

assert_eq!(vec.pop(), Some(2));
assert_eq!(vec.pop(), Some(1));
assert_eq!(vec.pop(), None);
```

## Step 3: Access Operations

### Goal
Implement safe and unsafe access to elements by index.

### Tasks
1. Implement `get()` method for safe element access
2. Implement `get_mut()` method for mutable element access
3. Implement `Index` trait for `vec[index]` syntax
4. Implement `IndexMut` trait for `vec[index] = value` syntax

### Key Concepts
- Bounds checking
- Reference lifetimes
- Trait implementation
- Operator overloading
- Safe vs unsafe access patterns

### Code to Implement
```rust
impl<T> MyVec<T> {
    pub fn get(&self, index: usize) -> Option<&T> { /* TODO */ }
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> { /* TODO */ }
}

impl<T> Index<usize> for MyVec<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output { /* TODO */ }
}

impl<T> IndexMut<usize> for MyVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output { /* TODO */ }
}
```

### Test Your Implementation
```rust
let mut vec = MyVec::new();
vec.push(10);
vec.push(20);

assert_eq!(vec.get(0), Some(&10));
assert_eq!(vec.get(2), None);

vec[0] = 15;
assert_eq!(vec[0], 15);
```

## Step 4: Iterator Implementation

### Goal
Create a custom iterator to enable `for` loops and iterator methods.

### Tasks
1. Define `MyVecIter` struct for iteration state
2. Implement `Iterator` trait for the iterator
3. Implement `IntoIterator` trait for `&MyVec<T>`
4. Handle iterator lifetimes correctly

### Key Concepts
- Iterator pattern
- Lifetime parameters
- Associated types
- Iterator trait methods
- Borrowing during iteration

### Code to Implement
```rust
pub struct MyVecIter<'a, T> {
    vec: &'a MyVec<T>,
    index: usize,
}

impl<'a, T> Iterator for MyVecIter<'a, T> {
    type Item = &'a T;
    fn next(&mut self) -> Option<Self::Item> { /* TODO */ }
}

impl<'a, T> IntoIterator for &'a MyVec<T> {
    type Item = &'a T;
    type IntoIter = MyVecIter<'a, T>;
    fn into_iter(self) -> Self::IntoIter { /* TODO */ }
}
```

### Test Your Implementation
```rust
let mut vec = MyVec::new();
vec.push(1);
vec.push(2);
vec.push(3);

let collected: Vec<&i32> = vec.into_iter().collect();
assert_eq!(collected, vec![&1, &2, &3]);

for item in &vec {
    println!("{}", item);
}
```

## Step 5: Standard Library Traits

### Goal
Implement common traits to make your vector feel native to Rust.

### Tasks
1. Implement `Debug` trait for printing with `{:?}`
2. Implement `Clone` trait for deep copying
3. Implement `Drop` trait for proper cleanup
4. Handle trait bounds correctly

### Key Concepts
- Trait bounds (`T: Clone`, `T: Debug`)
- Automatic trait derivation vs manual implementation
- Resource cleanup (RAII)
- Deep vs shallow copying

### Code to Implement
```rust
impl<T: fmt::Debug> fmt::Debug for MyVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { /* TODO */ }
}

impl<T: Clone> Clone for MyVec<T> {
    fn clone(&self) -> Self { /* TODO */ }
}

impl<T> Drop for MyVec<T> {
    fn drop(&mut self) { /* TODO */ }
}
```

### Test Your Implementation
```rust
let mut vec = MyVec::new();
vec.push(1);
vec.push(2);

println!("{:?}", vec); // Should print: [1, 2]

let cloned = vec.clone();
// Both vectors should be independent
```

## Step 6: Advanced Features (Extension)

### Goal
Add more sophisticated operations to complete the vector implementation.

### Tasks
1. Implement `insert()` method for inserting at arbitrary positions
2. Implement `remove()` method for removing at arbitrary positions
3. Implement `clear()` method for removing all elements
4. Implement `extend()` method for adding multiple elements

### Key Concepts
- Element shifting in arrays
- Bulk operations
- Iterator consumption
- Performance considerations

### Code to Implement
```rust
impl<T> MyVec<T> {
    pub fn insert(&mut self, index: usize, item: T) { /* TODO */ }
    pub fn remove(&mut self, index: usize) -> T { /* TODO */ }
    pub fn clear(&mut self) { /* TODO */ }
    pub fn extend<I>(&mut self, iter: I) 
    where 
        I: IntoIterator<Item = T> 
    { /* TODO */ }
}
```

## Common Pitfalls and Solutions

### Memory Management Issues
- **Problem**: Forgetting to deallocate memory
- **Solution**: Always implement `Drop` trait properly

### Lifetime Issues
- **Problem**: Iterator references outliving the vector
- **Solution**: Use proper lifetime parameters in iterator

### Bounds Checking
- **Problem**: Accessing out-of-bounds elements
- **Solution**: Always validate indices before unsafe operations

### Generic Constraints
- **Problem**: Trying to use operations not available for all types
- **Solution**: Add appropriate trait bounds

## Testing Strategy

### Unit Tests
Test each method individually:
```rust
#[test]
fn test_push_and_len() {
    let mut vec = MyVec::new();
    vec.push(1);
    assert_eq!(vec.len(), 1);
}
```

### Integration Tests
Test combinations of operations:
```rust
#[test]
fn test_push_pop_sequence() {
    let mut vec = MyVec::new();
    vec.push(1);
    vec.push(2);
    assert_eq!(vec.pop(), Some(2));
    assert_eq!(vec.len(), 1);
}
```

### Property Tests
Test with different types:
```rust
#[test]
fn test_with_strings() {
    let mut vec = MyVec::new();
    vec.push("hello".to_string());
    assert_eq!(vec[0], "hello");
}
```

## Performance Considerations

1. **Growth Strategy**: Doubling capacity reduces amortized cost of insertions
2. **Memory Layout**: Contiguous memory improves cache performance
3. **Bounds Checking**: Use unchecked access in performance-critical code (carefully)
4. **Reallocation**: Minimize by choosing good initial capacity

## Next Steps

After completing this project, you should understand:
- How to create generic data structures
- How to implement standard library traits
- How to manage memory manually in Rust
- How iterators work in Rust
- How to write safe abstractions over unsafe code

Consider exploring:
- More complex data structures (HashMap, BTreeMap)
- Advanced iterator patterns
- Custom allocators
- Concurrent data structures