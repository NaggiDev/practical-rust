use std::env;
use std::fs;
use std::io::{self, Read, Write};
use std::path::Path;
use text_processor::{read_text, write_text, TextProcessorError};
use text_processor::transformations::{
    to_uppercase, to_lowercase, count_statistics, 
    replace_pattern, remove_duplicates, sort_lines
};

fn main() {
    println!("Simple Text Processor");
    println!("---------------------");
    
    // Parse command-line arguments
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        print_usage();
        return;
    }
    
    // Process the command
    match process_command(&args) {
        Ok(_) => println!("Processing completed successfully."),
        Err(e) => eprintln!("Error: {}", e),
    }
}

fn print_usage() {
    println!("Usage: text-processor <command> [options]");
    println!();
    println!("Commands:");
    println!("  uppercase <input> [output]    - Convert text to uppercase");
    println!("  lowercase <input> [output]    - Convert text to lowercase");
    println!("  stats <input>                 - Count words, lines, and characters");
    println!("  replace <input> <pattern> <replacement> [output] - Replace pattern with text");
    println!("  dedup <input> [output]        - Remove duplicate lines");
    println!("  sort <input> [output]         - Sort lines alphabetically");
    println!();
    println!("Options:");
    println!("  <input>  - Input file path (use '-' for stdin)");
    println!("  <output> - Output file path (use '-' for stdout, default is stdout)");
}

fn process_command(args: &[String]) -> Result<(), TextProcessorError> {
    let command = &args[1];
    
    match command.as_str() {
        "uppercase" => {
            if args.len() < 3 {
                return Err(TextProcessorError::InvalidArguments(
                    "Missing input file path".to_string()
                ));
            }
            
            let input_path = &args[2];
            let output_path = args.get(3).map(|s| s.as_str()).unwrap_or("-");
            
            let input_text = read_text(input_path)?;
            let transformed = to_uppercase(&input_text);
            write_text(output_path, &transformed)?;
        },
        "lowercase" => {
            if args.len() < 3 {
                return Err(TextProcessorError::InvalidArguments(
                    "Missing input file path".to_string()
                ));
            }
            
            let input_path = &args[2];
            let output_path = args.get(3).map(|s| s.as_str()).unwrap_or("-");
            
            let input_text = read_text(input_path)?;
            let transformed = to_lowercase(&input_text);
            write_text(output_path, &transformed)?;
        },
        "stats" => {
            if args.len() < 3 {
                return Err(TextProcessorError::InvalidArguments(
                    "Missing input file path".to_string()
                ));
            }
            
            let input_path = &args[2];
            let input_text = read_text(input_path)?;
            let stats = count_statistics(&input_text);
            
            println!("Text Statistics:");
            println!("  Characters: {}", stats.char_count);
            println!("  Words: {}", stats.word_count);
            println!("  Lines: {}", stats.line_count);
        },
        "replace" => {
            if args.len() < 5 {
                return Err(TextProcessorError::InvalidArguments(
                    "Missing arguments. Usage: replace <input> <pattern> <replacement> [output]".to_string()
                ));
            }
            
            let input_path = &args[2];
            let pattern = &args[3];
            let replacement = &args[4];
            let output_path = args.get(5).map(|s| s.as_str()).unwrap_or("-");
            
            let input_text = read_text(input_path)?;
            let transformed = replace_pattern(&input_text, pattern, replacement);
            write_text(output_path, &transformed)?;
        },
        "dedup" => {
            if args.len() < 3 {
                return Err(TextProcessorError::InvalidArguments(
                    "Missing input file path".to_string()
                ));
            }
            
            let input_path = &args[2];
            let output_path = args.get(3).map(|s| s.as_str()).unwrap_or("-");
            
            let input_text = read_text(input_path)?;
            let transformed = remove_duplicates(&input_text);
            write_text(output_path, &transformed)?;
        },
        "sort" => {
            if args.len() < 3 {
                return Err(TextProcessorError::InvalidArguments(
                    "Missing input file path".to_string()
                ));
            }
            
            let input_path = &args[2];
            let output_path = args.get(3).map(|s| s.as_str()).unwrap_or("-");
            
            let input_text = read_text(input_path)?;
            let transformed = sort_lines(&input_text);
            write_text(output_path, &transformed)?;
        },
        _ => {
            return Err(TextProcessorError::InvalidArguments(
                format!("Unknown command: {}", command)
            ));
        }
    }
    
    Ok(())
}