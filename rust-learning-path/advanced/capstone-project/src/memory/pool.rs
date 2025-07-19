//! Memory pool implementation for efficient allocation of fixed-size objects.
//!
//! This module provides memory pools that can efficiently allocate and deallocate
//! objects of the same size, reducing fragmentation and improving performance.

use crate::error::{Result, EngineError};
use std::alloc::{Layout, GlobalAlloc, System};
use std::ptr::{self, NonNull};
use std::sync::Mutex;

/// A memory pool for fixed-size allocations
pub struct MemoryPool {
    block_size: usize,
    blocks_per_chunk: usize,
    chunks: Mutex<Vec<Chunk>>,
    free_list: Mutex<Vec<NonNull<u8>>>,
    total_allocated: std::sync::atomic::AtomicUsize,
    total_freed: std::sync::atomic::AtomicUsize,
}

impl MemoryPool {
    /// Create a new memory pool
    pub fn new(block_size: usize, blocks_per_chunk: usize) -> Result<Self> {
        if block_size == 0 {
            return Err(EngineError::memory("Block size must be greater than 0"));
        }
        if blocks_per_chunk == 0 {
            return Err(EngineError::memory("Blocks per chunk must be greater than 0"));
        }

        Ok(Self {
            block_size,
            blocks_per_chunk,
            chunks: Mutex::new(Vec::new()),
            free_list: Mutex::new(Vec::new()),
            total_allocated: std::sync::atomic::AtomicUsize::new(0),
            total_freed: std::sync::atomic::AtomicUsize::new(0),
        })
    }

    /// Allocate a block from the pool
    pub fn allocate(&self) -> Result<NonNull<u8>> {
        let mut free_list = self.free_list.lock().unwrap();
        
        // Try to get a block from the free list
        if let Some(ptr) = free_list.pop() {
            self.total_allocated.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return Ok(ptr);
        }

        // No free blocks available, allocate a new chunk
        drop(free_list); // Release the lock before allocating
        self.allocate_new_chunk()?;

        // Try again with the new chunk
        let mut free_list = self.free_list.lock().unwrap();
        if let Some(ptr) = free_list.pop() {
            self.total_allocated.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            Ok(ptr)
        } else {
            Err(EngineError::memory("Failed to allocate from new chunk"))
        }
    }

    /// Deallocate a block back to the pool
    pub fn deallocate(&self, ptr: NonNull<u8>) -> Result<()> {
        // Verify the pointer belongs to one of our chunks
        if !self.owns_pointer(ptr) {
            return Err(EngineError::memory("Pointer does not belong to this pool"));
        }

        let mut free_list = self.free_list.lock().unwrap();
        free_list.push(ptr);
        self.total_freed.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        
        Ok(())
    }

    /// Get pool statistics
    pub fn statistics(&self) -> PoolStatistics {
        let chunks = self.chunks.lock().unwrap();
        let free_list = self.free_list.lock().unwrap();
        
        let total_blocks = chunks.len() * self.blocks_per_chunk;
        let free_blocks = free_list.len();
        let allocated_blocks = total_blocks - free_blocks;

        PoolStatistics {
            block_size: self.block_size,
            blocks_per_chunk: self.blocks_per_chunk,
            total_chunks: chunks.len(),
            total_blocks,
            allocated_blocks,
            free_blocks,
            total_allocated: self.total_allocated.load(std::sync::atomic::Ordering::Relaxed),
            total_freed: self.total_freed.load(std::sync::atomic::Ordering::Relaxed),
            utilization: if total_blocks > 0 {
                allocated_blocks as f64 / total_blocks as f64
            } else {
                0.0
            },
        }
    }

    /// Allocate a new chunk and add its blocks to the free list
    fn allocate_new_chunk(&self) -> Result<()> {
        let chunk_size = self.block_size * self.blocks_per_chunk;
        let layout = Layout::from_size_align(chunk_size, std::mem::align_of::<u8>())
            .map_err(|e| EngineError::memory(format!("Invalid layout: {}", e)))?;

        let chunk_ptr = unsafe { System.alloc(layout) };
        if chunk_ptr.is_null() {
            return Err(EngineError::memory("Failed to allocate chunk"));
        }

        let chunk = Chunk {
            ptr: chunk_ptr,
            layout,
        };

        // Add the chunk to our list
        {
            let mut chunks = self.chunks.lock().unwrap();
            chunks.push(chunk);
        }

        // Add all blocks in the chunk to the free list
        {
            let mut free_list = self.free_list.lock().unwrap();
            for i in 0..self.blocks_per_chunk {
                let block_ptr = unsafe { chunk_ptr.add(i * self.block_size) };
                // Safety: We just allocated this memory and calculated valid offsets
                let block_ptr = unsafe { NonNull::new_unchecked(block_ptr) };
                free_list.push(block_ptr);
            }
        }

        Ok(())
    }

    /// Check if a pointer belongs to one of our chunks
    fn owns_pointer(&self, ptr: NonNull<u8>) -> bool {
        let chunks = self.chunks.lock().unwrap();
        let ptr_addr = ptr.as_ptr() as usize;

        for chunk in chunks.iter() {
            let chunk_start = chunk.ptr as usize;
            let chunk_end = chunk_start + chunk.layout.size();
            
            if ptr_addr >= chunk_start && ptr_addr < chunk_end {
                // Check if the pointer is properly aligned to a block boundary
                let offset = ptr_addr - chunk_start;
                return offset % self.block_size == 0;
            }
        }

        false
    }
}

