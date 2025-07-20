// Feedback System for Rust Learning Path
// Provides detailed feedback and guidance for incomplete project implementations

use std::collections::HashMap;
use std::fs;
use std::path::Path;
use crate::framework::{TestResult, TestSuite};
use crate::project_validator::{ProjectValidator, ProjectRequirement, ValidationType};

/// Feedback message with different severity levels
#[derive(Debug, Clone)]
pub struct FeedbackMessage {
    pub severity: FeedbackSeverity,
    pub title: String,
    pub message: String,
    pub suggestions: Vec<String>,
    pub resources: Vec<String>,
}

/// Severity levels for feedback messages
#[derive(Debug, Clone, PartialEq)]
pub enum FeedbackSeverity {
    Error,      // Critical issues that prevent project from working
    Warning,    // Issues that should be addressed but don't break functionality
    Info,       // Helpful information and suggestions
    Success,    // Positive feedback for completed requirements
}

/// Feedback generator that provides detailed guidance
pub struct FeedbackGenerator {
    feedback_templates: HashMap<String, FeedbackTemplate>,
}

/// Template for generating contextual feedback
#[derive(Debug, Clone)]
pub struct FeedbackTemplate {
    pub error_message: String,
    pub suggestions: Vec<String>,
    pub resources: Vec<String>,
    pub code_examples: Vec<String>,
}

impl FeedbackGenerator {
    pub fn new() -> Self {
        let mut generator = Self {
            feedback_templates: HashMap::new(),
        };
        
        generator.initialize_feedback_templates();
        generator
    }
    
    /// Generate comprehensive feedback for a project validation suite
    pub fn generate_project_feedback(&self, project_name: &str, validation_suite: &TestSuite) -> Vec<FeedbackMessage> {
        let mut feedback = Vec::new();
        
        // Add overall project status
        let success_rate = validation_suite.success_rate();
        if success_rate == 100.0 {
            feedback.push(FeedbackMessage {
                severity: FeedbackSeverity::Success,
                title: "🎉 Project Complete!".to_string(),
                message: format!("Congratulations! Your {} project meets all requirements.", project_name),
                suggestions: vec![
                    "Consider exploring the extension challenges".to_string(),
                    "Review the concepts you've learned".to_string(),
                    "Move on to the next project when ready".to_string(),
                ],
                resources: vec![],
            });
        } else if success_rate >= 75.0 {
            feedback.push(FeedbackMessage {
                severity: FeedbackSeverity::Warning,
                title: "⚠️ Almost There!".to_string(),
                message: format!("Your {} project is {:.1}% complete. Just a few more requirements to address.", project_name, success_rate),
                suggestions: vec![
                    "Focus on the failing requirements below".to_string(),
                    "Review the project instructions carefully".to_string(),
                ],
                resources: vec![],
            });
        } else {
            feedback.push(FeedbackMessage {
                severity: FeedbackSeverity::Error,
                title: "❌ Project Needs Work".to_string(),
                message: format!("Your {} project is {:.1}% complete. Several requirements need attention.", project_name, success_rate),
                suggestions: vec![
                    "Start with the basic project structure".to_string(),
                    "Follow the step-by-step instructions".to_string(),
                    "Don't hesitate to ask for help if needed".to_string(),
                ],
                resources: vec![
                    "https://doc.rust-lang.org/book/".to_string(),
                    "https://doc.rust-lang.org/rust-by-example/".to_string(),
                ],
            });
        }
        
        // Generate specific feedback for each failed test
        for test in &validation_suite.tests {
            if !test.passed {
                if let Some(specific_feedback) = self.generate_specific_feedback(&test.name, &test.message) {
                    feedback.push(specific_feedback);
                }
            }
        }
        
        // Add general guidance based on project type
        feedback.extend(self.generate_project_specific_guidance(project_name, validation_suite));
        
        feedback
    }
    
    /// Generate specific feedback for a failed test
    fn generate_specific_feedback(&self, test_name: &str, error_message: &str) -> Option<FeedbackMessage> {
        // Extract requirement ID from test name if present
        let requirement_id = self.extract_requirement_id(test_name);
        
        // Look for matching feedback template
        if let Some(template) = self.feedback_templates.get(&requirement_id) {
            return Some(FeedbackMessage {
                severity: FeedbackSeverity::Error,
                title: format!("❌ {}", test_name),
                message: format!("{}\n\nError: {}", template.error_message, error_message),
                suggestions: template.suggestions.clone(),
                resources: template.resources.clone(),
            });
        }
        
        // Generate generic feedback based on error patterns
        self.generate_generic_feedback(test_name, error_message)
    }
    
