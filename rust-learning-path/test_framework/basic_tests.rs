// Comprehensive unit tests for Basic Level code examples
use crate::framework::{TestResult, TestSuite};
use crate::{test_case, assert_with_msg};

/// Test all basic Rust concepts from Module 1
pub fn test_basic_concepts() -> TestSuite {
    let mut suite = TestSuite::new("Basic Level - Module 1 Concepts".to_string());
    
    // Variables and mutability tests
    suite.add_test(test_case!("Variables - Immutable", || {
        let x = 5;
        assert_with_msg!(x == 5, "Immutable variable should retain its value");
    }));
    
    suite.add_test(test_case!("Variables - Mutable", || {
        let mut y = 5;
        y = 10;
        assert_with_msg!(y == 10, "Mutable variable should allow value changes");
    }));
    
    suite.add_test(test_case!("Variables - Shadowing", || {
        let z = 5;
        let z = z + 1;
        assert_with_msg!(z == 6, "Variable shadowing should create new binding");
        
        {
            let z = z * 2;
            assert_with_msg!(z == 12, "Inner scope shadowing should work correctly");
        }
        
        assert_with_msg!(z == 6, "Outer scope variable should be restored after inner scope");
    }));
    
    suite.add_test(test_case!("Variables - Constants", || {
        const MAX_POINTS: u32 = 100_000;
        assert_with_msg!(MAX_POINTS == 100_000, "Constants should maintain their values");
    }));
    
    // Data types tests
    suite.add_test(test_case!("Data Types - Integers", || {
        let a: i8 = 127;
        let b: u8 = 255;
        let c: i16 = 32767;
        let d: u16 = 65535;
        
        assert_with_msg!(a == 127, "i8 should store signed 8-bit integers");
        assert_with_msg!(b == 255, "u8 should store unsigned 8-bit integers");
        assert_with_msg!(c == 32767, "i16 should store signed 16-bit integers");
        assert_with_msg!(d == 65535, "u16 should store unsigned 16-bit integers");
    }));
    
    suite.add_test(test_case!("Data Types - Integer Literals", || {
        let decimal = 98_222;
        let hex = 0xff;
        let octal = 0o77;
        let binary = 0b1111_0000;
        let byte = b'A';
        
        assert_with_msg!(decimal == 98222, "Decimal literals should work with underscores");
        assert_with_msg!(hex == 255, "Hexadecimal literals should be parsed correctly");
        assert_with_msg!(octal == 63, "Octal literals should be parsed correctly");
        assert_with_msg!(binary == 240, "Binary literals should be parsed correctly");
        assert_with_msg!(byte == 65, "Byte literals should represent ASCII values");
    }));
    
    suite.add_test(test_case!("Data Types - Floating Point", || {
        let x = 2.0;      // f64 (default)
        let y: f32 = 3.0; // f32
        
        assert_with_msg!((x - 2.0).abs() < f64::EPSILON, "f64 should handle floating point values");
        assert_with_msg!((y - 3.0).abs() < f32::EPSILON, "f32 should handle floating point values");
    }));
    
    suite.add_test(test_case!("Data Types - Numeric Operations", || {
        let sum = 5 + 10;
        let difference = 95.5 - 4.3;
        let product = 4 * 30;
        let quotient = 56.7 / 32.2;
        let remainder = 43 % 5;
        
        assert_with_msg!(sum == 15, "Addition should work correctly");
        assert_with_msg!((difference - 91.2).abs() < 0.1, "Subtraction should work with floats");
        assert_with_msg!(product == 120, "Multiplication should work correctly");
        assert_with_msg!((quotient - 1.76).abs() < 0.01, "Division should work with floats");
        assert_with_msg!(remainder == 3, "Modulo operation should work correctly");
    }));
    
    suite.add_test(test_case!("Data Types - Boolean", || {
        let t = true;
        let f: bool = false;
        
        assert_with_msg!(t == true, "Boolean true should be true");
        assert_with_msg!(f == false, "Boolean false should be false");
        assert_with_msg!(!f == true, "Boolean negation should work");
    }));
    
    suite.add_test(test_case!("Data Types - Character", || {
        let c = 'z';
        let z: char = 'ℤ';
        let heart_eyed_cat = '😻';
        
        assert_with_msg!(c == 'z', "ASCII characters should work");
        assert_with_msg!(z == 'ℤ', "Unicode characters should work");
        assert_with_msg!(heart_eyed_cat == '😻', "Emoji characters should work");
    }));
    
    suite.add_test(test_case!("Data Types - Tuples", || {
        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup;  // Destructuring
        
        assert_with_msg!(tup.0 == 500, "Tuple index access should work");
        assert_with_msg!((tup.1 - 6.4).abs() < f64::EPSILON, "Tuple should store different types");
        assert_with_msg!(tup.2 == 1, "Tuple should maintain type safety");
        
        assert_with_msg!(x == 500, "Tuple destructuring should work");
        assert_with_msg!((y - 6.4).abs() < f64::EPSILON, "Destructured values should match");
        assert_with_msg!(z == 1, "All tuple elements should destructure correctly");
    }));
    
    suite.add_test(test_case!("Data Types - Arrays", || {
        let arr: [i32; 5] = [1, 2, 3, 4, 5];
        let repeated = [3; 5];  // [3, 3, 3, 3, 3]
        
        assert_with_msg!(arr[0] == 1, "Array indexing should work");
        assert_with_msg!(arr[4] == 5, "Array should store all elements");
        assert_with_msg!(arr.len() == 5, "Array length should be correct");
        
        assert_with_msg!(repeated[0] == 3, "Array repetition syntax should work");
        assert_with_msg!(repeated[4] == 3, "All repeated elements should be the same");
        assert_with_msg!(repeated.len() == 5, "Repeated array should have correct length");
    }));
    
    // Control flow tests
    suite.add_test(test_case!("Control Flow - If Expression", || {
        let number = 6;
        let result = if number % 2 == 0 { "even" } else { "odd" };
        assert_with_msg!(result == "even", "If expression should return correct value");
        
        let number = 7;
        let result = if number % 2 == 0 { "even" } else { "odd" };
        assert_with_msg!(result == "odd", "If expression should handle both branches");
    }));
    
    suite.add_test(test_case!("Control Flow - Loop with Break", || {
        let mut counter = 0;
        let result = loop {
            counter += 1;
            if counter == 10 {
                break counter * 2;
            }
        };
        assert_with_msg!(result == 20, "Loop should return break value");
        assert_with_msg!(counter == 10, "Counter should reach expected value");
    }));
    
    suite.add_test(test_case!("Control Flow - While Loop", || {
        let mut sum = 0;
        let mut i = 1;
        while i <= 5 {
            sum += i;
            i += 1;
        }
        assert_with_msg!(sum == 15, "While loop should calculate sum correctly (1+2+3+4+5=15)");
        assert_with_msg!(i == 6, "While loop counter should be incremented correctly");
    }));
    
    suite.add_test(test_case!("Control Flow - For Loop", || {
        let a = [10, 20, 30, 40, 50];
        let mut sum = 0;
        for element in a.iter() {
            sum += element;
        }
        assert_with_msg!(sum == 150, "For loop should iterate over array correctly (10+20+30+40+50=150)");
        
        // Test range-based for loop
        let mut sum2 = 0;
        for i in 1..=5 {
            sum2 += i;
        }
        assert_with_msg!(sum2 == 15, "Range-based for loop should work correctly");
    }));
    
    suite.add_test(test_case!("Control Flow - Match Expression", || {
        let dice_roll = 4;
        let result = match dice_roll {
            1 => "one",
            2 => "two", 
            3 => "three",
            4 => "four",
            5 => "five",
            6 => "six",
            _ => "invalid",
        };
        assert_with_msg!(result == "four", "Match expression should handle specific values");
        
        let dice_roll = 7;
        let result = match dice_roll {
            1..=6 => "valid",
            _ => "invalid",
        };
        assert_with_msg!(result == "invalid", "Match expression should handle catch-all pattern");
    }));
    
    // Function tests
    suite.add_test(test_case!("Functions - Basic Function", || {
        fn add(a: i32, b: i32) -> i32 {
            a + b
        }
        
        assert_with_msg!(add(2, 3) == 5, "Basic function should return correct result");
        assert_with_msg!(add(-1, 1) == 0, "Function should handle negative numbers");
    }));
    
    suite.add_test(test_case!("Functions - Early Return", || {
        fn absolute_value(x: i32) -> i32 {
            if x < 0 {
                return -x;
            }
            x
        }
        
        assert_with_msg!(absolute_value(-5) == 5, "Function should handle early return for negative values");
        assert_with_msg!(absolute_value(5) == 5, "Function should handle positive values without early return");
        assert_with_msg!(absolute_value(0) == 0, "Function should handle zero correctly");
    }));
    
    suite.add_test(test_case!("Functions - Multiple Return Values", || {
        fn calculate_statistics(numbers: &[i32]) -> (i32, f64, i32, i32) {
            let sum: i32 = numbers.iter().sum();
            let avg: f64 = sum as f64 / numbers.len() as f64;
            let min: i32 = *numbers.iter().min().unwrap_or(&0);
            let max: i32 = *numbers.iter().max().unwrap_or(&0);
            
            (sum, avg, min, max)
        }
        
        let stats = calculate_statistics(&[1, 2, 3, 4, 5]);
        assert_with_msg!(stats.0 == 15, "Function should calculate sum correctly");
        assert_with_msg!((stats.1 - 3.0).abs() < f64::EPSILON, "Function should calculate average correctly");
        assert_with_msg!(stats.2 == 1, "Function should find minimum correctly");
        assert_with_msg!(stats.3 == 5, "Function should find maximum correctly");
        
        // Test edge case with empty slice
        let stats = calculate_statistics(&[]);
        assert_with_msg!(stats.0 == 0, "Empty slice should have sum of 0");
        assert_with_msg!(stats.1.is_nan(), "Empty slice should have NaN average");
        assert_with_msg!(stats.2 == 0, "Empty slice should have default min of 0");
        assert_with_msg!(stats.3 == 0, "Empty slice should have default max of 0");
    }));
    
    suite
}

