# Custom Memory Allocator - Rust Concepts Explained

This document provides detailed explanations of the Rust concepts demonstrated in the Custom Memory Allocator project. Understanding these concepts is crucial for working with unsafe Rust and low-level memory management.

## Table of Contents

1. [Unsafe Rust](#unsafe-rust)
2. [Raw Pointers](#raw-pointers)
3. [Memory Layout and Alignment](#memory-layout-and-alignment)
4. [The GlobalAlloc Trait](#the-globalalloc-trait)
5. [Memory Safety Invariants](#memory-safety-invariants)
6. [Atomic Operations](#atomic-operations)
7. [Thread Safety](#thread-safety)
8. [Memory Management Strategies](#memory-management-strategies)

## Unsafe Rust

Unsafe Rust allows you to bypass Rust's safety guarantees when necessary. It's required for low-level operations like memory allocation.

### Key Concepts

**Unsafe Blocks**: Code that might violate memory safety must be wrapped in `unsafe` blocks:

```rust
unsafe {
    let ptr = std::alloc::alloc(layout);
    *ptr = 42;
}
```

**Unsafe Functions**: Functions that have safety requirements must be marked `unsafe`:

```rust
unsafe fn write_to_raw_pointer(ptr: *mut u8, value: u8) {
    *ptr = value;
}
```

**Safety Documentation**: Every unsafe operation should document its safety requirements:

```rust
/// # Safety
/// 
/// The pointer must be valid and properly aligned for writes.
unsafe fn write_byte(ptr: *mut u8, value: u8) {
    *ptr = value;
}
```

### When to Use Unsafe

- Interfacing with C libraries (FFI)
- Implementing low-level data structures
- Performance-critical code that needs manual memory management
- Building safe abstractions over unsafe operations

### Safety Guidelines

1. **Minimize unsafe code**: Keep unsafe blocks as small as possible
2. **Document invariants**: Always explain what makes the code safe
3. **Test thoroughly**: Unsafe code is harder to debug
4. **Use debug assertions**: Check invariants in debug builds

## Raw Pointers

Raw pointers are Rust's equivalent to C pointers. They don't have ownership semantics and can be null.

### Types of Raw Pointers

**Immutable Raw Pointers** (`*const T`):
```rust
let x = 42;
let ptr: *const i32 = &x;
```

**Mutable Raw Pointers** (`*mut T`):
```rust
let mut x = 42;
let ptr: *mut i32 = &mut x;
```

### Operations on Raw Pointers

**Creating Raw Pointers**:
```rust
// From references
let ptr = &value as *const i32;
let ptr = &mut value as *mut i32;

// From addresses
let ptr = 0x1000 as *mut u8;

// Null pointers
let ptr = std::ptr::null_mut::<u8>();
```

**Dereferencing** (requires unsafe):
```rust
unsafe {
    let value = *ptr;
    *ptr = new_value;
}
```

**Pointer Arithmetic**:
```rust
unsafe {
    let next_ptr = ptr.add(1);        // Move forward by 1 element
    let prev_ptr = ptr.sub(1);        // Move backward by 1 element
    let byte_ptr = ptr.cast::<u8>();  // Cast to different type
}
```

### Safety Considerations

- Raw pointers can be null
- No automatic bounds checking
- No lifetime tracking
- Can create data races
- Must ensure proper alignment

## Memory Layout and Alignment

Understanding memory layout is crucial for implementing allocators correctly.

### Alignment Requirements

Different types have different alignment requirements:

```rust
use std::mem::{align_of, size_of};

println!("u8:  size={}, align={}", size_of::<u8>(), align_of::<u8>());   // 1, 1
println!("u16: size={}, align={}", size_of::<u16>(), align_of::<u16>()); // 2, 2
println!("u32: size={}, align={}", size_of::<u32>(), align_of::<u32>()); // 4, 4
println!("u64: size={}, align={}", size_of::<u64>(), align_of::<u64>()); // 8, 8
```

### Layout Struct

The `Layout` struct describes memory layout requirements:

```rust
use std::alloc::Layout;

// Create layout for a specific type
let layout = Layout::new::<u64>();

// Create layout with custom size and alignment
let layout = Layout::from_size_align(100, 8).unwrap();

// Get layout properties
println!("Size: {}, Align: {}", layout.size(), layout.align());
```

### Alignment Calculations

```rust
/// Align a value up to the nearest multiple of `align`
fn align_up(value: usize, align: usize) -> usize {
    (value + align - 1) & !(align - 1)
}

/// Check if a value is properly aligned
fn is_aligned(value: usize, align: usize) -> bool {
    value & (align - 1) == 0
}
```

### Why Alignment Matters

- **Performance**: Aligned access is faster on most architectures
- **Correctness**: Some architectures require aligned access
- **Safety**: Misaligned access can cause crashes or undefined behavior

## The GlobalAlloc Trait

The `GlobalAlloc` trait defines the interface for memory allocators in Rust.

### Trait Definition

```rust
pub unsafe trait GlobalAlloc {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8;
    unsafe fn dealloc(&self, ptr: *mut u8, layout: Layout);
    
    // Optional methods with default implementations
    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 { /* ... */ }
    unsafe fn realloc(&self, ptr: *mut u8, layout: Layout, new_size: usize) -> *mut u8 { /* ... */ }
}
```

### Implementation Requirements

**Allocation**:
- Return a pointer to memory that satisfies the layout requirements
- Return null if allocation fails
- The returned memory is uninitialized

**Deallocation**:
- The pointer must have been returned by a previous call to `alloc`
- The layout must be the same as used for allocation
- After deallocation, the pointer becomes invalid

### Safety Requirements

- Allocated memory must be properly aligned
- Allocation and deallocation must be paired
- The allocator must be thread-safe if used globally
- Memory must not be accessed after deallocation

## Memory Safety Invariants

Memory safety invariants are conditions that must always be true for safe operation.

### Common Invariants

**Pointer Validity**:
- Pointers must point to valid memory or be null
- Pointers must not be used after the memory is freed

**Alignment**:
- All pointers must be properly aligned for their type
- Misaligned access can cause undefined behavior

**Bounds**:
- Array/buffer access must be within bounds
- Pointer arithmetic must not overflow

**Lifetime**:
- Memory must not be accessed after it's freed
- References must not outlive the data they point to

### Documenting Invariants

```rust
/// A simple bump allocator
/// 
/// # Safety Invariants
/// 
/// - `memory` points to a valid memory region of size `size`
/// - `offset` is always <= `size`
/// - All returned pointers are within the memory region
/// - The memory region remains valid for the allocator's lifetime
pub struct BumpAllocator {
    memory: *mut u8,
    size: usize,
    offset: Mutex<usize>,
}
```

### Checking Invariants

Use debug assertions to check invariants:

```rust
unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
    let mut offset = self.offset.lock().unwrap();
    
    debug_assert!(*offset <= self.size, "Offset exceeds memory size");
    debug_assert!(layout.align().is_power_of_two(), "Alignment must be power of 2");
    
    // ... allocation logic
}
```

## Atomic Operations

Atomic operations provide thread-safe access to shared data without locks.

### Atomic Types

```rust
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};

let counter = AtomicUsize::new(0);
let flag = AtomicBool::new(false);
```

### Memory Ordering

Different ordering guarantees for atomic operations:

**Relaxed**: No ordering constraints, only atomicity:
```rust
counter.fetch_add(1, Ordering::Relaxed);
```

**Acquire/Release**: Synchronizes with other acquire/release operations:
```rust
counter.store(42, Ordering::Release);
let value = counter.load(Ordering::Acquire);
```

**SeqCst**: Sequential consistency, strongest guarantee:
```rust
counter.fetch_add(1, Ordering::SeqCst);
```

### Common Operations

```rust
// Load and store
let value = counter.load(Ordering::Relaxed);
counter.store(42, Ordering::Relaxed);

// Fetch and modify
let old_value = counter.fetch_add(1, Ordering::Relaxed);
let old_value = counter.fetch_sub(1, Ordering::Relaxed);

// Compare and swap
let result = counter.compare_exchange(
    expected_value,
    new_value,
    Ordering::Acquire,
    Ordering::Relaxed
);
```

## Thread Safety

Making allocators thread-safe requires careful synchronization.

### Synchronization Primitives

**Mutex**: Mutual exclusion lock:
```rust
use std::sync::Mutex;

struct ThreadSafeAllocator {
    inner: Mutex<AllocatorState>,
}

impl ThreadSafeAllocator {
    fn alloc(&self, layout: Layout) -> *mut u8 {
        let mut state = self.inner.lock().unwrap();
        // ... allocation logic
    }
}
```

**RwLock**: Reader-writer lock:
```rust
use std::sync::RwLock;

struct ReadWriteAllocator {
    stats: RwLock<Stats>,
}
```

**Atomic Operations**: Lock-free synchronization:
```rust
struct LockFreeCounter {
    count: AtomicUsize,
}

impl LockFreeCounter {
    fn increment(&self) -> usize {
        self.count.fetch_add(1, Ordering::Relaxed)
    }
}
```

### Send and Sync Traits

**Send**: Type can be transferred between threads:
```rust
unsafe impl Send for MyAllocator {}
```

**Sync**: Type can be shared between threads:
```rust
unsafe impl Sync for MyAllocator {}
```

### Thread Safety Considerations

- Avoid data races
- Minimize lock contention
- Consider lock-free alternatives for performance
- Test concurrent scenarios thoroughly

## Memory Management Strategies

Different allocation strategies have different trade-offs.

### Bump Allocator

**Advantages**:
- Very fast allocation (just increment a pointer)
- Simple implementation
- Good cache locality for sequential allocations

**Disadvantages**:
- No individual deallocation
- Memory fragmentation over time
- Not suitable for long-running programs

**Use Cases**:
- Arena allocation
- Temporary allocations
- Parsing and compilation phases

### Free List Allocator

**Advantages**:
- Supports individual deallocation
- Can reuse freed memory
- Reasonable performance for general use

**Disadvantages**:
- More complex implementation
- Potential fragmentation
- Slower than bump allocator

**Use Cases**:
- General-purpose allocation
- When deallocation patterns are unpredictable
- Long-running applications

### Pool Allocator

**Advantages**:
- Very fast for fixed-size allocations
- No fragmentation for same-size objects
- Predictable performance

**Disadvantages**:
- Only works for fixed sizes
- Memory overhead for multiple pools
- Complex for variable-size allocations

**Use Cases**:
- Object pools
- Network packet buffers
- Game engines with many similar objects

### Slab Allocator

**Advantages**:
- Optimized for specific object sizes
- Reduces fragmentation
- Good cache performance

**Disadvantages**:
- Complex implementation
- Memory overhead
- Less flexible than general allocators

**Use Cases**:
- Kernel memory management
- High-performance servers
- Systems with predictable allocation patterns

## Best Practices

### Safety First

1. **Document all unsafe code** with safety requirements
2. **Use debug assertions** to check invariants
3. **Test thoroughly**, especially edge cases and concurrent scenarios
4. **Minimize unsafe blocks** and isolate them in safe wrappers

### Performance Considerations

1. **Profile before optimizing** - measure actual performance
2. **Consider cache locality** in data structure design
3. **Minimize lock contention** in multi-threaded scenarios
4. **Use appropriate data structures** for the access pattern

### Testing Strategies

1. **Unit tests** for individual components
2. **Integration tests** for realistic scenarios
3. **Stress tests** for concurrent access
4. **Property-based tests** for invariant checking
5. **Fuzzing** for finding edge cases

### Debugging Unsafe Code

1. **Use Miri** for detecting undefined behavior
2. **Enable debug assertions** in development
3. **Use sanitizers** (AddressSanitizer, ThreadSanitizer)
4. **Add logging** for allocation/deallocation events
5. **Implement statistics** for monitoring allocator behavior

## Further Reading

- [The Rustonomicon](https://doc.rust-lang.org/nomicon/) - The Dark Arts of Unsafe Rust
- [Rust Reference - Unsafe](https://doc.rust-lang.org/reference/unsafe-blocks.html)
- [std::alloc documentation](https://doc.rust-lang.org/std/alloc/)
- [Memory Management in Rust](https://doc.rust-lang.org/book/ch04-00-understanding-ownership.html)
- [Atomic Operations in Rust](https://doc.rust-lang.org/std/sync/atomic/)

## Conclusion

Implementing a custom memory allocator teaches fundamental concepts about unsafe Rust, memory management, and systems programming. While most Rust programs don't need custom allocators, understanding these concepts helps you write better, more efficient code and gives you the tools to work at the systems level when needed.

Remember: with great power comes great responsibility. Unsafe Rust gives you the tools to build anything, but it's up to you to ensure it's safe and correct.