    /// Generate generic feedback based on common error patterns
    fn generate_generic_feedback(&self, test_name: &str, error_message: &str) -> Option<FeedbackMessage> {
        if error_message.contains("not found") || error_message.contains("No such file") {
            return Some(FeedbackMessage {
                severity: FeedbackSeverity::Error,
                title: format!("📁 Missing File: {}", test_name),
                message: "A required file is missing from your project.".to_string(),
                suggestions: vec![
                    "Check the project structure requirements".to_string(),
                    "Make sure you've created all necessary files".to_string(),
                    "Verify file names match exactly (case-sensitive)".to_string(),
                ],
                resources: vec![
                    "https://doc.rust-lang.org/cargo/guide/project-layout.html".to_string(),
                ],
            });
        }
        
        if error_message.contains("Compilation failed") || error_message.contains("error:") {
            return Some(FeedbackMessage {
                severity: FeedbackSeverity::Error,
                title: format!("🔧 Compilation Error: {}", test_name),
                message: "Your code has compilation errors that need to be fixed.".to_string(),
                suggestions: vec![
                    "Run 'cargo check' to see detailed error messages".to_string(),
                    "Fix syntax errors and type mismatches".to_string(),
                    "Make sure all dependencies are properly declared".to_string(),
                ],
                resources: vec![
                    "https://doc.rust-lang.org/error-index.html".to_string(),
                    "https://doc.rust-lang.org/book/ch01-03-hello-cargo.html".to_string(),
                ],
            });
        }
        
        if error_message.contains("Tests failed") {
            return Some(FeedbackMessage {
                severity: FeedbackSeverity::Warning,
                title: format!("🧪 Test Failures: {}", test_name),
                message: "Some tests are failing, indicating your implementation may not be complete.".to_string(),
                suggestions: vec![
                    "Run 'cargo test' to see which specific tests are failing".to_string(),
                    "Review the test cases to understand expected behavior".to_string(),
                    "Debug your implementation step by step".to_string(),
                ],
                resources: vec![
                    "https://doc.rust-lang.org/book/ch11-00-testing.html".to_string(),
                ],
            });
        }
        
        if error_message.contains("Function") && error_message.contains("not found") {
            return Some(FeedbackMessage {
                severity: FeedbackSeverity::Error,
                title: format!("🔍 Missing Function: {}", test_name),
                message: "A required function is missing from your implementation.".to_string(),
                suggestions: vec![
                    "Check the project requirements for required function names".to_string(),
                    "Make sure function names match exactly".to_string(),
                    "Verify functions are public if they need to be accessed from tests".to_string(),
                ],
                resources: vec![
                    "https://doc.rust-lang.org/book/ch03-03-how-functions-work.html".to_string(),
                ],
            });
        }
        
        None
    }
    
    /// Generate project-specific guidance
    fn generate_project_specific_guidance(&self, project_name: &str, validation_suite: &TestSuite) -> Vec<FeedbackMessage> {
        let mut guidance = Vec::new();
        
        match project_name {
            "calculator" => {
                guidance.extend(self.generate_calculator_guidance(validation_suite));
            }
            "file-explorer" => {
                guidance.extend(self.generate_file_explorer_guidance(validation_suite));
            }
            "library-management" => {
                guidance.extend(self.generate_library_management_guidance(validation_suite));
            }
            "thread-pool" => {
                guidance.extend(self.generate_thread_pool_guidance(validation_suite));
            }
            "async-network-server" => {
                guidance.extend(self.generate_async_server_guidance(validation_suite));
            }
            _ => {}
        }
        
        guidance
    }
    
    /// Generate calculator-specific guidance
    fn generate_calculator_guidance(&self, validation_suite: &TestSuite) -> Vec<FeedbackMessage> {
        let mut guidance = Vec::new();
        
        if validation_suite.success_rate() < 50.0 {
            guidance.push(FeedbackMessage {
                severity: FeedbackSeverity::Info,
                title: "💡 Calculator Project Tips".to_string(),
                message: "The calculator project focuses on basic Rust concepts like functions, error handling, and pattern matching.".to_string(),
                suggestions: vec![
                    "Start by implementing the basic project structure".to_string(),
                    "Create functions for parsing input and performing calculations".to_string(),
                    "Use Result types for error handling".to_string(),
                    "Implement match expressions for different operations".to_string(),
                ],
                resources: vec![
                    "https://doc.rust-lang.org/book/ch06-02-match.html".to_string(),
                    "https://doc.rust-lang.org/book/ch09-02-recoverable-errors-with-result.html".to_string(),
                ],
            });
        }
        
        guidance
    }
    
