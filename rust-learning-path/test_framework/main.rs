// Command-line interface for running Rust Learning Path tests
use std::env;
use std::process;

use rust_learning_path_tests::{run_all_tests, run_level_tests, run_concept_tests};
use rust_learning_path_tests::quiz_framework::{run_interactive_quiz_session, QuizBank};
use rust_learning_path_tests::project_validation_runner::{
    validate_single_project, validate_level_projects, validate_all_learning_path_projects,
    check_project_readiness, ProjectValidationRunner
};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        // Run all tests by default
        println!("Running all tests for Rust Learning Path...\n");
        let summary = run_all_tests();
        println!("{}", summary);
        
        if summary.total_failed > 0 {
            process::exit(1);
        }
        return;
    }
    
    match args[1].as_str() {
        "--help" | "-h" => {
            print_help();
        }
        "--level" | "-l" => {
            if args.len() < 3 {
                eprintln!("Error: --level requires a level name");
                print_help();
                process::exit(1);
            }
            
            let level = &args[2];
            println!("Running tests for {} level...\n", level);
            let summary = run_level_tests(level);
            println!("{}", summary);
            
            if summary.total_failed > 0 {
                process::exit(1);
            }
        }
        "--concept" | "-c" => {
            if args.len() < 3 {
                eprintln!("Error: --concept requires a concept name");
                print_help();
                process::exit(1);
            }
            
            let concept = &args[2];
            println!("Running tests for {} concept...\n", concept);
            let summary = run_concept_tests(concept);
            println!("{}", summary);
            
            if summary.total_failed > 0 {
                process::exit(1);
            }
        }
        "--validate" | "-v" => {
            println!("Validating all code examples...\n");
            let summary = run_all_tests();
            
            if summary.total_failed == 0 {
                println!("✅ All code examples are valid and working correctly!");
                println!("Total tests passed: {}", summary.total_passed);
            } else {
                println!("❌ Some code examples have issues:");
                println!("Failed tests: {}", summary.total_failed);
                println!("Passed tests: {}", summary.total_passed);
                process::exit(1);
            }
        }
        "--stats" | "-s" => {
            println!("Generating test statistics...\n");
            let summary = run_all_tests();
            
            println!("📊 Test Statistics:");
            println!("==================");
            println!("Total tests: {}", summary.total_tests);
            println!("Passed: {} ({:.1}%)", summary.total_passed, summary.success_rate);
            println!("Failed: {} ({:.1}%)", summary.total_failed, 100.0 - summary.success_rate);
            
            if summary.success_rate >= 95.0 {
                println!("\n🎉 Excellent! Almost all tests are passing.");
            } else if summary.success_rate >= 80.0 {
                println!("\n👍 Good! Most tests are passing.");
            } else if summary.success_rate >= 60.0 {
                println!("\n⚠️  Some tests need attention.");
            } else {
                println!("\n🚨 Many tests are failing - needs immediate attention.");
            }
        }
        "--quiz" | "-q" => {
            if args.len() >= 3 {
                let quiz_id = &args[2];
                let mut quiz_bank = QuizBank::new();
                
                match quiz_bank.run_quiz(quiz_id) {
                    Ok(summary) => {
                        println!("{}", summary);
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        process::exit(1);
                    }
                }
            } else {
                // Run interactive quiz session
                run_interactive_quiz_session();
            }
        }
        "--validate-project" | "-vp" => {
            if args.len() < 3 {
                eprintln!("Error: --validate-project requires a project name");
                print_help();
                process::exit(1);
            }
            
            let project_name = &args[2];
            println!("🧪 Validating project: {}", project_name);
            println!("=".repeat(50));
            
            let result = validate_single_project(project_name);
            println!("{}", result.generate_report());
            
            if result.validation_suite.success_rate() == 100.0 {
                println!("🎉 All project validations passed!");
            } else {
                println!("⚠️  Some validations failed. See feedback above.");
                process::exit(1);
            }
        }
        "--validate-level" | "-vl" => {
            if args.len() < 3 {
                eprintln!("Error: --validate-level requires a level name");
                print_help();
                process::exit(1);
            }
            
            let level = &args[2];
            println!("🎯 Validating {} level projects", level);
            println!("=".repeat(50));
            
            let result = validate_level_projects(level);
            
            if let Some(error) = &result.error_message {
                eprintln!("Error: {}", error);
                process::exit(1);
            }
            
            println!("📊 LEVEL SUMMARY");
            println!("Success rate: {:.1}%", result.overall_summary.success_rate);
            
            for project_result in &result.project_results {
                println!("\n{}", project_result.generate_report());
            }
            
            if result.overall_summary.success_rate == 100.0 {
                println!("🎉 All level validations passed!");
            } else {
                println!("⚠️  Some validations failed.");
                process::exit(1);
            }
        }
        "--validate-all-projects" | "-vap" => {
            println!("🚀 Validating all Rust Learning Path projects");
            println!("=".repeat(60));
            
            let result = validate_all_learning_path_projects();
            
            println!("📊 OVERALL SUMMARY");
            println!("Success rate: {:.1}%", result.overall_summary.success_rate);
            
            for level_result in &result.level_results {
                println!("\n🎯 {} Level: {:.1}%",
                    level_result.level.to_uppercase(),
                    level_result.overall_summary.success_rate);
                
                for project_result in &level_result.project_results {
                    let status = if project_result.validation_suite.success_rate() == 100.0 { "✅" } else { "❌" };
                    println!("  {} {} ({:.1}%)",
                        status,
                        project_result.project_name,
                        project_result.validation_suite.success_rate());
                }
            }
            
            if result.overall_summary.success_rate == 100.0 {
                println!("🎉 All project validations passed!");
            } else {
                println!("⚠️  Some validations failed.");
                process::exit(1);
            }
        }
        "--check-project" | "-cp" => {
            if args.len() < 3 {
                eprintln!("Error: --check-project requires a project name");
                print_help();
                process::exit(1);
            }
            
            let project_name = &args[2];
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
            }
        }
        "--list-projects" | "-lp" => {
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
        }
        _ => {
            eprintln!("Error: Unknown option '{}'", args[1]);
            print_help();
            process::exit(1);
        }
    }
}