/// Test string handling concepts
pub fn test_string_concepts() -> TestSuite {
    let mut suite = TestSuite::new("Basic Level - String Concepts".to_string());
    
    suite.add_test(test_case!("Strings - String Literals", || {
        let string_literal = "Hello";  // &str
        assert_with_msg!(string_literal == "Hello", "String literal should maintain value");
        assert_with_msg!(string_literal.len() == 5, "String literal should have correct length");
    }));
    
    suite.add_test(test_case!("Strings - String Type", || {
        let string = String::from("Hello, world!");  // String
        assert_with_msg!(string == "Hello, world!", "String should maintain value");
        assert_with_msg!(string.len() == 13, "String should have correct length");
        
        let mut mutable_string = String::new();
        mutable_string.push_str("Hello");
        mutable_string.push(' ');
        mutable_string.push_str("world!");
        
        assert_with_msg!(mutable_string == "Hello world!", "Mutable string should allow modifications");
    }));
    
    suite.add_test(test_case!("Strings - String Methods", || {
        let s = String::from("Hello, World!");
        
        assert_with_msg!(s.to_lowercase() == "hello, world!", "to_lowercase should work correctly");
        assert_with_msg!(s.to_uppercase() == "HELLO, WORLD!", "to_uppercase should work correctly");
        assert_with_msg!(s.contains("World"), "contains should find substring");
        assert_with_msg!(!s.contains("world"), "contains should be case sensitive");
        assert_with_msg!(s.starts_with("Hello"), "starts_with should work correctly");
        assert_with_msg!(s.ends_with("World!"), "ends_with should work correctly");
    }));
    
    suite
}

