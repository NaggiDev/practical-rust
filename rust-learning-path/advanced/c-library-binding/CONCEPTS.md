# FFI (Foreign Function Interface) Concepts

This document explains the key concepts demonstrated in the C Library Binding project.

## Table of Contents

1. [Foreign Function Interface (FFI)](#foreign-function-interface-ffi)
2. [C ABI Compatibility](#c-abi-compatibility)
3. [Memory Management Across Language Boundaries](#memory-management-across-language-boundaries)
4. [Type Conversion Between Rust and C](#type-conversion-between-rust-and-c)
5. [Error Handling in FFI](#error-handling-in-ffi)
6. [Safety Considerations](#safety-considerations)
7. [Build System Integration](#build-system-integration)
8. [Advanced FFI Patterns](#advanced-ffi-patterns)

## Foreign Function Interface (FFI)

### What is FFI?

Foreign Function Interface (FFI) is a mechanism that allows code written in one programming language to call functions written in another language. In Rust's case, FFI primarily refers to calling C functions from Rust code, though it can also work with other languages that use C-compatible ABIs.

### Why Use FFI?

1. **Legacy Integration**: Use existing C libraries without rewriting them
2. **Performance**: Access highly optimized C libraries
3. **System APIs**: Interface with operating system APIs
4. **Ecosystem**: Leverage the vast ecosystem of C libraries

### Basic FFI Declaration

```rust
extern "C" {
    fn add_numbers(a: i32, b: i32) -> i32;
}
```

The `extern "C"` block tells Rust that these functions use the C calling convention and are defined externally.

## C ABI Compatibility

### Application Binary Interface (ABI)

The ABI defines how functions are called at the binary level, including:
- How parameters are passed (registers vs stack)
- How return values are handled
- How the stack is managed
- Name mangling conventions

### C Calling Convention

Rust uses `extern "C"` to specify the C calling convention:
- Parameters are passed according to the platform's C ABI
- No name mangling is applied
- Compatible with C function signatures

### Data Layout Compatibility

Rust types must match C types in memory layout:

```rust
// Rust type -> C type
i32 -> int32_t
u32 -> uint32_t
*const c_char -> const char*
*mut c_char -> char*
```

## Memory Management Across Language Boundaries

### The Challenge

Rust's ownership system doesn't extend across FFI boundaries. When calling C functions:
- Rust can't track C-allocated memory
- C can't understand Rust's ownership rules
- Manual memory management becomes necessary

### Memory Allocation Patterns

#### 1. Rust Allocates, C Uses

```rust
let data = vec![1, 2, 3, 4, 5];
unsafe {
    process_array(data.as_ptr(), data.len());
}
// Rust automatically frees the data
```

#### 2. C Allocates, Rust Uses

```rust
unsafe {
    let ptr = allocate_string(100);
    // Use the string...
    free_string(ptr); // Must manually free
}
```

#### 3. Safe Wrapper Pattern

```rust
pub struct CAllocatedString {
    ptr: *mut c_char,
}

impl Drop for CAllocatedString {
    fn drop(&mut self) {
        unsafe { free_string(self.ptr); }
    }
}
```

### String Handling

C strings are null-terminated, while Rust strings are UTF-8 with length:

```rust
// Rust to C
let rust_string = "Hello";
let c_string = CString::new(rust_string).unwrap();
unsafe { c_function(c_string.as_ptr()); }

// C to Rust
unsafe {
    let c_str = CStr::from_ptr(c_ptr);
    let rust_string = c_str.to_string_lossy().into_owned();
}
```

## Type Conversion Between Rust and C

### Primitive Types

| Rust Type | C Type | Notes |
|-----------|--------|-------|
| `i8` | `int8_t` | 8-bit signed integer |
| `u8` | `uint8_t` | 8-bit unsigned integer |
| `i32` | `int32_t` | 32-bit signed integer |
| `u32` | `uint32_t` | 32-bit unsigned integer |
| `f32` | `float` | 32-bit floating point |
| `f64` | `double` | 64-bit floating point |

### Pointer Types

```rust
*const T    // const T* in C (read-only)
*mut T      // T* in C (mutable)
```

### Array Handling

```rust
// Passing Rust slice to C
let slice = &[1, 2, 3, 4, 5];
unsafe {
    c_function(slice.as_ptr(), slice.len());
}
```

### Complex Types

For complex types, you often need to define C-compatible representations:

```rust
#[repr(C)]
struct Point {
    x: f64,
    y: f64,
}
```

## Error Handling in FFI

### C Error Conventions

C libraries typically use several error reporting mechanisms:

1. **Return Codes**: Functions return 0 for success, non-zero for error
2. **Null Pointers**: Return NULL on failure
3. **Global Error State**: Set `errno` or similar global variable
4. **Out Parameters**: Use pointer parameters to return error information

### Rust Error Handling

Convert C errors to Rust's `Result` type:

```rust
pub fn safe_c_function(input: i32) -> Result<i32, FfiError> {
    let result = unsafe { c_function(input) };
    if result < 0 {
        Err(FfiError::from_c_error())
    } else {
        Ok(result)
    }
}
```

### Error Propagation

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

## Safety Considerations

### Unsafe Code Requirements

FFI operations are inherently unsafe because:
- Rust can't verify C code correctness
- Memory safety isn't guaranteed
- Type safety isn't enforced

### Safety Guidelines

1. **Validate Inputs**: Check for null pointers, valid ranges
2. **Handle Errors**: Always check C function return values
3. **Memory Management**: Ensure proper allocation/deallocation pairing
4. **Thread Safety**: Verify C library thread safety before concurrent use
5. **Buffer Overflows**: Ensure buffer sizes are adequate

### Safe Wrapper Pattern

```rust
pub fn safe_wrapper(input: &str) -> Result<String, FfiError> {
    // Validate input
    let c_input = CString::new(input)?;
    
    // Allocate output buffer
    let mut buffer = vec![0u8; input.len() + 1];
    
    // Call unsafe C function
    let result = unsafe {
        c_function(c_input.as_ptr(), buffer.as_mut_ptr(), buffer.len())
    };
    
    // Check for errors
    if result != 0 {
        return Err(FfiError::from_code(result));
    }
    
    // Convert result safely
    let c_str = unsafe { CStr::from_ptr(buffer.as_ptr()) };
    Ok(c_str.to_string_lossy().into_owned())
}
```

## Build System Integration

### Using cc Crate

The `cc` crate allows compiling C code as part of the Rust build:

```rust
// build.rs
cc::Build::new()
    .file("c-lib/mathlib.c")
    .include("c-lib")
    .compile("mathlib");
```

### Linking Libraries

```rust
// Link static library
println!("cargo:rustc-link-lib=static=mathlib");

// Link dynamic library
println!("cargo:rustc-link-lib=dylib=mathlib");

// Add library search path
println!("cargo:rustc-link-search=native=/path/to/libs");
```

### Build Dependencies

```toml
[build-dependencies]
cc = "1.0"          # For compiling C code
bindgen = "0.69"    # For generating bindings
```

### Automatic Binding Generation

Using `bindgen` to automatically generate Rust bindings:

```rust
// build.rs
let bindings = bindgen::Builder::default()
    .header("c-lib/mathlib.h")
    .parse_callbacks(Box::new(bindgen::CargoCallbacks))
    .generate()
    .expect("Unable to generate bindings");

bindings
    .write_to_file(out_path.join("bindings.rs"))
    .expect("Couldn't write bindings!");
```

## Advanced FFI Patterns

### Callbacks

Allowing C code to call back into Rust:

```rust
// Define callback type
type Callback = extern "C" fn(i32) -> i32;

// Rust function that can be called from C
extern "C" fn rust_callback(value: i32) -> i32 {
    value * 2
}

// Pass callback to C
unsafe {
    c_function_with_callback(rust_callback);
}
```

### Complex Data Structures

```rust
#[repr(C)]
struct ComplexStruct {
    id: u32,
    name: [c_char; 64],
    values: *mut f64,
    count: usize,
}
```

### Thread Safety

```rust
// Ensure C library is thread-safe
unsafe impl Send for CWrapper {}
unsafe impl Sync for CWrapper {}
```

### Dynamic Loading

Loading libraries at runtime:

```rust
use libloading::{Library, Symbol};

let lib = Library::new("./libmathlib.so")?;
let func: Symbol<unsafe extern fn(i32, i32) -> i32> = lib.get(b"add_numbers")?;
let result = unsafe { func(5, 3) };
```

## Best Practices

1. **Minimize Unsafe Code**: Keep unsafe blocks as small as possible
2. **Validate Everything**: Check all inputs and outputs
3. **Document Safety Requirements**: Clearly document safety invariants
4. **Test Thoroughly**: Include edge cases and error conditions
5. **Use Safe Wrappers**: Provide safe, idiomatic Rust APIs
6. **Handle Errors Gracefully**: Convert C errors to Rust Result types
7. **Manage Memory Carefully**: Ensure proper allocation/deallocation
8. **Consider Alternatives**: Sometimes rewriting in Rust is better than FFI

## Common Pitfalls

1. **Memory Leaks**: Forgetting to free C-allocated memory
2. **Use After Free**: Using pointers after memory is freed
3. **Buffer Overflows**: Not checking buffer sizes
4. **Null Pointer Dereference**: Not checking for null pointers
5. **Thread Safety**: Assuming C libraries are thread-safe
6. **String Encoding**: Mixing UTF-8 and C string encodings
7. **ABI Mismatches**: Incorrect type mappings between Rust and C

## Conclusion

FFI is a powerful feature that allows Rust to interoperate with existing C libraries and system APIs. However, it requires careful attention to safety, memory management, and error handling. By following best practices and using safe wrapper patterns, you can create robust and safe FFI bindings that leverage the strengths of both Rust and C.