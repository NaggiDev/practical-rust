//! Test case with code that should trigger various lints

use compiler_plugin::{lint_function, LintableStruct};

#[lint_function]
fn BadFunctionName() {  // Should trigger naming convention warning
    let unused_variable = 42;  // Should trigger unused variable warning
    let another_unused = "test";  // Should trigger unused variable warning
    
    // Complex nested control flow (should trigger complexity warning)
    if true {
        match Some(1) {
            Some(x) => {
                if x > 0 {
                    match Some(x * 2) {
                        Some(y) => {
                            if y < 100 {
                                match Some(y + 1) {
                                    Some(z) => println!("{}", z),
                                    None => {}
                                }
                            }
                        }
                        None => {}
                    }
                }
            }
            None => {}
        }
    }
}

#[lint_function]
fn AnotherBadlyNamedFunction() {  // Should trigger naming convention warning
    // Deeply nested function calls (should add to complexity)
    let _result = some_function(
        another_function(
            yet_another_function(
                deeply_nested_function(42)
            )
        )
    );
}

#[lint_function]
fn function_with_very_long_name_that_exceeds_reasonable_limits() {  // Should trigger long name warning
    println!("This function has an excessively long name");
}

#[derive(LintableStruct)]
struct badStructName {  // Should trigger PascalCase warning
    BadField: i32,      // Should trigger snake_case warning
    AnotherBadField: String,  // Should trigger snake_case warning
    YetAnotherBadField: bool,  // Should trigger snake_case warning
    field4: f64,
    field5: Vec<i32>,
    field6: Option<String>,
    field7: Result<i32, String>,
    field8: std::collections::HashMap<String, i32>,
    field9: Box<dyn std::fmt::Display>,
    field10: std::sync::Arc<std::sync::Mutex<i32>>,
    field11: tokio::sync::RwLock<String>,  // Should trigger "too many fields" warning
}

#[derive(LintableStruct)]
struct another_bad_struct {  // Should trigger PascalCase warning
    Field1: i32,  // Should trigger snake_case warning
    Field2: String,  // Should trigger snake_case warning
}

// Helper functions for testing nested calls
fn some_function(x: i32) -> i32 { x + 1 }
fn another_function(x: i32) -> i32 { x * 2 }
fn yet_another_function(x: i32) -> i32 { x - 1 }
fn deeply_nested_function(x: i32) -> i32 { x / 2 }

fn main() {
    BadFunctionName();
    AnotherBadlyNamedFunction();
    function_with_very_long_name_that_exceeds_reasonable_limits();
    
    let _bad_instance = badStructName {
        BadField: 1,
        AnotherBadField: "test".to_string(),
        YetAnotherBadField: true,
        field4: 3.14,
        field5: vec![1, 2, 3],
        field6: Some("test".to_string()),
        field7: Ok(42),
        field8: std::collections::HashMap::new(),
        field9: Box::new("display"),
        field10: std::sync::Arc::new(std::sync::Mutex::new(42)),
        field11: tokio::sync::RwLock::new("test".to_string()),
    };
    
    let _another_bad = another_bad_struct {
        Field1: 42,
        Field2: "test".to_string(),
    };
}