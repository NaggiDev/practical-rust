use std::alloc::{self, Layout};
use std::fmt;
use std::ops::{Index, IndexMut};
use std::ptr::{self, NonNull};

/// A custom dynamic array implementation demonstrating traits and generics
/// 
/// This is a simplified version of Vec<T> that shows how to:
/// - Work with generic types
/// - Implement standard library traits
/// - Manage memory manually
/// - Create custom iterators
pub struct MyVec<T> {
    // TODO: Define the struct fields
    // Hint: You'll need to store:
    // - A pointer to the data
    // - The current length
    // - The current capacity
    ptr: NonNull<T>,
    len: usize,
    cap: usize,
}

impl<T> MyVec<T> {
    /// Creates a new, empty MyVec
    pub fn new() -> Self {
        // TODO: Implement this method
        // Hint: Start with zero capacity and a dangling pointer
        Self {
            ptr: NonNull::dangling(),
            len: 0,
            cap: 0,
        }
    }

    /// Creates a new MyVec with the specified capacity
    pub fn with_capacity(capacity: usize) -> Self {
        // TODO: Implement this method
        // Hint: Allocate memory for the specified capacity
        if capacity == 0 {
            Self::new()
        } else {
            let layout = Layout::array::<T>(capacity).expect("Failed to create layout");
            let ptr = unsafe {
                let ptr = alloc::alloc(layout) as *mut T;
                NonNull::new(ptr).expect("Failed to allocate memory")
            };
            Self {
                ptr,
                len: 0,
                cap: capacity,
            }
        }
    }

    /// Returns the number of elements in the vector
    pub fn len(&self) -> usize {
        // TODO: Implement this method
        self.len
    }

    /// Returns the capacity of the vector
    pub fn capacity(&self) -> usize {
        // TODO: Implement this method
        self.cap
    }

    /// Returns true if the vector is empty
    pub fn is_empty(&self) -> bool {
        // TODO: Implement this method
        self.len == 0
    }

    /// Adds an element to the end of the vector
    pub fn push(&mut self, item: T) {
        // TODO: Implement this method
        // Hint: Check if you need to grow the capacity first
        if self.len == self.cap {
            self.grow();
        }
        
        unsafe {
            ptr::write(self.ptr.as_ptr().add(self.len), item);
        }
        self.len += 1;
    }

    /// Removes and returns the last element, or None if empty
    pub fn pop(&mut self) -> Option<T> {
        // TODO: Implement this method
        if self.len == 0 {
            None
        } else {
            self.len -= 1;
            unsafe {
                Some(ptr::read(self.ptr.as_ptr().add(self.len)))
            }
        }
    }

    /// Gets a reference to the element at the given index
    pub fn get(&self, index: usize) -> Option<&T> {
        // TODO: Implement this method
        if index < self.len {
            unsafe {
                Some(&*self.ptr.as_ptr().add(index))
            }
        } else {
            None
        }
    }

    /// Gets a mutable reference to the element at the given index
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        // TODO: Implement this method
        if index < self.len {
            unsafe {
                Some(&mut *self.ptr.as_ptr().add(index))
            }
        } else {
            None
        }
    }

    /// Grows the capacity of the vector
    fn grow(&mut self) {
        // TODO: Implement this method
        // Hint: Double the capacity, or start with 1 if capacity is 0
        let new_cap = if self.cap == 0 { 1 } else { self.cap * 2 };
        
        let new_layout = Layout::array::<T>(new_cap).expect("Failed to create layout");
        
        let new_ptr = if self.cap == 0 {
            unsafe {
                let ptr = alloc::alloc(new_layout) as *mut T;
                NonNull::new(ptr).expect("Failed to allocate memory")
            }
        } else {
            let old_layout = Layout::array::<T>(self.cap).expect("Failed to create old layout");
            unsafe {
                let ptr = alloc::realloc(self.ptr.as_ptr() as *mut u8, old_layout, new_layout.size()) as *mut T;
                NonNull::new(ptr).expect("Failed to reallocate memory")
            }
        };
        
        self.ptr = new_ptr;
        self.cap = new_cap;
    }
}

