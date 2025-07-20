// Project Validation Framework for Rust Learning Path
// This module provides comprehensive validation for all learning projects

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use crate::framework::{TestResult, TestSuite};
use crate::{test_case, assert_with_msg};

/// Represents a project requirement that needs to be validated
#[derive(Debug, Clone)]
pub struct ProjectRequirement {
    pub id: String,
    pub description: String,
    pub validation_type: ValidationType,
    pub required: bool,
}

/// Different types of validation that can be performed
#[derive(Debug, Clone)]
pub enum ValidationType {
    /// Check if a file exists at the specified path
    FileExists(String),
    /// Check if a function exists in the code
    FunctionExists(String),
    /// Check if tests pass
    TestsPassing,
    /// Check if code compiles
    Compiles,
    /// Check if specific functionality works (custom test)
    CustomTest(fn(&ProjectPath) -> Result<(), String>),
    /// Check if documentation exists
    DocumentationExists,
    /// Check if error handling is implemented
    ErrorHandling,
}

/// Represents a path to a project directory
pub struct ProjectPath {
    pub path: PathBuf,
}

impl ProjectPath {
    pub fn new(path: PathBuf) -> Self {
        Self { path }
    }
    
    pub fn exists(&self) -> bool {
        self.path.exists()
    }
    
    pub fn cargo_toml_exists(&self) -> bool {
        self.path.join("Cargo.toml").exists()
    }
    
    pub fn src_dir_exists(&self) -> bool {
        self.path.join("src").exists()
    }
    
    pub fn main_rs_exists(&self) -> bool {
        self.path.join("src/main.rs").exists()
    }
    
    pub fn lib_rs_exists(&self) -> bool {
        self.path.join("src/lib.rs").exists()
    }
    
    pub fn tests_dir_exists(&self) -> bool {
        self.path.join("tests").exists()
    }
    
    pub fn readme_exists(&self) -> bool {
        self.path.join("README.md").exists()
    }
    
    pub fn concepts_exists(&self) -> bool {
        self.path.join("CONCEPTS.md").exists()
    }
}

/// Project validator that checks if implementations meet requirements
pub struct ProjectValidator {
    pub project_requirements: HashMap<String, Vec<ProjectRequirement>>,
}

impl ProjectValidator {
    pub fn new() -> Self {
        let mut validator = Self {
            project_requirements: HashMap::new(),
        };
        
        // Initialize requirements for all projects
        validator.initialize_basic_level_requirements();
        validator.initialize_intermediate_level_requirements();
        validator.initialize_advanced_level_requirements();
        validator.initialize_expert_level_requirements();
        
        validator
    }
    
    /// Validate a specific project against its requirements
    pub fn validate_project(&self, project_name: &str, project_path: &Path) -> TestSuite {
        let mut suite = TestSuite::new(format!("Project Validation: {}", project_name));
        let project_path = ProjectPath::new(project_path.to_path_buf());
        
        if let Some(requirements) = self.project_requirements.get(project_name) {
            for requirement in requirements {
                let test_result = self.validate_requirement(&requirement, &project_path);
                suite.add_test(test_result);
            }
        } else {
            suite.add_test(TestResult::failure(
                "Unknown Project".to_string(),
                format!("No requirements defined for project: {}", project_name),
                0,
            ));
        }
        
        suite
    }
    
    /// Validate a single requirement
    fn validate_requirement(&self, requirement: &ProjectRequirement, project_path: &ProjectPath) -> TestResult {
        let start_time = std::time::Instant::now();
        
        let result = match &requirement.validation_type {
            ValidationType::FileExists(file_path) => {
                let full_path = project_path.path.join(file_path);
                if full_path.exists() {
                    Ok(())
                } else {
                    Err(format!("Required file not found: {}", file_path))
                }
            }
            ValidationType::FunctionExists(function_name) => {
                self.check_function_exists(project_path, function_name)
            }
            ValidationType::TestsPassing => {
                self.check_tests_passing(project_path)
            }
            ValidationType::Compiles => {
                self.check_compilation(project_path)
            }
            ValidationType::CustomTest(test_fn) => {
                test_fn(project_path)
            }
            ValidationType::DocumentationExists => {
                self.check_documentation(project_path)
            }
            ValidationType::ErrorHandling => {
                self.check_error_handling(project_path)
            }
        };
        
        let duration = start_time.elapsed().as_millis();
        
        match result {
            Ok(_) => TestResult::success(
                format!("{}: {}", requirement.id, requirement.description),
                duration,
            ),
            Err(error) => TestResult::failure(
                format!("{}: {}", requirement.id, requirement.description),
                error,
                duration,
            ),
        }
    }
    
