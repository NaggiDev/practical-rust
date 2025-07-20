// Standalone Quiz Runner for Rust Learning Path
// This provides a dedicated interface for running concept validation quizzes

use std::env;
use std::process;
use rust_learning_path_tests::quiz_framework::{QuizBank, run_interactive_quiz_session};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    if args.len() == 1 {
        // Run interactive quiz session by default
        run_interactive_quiz_session();
        return;
    }
    
    match args[1].as_str() {
        "--help" | "-h" => {
            print_help();
        }
        "--list" | "-l" => {
            list_available_quizzes();
        }
        "--run" | "-r" => {
            if args.len() < 3 {
                eprintln!("Error: --run requires a quiz ID");
                print_help();
                process::exit(1);
            }
            
            let quiz_id = &args[2];
            run_specific_quiz(quiz_id);
        }
        "--interactive" | "-i" => {
            run_interactive_quiz_session();
        }
        _ => {
            // Treat unknown argument as quiz ID
            let quiz_id = &args[1];
            run_specific_quiz(quiz_id);
        }
    }
}

fn list_available_quizzes() {
    let quiz_bank = QuizBank::new();
    let quizzes = quiz_bank.list_available_quizzes();
    
    println!("🎯 Available Concept Validation Quizzes:");
    println!("═══════════════════════════════════════");
    
    for quiz_id in quizzes {
        let formatted_name = quiz_id.replace("_", " ").to_uppercase();
        println!("  📝 {} (ID: {})", formatted_name, quiz_id);
    }
    
    println!("\nUse 'cargo run --bin quiz_runner -- --run <QUIZ_ID>' to run a specific quiz");
    println!("Or 'cargo run --bin quiz_runner' for interactive mode");
}

fn run_specific_quiz(quiz_id: &str) {
    let mut quiz_bank = QuizBank::new();
    
    match quiz_bank.run_quiz(quiz_id) {
        Ok(summary) => {
            println!("{}", summary);
            
            // Provide feedback based on performance
            if summary.score_percentage >= 80.0 {
                println!("\n🎉 Great job! You can move on to the next concept.");
            } else {
                println!("\n📚 Consider reviewing the concept materials before continuing.");
                println!("💡 Focus on the areas where you had incorrect answers.");
            }
        }
        Err(e) => {
            eprintln!("❌ Error: {}", e);
            println!("\n📋 Available quizzes:");
            list_available_quizzes();
            process::exit(1);
        }
    }
}

fn print_help() {
    println!("Rust Learning Path - Quiz Runner");
    println!("═══════════════════════════════");
    println!();
    println!("USAGE:");
    println!("    cargo run --bin quiz_runner                    Run interactive quiz session");
    println!("    cargo run --bin quiz_runner -- [OPTIONS]      Run with specific options");
    println!();
    println!("OPTIONS:");
    println!("    -h, --help                   Show this help message");
    println!("    -l, --list                   List all available quizzes");
    println!("    -r, --run <QUIZ_ID>          Run a specific quiz");
    println!("    -i, --interactive            Run interactive quiz session");
    println!();
    println!("QUIZ IDs:");
    println!("    basic_variables              Variables and data types");
    println!("    basic_ownership              Basic ownership concepts");
    println!("    structs_enums                Structs and enums");
    println!("    traits                       Traits and implementations");
    println!("    concurrency                  Concurrency and threading");
    println!("    async                        Async programming");
    println!();
    println!("EXAMPLES:");
    println!("    cargo run --bin quiz_runner                           # Interactive mode");
    println!("    cargo run --bin quiz_runner -- --list                 # List all quizzes");
    println!("    cargo run --bin quiz_runner -- --run basic_variables  # Run specific quiz");
    println!("    cargo run --bin quiz_runner -- basic_ownership        # Run quiz (short form)");
    println!();
    println!("TIPS:");
    println!("  • Take your time to read each question carefully");
    println!("  • Pay attention to the code snippets when provided");
    println!("  • Review the explanations for both correct and incorrect answers");
    println!("  • Aim for 80% or higher to demonstrate concept mastery");
}