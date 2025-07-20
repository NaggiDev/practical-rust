// Tests for the Project Validation System
// Ensures the validation framework works correctly

#[cfg(test)]
mod tests {
    use std::fs;
    use std::path::Path;
    use tempfile::TempDir;
    use crate::project_validator::{ProjectValidator, ProjectPath, ValidationType, ProjectRequirement};
    use crate::feedback_system::FeedbackGenerator;
    use crate::project_validation_runner::ProjectValidationRunner;

    /// Create a temporary test project structure
    fn create_test_project(temp_dir: &TempDir, has_cargo_toml: bool, has_src: bool, has_main: bool) -> std::path::PathBuf {
        let project_path = temp_dir.path().join("test_project");
        fs::create_dir_all(&project_path).unwrap();
        
        if has_cargo_toml {
            let cargo_toml = r#"[package]
name = "test_project"
version = "0.1.0"
edition = "2021"
"#;
            fs::write(project_path.join("Cargo.toml"), cargo_toml).unwrap();
        }
        
        if has_src {
            fs::create_dir_all(project_path.join("src")).unwrap();
            
            if has_main {
                let main_rs = r#"fn main() {
    println!("Hello, world!");
}

fn parse_input(input: &str) -> Result<String, String> {
    Ok(input.to_string())
}

fn perform_calculation(input: &str) -> Result<i32, String> {
    Ok(42)
}
"#;
                fs::write(project_path.join("src/main.rs"), main_rs).unwrap();
            }
        }
        
        project_path
    }

    #[test]
    fn test_project_path_checks() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = create_test_project(&temp_dir, true, true, true);
        let project_path = ProjectPath::new(project_path);
        
        assert!(project_path.exists());
        assert!(project_path.cargo_toml_exists());
        assert!(project_path.src_dir_exists());
        assert!(project_path.main_rs_exists());
        assert!(!project_path.lib_rs_exists());
    }

    #[test]
    fn test_project_path_missing_files() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = create_test_project(&temp_dir, false, false, false);
        let project_path = ProjectPath::new(project_path);
        
        assert!(project_path.exists());
        assert!(!project_path.cargo_toml_exists());
        assert!(!project_path.src_dir_exists());
        assert!(!project_path.main_rs_exists());
    }

    #[test]
    fn test_file_exists_validation() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = create_test_project(&temp_dir, true, true, true);
        
        let validator = ProjectValidator::new();
        let requirement = ProjectRequirement {
            id: "TEST-001".to_string(),
            description: "Cargo.toml exists".to_string(),
            validation_type: ValidationType::FileExists("Cargo.toml".to_string()),
            required: true,
        };
        
        let project_path = ProjectPath::new(project_path);
        let result = validator.validate_requirement(&requirement, &project_path);
        
        assert!(result.passed);
        assert!(result.name.contains("TEST-001"));
    }

    #[test]
    fn test_file_missing_validation() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = create_test_project(&temp_dir, false, false, false);
        
        let validator = ProjectValidator::new();
        let requirement = ProjectRequirement {
            id: "TEST-002".to_string(),
            description: "Cargo.toml exists".to_string(),
            validation_type: ValidationType::FileExists("Cargo.toml".to_string()),
            required: true,
        };
        
        let project_path = ProjectPath::new(project_path);
        let result = validator.validate_requirement(&requirement, &project_path);
        
        assert!(!result.passed);
        assert!(result.message.contains("not found"));
    }

    #[test]
    fn test_function_exists_validation() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = create_test_project(&temp_dir, true, true, true);
        
        let validator = ProjectValidator::new();
        let requirement = ProjectRequirement {
            id: "TEST-003".to_string(),
            description: "parse_input function exists".to_string(),
            validation_type: ValidationType::FunctionExists("parse_input".to_string()),
            required: true,
        };
        
        let project_path = ProjectPath::new(project_path);
        let result = validator.validate_requirement(&requirement, &project_path);
        
        assert!(result.passed);
    }

    #[test]
    fn test_function_missing_validation() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = create_test_project(&temp_dir, true, true, true);
        
        let validator = ProjectValidator::new();
        let requirement = ProjectRequirement {
            id: "TEST-004".to_string(),
            description: "missing_function exists".to_string(),
            validation_type: ValidationType::FunctionExists("missing_function".to_string()),
            required: true,
        };
        
        let project_path = ProjectPath::new(project_path);
        let result = validator.validate_requirement(&requirement, &project_path);
        
        assert!(!result.passed);
        assert!(result.message.contains("not found"));
    }

    #[test]
    fn test_project_validation_suite() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = create_test_project(&temp_dir, true, true, true);
        
        let validator = ProjectValidator::new();
        let suite = validator.validate_project("calculator", &project_path);
        
        assert!(!suite.tests.is_empty());
        assert!(suite.name.contains("calculator"));
        
        // Should have some passing tests (file structure) and some failing tests (no actual tests to run)
        assert!(suite.passed_count() > 0);
    }

    #[test]
    fn test_feedback_generator() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = create_test_project(&temp_dir, false, false, false);
        
        let validator = ProjectValidator::new();
        let suite = validator.validate_project("calculator", &project_path);
        
        let feedback_generator = FeedbackGenerator::new();
        let feedback = feedback_generator.generate_project_feedback("calculator", &suite);
        
        assert!(!feedback.is_empty());
        
        // Should have error feedback for missing files
        let has_error_feedback = feedback.iter().any(|f| {
            matches!(f.severity, crate::feedback_system::FeedbackSeverity::Error)
        });
        assert!(has_error_feedback);
    }

    #[test]
    fn test_project_validation_runner() {
        let runner = ProjectValidationRunner::new();
        let projects = runner.list_available_projects();
        
        // Should have some projects configured
        assert!(!projects.is_empty());
        assert!(projects.contains(&"calculator".to_string()));
    }

    #[test]
    fn test_project_readiness_check() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = create_test_project(&temp_dir, true, true, true);
        
        let project_path = ProjectPath::new(project_path);
        
        // Mock a readiness check
        let checks = vec![
            ("Project directory exists", project_path.exists()),
            ("Cargo.toml exists", project_path.cargo_toml_exists()),
            ("src directory exists", project_path.src_dir_exists()),
            ("main.rs exists", project_path.main_rs_exists()),
        ];
        
        let passed_count = checks.iter().filter(|(_, passed)| *passed).count();
        let total_count = checks.len();
        let readiness_score = passed_count as f64 / total_count as f64 * 100.0;
        
        assert_eq!(readiness_score, 100.0);
    }

    #[test]
    fn test_validation_with_incomplete_project() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = create_test_project(&temp_dir, true, false, false);
        
        let validator = ProjectValidator::new();
        let suite = validator.validate_project("calculator", &project_path);
        
        // Should have some failing tests due to missing src directory
        assert!(suite.failed_count() > 0);
        assert!(suite.success_rate() < 100.0);
    }

    #[test]
    fn test_error_handling_validation() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("test_project");
        fs::create_dir_all(&project_path).unwrap();
        fs::create_dir_all(project_path.join("src")).unwrap();
        
        // Create a main.rs with error handling patterns
        let main_rs_with_error_handling = r#"
