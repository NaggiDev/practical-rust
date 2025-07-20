// Command Line Interface for Project Validation
// Provides easy-to-use commands for validating Rust Learning Path projects

use std::env;
use std::process;
use crate::project_validation_runner::{
    ProjectValidationRunner, 
    validate_single_project, 
    validate_level_projects, 
    validate_all_learning_path_projects,
    check_project_readiness
};

/// Main CLI entry point for project validation
pub fn run_project_validator_cli() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_help();
        return;
    }
    
    match args[1].as_str() {
        "validate" => handle_validate_command(&args[2..]),
        "check" => handle_check_command(&args[2..]),
        "list" => handle_list_command(),
        "help" | "--help" | "-h" => print_help(),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            print_help();
            process::exit(1);
        }
    }
}

/// Handle the validate command
fn handle_validate_command(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: validate command requires a target");
        println!("Usage: validate <project|level|all>");
        return;
    }
    
    match args[0].as_str() {
        "all" => validate_all_projects(),
        "basic" | "intermediate" | "advanced" | "expert" => validate_level(&args[0]),
        project_name => validate_project(project_name),
    }
}

/// Handle the check command (readiness check)
fn handle_check_command(args: &[String]) {
    if args.is_empty() {
        eprintln!("Error: check command requires a project name");
        println!("Usage: check <project_name>");
        return;
    }
    
    let project_name = &args[0];
    let readiness = check_project_readiness(project_name);
    
    println!("🔍 PROJECT READINESS CHECK: {}", project_name.to_uppercase());
    println!("=".repeat(50));
    
    for check in &readiness.checks {
        let status = if check.passed { "✅" } else { "❌" };
        println!("{} {}", status, check.description);
    }
    
    println!("\n📊 Readiness Score: {:.1}%", readiness.readiness_score);
    
    if readiness.is_ready {
        println!("🎉 Project is ready for validation!");
    } else {
        println!("⚠️  Project needs more setup before validation");
        println!("💡 Complete the missing items above to improve readiness");
    }
}

/// Handle the list command
fn handle_list_command() {
    let runner = ProjectValidationRunner::new();
    let projects = runner.list_available_projects();
    
    println!("📋 AVAILABLE PROJECTS FOR VALIDATION");
    println!("=".repeat(40));
    
    let levels = vec![
        ("Basic Level", vec!["calculator", "file-explorer", "text-processor", "todo-list"]),
        ("Intermediate Level", vec!["library-management", "web-scraper", "custom-data-structure", "cli-database-tool"]),
        ("Advanced Level", vec!["thread-pool", "memory-allocator", "c-library-binding", "dsl-project"]),
        ("Expert Level", vec!["async-network-server", "custom-runtime", "compiler-plugin", "data-processing-pipeline"]),
    ];
    
    for (level_name, level_projects) in levels {
        println!("\n🎯 {}", level_name);
        for project in level_projects {
            if projects.contains(&project.to_string()) {
                println!("  • {}", project);
            }
        }
    }
    
    println!("\n💡 Usage Examples:");
    println!("  validate calculator          # Validate specific project");
    println!("  validate basic              # Validate all basic level projects");
    println!("  validate all                # Validate all projects");
    println!("  check calculator            # Check project readiness");
}

/// Validate a single project
fn validate_project(project_name: &str) {
    println!("🧪 Validating project: {}", project_name);
    println!("=".repeat(50));
    
    let result = validate_single_project(project_name);
    println!("{}", result.generate_report());
    
    // Exit with appropriate code
    if result.validation_suite.success_rate() == 100.0 {
        println!("🎉 All validations passed!");
        process::exit(0);
    } else {
        println!("⚠️  Some validations failed. See feedback above.");
        process::exit(1);
    }
}