impl Drop for MemoryPool {
    fn drop(&mut self) {
        let chunks = self.chunks.lock().unwrap();
        for chunk in chunks.iter() {
            unsafe {
                System.dealloc(chunk.ptr, chunk.layout);
            }
        }
    }
}

// Safety: MemoryPool uses proper synchronization
unsafe impl Send for MemoryPool {}
unsafe impl Sync for MemoryPool {}

/// Represents a chunk of memory in the pool
struct Chunk {
    ptr: *mut u8,
    layout: Layout,
}

/// Statistics for a memory pool
#[derive(Debug, Clone)]
pub struct PoolStatistics {
    pub block_size: usize,
    pub blocks_per_chunk: usize,
    pub total_chunks: usize,
    pub total_blocks: usize,
    pub allocated_blocks: usize,
    pub free_blocks: usize,
    pub total_allocated: usize,
    pub total_freed: usize,
    pub utilization: f64,
}

/// A typed memory pool for specific types
pub struct TypedMemoryPool<T> {
    inner: MemoryPool,
    _phantom: std::marker::PhantomData<T>,
}

impl<T> TypedMemoryPool<T> {
    /// Create a new typed memory pool
    pub fn new(blocks_per_chunk: usize) -> Result<Self> {
        let inner = MemoryPool::new(std::mem::size_of::<T>(), blocks_per_chunk)?;
        Ok(Self {
            inner,
            _phantom: std::marker::PhantomData,
        })
    }

    /// Allocate space for one T
    pub fn allocate(&self) -> Result<NonNull<T>> {
        let ptr = self.inner.allocate()?;
        // Safety: We allocated exactly size_of::<T>() bytes with proper alignment
        Ok(ptr.cast::<T>())
    }

    /// Deallocate space for one T
    pub fn deallocate(&self, ptr: NonNull<T>) -> Result<()> {
        self.inner.deallocate(ptr.cast::<u8>())
    }

    /// Get pool statistics
    pub fn statistics(&self) -> PoolStatistics {
        self.inner.statistics()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_pool_creation() {
        let pool = MemoryPool::new(64, 10).unwrap();
        let stats = pool.statistics();
        
        assert_eq!(stats.block_size, 64);
        assert_eq!(stats.blocks_per_chunk, 10);
        assert_eq!(stats.total_chunks, 0);
        assert_eq!(stats.total_blocks, 0);
    }

    #[test]
    fn test_allocation_and_deallocation() {
        let pool = MemoryPool::new(64, 10).unwrap();
        
        // Allocate a block
        let ptr1 = pool.allocate().unwrap();
        let stats = pool.statistics();
        assert_eq!(stats.total_chunks, 1);
        assert_eq!(stats.allocated_blocks, 1);
        assert_eq!(stats.free_blocks, 9);

        // Allocate another block
        let ptr2 = pool.allocate().unwrap();
        let stats = pool.statistics();
        assert_eq!(stats.allocated_blocks, 2);
        assert_eq!(stats.free_blocks, 8);

        // Deallocate blocks
        pool.deallocate(ptr1).unwrap();
        pool.deallocate(ptr2).unwrap();
        let stats = pool.statistics();
        assert_eq!(stats.allocated_blocks, 0);
        assert_eq!(stats.free_blocks, 10);
    }

    #[test]
    fn test_multiple_chunks() {
        let pool = MemoryPool::new(32, 2).unwrap(); // Small chunks for testing
        
        // Allocate more blocks than fit in one chunk
        let ptr1 = pool.allocate().unwrap();
        let ptr2 = pool.allocate().unwrap();
        let ptr3 = pool.allocate().unwrap(); // Should trigger new chunk allocation
        
        let stats = pool.statistics();
        assert_eq!(stats.total_chunks, 2);
        assert_eq!(stats.allocated_blocks, 3);
        
        // Clean up
        pool.deallocate(ptr1).unwrap();
        pool.deallocate(ptr2).unwrap();
        pool.deallocate(ptr3).unwrap();
    }

    #[test]
    fn test_typed_memory_pool() {
        let pool: TypedMemoryPool<u64> = TypedMemoryPool::new(5).unwrap();
        
        let ptr = pool.allocate().unwrap();
        let stats = pool.statistics();
        assert_eq!(stats.block_size, std::mem::size_of::<u64>());
        assert_eq!(stats.allocated_blocks, 1);
        
        pool.deallocate(ptr).unwrap();
        let stats = pool.statistics();
        assert_eq!(stats.allocated_blocks, 0);
    }

    #[test]
    fn test_invalid_parameters() {
        assert!(MemoryPool::new(0, 10).is_err());
        assert!(MemoryPool::new(64, 0).is_err());
    }

    #[test]
    fn test_pool_statistics() {
        let pool = MemoryPool::new(128, 5).unwrap();
        
        // Allocate some blocks
        let _ptr1 = pool.allocate().unwrap();
        let _ptr2 = pool.allocate().unwrap();
        
        let stats = pool.statistics();
        assert_eq!(stats.block_size, 128);
        assert_eq!(stats.blocks_per_chunk, 5);
        assert_eq!(stats.total_chunks, 1);
        assert_eq!(stats.total_blocks, 5);
        assert_eq!(stats.allocated_blocks, 2);
        assert_eq!(stats.free_blocks, 3);
        assert_eq!(stats.total_allocated, 2);
        assert_eq!(stats.total_freed, 0);
        assert!((stats.utilization - 0.4).abs() < 0.001); // 2/5 = 0.4
    }
}