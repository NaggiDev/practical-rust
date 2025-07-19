//! Example usage of the compiler plugin
//!
//! This example demonstrates how to use the various macros and features
//! provided by the compiler plugin.

use compiler_plugin::{lint_function, LintableStruct, analyze_module};

// Example of a well-structured function that should pass most lints
#[lint_function]
fn calculate_fibonacci(n: u32) -> u64 {
    if n <= 1 {
        return n as u64;
    }
    
    let mut prev = 0u64;
    let mut curr = 1u64;
    
    for _ in 2..=n {
        let next = prev + curr;
        prev = curr;
        curr = next;
    }
    
    curr
}

// Example of a function that might trigger some warnings
#[lint_function]
fn ComplexFunction(input: Vec<i32>) -> Option<i32> {  // Bad naming
    let unused_var = 42;  // Unused variable
    
    // Complex nested logic
    if !input.is_empty() {
        match input.first() {
            Some(first) => {
                if *first > 0 {
                    match input.last() {
                        Some(last) => {
                            if *last < 100 {
                                match input.get(input.len() / 2) {
                                    Some(middle) => Some(*middle),
                                    None => None,
                                }
                            } else {
                                None
                            }
                        }
                        None => None,
                    }
                } else {
                    None
                }
            }
            None => None,
        }
    } else {
        None
    }
}

// Example of a well-structured struct
#[derive(LintableStruct, Debug)]
struct UserAccount {
    user_id: u64,
    username: String,
    email: String,
    is_active: bool,
}

// Example of a struct that might trigger warnings
#[derive(LintableStruct, Debug)]
struct badDataStructure {  // Bad naming
    UserId: u64,           // Bad field naming
    UserName: String,      // Bad field naming
    EmailAddress: String,  // Bad field naming
    IsActive: bool,        // Bad field naming
    CreatedAt: String,     // Bad field naming
    UpdatedAt: String,     // Bad field naming
    LastLoginAt: Option<String>,  // Bad field naming
    ProfilePicture: Option<Vec<u8>>,  // Bad field naming
    Settings: std::collections::HashMap<String, String>,  // Bad field naming
    Permissions: Vec<String>,  // Bad field naming
    Metadata: serde_json::Value,  // Too many fields + bad naming
}

impl UserAccount {
    fn new(user_id: u64, username: String, email: String) -> Self {
        Self {
            user_id,
            username,
            email,
            is_active: true,
        }
    }
    
    fn deactivate(&mut self) {
        self.is_active = false;
    }
    
    fn is_valid_email(&self) -> bool {
        self.email.contains('@')
    }
}

// Example of module-level analysis
analyze_module! {
    fn helper_function_one() {
        println!("Helper 1");
    }
    
    fn helper_function_two() {
        println!("Helper 2");
    }
    
    fn helper_function_three() {
        println!("Helper 3");
    }
    
    struct HelperStruct {
        value: i32,
    }
    
    struct AnotherHelper {
        data: String,
    }
}

fn main() {
    println!("=== Compiler Plugin Usage Example ===\n");
    
    // Test the fibonacci function
    println!("Testing fibonacci calculation:");
    for i in 0..10 {
        let result = calculate_fibonacci(i);
        println!("fibonacci({}) = {}", i, result);
    }
    println!();
    
    // Test the complex function
    println!("Testing complex function:");
    let test_data = vec![1, 5, 10, 15, 20];
    match ComplexFunction(test_data) {
        Some(value) => println!("Complex function returned: {}", value),
        None => println!("Complex function returned None"),
    }
    println!();
    
    // Test the user account struct
    println!("Testing user account:");
    let mut account = UserAccount::new(
        12345,
        "john_doe".to_string(),
        "john@example.com".to_string(),
    );
    
    println!("Account: {:?}", account);
    println!("Email valid: {}", account.is_valid_email());
    println!("Lint info: {}", account.lint_info());
    
    account.deactivate();
    println!("After deactivation: {:?}", account);
    println!();
    
    // Test the bad data structure
    println!("Testing bad data structure:");
    let bad_data = badDataStructure {
        UserId: 67890,
        UserName: "jane_doe".to_string(),
        EmailAddress: "jane@example.com".to_string(),
        IsActive: true,
        CreatedAt: "2023-01-01".to_string(),
        UpdatedAt: "2023-01-02".to_string(),
        LastLoginAt: Some("2023-01-03".to_string()),
        ProfilePicture: None,
        Settings: std::collections::HashMap::new(),
        Permissions: vec!["read".to_string(), "write".to_string()],
        Metadata: serde_json::Value::Null,
    };
    
    println!("Bad data: {:?}", bad_data);
    println!("Lint info: {}", bad_data.lint_info());
    
    println!("\n=== Plugin Analysis Complete ===");
    println!("Check the compiler output above for any warnings or suggestions!");
}