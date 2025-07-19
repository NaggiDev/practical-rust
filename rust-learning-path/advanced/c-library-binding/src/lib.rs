//! # C Library Binding
//! 
//! This module provides safe Rust bindings for the mathlib C library.
//! It demonstrates Foreign Function Interface (FFI) concepts including:
//! - Declaring external C functions
//! - Converting between Rust and C data types
//! - Memory management across language boundaries
//! - Error handling in FFI contexts

use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};
use std::ptr;

// Manual FFI declarations for the C library
extern "C" {
    fn add_numbers(a: i32, b: i32) -> i32;
    fn multiply_numbers(a: i32, b: i32) -> i32;
    fn factorial(n: u32) -> u64;
    
    fn reverse_string(input: *const c_char, output: *mut c_char, output_size: usize) -> c_int;
    fn uppercase_string(input: *const c_char, output: *mut c_char, output_size: usize) -> c_int;
    fn string_length(str: *const c_char) -> usize;
    
    fn sum_array(array: *const i32, length: usize) -> i32;
    fn find_max(array: *const i32, length: usize, max_value: *mut i32) -> c_int;
    
    fn allocate_string(size: usize) -> *mut c_char;
    fn free_string(str: *mut c_char);
    
    fn get_last_error() -> *const c_char;
}

/// Custom error type for FFI operations
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

pub type FfiResult<T> = Result<T, FfiError>;

/// Safe wrapper for mathematical operations
pub mod math {
    use super::*;

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

    /// Multiply two 32-bit integers
    /// 
    /// # Examples
    /// ```
    /// use c_library_binding::math::multiply;
    /// assert_eq!(multiply(4, 7), 28);
    /// ```
    pub fn multiply(a: i32, b: i32) -> i32 {
        unsafe { multiply_numbers(a, b) }
    }

    /// Calculate factorial of a number
    /// 
    /// Returns an error if the input is too large (> 20)
    /// 
    /// # Examples
    /// ```
    /// use c_library_binding::math::factorial;
    /// assert_eq!(factorial(5).unwrap(), 120);
    /// ```
    pub fn factorial(n: u32) -> FfiResult<u64> {
        let result = unsafe { factorial(n) };
        if result == 0 && n > 0 {
            Err(FfiError {
                message: get_last_error_string(),
            })
        } else {
            Ok(result)
        }
    }
}

/// Safe wrapper for string operations
pub mod strings {
    use super::*;

    /// Reverse a string
    /// 
    /// # Examples
    /// ```
    /// use c_library_binding::strings::reverse;
    /// assert_eq!(reverse("hello").unwrap(), "olleh");
    /// ```
    pub fn reverse(input: &str) -> FfiResult<String> {
        let c_input = CString::new(input).map_err(|_| FfiError {
            message: "Input contains null bytes".to_string(),
        })?;

        let mut buffer = vec![0u8; input.len() + 1];
        let result = unsafe {
            reverse_string(
                c_input.as_ptr(),
                buffer.as_mut_ptr() as *mut c_char,
                buffer.len(),
            )
        };

        if result != 0 {
            return Err(FfiError {
                message: get_last_error_string(),
            });
        }

        // Convert back to Rust string
        let c_str = unsafe { CStr::from_ptr(buffer.as_ptr() as *const c_char) };
        Ok(c_str.to_string_lossy().into_owned())
    }

    /// Convert string to uppercase
    /// 
    /// # Examples
    /// ```
    /// use c_library_binding::strings::uppercase;
    /// assert_eq!(uppercase("hello").unwrap(), "HELLO");
    /// ```
    pub fn uppercase(input: &str) -> FfiResult<String> {
        let c_input = CString::new(input).map_err(|_| FfiError {
            message: "Input contains null bytes".to_string(),
        })?;

        let mut buffer = vec![0u8; input.len() + 1];
        let result = unsafe {
            uppercase_string(
                c_input.as_ptr(),
                buffer.as_mut_ptr() as *mut c_char,
                buffer.len(),
            )
        };

        if result != 0 {
            return Err(FfiError {
                message: get_last_error_string(),
            });
        }

        // Convert back to Rust string
        let c_str = unsafe { CStr::from_ptr(buffer.as_ptr() as *const c_char) };
        Ok(c_str.to_string_lossy().into_owned())
    }

