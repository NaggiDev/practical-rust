// Project Validation Runner for Rust Learning Path
// Orchestrates comprehensive project validation with detailed feedback

use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use crate::framework::{TestResult, TestSuite, TestRunner, TestSummary};
use crate::project_validator::{ProjectValidator, ProjectPath};
use crate::feedback_system::{FeedbackGenerator, format_feedback_messages};

/// Comprehensive project validation system
pub struct ProjectValidationRunner {
    validator: ProjectValidator,
    feedback_generator: FeedbackGenerator,
    project_paths: HashMap<String, PathBuf>,
}

impl ProjectValidationRunner {
    pub fn new() -> Self {
        let mut runner = Self {
            validator: ProjectValidator::new(),
            feedback_generator: FeedbackGenerator::new(),
            project_paths: HashMap::new(),
        };
        
        runner.initialize_project_paths();
        runner
    }
    
    /// Initialize paths to all projects in the learning path
    fn initialize_project_paths(&mut self) {
        let base_path = Path::new("rust-learning-path");
        
        // Basic level projects
        self.project_paths.insert(
            "calculator".to_string(),
            base_path.join("basic/module1/calculator"),
        );
        self.project_paths.insert(
            "file-explorer".to_string(),
            base_path.join("basic/module1/file-explorer"),
        );
        self.project_paths.insert(
            "text-processor".to_string(),
            base_path.join("basic/module2/text-processor"),
        );
        self.project_paths.insert(
            "todo-list".to_string(),
            base_path.join("basic/module4/todo-list"),
        );
        
        // Intermediate level projects
        self.project_paths.insert(
            "library-management".to_string(),
            base_path.join("intermediate/library-management"),
        );
        self.project_paths.insert(
            "web-scraper".to_string(),
            base_path.join("intermediate/multi-threaded-web-scraper"),
        );
        self.project_paths.insert(
            "custom-data-structure".to_string(),
            base_path.join("intermediate/custom-data-structure"),
        );
        self.project_paths.insert(
            "cli-database-tool".to_string(),
            base_path.join("intermediate/cli-database-tool"),
        );
        
        // Advanced level projects
        self.project_paths.insert(
            "thread-pool".to_string(),
            base_path.join("advanced/thread-pool"),
        );
        self.project_paths.insert(
            "memory-allocator".to_string(),
            base_path.join("advanced/custom-memory-allocator"),
        );
        self.project_paths.insert(
            "c-library-binding".to_string(),
            base_path.join("advanced/c-library-binding"),
        );
        self.project_paths.insert(
            "dsl-project".to_string(),
            base_path.join("advanced/dsl-project"),
        );
        
        // Expert level projects
        self.project_paths.insert(
            "async-network-server".to_string(),
            base_path.join("expert/async-network-server"),
        );
        self.project_paths.insert(
            "custom-runtime".to_string(),
            base_path.join("expert/custom-runtime"),
        );
        self.project_paths.insert(
            "compiler-plugin".to_string(),
            base_path.join("expert/compiler-plugin"),
        );
        self.project_paths.insert(
            "data-processing-pipeline".to_string(),
            base_path.join("expert/high-performance-data-processing"),
        );
    }
    