use std::result::Result;

fn main() {
    match divide(10.0, 2.0) {
        Ok(result) => println!("Result: {}", result),
        Err(e) => println!("Error: {}", e),
    }
}

fn divide(a: f64, b: f64) -> Result<f64, String> {
    if b == 0.0 {
        Err("Division by zero".to_string())
    } else {
        Ok(a / b)
    }
}
"#;
        fs::write(project_path.join("src/main.rs"), main_rs_with_error_handling).unwrap();
        
        let validator = ProjectValidator::new();
        let project_path = ProjectPath::new(project_path);
        
        // Test error handling validation
        let result = validator.check_error_handling(&project_path);
        assert!(result.is_ok());
    }

    #[test]
    fn test_no_error_handling_validation() {
        let temp_dir = TempDir::new().unwrap();
        let project_path = temp_dir.path().join("test_project");
        fs::create_dir_all(&project_path).unwrap();
        fs::create_dir_all(project_path.join("src")).unwrap();
        
        // Create a main.rs without error handling patterns
        let main_rs_without_error_handling = r#"
fn main() {
    println!("Hello, world!");
}

fn add(a: i32, b: i32) -> i32 {
    a + b
}
"#;
        fs::write(project_path.join("src/main.rs"), main_rs_without_error_handling).unwrap();
        
        let validator = ProjectValidator::new();
        let project_path = ProjectPath::new(project_path);
        
        // Test error handling validation
        let result = validator.check_error_handling(&project_path);
        assert!(result.is_err());
        assert!(result.unwrap_err().contains("No error handling patterns found"));
    }
}