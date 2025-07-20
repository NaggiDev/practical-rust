// Comprehensive unit tests for Intermediate Level code examples
use crate::framework::{TestResult, TestSuite};
use crate::{test_case, assert_with_msg};
use std::collections::HashMap;

/// Test advanced ownership concepts
pub fn test_ownership_concepts() -> TestSuite {
    let mut suite = TestSuite::new("Intermediate Level - Ownership Concepts".to_string());
    
    suite.add_test(test_case!("Ownership - Borrowing", || {
        fn calculate_length(s: &String) -> usize {
            s.len()
        }
        
        let s1 = String::from("hello");
        let len = calculate_length(&s1);
        
        assert_with_msg!(len == 5, "Borrowing should allow reading without taking ownership");
        assert_with_msg!(s1 == "hello", "Original string should still be accessible after borrowing");
    }));
    
    suite.add_test(test_case!("Ownership - Mutable Borrowing", || {
        fn change(s: &mut String) {
            s.push_str(", world");
        }
        
        let mut s = String::from("hello");
        change(&mut s);
        
        assert_with_msg!(s == "hello, world", "Mutable borrowing should allow modification");
    }));
    
    suite.add_test(test_case!("Ownership - Multiple Immutable Borrows", || {
        let s1 = String::from("hello");
        let r1 = &s1;
        let r2 = &s1;
        
        assert_with_msg!(*r1 == "hello", "First immutable borrow should work");
        assert_with_msg!(*r2 == "hello", "Second immutable borrow should work");
        assert_with_msg!(s1 == "hello", "Original should still be accessible");
    }));
    
    suite.add_test(test_case!("Ownership - Slice References", || {
        fn first_word(s: &str) -> &str {
            let bytes = s.as_bytes();
            
            for (i, &item) in bytes.iter().enumerate() {
                if item == b' ' {
                    return &s[0..i];
                }
            }
            
            &s[..]
        }
        
        let s = String::from("hello world");
        let word = first_word(&s);
        
        assert_with_msg!(word == "hello", "String slice should return first word");
        
        let s = String::from("hello");
        let word = first_word(&s);
        assert_with_msg!(word == "hello", "String slice should return entire string if no space");
    }));
    
    suite.add_test(test_case!("Ownership - Lifetimes", || {
        fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
            if x.len() > y.len() {
                x
            } else {
                y
            }
        }
        
        let string1 = String::from("long string is long");
        let string2 = "xyz";
        
        let result = longest(string1.as_str(), string2);
        assert_with_msg!(result == "long string is long", "Lifetime annotation should allow returning longer string");
    }));
    
    suite
}

