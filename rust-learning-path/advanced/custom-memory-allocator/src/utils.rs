//! Utility functions for memory allocation
//! 
//! This module provides helper functions for working with memory alignment,
//! pointer arithmetic, and other low-level memory operations.

use std::alloc::Layout;

/// Align a value up to the nearest multiple of `align`
/// 
/// # Safety
/// 
/// `align` must be a power of 2.
/// 
/// # Examples
/// 
/// ```
/// use custom_allocator::align_up;
/// 
/// assert_eq!(align_up(10, 8), 16);
/// assert_eq!(align_up(16, 8), 16);
/// assert_eq!(align_up(1, 4), 4);
/// ```
pub fn align_up(value: usize, align: usize) -> usize {
    debug_assert!(align.is_power_of_two(), "Alignment must be a power of 2");
    (value + align - 1) & !(align - 1)
}

/// Check if a value is aligned to the given alignment
/// 
/// # Examples
/// 
/// ```
/// use custom_allocator::is_aligned;
/// 
/// assert!(is_aligned(16, 8));
/// assert!(!is_aligned(10, 8));
/// assert!(is_aligned(0, 4));
/// ```
pub fn is_aligned(value: usize, align: usize) -> bool {
    debug_assert!(align.is_power_of_two(), "Alignment must be a power of 2");
    value & (align - 1) == 0
}

/// Calculate the size needed for a layout, including padding for alignment
pub fn layout_size_with_padding(layout: Layout) -> usize {
    align_up(layout.size(), layout.align())
}

/// Check if a pointer is properly aligned for the given layout
/// 
/// # Safety
/// 
/// This function dereferences the pointer to check alignment.
/// The pointer should be valid or null.
pub unsafe fn is_pointer_aligned(ptr: *mut u8, layout: Layout) -> bool {
    if ptr.is_null() {
        return false;
    }
    is_aligned(ptr as usize, layout.align())
}

/// Calculate the next aligned address from a given address
pub fn next_aligned_address(addr: usize, align: usize) -> usize {
    align_up(addr, align)
}

/// Calculate the padding needed to align an address
pub fn padding_needed(addr: usize, align: usize) -> usize {
    let aligned = align_up(addr, align);
    aligned - addr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_align_up() {
        assert_eq!(align_up(0, 8), 0);
        assert_eq!(align_up(1, 8), 8);
        assert_eq!(align_up(7, 8), 8);
        assert_eq!(align_up(8, 8), 8);
        assert_eq!(align_up(9, 8), 16);
        assert_eq!(align_up(15, 8), 16);
        assert_eq!(align_up(16, 8), 16);
    }

    #[test]
    fn test_is_aligned() {
        assert!(is_aligned(0, 8));
        assert!(!is_aligned(1, 8));
        assert!(!is_aligned(7, 8));
        assert!(is_aligned(8, 8));
        assert!(!is_aligned(9, 8));
        assert!(!is_aligned(15, 8));
        assert!(is_aligned(16, 8));
    }

    #[test]
    fn test_layout_size_with_padding() {
        let layout = Layout::from_size_align(10, 8).unwrap();
        assert_eq!(layout_size_with_padding(layout), 16);
        
        let layout = Layout::from_size_align(16, 8).unwrap();
        assert_eq!(layout_size_with_padding(layout), 16);
        
        let layout = Layout::from_size_align(1, 4).unwrap();
        assert_eq!(layout_size_with_padding(layout), 4);
    }

    #[test]
    fn test_next_aligned_address() {
        assert_eq!(next_aligned_address(10, 8), 16);
        assert_eq!(next_aligned_address(16, 8), 16);
        assert_eq!(next_aligned_address(1, 4), 4);
        assert_eq!(next_aligned_address(0, 8), 0);
    }

    #[test]
    fn test_padding_needed() {
        assert_eq!(padding_needed(10, 8), 6);
        assert_eq!(padding_needed(16, 8), 0);
        assert_eq!(padding_needed(1, 4), 3);
        assert_eq!(padding_needed(0, 8), 0);
    }

    #[test]
    unsafe fn test_is_pointer_aligned() {
        // Test with null pointer
        assert!(!is_pointer_aligned(std::ptr::null_mut(), Layout::from_size_align(1, 8).unwrap()));
        
        // Test with aligned addresses
        let layout = Layout::from_size_align(1, 8).unwrap();
        assert!(is_pointer_aligned(8 as *mut u8, layout));
        assert!(is_pointer_aligned(16 as *mut u8, layout));
        assert!(!is_pointer_aligned(9 as *mut u8, layout));
        assert!(!is_pointer_aligned(15 as *mut u8, layout));
    }
}