    /// Get the length of a string using the C library
    /// 
    /// # Examples
    /// ```
    /// use c_library_binding::strings::length;
    /// assert_eq!(length("hello").unwrap(), 5);
    /// ```
    pub fn length(input: &str) -> FfiResult<usize> {
        let c_input = CString::new(input).map_err(|_| FfiError {
            message: "Input contains null bytes".to_string(),
        })?;

        let result = unsafe { string_length(c_input.as_ptr()) };
        Ok(result)
    }
}

/// Safe wrapper for array operations
pub mod arrays {
    use super::*;

    /// Sum all elements in an array
    /// 
    /// # Examples
    /// ```
    /// use c_library_binding::arrays::sum;
    /// assert_eq!(sum(&[1, 2, 3, 4, 5]), 15);
    /// ```
    pub fn sum(array: &[i32]) -> i32 {
        if array.is_empty() {
            return 0;
        }
        unsafe { sum_array(array.as_ptr(), array.len()) }
    }

    /// Find the maximum value in an array
    /// 
    /// # Examples
    /// ```
    /// use c_library_binding::arrays::find_maximum;
    /// assert_eq!(find_maximum(&[1, 5, 3, 9, 2]).unwrap(), 9);
    /// ```
    pub fn find_maximum(array: &[i32]) -> FfiResult<i32> {
        if array.is_empty() {
            return Err(FfiError {
                message: "Cannot find maximum of empty array".to_string(),
            });
        }

        let mut max_value: i32 = 0;
        let result = unsafe { find_max(array.as_ptr(), array.len(), &mut max_value) };

        if result != 0 {
            Err(FfiError {
                message: get_last_error_string(),
            })
        } else {
            Ok(max_value)
        }
    }
}

/// Safe wrapper for memory management operations
pub mod memory {
    use super::*;

    /// A safe wrapper around C-allocated strings
    pub struct CAllocatedString {
        ptr: *mut c_char,
    }

    impl CAllocatedString {
        /// Allocate a new string with the given size
        pub fn new(size: usize) -> FfiResult<Self> {
            let ptr = unsafe { allocate_string(size) };
            if ptr.is_null() {
                Err(FfiError {
                    message: get_last_error_string(),
                })
            } else {
                Ok(CAllocatedString { ptr })
            }
        }

        /// Get a mutable pointer to the underlying C string
        /// 
        /// # Safety
        /// The caller must ensure that:
        /// - The pointer is not used after this object is dropped
        /// - The string is properly null-terminated before reading
        pub unsafe fn as_mut_ptr(&mut self) -> *mut c_char {
            self.ptr
        }

        /// Convert to a Rust string (if properly null-terminated)
        /// 
        /// # Safety
        /// The caller must ensure the C string is properly null-terminated
        pub unsafe fn to_string(&self) -> FfiResult<String> {
            if self.ptr.is_null() {
                return Err(FfiError {
                    message: "Null pointer".to_string(),
                });
            }

            let c_str = CStr::from_ptr(self.ptr);
            Ok(c_str.to_string_lossy().into_owned())
        }
    }

    impl Drop for CAllocatedString {
        fn drop(&mut self) {
            if !self.ptr.is_null() {
                unsafe { free_string(self.ptr) };
            }
        }
    }
}

/// Get the last error message from the C library
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_operations() {
        assert_eq!(math::add(5, 3), 8);
        assert_eq!(math::multiply(4, 7), 28);
        assert_eq!(math::factorial(5).unwrap(), 120);
        assert_eq!(math::factorial(0).unwrap(), 1);
        
        // Test error case
        assert!(math::factorial(25).is_err());
    }

    #[test]
    fn test_string_operations() {
        assert_eq!(strings::reverse("hello").unwrap(), "olleh");
        assert_eq!(strings::uppercase("hello").unwrap(), "HELLO");
        assert_eq!(strings::length("hello").unwrap(), 5);
        assert_eq!(strings::length("").unwrap(), 0);
    }

    #[test]
    fn test_array_operations() {
        let arr = [1, 2, 3, 4, 5];
        assert_eq!(arrays::sum(&arr), 15);
        assert_eq!(arrays::find_maximum(&arr).unwrap(), 5);
        
        let empty: &[i32] = &[];
        assert_eq!(arrays::sum(empty), 0);
        assert!(arrays::find_maximum(empty).is_err());
    }

    #[test]
    fn test_memory_management() {
        let mut c_string = memory::CAllocatedString::new(100).unwrap();
        // The CAllocatedString will be automatically freed when dropped
        assert!(!unsafe { c_string.as_mut_ptr() }.is_null());
    }
}