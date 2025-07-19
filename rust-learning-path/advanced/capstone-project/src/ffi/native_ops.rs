//! High-level native operations that combine multiple FFI calls.
//!
//! This module provides more complex operations that use multiple C library
//! functions to perform sophisticated computations.

use super::bindings;
use crate::error::{Result, EngineError};
use std::ffi::CString;
use std::os::raw::c_char;

/// Perform a series of mathematical operations on an array
pub fn complex_math_operation(numbers: Vec<i64>) -> Result<ComplexMathResult> {
    if numbers.is_empty() {
        return Err(EngineError::ffi("Input array cannot be empty".to_string()));
    }

    let mut result = ComplexMathResult::default();

    // Calculate sum using FFI
    result.sum = unsafe { bindings::fast_array_sum(numbers.as_ptr(), numbers.len()) };

    // Calculate max using FFI
    result.max = unsafe { bindings::fast_array_max(numbers.as_ptr(), numbers.len()) };
    if result.max == bindings::TASK_OPS_ERROR_NULL_POINTER as i64 {
        return Err(EngineError::ffi("Failed to calculate maximum".to_string()));
    }

    // Calculate GCD of all numbers
    result.gcd = numbers.iter().fold(numbers[0], |acc, &x| unsafe {
        bindings::fast_gcd(acc, x)
    });

    // Calculate factorial of the first number (if reasonable)
    if numbers[0] >= 0 && numbers[0] <= 20 {
        result.factorial_first = Some(unsafe { bindings::fast_factorial(numbers[0] as i32) });
        if result.factorial_first == Some(-1) {
            result.factorial_first = None;
        }
    }

    // Calculate Fibonacci of the last number (if reasonable)
    let last = numbers[numbers.len() - 1];
    if last >= 0 && last <= 50 {
        result.fibonacci_last = Some(unsafe { bindings::fast_fibonacci(last as i32) });
        if result.fibonacci_last == Some(-1) {
            result.fibonacci_last = None;
        }
    }

    // Calculate square root of the sum
    if result.sum >= 0 {
        let sqrt_result = unsafe { bindings::fast_sqrt(result.sum as f64) };
        if sqrt_result >= 0.0 {
            result.sqrt_sum = Some(sqrt_result);
        }
    }

    Ok(result)
}

/// Result of complex mathematical operations
#[derive(Debug, Clone, PartialEq)]
pub struct ComplexMathResult {
    pub sum: i64,
    pub max: i64,
    pub gcd: i64,
    pub factorial_first: Option<i64>,
    pub fibonacci_last: Option<i64>,
    pub sqrt_sum: Option<f64>,
}

impl Default for ComplexMathResult {
    fn default() -> Self {
        Self {
            sum: 0,
            max: 0,
            gcd: 0,
            factorial_first: None,
            fibonacci_last: None,
            sqrt_sum: None,
        }
    }
}

/// Perform complex string processing operations
pub fn complex_string_operation(input: String) -> Result<ComplexStringResult> {
    if input.is_empty() {
        return Err(EngineError::ffi("Input string cannot be empty".to_string()));
    }

    let mut result = ComplexStringResult::default();

    // Calculate hash of original string
    let c_string = CString::new(input.clone())
        .map_err(|e| EngineError::ffi(format!("String conversion failed: {}", e)))?;
    result.original_hash = unsafe {
        bindings::fast_string_hash(c_string.as_ptr(), input.len())
    };

    // Create reversed version
    let mut reversed_bytes = input.clone().into_bytes();
    unsafe {
        bindings::fast_string_reverse(
            reversed_bytes.as_mut_ptr() as *mut c_char,
            reversed_bytes.len(),
        );
    }
    result.reversed = String::from_utf8(reversed_bytes)
        .map_err(|e| EngineError::ffi(format!("String conversion failed: {}", e)))?;

    // Calculate hash of reversed string
    let reversed_c_string = CString::new(result.reversed.clone())
        .map_err(|e| EngineError::ffi(format!("String conversion failed: {}", e)))?;
    result.reversed_hash = unsafe {
        bindings::fast_string_hash(reversed_c_string.as_ptr(), result.reversed.len())
    };

    // Create uppercase version
    let mut uppercase_bytes = input.clone().into_bytes();
    unsafe {
        bindings::fast_string_uppercase(
            uppercase_bytes.as_mut_ptr() as *mut c_char,
            uppercase_bytes.len(),
        );
    }
    result.uppercase = String::from_utf8(uppercase_bytes)
        .map_err(|e| EngineError::ffi(format!("String conversion failed: {}", e)))?;

    // Calculate hash of uppercase string
    let uppercase_c_string = CString::new(result.uppercase.clone())
        .map_err(|e| EngineError::ffi(format!("String conversion failed: {}", e)))?;
    result.uppercase_hash = unsafe {
        bindings::fast_string_hash(uppercase_c_string.as_ptr(), result.uppercase.len())
    };

    // Check if string is palindrome (compare with reversed)
    result.is_palindrome = input.to_lowercase() == result.reversed.to_lowercase();

    Ok(result)
}