    /// Validate a specific project and generate comprehensive feedback
    pub fn validate_project_with_feedback(&self, project_name: &str) -> ProjectValidationResult {
        if let Some(project_path) = self.project_paths.get(project_name) {
            if project_path.exists() {
                // Run validation
                let validation_suite = self.validator.validate_project(project_name, project_path);
                
                // Generate feedback
                let feedback_messages = self.feedback_generator.generate_project_feedback(
                    project_name,
                    &validation_suite,
                );
                
                // Create comprehensive result
                ProjectValidationResult {
                    project_name: project_name.to_string(),
                    project_path: project_path.clone(),
                    validation_suite,
                    feedback_messages,
                    exists: true,
                }
            } else {
                // Project doesn't exist
                let mut suite = TestSuite::new(format!("Project Validation: {}", project_name));
                suite.add_test(TestResult::failure(
                    "Project Directory Missing".to_string(),
                    format!("Project directory not found: {}", project_path.display()),
                    0,
                ));
                
                let feedback_messages = vec![
                    crate::feedback_system::FeedbackMessage {
                        severity: crate::feedback_system::FeedbackSeverity::Error,
                        title: "❌ Project Not Found".to_string(),
                        message: format!("The {} project directory doesn't exist.", project_name),
                        suggestions: vec![
                            "Make sure you're in the correct directory".to_string(),
                            "Check if the project has been created yet".to_string(),
                            "Verify the project name is spelled correctly".to_string(),
                        ],
                        resources: vec![],
                    }
                ];
                
                ProjectValidationResult {
                    project_name: project_name.to_string(),
                    project_path: project_path.clone(),
                    validation_suite: suite,
                    feedback_messages,
                    exists: false,
                }
            }
        } else {
            // Unknown project
            let mut suite = TestSuite::new(format!("Project Validation: {}", project_name));
            suite.add_test(TestResult::failure(
                "Unknown Project".to_string(),
                format!("No configuration found for project: {}", project_name),
                0,
            ));
            
            let feedback_messages = vec![
                crate::feedback_system::FeedbackMessage {
                    severity: crate::feedback_system::FeedbackSeverity::Error,
                    title: "❓ Unknown Project".to_string(),
                    message: format!("The project '{}' is not recognized.", project_name),
                    suggestions: vec![
                        "Check the available project names".to_string(),
                        "Make sure the project name is spelled correctly".to_string(),
                    ],
                    resources: vec![],
                }
            ];
            
            ProjectValidationResult {
                project_name: project_name.to_string(),
                project_path: PathBuf::new(),
                validation_suite: suite,
                feedback_messages,
                exists: false,
            }
        }
    }
    
    /// Validate all projects in a specific level
    pub fn validate_level(&self, level: &str) -> LevelValidationResult {
        let project_names = match level.to_lowercase().as_str() {
            "basic" => vec!["calculator", "file-explorer", "text-processor", "todo-list"],
            "intermediate" => vec!["library-management", "web-scraper", "custom-data-structure", "cli-database-tool"],
            "advanced" => vec!["thread-pool", "memory-allocator", "c-library-binding", "dsl-project"],
            "expert" => vec!["async-network-server", "custom-runtime", "compiler-plugin", "data-processing-pipeline"],
            _ => {
                return LevelValidationResult {
                    level: level.to_string(),
                    project_results: vec![],
                    overall_summary: TestSummary {
                        total_tests: 0,
                        total_passed: 0,
                        total_failed: 1,
                        success_rate: 0.0,
                    },
                    error_message: Some(format!("Unknown level: {}. Available levels: basic, intermediate, advanced, expert", level)),
                };
            }
        };
        
        let mut project_results = Vec::new();
        let mut total_tests = 0;
        let mut total_passed = 0;
        let mut total_failed = 0;
        
        for project_name in project_names {
            let result = self.validate_project_with_feedback(project_name);
            
            total_tests += result.validation_suite.total_count();
            total_passed += result.validation_suite.passed_count();
            total_failed += result.validation_suite.failed_count();
            
            project_results.push(result);
        }
        
        let success_rate = if total_tests == 0 {
            0.0
        } else {
            total_passed as f64 / total_tests as f64 * 100.0
        };
        
        LevelValidationResult {
            level: level.to_string(),
            project_results,
            overall_summary: TestSummary {
                total_tests,
                total_passed,
                total_failed,
                success_rate,
            },
            error_message: None,
        }
    }
    
    /// Validate all projects in the learning path
    pub fn validate_all_projects(&self) -> AllProjectsValidationResult {
        let levels = vec!["basic", "intermediate", "advanced", "expert"];
        let mut level_results = Vec::new();
        let mut total_tests = 0;
        let mut total_passed = 0;
        let mut total_failed = 0;
        
        for level in levels {
            let level_result = self.validate_level(level);
            
            total_tests += level_result.overall_summary.total_tests;
            total_passed += level_result.overall_summary.total_passed;
            total_failed += level_result.overall_summary.total_failed;
            
            level_results.push(level_result);
        }
        
        let success_rate = if total_tests == 0 {
            0.0
        } else {
            total_passed as f64 / total_tests as f64 * 100.0
        };
        
        AllProjectsValidationResult {
            level_results,
            overall_summary: TestSummary {
                total_tests,
                total_passed,
                total_failed,
                success_rate,
            },
        }
    }
    
    /// Get list of available projects
    pub fn list_available_projects(&self) -> Vec<String> {
        self.project_paths.keys().cloned().collect()
    }
    
