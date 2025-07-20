// Command-line interface for running Rust Learning Path tests
use std::env;
use std::process;

use rust_learning_path_tests::{run_all_tests, run_level_tests, run_concept_tests};

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
}