/// Test error handling concepts
pub fn test_error_handling() -> TestSuite {
    let mut suite = TestSuite::new("Basic Level - Error Handling".to_string());
    
    suite.add_test(test_case!("Error Handling - Option Type", || {
        fn find_character(s: &str, ch: char) -> Option<usize> {
            s.chars().position(|c| c == ch)
        }
        
        let result = find_character("hello", 'e');
        assert_with_msg!(result == Some(1), "Option should contain Some value when found");
        
        let result = find_character("hello", 'x');
        assert_with_msg!(result == None, "Option should be None when not found");
    }));
    
    suite.add_test(test_case!("Error Handling - Result Type", || {
        fn divide(a: f64, b: f64) -> Result<f64, String> {
            if b == 0.0 {
                Err("Division by zero".to_string())
            } else {
                Ok(a / b)
            }
        }
        
        let result = divide(10.0, 2.0);
        assert_with_msg!(result.is_ok(), "Result should be Ok for valid division");
        assert_with_msg!((result.unwrap() - 5.0).abs() < f64::EPSILON, "Division result should be correct");
        
        let result = divide(10.0, 0.0);
        assert_with_msg!(result.is_err(), "Result should be Err for division by zero");
        assert_with_msg!(result.unwrap_err() == "Division by zero", "Error message should be correct");
    }));
    
    suite.add_test(test_case!("Error Handling - Pattern Matching with Results", || {
        fn parse_number(s: &str) -> Result<i32, std::num::ParseIntError> {
            s.parse::<i32>()
        }
        
        match parse_number("42") {
            Ok(n) => assert_with_msg!(n == 42, "Parsing valid number should succeed"),
            Err(_) => panic!("Should not fail to parse valid number"),
        }
        
        match parse_number("not_a_number") {
            Ok(_) => panic!("Should not succeed parsing invalid number"),
            Err(_) => assert_with_msg!(true, "Parsing invalid number should fail"),
        }
    }));
    
    suite
}