/// Result of complex string operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComplexStringResult {
    pub original_hash: u64,
    pub reversed: String,
    pub reversed_hash: u64,
    pub uppercase: String,
    pub uppercase_hash: u64,
    pub is_palindrome: bool,
}

impl Default for ComplexStringResult {
    fn default() -> Self {
        Self {
            original_hash: 0,
            reversed: String::new(),
            reversed_hash: 0,
            uppercase: String::new(),
            uppercase_hash: 0,
            is_palindrome: false,
        }
    }
}

/// Perform batch operations on multiple arrays
pub fn batch_array_operations(arrays: Vec<Vec<i64>>) -> Result<Vec<BatchArrayResult>> {
    if arrays.is_empty() {
        return Err(EngineError::ffi("Input arrays cannot be empty".to_string()));
    }

    let mut results = Vec::new();

    for (index, mut array) in arrays.into_iter().enumerate() {
        if array.is_empty() {
            return Err(EngineError::ffi(format!(
                "Array at index {} is empty",
                index
            )));
        }

        let mut result = BatchArrayResult {
            index,
            original_size: array.len(),
            ..Default::default()
        };

        // Calculate sum
        result.sum = unsafe { bindings::fast_array_sum(array.as_ptr(), array.len()) };

        // Calculate max
        result.max = unsafe { bindings::fast_array_max(array.as_ptr(), array.len()) };
        if result.max == bindings::TASK_OPS_ERROR_NULL_POINTER as i64 {
            return Err(EngineError::ffi(format!(
                "Failed to calculate max for array at index {}",
                index
            )));
        }

        // Sort the array
        unsafe {
            bindings::fast_array_sort(array.as_mut_ptr(), array.len());
        }
        result.sorted = array.clone();

        // Calculate median (middle element of sorted array)
        let mid = array.len() / 2;
        result.median = if array.len() % 2 == 0 {
            (array[mid - 1] + array[mid]) / 2
        } else {
            array[mid]
        };

        // Calculate range
        result.range = result.max - array[0]; // array[0] is min after sorting

        results.push(result);
    }

    Ok(results)
}

/// Result of batch array operations
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BatchArrayResult {
    pub index: usize,
    pub original_size: usize,
    pub sum: i64,
    pub max: i64,
    pub median: i64,
    pub range: i64,
    pub sorted: Vec<i64>,
}

impl Default for BatchArrayResult {
    fn default() -> Self {
        Self {
            index: 0,
            original_size: 0,
            sum: 0,
            max: 0,
            median: 0,
            range: 0,
            sorted: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_complex_math_operation() {
        let numbers = vec![12, 8, 15, 3, 6];
        let result = complex_math_operation(numbers).unwrap();

        assert_eq!(result.sum, 44);
        assert_eq!(result.max, 15);
        assert_eq!(result.gcd, 1); // GCD of 12, 8, 15, 3, 6
        assert!(result.factorial_first.is_some());
        assert!(result.fibonacci_last.is_some());
        assert!(result.sqrt_sum.is_some());
    }

    #[test]
    fn test_complex_string_operation() {
        let input = "hello".to_string();
        let result = complex_string_operation(input).unwrap();

        assert_eq!(result.reversed, "olleh");
        assert_eq!(result.uppercase, "HELLO");
        assert!(!result.is_palindrome);
        assert_ne!(result.original_hash, 0);
        assert_ne!(result.reversed_hash, 0);
        assert_ne!(result.uppercase_hash, 0);
    }

    #[test]
    fn test_palindrome_detection() {
        let input = "racecar".to_string();
        let result = complex_string_operation(input).unwrap();

        assert!(result.is_palindrome);
        assert_eq!(result.reversed, "racecar");
    }

    #[test]
    fn test_batch_array_operations() {
        let arrays = vec![
            vec![3, 1, 4, 1, 5],
            vec![2, 7, 1, 8, 2],
            vec![9, 6, 5, 3, 5],
        ];

        let results = batch_array_operations(arrays).unwrap();
        assert_eq!(results.len(), 3);

        // Check first array results
        assert_eq!(results[0].sum, 14);
        assert_eq!(results[0].max, 5);
        assert_eq!(results[0].sorted, vec![1, 1, 3, 4, 5]);
        assert_eq!(results[0].median, 3);
        assert_eq!(results[0].range, 4); // 5 - 1
    }

    #[test]
    fn test_empty_input_errors() {
        assert!(complex_math_operation(vec![]).is_err());
        assert!(complex_string_operation(String::new()).is_err());
        assert!(batch_array_operations(vec![]).is_err());
        assert!(batch_array_operations(vec![vec![]]).is_err());
    }
}