# Step-by-Step Implementation Guide

This guide walks you through implementing the C Library Binding project step by step, explaining each concept as you build it.

## Prerequisites

Before starting, ensure you have:
- Rust toolchain installed (`rustc`, `cargo`)
- C compiler (gcc, clang, or MSVC)
- Basic understanding of C programming
- Familiarity with Rust ownership and borrowing

## Step 1: Set Up the C Library

### 1.1 Create the C Header File

First, create `c-lib/mathlib.h` with function declarations:

```c
#ifndef MATHLIB_H
#define MATHLIB_H

#include <stddef.h>
#include <stdint.h>

// Mathematical operations
int32_t add_numbers(int32_t a, int32_t b);
int32_t multiply_numbers(int32_t a, int32_t b);
uint64_t factorial(uint32_t n);

// String operations
int reverse_string(const char* input, char* output, size_t output_size);
int uppercase_string(const char* input, char* output, size_t output_size);
size_t string_length(const char* str);

// Array operations
int32_t sum_array(const int32_t* array, size_t length);
int32_t find_max(const int32_t* array, size_t length, int32_t* max_value);

// Memory allocation helpers
char* allocate_string(size_t size);
void free_string(char* str);

// Error handling
const char* get_last_error(void);

#endif // MATHLIB_H
```

**Key Concepts:**
- **C ABI Compatibility**: Using standard C types (`int32_t`, `size_t`)
- **Function Signatures**: Designing C functions that are easy to call from Rust
- **Error Handling**: Using return codes and error messages

### 1.2 Implement the C Library

Create `c-lib/mathlib.c` with the function implementations:

```c
#include "mathlib.h"
#include <stdlib.h>
#include <string.h>
#include <ctype.h>

static char last_error[256] = {0};

static void set_error(const char* message) {
    strncpy(last_error, message, sizeof(last_error) - 1);
    last_error[sizeof(last_error) - 1] = '\0';
}

int32_t add_numbers(int32_t a, int32_t b) {
    return a + b;
}

// ... (implement other functions)
```

**Key Concepts:**
- **Error State Management**: Using global error state
- **Memory Safety**: Careful buffer management
- **Input Validation**: Checking for null pointers and invalid inputs

### 1.3 Test the C Library

Create `c-lib/test_mathlib.c` to verify the C library works:

```c
#include <stdio.h>
#include <assert.h>
#include "mathlib.h"

int main() {
    assert(add_numbers(5, 3) == 8);
    assert(factorial(5) == 120);
    // ... more tests
    printf("All C tests passed!\n");
    return 0;
}
```

**Build and test:**
```bash
cd c-lib
gcc -o test_mathlib test_mathlib.c mathlib.c
./test_mathlib
```

## Step 2: Create Basic FFI Declarations

### 2.1 Set Up Rust Project

Create `Cargo.toml`:

```toml
[package]
name = "c-library-binding"
version = "0.1.0"
edition = "2021"

[dependencies]
libc = "0.2"

[build-dependencies]
cc = "1.0"
```

**Key Concepts:**
- **libc crate**: Provides C-compatible types
- **cc crate**: Compiles C code as part of Rust build

### 2.2 Create Build Script

Create `build.rs`:

```rust
fn main() {
    cc::Build::new()
        .file("c-lib/mathlib.c")
        .include("c-lib")
        .compile("mathlib");
    
    println!("cargo:rustc-link-lib=static=mathlib");
    println!("cargo:rerun-if-changed=c-lib/mathlib.c");
    println!("cargo:rerun-if-changed=c-lib/mathlib.h");
}
```

**Key Concepts:**
- **Static Linking**: Embedding C library in Rust binary
- **Build Dependencies**: Automatic recompilation when C code changes

### 2.3 Declare FFI Functions

In `src/lib.rs`, declare the external C functions:

```rust
use std::os::raw::{c_char, c_int};

extern "C" {
    fn add_numbers(a: i32, b: i32) -> i32;
    fn multiply_numbers(a: i32, b: i32) -> i32;
    fn factorial(n: u32) -> u64;
    // ... more declarations
}
```

**Key Concepts:**
- **extern "C"**: Specifies C calling convention
- **Type Mapping**: Rust types that correspond to C types
- **Unsafe Operations**: FFI calls are inherently unsafe

