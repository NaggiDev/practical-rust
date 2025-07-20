// Test Framework for Rust Learning Path
// This module provides utilities for testing all code examples

use std::collections::HashMap;
use std::fmt;

/// Test result for individual test cases
#[derive(Debug, Clone)]
pub struct TestResult {
    pub name: String,
    pub passed: bool,
    pub message: String,
    pub execution_time_ms: u128,
}

impl TestResult {
    pub fn new(name: String, passed: bool, message: String, execution_time_ms: u128) -> Self {
        Self {
            name,
            passed,
            message,
            execution_time_ms,
        }
    }
    
    pub fn success(name: String, execution_time_ms: u128) -> Self {
        Self::new(name, true, "Test passed".to_string(), execution_time_ms)
    }
    
    pub fn failure(name: String, message: String, execution_time_ms: u128) -> Self {
        Self::new(name, false, message, execution_time_ms)
    }
}

impl fmt::Display for TestResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let status = if self.passed { "✓ PASS" } else { "✗ FAIL" };
        write!(f, "{} {} ({} ms) - {}", status, self.name, self.execution_time_ms, self.message)
    }
}

/// Test suite for organizing related tests
#[derive(Debug)]
pub struct TestSuite {
    pub name: String,
    pub tests: Vec<TestResult>,
}

impl TestSuite {
    pub fn new(name: String) -> Self {
        Self {
            name,
            tests: Vec::new(),
        }
    }
    
    pub fn add_test(&mut self, test: TestResult) {
        self.tests.push(test);
    }
    
    pub fn passed_count(&self) -> usize {
        self.tests.iter().filter(|t| t.passed).count()
    }
    
    pub fn failed_count(&self) -> usize {
        self.tests.iter().filter(|t| !t.passed).count()
    }
    
    pub fn total_count(&self) -> usize {
        self.tests.len()
    }
    
    pub fn success_rate(&self) -> f64 {
        if self.total_count() == 0 {
            0.0
        } else {
            self.passed_count() as f64 / self.total_count() as f64 * 100.0
        }
    }
}

impl fmt::Display for TestSuite {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n=== {} ===", self.name)?;
        writeln!(f, "Tests: {} passed, {} failed, {} total", 
                 self.passed_count(), self.failed_count(), self.total_count())?;
        writeln!(f, "Success rate: {:.1}%", self.success_rate())?;
        writeln!(f, "---")?;
        
        for test in &self.tests {
            writeln!(f, "{}", test)?;
        }
        
        Ok(())
    }
}

/// Main test runner for the entire learning path
#[derive(Debug)]
pub struct TestRunner {
    pub suites: HashMap<String, TestSuite>,
}

impl TestRunner {
    pub fn new() -> Self {
        Self {
            suites: HashMap::new(),
        }
    }
    
    pub fn add_suite(&mut self, suite: TestSuite) {
        self.suites.insert(suite.name.clone(), suite);
    }
    
    pub fn get_suite_mut(&mut self, name: &str) -> Option<&mut TestSuite> {
        self.suites.get_mut(name)
    }
    
    pub fn run_all(&self) -> TestSummary {
        let mut total_passed = 0;
        let mut total_failed = 0;
        let mut total_tests = 0;
        
        println!("Running all tests for Rust Learning Path...\n");
        
        for suite in self.suites.values() {
            println!("{}", suite);
            total_passed += suite.passed_count();
            total_failed += suite.failed_count();
            total_tests += suite.total_count();
        }
        
        TestSummary {
            total_tests,
            total_passed,
            total_failed,
            success_rate: if total_tests == 0 { 0.0 } else { total_passed as f64 / total_tests as f64 * 100.0 },
        }
    }
}

/// Summary of all test results
#[derive(Debug)]
pub struct TestSummary {
    pub total_tests: usize,
    pub total_passed: usize,
    pub total_failed: usize,
    pub success_rate: f64,
}

impl fmt::Display for TestSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "\n========== TEST SUMMARY ==========")?;
        writeln!(f, "Total tests: {}", self.total_tests)?;
        writeln!(f, "Passed: {}", self.total_passed)?;
        writeln!(f, "Failed: {}", self.total_failed)?;
        writeln!(f, "Success rate: {:.1}%", self.success_rate)?;
        
        if self.total_failed == 0 {
            writeln!(f, "\n🎉 All tests passed!")?;
        } else {
            writeln!(f, "\n⚠️  {} test(s) failed", self.total_failed)?;
        }
        
        Ok(())
    }
}

/// Macro for creating test cases with timing
#[macro_export]
macro_rules! test_case {
    ($name:expr, $test_fn:expr) => {{
        let start = std::time::Instant::now();
        let result = std::panic::catch_unwind(|| $test_fn);
        let duration = start.elapsed().as_millis();
        
        match result {
            Ok(_) => TestResult::success($name.to_string(), duration),
            Err(panic_info) => {
                let message = if let Some(s) = panic_info.downcast_ref::<&str>() {
                    format!("Panic: {}", s)
                } else if let Some(s) = panic_info.downcast_ref::<String>() {
                    format!("Panic: {}", s)
                } else {
                    "Test panicked with unknown error".to_string()
                };
                TestResult::failure($name.to_string(), message, duration)
            }
        }
    }};
}

/// Macro for asserting with custom error messages
#[macro_export]
macro_rules! assert_with_msg {
    ($condition:expr, $msg:expr) => {
        if !$condition {
            panic!("{}", $msg);
        }
    };
    ($condition:expr, $msg:expr, $($arg:tt)*) => {
        if !$condition {
            panic!($msg, $($arg)*);
        }
    };
}