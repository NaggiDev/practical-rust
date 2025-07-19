//! Foreign Function Interface (FFI) integration with C libraries.
//!
//! This module provides safe Rust wrappers around C library functions
//! for performance-critical operations.

pub mod bindings;
pub mod native_ops;

pub use bindings::*;
pub use native_ops::*;

use crate::error::{Result, FfiError, EngineError};
use std::ffi::{CStr, CString};
use std::os::raw::{c_char, c_int};

/// Mathematical operations available through FFI
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum MathOperation {
    Factorial,
    Fibonacci,
    SquareRoot,
    GreatestCommonDivisor,
}

/// String operations available through FFI
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringOperation {
    Reverse,
    Uppercase,
    Hash,
}

/// Array operations available through FFI
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ArrayOperation {
    Sum,
    Max,
    Sort,
}

/// Safe wrapper for mathematical operations
pub fn execute_math_operation(op: MathOperation, args: &[i64]) -> Result<i64> {
    match op {
        MathOperation::Factorial => {
            if args.len() != 1 {
                return Err(EngineError::ffi("Factorial requires exactly 1 argument".to_string()));
            }
            let n = args[0];
            if n < 0 || n > 20 {
                return Err(EngineError::ffi("Factorial argument must be between 0 and 20".to_string()));
            }
            
            let result = unsafe { bindings::fast_factorial(n as i32) };
            if result < 0 {
                Err(EngineError::ffi("Factorial calculation failed".to_string()))
            } else {
                Ok(result)
            }
        }
        
        MathOperation::Fibonacci => {
            if args.len() != 1 {
                return Err(EngineError::ffi("Fibonacci requires exactly 1 argument".to_string()));
            }
            let n = args[0];
            if n < 0 {
                return Err(EngineError::ffi("Fibonacci argument must be non-negative".to_string()));
            }
            
            let result = unsafe { bindings::fast_fibonacci(n as i32) };
            if result < 0 {
                Err(EngineError::ffi("Fibonacci calculation failed".to_string()))
            } else {
                Ok(result)
            }
        }
        
        MathOperation::SquareRoot => {
            if args.len() != 1 {
                return Err(EngineError::ffi("SquareRoot requires exactly 1 argument".to_string()));
            }
            let x = args[0] as f64;
            if x < 0.0 {
                return Err(EngineError::ffi("SquareRoot argument must be non-negative".to_string()));
            }
            
            let result = unsafe { bindings::fast_sqrt(x) };
            if result < 0.0 {
                Err(EngineError::ffi("SquareRoot calculation failed".to_string()))
            } else {
                Ok(result as i64)
            }
        }
        
        MathOperation::GreatestCommonDivisor => {
            if args.len() != 2 {
                return Err(EngineError::ffi("GCD requires exactly 2 arguments".to_string()));
            }
            let a = args[0];
            let b = args[1];
            
            let result = unsafe { bindings::fast_gcd(a, b) };
            Ok(result)
        }
    }
}

/// Safe wrapper for string operations
pub fn execute_string_operation(op: StringOperation, input: String) -> Result<String> {
    match op {
        StringOperation::Reverse => {
            let mut chars: Vec<char> = input.chars().collect();
            let len = chars.len();
            
            // Convert to bytes for C function
            let mut bytes: Vec<u8> = input.into_bytes();
            
            unsafe {
                bindings::fast_string_reverse(bytes.as_mut_ptr() as *mut c_char, len);
            }
            
            // Convert back to String
            String::from_utf8(bytes)
                .map_err(|e| EngineError::ffi(format!("String conversion failed: {}", e)))
        }
        
        StringOperation::Uppercase => {
            let mut bytes: Vec<u8> = input.into_bytes();
            let len = bytes.len();
            
            unsafe {
                bindings::fast_string_uppercase(bytes.as_mut_ptr() as *mut c_char, len);
            }
            
            String::from_utf8(bytes)
                .map_err(|e| EngineError::ffi(format!("String conversion failed: {}", e)))
        }
        
        StringOperation::Hash => {
            let c_string = CString::new(input.clone())
                .map_err(|e| EngineError::ffi(format!("String conversion failed: {}", e)))?;
            
            let hash = unsafe {
                bindings::fast_string_hash(c_string.as_ptr(), input.len())
            };
            
            Ok(hash.to_string())
        }
    }
}

/// Safe wrapper for array operations
pub fn execute_array_operation(op: ArrayOperation, mut array: Vec<i64>) -> Result<i64> {
    if array.is_empty() {
        return Err(EngineError::ffi("Array cannot be empty".to_string()));
    }
    
    match op {
        ArrayOperation::Sum => {
            let result = unsafe {
                bindings::fast_array_sum(array.as_ptr(), array.len())
            };
            Ok(result)
        }
        
        ArrayOperation::Max => {
            let result = unsafe {
                bindings::fast_array_max(array.as_ptr(), array.len())
            };
            if result == bindings::TASK_OPS_ERROR_NULL_POINTER as i64 {
                Err(EngineError::ffi("Array max calculation failed".to_string()))
            } else {
                Ok(result)
            }
        }
        
        ArrayOperation::Sort => {
            unsafe {
                bindings::fast_array_sort(array.as_mut_ptr(), array.len());
            }
            // Return the first element of the sorted array
            Ok(array[0])
        }
    }
}

/// Utility function to convert C error codes to Rust errors
pub fn check_ffi_result(result: c_int, operation: &str) -> Result<()> {
    match result {
        bindings::TASK_OPS_SUCCESS => Ok(()),
        bindings::TASK_OPS_ERROR_NULL_POINTER => Err(EngineError::from(FfiError {
            operation: operation.to_string(),
            code: result,
            message: "Null pointer error".to_string(),
        })),
        bindings::TASK_OPS_ERROR_INVALID_SIZE => Err(EngineError::from(FfiError {
            operation: operation.to_string(),
            code: result,
            message: "Invalid size error".to_string(),
        })),
        bindings::TASK_OPS_ERROR_OVERFLOW => Err(EngineError::from(FfiError {
            operation: operation.to_string(),
            code: result,
            message: "Overflow error".to_string(),
        })),
        _ => Err(EngineError::from(FfiError {
            operation: operation.to_string(),
            code: result,
            message: "Unknown error".to_string(),
        })),
    }
}