//! Raw FFI bindings to the C library.
//!
//! This module contains the unsafe extern "C" declarations that directly
//! correspond to the C library functions.

use std::os::raw::{c_char, c_int, c_void};

// Error codes from C library
pub const TASK_OPS_SUCCESS: c_int = 0;
pub const TASK_OPS_ERROR_NULL_POINTER: c_int = -1;
pub const TASK_OPS_ERROR_INVALID_SIZE: c_int = -2;
pub const TASK_OPS_ERROR_OVERFLOW: c_int = -3;

extern "C" {
    // Mathematical operations
    pub fn fast_factorial(n: i32) -> i64;
    pub fn fast_fibonacci(n: i32) -> i64;
    pub fn fast_sqrt(x: f64) -> f64;
    pub fn fast_gcd(a: i64, b: i64) -> i64;

    // Array operations
    pub fn fast_array_sum(arr: *const i64, len: usize) -> i64;
    pub fn fast_array_max(arr: *const i64, len: usize) -> i64;
    pub fn fast_array_sort(arr: *mut i64, len: usize);

    // String operations
    pub fn fast_string_reverse(str: *mut c_char, len: usize);
    pub fn fast_string_uppercase(str: *mut c_char, len: usize);
    pub fn fast_string_hash(str: *const c_char, len: usize) -> u64;

    // Memory operations
    pub fn fast_memory_copy(dest: *mut c_void, src: *const c_void, len: usize);
    pub fn fast_memory_compare(a: *const c_void, b: *const c_void, len: usize) -> c_int;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_factorial() {
        unsafe {
            assert_eq!(fast_factorial(0), 1);
            assert_eq!(fast_factorial(1), 1);
            assert_eq!(fast_factorial(5), 120);
            assert_eq!(fast_factorial(-1), -1); // Error case
        }
    }

    #[test]
    fn test_fibonacci() {
        unsafe {
            assert_eq!(fast_fibonacci(0), 0);
            assert_eq!(fast_fibonacci(1), 1);
            assert_eq!(fast_fibonacci(10), 55);
            assert_eq!(fast_fibonacci(-1), -1); // Error case
        }
    }

    #[test]
    fn test_sqrt() {
        unsafe {
            assert!((fast_sqrt(4.0) - 2.0).abs() < 0.001);
            assert!((fast_sqrt(9.0) - 3.0).abs() < 0.001);
            assert_eq!(fast_sqrt(-1.0), -1.0); // Error case
        }
    }

    #[test]
    fn test_gcd() {
        unsafe {
            assert_eq!(fast_gcd(12, 8), 4);
            assert_eq!(fast_gcd(17, 13), 1);
            assert_eq!(fast_gcd(-12, 8), 4);
        }
    }

    #[test]
    fn test_array_operations() {
        let mut arr = vec![3, 1, 4, 1, 5, 9, 2, 6];
        
        unsafe {
            // Test sum
            let sum = fast_array_sum(arr.as_ptr(), arr.len());
            assert_eq!(sum, 31);

            // Test max
            let max = fast_array_max(arr.as_ptr(), arr.len());
            assert_eq!(max, 9);

            // Test sort
            fast_array_sort(arr.as_mut_ptr(), arr.len());
            assert_eq!(arr, vec![1, 1, 2, 3, 4, 5, 6, 9]);
        }
    }

    #[test]
    fn test_string_operations() {
        // Test string reverse
        let mut test_str = b"hello".to_vec();
        unsafe {
            fast_string_reverse(test_str.as_mut_ptr() as *mut c_char, test_str.len());
        }
        assert_eq!(test_str, b"olleh");

        // Test string uppercase
        let mut test_str = b"hello".to_vec();
        unsafe {
            fast_string_uppercase(test_str.as_mut_ptr() as *mut c_char, test_str.len());
        }
        assert_eq!(test_str, b"HELLO");

        // Test string hash
        let test_str = CString::new("hello").unwrap();
        unsafe {
            let hash1 = fast_string_hash(test_str.as_ptr(), 5);
            let hash2 = fast_string_hash(test_str.as_ptr(), 5);
            assert_eq!(hash1, hash2); // Same string should produce same hash
            assert_ne!(hash1, 0); // Hash should not be zero for non-empty string
        }
    }

    #[test]
    fn test_memory_operations() {
        let src = vec![1u8, 2, 3, 4, 5];
        let mut dest = vec![0u8; 5];

        unsafe {
            fast_memory_copy(
                dest.as_mut_ptr() as *mut c_void,
                src.as_ptr() as *const c_void,
                5,
            );
        }
        assert_eq!(dest, src);

        unsafe {
            let result = fast_memory_compare(
                src.as_ptr() as *const c_void,
                dest.as_ptr() as *const c_void,
                5,
            );
            assert_eq!(result, 0); // Should be equal
        }
    }
}