## Step 3: Handle C Strings and Memory Management

### 3.1 String Conversion Functions

```rust
use std::ffi::{CStr, CString};

pub fn reverse(input: &str) -> Result<String, FfiError> {
    // Convert Rust string to C string
    let c_input = CString::new(input)?;
    
    // Allocate buffer for output
    let mut buffer = vec![0u8; input.len() + 1];
    
    // Call C function
    let result = unsafe {
        reverse_string(
            c_input.as_ptr(),
            buffer.as_mut_ptr() as *mut c_char,
            buffer.len(),
        )
    };
    
    // Check for errors
    if result != 0 {
        return Err(FfiError::from_c_error());
    }
    
    // Convert back to Rust string
    let c_str = unsafe { CStr::from_ptr(buffer.as_ptr() as *const c_char) };
    Ok(c_str.to_string_lossy().into_owned())
}
```

**Key Concepts:**
- **CString/CStr**: Converting between Rust and C strings
- **Memory Layout**: Ensuring buffer sizes are adequate
- **Null Termination**: C strings are null-terminated
- **Error Propagation**: Converting C errors to Rust Results

### 3.2 Memory Management Wrapper

```rust
pub struct CAllocatedString {
    ptr: *mut c_char,
}

impl CAllocatedString {
    pub fn new(size: usize) -> Result<Self, FfiError> {
        let ptr = unsafe { allocate_string(size) };
        if ptr.is_null() {
            Err(FfiError::from_c_error())
        } else {
            Ok(CAllocatedString { ptr })
        }
    }
}

impl Drop for CAllocatedString {
    fn drop(&mut self) {
        if !self.ptr.is_null() {
            unsafe { free_string(self.ptr) };
        }
    }
}
```

**Key Concepts:**
- **RAII Pattern**: Automatic resource cleanup
- **Drop Trait**: Ensuring C memory is freed
- **Null Pointer Checks**: Defensive programming

## Step 4: Work with Arrays and Pointers

### 4.1 Array Processing

```rust
pub fn sum(array: &[i32]) -> i32 {
    if array.is_empty() {
        return 0;
    }
    unsafe { sum_array(array.as_ptr(), array.len()) }
}

pub fn find_maximum(array: &[i32]) -> Result<i32, FfiError> {
    if array.is_empty() {
        return Err(FfiError::new("Cannot find maximum of empty array"));
    }

    let mut max_value: i32 = 0;
    let result = unsafe { 
        find_max(array.as_ptr(), array.len(), &mut max_value) 
    };

    if result != 0 {
        Err(FfiError::from_c_error())
    } else {
        Ok(max_value)
    }
}
```

**Key Concepts:**
- **Slice to Pointer**: Converting Rust slices to C arrays
- **Mutable References**: Passing output parameters to C
- **Length Tracking**: C doesn't know array lengths

## Step 5: Error Handling in FFI

### 5.1 Custom Error Type

```rust
#[derive(Debug, Clone)]
pub struct FfiError {
    pub message: String,
}

impl std::fmt::Display for FfiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "FFI Error: {}", self.message)
    }
}

impl std::error::Error for FfiError {}
```

### 5.2 Error Conversion

```rust
fn get_last_error_string() -> String {
    unsafe {
        let error_ptr = get_last_error();
        if error_ptr.is_null() {
            "Unknown error".to_string()
        } else {
            CStr::from_ptr(error_ptr).to_string_lossy().into_owned()
        }
    }
}
```

**Key Concepts:**
- **Error Types**: Creating domain-specific error types
- **Error Conversion**: Translating C errors to Rust errors
- **Error Propagation**: Using Result types consistently

## Step 6: Create Safe Rust Wrappers

### 6.1 Module Organization

```rust
pub mod math {
    use super::*;
    
    pub fn add(a: i32, b: i32) -> i32 {
        unsafe { add_numbers(a, b) }
    }
    
    pub fn factorial(n: u32) -> Result<u64, FfiError> {
        let result = unsafe { factorial(n) };
        if result == 0 && n > 0 {
            Err(FfiError::from_c_error())
        } else {
            Ok(result)
        }
    }
}

pub mod strings {
    // String operations...
}

pub mod arrays {
    // Array operations...
}
```