/// Test struct and enum concepts
pub fn test_struct_enum_concepts() -> TestSuite {
    let mut suite = TestSuite::new("Intermediate Level - Structs and Enums".to_string());
    
    suite.add_test(test_case!("Structs - Basic Struct", || {
        #[derive(Debug, PartialEq)]
        struct User {
            username: String,
            email: String,
            sign_in_count: u64,
            active: bool,
        }
        
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        
        assert_with_msg!(user1.username == "someusername123", "Struct field access should work");
        assert_with_msg!(user1.email == "someone@example.com", "Struct should store all fields correctly");
        assert_with_msg!(user1.active == true, "Boolean fields should work in structs");
        assert_with_msg!(user1.sign_in_count == 1, "Numeric fields should work in structs");
    }));
    
    suite.add_test(test_case!("Structs - Struct Update Syntax", || {
        #[derive(Debug, PartialEq)]
        struct User {
            username: String,
            email: String,
            sign_in_count: u64,
            active: bool,
        }
        
        let user1 = User {
            email: String::from("someone@example.com"),
            username: String::from("someusername123"),
            active: true,
            sign_in_count: 1,
        };
        
        let user2 = User {
            email: String::from("another@example.com"),
            username: String::from("anotherusername567"),
            ..user1
        };
        
        assert_with_msg!(user2.email == "another@example.com", "Updated fields should have new values");
        assert_with_msg!(user2.username == "anotherusername567", "Updated fields should override");
        assert_with_msg!(user2.active == true, "Struct update syntax should copy remaining fields");
        assert_with_msg!(user2.sign_in_count == 1, "Struct update syntax should preserve other fields");
    }));
    
    suite.add_test(test_case!("Structs - Tuple Structs", || {
        #[derive(Debug, PartialEq)]
        struct Color(i32, i32, i32);
        
        #[derive(Debug, PartialEq)]
        struct Point(i32, i32, i32);
        
        let black = Color(0, 0, 0);
        let origin = Point(0, 0, 0);
        
        assert_with_msg!(black.0 == 0, "Tuple struct field access should work");
        assert_with_msg!(black.1 == 0, "Tuple struct should store multiple values");
        assert_with_msg!(black.2 == 0, "Tuple struct indexing should work for all fields");
        
        // Different types even with same structure
        // This would not compile: assert_with_msg!(black == origin, "Different tuple struct types");
        assert_with_msg!(black == Color(0, 0, 0), "Same tuple struct types should be equal");
    }));
    
    suite.add_test(test_case!("Structs - Methods", || {
        #[derive(Debug)]
        struct Rectangle {
            width: u32,
            height: u32,
        }
        
        impl Rectangle {
            fn area(&self) -> u32 {
                self.width * self.height
            }
            
            fn can_hold(&self, other: &Rectangle) -> bool {
                self.width > other.width && self.height > other.height
            }
            
            fn square(size: u32) -> Rectangle {
                Rectangle {
                    width: size,
                    height: size,
                }
            }
        }
        
        let rect1 = Rectangle {
            width: 30,
            height: 50,
        };
        
        let rect2 = Rectangle {
            width: 10,
            height: 40,
        };
        
        let square = Rectangle::square(25);
        
        assert_with_msg!(rect1.area() == 1500, "Method should calculate area correctly");
        assert_with_msg!(rect1.can_hold(&rect2), "Method should compare rectangles correctly");
        assert_with_msg!(!rect2.can_hold(&rect1), "Method should handle reverse comparison");
        assert_with_msg!(square.width == 25, "Associated function should create square");
        assert_with_msg!(square.height == 25, "Associated function should set both dimensions");
    }));
    
    suite.add_test(test_case!("Enums - Basic Enum", || {
        #[derive(Debug, PartialEq)]
        enum IpAddrKind {
            V4,
            V6,
        }
        
        let four = IpAddrKind::V4;
        let six = IpAddrKind::V6;
        
        assert_with_msg!(four == IpAddrKind::V4, "Enum variants should be comparable");
        assert_with_msg!(six == IpAddrKind::V6, "Enum should support multiple variants");
    }));
    
    suite.add_test(test_case!("Enums - Enum with Data", || {
        #[derive(Debug, PartialEq)]
        enum IpAddr {
            V4(u8, u8, u8, u8),
            V6(String),
        }
        
        let home = IpAddr::V4(127, 0, 0, 1);
        let loopback = IpAddr::V6(String::from("::1"));
        
        match home {
            IpAddr::V4(a, b, c, d) => {
                assert_with_msg!(a == 127, "Enum should store tuple data correctly");
                assert_with_msg!(b == 0, "Enum tuple data should be accessible");
                assert_with_msg!(c == 0, "All enum tuple fields should be available");
                assert_with_msg!(d == 1, "Enum destructuring should work");
            }
            _ => panic!("Should match V4 variant"),
        }
        
        match loopback {
            IpAddr::V6(addr) => {
                assert_with_msg!(addr == "::1", "Enum should store string data correctly");
            }
            _ => panic!("Should match V6 variant"),
        }
    }));
    
    suite.add_test(test_case!("Enums - Option Enum", || {
        let some_number = Some(5);
        let some_string = Some("a string");
        let absent_number: Option<i32> = None;
        
        assert_with_msg!(some_number.is_some(), "Some variant should be detected");
        assert_with_msg!(some_string.is_some(), "Option should work with different types");
        assert_with_msg!(absent_number.is_none(), "None variant should be detected");
        
        match some_number {
            Some(i) => assert_with_msg!(i == 5, "Some value should be extractable"),
            None => panic!("Should not be None"),
        }
        
        let value = some_number.unwrap_or(0);
        assert_with_msg!(value == 5, "unwrap_or should return contained value");
        
        let value = absent_number.unwrap_or(0);
        assert_with_msg!(value == 0, "unwrap_or should return default for None");
    }));
    
    suite
}

