// Main library file for the Rust Learning Path Test Framework
// This provides comprehensive unit testing for all code examples

pub mod framework;
pub mod basic_tests;
pub mod intermediate_tests;
pub mod advanced_tests;
pub mod expert_tests;
pub mod quiz_framework;

pub use crate::framework::{TestResult, TestSuite, TestRunner, TestSummary, test_case, assert_with_msg};

/// Run all tests for the entire Rust Learning Path
pub fn run_all_tests() -> TestSummary {
    let mut runner = TestRunner::new();
    
    // Add Basic Level tests
    runner.add_suite(basic_tests::test_basic_concepts());
    runner.add_suite(basic_tests::test_string_concepts());
    runner.add_suite(basic_tests::test_error_handling());
    
    // Add Intermediate Level tests
    runner.add_suite(intermediate_tests::test_ownership_concepts());
    runner.add_suite(intermediate_tests::test_struct_enum_concepts());
    runner.add_suite(intermediate_tests::test_trait_concepts());
    runner.add_suite(intermediate_tests::test_generic_concepts());
    runner.add_suite(intermediate_tests::test_collection_concepts());
    
    // Add Advanced Level tests
    runner.add_suite(advanced_tests::test_concurrency_concepts());
    runner.add_suite(advanced_tests::test_unsafe_concepts());
    runner.add_suite(advanced_tests::test_advanced_trait_concepts());
    runner.add_suite(advanced_tests::test_macro_concepts());
    runner.add_suite(advanced_tests::test_error_handling_patterns());
    
    // Add Expert Level tests
    runner.add_suite(expert_tests::test_async_concepts());
    runner.add_suite(expert_tests::test_memory_management_concepts());
    runner.add_suite(expert_tests::test_performance_concepts());
    runner.add_suite(expert_tests::test_compiler_concepts());
    
    runner.run_all()
}

/// Run tests for a specific level
pub fn run_level_tests(level: &str) -> TestSummary {
    let mut runner = TestRunner::new();
    
    match level.to_lowercase().as_str() {
        "basic" => {
            runner.add_suite(basic_tests::test_basic_concepts());
            runner.add_suite(basic_tests::test_string_concepts());
            runner.add_suite(basic_tests::test_error_handling());
        }
        "intermediate" => {
            runner.add_suite(intermediate_tests::test_ownership_concepts());
            runner.add_suite(intermediate_tests::test_struct_enum_concepts());
            runner.add_suite(intermediate_tests::test_trait_concepts());
            runner.add_suite(intermediate_tests::test_generic_concepts());
            runner.add_suite(intermediate_tests::test_collection_concepts());
        }
        "advanced" => {
            runner.add_suite(advanced_tests::test_concurrency_concepts());
            runner.add_suite(advanced_tests::test_unsafe_concepts());
            runner.add_suite(advanced_tests::test_advanced_trait_concepts());
            runner.add_suite(advanced_tests::test_macro_concepts());
            runner.add_suite(advanced_tests::test_error_handling_patterns());
        }
        "expert" => {
            runner.add_suite(expert_tests::test_async_concepts());
            runner.add_suite(expert_tests::test_memory_management_concepts());
            runner.add_suite(expert_tests::test_performance_concepts());
            runner.add_suite(expert_tests::test_compiler_concepts());
        }
        _ => {
            eprintln!("Unknown level: {}. Available levels: basic, intermediate, advanced, expert", level);
            return TestSummary {
                total_tests: 0,
                total_passed: 0,
                total_failed: 0,
                success_rate: 0.0,
            };
        }
    }
    
    runner.run_all()
}

/// Run tests for a specific concept area
pub fn run_concept_tests(concept: &str) -> TestSummary {
    let mut runner = TestRunner::new();
    
    match concept.to_lowercase().as_str() {
        "variables" | "data_types" | "control_flow" | "functions" => {
            runner.add_suite(basic_tests::test_basic_concepts());
        }
        "strings" => {
            runner.add_suite(basic_tests::test_string_concepts());
        }
        "error_handling" => {
            runner.add_suite(basic_tests::test_error_handling());
            runner.add_suite(advanced_tests::test_error_handling_patterns());
        }
        "ownership" | "borrowing" | "lifetimes" => {
            runner.add_suite(intermediate_tests::test_ownership_concepts());
        }
        "structs" | "enums" => {
            runner.add_suite(intermediate_tests::test_struct_enum_concepts());
        }
        "traits" => {
            runner.add_suite(intermediate_tests::test_trait_concepts());
            runner.add_suite(advanced_tests::test_advanced_trait_concepts());
        }
        "generics" => {
            runner.add_suite(intermediate_tests::test_generic_concepts());
        }
        "collections" => {
            runner.add_suite(intermediate_tests::test_collection_concepts());
        }
        "concurrency" | "threading" => {
            runner.add_suite(advanced_tests::test_concurrency_concepts());
        }
        "unsafe" => {
            runner.add_suite(advanced_tests::test_unsafe_concepts());
        }
        "macros" => {
            runner.add_suite(advanced_tests::test_macro_concepts());
        }
        "async" => {
            runner.add_suite(expert_tests::test_async_concepts());
        }
        "memory" => {
            runner.add_suite(expert_tests::test_memory_management_concepts());
        }
        "performance" => {
            runner.add_suite(expert_tests::test_performance_concepts());
        }
        "compiler" => {
            runner.add_suite(expert_tests::test_compiler_concepts());
        }
        _ => {
            eprintln!("Unknown concept: {}. Run with --help to see available concepts", concept);
            return TestSummary {
                total_tests: 0,
                total_passed: 0,
                total_failed: 0,
                success_rate: 0.0,
            };
        }
    }
    
    runner.run_all()
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_framework_basic_functionality() {
        let mut suite = TestSuite::new("Test Framework Test".to_string());
        
        suite.add_test(TestResult::success("Test 1".to_string(), 10));
        suite.add_test(TestResult::failure("Test 2".to_string(), "Failed".to_string(), 20));
        
        assert_eq!(suite.passed_count(), 1);
        assert_eq!(suite.failed_count(), 1);
        assert_eq!(suite.total_count(), 2);
        assert_eq!(suite.success_rate(), 50.0);
    }
    
    #[test]
    fn test_macro_functionality() {
        let result = test_case!("Macro Test", || {
            assert_eq!(2 + 2, 4);
        });
        
        assert!(result.passed);
        assert_eq!(result.name, "Macro Test");
    }
    
    #[test]
    fn test_assert_with_msg_macro() {
        let result = std::panic::catch_unwind(|| {
            assert_with_msg!(2 + 2 == 5, "Math should work correctly");
        });
        
        assert!(result.is_err());
    }
}