    /// Generate file explorer-specific guidance
    fn generate_file_explorer_guidance(&self, validation_suite: &TestSuite) -> Vec<FeedbackMessage> {
        let mut guidance = Vec::new();
        
        if validation_suite.success_rate() < 50.0 {
            guidance.push(FeedbackMessage {
                severity: FeedbackSeverity::Info,
                title: "💡 File Explorer Project Tips".to_string(),
                message: "The file explorer project teaches you about working with the file system and error handling.".to_string(),
                suggestions: vec![
                    "Use std::fs module for file system operations".to_string(),
                    "Handle potential I/O errors with Result types".to_string(),
                    "Consider using Path and PathBuf for path manipulation".to_string(),
                ],
                resources: vec![
                    "https://doc.rust-lang.org/std/fs/index.html".to_string(),
                    "https://doc.rust-lang.org/std/path/index.html".to_string(),
                ],
            });
        }
        
        guidance
    }
    
    /// Generate library management-specific guidance
    fn generate_library_management_guidance(&self, validation_suite: &TestSuite) -> Vec<FeedbackMessage> {
        let mut guidance = Vec::new();
        
        if validation_suite.success_rate() < 50.0 {
            guidance.push(FeedbackMessage {
                severity: FeedbackSeverity::Info,
                title: "💡 Library Management Project Tips".to_string(),
                message: "This project focuses on structs, methods, and data management.".to_string(),
                suggestions: vec![
                    "Define structs for Book and Library".to_string(),
                    "Implement methods for adding, removing, and searching books".to_string(),
                    "Use Vec<T> to store collections of books".to_string(),
                    "Consider using HashMap for efficient lookups".to_string(),
                ],
                resources: vec![
                    "https://doc.rust-lang.org/book/ch05-00-structs.html".to_string(),
                    "https://doc.rust-lang.org/book/ch08-03-hash-maps.html".to_string(),
                ],
            });
        }
        
        guidance
    }
    
    /// Generate thread pool-specific guidance
    fn generate_thread_pool_guidance(&self, validation_suite: &TestSuite) -> Vec<FeedbackMessage> {
        let mut guidance = Vec::new();
        
        if validation_suite.success_rate() < 50.0 {
            guidance.push(FeedbackMessage {
                severity: FeedbackSeverity::Info,
                title: "💡 Thread Pool Project Tips".to_string(),
                message: "This advanced project involves concurrency and thread management.".to_string(),
                suggestions: vec![
                    "Use std::thread for creating threads".to_string(),
                    "Implement channels for communication between threads".to_string(),
                    "Consider using Arc and Mutex for shared state".to_string(),
                    "Handle thread panics gracefully".to_string(),
                ],
                resources: vec![
                    "https://doc.rust-lang.org/book/ch16-00-concurrency.html".to_string(),
                    "https://doc.rust-lang.org/std/sync/index.html".to_string(),
                ],
            });
        }
        
        guidance
    }
    
    /// Generate async server-specific guidance
    fn generate_async_server_guidance(&self, validation_suite: &TestSuite) -> Vec<FeedbackMessage> {
        let mut guidance = Vec::new();
        
        if validation_suite.success_rate() < 50.0 {
            guidance.push(FeedbackMessage {
                severity: FeedbackSeverity::Info,
                title: "💡 Async Network Server Project Tips".to_string(),
                message: "This expert-level project involves async programming and network I/O.".to_string(),
                suggestions: vec![
                    "Add tokio dependency to Cargo.toml".to_string(),
                    "Use #[tokio::main] for async main function".to_string(),
                    "Implement async functions with async/await".to_string(),
                    "Use TcpListener for network connections".to_string(),
                ],
                resources: vec![
                    "https://tokio.rs/tokio/tutorial".to_string(),
                    "https://doc.rust-lang.org/book/ch20-00-final-project-a-web-server.html".to_string(),
                ],
            });
        }
        
        guidance
    }
    
    /// Extract requirement ID from test name
    fn extract_requirement_id(&self, test_name: &str) -> String {
        // Look for pattern like "CALC-001: Description"
        if let Some(colon_pos) = test_name.find(':') {
            let potential_id = &test_name[..colon_pos];
            if potential_id.contains('-') && potential_id.len() <= 10 {
                return potential_id.to_string();
            }
        }
        
        // Fallback to generic key
        "GENERIC".to_string()
    }
    
