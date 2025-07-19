//! High-Performance Data Processing Library
//! 
//! This library provides optimized data processing capabilities using various
//! performance optimization techniques including SIMD operations, memory-mapped
//! files, and parallel processing.

pub mod processor;
pub mod simd_ops;
pub mod memory_map;
pub mod parallel;

pub use processor::{DataProcessor, ProcessingConfig, ProcessingResult};
pub use memory_map::MemoryMappedProcessor;
pub use simd_ops::SimdOperations;
pub use parallel::ParallelProcessor;

/// Re-export commonly used types
pub use anyhow::{Result, Error};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_library_exports() {
        // Ensure all main types are accessible
        let _config = ProcessingConfig::default();
    }
}