    /// Check project readiness (basic structure exists)
    pub fn check_project_readiness(&self, project_name: &str) -> ProjectReadinessCheck {
        if let Some(project_path) = self.project_paths.get(project_name) {
            let project_path = ProjectPath::new(project_path.clone());
            
            let mut checks = Vec::new();
            let mut passed = 0;
            let mut total = 0;
            
            // Check basic structure
            let basic_checks = vec![
                ("Project directory exists", project_path.exists()),
                ("Cargo.toml exists", project_path.cargo_toml_exists()),
                ("src directory exists", project_path.src_dir_exists()),
                ("main.rs or lib.rs exists", project_path.main_rs_exists() || project_path.lib_rs_exists()),
            ];
            
            for (description, result) in basic_checks {
                checks.push(ReadinessCheck {
                    description: description.to_string(),
                    passed: result,
                });
                total += 1;
                if result {
                    passed += 1;
                }
            }
            
            let readiness_score = if total == 0 { 0.0 } else { passed as f64 / total as f64 * 100.0 };
            
            ProjectReadinessCheck {
                project_name: project_name.to_string(),
                checks,
                readiness_score,
                is_ready: readiness_score >= 75.0,
            }
        } else {
            ProjectReadinessCheck {
                project_name: project_name.to_string(),
                checks: vec![ReadinessCheck {
                    description: "Project configuration exists".to_string(),
                    passed: false,
                }],
                readiness_score: 0.0,
                is_ready: false,
            }
        }
    }
}

/// Result of validating a single project
#[derive(Debug)]
pub struct ProjectValidationResult {
    pub project_name: String,
    pub project_path: PathBuf,
    pub validation_suite: TestSuite,
    pub feedback_messages: Vec<crate::feedback_system::FeedbackMessage>,
    pub exists: bool,
}

impl ProjectValidationResult {
    /// Generate a formatted report for this project
    pub fn generate_report(&self) -> String {
        let mut report = String::new();
        
        report.push_str(&format!("📁 PROJECT: {}\n", self.project_name.to_uppercase()));
        report.push_str(&format!("📍 Path: {}\n", self.project_path.display()));
        report.push_str(&"=".repeat(60));
        report.push('\n');
        
        // Validation results
        report.push_str(&format!("\n📊 VALIDATION RESULTS\n"));
        report.push_str(&format!("Tests: {} passed, {} failed, {} total\n",
            self.validation_suite.passed_count(),
            self.validation_suite.failed_count(),
            self.validation_suite.total_count()));
        report.push_str(&format!("Success rate: {:.1}%\n", self.validation_suite.success_rate()));
        
        // Individual test results
        if !self.validation_suite.tests.is_empty() {
            report.push_str("\n🧪 TEST DETAILS\n");
            for test in &self.validation_suite.tests {
                let status = if test.passed { "✅" } else { "❌" };
                report.push_str(&format!("{} {}\n", status, test.name));
                if !test.passed {
                    report.push_str(&format!("   Error: {}\n", test.message));
                }
            }
        }
        
        // Feedback
        if !self.feedback_messages.is_empty() {
            report.push_str("\n");
            report.push_str(&format_feedback_messages(&self.feedback_messages));
        }
        
        report
    }
}

/// Result of validating all projects in a level
#[derive(Debug)]
pub struct LevelValidationResult {
    pub level: String,
    pub project_results: Vec<ProjectValidationResult>,
    pub overall_summary: TestSummary,
    pub error_message: Option<String>,
}

/// Result of validating all projects
#[derive(Debug)]
pub struct AllProjectsValidationResult {
    pub level_results: Vec<LevelValidationResult>,
    pub overall_summary: TestSummary,
}

/// Project readiness check result
#[derive(Debug)]
pub struct ProjectReadinessCheck {
    pub project_name: String,
    pub checks: Vec<ReadinessCheck>,
    pub readiness_score: f64,
    pub is_ready: bool,
}

/// Individual readiness check
#[derive(Debug)]
pub struct ReadinessCheck {
    pub description: String,
    pub passed: bool,
}

/// Convenience functions for common validation tasks
pub fn validate_single_project(project_name: &str) -> ProjectValidationResult {
    let runner = ProjectValidationRunner::new();
    runner.validate_project_with_feedback(project_name)
}

pub fn validate_level_projects(level: &str) -> LevelValidationResult {
    let runner = ProjectValidationRunner::new();
    runner.validate_level(level)
}

pub fn validate_all_learning_path_projects() -> AllProjectsValidationResult {
    let runner = ProjectValidationRunner::new();
    runner.validate_all_projects()
}

pub fn check_project_readiness(project_name: &str) -> ProjectReadinessCheck {
    let runner = ProjectValidationRunner::new();
    runner.check_project_readiness(project_name)
}