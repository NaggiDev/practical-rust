use clap::{Parser, Subcommand};
use std::path::PathBuf;

pub mod database;
pub mod error;
pub mod record;

use database::Database;
use error::DatabaseError;
use record::Record;

// Re-export for integration tests
pub use database::Database;
pub use error::DatabaseError;
pub use record::Record;

/// A simple command-line database tool for learning Rust error handling
#[derive(Parser)]
#[command(name = "cli-db")]
#[command(about = "A CLI database tool for learning Rust")]
struct Cli {
    /// Path to the database file
    #[arg(short, long, default_value = "database.json")]
    database: PathBuf,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Create a new record
    Create {
        /// Record ID
        id: String,
        /// Record name
        name: String,
        /// Record value
        value: String,
    },
    /// Read a record by ID
    Read {
        /// Record ID to read
        id: String,
    },
    /// Update an existing record
    Update {
        /// Record ID to update
        id: String,
        /// New name (optional)
        #[arg(long)]
        name: Option<String>,
        /// New value (optional)
        #[arg(long)]
        value: Option<String>,
    },
    /// Delete a record by ID
    Delete {
        /// Record ID to delete
        id: String,
    },
    /// List all records
    List,
    /// Initialize a new database
    Init,
}

fn main() {
    if let Err(e) = run() {
        eprintln!("Error: {}", e);
        std::process::exit(1);
    }
}

fn run() -> Result<(), DatabaseError> {
    let cli = Cli::parse();
    let mut db = Database::new(cli.database)?;

    match cli.command {
        Commands::Init => {
            db.initialize()?;
            println!("Database initialized successfully");
        }
        Commands::Create { id, name, value } => {
            let record = Record::new(id.clone(), name, value);
            db.create_record(record)?;
            println!("Record '{}' created successfully", id);
        }
        Commands::Read { id } => {
            match db.read_record(&id)? {
                Some(record) => println!("{}", record),
                None => println!("Record '{}' not found", id),
            }
        }
        Commands::Update { id, name, value } => {
            let updated = db.update_record(&id, name, value)?;
            if updated {
                println!("Record '{}' updated successfully", id);
            } else {
                println!("Record '{}' not found", id);
            }
        }
        Commands::Delete { id } => {
            let deleted = db.delete_record(&id)?;
            if deleted {
                println!("Record '{}' deleted successfully", id);
            } else {
                println!("Record '{}' not found", id);
            }
        }
        Commands::List => {
            let records = db.list_records()?;
            if records.is_empty() {
                println!("No records found");
            } else {
                println!("Found {} record(s):", records.len());
                for record in records {
                    println!("  {}", record);
                }
            }
        }
    }

    Ok(())
}

// TODO: Implement the following functions as part of the learning exercise:
// 1. Enhanced error reporting with context
// 2. Interactive mode for multiple operations
// 3. Backup and restore functionality
// 4. Query operations with filtering