/// Validate all projects in a level
fn validate_level(level: &str) {
    println!("🎯 Validating {} level projects", level);
    println!("=".repeat(50));
    
    let result = validate_level_projects(level);
    
    if let Some(error) = &result.error_message {
        eprintln!("Error: {}", error);
        process::exit(1);
    }
    
    // Print summary
    println!("📊 LEVEL SUMMARY");
    println!("Total tests: {}", result.overall_summary.total_tests);
    println!("Passed: {}", result.overall_summary.total_passed);
    println!("Failed: {}", result.overall_summary.total_failed);
    println!("Success rate: {:.1}%", result.overall_summary.success_rate);
    
    // Print individual project results
    for project_result in &result.project_results {
        println!("\n{}", project_result.generate_report());
    }
    
    // Exit with appropriate code
    if result.overall_summary.success_rate == 100.0 {
        println!("🎉 All level validations passed!");
        process::exit(0);
    } else {
        println!("⚠️  Some validations failed. See feedback above.");
        process::exit(1);
    }
}

/// Validate all projects
fn validate_all_projects() {
    println!("🚀 Validating all Rust Learning Path projects");
    println!("=".repeat(60));
    
    let result = validate_all_learning_path_projects();
    
    // Print overall summary
    println!("📊 OVERALL SUMMARY");
    println!("Total tests: {}", result.overall_summary.total_tests);
    println!("Passed: {}", result.overall_summary.total_passed);
    println!("Failed: {}", result.overall_summary.total_failed);
    println!("Success rate: {:.1}%", result.overall_summary.success_rate);
    
    // Print level summaries
    for level_result in &result.level_results {
        println!("\n🎯 {} Level: {:.1}% ({}/{} tests passed)",
            level_result.level.to_uppercase(),
            level_result.overall_summary.success_rate,
            level_result.overall_summary.total_passed,
            level_result.overall_summary.total_tests);
        
        for project_result in &level_result.project_results {
            let status = if project_result.validation_suite.success_rate() == 100.0 { "✅" } else { "❌" };
            println!("  {} {} ({:.1}%)",
                status,
                project_result.project_name,
                project_result.validation_suite.success_rate());
        }
    }
    
    // Detailed reports for failed projects
    let mut has_failures = false;
    for level_result in &result.level_results {
        for project_result in &level_result.project_results {
            if project_result.validation_suite.success_rate() < 100.0 {
                if !has_failures {
                    println!("\n" + "=".repeat(60).as_str());
                    println!("📋 DETAILED FAILURE REPORTS");
                    println!("=".repeat(60));
                    has_failures = true;
                }
                println!("\n{}", project_result.generate_report());
            }
        }
    }
    
    // Exit with appropriate code
    if result.overall_summary.success_rate == 100.0 {
        println!("🎉 All project validations passed!");
        process::exit(0);
    } else {
        println!("⚠️  Some validations failed. See detailed reports above.");
        process::exit(1);
    }
}

/// Print help information
fn print_help() {
    println!("🧪 Rust Learning Path Project Validator");
    println!("=".repeat(40));
    println!();
    println!("USAGE:");
    println!("    project-validator <COMMAND> [OPTIONS]");
    println!();
    println!("COMMANDS:");
    println!("    validate <target>    Validate projects");
    println!("    check <project>      Check project readiness");
    println!("    list                 List available projects");
    println!("    help                 Show this help message");
    println!();
    println!("VALIDATE TARGETS:");
    println!("    all                  Validate all projects");
    println!("    basic                Validate basic level projects");
    println!("    intermediate         Validate intermediate level projects");
    println!("    advanced             Validate advanced level projects");
    println!("    expert               Validate expert level projects");
    println!("    <project_name>       Validate specific project");
    println!();
    println!("EXAMPLES:");
    println!("    project-validator validate calculator");
    println!("    project-validator validate basic");
    println!("    project-validator validate all");
    println!("    project-validator check calculator");
    println!("    project-validator list");
    println!();
    println!("PROJECT VALIDATION CHECKS:");
    println!("    • Project structure (Cargo.toml, src/, etc.)");
    println!("    • Code compilation");
    println!("    • Required functions and types");
    println!("    • Test execution");
    println!("    • Error handling implementation");
    println!("    • Documentation presence");
    println!();
    println!("For more information, visit the project documentation.");
}