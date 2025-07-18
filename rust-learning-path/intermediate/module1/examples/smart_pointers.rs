// Example demonstrating Rust's smart pointers
use std::cell::RefCell;
use std::rc::{Rc, Weak};
use std::sync::{Arc, Mutex};
use std::thread;

fn main() {
    // Box<T> example
    let b = Box::new(5);
    println!("Box value: {}", b);
    
    // Recursive type with Box
    let list = List::Cons(1, Box::new(List::Cons(2, Box::new(List::Nil))));
    println!("List: {:?}", list);
    
    // Rc<T> example
    let text = Rc::new(String::from("Hello, world!"));
    println!("Reference count: {}", Rc::strong_count(&text)); // 1
    
    {
        let text2 = Rc::clone(&text);
        println!("Reference count: {}", Rc::strong_count(&text)); // 2
        
        let text3 = Rc::clone(&text);
        println!("Reference count: {}", Rc::strong_count(&text)); // 3
        
        println!("Shared values: {}, {}, {}", text, text2, text3);
    }
    
    println!("Reference count after scope: {}", Rc::strong_count(&text)); // 1
    
    // RefCell<T> example
    let data = RefCell::new(5);
    
    {
        let mut data_mut = data.borrow_mut();
        *data_mut += 10;
    }
    
    println!("RefCell value: {}", data.borrow());
    
    // Combining Rc and RefCell
    let shared_list = Rc::new(RefCell::new(vec![1, 2, 3]));
    let shared_list2 = Rc::clone(&shared_list);
    
    shared_list.borrow_mut().push(4);
    shared_list2.borrow_mut().push(5);
    
    println!("Shared list: {:?}", shared_list.borrow());
    
    // Arc<T> and Mutex<T> for thread safety
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
    
    println!("Counter: {}", *counter.lock().unwrap());
    
    // Weak references
    let parent = Rc::new(Node {
        value: 5,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Weak::new()),
    });
    
    let child = Rc::new(Node {
        value: 10,
        children: RefCell::new(vec![]),
        parent: RefCell::new(Rc::downgrade(&parent)),
    });
    
    parent.children.borrow_mut().push(Rc::clone(&child));
    
    println!("Child's parent: {}", child.parent.borrow().upgrade().unwrap().value);
}

// Recursive type definition using Box
#[derive(Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

// Tree structure using Rc and Weak
struct Node {
    value: i32,
    children: RefCell<Vec<Rc<Node>>>,
    parent: RefCell<Weak<Node>>,
}

// Custom smart pointer example
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

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_box() {
        let b = Box::new(5);
        assert_eq!(*b, 5);
    }
    
    #[test]
    fn test_rc() {
        let text = Rc::new(String::from("hello"));
        assert_eq!(Rc::strong_count(&text), 1);
        
        {
            let text2 = Rc::clone(&text);
            assert_eq!(Rc::strong_count(&text), 2);
        }
        
        assert_eq!(Rc::strong_count(&text), 1);
    }
    
    #[test]
    fn test_refcell() {
        let data = RefCell::new(5);
        
        {
            let mut data_mut = data.borrow_mut();
            *data_mut += 10;
        }
        
        assert_eq!(*data.borrow(), 15);
    }
    
    #[test]
    fn test_rc_refcell() {
        let shared_list = Rc::new(RefCell::new(vec![1, 2, 3]));
        let shared_list2 = Rc::clone(&shared_list);
        
        shared_list.borrow_mut().push(4);
        shared_list2.borrow_mut().push(5);
        
        assert_eq!(*shared_list.borrow(), vec![1, 2, 3, 4, 5]);
    }
    
    #[test]
    fn test_my_box() {
        let x = 5;
        let y = MyBox::new(x);
        
        assert_eq!(5, x);
        assert_eq!(5, *y);
    }
    
    #[test]
    fn test_weak_reference() {
        let parent = Rc::new(Node {
            value: 5,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Weak::new()),
        });
        
        let child = Rc::new(Node {
            value: 10,
            children: RefCell::new(vec![]),
            parent: RefCell::new(Rc::downgrade(&parent)),
        });
        
        parent.children.borrow_mut().push(Rc::clone(&child));
        
        assert_eq!(child.parent.borrow().upgrade().unwrap().value, 5);
    }
}