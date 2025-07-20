// Comprehensive unit tests for Advanced Level code examples
use crate::framework::{TestResult, TestSuite};
use crate::{test_case, assert_with_msg};
use std::sync::{Arc, Mutex, mpsc};
use std::thread;
use std::time::Duration;

/// Test concurrency concepts
pub fn test_concurrency_concepts() -> TestSuite {
    let mut suite = TestSuite::new("Advanced Level - Concurrency Concepts".to_string());
    
    suite.add_test(test_case!("Concurrency - Basic Threading", || {
        let handle = thread::spawn(|| {
            for i in 1..10 {
                println!("hi number {} from the spawned thread!", i);
                thread::sleep(Duration::from_millis(1));
            }
        });
        
        for i in 1..5 {
            println!("hi number {} from the main thread!", i);
            thread::sleep(Duration::from_millis(1));
        }
        
        handle.join().unwrap();
        assert_with_msg!(true, "Thread should complete successfully");
    }));
    
    suite.add_test(test_case!("Concurrency - Move Closures", || {
        let v = vec![1, 2, 3];
        
        let handle = thread::spawn(move || {
            println!("Here's a vector: {:?}", v);
            v.len()
        });
        
        let result = handle.join().unwrap();
        assert_with_msg!(result == 3, "Move closure should transfer ownership to thread");
    }));
    
    suite.add_test(test_case!("Concurrency - Message Passing", || {
        let (tx, rx) = mpsc::channel();
        
        thread::spawn(move || {
            let val = String::from("hi");
            tx.send(val).unwrap();
        });
        
        let received = rx.recv().unwrap();
        assert_with_msg!(received == "hi", "Message passing should work between threads");
    }));
    
    suite.add_test(test_case!("Concurrency - Multiple Producers", || {
        let (tx, rx) = mpsc::channel();
        
        let tx1 = tx.clone();
        thread::spawn(move || {
            let vals = vec![
                String::from("hi"),
                String::from("from"),
                String::from("the"),
                String::from("thread"),
            ];
            
            for val in vals {
                tx1.send(val).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
        });
        
        thread::spawn(move || {
            let vals = vec![
                String::from("more"),
                String::from("messages"),
                String::from("for"),
                String::from("you"),
            ];
            
            for val in vals {
                tx.send(val).unwrap();
                thread::sleep(Duration::from_millis(1));
            }
        });
        
        let mut received_count = 0;
        for received in rx {
            println!("Got: {}", received);
            received_count += 1;
            if received_count == 8 {
                break;
            }
        }
        
        assert_with_msg!(received_count == 8, "Should receive messages from multiple producers");
    }));
    
    suite.add_test(test_case!("Concurrency - Shared State with Mutex", || {
        let counter = Arc::new(Mutex::new(0));
        let mut handles = vec![];
        
        for _ in 0..10 {
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
        
        let result = *counter.lock().unwrap();
        assert_with_msg!(result == 10, "Mutex should protect shared state across threads");
    }));
    
    suite
}

/// Test unsafe Rust concepts
pub fn test_unsafe_concepts() -> TestSuite {
    let mut suite = TestSuite::new("Advanced Level - Unsafe Rust Concepts".to_string());
    
    suite.add_test(test_case!("Unsafe - Raw Pointers", || {
        let mut num = 5;
        
        let r1 = &num as *const i32;
        let r2 = &mut num as *mut i32;
        
        unsafe {
            assert_with_msg!(*r1 == 5, "Raw pointer dereferencing should work");
            *r2 = 10;
            assert_with_msg!(*r1 == 10, "Mutable raw pointer should allow modification");
        }
    }));
    
    suite.add_test(test_case!("Unsafe - Unsafe Functions", || {
        unsafe fn dangerous() -> i32 {
            42
        }
        
        unsafe {
            let result = dangerous();
            assert_with_msg!(result == 42, "Unsafe function should execute correctly");
        }
    }));
    
    suite.add_test(test_case!("Unsafe - Safe Abstraction", || {
        fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
            let len = slice.len();
            let ptr = slice.as_mut_ptr();
            
            assert!(mid <= len);
            
            unsafe {
                (
                    std::slice::from_raw_parts_mut(ptr, mid),
                    std::slice::from_raw_parts_mut(ptr.add(mid), len - mid),
                )
            }
        }
        
        let mut v = vec![1, 2, 3, 4, 5, 6];
        let (left, right) = split_at_mut(&mut v, 3);
        
        assert_with_msg!(left.len() == 3, "Left slice should have correct length");
        assert_with_msg!(right.len() == 3, "Right slice should have correct length");
        assert_with_msg!(left[0] == 1, "Left slice should contain correct elements");
        assert_with_msg!(right[0] == 4, "Right slice should contain correct elements");
    }));
    
    suite.add_test(test_case!("Unsafe - Static Variables", || {
        static mut COUNTER: usize = 0;
        
        fn add_to_count(inc: usize) {
            unsafe {
                COUNTER += inc;
            }
        }
        
        add_to_count(3);
        
        unsafe {
            assert_with_msg!(COUNTER == 3, "Static mutable variable should be modifiable");
        }
    }));
    
    suite
}

/// Test advanced trait concepts
pub fn test_advanced_trait_concepts() -> TestSuite {
    let mut suite = TestSuite::new("Advanced Level - Advanced Trait Concepts".to_string());
    
    suite.add_test(test_case!("Advanced Traits - Associated Types", || {
        trait Iterator {
            type Item;
            
            fn next(&mut self) -> Option<Self::Item>;
        }
        
        struct Counter {
            current: usize,
            max: usize,
        }
        
        impl Counter {
            fn new(max: usize) -> Counter {
                Counter { current: 0, max }
            }
        }
        
        impl Iterator for Counter {
            type Item = usize;
            
            fn next(&mut self) -> Option<Self::Item> {
                if self.current < self.max {
                    let current = self.current;
                    self.current += 1;
                    Some(current)
                } else {
                    None
                }
            }
        }
        
        let mut counter = Counter::new(3);
        assert_with_msg!(counter.next() == Some(0), "Iterator should return first item");
        assert_with_msg!(counter.next() == Some(1), "Iterator should return second item");
        assert_with_msg!(counter.next() == Some(2), "Iterator should return third item");
        assert_with_msg!(counter.next() == None, "Iterator should return None when exhausted");
    }));
    
    suite.add_test(test_case!("Advanced Traits - Operator Overloading", || {
        use std::ops::Add;
        
        #[derive(Debug, PartialEq)]
        struct Point {
            x: i32,
            y: i32,
        }
        
        impl Add for Point {
            type Output = Point;
            
            fn add(self, other: Point) -> Point {
                Point {
                    x: self.x + other.x,
                    y: self.y + other.y,
                }
            }
        }
        
        let p1 = Point { x: 1, y: 0 };
        let p2 = Point { x: 2, y: 3 };
        let p3 = p1 + p2;
        
        assert_with_msg!(p3 == Point { x: 3, y: 3 }, "Operator overloading should work");
    }));
    
    suite.add_test(test_case!("Advanced Traits - Trait Objects", || {
        trait Draw {
            fn draw(&self) -> String;
        }
        
        struct Screen {
            pub components: Vec<Box<dyn Draw>>,
        }
        
        impl Screen {
            fn run(&self) -> Vec<String> {
                self.components.iter().map(|component| component.draw()).collect()
            }
        }
        
        struct Button {
            pub width: u32,
            pub height: u32,
            pub label: String,
        }
        
        impl Draw for Button {
            fn draw(&self) -> String {
                format!("Button: {}", self.label)
            }
        }
        
        struct SelectBox {
            pub width: u32,
            pub height: u32,
            pub options: Vec<String>,
        }
        
        impl Draw for SelectBox {
            fn draw(&self) -> String {
                format!("SelectBox with {} options", self.options.len())
            }
        }
        
        let screen = Screen {
            components: vec![
                Box::new(SelectBox {
                    width: 75,
                    height: 10,
                    options: vec![
                        String::from("Yes"),
                        String::from("Maybe"),
                        String::from("No"),
                    ],
                }),
                Box::new(Button {
                    width: 50,
                    height: 10,
                    label: String::from("OK"),
                }),
            ],
        };
        
        let results = screen.run();
        assert_with_msg!(results.len() == 2, "Trait objects should allow heterogeneous collections");
        assert_with_msg!(results[0] == "SelectBox with 3 options", "First trait object should work");
        assert_with_msg!(results[1] == "Button: OK", "Second trait object should work");
    }));
    
    suite
}

/// Test macro concepts
pub fn test_macro_concepts() -> TestSuite {
    let mut suite = TestSuite::new("Advanced Level - Macro Concepts".to_string());
    
    suite.add_test(test_case!("Macros - Declarative Macros", || {
        macro_rules! vec_custom {
            ( $( $x:expr ),* ) => {
                {
                    let mut temp_vec = Vec::new();
                    $(
                        temp_vec.push($x);
                    )*
                    temp_vec
                }
            };
        }
        
        let v1 = vec_custom![1, 2, 3];
        let v2 = vec_custom![1, 2, 3, 4, 5];
        
        assert_with_msg!(v1.len() == 3, "Custom vec macro should create vector with correct length");
        assert_with_msg!(v1[0] == 1, "Custom vec macro should store elements correctly");
        assert_with_msg!(v2.len() == 5, "Custom vec macro should handle variable arguments");
    }));
    
    suite.add_test(test_case!("Macros - Pattern Matching in Macros", || {
        macro_rules! calculate {
            (eval $e:expr) => {{
                {
                    let val: usize = $e; // Force types to be integers
                    println!("{} = {}", stringify!{$e}, val);
                    val
                }
            }};
        }
        
        let result = calculate! {
            eval 1 + 2 * 3
        };
        
        assert_with_msg!(result == 7, "Macro should evaluate expression correctly");
    }));
    
    suite.add_test(test_case!("Macros - Repetition in Macros", || {
        macro_rules! find_min {
            ($x:expr) => ($x);
            ($x:expr, $($y:expr),+) => (
                std::cmp::min($x, find_min!($($y),+))
            )
        }
        
        let min = find_min!(1u32, 2, 3, 4);
        assert_with_msg!(min == 1, "Recursive macro should find minimum value");
        
        let min = find_min!(5u32, 2, 8, 1, 9);
        assert_with_msg!(min == 1, "Recursive macro should handle multiple values");
    }));
    
    suite
}

/// Test error handling patterns
pub fn test_error_handling_patterns() -> TestSuite {
    let mut suite = TestSuite::new("Advanced Level - Error Handling Patterns".to_string());
    
    suite.add_test(test_case!("Error Handling - Custom Error Types", || {
        use std::fmt;
        use std::error::Error;
        
        #[derive(Debug)]
        enum MathError {
            DivisionByZero,
            NegativeSquareRoot,
        }
        
        impl fmt::Display for MathError {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                match self {
                    MathError::DivisionByZero => write!(f, "Cannot divide by zero"),
                    MathError::NegativeSquareRoot => write!(f, "Cannot take square root of negative number"),
                }
            }
        }
        
        impl Error for MathError {}
        
        fn divide(a: f64, b: f64) -> Result<f64, MathError> {
            if b == 0.0 {
                Err(MathError::DivisionByZero)
            } else {
                Ok(a / b)
            }
        }
        
        fn sqrt(x: f64) -> Result<f64, MathError> {
            if x < 0.0 {
                Err(MathError::NegativeSquareRoot)
            } else {
                Ok(x.sqrt())
            }
        }
        
        let result = divide(10.0, 2.0);
        assert_with_msg!(result.is_ok(), "Valid division should succeed");
        assert_with_msg!((result.unwrap() - 5.0).abs() < f64::EPSILON, "Division result should be correct");
        
        let result = divide(10.0, 0.0);
        assert_with_msg!(result.is_err(), "Division by zero should fail");
        
        let result = sqrt(4.0);
        assert_with_msg!(result.is_ok(), "Valid square root should succeed");
        assert_with_msg!((result.unwrap() - 2.0).abs() < f64::EPSILON, "Square root result should be correct");
        
        let result = sqrt(-1.0);
        assert_with_msg!(result.is_err(), "Negative square root should fail");
    }));
    
    suite.add_test(test_case!("Error Handling - Error Propagation", || {
        use std::fs::File;
        use std::io::{self, Read};
        
        fn read_username_from_file() -> Result<String, io::Error> {
            let mut f = File::open("hello.txt")?;
            let mut s = String::new();
            f.read_to_string(&mut s)?;
            Ok(s)
        }
        
        // This will fail because the file doesn't exist, but that's expected
        let result = read_username_from_file();
        assert_with_msg!(result.is_err(), "Reading non-existent file should fail");
        
        // Test the ? operator with Option
        fn last_char_of_first_line(text: &str) -> Option<char> {
            text.lines().next()?.chars().last()
        }
        
        let result = last_char_of_first_line("hello\nworld");
        assert_with_msg!(result == Some('o'), "Should find last character of first line");
        
        let result = last_char_of_first_line("");
        assert_with_msg!(result == None, "Empty string should return None");
    }));
    
    suite
}