    /// Check if a function exists in the project code
    fn check_function_exists(&self, project_path: &ProjectPath, function_name: &str) -> Result<(), String> {
        let src_files = vec!["src/main.rs", "src/lib.rs"];
        
        for src_file in src_files {
            let file_path = project_path.path.join(src_file);
            if file_path.exists() {
                if let Ok(content) = fs::read_to_string(&file_path) {
                    if content.contains(&format!("fn {}", function_name)) {
                        return Ok(());
                    }
                }
            }
        }
        
        Err(format!("Function '{}' not found in project source code", function_name))
    }
    
    /// Check if tests pass
    fn check_tests_passing(&self, project_path: &ProjectPath) -> Result<(), String> {
        if !project_path.cargo_toml_exists() {
            return Err("No Cargo.toml found - not a valid Rust project".to_string());
        }
        
        let output = Command::new("cargo")
            .args(&["test", "--quiet"])
            .current_dir(&project_path.path)
            .output();
        
        match output {
            Ok(result) => {
                if result.status.success() {
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&result.stderr);
                    Err(format!("Tests failed: {}", stderr))
                }
            }
            Err(e) => Err(format!("Failed to run tests: {}", e)),
        }
    }
    
    /// Check if project compiles
    fn check_compilation(&self, project_path: &ProjectPath) -> Result<(), String> {
        if !project_path.cargo_toml_exists() {
            return Err("No Cargo.toml found - not a valid Rust project".to_string());
        }
        
        let output = Command::new("cargo")
            .args(&["check", "--quiet"])
            .current_dir(&project_path.path)
            .output();
        
        match output {
            Ok(result) => {
                if result.status.success() {
                    Ok(())
                } else {
                    let stderr = String::from_utf8_lossy(&result.stderr);
                    Err(format!("Compilation failed: {}", stderr))
                }
            }
            Err(e) => Err(format!("Failed to compile: {}", e)),
        }
    }
    
    /// Check if documentation exists
    fn check_documentation(&self, project_path: &ProjectPath) -> Result<(), String> {
        let required_docs = vec!["README.md", "CONCEPTS.md"];
        let mut missing_docs = Vec::new();
        
        for doc in required_docs {
            if !project_path.path.join(doc).exists() {
                missing_docs.push(doc);
            }
        }
        
        if missing_docs.is_empty() {
            Ok(())
        } else {
            Err(format!("Missing documentation files: {}", missing_docs.join(", ")))
        }
    }
    
    /// Check if error handling is implemented
    fn check_error_handling(&self, project_path: &ProjectPath) -> Result<(), String> {
        let src_files = vec!["src/main.rs", "src/lib.rs"];
        let error_patterns = vec!["Result<", "Option<", "match", "if let", "unwrap_or"];
        
        for src_file in src_files {
            let file_path = project_path.path.join(src_file);
            if file_path.exists() {
                if let Ok(content) = fs::read_to_string(&file_path) {
                    for pattern in &error_patterns {
                        if content.contains(pattern) {
                            return Ok(());
                        }
                    }
                }
            }
        }
        
        Err("No error handling patterns found (Result, Option, match, etc.)".to_string())
    }
    
    /// Initialize requirements for basic level projects
    fn initialize_basic_level_requirements(&mut self) {
        // Calculator project requirements
        let calculator_requirements = vec![
            ProjectRequirement {
                id: "CALC-001".to_string(),
                description: "Project structure exists".to_string(),
                validation_type: ValidationType::FileExists("Cargo.toml".to_string()),
                required: true,
            },
            ProjectRequirement {
                id: "CALC-002".to_string(),
                description: "Source code exists".to_string(),
                validation_type: ValidationType::FileExists("src/main.rs".to_string()),
                required: true,
            },
            ProjectRequirement {
                id: "CALC-003".to_string(),
                description: "Project compiles successfully".to_string(),
                validation_type: ValidationType::Compiles,
                required: true,
            },
            ProjectRequirement {
                id: "CALC-004".to_string(),
                description: "Parse input function exists".to_string(),
                validation_type: ValidationType::FunctionExists("parse_input".to_string()),
                required: true,
            },
            ProjectRequirement {
                id: "CALC-005".to_string(),
                description: "Perform calculation function exists".to_string(),
                validation_type: ValidationType::FunctionExists("perform_calculation".to_string()),
                required: true,
            },
            ProjectRequirement {
                id: "CALC-006".to_string(),
                description: "Tests exist and pass".to_string(),
                validation_type: ValidationType::TestsPassing,
                required: true,
            },
            ProjectRequirement {
                id: "CALC-007".to_string(),
                description: "Error handling implemented".to_string(),
                validation_type: ValidationType::ErrorHandling,
                required: true,
            },
            ProjectRequirement {
                id: "CALC-008".to_string(),
                description: "Documentation exists".to_string(),
                validation_type: ValidationType::DocumentationExists,
                required: true,
            },
        ];
        
        self.project_requirements.insert("calculator".to_string(), calculator_requirements);
        
        // File Explorer project requirements
        let file_explorer_requirements = vec![
            ProjectRequirement {
                id: "FEXP-001".to_string(),
                description: "Project structure exists".to_string(),
                validation_type: ValidationType::FileExists("Cargo.toml".to_string()),
                required: true,
            },
            ProjectRequirement {
                id: "FEXP-002".to_string(),
                description: "Source code exists".to_string(),
                validation_type: ValidationType::FileExists("src/main.rs".to_string()),
                required: true,
            },
            ProjectRequirement {
                id: "FEXP-003".to_string(),
                description: "Project compiles successfully".to_string(),
                validation_type: ValidationType::Compiles,
                required: true,
            },
            ProjectRequirement {
                id: "FEXP-004".to_string(),
                description: "List directory function exists".to_string(),
                validation_type: ValidationType::FunctionExists("list_directory".to_string()),
                required: true,
            },
            ProjectRequirement {
                id: "FEXP-005".to_string(),
                description: "Error handling implemented".to_string(),
                validation_type: ValidationType::ErrorHandling,
                required: true,
            },
        ];
        
        self.project_requirements.insert("file-explorer".to_string(), file_explorer_requirements);
    }
    
    /// Initialize requirements for intermediate level projects
    fn initialize_intermediate_level_requirements(&mut self) {
        // Library Management System requirements
        let library_requirements = vec![
            ProjectRequirement {
                id: "LIB-001".to_string(),
                description: "Project structure exists".to_string(),
                validation_type: ValidationType::FileExists("Cargo.toml".to_string()),
                required: true,
            },
            ProjectRequirement {
                id: "LIB-002".to_string(),
                description: "Library structure exists".to_string(),
                validation_type: ValidationType::FileExists("src/lib.rs".to_string()),
                required: true,
            },
            ProjectRequirement {
                id: "LIB-003".to_string(),
                description: "Project compiles successfully".to_string(),
                validation_type: ValidationType::Compiles,
                required: true,
            },
            ProjectRequirement {
                id: "LIB-004".to_string(),
                description: "Book struct exists".to_string(),
                validation_type: ValidationType::CustomTest(|path| {
                    Self::check_struct_exists(path, "Book")
                }),
                required: true,
            },
            ProjectRequirement {
                id: "LIB-005".to_string(),
                description: "Library struct exists".to_string(),
                validation_type: ValidationType::CustomTest(|path| {
                    Self::check_struct_exists(path, "Library")
                }),
                required: true,
            },
            ProjectRequirement {
                id: "LIB-006".to_string(),
                description: "Tests exist and pass".to_string(),
                validation_type: ValidationType::TestsPassing,
                required: true,
            },
        ];
        
        self.project_requirements.insert("library-management".to_string(), library_requirements);
    }
    
    /// Initialize requirements for advanced level projects
    fn initialize_advanced_level_requirements(&mut self) {
        // Thread Pool requirements
        let thread_pool_requirements = vec![
            ProjectRequirement {
                id: "TPOOL-001".to_string(),
                description: "Project structure exists".to_string(),
                validation_type: ValidationType::FileExists("Cargo.toml".to_string()),
                required: true,
            },
            ProjectRequirement {
                id: "TPOOL-002".to_string(),
                description: "ThreadPool struct exists".to_string(),
                validation_type: ValidationType::CustomTest(|path| {
                    Self::check_struct_exists(path, "ThreadPool")
                }),
                required: true,
            },
            ProjectRequirement {
                id: "TPOOL-003".to_string(),
                description: "Execute method exists".to_string(),
                validation_type: ValidationType::FunctionExists("execute".to_string()),
                required: true,
            },
            ProjectRequirement {
                id: "TPOOL-004".to_string(),
                description: "Project compiles successfully".to_string(),
                validation_type: ValidationType::Compiles,
                required: true,
            },
        ];
        
        self.project_requirements.insert("thread-pool".to_string(), thread_pool_requirements);
    }
    
    /// Initialize requirements for expert level projects
    fn initialize_expert_level_requirements(&mut self) {
        // Async Network Server requirements
        let async_server_requirements = vec![
            ProjectRequirement {
                id: "ASYNC-001".to_string(),
                description: "Project structure exists".to_string(),
                validation_type: ValidationType::FileExists("Cargo.toml".to_string()),
                required: true,
            },
            ProjectRequirement {
                id: "ASYNC-002".to_string(),
                description: "Async main function exists".to_string(),
                validation_type: ValidationType::CustomTest(|path| {
                    Self::check_async_main_exists(path)
                }),
                required: true,
            },
            ProjectRequirement {
                id: "ASYNC-003".to_string(),
                description: "Project compiles successfully".to_string(),
                validation_type: ValidationType::Compiles,
                required: true,
            },
        ];
        
        self.project_requirements.insert("async-network-server".to_string(), async_server_requirements);
    }
    
    /// Helper function to check if a struct exists
    fn check_struct_exists(project_path: &ProjectPath, struct_name: &str) -> Result<(), String> {
        let src_files = vec!["src/main.rs", "src/lib.rs"];
        
        for src_file in src_files {
            let file_path = project_path.path.join(src_file);
            if file_path.exists() {
                if let Ok(content) = fs::read_to_string(&file_path) {
                    if content.contains(&format!("struct {}", struct_name)) {
                        return Ok(());
                    }
                }
            }
        }
        
        Err(format!("Struct '{}' not found in project source code", struct_name))
    }
    
    /// Helper function to check if async main exists
    fn check_async_main_exists(project_path: &ProjectPath) -> Result<(), String> {
        let file_path = project_path.path.join("src/main.rs");
        if file_path.exists() {
            if let Ok(content) = fs::read_to_string(&file_path) {
                if content.contains("async fn main") || content.contains("#[tokio::main]") {
                    return Ok(());
                }
            }
        }
        
        Err("Async main function not found".to_string())
    }
}

/// Validate all projects in the learning path
pub fn validate_all_projects() -> TestSuite {
    let mut suite = TestSuite::new("All Projects Validation".to_string());
    let validator = ProjectValidator::new();
    
    // Define project paths
    let projects = vec![
        ("calculator", "rust-learning-path/basic/module1/calculator"),
        ("file-explorer", "rust-learning-path/basic/module1/file-explorer"),
        ("library-management", "rust-learning-path/intermediate/library-management"),
        ("thread-pool", "rust-learning-path/advanced/thread-pool"),
        ("async-network-server", "rust-learning-path/expert/async-network-server"),
    ];
    
    for (project_name, project_path) in projects {
        let path = Path::new(project_path);
        if path.exists() {
            let project_suite = validator.validate_project(project_name, path);
            
            // Add individual project results to the main suite
            for test in project_suite.tests {
                suite.add_test(test);
            }
        } else {
            suite.add_test(TestResult::failure(
                format!("Project Missing: {}", project_name),
                format!("Project directory not found: {}", project_path),
                0,
            ));
        }
    }
    
    suite
}