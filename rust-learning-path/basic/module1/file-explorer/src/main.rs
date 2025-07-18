use std::env;
use std::fs;
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::time::SystemTime;

/// Main function that runs the file explorer
fn main() -> io::Result<()> {
    println!("Simple File System Explorer");
    println!("Type 'help' for available commands");
    
    // Get the current directory as the starting point
    let mut current_dir = env::current_dir()?;
    
    // Main program loop
    loop {
        // Display prompt with current directory
        print!("\n{}> ", current_dir.display());
        io::stdout().flush()?;
        
        // Read user input
        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let input = input.trim();
        
        // Handle empty input
        if input.is_empty() {
            continue;
        }
        
        // Parse command and arguments
        let parts: Vec<&str> = input.split_whitespace().collect();
        let command = parts[0];
        let args = &parts[1..];
        
        // Handle the command
        match handle_command(command, args, &mut current_dir) {
            Ok(continue_running) => {
                if !continue_running {
                    println!("Exiting file explorer. Goodbye!");
                    break;
                }
            },
            Err(e) => println!("Error: {}", e),
        }
    }
    
    Ok(())
}

/// Handle user commands
fn handle_command(command: &str, args: &[&str], current_dir: &mut PathBuf) -> io::Result<bool> {
    match command {
        // TODO: Implement "ls" command to list directory contents
        "ls" => {
            list_directory_contents(current_dir)?;
            Ok(true)
        },
        
        // TODO: Implement "cd" command to change directory
        "cd" => {
            if args.is_empty() {
                println!("Usage: cd <directory>");
                return Ok(true);
            }
            
            // TODO: Handle navigation to parent directory with ".."
            // TODO: Handle navigation to home directory with "~"
            // TODO: Handle navigation to absolute paths
            // TODO: Handle navigation to relative paths
            
            println!("Directory navigation not yet implemented");
            Ok(true)
        },
        
        // TODO: Implement "pwd" command to print current directory
        "pwd" => {
            println!("{}", current_dir.display());
            Ok(true)
        },
        
        // TODO: Implement "info" command to display file information
        "info" => {
            if args.is_empty() {
                println!("Usage: info <file>");
                return Ok(true);
            }
            
            // TODO: Get and display file metadata
            
            println!("File information display not yet implemented");
            Ok(true)
        },
        
        // TODO: Implement "cat" command to display file contents
        "cat" => {
            if args.is_empty() {
                println!("Usage: cat <file>");
                return Ok(true);
            }
            
            // TODO: Read and display file contents
            
            println!("File content display not yet implemented");
            Ok(true)
        },
        
        // TODO: Implement "mkdir" command to create a directory
        "mkdir" => {
            if args.is_empty() {
                println!("Usage: mkdir <directory>");
                return Ok(true);
            }
            
            // TODO: Create a new directory
            
            println!("Directory creation not yet implemented");
            Ok(true)
        },
        
        // Display help information
        "help" => {
            display_help();
            Ok(true)
        },
        
        // Exit the program
        "exit" | "quit" => Ok(false),
        
        // Handle unknown commands
        _ => {
            println!("Unknown command: {}. Type 'help' for available commands.", command);
            Ok(true)
        },
    }
}

/// List the contents of the current directory
fn list_directory_contents(path: &Path) -> io::Result<()> {
    println!("\nContents of {}:", path.display());
    println!("{:<30} {:<10} {:<15} {}", "Name", "Type", "Size (bytes)", "Modified");
    println!("{:-<70}", "");
    
    // TODO: Read directory entries
    // TODO: Sort entries by name
    // TODO: Display entry information (name, type, size, modified date)
    
    println!("Directory listing not yet implemented");
    Ok(())
}

/// Display help information
fn display_help() {
    println!("\nAvailable commands:");
    println!("  ls                  List directory contents");
    println!("  cd <directory>      Change to specified directory");
    println!("  pwd                 Print current directory path");
    println!("  info <file>         Display file information");
    println!("  cat <file>          Display file contents");
    println!("  mkdir <directory>   Create a new directory");
    println!("  help                Display this help message");
    println!("  exit, quit          Exit the program");
}

/// Get formatted file size
fn format_file_size(size: u64) -> String {
    // TODO: Format file size in a human-readable way (KB, MB, GB)
    format!("{}", size)
}

/// Get formatted timestamp
fn format_timestamp(time: SystemTime) -> String {
    // TODO: Format timestamp in a human-readable way
    format!("{:?}", time)
}