    /// Initialize feedback templates for common issues
    fn initialize_feedback_templates(&mut self) {
        // Calculator project templates
        self.feedback_templates.insert("CALC-001".to_string(), FeedbackTemplate {
            error_message: "The Cargo.toml file is missing. This file is required for all Rust projects.".to_string(),
            suggestions: vec![
                "Create a Cargo.toml file in your project root".to_string(),
                "Use 'cargo new calculator' to create a new project with proper structure".to_string(),
                "Make sure the file is named exactly 'Cargo.toml' (case-sensitive)".to_string(),
            ],
            resources: vec![
                "https://doc.rust-lang.org/cargo/reference/manifest.html".to_string(),
            ],
            code_examples: vec![
                r#"[package]
name = "calculator"
version = "0.1.0"
edition = "2021""#.to_string(),
            ],
        });
        
        self.feedback_templates.insert("CALC-004".to_string(), FeedbackTemplate {
            error_message: "The parse_input function is missing. This function should parse user input into operands and operator.".to_string(),
            suggestions: vec![
                "Create a function named 'parse_input' that takes a string and returns a Result".to_string(),
                "The function should split the input and extract two numbers and an operator".to_string(),
                "Handle parsing errors by returning appropriate error types".to_string(),
            ],
            resources: vec![
                "https://doc.rust-lang.org/book/ch03-03-how-functions-work.html".to_string(),
                "https://doc.rust-lang.org/std/primitive.str.html#method.split".to_string(),
            ],
            code_examples: vec![
                r#"fn parse_input(input: &str) -> Result<Calculation, CalculatorError> {
    // Implementation here
}"#.to_string(),
            ],
        });
        
        // Add more templates for other requirements...
        self.add_more_feedback_templates();
    }
    
    /// Add additional feedback templates
    fn add_more_feedback_templates(&mut self) {
        // File Explorer templates
        self.feedback_templates.insert("FEXP-004".to_string(), FeedbackTemplate {
            error_message: "The list_directory function is missing. This function should list the contents of a directory.".to_string(),
            suggestions: vec![
                "Create a function named 'list_directory' that takes a path parameter".to_string(),
                "Use std::fs::read_dir to read directory contents".to_string(),
                "Handle potential I/O errors with Result types".to_string(),
            ],
            resources: vec![
                "https://doc.rust-lang.org/std/fs/fn.read_dir.html".to_string(),
            ],
            code_examples: vec![
                r#"fn list_directory(path: &Path) -> Result<Vec<String>, std::io::Error> {
    // Implementation here
}"#.to_string(),
            ],
        });
        
        // Generic templates
        self.feedback_templates.insert("GENERIC".to_string(), FeedbackTemplate {
            error_message: "A project requirement was not met.".to_string(),
            suggestions: vec![
                "Review the project requirements carefully".to_string(),
                "Check the project README for detailed instructions".to_string(),
                "Make sure your implementation follows the specified interface".to_string(),
            ],
            resources: vec![
                "https://doc.rust-lang.org/book/".to_string(),
            ],
            code_examples: vec![],
        });
    }
}

/// Format feedback messages for display
pub fn format_feedback_messages(messages: &[FeedbackMessage]) -> String {
    let mut output = String::new();
    
    output.push_str("=".repeat(80).as_str());
    output.push_str("\n📋 PROJECT FEEDBACK REPORT\n");
    output.push_str("=".repeat(80).as_str());
    output.push('\n');
    
    for (i, message) in messages.iter().enumerate() {
        output.push_str(&format!("\n{}. {}\n", i + 1, message.title));
        output.push_str(&format!("   {}\n", message.message));
        
        if !message.suggestions.is_empty() {
            output.push_str("\n   💡 Suggestions:\n");
            for suggestion in &message.suggestions {
                output.push_str(&format!("   • {}\n", suggestion));
            }
        }
        
        if !message.resources.is_empty() {
            output.push_str("\n   📚 Resources:\n");
            for resource in &message.resources {
                output.push_str(&format!("   • {}\n", resource));
            }
        }
        
        output.push_str(&"-".repeat(40));
        output.push('\n');
    }
    
    output.push_str("\n");
    output.push_str("=".repeat(80).as_str());
    output.push('\n');
    
    output
}