/// Test trait concepts
pub fn test_trait_concepts() -> TestSuite {
    let mut suite = TestSuite::new("Intermediate Level - Trait Concepts".to_string());
    
    suite.add_test(test_case!("Traits - Basic Trait", || {
        trait Summary {
            fn summarize(&self) -> String;
        }
        
        struct NewsArticle {
            headline: String,
            location: String,
            author: String,
            content: String,
        }
        
        impl Summary for NewsArticle {
            fn summarize(&self) -> String {
                format!("{}, by {} ({})", self.headline, self.author, self.location)
            }
        }
        
        struct Tweet {
            username: String,
            content: String,
            reply: bool,
            retweet: bool,
        }
        
        impl Summary for Tweet {
            fn summarize(&self) -> String {
                format!("{}: {}", self.username, self.content)
            }
        }
        
        let article = NewsArticle {
            headline: String::from("Penguins win the Stanley Cup Championship!"),
            location: String::from("Pittsburgh, PA, USA"),
            author: String::from("Iceburgh"),
            content: String::from("The Pittsburgh Penguins once again are the best hockey team in the NHL."),
        };
        
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
        
        assert_with_msg!(
            article.summarize() == "Penguins win the Stanley Cup Championship!, by Iceburgh (Pittsburgh, PA, USA)",
            "Trait implementation should work for NewsArticle"
        );
        
        assert_with_msg!(
            tweet.summarize() == "horse_ebooks: of course, as you probably already know, people",
            "Trait implementation should work for Tweet"
        );
    }));
    
    suite.add_test(test_case!("Traits - Default Implementation", || {
        trait Summary {
            fn summarize_author(&self) -> String;
            
            fn summarize(&self) -> String {
                format!("(Read more from {}...)", self.summarize_author())
            }
        }
        
        struct Tweet {
            username: String,
            content: String,
            reply: bool,
            retweet: bool,
        }
        
        impl Summary for Tweet {
            fn summarize_author(&self) -> String {
                format!("@{}", self.username)
            }
        }
        
        let tweet = Tweet {
            username: String::from("horse_ebooks"),
            content: String::from("of course, as you probably already know, people"),
            reply: false,
            retweet: false,
        };
        
        assert_with_msg!(
            tweet.summarize() == "(Read more from @horse_ebooks...)",
            "Default trait implementation should work"
        );
    }));
    
    suite.add_test(test_case!("Traits - Trait Bounds", || {
        trait Display {
            fn fmt(&self) -> String;
        }
        
        trait Debug {
            fn debug(&self) -> String;
        }
        
        fn notify<T: Display + Debug>(item: &T) -> String {
            format!("Breaking news! {} | Debug: {}", item.fmt(), item.debug())
        }
        
        struct Article {
            title: String,
            content: String,
        }
        
        impl Display for Article {
            fn fmt(&self) -> String {
                self.title.clone()
            }
        }
        
        impl Debug for Article {
            fn debug(&self) -> String {
                format!("Article {{ title: {}, content: {} }}", self.title, self.content)
            }
        }
        
        let article = Article {
            title: String::from("Breaking News"),
            content: String::from("Something happened"),
        };
        
        let result = notify(&article);
        assert_with_msg!(
            result.contains("Breaking news! Breaking News"),
            "Trait bound function should use Display trait"
        );
        assert_with_msg!(
            result.contains("Debug: Article { title: Breaking News"),
            "Trait bound function should use Debug trait"
        );
    }));
    
    suite
}