fn print_help() {
    println!("Rust Learning Path Test Runner");
    println!("==============================");
    println!();
    println!("USAGE:");
    println!("    cargo run                    Run all tests");
    println!("    cargo run -- [OPTIONS]       Run with specific options");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help                   Show this help message");
    println!("    -l, --level <LEVEL>          Run tests for specific level");
    println!("    -c, --concept <CONCEPT>      Run tests for specific concept");
    println!("    -v, --validate               Validate all code examples");
    println!("    -s, --stats                  Show test statistics");
    println!("    -q, --quiz [QUIZ_ID]         Run interactive quizzes (or specific quiz)");
    println!();
    println!("PROJECT VALIDATION OPTIONS:");
    println!("    -vp, --validate-project <PROJECT>   Validate specific project");
    println!("    -vl, --validate-level <LEVEL>       Validate all projects in level");
    println!("    -vap, --validate-all-projects       Validate all projects");
    println!("    -cp, --check-project <PROJECT>      Check project readiness");
    println!("    -lp, --list-projects                 List available projects");
    println!();
    println!("LEVELS:");
    println!("    basic                        Basic Rust concepts");
    println!("    intermediate                 Intermediate Rust features");
    println!("    advanced                     Advanced Rust topics");
    println!("    expert                       Expert-level Rust");
    println!();
    println!("CONCEPTS:");
    println!("    variables                    Variable and mutability tests");
    println!("    data_types                   Data type tests");
    println!("    control_flow                 Control flow tests");
    println!("    functions                    Function tests");
    println!("    strings                      String handling tests");
    println!("    error_handling               Error handling tests");
    println!("    ownership                    Ownership and borrowing tests");
    println!("    structs                      Struct tests");
    println!("    enums                        Enum tests");
    println!("    traits                       Trait tests");
    println!("    generics                     Generic tests");
    println!("    collections                  Collection tests");
    println!("    concurrency                  Concurrency tests");
    println!("    unsafe                       Unsafe Rust tests");
    println!("    macros                       Macro tests");
    println!("    async                        Async programming tests");
    println!("    memory                       Memory management tests");
    println!("    performance                  Performance optimization tests");
    println!("    compiler                     Compiler internals tests");
    println!();
    println!("EXAMPLES:");
    println!("    cargo run                                    # Run all tests");
    println!("    cargo run -- --level basic                  # Run basic level tests");
    println!("    cargo run -- --concept ownership            # Run ownership tests");
    println!("    cargo run -- --validate                     # Validate all examples");
    println!("    cargo run -- --stats                        # Show statistics");
    println!("    cargo run -- --quiz                         # Run interactive quizzes");
    println!("    cargo run -- --quiz basic_variables         # Run specific quiz");
    println!();
    println!("PROJECT VALIDATION EXAMPLES:");
    println!("    cargo run -- --validate-project calculator  # Validate calculator project");
    println!("    cargo run -- --validate-level basic         # Validate basic level projects");
    println!("    cargo run -- --validate-all-projects        # Validate all projects");
    println!("    cargo run -- --check-project calculator     # Check calculator readiness");
    println!("    cargo run -- --list-projects                # List available projects");
}