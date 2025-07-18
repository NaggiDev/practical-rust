// Comprehensive tests for advanced ownership concepts
#[cfg(test)]
mod tests {
    use std::cell::RefCell;
    use std::rc::{Rc, Weak};
    use std::sync::{Arc, Mutex};
    use std::thread;
    
    // Test basic ownership and borrowing
    #[test]
    fn test_ownership_basics() {
        // Ownership transfer
        let s1 = String::from("hello");
        let s2 = s1;
        
        // s1 is no longer valid
        // Uncommenting the next line would cause a compile error
        // assert_eq!(s1, "hello");
        
        assert_eq!(s2, "hello");
        
        // Cloning instead of moving
        let s3 = String::from("world");
        let s4 = s3.clone();
        
        // Both s3 and s4 are valid
        assert_eq!(s3, "world");
        assert_eq!(s4, "world");
    }
    
    // Test borrowing rules
    #[test]
    fn test_borrowing_rules() {
        let mut value = 10;
        
        // Multiple immutable borrows are allowed
        let ref1 = &value;
        let ref2 = &value;
        assert_eq!(*ref1, 10);
        assert_eq!(*ref2, 10);
        
        // After immutable references are no longer used, we can create a mutable reference
        let ref_mut = &mut value;
        *ref_mut += 10;
        assert_eq!(value, 20);
        
        // Non-lexical lifetimes example
        let mut v = vec![1, 2, 3];
        let first = &v[0];
        assert_eq!(*first, 1);
        
        // first is no longer used, so we can mutate v
        v.push(4);
        assert_eq!(v, vec![1, 2, 3, 4]);
    }
    
    // Test lifetimes
    #[test]
    fn test_lifetimes() {
        // Basic lifetime example
        let string1 = String::from("long string is long");
        let string2 = String::from("short");
        
        let result = longest(&string1, &string2);
        assert_eq!(result, "long string is long");
        
        // Lifetime in structs
        let novel = String::from("Call me Ishmael. Some years ago...");
        let first_sentence = novel.split('.').next().expect("Could not find a '.'");
        let excerpt = Excerpt {
            part: first_sentence,
        };
        
        assert_eq!(excerpt.part, "Call me Ishmael");
    }
    
    // Function with explicit lifetime annotation
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() {
            x
        } else {
            y
        }
    }
    
    // Struct with lifetime annotation
    struct Excerpt<'a> {
        part: &'a str,
    }
    
    // Test Box<T>
    #[test]
    fn test_box() {
        // Allocate an integer on the heap
        let b = Box::new(5);
        assert_eq!(*b, 5);
        
        // Box for recursive types
        enum List {
            Cons(i32, Box<List>),
            Nil,
        }
        
        let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
        
        // Pattern matching on the boxed list
        if let List::Cons(value, _) = list {
            assert_eq!(value, 1);
        } else {
            panic!("Expected Cons variant");
        }
    }
    
    // Test Rc<T>
    #[test]
    fn test_rc() {
        // Create a reference-counted string
        let text = Rc::new(String::from("Hello, world!"));
        assert_eq!(Rc::strong_count(&text), 1);
        
        // Create a clone (increases the reference count)
        let text2 = Rc::clone(&text);
        assert_eq!(Rc::strong_count(&text), 2);
        
        // Create another clone
        let text3 = Rc::clone(&text);
        assert_eq!(Rc::strong_count(&text), 3);
        
        // text3 goes out of scope, reference count decreases
        drop(text3);
        assert_eq!(Rc::strong_count(&text), 2);
        
        // Check that both remaining references point to the same data
        assert_eq!(*text, "Hello, world!");
        assert_eq!(*text2, "Hello, world!");
    }
    
    // Test RefCell<T>
    #[test]
    fn test_refcell() {
        let data = RefCell::new(5);
        
        // Borrow mutably and modify the value
        *data.borrow_mut() += 1;
        assert_eq!(*data.borrow(), 6);
        
        // Multiple immutable borrows are allowed
        let borrow1 = data.borrow();
        let borrow2 = data.borrow();
        assert_eq!(*borrow1, 6);
        assert_eq!(*borrow2, 6);
        
        // Drop the immutable borrows
        drop(borrow1);
        drop(borrow2);
        
        // Now we can borrow mutably again
        *data.borrow_mut() += 1;
        assert_eq!(*data.borrow(), 7);
    }
    
    // Test combining Rc<T> and RefCell<T>
    #[test]
    fn test_rc_refcell() {
        let shared_data = Rc::new(RefCell::new(vec![1, 2, 3]));
        
        // Create a clone of the Rc
        let shared_data2 = Rc::clone(&shared_data);
        
        // Modify the data through the first reference
        shared_data.borrow_mut().push(4);
        
        // Modify the data through the second reference
        shared_data2.borrow_mut().push(5);
        
        // Both references see the changes
        assert_eq!(*shared_data.borrow(), vec![1, 2, 3, 4, 5]);
        assert_eq!(*shared_data2.borrow(), vec![1, 2, 3, 4, 5]);
    }
    
    // Test Arc<T> and Mutex<T>
    #[test]
    fn test_arc_mutex() {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        
        for _ in 0..5 {
            let counter = Arc::clone(&counter);
            let handle = thread::spawn(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
            handles.push(handle);
        }
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        assert_eq!(*counter.lock().unwrap(), 5);
    }
    
    // Test Weak<T>
    #[test]
    fn test_weak() {
        // A tree structure where nodes can have a parent and multiple children
        struct Node {
            value: i32,
            parent: RefCell<Weak<Node>>,
            children: RefCell<Vec<Rc<Node>>>,
        }
        
        // Create a leaf node with no parent or children
        let leaf = Rc::new(Node {
            value: 3,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![]),
        });
        
        // Verify that the leaf has no parent
        assert!(leaf.parent.borrow().upgrade().is_none());
        
        // Create a branch node with the leaf as a child
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });
        
        // Set the leaf's parent to the branch
        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);
        
        // Verify that the leaf's parent is now the branch
        assert_eq!(leaf.parent.borrow().upgrade().unwrap().value, 5);
        
        // Verify that the branch has the leaf as a child
        assert_eq!(branch.children.borrow()[0].value, 3);
    }
    
    // Test custom smart pointer with Deref trait
    #[test]
    fn test_custom_smart_pointer() {
        struct MyBox<T>(T);
        
        impl<T> MyBox<T> {
            fn new(x: T) -> MyBox<T> {
                MyBox(x)
            }
        }
        
        use std::ops::Deref;
        
        impl<T> Deref for MyBox<T> {
            type Target = T;
            
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
        
        let x = 5;
        let y = MyBox::new(x);
        
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    
    // Test interior mutability patterns
    #[test]
    fn test_interior_mutability() {
        // Using Cell for simple types
        use std::cell::Cell;
        
        let cell = Cell::new(10);
        cell.set(20);
        assert_eq!(cell.get(), 20);
        
        // Using RefCell for more complex types
        let text = RefCell::new(String::from("hello"));
        text.borrow_mut().push_str(" world");
        assert_eq!(*text.borrow(), "hello world");
        
        // Combining Rc and RefCell for shared mutable state
        let shared = Rc::new(RefCell::new(String::from("shared")));
        let shared2 = Rc::clone(&shared);
        
        shared.borrow_mut().push_str(" data");
        assert_eq!(*shared2.borrow(), "shared data");
    }
}