/// Test generic concepts
pub fn test_generic_concepts() -> TestSuite {
    let mut suite = TestSuite::new("Intermediate Level - Generic Concepts".to_string());
    
    suite.add_test(test_case!("Generics - Generic Functions", || {
        fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
            let mut largest = list[0];
            
            for &item in list {
                if item > largest {
                    largest = item;
                }
            }
            
            largest
        }
        
        let number_list = vec![34, 50, 25, 100, 65];
        let result = largest(&number_list);
        assert_with_msg!(result == 100, "Generic function should find largest number");
        
        let char_list = vec!['y', 'm', 'a', 'q'];
        let result = largest(&char_list);
        assert_with_msg!(result == 'y', "Generic function should work with different types");
    }));
    
    suite.add_test(test_case!("Generics - Generic Structs", || {
        #[derive(Debug, PartialEq)]
        struct Point<T> {
            x: T,
            y: T,
        }
        
        impl<T> Point<T> {
            fn x(&self) -> &T {
                &self.x
            }
        }
        
        impl Point<f32> {
            fn distance_from_origin(&self) -> f32 {
                (self.x.powi(2) + self.y.powi(2)).sqrt()
            }
        }
        
        let integer_point = Point { x: 5, y: 10 };
        let float_point = Point { x: 1.0, y: 4.0 };
        
        assert_with_msg!(*integer_point.x() == 5, "Generic struct method should work");
        assert_with_msg!(*float_point.x() == 1.0, "Generic struct should work with different types");
        
        let distance = float_point.distance_from_origin();
        assert_with_msg!((distance - 4.123).abs() < 0.01, "Specific implementation should work");
    }));
    
    suite.add_test(test_case!("Generics - Generic Enums", || {
        #[derive(Debug, PartialEq)]
        enum Result<T, E> {
            Ok(T),
            Err(E),
        }
        
        let success: Result<i32, &str> = Result::Ok(42);
        let failure: Result<i32, &str> = Result::Err("Something went wrong");
        
        match success {
            Result::Ok(value) => assert_with_msg!(value == 42, "Generic enum should store success value"),
            Result::Err(_) => panic!("Should be Ok variant"),
        }
        
        match failure {
            Result::Ok(_) => panic!("Should be Err variant"),
            Result::Err(msg) => assert_with_msg!(msg == "Something went wrong", "Generic enum should store error value"),
        }
    }));
    
    suite
}

/// Test collection concepts
pub fn test_collection_concepts() -> TestSuite {
    let mut suite = TestSuite::new("Intermediate Level - Collection Concepts".to_string());
    
    suite.add_test(test_case!("Collections - Vector", || {
        let mut v = Vec::new();
        v.push(5);
        v.push(6);
        v.push(7);
        v.push(8);
        
        assert_with_msg!(v.len() == 4, "Vector should track length correctly");
        assert_with_msg!(v[0] == 5, "Vector indexing should work");
        assert_with_msg!(v[3] == 8, "Vector should store all elements");
        
        let v2 = vec![1, 2, 3, 4, 5];
        assert_with_msg!(v2.len() == 5, "vec! macro should create vector correctly");
        
        match v2.get(2) {
            Some(third) => assert_with_msg!(*third == 3, "Vector get method should return Some"),
            None => panic!("Should find element at index 2"),
        }
        
        match v2.get(100) {
            Some(_) => panic!("Should not find element at index 100"),
            None => assert_with_msg!(true, "Vector get method should return None for invalid index"),
        }
    }));
    
    suite.add_test(test_case!("Collections - HashMap", || {
        let mut scores = HashMap::new();
        
        scores.insert(String::from("Blue"), 10);
        scores.insert(String::from("Yellow"), 50);
        
        assert_with_msg!(scores.len() == 2, "HashMap should track size correctly");
        
        let team_name = String::from("Blue");
        let score = scores.get(&team_name);
        
        match score {
            Some(s) => assert_with_msg!(*s == 10, "HashMap should retrieve values correctly"),
            None => panic!("Should find Blue team score"),
        }
        
        // Test overwriting
        scores.insert(String::from("Blue"), 25);
        assert_with_msg!(scores[&String::from("Blue")] == 25, "HashMap should allow overwriting values");
        
        // Test entry API
        scores.entry(String::from("Red")).or_insert(30);
        scores.entry(String::from("Blue")).or_insert(40);
        
        assert_with_msg!(scores[&String::from("Red")] == 30, "Entry API should insert new values");
        assert_with_msg!(scores[&String::from("Blue")] == 25, "Entry API should not overwrite existing values");
    }));
    
    suite.add_test(test_case!("Collections - String", || {
        let mut s = String::new();
        s.push_str("hello");
        s.push(' ');
        s.push_str("world");
        
        assert_with_msg!(s == "hello world", "String should support building incrementally");
        
        let s1 = String::from("Hello, ");
        let s2 = String::from("world!");
        let s3 = s1 + &s2; // s1 has been moved here and can no longer be used
        
        assert_with_msg!(s3 == "Hello, world!", "String concatenation should work");
        
        let s1 = String::from("tic");
        let s2 = String::from("tac");
        let s3 = String::from("toe");
        
        let s = format!("{}-{}-{}", s1, s2, s3);
        assert_with_msg!(s == "tic-tac-toe", "format! macro should concatenate strings");
        
        // Test string slicing
        let hello = "Здравствуйте";
        let s = &hello[0..4];
        assert_with_msg!(s == "Зд", "String slicing should work with Unicode");
    }));
    
    suite
}