// TODO: Implement the Index trait for MyVec
// This allows using vec[index] syntax
impl<T> Index<usize> for MyVec<T> {
    type Output = T;
    
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("Index out of bounds")
    }
}

// TODO: Implement the IndexMut trait for MyVec
// This allows using vec[index] = value syntax
impl<T> IndexMut<usize> for MyVec<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("Index out of bounds")
    }
}

// TODO: Implement the Debug trait for MyVec
// This allows printing the vector with {:?}
impl<T: fmt::Debug> fmt::Debug for MyVec<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list()
            .entries((0..self.len).map(|i| unsafe { &*self.ptr.as_ptr().add(i) }))
            .finish()
    }
}

// TODO: Implement the Drop trait for MyVec
// This ensures proper cleanup when the vector is dropped
impl<T> Drop for MyVec<T> {
    fn drop(&mut self) {
        if self.cap != 0 {
            // Drop all elements
            for i in 0..self.len {
                unsafe {
                    ptr::drop_in_place(self.ptr.as_ptr().add(i));
                }
            }
            
            // Deallocate memory
            let layout = Layout::array::<T>(self.cap).expect("Failed to create layout");
            unsafe {
                alloc::dealloc(self.ptr.as_ptr() as *mut u8, layout);
            }
        }
    }
}

// TODO: Implement Clone for MyVec where T: Clone
impl<T: Clone> Clone for MyVec<T> {
    fn clone(&self) -> Self {
        let mut new_vec = Self::with_capacity(self.len);
        for i in 0..self.len {
            unsafe {
                let item = &*self.ptr.as_ptr().add(i);
                new_vec.push(item.clone());
            }
        }
        new_vec
    }
}

/// Custom iterator for MyVec
pub struct MyVecIter<'a, T> {
    vec: &'a MyVec<T>,
    index: usize,
}

impl<'a, T> Iterator for MyVecIter<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.vec.len {
            let item = unsafe { &*self.vec.ptr.as_ptr().add(self.index) };
            self.index += 1;
            Some(item)
        } else {
            None
        }
    }
}

// TODO: Implement IntoIterator for &MyVec<T>
impl<'a, T> IntoIterator for &'a MyVec<T> {
    type Item = &'a T;
    type IntoIter = MyVecIter<'a, T>;
    
    fn into_iter(self) -> Self::IntoIter {
        MyVecIter {
            vec: self,
            index: 0,
        }
    }
}