**Key Concepts:**
- **Safe Abstractions**: Hiding unsafe code behind safe interfaces
- **Module Organization**: Logical grouping of related functions
- **API Design**: Creating idiomatic Rust APIs

### 6.2 Documentation and Examples

```rust
/// Add two 32-bit integers
/// 
/// # Examples
/// ```
/// use c_library_binding::math::add;
/// assert_eq!(add(5, 3), 8);
/// ```
pub fn add(a: i32, b: i32) -> i32 {
    unsafe { add_numbers(a, b) }
}
```

**Key Concepts:**
- **Documentation**: Explaining function behavior and safety
- **Examples**: Providing usage examples in documentation
- **Testing**: Doc tests verify examples work

## Step 7: Advanced FFI Patterns

### 7.1 Thread Safety Considerations

```rust
// Only implement if C library is thread-safe
unsafe impl Send for CAllocatedString {}
unsafe impl Sync for CAllocatedString {}
```

### 7.2 Performance Optimization

```rust
// Batch operations to reduce FFI overhead
pub fn batch_sum(arrays: &[&[i32]]) -> Vec<i32> {
    arrays.iter().map(|arr| sum(arr)).collect()
}
```

**Key Concepts:**
- **Thread Safety**: Understanding C library thread safety
- **Performance**: Minimizing FFI call overhead
- **Batch Operations**: Processing multiple items efficiently

## Step 8: Testing and Validation

### 8.1 Unit Tests

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_operations() {
        assert_eq!(math::add(5, 3), 8);
        assert_eq!(math::factorial(5).unwrap(), 120);
    }

    #[test]
    fn test_error_handling() {
        assert!(math::factorial(25).is_err());
    }
}
```

### 8.2 Integration Tests

```rust
// tests/integration_tests.rs
use c_library_binding::*;

#[test]
fn test_full_workflow() {
    let input = "rust";
    let reversed = strings::reverse(input).unwrap();
    let uppercase = strings::uppercase(&reversed).unwrap();
    assert_eq!(uppercase, "TSUR");
}
```

**Key Concepts:**
- **Unit Testing**: Testing individual functions
- **Integration Testing**: Testing complete workflows
- **Error Testing**: Verifying error conditions work correctly

## Building and Running

### Build the Project

```bash
cargo build
```

### Run Tests

```bash
cargo test
```

### Run Examples

```bash
cargo run --example basic_usage
cargo run --example advanced_patterns
```

## Common Issues and Solutions

### Issue 1: Linking Errors

**Problem**: `undefined reference to 'add_numbers'`

**Solution**: Ensure build.rs compiles the C library and links it correctly.

### Issue 2: Segmentation Faults

**Problem**: Crashes when calling FFI functions

**Solution**: 
- Check pointer validity
- Ensure buffer sizes are adequate
- Verify function signatures match

### Issue 3: Memory Leaks

**Problem**: C-allocated memory not freed

**Solution**: Use RAII pattern with Drop trait to ensure cleanup.

### Issue 4: String Encoding Issues

**Problem**: Garbled text when converting strings

**Solution**: 
- Ensure C strings are null-terminated
- Handle UTF-8 encoding properly
- Use `to_string_lossy()` for robust conversion

## Next Steps

After completing this project, consider:

1. **Automatic Binding Generation**: Learn to use `bindgen` for complex C libraries
2. **Dynamic Loading**: Load C libraries at runtime using `libloading`
3. **Callbacks**: Implement C functions that call back into Rust
4. **Complex Data Structures**: Handle C structs and unions
5. **Cross-Platform Considerations**: Handle platform-specific differences

## Resources

- [Rust FFI Omnibus](http://jakegoulding.com/rust-ffi-omnibus/)
- [Rust Book - Unsafe Rust](https://doc.rust-lang.org/book/ch19-01-unsafe-rust.html)
- [Rust Reference - External Blocks](https://doc.rust-lang.org/reference/items/external-blocks.html)
- [bindgen User Guide](https://rust-lang.github.io/rust-bindgen/)

This step-by-step guide provides a comprehensive introduction to FFI in Rust while maintaining safety and following best practices.