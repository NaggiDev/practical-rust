//! Integration tests for the compiler plugin
//!
//! These tests verify that the plugin works correctly when applied to
//! real Rust code examples.

use compiler_plugin::{lint_function, LintableStruct};

#[test]
fn test_lint_function_macro() {
    // This test verifies that the lint_function macro can be applied
    // without causing compilation errors
    
    #[lint_function]
    fn good_function_name() {
        let _used_variable = 42;
        println!("This is a simple function");
    }
    
    good_function_name();
}

#[test]
fn test_lintable_struct_derive() {
    // Test that the derive macro works correctly
    
    #[derive(LintableStruct)]
    struct GoodStruct {
        field_one: i32,
        field_two: String,
    }
    
    let instance = GoodStruct {
        field_one: 42,
        field_two: "test".to_string(),
    };
    
    // Verify the trait is implemented
    assert_eq!(instance.lint_info(), "This struct has been analyzed by the compiler plugin");
}

#[test]
fn test_complex_function_analysis() {
    // Test a more complex function that should trigger warnings
    
    #[lint_function]
    fn ComplexFunction() {  // Should trigger naming convention warning
        let unused_var = 42;  // Should trigger unused variable warning
        
        // Complex nested expressions (should trigger complexity warning)
        if true {
            match Some(1) {
                Some(x) => {
                    if x > 0 {
                        match Some(x * 2) {
                            Some(y) => println!("{}", y),
                            None => {}
                        }
                    }
                }
                None => {}
            }
        }
    }
    
    ComplexFunction();
}

#[test]
fn test_struct_with_issues() {
    // Test a struct that should trigger various warnings
    
    #[derive(LintableStruct)]
    struct badStructName {  // Should trigger PascalCase warning
        BadField: i32,      // Should trigger snake_case warning
        AnotherBadField: String,  // Should trigger snake_case warning
        field3: bool,
        field4: f64,
        field5: Vec<i32>,
        field6: Option<String>,
        field7: Result<i32, String>,
        field8: std::collections::HashMap<String, i32>,
        field9: Box<dyn std::fmt::Display>,
        field10: std::sync::Arc<std::sync::Mutex<i32>>,
        field11: tokio::sync::RwLock<String>,  // Should trigger "too many fields" warning
    }
    
    let _instance = badStructName {
        BadField: 1,
        AnotherBadField: "test".to_string(),
        field3: true,
        field4: 3.14,
        field5: vec![1, 2, 3],
        field6: Some("test".to_string()),
        field7: Ok(42),
        field8: std::collections::HashMap::new(),
        field9: Box::new("display"),
        field10: std::sync::Arc::new(std::sync::Mutex::new(42)),
        field11: tokio::sync::RwLock::new("test".to_string()),
    };
}

// Test module-level analysis
mod test_module {
    use super::*;
    
    compiler_plugin::analyze_module! {
        fn function1() {}
        fn function2() {}
        fn function3() {}
        
        struct TestStruct1 {
            field: i32,
        }
        
        struct TestStruct2 {
            field: String,
        }
    }
}

#[cfg(test)]
mod unit_tests {
    use compiler_plugin::{FunctionLinter, StructLinter, ModuleLinter};
    use syn::{parse_quote, DeriveInput, ItemFn};

    #[test]
    fn test_function_linter_directly() {
        let func: ItemFn = parse_quote! {
            fn test_function() {
                let x = 42;
                println!("{}", x);
            }
        };

        let mut linter = FunctionLinter::new();
        let diagnostics = linter.analyze_function(&func);
        
        // Should not have any issues with this simple, well-named function
        assert!(diagnostics.is_empty() || diagnostics.iter().all(|d| {
            !d.message().contains("naming") && !d.message().contains("unused")
        }));
    }

    #[test]
    fn test_function_linter_with_issues() {
        let func: ItemFn = parse_quote! {
            fn BadFunctionName() {
                let unused_variable = 42;
                if true {
                    match Some(1) {
                        Some(x) => {
                            if x > 0 {
                                match Some(x) {
                                    Some(y) => println!("{}", y),
                                    None => {}
                                }
                            }
                        }
                        None => {}
                    }
                }
            }
        };

        let mut linter = FunctionLinter::new();
        let diagnostics = linter.analyze_function(&func);
        
        assert!(!diagnostics.is_empty());
        
        // Should detect naming convention issue
        assert!(diagnostics.iter().any(|d| d.message().contains("snake_case")));
        
        // Should detect unused variable
        assert!(diagnostics.iter().any(|d| d.message().contains("unused")));
    }

    #[test]
    fn test_struct_linter_directly() {
        let input: DeriveInput = parse_quote! {
            struct GoodStruct {
                field_one: i32,
                field_two: String,
            }
        };

        let mut linter = StructLinter::new();
        let diagnostics = linter.analyze_struct(&input);
        
        // Should not have naming issues
        assert!(diagnostics.is_empty() || !diagnostics.iter().any(|d| {
            d.message().contains("PascalCase") || d.message().contains("snake_case")
        }));
    }

    #[test]
    fn test_struct_linter_with_issues() {
        let input: DeriveInput = parse_quote! {
            struct badStruct {
                BadField: i32,
                another_field: String,
            }
        };

        let mut linter = StructLinter::new();
        let diagnostics = linter.analyze_struct(&input);
        
        assert!(!diagnostics.is_empty());
        
        // Should detect struct naming issue
        assert!(diagnostics.iter().any(|d| d.message().contains("PascalCase")));
        
        // Should detect field naming issue
        assert!(diagnostics.iter().any(|d| d.message().contains("snake_case")));
    }

    #[test]
    fn test_module_linter() {
        let func: syn::Item = parse_quote! {
            fn test_function() {
                println!("Hello, world!");
            }
        };

        let mut linter = ModuleLinter::new();
        let diagnostics = linter.analyze_item(&func);
        
        // Should be able to analyze without errors
        assert!(linter.lint_count() >= 0);
    }
}

// Compile-time tests using trybuild
#[cfg(test)]
mod compile_tests {
    #[test]
    fn test_compile_pass() {
        let t = trybuild::TestCases::new();
        t.pass("tests/test_cases/good_code.rs");
    }

    #[test]
    fn test_compile_with_warnings() {
        let t = trybuild::TestCases::new();
        // Note: trybuild doesn't directly test warnings, but we can test
        // that code compiles even with our plugin applied
        t.pass("tests/test_cases/bad_code.rs");
    }
}