fn main() {
    println!("=== Custom Data Structure Demo ===\n");

    // Create a new vector
    let mut vec = MyVec::new();
    println!("Created empty vector: {:?}", vec);
    println!("Length: {}, Capacity: {}", vec.len(), vec.capacity());

    // Add some elements
    println!("\nAdding elements...");
    vec.push(1);
    vec.push(2);
    vec.push(3);
    vec.push(4);
    vec.push(5);
    
    println!("After pushing 5 elements: {:?}", vec);
    println!("Length: {}, Capacity: {}", vec.len(), vec.capacity());

    // Access elements
    println!("\nAccessing elements:");
    println!("vec[0] = {}", vec[0]);
    println!("vec[2] = {}", vec[2]);
    
    // Modify elements
    vec[1] = 10;
    println!("After setting vec[1] = 10: {:?}", vec);

    // Pop elements
    println!("\nPopping elements:");
    while let Some(item) = vec.pop() {
        println!("Popped: {}", item);
        println!("Vector now: {:?}", vec);
    }

    // Demonstrate with different types
    println!("\n=== String Vector Demo ===");
    let mut string_vec = MyVec::new();
    string_vec.push("Hello".to_string());
    string_vec.push("World".to_string());
    string_vec.push("Rust".to_string());
    
    println!("String vector: {:?}", string_vec);
    
    // Iterate over the vector
    println!("\nIterating over string vector:");
    for (i, item) in string_vec.into_iter().enumerate() {
        println!("  [{}]: {}", i, item);
    }

    // Clone demonstration
    println!("\n=== Clone Demo ===");
    let mut original = MyVec::new();
    original.push(1);
    original.push(2);
    original.push(3);
    
    let cloned = original.clone();
    println!("Original: {:?}", original);
    println!("Cloned: {:?}", cloned);
    
    original.push(4);
    println!("After modifying original:");
    println!("Original: {:?}", original);
    println!("Cloned: {:?}", cloned);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_vector() {
        let vec: MyVec<i32> = MyVec::new();
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 0);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_with_capacity() {
        let vec: MyVec<i32> = MyVec::with_capacity(10);
        assert_eq!(vec.len(), 0);
        assert_eq!(vec.capacity(), 10);
        assert!(vec.is_empty());
    }

    #[test]
    fn test_push_and_len() {
        let mut vec = MyVec::new();
        assert_eq!(vec.len(), 0);
        
        vec.push(1);
        assert_eq!(vec.len(), 1);
        
        vec.push(2);
        assert_eq!(vec.len(), 2);
    }

    #[test]
    fn test_pop() {
        let mut vec = MyVec::new();
        assert_eq!(vec.pop(), None);
        
        vec.push(1);
        vec.push(2);
        
        assert_eq!(vec.pop(), Some(2));
        assert_eq!(vec.pop(), Some(1));
        assert_eq!(vec.pop(), None);
    }

    #[test]
    fn test_indexing() {
        let mut vec = MyVec::new();
        vec.push(10);
        vec.push(20);
        vec.push(30);
        
        assert_eq!(vec[0], 10);
        assert_eq!(vec[1], 20);
        assert_eq!(vec[2], 30);
        
        vec[1] = 25;
        assert_eq!(vec[1], 25);
    }

    #[test]
    fn test_get() {
        let mut vec = MyVec::new();
        vec.push(1);
        vec.push(2);
        
        assert_eq!(vec.get(0), Some(&1));
        assert_eq!(vec.get(1), Some(&2));
        assert_eq!(vec.get(2), None);
    }

    #[test]
    fn test_get_mut() {
        let mut vec = MyVec::new();
        vec.push(1);
        vec.push(2);
        
        if let Some(item) = vec.get_mut(0) {
            *item = 10;
        }
        
        assert_eq!(vec[0], 10);
    }

    #[test]
    fn test_growth() {
        let mut vec = MyVec::new();
        let initial_capacity = vec.capacity();
        
        // Push enough elements to trigger growth
        for i in 0..10 {
            vec.push(i);
        }
        
        assert!(vec.capacity() > initial_capacity);
        assert_eq!(vec.len(), 10);
    }

    #[test]
    fn test_iterator() {
        let mut vec = MyVec::new();
        vec.push(1);
        vec.push(2);
        vec.push(3);
        
        let collected: Vec<&i32> = vec.into_iter().collect();
        assert_eq!(collected, vec![&1, &2, &3]);
    }

    #[test]
    fn test_clone() {
        let mut original = MyVec::new();
        original.push(1);
        original.push(2);
        original.push(3);
        
        let cloned = original.clone();
        
        assert_eq!(original.len(), cloned.len());
        for i in 0..original.len() {
            assert_eq!(original[i], cloned[i]);
        }
    }

    #[test]
    fn test_different_types() {
        let mut string_vec = MyVec::new();
        string_vec.push("hello".to_string());
        string_vec.push("world".to_string());
        
        assert_eq!(string_vec.len(), 2);
        assert_eq!(string_vec[0], "hello");
        assert_eq!(string_vec[1], "world");
    }
}